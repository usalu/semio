//! 🌀 Lie groups for rigid motion: quaternions, SO(3)/SE(3)/Sim(3) exp-log maps, Jacobians, Umeyama alignment and pose interpolation.

use mathematical_algebra::{vec3d_cross, vec3d_length, vec3d_normalize, vec3d_sub, Mat3d, MatD};

// #region 🔖Quat
/// 🧭 Unit quaternion `w + xi + yj + zk` in Hamilton convention for chaining and interpolating 3D rotations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quatd {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quatd {
    /// 🪞 Identity rotation quaternion.
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }

    /// 📐 Four-vector dot product, `1` for equal unit quaternions and `-1` for their antipodes.
    pub fn dot(self, other: Self) -> f64 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 📏 Unit-norm copy; a near-zero quaternion falls back to identity.
    pub fn normalize(self) -> Self {
        let n = self.dot(self).sqrt();
        if n < 1e-300 {
            return Self::identity();
        }
        Self { w: self.w / n, x: self.x / n, y: self.y / n, z: self.z / n }
    }

    /// ✖️ Hamilton product `self ⊗ other`, applying `other` first and `self` second.
    #[allow(clippy::should_implement_trait, reason = "value-semantics mul mirrors mathematical_algebra::Mat3d::mul; operator overloading is intentionally avoided in this workspace")]
    pub fn mul(self, other: Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }

    /// 🔄 Conjugate, the inverse rotation for unit quaternions.
    pub fn conjugate(self) -> Self {
        Self { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }

    /// 🌪️ Rotates a 3D vector by this unit quaternion via the Rodrigues-style two-cross shortcut.
    pub fn rotate(self, v: [f64; 3]) -> [f64; 3] {
        let u = [self.x, self.y, self.z];
        let tw = vec3d_cross(u, v);
        let tw = [2.0 * tw[0], 2.0 * tw[1], 2.0 * tw[2]];
        let cr = vec3d_cross(u, tw);
        [v[0] + self.w * tw[0] + cr[0], v[1] + self.w * tw[1] + cr[1], v[2] + self.w * tw[2] + cr[2]]
    }

    /// 🧊 Column-major rotation matrix of the normalized quaternion.
    pub fn to_mat3d(self) -> Mat3d {
        let q = self.normalize();
        let (w, x, y, z) = (q.w, q.x, q.y, q.z);
        Mat3d::from_axes(
            [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + w * z), 2.0 * (x * z - w * y)],
            [2.0 * (x * y - w * z), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + w * x)],
            [2.0 * (x * z + w * y), 2.0 * (y * z - w * x), 1.0 - 2.0 * (x * x + y * y)],
        )
    }

    /// 🎯 Quaternion from a rotation matrix via Shepperd's method, branching on the largest diagonal entry for stability at negative traces.
    pub fn from_mat3d(m: &Mat3d) -> Self {
        let e = |r: usize, c: usize| m.cols[c][r];
        let trace = e(0, 0) + e(1, 1) + e(2, 2);
        let q = if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Self { w: 0.25 * s, x: (e(2, 1) - e(1, 2)) / s, y: (e(0, 2) - e(2, 0)) / s, z: (e(1, 0) - e(0, 1)) / s }
        } else if e(0, 0) >= e(1, 1) && e(0, 0) >= e(2, 2) {
            let s = (1.0 + e(0, 0) - e(1, 1) - e(2, 2)).max(0.0).sqrt() * 2.0;
            Self { w: (e(2, 1) - e(1, 2)) / s, x: 0.25 * s, y: (e(0, 1) + e(1, 0)) / s, z: (e(0, 2) + e(2, 0)) / s }
        } else if e(1, 1) >= e(2, 2) {
            let s = (1.0 + e(1, 1) - e(0, 0) - e(2, 2)).max(0.0).sqrt() * 2.0;
            Self { w: (e(0, 2) - e(2, 0)) / s, x: (e(0, 1) + e(1, 0)) / s, y: 0.25 * s, z: (e(1, 2) + e(2, 1)) / s }
        } else {
            let s = (1.0 + e(2, 2) - e(0, 0) - e(1, 1)).max(0.0).sqrt() * 2.0;
            Self { w: (e(1, 0) - e(0, 1)) / s, x: (e(0, 2) + e(2, 0)) / s, y: (e(1, 2) + e(2, 1)) / s, z: 0.25 * s }
        };
        q.normalize()
    }

    /// 🌈 Spherical linear interpolation along the shortest arc, degrading to normalized lerp for nearly parallel inputs.
    pub fn slerp(a: Self, b: Self, t: f64) -> Self {
        let mut d = a.dot(b);
        let mut bq = b;
        if d < 0.0 {
            bq = Self { w: -b.w, x: -b.x, y: -b.y, z: -b.z };
            d = -d;
        }
        if d > 1.0 - 1e-9 {
            let lerped = Self { w: a.w + t * (bq.w - a.w), x: a.x + t * (bq.x - a.x), y: a.y + t * (bq.y - a.y), z: a.z + t * (bq.z - a.z) };
            return lerped.normalize();
        }
        let theta = d.clamp(-1.0, 1.0).acos();
        let sin_t = theta.sin();
        let wa = ((1.0 - t) * theta).sin() / sin_t;
        let wb = (t * theta).sin() / sin_t;
        Self { w: wa * a.w + wb * bq.w, x: wa * a.x + wb * bq.x, y: wa * a.y + wb * bq.y, z: wa * a.z + wb * bq.z }.normalize()
    }
}
// #endregion 🔖Quat

// #region 🔖So3
fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vec3_scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn mat3_axpy(base: Mat3d, m: Mat3d, s: f64) -> Mat3d {
    Mat3d { cols: std::array::from_fn(|c| std::array::from_fn(|r| base.cols[c][r] + s * m.cols[c][r])) }
}

fn mat3_scaled_identity(s: f64) -> Mat3d {
    Mat3d::from_axes([s, 0.0, 0.0], [0.0, s, 0.0], [0.0, 0.0, s])
}

fn mat3_det(m: Mat3d) -> f64 {
    vec3_dot(m.cols[0], vec3d_cross(m.cols[1], m.cols[2]))
}

fn mat3_inverse(m: Mat3d) -> Option<Mat3d> {
    let det = mat3_det(m);
    if det.abs() < 1e-300 {
        return None;
    }
    let inv = 1.0 / det;
    let e = |r: usize, c: usize| m.cols[c][r];
    let cof = |i: usize, j: usize| {
        let (r1, r2) = ((i + 1) % 3, (i + 2) % 3);
        let (c1, c2) = ((j + 1) % 3, (j + 2) % 3);
        e(r1, c1) * e(r2, c2) - e(r1, c2) * e(r2, c1)
    };
    Some(Mat3d { cols: std::array::from_fn(|c| std::array::from_fn(|r| cof(c, r) * inv)) })
}

fn sym3_eigen(m: Mat3d) -> ([f64; 3], Mat3d) {
    let e = |r: usize, c: usize| 0.5 * (m.cols[c][r] + m.cols[r][c]);
    let mut a = [[e(0, 0), e(0, 1), e(0, 2)], [e(1, 0), e(1, 1), e(1, 2)], [e(2, 0), e(2, 1), e(2, 2)]];
    let mut v = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let scale_sq: f64 = a.iter().flatten().map(|x| x * x).sum();
    for _ in 0..30 {
        let off = a[0][1] * a[0][1] + a[0][2] * a[0][2] + a[1][2] * a[1][2];
        if off <= scale_sq * 1e-30 + 1e-300 {
            break;
        }
        for &(p, q) in &[(0_usize, 1_usize), (0, 2), (1, 2)] {
            let apq = a[p][q];
            if apq.abs() < 1e-300 {
                continue;
            }
            let tau = (a[q][q] - a[p][p]) / (2.0 * apq);
            let t = tau.signum() / (tau.abs() + (1.0 + tau * tau).sqrt());
            let c = 1.0 / (1.0 + t * t).sqrt();
            let s = t * c;
            for k in 0..3 {
                let (akp, akq) = (a[k][p], a[k][q]);
                a[k][p] = c * akp - s * akq;
                a[k][q] = s * akp + c * akq;
            }
            for k in 0..3 {
                let (apk, aqk) = (a[p][k], a[q][k]);
                a[p][k] = c * apk - s * aqk;
                a[q][k] = s * apk + c * aqk;
            }
            for k in 0..3 {
                let (vkp, vkq) = (v[k][p], v[k][q]);
                v[k][p] = c * vkp - s * vkq;
                v[k][q] = s * vkp + c * vkq;
            }
        }
    }
    let vals = [a[0][0], a[1][1], a[2][2]];
    let mut order = [0_usize, 1, 2];
    order.sort_by(|&i, &j| vals[i].total_cmp(&vals[j]));
    let col = |j: usize| [v[0][j], v[1][j], v[2][j]];
    ([vals[order[0]], vals[order[1]], vals[order[2]]], Mat3d::from_axes(col(order[0]), col(order[1]), col(order[2])))
}

fn pick_orthogonal(a: [f64; 3]) -> [f64; 3] {
    let e = if a[0].abs() <= a[1].abs() && a[0].abs() <= a[2].abs() {
        [1.0, 0.0, 0.0]
    } else if a[1].abs() <= a[2].abs() {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    vec3d_normalize(vec3d_sub(e, vec3_scale(a, vec3_dot(e, a))))
}

fn svd3(m: Mat3d) -> (Mat3d, [f64; 3], Mat3d) {
    let (vals, vecs) = sym3_eigen(m.transpose().mul(m));
    let v_cols = [vecs.cols[2], vecs.cols[1], vecs.cols[0]];
    let sigma: [f64; 3] = std::array::from_fn(|i| vals[2 - i].max(0.0).sqrt());
    let tol = sigma[0] * 1e-9;
    let mut u0 = vec3d_normalize(m.mul_vec3(v_cols[0]));
    if vec3d_length(u0) < 0.5 {
        u0 = [1.0, 0.0, 0.0];
    }
    let mut u1 = if sigma[1] > tol { m.mul_vec3(v_cols[1]) } else { pick_orthogonal(u0) };
    u1 = vec3d_normalize(vec3d_sub(u1, vec3_scale(u0, vec3_dot(u1, u0))));
    if vec3d_length(u1) < 0.5 {
        u1 = pick_orthogonal(u0);
    }
    let mut u2 = vec3d_cross(u0, u1);
    if sigma[2] > tol && vec3_dot(m.mul_vec3(v_cols[2]), u2) < 0.0 {
        u2 = vec3_scale(u2, -1.0);
    }
    (Mat3d::from_axes(u0, u1, u2), sigma, Mat3d::from_axes(v_cols[0], v_cols[1], v_cols[2]))
}

/// 🔄 Rotation group SO(3) element stored as an orthonormal, det `+1` column-major matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct So3(pub Mat3d);

impl So3 {
    /// 🪞 Identity rotation.
    pub fn identity() -> Self {
        Self(Mat3d::IDENTITY)
    }

    /// 🎩 Skew-symmetric hat operator: `hat(w) · v = w × v`.
    pub fn hat(w: [f64; 3]) -> Mat3d {
        Mat3d { cols: [[0.0, w[2], -w[1]], [-w[2], 0.0, w[0]], [w[1], -w[0], 0.0]] }
    }

    /// 🎣 Inverse of [`So3::hat`], reading the axis vector out of a skew-symmetric matrix.
    pub fn vee(m: &Mat3d) -> [f64; 3] {
        [m.cols[1][2], m.cols[2][0], m.cols[0][1]]
    }

    /// 🚀 Exponential map via the Rodrigues formula, with a Taylor fallback for small angles.
    pub fn exp(w: [f64; 3]) -> Self {
        let theta = vec3d_length(w);
        let k = Self::hat(w);
        let k2 = k.mul(k);
        let t2 = theta * theta;
        let (a, b) = if theta < 1e-4 { (1.0 - t2 / 6.0, 0.5 - t2 / 24.0) } else { (theta.sin() / theta, (1.0 - theta.cos()) / t2) };
        Self(mat3_axpy(mat3_axpy(Mat3d::IDENTITY, k, a), k2, b))
    }

    /// 🪵 Logarithm map returning the rotation vector, robust near zero (Taylor) and near `π` (largest-diagonal axis extraction).
    pub fn log(&self) -> [f64; 3] {
        let e = |r: usize, c: usize| self.0.cols[c][r];
        let vee_twice = [e(2, 1) - e(1, 2), e(0, 2) - e(2, 0), e(1, 0) - e(0, 1)];
        let sin_t = 0.5 * vec3d_length(vee_twice);
        let cos_t = 0.5 * (e(0, 0) + e(1, 1) + e(2, 2) - 1.0);
        let theta = sin_t.atan2(cos_t);
        if cos_t > -0.99 {
            if sin_t < 1e-10 {
                return vec3_scale(vee_twice, 0.5 + theta * theta / 12.0);
            }
            return vec3_scale(vee_twice, theta / (2.0 * sin_t));
        }
        let diag = [e(0, 0), e(1, 1), e(2, 2)];
        let k = if diag[0] >= diag[1] && diag[0] >= diag[2] {
            0
        } else if diag[1] >= diag[2] {
            1
        } else {
            2
        };
        let denom = 1.0 - cos_t;
        let axis_k = ((diag[k] - cos_t) / denom).max(0.0).sqrt();
        let mut axis = [0.0; 3];
        axis[k] = axis_k;
        for i in 0..3 {
            if i != k {
                axis[i] = (e(i, k) + e(k, i)) / (2.0 * denom * axis_k);
            }
        }
        let axis = vec3d_normalize(axis);
        let axis = if vec3_dot(axis, vee_twice) < 0.0 { vec3_scale(axis, -1.0) } else { axis };
        vec3_scale(axis, theta)
    }

    /// 🔗 Group composition `self · other`.
    pub fn compose(&self, other: &Self) -> Self {
        Self(self.0.mul(other.0))
    }

    /// ↩️ Inverse rotation, the transpose of the matrix.
    pub fn inverse(&self) -> Self {
        Self(self.0.transpose())
    }

    /// 🎬 Rotates a point or vector.
    pub fn act(&self, v: [f64; 3]) -> [f64; 3] {
        self.0.mul_vec3(v)
    }

    /// 🃏 Left Jacobian `J_l(w)` of SO(3), with small-angle Taylor coefficients.
    pub fn jl(w: [f64; 3]) -> Mat3d {
        let theta = vec3d_length(w);
        let k = Self::hat(w);
        let k2 = k.mul(k);
        let t2 = theta * theta;
        let (b, c) = if theta < 1e-4 { (0.5 - t2 / 24.0, 1.0 / 6.0 - t2 / 120.0) } else { ((1.0 - theta.cos()) / t2, (theta - theta.sin()) / (t2 * theta)) };
        mat3_axpy(mat3_axpy(Mat3d::IDENTITY, k, b), k2, c)
    }

    /// 🂠 Inverse left Jacobian `J_l(w)⁻¹`, with small-angle Taylor coefficients and a guard where `sin θ` vanishes.
    pub fn jl_inv(w: [f64; 3]) -> Mat3d {
        let theta = vec3d_length(w);
        let k = Self::hat(w);
        let k2 = k.mul(k);
        let t2 = theta * theta;
        let d = if theta < 1e-4 {
            1.0 / 12.0 + t2 / 720.0
        } else {
            let sin_t = theta.sin();
            if sin_t.abs() < 1e-9 { 1.0 / t2 } else { 1.0 / t2 - (1.0 + theta.cos()) / (2.0 * theta * sin_t) }
        };
        mat3_axpy(mat3_axpy(Mat3d::IDENTITY, k, -0.5), k2, d)
    }

    /// 🧲 Nearest rotation (Frobenius norm) with det `+1` via a local 3×3 Jacobi-eigen polar decomposition, since `mathematical_algebra` exposes no SVD yet; swap to the shared SVD once it lands.
    pub fn project_to_so3(m: &Mat3d) -> Self {
        let (u, _sigma, v) = svd3(*m);
        let d = if mat3_det(u) * mat3_det(v) < 0.0 { -1.0 } else { 1.0 };
        let u_fixed = Mat3d::from_axes(u.cols[0], u.cols[1], vec3_scale(u.cols[2], d));
        Self(u_fixed.mul(v.transpose()))
    }

    /// 🎯 Rotation from a unit quaternion.
    pub fn from_quat(q: Quatd) -> Self {
        Self(q.to_mat3d())
    }

    /// 🧭 Unit quaternion of this rotation.
    pub fn to_quat(&self) -> Quatd {
        Quatd::from_mat3d(&self.0)
    }
}
// #endregion 🔖So3

// #region 🔖Se3
fn xi6_scale(xi: [f64; 6], s: f64) -> [f64; 6] {
    std::array::from_fn(|k| xi[k] * s)
}

/// 🦾 Rigid transform in SE(3): rotation `r` followed by translation `t`, so `p ↦ r·p + t`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Se3 {
    pub r: So3,
    pub t: [f64; 3],
}

impl Se3 {
    /// 🪞 Identity transform.
    pub fn identity() -> Self {
        Self { r: So3::identity(), t: [0.0; 3] }
    }

    /// 🔗 Group composition `self · other`.
    pub fn compose(&self, other: &Self) -> Self {
        Self { r: self.r.compose(&other.r), t: vec3_add(self.r.act(other.t), self.t) }
    }

    /// ↩️ Inverse transform.
    pub fn inverse(&self) -> Self {
        let rinv = self.r.inverse();
        Self { r: rinv, t: vec3_scale(rinv.act(self.t), -1.0) }
    }

    /// 🚀 Exponential map of a twist `xi = (rho, phi)`, translating via the SO(3) left Jacobian `V = J_l(phi)`.
    pub fn exp(xi: [f64; 6]) -> Self {
        let rho = [xi[0], xi[1], xi[2]];
        let phi = [xi[3], xi[4], xi[5]];
        Self { r: So3::exp(phi), t: So3::jl(phi).mul_vec3(rho) }
    }

    /// 🪵 Logarithm map returning the twist `(rho, phi)` with `rho = J_l(phi)⁻¹ · t`.
    pub fn log(&self) -> [f64; 6] {
        let phi = self.r.log();
        let rho = So3::jl_inv(phi).mul_vec3(self.t);
        [rho[0], rho[1], rho[2], phi[0], phi[1], phi[2]]
    }

    /// 🗺️ 6×6 adjoint `[[R, hat(t)·R], [0, R]]` acting on twists ordered `(rho, phi)`.
    pub fn adjoint(&self) -> MatD {
        let rot = self.r.0;
        let t_hat_r = So3::hat(self.t).mul(rot);
        let mut adj = MatD::zeros(6, 6);
        for c in 0..3 {
            for r in 0..3 {
                adj.set(r, c, rot.cols[c][r]);
                adj.set(r, c + 3, t_hat_r.cols[c][r]);
                adj.set(r + 3, c + 3, rot.cols[c][r]);
            }
        }
        adj
    }

    /// 🎬 Applies the rigid transform to a point.
    pub fn act(&self, p: [f64; 3]) -> [f64; 3] {
        vec3_add(self.r.act(p), self.t)
    }
}
// #endregion 🔖Se3

// #region 🔖Sim3
fn sim3_w(phi: [f64; 3], sigma: f64) -> Mat3d {
    let theta = vec3d_length(phi);
    let k = So3::hat(phi);
    let k2 = k.mul(k);
    let t2 = theta * theta;
    let scale = sigma.exp();
    let (coeff_c, coeff_a, coeff_b) = if sigma.abs() < 1e-3 {
        let c = 1.0 + sigma / 2.0 + sigma * sigma / 6.0;
        if theta < 1e-3 {
            let a = 0.5 - t2 / 24.0 + sigma * (1.0 / 3.0 - t2 / 30.0) + sigma * sigma / 8.0;
            let b = 1.0 / 6.0 - t2 / 120.0 + sigma * (0.125 - t2 / 144.0) + sigma * sigma / 20.0;
            (c, a, b)
        } else {
            let (sin_t, cos_t) = (theta.sin(), theta.cos());
            let a = (1.0 - cos_t) / t2 + sigma * (sin_t - theta * cos_t) / (t2 * theta) + sigma * sigma * ((2.0 - t2) * cos_t + 2.0 * theta * sin_t - 2.0) / (2.0 * t2 * t2);
            let b = (theta - sin_t) / (t2 * theta) + sigma * (t2 / 2.0 + 1.0 - cos_t - theta * sin_t) / (t2 * t2) + sigma * sigma * (theta * t2 / 3.0 - t2 * sin_t - 2.0 * theta * cos_t + 2.0 * sin_t) / (2.0 * t2 * t2 * theta);
            (c, a, b)
        }
    } else {
        let s2 = sigma * sigma;
        let c = (scale - 1.0) / sigma;
        if theta < 1e-3 {
            let a = ((sigma - 1.0) * scale + 1.0) / s2 - t2 * (scale * (sigma * s2 - 3.0 * s2 + 6.0 * sigma - 6.0) + 6.0) / (6.0 * s2 * s2);
            let b = (scale * (0.5 * s2 - sigma + 1.0) - 1.0) / (s2 * sigma) - t2 * (scale * (s2 * s2 - 4.0 * sigma * s2 + 12.0 * s2 - 24.0 * sigma + 24.0) - 24.0) / (24.0 * s2 * s2 * sigma);
            (c, a, b)
        } else {
            let (sin_t, cos_t) = (theta.sin(), theta.cos());
            let denom = s2 + t2;
            let a = (scale * (sigma * sin_t - theta * cos_t) + theta) / (theta * denom);
            let b = (c - (scale * (sigma * cos_t + theta * sin_t) - sigma) / denom) / t2;
            (c, a, b)
        }
    };
    mat3_axpy(mat3_axpy(mat3_scaled_identity(coeff_c), k, coeff_a), k2, coeff_b)
}

/// 🪐 Similarity transform in Sim(3): scale `s`, rotation `r`, translation `t`, so `p ↦ s·r·p + t`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sim3 {
    pub s: f64,
    pub r: So3,
    pub t: [f64; 3],
}

impl Sim3 {
    /// 🪞 Identity similarity.
    pub fn identity() -> Self {
        Self { s: 1.0, r: So3::identity(), t: [0.0; 3] }
    }

    /// 🔗 Group composition `self · other`.
    pub fn compose(&self, other: &Self) -> Self {
        Self { s: self.s * other.s, r: self.r.compose(&other.r), t: vec3_add(vec3_scale(self.r.act(other.t), self.s), self.t) }
    }

    /// ↩️ Inverse similarity.
    pub fn inverse(&self) -> Self {
        let rinv = self.r.inverse();
        let sinv = 1.0 / self.s;
        Self { s: sinv, r: rinv, t: vec3_scale(rinv.act(self.t), -sinv) }
    }

    /// 🎬 Applies the similarity to a point.
    pub fn act(&self, p: [f64; 3]) -> [f64; 3] {
        vec3_add(vec3_scale(self.r.act(p), self.s), self.t)
    }

    /// 🚀 Exponential map of `xi = (rho, phi, sigma)` using the scale-aware `W` integral matrix with small-value Taylor guards.
    pub fn exp(xi: [f64; 7]) -> Self {
        let rho = [xi[0], xi[1], xi[2]];
        let phi = [xi[3], xi[4], xi[5]];
        let sigma = xi[6];
        Self { s: sigma.exp(), r: So3::exp(phi), t: sim3_w(phi, sigma).mul_vec3(rho) }
    }

    /// 🪵 Logarithm map returning `(rho, phi, sigma)` with `rho = W(phi, sigma)⁻¹ · t`.
    pub fn log(&self) -> [f64; 7] {
        let sigma = self.s.ln();
        let phi = self.r.log();
        let rho = mat3_inverse(sim3_w(phi, sigma)).map_or(self.t, |inv| inv.mul_vec3(self.t));
        [rho[0], rho[1], rho[2], phi[0], phi[1], phi[2], sigma]
    }
}
// #endregion 🔖Sim3

// #region 🔖Align
/// 🧷 Umeyama/Kabsch closed-form alignment: least-squares similarity `dst ≈ s·R·src + t` from paired 3D points, `None` for fewer than three pairs, mismatched lengths or rank-deficient (collinear/coincident) configurations.
pub fn umeyama(src: &[[f64; 3]], dst: &[[f64; 3]], with_scale: bool) -> Option<Sim3> {
    let n = src.len();
    if n < 3 || dst.len() != n {
        return None;
    }
    let nf = n as f64;
    let mut mu_s = [0.0; 3];
    let mut mu_d = [0.0; 3];
    for (s, d) in src.iter().zip(dst) {
        for k in 0..3 {
            mu_s[k] += s[k];
            mu_d[k] += d[k];
        }
    }
    for k in 0..3 {
        mu_s[k] /= nf;
        mu_d[k] /= nf;
    }
    let mut var_src = 0.0;
    let mut cov = [[0.0_f64; 3]; 3];
    for (s, d) in src.iter().zip(dst) {
        let sc = vec3d_sub(*s, mu_s);
        let dc = vec3d_sub(*d, mu_d);
        var_src += vec3_dot(sc, sc);
        for r in 0..3 {
            for c in 0..3 {
                cov[r][c] += dc[r] * sc[c];
            }
        }
    }
    var_src /= nf;
    let cov_m = Mat3d { cols: std::array::from_fn(|c| std::array::from_fn(|r| cov[r][c] / nf)) };
    let (u, sigma, v) = svd3(cov_m);
    if var_src < 1e-18 || sigma[0] < 1e-300 || sigma[1] <= sigma[0] * 1e-9 {
        return None;
    }
    let d_sign = if mat3_det(u) * mat3_det(v) < 0.0 { -1.0 } else { 1.0 };
    let rot = Mat3d::from_axes(u.cols[0], u.cols[1], vec3_scale(u.cols[2], d_sign)).mul(v.transpose());
    let scale = if with_scale { (sigma[0] + sigma[1] + d_sign * sigma[2]) / var_src } else { 1.0 };
    let t = vec3d_sub(mu_d, vec3_scale(rot.mul_vec3(mu_s), scale));
    Some(Sim3 { s: scale, r: So3(rot), t })
}
// #endregion 🔖Align

// #region 🔖Interpolate
/// 🎚️ Geodesic interpolation between rigid poses via the relative twist: `a · exp(t · log(a⁻¹ · b))`.
pub fn se3_lerp(a: &Se3, b: &Se3, t: f64) -> Se3 {
    let xi = a.inverse().compose(b).log();
    a.compose(&Se3::exp(xi6_scale(xi, t)))
}

/// 🎢 Cumulative cubic B-spline over timestamped control poses (De Boor form with uniform-knot cumulative basis on tangent increments), clamped to the valid span `[t₁, tₙ₋₂]`; `None` for fewer than four poses.
pub fn se3_spline(poses: &[(f64, Se3)], t: f64) -> Option<Se3> {
    let n = poses.len();
    if n < 4 {
        return None;
    }
    let tc = t.clamp(poses[1].0, poses[n - 2].0);
    let mut i = 1;
    while i < n - 3 && poses[i + 1].0 <= tc {
        i += 1;
    }
    let dt = poses[i + 1].0 - poses[i].0;
    let u = if dt > 1e-300 { ((tc - poses[i].0) / dt).clamp(0.0, 1.0) } else { 0.0 };
    let u2 = u * u;
    let u3 = u2 * u;
    let basis = [(5.0 + 3.0 * u - 3.0 * u2 + u3) / 6.0, (1.0 + 3.0 * u + 3.0 * u2 - 2.0 * u3) / 6.0, u3 / 6.0];
    let mut out = poses[i - 1].1;
    for (step, weight) in basis.iter().enumerate() {
        let increment = poses[i + step - 1].1.inverse().compose(&poses[i + step].1).log();
        out = out.compose(&Se3::exp(xi6_scale(increment, *weight)));
    }
    Some(out)
}
// #endregion 🔖Interpolate

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical_algebra::VecD;
    use std::f64::consts::PI;

    fn lcg(state: &mut u64) -> f64 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((*state >> 11) as f64 / (1_u64 << 53) as f64) * 2.0 - 1.0
    }

    fn vec3_close(a: [f64; 3], b: [f64; 3], tol: f64) -> bool {
        (0..3).all(|k| (a[k] - b[k]).abs() < tol)
    }

    fn vecn_close(a: &[f64], b: &[f64], tol: f64) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < tol)
    }

    fn mat_close(a: &Mat3d, b: &Mat3d, tol: f64) -> bool {
        (0..3).all(|c| (0..3).all(|r| (a.cols[c][r] - b.cols[c][r]).abs() < tol))
    }

    #[test]
    fn so3_exp_log_round_trips_small_moderate_and_near_pi() {
        let axis = vec3d_normalize([1.0, 2.0, -2.0]);
        let cases = [[1e-9, -2e-9, 1.5e-9], [0.3, -0.4, 0.5], [1.2, 0.7, -0.9], vec3_scale(axis, PI - 1e-4), vec3_scale(axis, PI - 1e-6)];
        for w in cases {
            assert!(vec3_close(So3::exp(w).log(), w, 1e-9), "failed for {w:?}");
        }
    }

    #[test]
    fn so3_log_matrix_round_trip_at_pi() {
        let axis = vec3d_normalize([0.3, -0.5, 0.81]);
        let r = So3::exp(vec3_scale(axis, PI));
        let back = So3::exp(r.log());
        assert!(mat_close(&r.0, &back.0, 1e-9));
    }

    #[test]
    fn so3_compose_inverse_and_hat_vee_round_trip() {
        let r = So3::exp([0.4, -0.9, 0.2]);
        assert!(mat_close(&r.compose(&r.inverse()).0, &Mat3d::IDENTITY, 1e-12));
        let w = [0.7, -0.2, 1.4];
        assert!(vec3_close(So3::vee(&So3::hat(w)), w, 0.0_f64.max(1e-15)));
        let v = [0.5, -1.0, 2.0];
        assert!(vec3_close(So3::hat(w).mul_vec3(v), vec3d_cross(w, v), 1e-12));
    }

    #[test]
    fn quat_matrix_round_trip_covers_all_shepperd_branches() {
        let axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], vec3d_normalize([0.6, 0.64, 0.48])];
        for axis in axes {
            for angle in [0.2, PI - 1e-3] {
                let half = angle / 2.0;
                let q = Quatd { w: half.cos(), x: half.sin() * axis[0], y: half.sin() * axis[1], z: half.sin() * axis[2] };
                let m = q.to_mat3d();
                assert!(mat_close(&m, &So3::exp(vec3_scale(axis, angle)).0, 1e-9));
                let q2 = Quatd::from_mat3d(&m);
                assert!((q.dot(q2).abs() - 1.0).abs() < 1e-9, "failed for axis {axis:?} angle {angle}");
            }
        }
    }

    #[test]
    fn quat_rotate_matches_matrix_action() {
        let q = So3::exp([0.5, -0.3, 0.9]).to_quat();
        let v = [1.2, -0.7, 0.4];
        assert!(vec3_close(q.rotate(v), q.to_mat3d().mul_vec3(v), 1e-12));
        assert!(vec3_close(q.conjugate().rotate(q.rotate(v)), v, 1e-12));
    }

    #[test]
    fn quat_slerp_hits_endpoints_and_stays_unit() {
        let a = So3::exp([0.2, 0.0, 0.0]).to_quat();
        let b = So3::exp([0.0, 1.1, 0.4]).to_quat();
        assert!((Quatd::slerp(a, b, 0.0).dot(a).abs() - 1.0).abs() < 1e-12);
        assert!((Quatd::slerp(a, b, 1.0).dot(b).abs() - 1.0).abs() < 1e-12);
        let mid = Quatd::slerp(a, b, 0.5);
        assert!((mid.dot(mid) - 1.0).abs() < 1e-12);
        assert!((mid.dot(a).abs() - mid.dot(b).abs()).abs() < 1e-9);
        let near = So3::exp([0.2 + 1e-9, 0.0, 0.0]).to_quat();
        let lerped = Quatd::slerp(a, near, 0.5);
        assert!((lerped.dot(lerped) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn jl_times_jl_inv_is_identity() {
        for w in [[1e-6, -2e-6, 1e-6], [0.4, -0.2, 0.7], [1.5, 0.9, -1.1]] {
            let product = So3::jl(w).mul(So3::jl_inv(w));
            assert!(mat_close(&product, &Mat3d::IDENTITY, 1e-9), "failed for {w:?}");
        }
    }

    #[test]
    fn se3_exp_log_round_trips() {
        let axis = vec3d_normalize([-0.2, 0.9, 0.4]);
        let near_pi = vec3_scale(axis, PI - 1e-4);
        let cases = [
            [1e-9, -2e-9, 1.5e-9, 2e-9, 1e-9, -1e-9],
            [0.5, -0.3, 0.8, 0.4, 0.2, -0.6],
            [-1.2, 0.7, 0.3, 1.1, -0.8, 0.9],
            [0.6, -0.4, 1.0, near_pi[0], near_pi[1], near_pi[2]],
        ];
        for xi in cases {
            assert!(vecn_close(&Se3::exp(xi).log(), &xi, 1e-9), "failed for {xi:?}");
        }
        let g = Se3::exp(cases[1]);
        let round = g.compose(&g.inverse());
        assert!(mat_close(&round.r.0, &Mat3d::IDENTITY, 1e-12) && vec3_close(round.t, [0.0; 3], 1e-12));
    }

    #[test]
    fn sim3_exp_log_round_trips() {
        let axis = vec3d_normalize([0.5, -0.1, 0.86]);
        let near_pi = vec3_scale(axis, PI - 1e-4);
        let cases = [
            [1e-9, 2e-9, -1e-9, -2e-9, 1e-9, 1e-9, 1e-9],
            [0.5, -0.3, 0.8, 0.4, 0.2, -0.6, 0.0],
            [0.5, -0.3, 0.8, 0.4, 0.2, -0.6, 0.3],
            [-0.9, 0.6, 0.2, 1.2, -0.5, 0.7, -0.4],
            [0.3, 0.8, -0.5, near_pi[0], near_pi[1], near_pi[2], 0.25],
        ];
        for xi in cases {
            assert!(vecn_close(&Sim3::exp(xi).log(), &xi, 1e-9), "failed for {xi:?}");
        }
        let g = Sim3::exp(cases[2]);
        let round = g.compose(&g.inverse());
        assert!((round.s - 1.0).abs() < 1e-12 && mat_close(&round.r.0, &Mat3d::IDENTITY, 1e-12) && vec3_close(round.t, [0.0; 3], 1e-12));
    }

    #[test]
    fn se3_adjoint_matches_conjugation() {
        let g = Se3::exp([0.4, -0.2, 0.3, 0.5, 0.2, -0.4]);
        let xi = [0.3, 0.1, -0.2, 0.25, -0.15, 0.2];
        let adj_xi = g.adjoint().mul_vec(&VecD::from_vec(xi.to_vec()));
        let mut mapped = [0.0; 6];
        for (k, slot) in mapped.iter_mut().enumerate() {
            *slot = adj_xi.get(k);
        }
        let lhs = Se3::exp(mapped);
        let rhs = g.compose(&Se3::exp(xi)).compose(&g.inverse());
        assert!(mat_close(&lhs.r.0, &rhs.r.0, 1e-8));
        assert!(vec3_close(lhs.t, rhs.t, 1e-8));
    }

    #[test]
    fn umeyama_recovers_planted_similarity() {
        let mut state = 12345_u64;
        let src: Vec<[f64; 3]> = (0..10).map(|_| [lcg(&mut state), lcg(&mut state), lcg(&mut state)]).collect();
        let truth = Sim3 { s: 1.7, r: So3::exp([0.3, -0.7, 0.5]), t: [0.4, -1.2, 2.5] };
        let dst: Vec<[f64; 3]> = src.iter().map(|p| truth.act(*p)).collect();
        let sim = umeyama(&src, &dst, true).expect("well-posed alignment");
        assert!((sim.s - truth.s).abs() < 1e-9);
        assert!(mat_close(&sim.r.0, &truth.r.0, 1e-9));
        assert!(vec3_close(sim.t, truth.t, 1e-9));
        for (s, d) in src.iter().zip(&dst) {
            assert!(vec3_close(sim.act(*s), *d, 1e-9));
        }
    }

    #[test]
    fn umeyama_rigid_without_scale() {
        let mut state = 777_u64;
        let src: Vec<[f64; 3]> = (0..10).map(|_| [lcg(&mut state), lcg(&mut state), lcg(&mut state)]).collect();
        let truth = Sim3 { s: 1.0, r: So3::exp([-0.6, 0.2, 0.9]), t: [1.5, 0.3, -0.8] };
        let dst: Vec<[f64; 3]> = src.iter().map(|p| truth.act(*p)).collect();
        let sim = umeyama(&src, &dst, false).expect("well-posed alignment");
        assert!((sim.s - 1.0).abs() < 1e-15);
        assert!(mat_close(&sim.r.0, &truth.r.0, 1e-9));
        assert!(vec3_close(sim.t, truth.t, 1e-9));
    }

    #[test]
    fn umeyama_rejects_degenerate_inputs() {
        assert!(umeyama(&[[0.0; 3], [1.0; 3]], &[[0.0; 3], [1.0; 3]], true).is_none());
        let line: Vec<[f64; 3]> = (0..10).map(|k| vec3_scale([1.0, 1.0, 1.0], k as f64)).collect();
        let shifted: Vec<[f64; 3]> = line.iter().map(|p| vec3_add(*p, [0.5, -0.2, 0.1])).collect();
        assert!(umeyama(&line, &shifted, true).is_none());
        let coincident = vec![[2.0, -1.0, 0.5]; 5];
        assert!(umeyama(&coincident, &coincident, true).is_none());
    }

    #[test]
    fn project_to_so3_restores_perturbed_rotation() {
        let r_true = So3::exp([0.4, -0.8, 0.3]);
        assert!(mat_close(&So3::project_to_so3(&r_true.0).0, &r_true.0, 1e-9));
        let mut state = 99_u64;
        let noisy = Mat3d { cols: std::array::from_fn(|c| std::array::from_fn(|r| r_true.0.cols[c][r] + 0.05 * lcg(&mut state))) };
        let projected = So3::project_to_so3(&noisy);
        assert!(mat_close(&projected.0.transpose().mul(projected.0), &Mat3d::IDENTITY, 1e-9));
        assert!((mat3_det(projected.0) - 1.0).abs() < 1e-9);
        assert!(mat_close(&projected.0, &r_true.0, 0.15));
    }

    #[test]
    fn se3_lerp_endpoints_and_midpoint() {
        let a = Se3::exp([0.5, -0.3, 0.8, 0.4, 0.2, -0.6]);
        let b = Se3::exp([-0.9, 0.6, 0.2, 1.2, -0.5, 0.7]);
        let at0 = se3_lerp(&a, &b, 0.0);
        let at1 = se3_lerp(&a, &b, 1.0);
        assert!(mat_close(&at0.r.0, &a.r.0, 1e-9) && vec3_close(at0.t, a.t, 1e-9));
        assert!(mat_close(&at1.r.0, &b.r.0, 1e-9) && vec3_close(at1.t, b.t, 1e-9));
        let mid = se3_lerp(&a, &b, 0.5);
        let rel_full = a.inverse().compose(&b).log();
        let rel_half = a.inverse().compose(&mid).log();
        assert!(vecn_close(&rel_half, &xi6_scale(rel_full, 0.5), 1e-9));
    }

    #[test]
    fn se3_spline_tracks_constant_velocity() {
        let v = [0.1, -0.05, 0.2, 0.3, 0.1, -0.2];
        let poses: Vec<(f64, Se3)> = (0..7).map(|k| (k as f64, Se3::exp(xi6_scale(v, k as f64)))).collect();
        for t in [1.0, 2.5, 4.7, 5.0] {
            let spline = se3_spline(&poses, t).expect("enough poses");
            let exact = Se3::exp(xi6_scale(v, t));
            assert!(mat_close(&spline.r.0, &exact.r.0, 1e-8), "failed at t = {t}");
            assert!(vec3_close(spline.t, exact.t, 1e-8), "failed at t = {t}");
        }
        let clamped = se3_spline(&poses, -3.0).expect("enough poses");
        let start = Se3::exp(v);
        assert!(mat_close(&clamped.r.0, &start.r.0, 1e-8) && vec3_close(clamped.t, start.t, 1e-8));
        assert!(se3_spline(&poses[0..3], 1.0).is_none());
    }
}
// #endregion 🔖Tests
