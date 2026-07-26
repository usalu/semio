//! 🧭 3×3/4×4 matrices, unit quaternions, rigid+uniform-scale transforms and orthonormal frames —
//! plain `f64` arrays, no external linear-algebra crate. `Trsf` (not a raw `Mat4`) is the type
//! every kernel operation accepts for placement, so a transform can never silently carry
//! non-uniform shear that would break analytic-surface recognition downstream.

use crate::vec::{Pnt3, Vec3};

// #region 🔖Mat

/// 🧭 A 3×3 matrix in row-major order, used for rotations and normal-transform cofactors.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mat3 {
    pub rows: [[f64; 3]; 3],
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3 { rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    pub fn from_rows(rows: [[f64; 3]; 3]) -> Self {
        Mat3 { rows }
    }
    pub fn from_columns(c0: Vec3, c1: Vec3, c2: Vec3) -> Self {
        Mat3 { rows: [[c0.x, c1.x, c2.x], [c0.y, c1.y, c2.y], [c0.z, c1.z, c2.z]] }
    }
    pub fn column(&self, i: usize) -> Vec3 {
        Vec3::new(self.rows[0][i], self.rows[1][i], self.rows[2][i])
    }
    pub fn transform(&self, v: Vec3) -> Vec3 {
        Vec3::new(self.rows[0][0] * v.x + self.rows[0][1] * v.y + self.rows[0][2] * v.z, self.rows[1][0] * v.x + self.rows[1][1] * v.y + self.rows[1][2] * v.z, self.rows[2][0] * v.x + self.rows[2][1] * v.y + self.rows[2][2] * v.z)
    }
    pub fn transpose(&self) -> Mat3 {
        Mat3::from_rows([[self.rows[0][0], self.rows[1][0], self.rows[2][0]], [self.rows[0][1], self.rows[1][1], self.rows[2][1]], [self.rows[0][2], self.rows[1][2], self.rows[2][2]]])
    }
    pub fn mul(&self, o: &Mat3) -> Mat3 {
        let mut out = [[0.0; 3]; 3];
        for (r, out_row) in out.iter_mut().enumerate() {
            for (c, out_cell) in out_row.iter_mut().enumerate() {
                *out_cell = (0..3).map(|k| self.rows[r][k] * o.rows[k][c]).sum();
            }
        }
        Mat3::from_rows(out)
    }
    pub fn determinant(&self) -> f64 {
        let m = &self.rows;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
    /// 🧭 From an axis-angle rotation (Rodrigues' formula). `axis` need not be normalized.
    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Mat3 {
        let a = axis.normalized().unwrap_or(Vec3::Z);
        let (s, c) = angle.sin_cos();
        let t = 1.0 - c;
        Mat3::from_rows([
            [t * a.x * a.x + c, t * a.x * a.y - s * a.z, t * a.x * a.z + s * a.y],
            [t * a.x * a.y + s * a.z, t * a.y * a.y + c, t * a.y * a.z - s * a.x],
            [t * a.x * a.z - s * a.y, t * a.y * a.z + s * a.x, t * a.z * a.z + c],
        ])
    }
}

// #endregion 🔖Mat

// #region 🔖Quat

/// 🧭 A unit quaternion `(w, x, y, z)` representing a pure rotation.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quat {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quat {
    pub const IDENTITY: Quat = Quat { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };

    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Self {
        let a = axis.normalized().unwrap_or(Vec3::Z);
        let half = angle * 0.5;
        let (s, c) = half.sin_cos();
        Quat { w: c, x: a.x * s, y: a.y * s, z: a.z * s }
    }
    pub fn norm(self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalized(self) -> Quat {
        let n = self.norm();
        if n <= f64::EPSILON {
            Quat::IDENTITY
        } else {
            Quat { w: self.w / n, x: self.x / n, y: self.y / n, z: self.z / n }
        }
    }
    pub fn conjugate(self) -> Quat {
        Quat { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, o: Quat) -> Quat {
        Quat {
            w: self.w * o.w - self.x * o.x - self.y * o.y - self.z * o.z,
            x: self.w * o.x + self.x * o.w + self.y * o.z - self.z * o.y,
            y: self.w * o.y - self.x * o.z + self.y * o.w + self.z * o.x,
            z: self.w * o.z + self.x * o.y - self.y * o.x + self.z * o.w,
        }
    }
    pub fn rotate(self, v: Vec3) -> Vec3 {
        let qv = Quat { w: 0.0, x: v.x, y: v.y, z: v.z };
        let r = self.mul(qv).mul(self.conjugate());
        Vec3::new(r.x, r.y, r.z)
    }
    pub fn to_mat3(self) -> Mat3 {
        let q = self.normalized();
        let (w, x, y, z) = (q.w, q.x, q.y, q.z);
        Mat3::from_rows([
            [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * w), 2.0 * (x * z + y * w)],
            [2.0 * (x * y + z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * w)],
            [2.0 * (x * z - y * w), 2.0 * (y * z + x * w), 1.0 - 2.0 * (x * x + y * y)],
        ])
    }
    /// 🧭 Spherical linear interpolation, taking the short arc between `self` and `o`.
    pub fn slerp(self, o: Quat, t: f64) -> Quat {
        let a = self.normalized();
        let mut b = o.normalized();
        let mut dot = a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z;
        if dot < 0.0 {
            b = Quat { w: -b.w, x: -b.x, y: -b.y, z: -b.z };
            dot = -dot;
        }
        if dot > 0.9995 {
            return Quat { w: a.w + (b.w - a.w) * t, x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t, z: a.z + (b.z - a.z) * t }.normalized();
        }
        let theta0 = dot.acos();
        let theta = theta0 * t;
        let (s, c) = theta.sin_cos();
        let s0 = theta0.sin();
        let scale_a = c - dot * s / s0;
        let scale_b = s / s0;
        Quat { w: a.w * scale_a + b.w * scale_b, x: a.x * scale_a + b.x * scale_b, y: a.y * scale_a + b.y * scale_b, z: a.z * scale_a + b.z * scale_b }
    }
}

// #endregion 🔖Quat

// #region 🔖Trsf

/// 🧭 A rigid transform with optional uniform scale: `p ↦ rotation·(scale·p) + translation`.
/// Deliberately excludes shear/non-uniform scale so analytic surfaces stay analytic after any
/// transform the kernel applies.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trsf {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: f64,
}

impl Trsf {
    pub const IDENTITY: Trsf = Trsf { rotation: Quat::IDENTITY, translation: Vec3::ZERO, scale: 1.0 };

    pub fn translation(t: Vec3) -> Self {
        Trsf { translation: t, ..Trsf::IDENTITY }
    }
    pub fn rotation(q: Quat) -> Self {
        Trsf { rotation: q, ..Trsf::IDENTITY }
    }
    pub fn uniform_scale(s: f64) -> Self {
        Trsf { scale: s, ..Trsf::IDENTITY }
    }
    pub fn apply_point(&self, p: Pnt3) -> Pnt3 {
        let scaled = p.to_vec() * self.scale;
        Pnt3::from_array(self.rotation.rotate(scaled).to_array()) + self.translation
    }
    pub fn apply_vector(&self, v: Vec3) -> Vec3 {
        self.rotation.rotate(v * self.scale)
    }
    /// 🧭 Transforms a surface normal correctly under non-uniform-free `Trsf` (uniform scale
    /// leaves direction unchanged, so this is just the rotation — kept as its own method so
    /// callers never reach for `apply_vector` on a normal by habit).
    pub fn apply_normal(&self, n: Vec3) -> Vec3 {
        self.rotation.rotate(n)
    }
    pub fn compose(&self, inner: &Trsf) -> Trsf {
        Trsf { rotation: self.rotation.mul(inner.rotation), translation: self.apply_vector(inner.translation) + self.translation, scale: self.scale * inner.scale }
    }
    pub fn inverse(&self) -> Trsf {
        let inv_rot = self.rotation.conjugate();
        let inv_scale = 1.0 / self.scale;
        let inv_translation = inv_rot.rotate(-self.translation * inv_scale);
        Trsf { rotation: inv_rot, translation: inv_translation, scale: inv_scale }
    }
}

// #endregion 🔖Trsf

// #region 🔖Frame

/// 🧭 A right-handed orthonormal frame: origin plus three unit axes with `z = x × y`.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Frame3 {
    pub origin: Pnt3,
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Frame3 {
    pub const WORLD: Frame3 = Frame3 { origin: Pnt3 { x: 0.0, y: 0.0, z: 0.0 }, x: Vec3::X, y: Vec3::Y, z: Vec3::Z };

    /// 🧭 Builds a frame from an origin and a normal `z`-axis; `x`/`y` are derived deterministically
    /// via [`Vec3::any_orthogonal`] so the same normal always produces the same frame.
    pub fn from_normal(origin: Pnt3, normal: Vec3) -> Option<Frame3> {
        let z = normal.normalized()?;
        let x = z.any_orthogonal();
        let y = z.cross(x);
        Some(Frame3 { origin, x, y, z })
    }
    /// 🧭 Builds a frame from an origin, a preferred `x` direction and a `z` normal — `x` is
    /// Gram-Schmidt orthogonalized against `z`, `y` completes the right-handed triad.
    pub fn from_x_z(origin: Pnt3, x_hint: Vec3, z_hint: Vec3) -> Option<Frame3> {
        let z = z_hint.normalized()?;
        let x_proj = x_hint - z * x_hint.dot(z);
        let x = x_proj.normalized()?;
        let y = z.cross(x);
        Some(Frame3 { origin, x, y, z })
    }
    pub fn to_world(&self, local: Pnt3) -> Pnt3 {
        self.origin + self.x * local.x + self.y * local.y + self.z * local.z
    }
    pub fn to_world_vector(&self, local: Vec3) -> Vec3 {
        self.x * local.x + self.y * local.y + self.z * local.z
    }
    pub fn to_local(&self, world: Pnt3) -> Pnt3 {
        let v = world - self.origin;
        Pnt3::new(v.dot(self.x), v.dot(self.y), v.dot(self.z))
    }
    pub fn to_local_vector(&self, world: Vec3) -> Vec3 {
        Vec3::new(world.dot(self.x), world.dot(self.y), world.dot(self.z))
    }
}

// #endregion 🔖Frame

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quat_from_axis_angle_rotates_correctly() {
        let q = Quat::from_axis_angle(Vec3::Z, std::f64::consts::FRAC_PI_2);
        let r = q.rotate(Vec3::X);
        assert!((r - Vec3::Y).norm() < 1e-9);
    }

    #[test]
    fn quat_conjugate_is_inverse_rotation() {
        let q = Quat::from_axis_angle(Vec3::new(1.0, 2.0, 3.0), 0.7);
        let v = Vec3::new(4.0, -1.0, 2.0);
        let round_trip = q.conjugate().rotate(q.rotate(v));
        assert!((round_trip - v).norm() < 1e-9);
    }

    #[test]
    fn quat_to_mat3_matches_direct_rotation() {
        let q = Quat::from_axis_angle(Vec3::new(0.3, 0.7, -0.2), 1.1);
        let v = Vec3::new(1.0, 0.0, 0.0);
        let via_quat = q.rotate(v);
        let via_mat = q.to_mat3().transform(v);
        assert!((via_quat - via_mat).norm() < 1e-9);
    }

    #[test]
    fn slerp_endpoints_match_inputs() {
        let a = Quat::from_axis_angle(Vec3::Z, 0.0);
        let b = Quat::from_axis_angle(Vec3::Z, 1.5);
        let s0 = a.slerp(b, 0.0);
        let s1 = a.slerp(b, 1.0);
        assert!((s0.rotate(Vec3::X) - a.rotate(Vec3::X)).norm() < 1e-9);
        assert!((s1.rotate(Vec3::X) - b.rotate(Vec3::X)).norm() < 1e-9);
    }

    #[test]
    fn trsf_inverse_round_trips_a_point() {
        let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.9), translation: Vec3::new(5.0, -2.0, 3.0), scale: 2.5 };
        let p = Pnt3::new(1.0, 2.0, 3.0);
        let round_trip = t.inverse().apply_point(t.apply_point(p));
        assert!(round_trip.distance(p) < 1e-9);
    }

    #[test]
    fn trsf_compose_matches_sequential_application() {
        let a = Trsf { rotation: Quat::from_axis_angle(Vec3::Z, 0.4), translation: Vec3::new(1.0, 0.0, 0.0), scale: 1.0 };
        let b = Trsf { rotation: Quat::from_axis_angle(Vec3::X, 0.9), translation: Vec3::new(0.0, 2.0, 0.0), scale: 1.5 };
        let p = Pnt3::new(3.0, -1.0, 2.0);
        let composed = a.compose(&b).apply_point(p);
        let sequential = a.apply_point(b.apply_point(p));
        assert!(composed.distance(sequential) < 1e-9);
    }

    #[test]
    fn frame_from_normal_round_trips_local_world() {
        let f = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 5.0)).unwrap();
        let local = Pnt3::new(2.0, -1.0, 0.5);
        let round_trip = f.to_local(f.to_world(local));
        assert!(round_trip.distance(local) < 1e-9);
    }

    #[test]
    fn frame_from_normal_is_right_handed() {
        let f = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
        assert!((f.x.cross(f.y) - f.z).norm() < 1e-9);
    }

    #[test]
    fn frame_from_normal_deterministic_for_axis_aligned_normals() {
        let f1 = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::X).unwrap();
        let f2 = Frame3::from_normal(Pnt3::new(5.0, 5.0, 5.0), Vec3::X).unwrap();
        assert_eq!(f1.x, f2.x);
        assert_eq!(f1.y, f2.y);
    }

    #[test]
    fn mat3_determinant_of_identity_is_one() {
        assert!((Mat3::IDENTITY.determinant() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn mat3_from_axis_angle_matches_quat_rotation() {
        let axis = Vec3::new(0.2, -0.4, 0.9);
        let angle = 1.3;
        let m = Mat3::from_axis_angle(axis, angle);
        let q = Quat::from_axis_angle(axis, angle);
        let v = Vec3::new(1.0, 2.0, -3.0);
        assert!((m.transform(v) - q.rotate(v)).norm() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn trsf_inverse_round_trips_random_points() {
            let mut rng = mathematical_random::Rng::from_seed(7);
            for _ in 0..200 {
                let axis = Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5);
                let angle = rng.next_f64() * std::f64::consts::TAU;
                let t = Trsf { rotation: Quat::from_axis_angle(axis, angle), translation: Vec3::new(rng.next_f64() * 10.0, rng.next_f64() * 10.0, rng.next_f64() * 10.0), scale: 0.1 + rng.next_f64() * 5.0 };
                let p = Pnt3::new(rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0);
                let round_trip = t.inverse().apply_point(t.apply_point(p));
                assert!(round_trip.distance(p) < 1e-7, "round trip drifted: {round_trip:?} vs {p:?}");
            }
        }
    }
}
// #endregion 🔖Tests
