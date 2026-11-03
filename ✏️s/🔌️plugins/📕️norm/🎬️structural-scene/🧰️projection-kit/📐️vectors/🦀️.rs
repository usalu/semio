//! 📐️ Vectors, frames and solids: pure `f64` helpers over [`Vec3`] and [`Frame`].

use crate::*;

pub fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
    v3(a.x + b.x, a.y + b.y, a.z + b.z)
}

pub fn mul(a: &Vec3, k: f64) -> Vec3 {
    v3(a.x * k, a.y * k, a.z * k)
}

pub fn unit(a: &Vec3) -> Vec3 {
    let l = a.length();
    if l < 1e-12 { Vec3::X } else { mul(a, 1.0 / l) }
}

pub fn along(origin: &Vec3, direction: &Vec3, distance: f64) -> Vec3 {
    add(origin, &mul(direction, distance))
}

pub fn polyline(points: &[Vec3]) -> Path3 {
    Path3::Polyline(Polyline3 { points: points.to_vec() })
}

/// 🧭️ Right-handed member frame: `x` along the axis, `y` horizontal (or `-Y` for vertical axes), `z` completing it.
pub fn member_frame(origin: &Vec3, axis: &Vec3) -> Frame {
    let x = unit(axis);
    let reference = if x.cross(&Vec3::Z).length() < 1e-9 { Vec3::X } else { Vec3::Z };
    let y = unit(&reference.cross(&x));
    let z = x.cross(&y);
    Frame { origin: origin.clone(), x, y, z }
}

pub fn world_frame(origin: Vec3) -> Frame {
    Frame { origin, ..Frame::WORLD }
}

/// 📍️ The world point at `s` along the frame axis and `y`, `z` across it.
pub fn at(f: &Frame, s: f64, y: f64, z: f64) -> Vec3 {
    v3(f.origin.x + s * f.x.x + y * f.y.x + z * f.z.x, f.origin.y + s * f.x.y + y * f.y.y + z * f.z.y, f.origin.z + s * f.x.z + y * f.y.z + z * f.z.z)
}

/// 🔄️ `direction` turned by `yaw` rad about `+Z`.
pub fn rotate_yaw(direction: &Vec3, yaw: f64) -> Vec3 {
    v3(direction.x * yaw.cos() - direction.y * yaw.sin(), direction.x * yaw.sin() + direction.y * yaw.cos(), direction.z)
}

pub fn ring3(points: Vec<Vec3>) -> Polygon3 {
    Polygon3 { outer: Ring3 { points }, holes: Vec::new() }
}

pub fn polygon3(points: &[Vec3]) -> Polygon3 {
    ring3(points.to_vec())
}

pub fn polygon2(points: &[[f64; 2]]) -> Polygon2 {
    Polygon2::solid(points.iter().map(|p| Vec2::new(p[0], p[1])).collect())
}

/// 🧊️ Solid helpers; the schema convention anchors a `Box` at the centre of its base on the frame origin.
pub mod solid {
    use super::*;

    /// 📦️ A box whose base-centre convention is fed with its lower `corner` instead: the frame origin is moved to the base centre along the frame axes.
    pub fn box_from_corner(frame: &Frame, size: Vec3) -> SolidShape {
        let centre = at(frame, size.x / 2.0, size.y / 2.0, 0.0);
        SolidShape::Box { frame: Frame { origin: centre, x: frame.x.clone(), y: frame.y.clone(), z: frame.z.clone() }, size }
    }
}
