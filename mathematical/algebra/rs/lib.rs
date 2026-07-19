//! 🧮 Linear algebra: 3D vector/matrix math for scenes and cameras, plus a 2D matrix for planar transforms.

// #region 🔖Vec3
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_array(v: [f32; 3]) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics add/sub used pervasively as plain methods (not operator overloads) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics add/sub used pervasively as plain methods (not operator overloads) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len < 1e-8 {
            return Self::ZERO;
        }
        self.scale(1.0 / len)
    }
}
// #endregion 🔖Vec3

// #region 🔖Mat4
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self { cols: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let gl_z = (far + near) / (near - far);
        let gl_w = (2.0 * far * near) / (near - far);
        Self { cols: [[f / aspect, 0.0, 0.0, 0.0], [0.0, f, 0.0, 0.0], [0.0, 0.0, 0.5 * gl_z - 0.5, -1.0], [0.0, 0.0, 0.5 * gl_w, 0.0]] }
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = target.sub(eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Self { cols: [[s.x, u.x, -f.x, 0.0], [s.y, u.y, -f.y, 0.0], [s.z, u.z, -f.z, 0.0], [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn mul(self, other: Self) -> Self {
        let mut out = Self::identity();
        for col in 0..4 {
            for row in 0..4 {
                out.cols[col][row] = self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2] + self.cols[3][row] * other.cols[col][3];
            }
        }
        out
    }

    pub fn transform_point(self, p: Vec3) -> Vec3 {
        let x = p.x * self.cols[0][0] + p.y * self.cols[1][0] + p.z * self.cols[2][0] + self.cols[3][0];
        let y = p.x * self.cols[0][1] + p.y * self.cols[1][1] + p.z * self.cols[2][1] + self.cols[3][1];
        let z = p.x * self.cols[0][2] + p.y * self.cols[1][2] + p.z * self.cols[2][2] + self.cols[3][2];
        let w = p.x * self.cols[0][3] + p.y * self.cols[1][3] + p.z * self.cols[2][3] + self.cols[3][3];
        if w.abs() < 1e-8 {
            return Vec3::new(x, y, z);
        }
        Vec3::new(x / w, y / w, z / w)
    }

    pub fn transform_direction(self, dir: Vec3) -> Vec3 {
        let x = dir.x * self.cols[0][0] + dir.y * self.cols[1][0] + dir.z * self.cols[2][0];
        let y = dir.x * self.cols[0][1] + dir.y * self.cols[1][1] + dir.z * self.cols[2][1];
        let z = dir.x * self.cols[0][2] + dir.y * self.cols[1][2] + dir.z * self.cols[2][2];
        Vec3::new(x, y, z).normalize()
    }

    /// 🧮 Full 4x4 inverse via Gauss-Jordan elimination on an augmented `[A | I]` matrix.
    /// Indexed as `a[row][col]`; `self.cols[c][r]` is read/written as `a[r][c]` throughout.
    pub fn inverse(self) -> Self {
        let mut a = [[0.0f32; 8]; 4];
        for (row, arow) in a.iter_mut().enumerate() {
            for (col, slot) in arow.iter_mut().take(4).enumerate() {
                *slot = self.cols[col][row];
            }
            arow[4 + row] = 1.0;
        }
        for pivot in 0..4 {
            let (mut best_row, mut best_val) = (pivot, a[pivot][pivot].abs());
            for (row, arow) in a.iter().enumerate().skip(pivot + 1) {
                if arow[pivot].abs() > best_val {
                    best_row = row;
                    best_val = arow[pivot].abs();
                }
            }
            if best_val < 1e-8 {
                return Self::identity();
            }
            if best_row != pivot {
                a.swap(pivot, best_row);
            }
            let pivot_value = a[pivot][pivot];
            for slot in a[pivot].iter_mut() {
                *slot /= pivot_value;
            }
            let pivot_row = a[pivot];
            for (row, arow) in a.iter_mut().enumerate() {
                if row == pivot {
                    continue;
                }
                let factor = arow[pivot];
                if factor == 0.0 {
                    continue;
                }
                for (col, slot) in arow.iter_mut().enumerate() {
                    *slot -= factor * pivot_row[col];
                }
            }
        }
        let mut inv = [[0.0f32; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                inv[col][row] = a[row][4 + col];
            }
        }
        Self { cols: inv }
    }

    pub fn translation(v: Vec3) -> Self {
        let mut m = Self::identity();
        m.cols[3] = [v.x, v.y, v.z, 1.0];
        m
    }

    pub fn scale_vec(v: Vec3) -> Self {
        Self { cols: [[v.x, 0.0, 0.0, 0.0], [0.0, v.y, 0.0, 0.0], [0.0, 0.0, v.z, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn from_quat(x: f32, y: f32, z: f32, w: f32) -> Self {
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        Self { cols: [[1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0], [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0], [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn to_cols_array(self) -> [f32; 16] {
        let mut out = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = self.cols[col][row];
            }
        }
        out
    }
}
// #endregion 🔖Mat4

// #region 🔖Mat2
/// 🧮 2x2 matrix, column-major storage; `new(a, b, c, d)` takes row-major entries of `[[a, b], [c, d]]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2 {
    pub cols: [[f64; 2]; 2],
}

impl Mat2 {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0], [0.0, 1.0]] };

    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { cols: [[a, c], [b, d]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1];
        Self { cols: [[entry(0, 0), entry(1, 0)], [entry(0, 1), entry(1, 1)]] }
    }

    pub fn apply(self, v: (f64, f64)) -> (f64, f64) {
        (self.cols[0][0] * v.0 + self.cols[1][0] * v.1, self.cols[0][1] * v.0 + self.cols[1][1] * v.1)
    }

    pub fn det(self) -> f64 {
        self.cols[0][0] * self.cols[1][1] - self.cols[1][0] * self.cols[0][1]
    }

    pub fn trace(self) -> f64 {
        self.cols[0][0] + self.cols[1][1]
    }

    pub fn transpose(self) -> Self {
        Self::new(self.cols[0][0], self.cols[0][1], self.cols[1][0], self.cols[1][1])
    }

    pub fn inverse(self) -> Option<Self> {
        let d = self.det();
        if d.abs() < 1e-12 {
            return None;
        }
        let inv_d = 1.0 / d;
        Some(Self::new(self.cols[1][1] * inv_d, -self.cols[1][0] * inv_d, -self.cols[0][1] * inv_d, self.cols[0][0] * inv_d))
    }

    /// 🧮 Real eigenvalues (if any) of a 2x2 matrix via the characteristic polynomial `λ² - tr·λ + det = 0`.
    pub fn eigenvalues(self) -> Option<(f64, f64)> {
        let t = self.trace();
        let d = self.det();
        let disc = t * t - 4.0 * d;
        if disc < 0.0 {
            return None;
        }
        let sq = disc.sqrt();
        Some(((t + sq) * 0.5, (t - sq) * 0.5))
    }
}
// #endregion 🔖Mat2

// #region 🔖VecD
/// 📏 Heap-allocated f64 vector for element and system-level numerics (loads, displacements, residuals).
#[derive(Clone, Debug, PartialEq)]
pub struct VecD(pub Vec<f64>);

impl VecD {
    pub fn zeros(n: usize) -> Self {
        Self(vec![0.0; n])
    }

    pub fn from_vec(data: Vec<f64>) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, i: usize) -> f64 {
        self.0[i]
    }

    pub fn set(&mut self, i: usize, value: f64) {
        self.0[i] = value;
    }

    pub fn add_at(&mut self, i: usize, value: f64) {
        self.0[i] += value;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn scale(&self, s: f64) -> Self {
        Self(self.0.iter().map(|v| v * s).collect())
    }

    pub fn add(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a + b).collect())
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0.iter()).map(|(a, b)| a - b).collect())
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn norm_inf(&self) -> f64 {
        self.0.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()))
    }
}
// #endregion 🔖VecD

// #region 🔖MatD
/// 🧮 Dynamic dense f64 matrix, row-major storage; sized for element stiffness matrices and small global systems.
#[derive(Clone, Debug, PartialEq)]
pub struct MatD {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl MatD {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![0.0; rows * cols] }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn add_at(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    pub fn matmul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows, "matmul dimension mismatch");
        let mut out = Self::zeros(self.rows, other.cols);
        for row in 0..self.rows {
            for k in 0..self.cols {
                let a = self.get(row, k);
                if a == 0.0 {
                    continue;
                }
                for col in 0..other.cols {
                    out.add_at(row, col, a * other.get(k, col));
                }
            }
        }
        out
    }

    pub fn mul_vec(&self, x: &VecD) -> VecD {
        assert_eq!(self.cols, x.len(), "mul_vec dimension mismatch");
        let mut out = VecD::zeros(self.rows);
        for row in 0..self.rows {
            let mut sum = 0.0;
            for col in 0..self.cols {
                sum += self.get(row, col) * x.get(col);
            }
            out.set(row, sum);
        }
        out
    }

    /// 🧮 `Bᵀ D B` scaled by `weight`, accumulated into `self` — the element-stiffness Gauss-point kernel.
    pub fn add_triple_product(&mut self, b: &MatD, d: &MatD, weight: f64) {
        let btdb = b.transpose().matmul(d).matmul(b);
        for i in 0..self.data.len() {
            self.data[i] += weight * btdb.data[i];
        }
    }

    /// 🧮 Solves `Ax = b` via Gaussian elimination with partial pivoting; `None` if `A` is singular.
    pub fn lu_solve(&self, b: &VecD) -> Option<VecD> {
        assert_eq!(self.rows, self.cols, "lu_solve requires a square matrix");
        assert_eq!(self.rows, b.len(), "lu_solve dimension mismatch");
        let n = self.rows;
        let mut a = self.data.clone();
        let mut x = b.0.clone();
        for pivot in 0..n {
            let (mut best_row, mut best_val) = (pivot, a[pivot * n + pivot].abs());
            for row in (pivot + 1)..n {
                let val = a[row * n + pivot].abs();
                if val > best_val {
                    best_row = row;
                    best_val = val;
                }
            }
            if best_val < 1e-12 {
                return None;
            }
            if best_row != pivot {
                for col in 0..n {
                    a.swap(pivot * n + col, best_row * n + col);
                }
                x.swap(pivot, best_row);
            }
            let pivot_value = a[pivot * n + pivot];
            for row in (pivot + 1)..n {
                let factor = a[row * n + pivot] / pivot_value;
                if factor == 0.0 {
                    continue;
                }
                for col in pivot..n {
                    a[row * n + col] -= factor * a[pivot * n + col];
                }
                x[row] -= factor * x[pivot];
            }
        }
        for row in (0..n).rev() {
            let mut sum = x[row];
            for col in (row + 1)..n {
                sum -= a[row * n + col] * x[col];
            }
            x[row] = sum / a[row * n + row];
        }
        Some(VecD(x))
    }
}
// #endregion 🔖MatD

// #region 🔖Mat3d
/// 🧊 3x3 f64 matrix for element local frames and rotation transforms, column-major storage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3d {
    pub cols: [[f64; 3]; 3],
}

impl Mat3d {
    pub const IDENTITY: Self = Self { cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };

    /// 🧭 Rotation matrix from an orthonormal local basis, columns `(x, y, z)` expressed in global coordinates.
    pub fn from_axes(x: [f64; 3], y: [f64; 3], z: [f64; 3]) -> Self {
        Self { cols: [x, y, z] }
    }

    pub fn transpose(self) -> Self {
        Self { cols: [[self.cols[0][0], self.cols[1][0], self.cols[2][0]], [self.cols[0][1], self.cols[1][1], self.cols[2][1]], [self.cols[0][2], self.cols[1][2], self.cols[2][2]]] }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics mul used pervasively as a plain method (not operator overload) by dependent crates outside this campaign wave's scope; renaming is a breaking API change")]
    pub fn mul(self, other: Self) -> Self {
        let entry = |row: usize, col: usize| self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2];
        Self { cols: [[entry(0, 0), entry(1, 0), entry(2, 0)], [entry(0, 1), entry(1, 1), entry(2, 1)], [entry(0, 2), entry(1, 2), entry(2, 2)]] }
    }

    pub fn mul_vec3(self, v: [f64; 3]) -> [f64; 3] {
        [self.cols[0][0] * v[0] + self.cols[1][0] * v[1] + self.cols[2][0] * v[2], self.cols[0][1] * v[0] + self.cols[1][1] * v[1] + self.cols[2][1] * v[2], self.cols[0][2] * v[0] + self.cols[1][2] * v[1] + self.cols[2][2] * v[2]]
    }
}

pub fn vec3d_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn vec3d_length(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn vec3d_normalize(v: [f64; 3]) -> [f64; 3] {
    let len = vec3d_length(v);
    if len < 1e-12 {
        return [0.0, 0.0, 0.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn vec3d_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
// #endregion 🔖Mat3d

// #region 🔖CsrMatrix
/// 🕸️ Sparse matrix in compressed-sparse-row form, for large graph-adjacency / Laplacian-style numerics where a dense `MatD` would be wasteful.
#[derive(Clone, Debug, PartialEq)]
pub struct CsrMatrix {
    pub rows: usize,
    pub cols: usize,
    pub row_ptr: Vec<usize>,
    pub col_idx: Vec<usize>,
    pub values: Vec<f64>,
}

impl CsrMatrix {
    /// 🕸️ Builds a CSR matrix from `(row, col, value)` triplets, summing duplicate `(row, col)` entries and sorting column indices within each row for deterministic iteration.
    pub fn from_triplets(rows: usize, cols: usize, triplets: &[(usize, usize, f64)]) -> Self {
        let mut by_row: Vec<Vec<(usize, f64)>> = vec![Vec::new(); rows];
        for &(row, col, value) in triplets {
            by_row[row].push((col, value));
        }
        let mut row_ptr = Vec::with_capacity(rows + 1);
        let mut col_idx = Vec::new();
        let mut values = Vec::new();
        row_ptr.push(0);
        for entries in by_row.iter_mut() {
            entries.sort_by_key(|&(col, _)| col);
            let mut last_col: Option<usize> = None;
            for &(col, value) in entries.iter() {
                if last_col == Some(col) {
                    let idx = values.len() - 1;
                    values[idx] += value;
                } else {
                    col_idx.push(col);
                    values.push(value);
                    last_col = Some(col);
                }
            }
            row_ptr.push(col_idx.len());
        }
        Self { rows, cols, row_ptr, col_idx, values }
    }

    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    pub fn row(&self, r: usize) -> impl Iterator<Item = (usize, f64)> + '_ {
        let start = self.row_ptr[r];
        let end = self.row_ptr[r + 1];
        self.col_idx[start..end].iter().copied().zip(self.values[start..end].iter().copied())
    }

    /// 🕸️ Sparse matrix-vector product `A x`.
    pub fn spmv(&self, x: &VecD) -> VecD {
        assert_eq!(self.cols, x.len(), "spmv dimension mismatch");
        let mut out = VecD::zeros(self.rows);
        for row in 0..self.rows {
            let mut sum = 0.0;
            for (col, value) in self.row(row) {
                sum += value * x.get(col);
            }
            out.set(row, sum);
        }
        out
    }

    pub fn transpose(&self) -> Self {
        let mut triplets = Vec::with_capacity(self.nnz());
        for row in 0..self.rows {
            for (col, value) in self.row(row) {
                triplets.push((col, row, value));
            }
        }
        Self::from_triplets(self.cols, self.rows, &triplets)
    }

    pub fn to_dense(&self) -> MatD {
        let mut out = MatD::zeros(self.rows, self.cols);
        for row in 0..self.rows {
            for (col, value) in self.row(row) {
                out.set(row, col, value);
            }
        }
        out
    }
}
// #endregion 🔖CsrMatrix

// #region 🔖AlgebraError
/// ⚠️ Error type for fallible dense/sparse linear-algebra operations: decompositions, iterative solvers, eigensolvers.
#[derive(Clone, Debug, PartialEq)]
pub enum AlgebraError {
    NotPositiveDefinite,
    Singular,
    DimensionMismatch { expected: (usize, usize), got: (usize, usize) },
    PowerIterationFailedConvergence { iterations: usize },
    NotSymmetric,
}

impl std::fmt::Display for AlgebraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPositiveDefinite => write!(f, "matrix is not positive definite"),
            Self::Singular => write!(f, "matrix is singular"),
            Self::DimensionMismatch { expected, got } => write!(f, "dimension mismatch: expected {expected:?}, got {got:?}"),
            Self::PowerIterationFailedConvergence { iterations } => write!(f, "iterative solver failed to converge after {iterations} iterations"),
            Self::NotSymmetric => write!(f, "matrix is not symmetric"),
        }
    }
}

impl std::error::Error for AlgebraError {}
// #endregion 🔖AlgebraError

// #region 🔖Cholesky
/// 🧮 Dense Cholesky decomposition `A = L Lᵀ` of a symmetric positive-definite matrix via the standard column-by-column algorithm; returns `AlgebraError::NotPositiveDefinite` the moment a diagonal pivot goes non-positive.
pub fn cholesky(a: &MatD) -> Result<MatD, AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    let mut l = MatD::zeros(n, n);
    for col in 0..n {
        let mut sum = a.get(col, col);
        for k in 0..col {
            sum -= l.get(col, k) * l.get(col, k);
        }
        if sum <= 0.0 {
            return Err(AlgebraError::NotPositiveDefinite);
        }
        let pivot = sum.sqrt();
        l.set(col, col, pivot);
        for row in (col + 1)..n {
            let mut sum = a.get(row, col);
            for k in 0..col {
                sum -= l.get(row, k) * l.get(col, k);
            }
            l.set(row, col, sum / pivot);
        }
    }
    Ok(l)
}

/// 🧮 Solves `A x = b` given the Cholesky factor `L` of `A` (`A = L Lᵀ`), via forward substitution `L y = b` then back substitution `Lᵀ x = y`.
pub fn cholesky_solve(l: &MatD, b: &VecD) -> VecD {
    let n = l.rows;
    let mut y = vec![0.0; n];
    for row in 0..n {
        let mut sum = b.get(row);
        for k in 0..row {
            sum -= l.get(row, k) * y[k];
        }
        y[row] = sum / l.get(row, row);
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let mut sum = y[row];
        for k in (row + 1)..n {
            sum -= l.get(k, row) * x[k];
        }
        x[row] = sum / l.get(row, row);
    }
    VecD(x)
}
// #endregion 🔖Cholesky

// #region 🔖QrHouseholder
/// 🪞 Dense QR decomposition via Householder reflections; works for any `rows >= cols` matrix, returning orthogonal `Q` (rows x rows) and upper-triangular `R` (rows x cols) with `Q * R == A` up to float tolerance.
pub fn qr_householder(a: &MatD) -> (MatD, MatD) {
    let m = a.rows;
    let n = a.cols;
    let mut r = a.clone();
    let mut q = MatD::identity(m);
    let steps = n.min(m.saturating_sub(1));
    for k in 0..steps {
        let mut norm_x = 0.0;
        for row in k..m {
            norm_x += r.get(row, k) * r.get(row, k);
        }
        norm_x = norm_x.sqrt();
        if norm_x < 1e-14 {
            continue;
        }
        let alpha = if r.get(k, k) >= 0.0 { -norm_x } else { norm_x };
        let mut v = vec![0.0; m - k];
        for row in k..m {
            v[row - k] = r.get(row, k);
        }
        v[0] -= alpha;
        let v_norm: f64 = v.iter().map(|vi| vi * vi).sum();
        if v_norm < 1e-28 {
            continue;
        }
        for col in 0..n {
            let mut dot = 0.0;
            for row in k..m {
                dot += v[row - k] * r.get(row, col);
            }
            let factor = 2.0 * dot / v_norm;
            for row in k..m {
                r.add_at(row, col, -factor * v[row - k]);
            }
        }
        for row in 0..m {
            let mut dot = 0.0;
            for col in k..m {
                dot += q.get(row, col) * v[col - k];
            }
            let factor = 2.0 * dot / v_norm;
            for col in k..m {
                q.add_at(row, col, -factor * v[col - k]);
            }
        }
    }
    (q, r)
}
// #endregion 🔖QrHouseholder

// #region 🔖JacobiEigenSymmetric
/// 🔄 Full eigendecomposition of a small-to-medium dense symmetric matrix via the classical cyclic Jacobi rotation method (O(n³) per sweep — realistic ceiling is a few thousand rows; use `lanczos_extreme_eigen` for large sparse matrices instead). Eigenvalues are returned ascending; eigenvectors are the columns of the returned matrix, matched by index.
pub fn jacobi_eigen_symmetric(a: &MatD, max_sweeps: usize) -> Result<(Vec<f64>, MatD), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    for row in 0..n {
        for col in 0..n {
            if (a.get(row, col) - a.get(col, row)).abs() > 1e-9 {
                return Err(AlgebraError::NotSymmetric);
            }
        }
    }
    let mut m = a.clone();
    let mut v = MatD::identity(n);
    let off_diag_norm = |m: &MatD| -> f64 {
        let mut sum = 0.0;
        for row in 0..n {
            for col in 0..n {
                if row != col {
                    sum += m.get(row, col) * m.get(row, col);
                }
            }
        }
        sum.sqrt()
    };
    let tol = 1e-12 * (1.0 + a.data.iter().map(|x| x.abs()).fold(0.0_f64, f64::max));
    let mut converged = false;
    for _sweep in 0..max_sweeps {
        if off_diag_norm(&m) < tol {
            converged = true;
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                let apq = m.get(p, q);
                if apq.abs() < 1e-300 {
                    continue;
                }
                let app = m.get(p, p);
                let aqq = m.get(q, q);
                let theta = (aqq - app) / (2.0 * apq);
                let t = theta.signum() / (theta.abs() + (1.0 + theta * theta).sqrt());
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                for k in 0..n {
                    let mkp = m.get(k, p);
                    let mkq = m.get(k, q);
                    m.set(k, p, c * mkp - s * mkq);
                    m.set(k, q, s * mkp + c * mkq);
                }
                for k in 0..n {
                    let mpk = m.get(p, k);
                    let mqk = m.get(q, k);
                    m.set(p, k, c * mpk - s * mqk);
                    m.set(q, k, s * mpk + c * mqk);
                }
                for k in 0..n {
                    let vkp = v.get(k, p);
                    let vkq = v.get(k, q);
                    v.set(k, p, c * vkp - s * vkq);
                    v.set(k, q, s * vkp + c * vkq);
                }
            }
        }
    }
    if !converged {
        return Err(AlgebraError::PowerIterationFailedConvergence { iterations: max_sweeps });
    }
    let eigenvalues: Vec<f64> = (0..n).map(|i| m.get(i, i)).collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| eigenvalues[i].partial_cmp(&eigenvalues[j]).unwrap());
    let sorted_vals: Vec<f64> = order.iter().map(|&i| eigenvalues[i]).collect();
    let mut sorted_vecs = MatD::zeros(n, n);
    for (new_col, &old_col) in order.iter().enumerate() {
        for row in 0..n {
            sorted_vecs.set(row, new_col, v.get(row, old_col));
        }
    }
    Ok((sorted_vals, sorted_vecs))
}
// #endregion 🔖JacobiEigenSymmetric

// #region 🔖LanczosExtremeEigen
/// 🕸️ Extreme eigenpairs of a large sparse symmetric matrix via Lanczos iteration with full reorthogonalization: builds a small tridiagonal Krylov-subspace matrix, diagonalizes it with `jacobi_eigen_symmetric`, and lifts the Ritz vectors back to the original space. Feeds algebraic-connectivity / Fiedler-vector algorithms in later NetworkX-parity waves. `largest` selects by eigenvalue magnitude, not sign.
pub fn lanczos_extreme_eigen(a: &CsrMatrix, k: usize, largest: bool, max_iter: usize, seed: u64) -> Result<(Vec<f64>, Vec<VecD>), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    let m = max_iter.min(n);
    let mut rng_state = seed ^ 0x9E37_79B9_7F4A_7C15;
    let mut next_rand = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        (rng_state as f64 / u64::MAX as f64) * 2.0 - 1.0
    };
    let mut v0 = VecD::from_vec((0..n).map(|_| next_rand()).collect());
    if v0.norm2() < 1e-300 {
        v0 = VecD::from_vec(vec![1.0; n]);
    }
    let norm0 = v0.norm2().max(1e-300);
    v0 = v0.scale(1.0 / norm0);
    let mut basis: Vec<VecD> = vec![v0];
    let mut alpha = Vec::with_capacity(m);
    let mut beta = Vec::with_capacity(m);
    for j in 0..m {
        let mut w = a.spmv(&basis[j]);
        alpha.push(basis[j].dot(&w));
        for basis_vec in basis.iter() {
            let proj = basis_vec.dot(&w);
            w = w.sub(&basis_vec.scale(proj));
        }
        for basis_vec in basis.iter() {
            let proj = basis_vec.dot(&w);
            w = w.sub(&basis_vec.scale(proj));
        }
        let bj = w.norm2();
        if j + 1 < m {
            beta.push(bj);
            if bj < 1e-12 {
                break;
            }
            basis.push(w.scale(1.0 / bj));
        }
    }
    let dim = alpha.len();
    let mut t = MatD::zeros(dim, dim);
    for i in 0..dim {
        t.set(i, i, alpha[i]);
        if i + 1 < dim {
            t.set(i, i + 1, beta[i]);
            t.set(i + 1, i, beta[i]);
        }
    }
    let (ritz_vals, ritz_vecs) = jacobi_eigen_symmetric(&t, 500)?;
    let mut order: Vec<usize> = (0..dim).collect();
    order.sort_by(|&x, &y| ritz_vals[y].abs().partial_cmp(&ritz_vals[x].abs()).unwrap());
    if !largest {
        order.reverse();
    }
    let take = k.min(dim);
    let mut eigenvalues = Vec::with_capacity(take);
    let mut eigenvectors = Vec::with_capacity(take);
    for &idx in order.iter().take(take) {
        eigenvalues.push(ritz_vals[idx]);
        let mut vec_full = VecD::zeros(n);
        for (col_i, basis_vec) in basis.iter().enumerate() {
            let coeff = ritz_vecs.get(col_i, idx);
            vec_full = vec_full.add(&basis_vec.scale(coeff));
        }
        eigenvectors.push(vec_full);
    }
    Ok((eigenvalues, eigenvectors))
}
// #endregion 🔖LanczosExtremeEigen

// #region 🔖PowerIteration
/// 🔁 Dominant eigenpair of a large sparse symmetric matrix via power iteration; convergence is measured via the residual `‖A x - λ x‖`. To recover further eigenpairs, deflate by rebuilding `a`'s triplets with `λ v vᵀ` subtracted and re-call — this returns a single eigenpair by design so callers control the deflation loop.
pub fn power_iteration(a: &CsrMatrix, max_iter: usize, tol: f64, seed: u64) -> Result<(f64, VecD), AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    let n = a.rows;
    let mut rng_state = seed ^ 0x2545_F491_4F6C_DD1D;
    let mut next_rand = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        (rng_state as f64 / u64::MAX as f64) * 2.0 - 1.0
    };
    let mut v = VecD::from_vec((0..n).map(|_| next_rand()).collect());
    let norm = v.norm2().max(1e-300);
    v = v.scale(1.0 / norm);
    for _ in 0..max_iter {
        let w = a.spmv(&v);
        let norm_w = w.norm2();
        if norm_w < 1e-300 {
            return Ok((0.0, v));
        }
        let v_next = w.scale(1.0 / norm_w);
        let av_next = a.spmv(&v_next);
        let lambda = v_next.dot(&av_next);
        let residual = av_next.sub(&v_next.scale(lambda)).norm2();
        v = v_next;
        if residual < tol {
            return Ok((lambda, v));
        }
    }
    Err(AlgebraError::PowerIterationFailedConvergence { iterations: max_iter })
}
// #endregion 🔖PowerIteration

// #region 🔖ConjugateGradient
/// 🧮 Conjugate-gradient solver for sparse symmetric positive-definite systems `A x = b`; feeds Laplacian-style solves (current-flow centrality, resistance distance) in later NetworkX-parity waves. Reuses `AlgebraError::PowerIterationFailedConvergence` for the non-convergence case (same "iterative solver ran out of iterations" semantic as power iteration and Jacobi).
pub fn conjugate_gradient(a: &CsrMatrix, b: &VecD, tol: f64, max_iter: usize) -> Result<VecD, AlgebraError> {
    if a.rows != a.cols {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, a.rows), got: (a.rows, a.cols) });
    }
    if a.rows != b.len() {
        return Err(AlgebraError::DimensionMismatch { expected: (a.rows, 1), got: (b.len(), 1) });
    }
    let mut x = VecD::zeros(a.rows);
    let mut r = b.sub(&a.spmv(&x));
    let mut p = r.clone();
    let mut rs_old = r.dot(&r);
    if rs_old.sqrt() < tol {
        return Ok(x);
    }
    for iteration in 0..max_iter {
        let ap = a.spmv(&p);
        let denom = p.dot(&ap);
        if denom.abs() < 1e-300 {
            return Err(AlgebraError::PowerIterationFailedConvergence { iterations: iteration });
        }
        let alpha = rs_old / denom;
        x = x.add(&p.scale(alpha));
        r = r.sub(&ap.scale(alpha));
        let rs_new = r.dot(&r);
        if rs_new.sqrt() < tol {
            return Ok(x);
        }
        p = r.add(&p.scale(rs_new / rs_old));
        rs_old = rs_new;
    }
    Err(AlgebraError::PowerIterationFailedConvergence { iterations: max_iter })
}
// #endregion 🔖ConjugateGradient

// #region 🔖ExpmPade
/// 🧮 Dense matrix exponential via scaling-and-squaring with an order-6 diagonal Padé approximant (coefficients from the closed-form `(2m-j)! m! / ((2m)! j! (m-j)!)` formula, m=6). O(n³) cost — fine for graphs up to a few hundred nodes; callers on bigger graphs should prefer eigen-based communicability once that's built in a later wave.
pub fn expm_pade(a: &MatD) -> MatD {
    assert_eq!(a.rows, a.cols, "expm_pade requires a square matrix");
    let n = a.rows;
    if n == 0 {
        return MatD::zeros(0, 0);
    }
    let one_norm = {
        let mut max_col_sum = 0.0_f64;
        for col in 0..n {
            let mut sum = 0.0;
            for row in 0..n {
                sum += a.get(row, col).abs();
            }
            max_col_sum = max_col_sum.max(sum);
        }
        max_col_sum
    };
    let mut squarings = 0i32;
    let mut scaled_norm = one_norm;
    while scaled_norm > 1.0 {
        squarings += 1;
        scaled_norm /= 2.0;
    }
    let factor = 2f64.powi(squarings);
    let mut a_scaled = a.clone();
    for value in a_scaled.data.iter_mut() {
        *value /= factor;
    }
    const PADE_COEFFS: [f64; 7] = [1.0, 0.5, 5.0 / 44.0, 1.0 / 66.0, 1.0 / 792.0, 1.0 / 15840.0, 1.0 / 665_280.0];
    let mut powers = Vec::with_capacity(7);
    powers.push(MatD::identity(n));
    for i in 1..7 {
        powers.push(powers[i - 1].matmul(&a_scaled));
    }
    let mut n_mat = MatD::zeros(n, n);
    let mut d_mat = MatD::zeros(n, n);
    for (i, coeff) in PADE_COEFFS.iter().enumerate() {
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        for idx in 0..n_mat.data.len() {
            n_mat.data[idx] += coeff * powers[i].data[idx];
            d_mat.data[idx] += sign * coeff * powers[i].data[idx];
        }
    }
    let mut result = MatD::zeros(n, n);
    for col in 0..n {
        let mut rhs = VecD::zeros(n);
        for row in 0..n {
            rhs.set(row, n_mat.get(row, col));
        }
        let x = d_mat.lu_solve(&rhs).expect("Padé denominator of a scaled matrix argument is diagonally dominant and thus non-singular");
        for row in 0..n {
            result.set(row, col, x.get(row));
        }
    }
    for _ in 0..squarings {
        result = result.matmul(&result);
    }
    result
}
// #endregion 🔖ExpmPade

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_normalize_zero_stays_zero() {
        assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
    }

    #[test]
    fn vec3_cross_is_perpendicular() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert!((c.dot(a)).abs() < 1e-6);
        assert!((c.dot(b)).abs() < 1e-6);
        assert!((c.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn mat4_identity_transforms_point_unchanged() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        let out = Mat4::identity().transform_point(p);
        assert!((out.x - p.x).abs() < 1e-6 && (out.y - p.y).abs() < 1e-6 && (out.z - p.z).abs() < 1e-6);
    }

    #[test]
    fn mat4_inverse_round_trips_translation() {
        let m = Mat4::translation(Vec3::new(3.0, -2.0, 5.0));
        let inv = m.inverse();
        let p = Vec3::new(1.0, 1.0, 1.0);
        let round = inv.transform_point(m.transform_point(p));
        assert!((round.x - p.x).abs() < 1e-4 && (round.y - p.y).abs() < 1e-4 && (round.z - p.z).abs() < 1e-4);
    }

    #[test]
    fn mat2_identity_apply_is_noop() {
        assert_eq!(Mat2::IDENTITY.apply((3.0, -4.0)), (3.0, -4.0));
    }

    #[test]
    fn mat2_apply_matches_matrix_vector_multiply() {
        let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
        assert_eq!(m.apply((1.0, 1.0)), (2.0, 3.0));
    }

    #[test]
    fn mat2_det_of_scale_matrix() {
        let m = Mat2::new(2.0, 0.0, 0.0, 3.0);
        assert!((m.det() - 6.0).abs() < 1e-9);
    }

    #[test]
    fn mat2_inverse_round_trips() {
        let m = Mat2::new(2.0, 1.0, 1.0, 3.0);
        let inv = m.inverse().expect("invertible");
        let round = m.mul(inv);
        assert!((round.cols[0][0] - 1.0).abs() < 1e-9);
        assert!((round.cols[1][1] - 1.0).abs() < 1e-9);
        assert!(round.cols[1][0].abs() < 1e-9);
        assert!(round.cols[0][1].abs() < 1e-9);
    }

    #[test]
    fn mat2_singular_matrix_has_no_inverse() {
        let m = Mat2::new(1.0, 2.0, 2.0, 4.0);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn mat2_transpose_swaps_off_diagonal() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let t = m.transpose();
        assert_eq!(t.apply((1.0, 0.0)), (1.0, 2.0));
        assert_eq!(t.apply((0.0, 1.0)), (3.0, 4.0));
    }

    #[test]
    fn mat2_eigenvalues_of_diagonal_matrix_are_the_diagonal() {
        let m = Mat2::new(2.0, 0.0, 0.0, 5.0);
        let (l1, l2) = m.eigenvalues().expect("real eigenvalues");
        let mut vals = [l1, l2];
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((vals[0] - 2.0).abs() < 1e-9);
        assert!((vals[1] - 5.0).abs() < 1e-9);
    }

    #[test]
    fn mat2_rotation_like_matrix_has_complex_eigenvalues() {
        let m = Mat2::new(0.0, -1.0, 1.0, 0.0);
        assert!(m.eigenvalues().is_none());
    }

    #[test]
    fn vecd_dot_and_norm() {
        let a = VecD::from_vec(vec![3.0, 4.0]);
        let b = VecD::from_vec(vec![1.0, 0.0]);
        assert!((a.dot(&b) - 3.0).abs() < 1e-12);
        assert!((a.norm2() - 5.0).abs() < 1e-12);
        assert!((a.norm_inf() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn vecd_add_sub_scale_round_trip() {
        let a = VecD::from_vec(vec![1.0, 2.0, 3.0]);
        let b = VecD::from_vec(vec![0.5, 0.5, 0.5]);
        let sum = a.add(&b);
        let back = sum.sub(&b);
        for i in 0..3 {
            assert!((back.get(i) - a.get(i)).abs() < 1e-12);
        }
        assert!((a.scale(2.0).get(1) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn matd_matmul_identity_is_noop() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 2.0);
        m.set(1, 0, 3.0);
        m.set(1, 1, 4.0);
        let id = MatD::identity(2);
        let out = m.matmul(&id);
        assert_eq!(out, m);
    }

    #[test]
    fn matd_transpose_round_trips() {
        let mut m = MatD::zeros(2, 3);
        for row in 0..2 {
            for col in 0..3 {
                m.set(row, col, (row * 3 + col) as f64);
            }
        }
        assert_eq!(m.transpose().transpose(), m);
    }

    #[test]
    fn matd_mul_vec_matches_matrix_vector_multiply() {
        let mut m = MatD::zeros(2, 2);
        m.set(0, 0, 2.0);
        m.set(0, 1, 0.0);
        m.set(1, 0, 0.0);
        m.set(1, 1, 3.0);
        let x = VecD::from_vec(vec![1.0, 1.0]);
        let y = m.mul_vec(&x);
        assert!((y.get(0) - 2.0).abs() < 1e-12);
        assert!((y.get(1) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn matd_lu_solve_matches_hand_solved_system() {
        // 2x + y = 5, x + 3y = 10  =>  x = 1, y = 3
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 3.0);
        let b = VecD::from_vec(vec![5.0, 10.0]);
        let x = a.lu_solve(&b).expect("solvable");
        assert!((x.get(0) - 1.0).abs() < 1e-9);
        assert!((x.get(1) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn matd_lu_solve_detects_singular_matrix() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 4.0);
        let b = VecD::from_vec(vec![1.0, 2.0]);
        assert!(a.lu_solve(&b).is_none());
    }

    #[test]
    fn matd_add_triple_product_matches_btdb() {
        let mut b = MatD::zeros(1, 2);
        b.set(0, 0, 1.0);
        b.set(0, 1, 2.0);
        let mut d = MatD::zeros(1, 1);
        d.set(0, 0, 3.0);
        let mut ke = MatD::zeros(2, 2);
        ke.add_triple_product(&b, &d, 1.0);
        // Bᵀ D B = [1;2] * 3 * [1 2] = [[3,6],[6,12]]
        assert!((ke.get(0, 0) - 3.0).abs() < 1e-12);
        assert!((ke.get(0, 1) - 6.0).abs() < 1e-12);
        assert!((ke.get(1, 1) - 12.0).abs() < 1e-12);
    }

    #[test]
    fn mat3d_identity_axes_is_identity() {
        let m = Mat3d::from_axes([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(m, Mat3d::IDENTITY);
    }

    #[test]
    fn mat3d_transpose_is_inverse_for_orthonormal_basis() {
        let x = vec3d_normalize([1.0, 1.0, 0.0]);
        let z = [0.0, 0.0, 1.0];
        let y = vec3d_cross(z, x);
        let m = Mat3d::from_axes(x, y, z);
        let round = m.mul(m.transpose());
        for row in 0..3 {
            for col in 0..3 {
                let expected = if row == col { 1.0 } else { 0.0 };
                assert!((round.cols[col][row] - expected).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn mat3d_mul_vec3_transforms_basis_vector() {
        let m = Mat3d::from_axes([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let out = m.mul_vec3([1.0, 0.0, 0.0]);
        assert!((out[0] - 0.0).abs() < 1e-12);
        assert!((out[1] - 1.0).abs() < 1e-12);
        assert!((out[2] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn vec3d_cross_is_perpendicular_to_inputs() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let c = vec3d_cross(a, b);
        assert!((c[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn csr_from_triplets_dedupes_and_sorts() {
        let triplets = [(0, 1, 2.0), (0, 1, 3.0), (0, 0, 1.0), (1, 0, 4.0)];
        let m = CsrMatrix::from_triplets(2, 2, &triplets);
        assert_eq!(m.nnz(), 3);
        let row0: Vec<(usize, f64)> = m.row(0).collect();
        assert_eq!(row0, vec![(0, 1.0), (1, 5.0)]);
    }

    #[test]
    fn csr_spmv_matches_dense_mul_vec() {
        let triplets = [(0, 0, 2.0), (0, 1, 1.0), (1, 1, 3.0)];
        let m = CsrMatrix::from_triplets(2, 2, &triplets);
        let x = VecD::from_vec(vec![1.0, 2.0]);
        let sparse_result = m.spmv(&x);
        let dense_result = m.to_dense().mul_vec(&x);
        assert_eq!(sparse_result, dense_result);
    }

    #[test]
    fn csr_transpose_swaps_rows_and_cols() {
        let triplets = [(0, 1, 5.0)];
        let m = CsrMatrix::from_triplets(2, 3, &triplets);
        let t = m.transpose();
        assert_eq!(t.rows, 3);
        assert_eq!(t.cols, 2);
        assert_eq!(t.row(1).collect::<Vec<_>>(), vec![(0, 5.0)]);
    }

    #[test]
    fn algebra_error_display_is_human_readable() {
        assert_eq!(AlgebraError::Singular.to_string(), "matrix is singular");
        assert_eq!(AlgebraError::NotSymmetric.to_string(), "matrix is not symmetric");
    }

    #[test]
    fn cholesky_matches_hand_solved_spd_system() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 4.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 3.0);
        let l = cholesky(&a).expect("SPD");
        let reconstructed = l.matmul(&l.transpose());
        for row in 0..2 {
            for col in 0..2 {
                assert!((reconstructed.get(row, col) - a.get(row, col)).abs() < 1e-9);
            }
        }
        let b = VecD::from_vec(vec![6.0, 5.0]);
        let x = cholesky_solve(&l, &b);
        let x_lu = a.lu_solve(&b).expect("solvable");
        for i in 0..2 {
            assert!((x.get(i) - x_lu.get(i)).abs() < 1e-9);
        }
    }

    #[test]
    fn cholesky_rejects_non_positive_definite() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 2.0);
        a.set(1, 1, 1.0);
        assert_eq!(cholesky(&a), Err(AlgebraError::NotPositiveDefinite));
    }

    #[test]
    fn qr_householder_reconstructs_and_is_orthogonal() {
        let mut a = MatD::zeros(3, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 0.0);
        a.set(1, 0, 0.0);
        a.set(1, 1, 1.0);
        a.set(2, 0, 1.0);
        a.set(2, 1, 1.0);
        let (q, r) = qr_householder(&a);
        let product = q.matmul(&r);
        for row in 0..3 {
            for col in 0..2 {
                assert!((product.get(row, col) - a.get(row, col)).abs() < 1e-9);
                if row > col {
                    assert!(r.get(row, col).abs() < 1e-9);
                }
            }
        }
        let qt_q = q.transpose().matmul(&q);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((qt_q.get(i, j) - expected).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn jacobi_eigen_diagonal_matrix_returns_sorted_diagonal() {
        let mut a = MatD::zeros(3, 3);
        a.set(0, 0, 5.0);
        a.set(1, 1, 1.0);
        a.set(2, 2, 3.0);
        let (vals, vecs) = jacobi_eigen_symmetric(&a, 100).expect("converges");
        assert!((vals[0] - 1.0).abs() < 1e-9);
        assert!((vals[1] - 3.0).abs() < 1e-9);
        assert!((vals[2] - 5.0).abs() < 1e-9);
        for col in 0..3 {
            let mut norm = 0.0;
            for row in 0..3 {
                norm += vecs.get(row, col) * vecs.get(row, col);
            }
            assert!((norm - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn jacobi_eigen_matches_characteristic_polynomial_roots() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 2.0);
        let (vals, _) = jacobi_eigen_symmetric(&a, 100).expect("converges");
        assert!((vals[0] - 1.0).abs() < 1e-9);
        assert!((vals[1] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn jacobi_eigen_rejects_asymmetric_matrix() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 3.0);
        a.set(1, 1, 4.0);
        assert_eq!(jacobi_eigen_symmetric(&a, 100), Err(AlgebraError::NotSymmetric));
    }

    #[test]
    fn jacobi_eigen_reports_non_convergence_with_zero_sweeps() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 2.0);
        a.set(0, 1, 1.0);
        a.set(1, 0, 1.0);
        a.set(1, 1, 2.0);
        assert_eq!(jacobi_eigen_symmetric(&a, 0), Err(AlgebraError::PowerIterationFailedConvergence { iterations: 0 }));
    }

    #[test]
    fn power_iteration_finds_dominant_eigenpair() {
        let triplets = [(0, 0, 1.0), (1, 1, 5.0), (2, 2, 2.0)];
        let a = CsrMatrix::from_triplets(3, 3, &triplets);
        let (lambda, v) = power_iteration(&a, 200, 1e-10, 7).expect("converges");
        assert!((lambda - 5.0).abs() < 1e-6);
        assert!(v.get(1).abs() > 0.99);
    }

    #[test]
    fn power_iteration_rejects_non_square() {
        let a = CsrMatrix::from_triplets(2, 3, &[]);
        assert!(matches!(power_iteration(&a, 10, 1e-6, 1), Err(AlgebraError::DimensionMismatch { .. })));
    }

    #[test]
    fn conjugate_gradient_matches_hand_solved_system() {
        let triplets = [(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let a = CsrMatrix::from_triplets(2, 2, &triplets);
        let b = VecD::from_vec(vec![5.0, 10.0]);
        let x = conjugate_gradient(&a, &b, 1e-10, 100).expect("converges");
        assert!((x.get(0) - 1.0).abs() < 1e-6);
        assert!((x.get(1) - 3.0).abs() < 1e-6);
    }

    #[test]
    fn conjugate_gradient_reports_non_convergence_with_zero_iterations() {
        let triplets = [(0, 0, 2.0), (1, 1, 2.0)];
        let a = CsrMatrix::from_triplets(2, 2, &triplets);
        let b = VecD::from_vec(vec![1.0, 1.0]);
        assert_eq!(conjugate_gradient(&a, &b, 1e-12, 0), Err(AlgebraError::PowerIterationFailedConvergence { iterations: 0 }));
    }

    #[test]
    fn lanczos_extreme_eigen_finds_largest_on_diagonal_matrix() {
        let triplets = [(0, 0, 1.0), (1, 1, 4.0), (2, 2, 2.0), (3, 3, 3.0)];
        let a = CsrMatrix::from_triplets(4, 4, &triplets);
        let (vals, _) = lanczos_extreme_eigen(&a, 2, true, 4, 11).expect("converges");
        let mut sorted = vals.clone();
        sorted.sort_by(|x, y| y.partial_cmp(x).unwrap());
        assert!((sorted[0] - 4.0).abs() < 1e-6);
        assert!((sorted[1] - 3.0).abs() < 1e-6);
    }

    #[test]
    fn lanczos_extreme_eigen_finds_smallest_on_diagonal_matrix() {
        let triplets = [(0, 0, 1.0), (1, 1, 4.0), (2, 2, 2.0), (3, 3, 3.0)];
        let a = CsrMatrix::from_triplets(4, 4, &triplets);
        let (vals, _) = lanczos_extreme_eigen(&a, 2, false, 4, 11).expect("converges");
        let mut sorted = vals.clone();
        sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
        assert!((sorted[0] - 1.0).abs() < 1e-6);
        assert!((sorted[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn lanczos_extreme_eigen_rejects_non_square() {
        let a = CsrMatrix::from_triplets(2, 3, &[]);
        assert!(matches!(lanczos_extreme_eigen(&a, 1, true, 5, 1), Err(AlgebraError::DimensionMismatch { .. })));
    }

    #[test]
    fn expm_pade_of_zero_matrix_is_identity() {
        let z = MatD::zeros(3, 3);
        let e = expm_pade(&z);
        assert_eq!(e, MatD::identity(3));
    }

    #[test]
    fn expm_pade_of_diagonal_matrix_is_elementwise_exp() {
        let mut a = MatD::zeros(2, 2);
        a.set(0, 0, 1.0);
        a.set(1, 1, 2.0);
        let e = expm_pade(&a);
        assert!((e.get(0, 0) - 1.0_f64.exp()).abs() < 1e-8);
        assert!((e.get(1, 1) - 2.0_f64.exp()).abs() < 1e-8);
        assert!(e.get(0, 1).abs() < 1e-12);
        assert!(e.get(1, 0).abs() < 1e-12);
    }

    fn tridiagonal_spd(n: usize) -> CsrMatrix {
        let mut triplets = Vec::new();
        for i in 0..n {
            triplets.push((i, i, 2.0));
            if i + 1 < n {
                triplets.push((i, i + 1, -1.0));
                triplets.push((i + 1, i, -1.0));
            }
        }
        CsrMatrix::from_triplets(n, n, &triplets)
    }

    mod long {
        use super::*;

        #[test]
        fn conjugate_gradient_converges_on_tridiagonal_system() {
            let n = 40;
            let a = tridiagonal_spd(n);
            let x_true = VecD::from_vec((0..n).map(|i| (i as f64) + 1.0).collect());
            let b = a.spmv(&x_true);
            let x = conjugate_gradient(&a, &b, 1e-10, 1000).expect("converges");
            for i in 0..n {
                assert!((x.get(i) - x_true.get(i)).abs() < 1e-6);
            }
        }

        #[test]
        fn lanczos_matches_dense_jacobi_on_tridiagonal() {
            let n = 16;
            let a_sparse = tridiagonal_spd(n);
            let a_dense = a_sparse.to_dense();
            let (lanczos_vals, _) = lanczos_extreme_eigen(&a_sparse, 3, true, n, 42).expect("lanczos succeeds");
            let (dense_vals, _) = jacobi_eigen_symmetric(&a_dense, 500).expect("jacobi succeeds");
            let mut dense_top3: Vec<f64> = dense_vals.clone();
            dense_top3.sort_by(|a, b| b.partial_cmp(a).unwrap());
            dense_top3.truncate(3);
            let mut lanczos_sorted = lanczos_vals.clone();
            lanczos_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
            for (l, d) in lanczos_sorted.iter().zip(dense_top3.iter()) {
                assert!((l - d).abs() < 1e-6);
            }
        }
    }
}
// #endregion 🔖Tests
