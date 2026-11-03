use crate::{Control, MechanicsError, MechanicsResult};

#[derive(Clone, Debug, PartialEq)]
pub struct Skyline {
    pub n: usize,
    first: Vec<usize>,
    cols: Vec<Vec<f64>>,
}

impl Skyline {
    /// 🏔️ A zero symmetric matrix whose column `j` stores rows `first[j]..=j` (upper triangle, column profile).
    pub fn new(first: Vec<usize>) -> Self {
        let cols = first.iter().enumerate().map(|(j, &f)| vec![0.0; j - f + 1]).collect();
        Self { n: first.len(), first, cols }
    }

    pub fn first(&self) -> &[usize] {
        &self.first
    }

    pub fn zeros_like(&self) -> Self {
        Self::new(self.first.clone())
    }

    pub fn add(&mut self, i: usize, j: usize, v: f64) {
        let (r, c) = if i <= j { (i, j) } else { (j, i) };
        self.cols[c][r - self.first[c]] += v;
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        let (r, c) = if i <= j { (i, j) } else { (j, i) };
        if r < self.first[c] { 0.0 } else { self.cols[c][r - self.first[c]] }
    }

    pub fn mul_vec(&self, x: &[f64]) -> Vec<f64> {
        let mut y = vec![0.0; self.n];
        for j in 0..self.n {
            let f = self.first[j];
            for (o, &a) in self.cols[j].iter().enumerate() {
                let i = f + o;
                y[i] += a * x[j];
                if i != j {
                    y[j] += a * x[i];
                }
            }
        }
        y
    }

    pub fn to_dense(&self) -> super::Dense {
        let mut d = super::Dense::zeros(self.n);
        for j in 0..self.n {
            for i in self.first[j]..=j {
                let v = self.get(i, j);
                d.a[i * self.n + j] = v;
                d.a[j * self.n + i] = v;
            }
        }
        d
    }

    /// 🔨️ In-place `UᵀU` factorisation; the profile is preserved so fill-in is zero by construction.
    pub fn cholesky(&mut self, control: &Control<'_>) -> MechanicsResult<()> {
        for j in 0..self.n {
            if j % 64 == 0 {
                control.check()?;
                control.report("factorise", j as f64 / self.n as f64);
            }
            let fj = self.first[j];
            let (left, right) = self.cols.split_at_mut(j);
            let cj = &mut right[0];
            let orig = cj[j - fj];
            for i in fj..j {
                let fi = self.first[i];
                let ci = &left[i];
                let mut s = cj[i - fj];
                for k in fi.max(fj)..i {
                    s -= ci[k - fi] * cj[k - fj];
                }
                cj[i - fj] = s / ci[i - fi];
            }
            let mut d = orig;
            for k in fj..j {
                d -= cj[k - fj] * cj[k - fj];
            }
            if d <= 1e-12 * orig.abs() || d <= 0.0 || !d.is_finite() {
                return Err(MechanicsError::Singular { dof: j });
            }
            cj[j - fj] = d.sqrt();
        }
        Ok(())
    }

    /// 🪜️ Solve in place using a matrix factorised by [`Skyline::cholesky`].
    pub fn solve(&self, b: &mut [f64]) {
        for j in 0..self.n {
            let f = self.first[j];
            let mut s = b[j];
            for i in f..j {
                s -= self.cols[j][i - f] * b[i];
            }
            b[j] = s / self.cols[j][j - f];
        }
        for j in (0..self.n).rev() {
            let f = self.first[j];
            b[j] /= self.cols[j][j - f];
            let x = b[j];
            for i in f..j {
                b[i] -= self.cols[j][i - f] * x;
            }
        }
    }
}
