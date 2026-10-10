//! 📐️ The geometry of a gbXML surface: the polygon in a frame whose +Y points at true north (so `Azimuth` means the same whatever `CADModelAzimuth` a reader applies), the frame of the surface seen from outside (right, up, outward normal), and its
//! rectangular geometry (the bounding rectangle in that frame with its lower-left corner), all rounded to nine decimals so the document and the tables the oracles compare are made of the same numbers.
//! 📎 https://www.gbxml.org/schema_doc/7.03/GreenBuildingXML_Ver7.03.html

use crate::standards::v1::subsets::any::schema::inferences::opening_frames::Vec3;

/// 📍️ A point or vector in metres.
pub type P3 = [f64; 3];

/// 🔢️ A number rounded to nine decimals, never `-0`.
pub fn round9(value: f64) -> f64 {
    let rounded = (value * 1e9).round() / 1e9;
    if rounded == 0.0 {
        0.0
    } else {
        rounded
    }
}

/// 🧭️ A point of the building turned clockwise by `bearing` radians (true north minus the rotation of the building) so +Y is true north.
pub fn turned(point: &Vec3, bearing: f64) -> P3 {
    let (sine, cosine) = bearing.sin_cos();
    [round9(point.x * cosine + point.y * sine), round9(-point.x * sine + point.y * cosine), round9(point.z)]
}

fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: P3, b: P3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: P3, b: P3) -> P3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit(a: P3) -> Option<P3> {
    let length = dot(a, a).sqrt();
    (length > 1e-12).then(|| [a[0] / length, a[1] / length, a[2] / length])
}

/// 🧮️ Twice the area vector of a polygon (Newell): its direction is the normal of the counter-clockwise side, its length twice the area.
pub fn area_vector(polygon: &[P3]) -> P3 {
    let mut sum = [0.0; 3];
    for (index, a) in polygon.iter().enumerate() {
        let b = polygon[(index + 1) % polygon.len()];
        sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    sum
}

/// 📏️ The area of a planar polygon.
pub fn area(polygon: &[P3]) -> f64 {
    dot(area_vector(polygon), area_vector(polygon)).sqrt() / 2.0
}

/// 🪟️ The frame of a surface seen from outside: `right` and `up` span its plane, `normal` points out. Up is the vertical projected into the plane (for a horizontal surface north), right is `up x normal`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub right: P3,
    pub up: P3,
    pub normal: P3,
}

impl Frame {
    /// 🪟️ The frame of a polygon, `None` for a degenerate one.
    pub fn of(polygon: &[P3]) -> Option<Self> {
        let normal = unit(area_vector(polygon))?;
        let vertical = [0.0, 0.0, 1.0];
        let up = unit(sub(vertical, scaled(normal, dot(vertical, normal)))).or_else(|| unit(sub([0.0, 1.0, 0.0], scaled(normal, normal[1]))))?;
        Some(Self { right: cross(up, normal), up, normal })
    }

    /// 🧭️ The azimuth of the outward normal in degrees clockwise from +Y, zero for a horizontal surface.
    pub fn azimuth(&self) -> f64 {
        if self.normal[0].hypot(self.normal[1]) < 1e-9 {
            0.0
        } else {
            round9(self.normal[0].atan2(self.normal[1]).to_degrees().rem_euclid(360.0))
        }
    }

    /// 📐️ The tilt in degrees: 0 faces up, 90 a wall, 180 faces down.
    pub fn tilt(&self) -> f64 {
        round9(self.normal[2].clamp(-1.0, 1.0).acos().to_degrees())
    }
}

fn scaled(a: P3, factor: f64) -> P3 {
    [a[0] * factor, a[1] * factor, a[2] * factor]
}

/// 📦️ The rectangular geometry of a polygon: the lower-left corner of its bounding rectangle in the frame, and the width and height of that rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub corner: P3,
    pub width: f64,
    pub height: f64,
}

/// 📦️ The bounding rectangle of `polygon` in the axes of `frame`, measured from the first vertex.
pub fn rect_in(frame: &Frame, polygon: &[P3]) -> Rect {
    let origin = polygon[0];
    let (mut low, mut high) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for point in polygon {
        let offset = sub(*point, origin);
        let local = [dot(offset, frame.right), dot(offset, frame.up)];
        for axis in 0..2 {
            low[axis] = low[axis].min(local[axis]);
            high[axis] = high[axis].max(local[axis]);
        }
    }
    let corner = [origin[0] + frame.right[0] * low[0] + frame.up[0] * low[1], origin[1] + frame.right[1] * low[0] + frame.up[1] * low[1], origin[2] + frame.right[2] * low[0] + frame.up[2] * low[1]];
    Rect { corner: corner.map(round9), width: round9(high[0] - low[0]), height: round9(high[1] - low[1]) }
}

/// 📍️ The position of `polygon`'s bounding rectangle inside the rectangle of its host surface: the offset of its corner along the host's right and up axes, from the host's corner.
pub fn offset_in(host: &Frame, host_corner: P3, polygon: &[P3]) -> ([f64; 2], Rect) {
    let rect = rect_in(host, polygon);
    let offset = sub(rect.corner, host_corner);
    ([round9(dot(offset, host.right)), round9(dot(offset, host.up))], rect)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
