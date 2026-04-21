use serde::{Deserialize, Serialize};

use crate::hash::HashWriter;

/// 3D coordinate (right-handed).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coord {
    pub const ZERO: Coord = Coord { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.f64(self.x).f64(self.y).f64(self.z);
    }

    pub fn add(&self, other: &Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Coord) -> Coord {
        Coord::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(&self, s: f64) -> Coord {
        Coord::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// 3D unit vector (the type is not enforced at construction time).
pub type Vector = Coord;

/// Oriented plane: origin `p`, x-axis and y-axis directions.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Plane {
    #[serde(default)]
    pub origin: Coord,
    #[serde(default = "Plane::default_x_axis")]
    pub x_axis: Vector,
    #[serde(default = "Plane::default_y_axis")]
    pub y_axis: Vector,
}

impl Plane {
    fn default_x_axis() -> Vector { Vector::new(1.0, 0.0, 0.0) }
    fn default_y_axis() -> Vector { Vector::new(0.0, 1.0, 0.0) }

    pub fn world_xy() -> Self {
        Self {
            origin: Coord::ZERO,
            x_axis: Self::default_x_axis(),
            y_axis: Self::default_y_axis(),
        }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        self.origin.hash_into(w);
        self.x_axis.hash_into(w);
        self.y_axis.hash_into(w);
    }
}

/// Simple orbital camera descriptor.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Camera {
    #[serde(default)]
    pub position: Coord,
    #[serde(default)]
    pub target: Coord,
    #[serde(default = "Camera::default_up")]
    pub up: Vector,
    #[serde(default = "Camera::default_fov")]
    pub fov: f64,
}

impl Camera {
    fn default_up() -> Vector { Vector::new(0.0, 0.0, 1.0) }
    fn default_fov() -> f64 { 45.0 }
}

/// 2D location on the diagram canvas.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}

impl Location {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.f64(self.x).f64(self.y);
    }
}
