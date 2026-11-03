//! 🧭️ Vector and pose algebra of the viewport lowering: plain `[f64; 3]` triples, quaternions as `[x, y, z, w]`.

pub type V3 = [f64; 3];

pub fn v3(p: &super::scene::Vec3) -> V3 {
    [p.x, p.y, p.z]
}

pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn mul(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

pub fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub fn len(a: V3) -> f64 {
    dot(a, a).sqrt()
}

pub fn unit(a: V3) -> V3 {
    let l = len(a);
    if l < 1e-12 { [0.0, 0.0, 1.0] } else { mul(a, 1.0 / l) }
}

pub fn lerp(a: V3, b: V3, t: f64) -> V3 {
    add(a, mul(sub(b, a), t))
}

/// 🧱️ Unit direction orthogonal to `x`, as close to `hint` as possible.
pub fn orthogonal(x: V3, hint: V3) -> V3 {
    let x = unit(x);
    let candidate = sub(hint, mul(x, dot(hint, x)));
    if len(candidate) > 1e-9 { return unit(candidate); }
    let fallback = if x[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    unit(sub(fallback, mul(x, dot(fallback, x))))
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub position: V3,
    pub rotation: [f64; 4],
    pub scale: V3,
}

/// 🔄️ Quaternion of the rotation whose matrix columns are the orthonormal images of the local axes.
pub fn quat_from_basis(x: V3, y: V3, z: V3) -> [f64; 4] {
    let cols = [x, y, z];
    let m = |r: usize, c: usize| cols[c][r];
    let trace = m(0, 0) + m(1, 1) + m(2, 2);
    let q = if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [(m(2, 1) - m(1, 2)) / s, (m(0, 2) - m(2, 0)) / s, (m(1, 0) - m(0, 1)) / s, s / 4.0]
    } else if m(0, 0) >= m(1, 1) && m(0, 0) >= m(2, 2) {
        let s = (1.0 + m(0, 0) - m(1, 1) - m(2, 2)).sqrt() * 2.0;
        [s / 4.0, (m(0, 1) + m(1, 0)) / s, (m(0, 2) + m(2, 0)) / s, (m(2, 1) - m(1, 2)) / s]
    } else if m(1, 1) >= m(2, 2) {
        let s = (1.0 + m(1, 1) - m(0, 0) - m(2, 2)).sqrt() * 2.0;
        [(m(0, 1) + m(1, 0)) / s, s / 4.0, (m(1, 2) + m(2, 1)) / s, (m(0, 2) - m(2, 0)) / s]
    } else {
        let s = (1.0 + m(2, 2) - m(0, 0) - m(1, 1)).sqrt() * 2.0;
        [(m(0, 2) + m(2, 0)) / s, (m(1, 2) + m(2, 1)) / s, s / 4.0, (m(1, 0) - m(0, 1)) / s]
    };
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    [q[0] / n, q[1] / n, q[2] / n, q[3] / n]
}

pub fn rotate(q: [f64; 4], v: V3) -> V3 {
    let u = [q[0], q[1], q[2]];
    let t = mul(cross(u, v), 2.0);
    add(add(v, mul(t, q[3])), cross(u, t))
}

/// 🧷️ Pose placing a mesh whose local x, y, z are the given world directions.
pub fn basis_pose(position: V3, ax: V3, ay: V3, az: V3, scale: V3) -> Pose {
    Pose { position, rotation: quat_from_basis(unit(ax), unit(ay), unit(az)), scale }
}

/// 📏️ Pose of a mesh extruded along local +Z, stretched from `from` to `to`; local x follows `hint` projected orthogonal to the axis.
pub fn span_pose(from: V3, to: V3, hint: V3, section_scale: (f64, f64)) -> Pose {
    let z = unit(sub(to, from));
    let x = orthogonal(z, hint);
    let y = cross(z, x);
    basis_pose(from, x, y, z, [section_scale.0, section_scale.1, len(sub(to, from)).max(1e-9)])
}
