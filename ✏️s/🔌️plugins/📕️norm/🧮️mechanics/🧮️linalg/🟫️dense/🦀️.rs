use crate::{Control, MechanicsError, MechanicsResult};

#[derive(Clone, Debug, PartialEq)]
pub struct Dense {
    pub n: usize,
    pub a: Vec<f64>,
}

impl Dense {
    /// ⬜️ An `n x n` zero matrix in row-major storage.
    pub fn zeros(n: usize) -> Self {
        Self { n, a: vec![0.0; n * n] }
    }

    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.a[i * self.n + j]
    }

    pub fn add(&mut self, i: usize, j: usize, v: f64) {
        self.a[i * self.n + j] += v;
    }

    pub fn mul_vec(&self, x: &[f64]) -> Vec<f64> {
        (0..self.n).map(|i| (0..self.n).map(|j| self.at(i, j) * x[j]).sum()).collect()
    }

    /// 🔻️ Lower Cholesky factor `L` with `A = L Lᵀ`; fails with the offending DOF when `A` is not positive definite.
    pub fn cholesky(&self, control: &Control<'_>) -> MechanicsResult<Dense> {
        let n = self.n;
        let mut l = Dense::zeros(n);
        for j in 0..n {
            control.check()?;
            let mut d = self.at(j, j);
            for k in 0..j {
                d -= l.at(j, k) * l.at(j, k);
            }
            if d <= 1e-12 * self.at(j, j).abs() || d <= 0.0 || !d.is_finite() {
                return Err(MechanicsError::Singular { dof: j });
            }
            let ljj = d.sqrt();
            l.a[j * n + j] = ljj;
            for i in j + 1..n {
                let mut s = self.at(i, j);
                for k in 0..j {
                    s -= l.at(i, k) * l.at(j, k);
                }
                l.a[i * n + j] = s / ljj;
            }
        }
        Ok(l)
    }

    pub fn forward(&self, b: &mut [f64]) {
        for i in 0..self.n {
            let mut s = b[i];
            for k in 0..i {
                s -= self.at(i, k) * b[k];
            }
            b[i] = s / self.at(i, i);
        }
    }

    pub fn backward_transposed(&self, b: &mut [f64]) {
        for i in (0..self.n).rev() {
            let mut s = b[i];
            for k in i + 1..self.n {
                s -= self.at(k, i) * b[k];
            }
            b[i] = s / self.at(i, i);
        }
    }

    /// 🧷️ Solve `A x = b` in place from the Cholesky factor of `A`.
    pub fn cholesky_solve(&self, b: &mut [f64]) {
        self.forward(b);
        self.backward_transposed(b);
    }
}
