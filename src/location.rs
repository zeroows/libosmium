use std::fmt::{Display, Formatter};

const UNDEFINED_COORDINATE: i32 = i32::MAX;
pub const PRECISION: i32 = 10000000;

/// A gps coordinate in angles of latitude and longitude
///
/// The actual data is stored in `x` and `y` as integers which are `1/PRECISION`-th of a degree.
///
/// [Libosmium's cpp reference](https://docs.osmcode.org/libosmium/latest/classosmium_1_1Location.html)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub raw_x: i32,
    pub raw_y: i32,
}

impl Location {
    pub fn is_defined(&self) -> bool {
        self.raw_x != UNDEFINED_COORDINATE || self.raw_y != UNDEFINED_COORDINATE
    }

    pub fn is_undefined(&self) -> bool {
        self.raw_x == UNDEFINED_COORDINATE && self.raw_y == UNDEFINED_COORDINATE
    }

    pub fn is_valid(&self) -> bool {
        self.raw_x >= -180 * PRECISION
            && self.raw_x <= 180 * PRECISION
            && self.raw_y >= -90 * PRECISION
            && self.raw_y <= 90 * PRECISION
    }

    pub fn lon(&self) -> f64 {
        self.raw_x as f64 / PRECISION as f64
    }

    pub fn lat(&self) -> f64 {
        self.raw_y as f64 / PRECISION as f64
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (mut x1, mut x2) = (self.raw_x / PRECISION, self.raw_x % PRECISION);
        let (mut y1, mut y2) = (self.raw_y / PRECISION, self.raw_y % PRECISION);
        x2 = x2.abs();
        y2 = y2.abs();
        let lat = if self.raw_y > 0 {
            "N"
        } else {
            y1 = -y1;
            "S"
        };
        let lon = if self.raw_x > 0 {
            "E"
        } else {
            x1 = -x1;
            "W"
        };
        write!(f, "{}.{}° {} {}.{}° {}", y1, y2, lat, x1, x2, lon)
    }
}
