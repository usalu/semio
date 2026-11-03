//! 📐️ Copyable f64 vectors for the kernel: free functions on `[f64; 3]` and `[f64; 2]`, converted from and to schema vectors only at the edges.

use crate::{Vec2, Vec3};

pub type P = [f64; 3];
pub type Q = [f64; 2];

pub fn p(v: &Vec3) -> P {
    [v.x, v.y, v.z]
}

pub fn v3(a: P) -> Vec3 {
    Vec3::new(a[0], a[1], a[2])
}

pub fn q(v: &Vec2) -> Q {
    [v.x, v.y]
}

pub fn v2(a: Q) -> Vec2 {
    Vec2::new(a[0], a[1])
}

pub fn add(a: P, b: P) -> P {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn sub(a: P, b: P) -> P {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn mul(a: P, s: f64) -> P {
    [a[0] * s, a[1] * s, a[2] * s]
}

pub fn dot(a: P, b: P) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn cross(a: P, b: P) -> P {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub fn len(a: P) -> f64 {
    dot(a, a).sqrt()
}

pub fn unit(a: P) -> P {
    let l = len(a);
    if l > 0.0 { mul(a, 1.0 / l) } else { a }
}

pub fn lerp(a: P, b: P, t: f64) -> P {
    add(mul(a, 1.0 - t), mul(b, t))
}

pub fn lerp2(a: Q, b: Q, t: f64) -> Q {
    [a[0] * (1.0 - t) + b[0] * t, a[1] * (1.0 - t) + b[1] * t]
}

pub fn rotate(a: P, axis: P, angle: f64) -> P {
    let (s, c) = angle.sin_cos();
    add(add(mul(a, c), mul(cross(axis, a), s)), mul(axis, dot(axis, a) * (1.0 - c)))
}

pub fn perpendicular(a: P) -> P {
    let helper = if a[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    unit(cross(a, helper))
}

pub fn basis(axis: P) -> (P, P, P) {
    let t = unit(axis);
    let u = perpendicular(t);
    (t, u, cross(t, u))
}

pub fn rotate_between(a: P, from: P, to: P) -> P {
    let axis = cross(from, to);
    let s = len(axis);
    let c = dot(from, to);
    if s < 1e-12 {
        if c > 0.0 { a } else { rotate(a, perpendicular(from), std::f64::consts::PI) }
    } else {
        rotate(a, mul(axis, 1.0 / s), s.atan2(c))
    }
}

pub fn quantise(a: P) -> [i64; 3] {
    [(a[0] * 1e8).round() as i64, (a[1] * 1e8).round() as i64, (a[2] * 1e8).round() as i64]
}
