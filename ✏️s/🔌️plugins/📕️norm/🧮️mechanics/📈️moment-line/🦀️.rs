//! 📈️ Moment line M(x) of single, cantilever and n-span continuous beams in closed form: uniform line load and point loads per span, solved by the three-moment equation on the free-span moment areas. Bending moments are positive for sagging, loads and reactions positive downward and upward, lengths in m, forces in N, moments in N·m. Companion of [`super::continuous_beam`] (EN 1990 6.4.3.2 envelopes) without a finite-element solve; the Rust tests compare against the numpy slope-deflection oracle `🔮️oracles/📈️moment-line/🐍️.py`.

/// 📥️ A downward point force `force` (N) at the distance `at` (m) from the start of its span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLoad {
    pub at: f64,
    pub force: f64,
}

/// 📥️ The downward loads of one span: a uniform line load `udl` (N/m) and point forces.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpanLoad {
    pub udl: f64,
    pub points: Vec<PointLoad>,
}

/// 🧱️ The support arrangement: pinned at every support, clamped at the first support and pinned on the others, or one clamped span with a free end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeamSupports {
    Pinned,
    ClampedStart,
    Cantilever,
}

/// 📈️ The solved beam: the moment, shear and reactions along `x` measured from the first support.
#[derive(Clone, Debug, PartialEq)]
pub struct MomentLine {
    spans: Vec<f64>,
    loads: Vec<SpanLoad>,
    offsets: Vec<f64>,
    start_moment: Vec<f64>,
    start_shear: Vec<f64>,
}

fn free_terms(length: f64, load: &SpanLoad) -> (f64, f64) {
    let udl = load.udl * length.powi(3) / 24.0;
    load.points.iter().fold((udl, udl), |(right, left), p| {
        let (a, b) = (p.at, length - p.at);
        let area = p.force * a * b / (6.0 * length);
        (right + area * (length + a), left + area * (length + b))
    })
}

fn gauss(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = b.len();
    for col in 0..n {
        let pivot = (col..n).max_by(|&i, &j| a[i][col].abs().total_cmp(&a[j][col].abs())).unwrap_or(col);
        if a[pivot][col].abs() < 1e-300 {
            return Err("the support moments are singular".into());
        }
        a.swap(col, pivot);
        b.swap(col, pivot);
        for row in col + 1..n {
            let factor = a[row][col] / a[col][col];
            for k in col..n {
                a[row][k] -= factor * a[col][k];
            }
            b[row] -= factor * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        x[row] = (b[row] - (row + 1..n).map(|k| a[row][k] * x[k]).sum::<f64>()) / a[row][row];
    }
    Ok(x)
}

impl MomentLine {
    /// 🧮️ Solve the beam; refuses an empty or non-positive span, a load list of another length, a non-finite load, a point load on a support or outside its span (the free end of a cantilever carries a tip load) and a cantilever of more than one span.
    pub fn solve(spans: &[f64], loads: &[SpanLoad], supports: BeamSupports) -> Result<Self, String> {
        let n = spans.len();
        if n == 0 || loads.len() != n {
            return Err("a beam has at least one span and one load record per span".into());
        }
        if spans.iter().any(|l| !(l.is_finite() && *l > 0.0)) {
            return Err("every span length must be finite and positive".into());
        }
        if loads.iter().zip(spans).any(|(load, l)| !load.udl.is_finite() || load.points.iter().any(|p| !p.force.is_finite() || !(p.at > 0.0 && (p.at < *l || (supports == BeamSupports::Cantilever && p.at == *l))))) {
            return Err("loads must be finite and point loads must lie strictly inside their span".into());
        }
        if supports == BeamSupports::Cantilever && n != 1 {
            return Err("a cantilever has exactly one span".into());
        }
        let (start_moment, start_shear) = if supports == BeamSupports::Cantilever {
            let (l, load) = (spans[0], &loads[0]);
            let total = load.udl * l + load.points.iter().map(|p| p.force).sum::<f64>();
            let moment = load.udl * l * l / 2.0 + load.points.iter().map(|p| p.force * p.at).sum::<f64>();
            (vec![-moment], vec![total])
        } else {
            let terms: Vec<(f64, f64)> = spans.iter().zip(loads).map(|(l, load)| free_terms(*l, load)).collect();
            let mut a = vec![vec![0.0; n + 1]; n + 1];
            let mut b = vec![0.0; n + 1];
            if supports == BeamSupports::ClampedStart {
                a[0][0] = 2.0 * spans[0];
                a[0][1] = spans[0];
                b[0] = -6.0 * terms[0].1;
            } else {
                a[0][0] = 1.0;
            }
            a[n][n] = 1.0;
            for i in 1..n {
                a[i][i - 1] = spans[i - 1];
                a[i][i] = 2.0 * (spans[i - 1] + spans[i]);
                a[i][i + 1] = spans[i];
                b[i] = -6.0 * (terms[i - 1].0 + terms[i].1);
            }
            let m = gauss(a, b)?;
            let shear: Vec<f64> = (0..n).map(|j| (m[j + 1] - m[j]) / spans[j] + loads[j].udl * spans[j] / 2.0 + loads[j].points.iter().map(|p| p.force * (spans[j] - p.at) / spans[j]).sum::<f64>()).collect();
            (m[..n].to_vec(), shear)
        };
        let offsets = std::iter::once(0.0).chain(spans.iter().scan(0.0, |sum, l| { *sum += l; Some(*sum) })).collect();
        Ok(Self { spans: spans.to_vec(), loads: loads.to_vec(), offsets, start_moment, start_shear })
    }

    /// 🧮️ A simply supported single span.
    pub fn single(length: f64, load: SpanLoad) -> Result<Self, String> {
        Self::solve(&[length], &[load], BeamSupports::Pinned)
    }

    /// 🧮️ A cantilever clamped at x = 0 with a free end at `length`.
    pub fn cantilever(length: f64, load: SpanLoad) -> Result<Self, String> {
        Self::solve(&[length], &[load], BeamSupports::Cantilever)
    }

    /// 🧮️ A continuous beam on pinned supports, optionally clamped at the first support.
    pub fn continuous(spans: &[f64], loads: &[SpanLoad], clamped_start: bool) -> Result<Self, String> {
        Self::solve(spans, loads, if clamped_start { BeamSupports::ClampedStart } else { BeamSupports::Pinned })
    }

    /// 📏️ The span lengths.
    pub fn spans(&self) -> &[f64] {
        &self.spans
    }

    /// 📏️ The total length.
    pub fn length(&self) -> f64 {
        self.offsets[self.spans.len()]
    }

    fn span_of(&self, x: f64) -> (usize, f64) {
        let x = x.clamp(0.0, self.length());
        let j = (0..self.spans.len()).find(|&j| x <= self.offsets[j + 1]).unwrap_or(self.spans.len() - 1);
        (j, x - self.offsets[j])
    }

    fn local_moment(&self, j: usize, x: f64) -> f64 {
        let load = &self.loads[j];
        self.start_moment[j] + self.start_shear[j] * x - load.udl * x * x / 2.0 - load.points.iter().filter(|p| p.at <= x).map(|p| p.force * (x - p.at)).sum::<f64>()
    }

    /// 📈️ The bending moment M(x) at the distance `x` from the first support (sagging positive, `x` clamped to the beam).
    pub fn moment_at(&self, x: f64) -> f64 {
        let (j, local) = self.span_of(x);
        self.local_moment(j, local)
    }

    /// 📈️ The shear force V(x) (upward on the left face positive): the left limit of a point load, the right limit at a span start.
    pub fn shear_at(&self, x: f64) -> f64 {
        let (j, local) = self.span_of(x);
        let load = &self.loads[j];
        self.start_shear[j] - load.udl * local - load.points.iter().filter(|p| p.at < local).map(|p| p.force).sum::<f64>()
    }

    /// 🧲️ The bending moment over the support `index` (0..=spans, sagging positive; negative where the beam hogs).
    pub fn support_moment(&self, index: usize) -> f64 {
        if index < self.spans.len() {
            self.start_moment[index]
        } else {
            self.local_moment(self.spans.len() - 1, self.spans[self.spans.len() - 1])
        }
    }

    /// 🧲️ The upward reaction of every support (zero at the free end of a cantilever).
    pub fn reactions(&self) -> Vec<f64> {
        let n = self.spans.len();
        let end_shear = |j: usize| self.shear_at(self.offsets[j + 1]);
        let mut out = vec![0.0; n + 1];
        out[0] = self.start_shear[0];
        for j in 0..n {
            out[j + 1] = if j + 1 < n { self.start_shear[j + 1] - end_shear(j) } else { -end_shear(j) };
        }
        out
    }

    /// 🎯️ The smallest and largest bending moment of a span, exact: the span ends, every point load and every zero of the shear.
    pub fn extremes(&self, span: usize) -> (f64, f64) {
        let (length, load) = (self.spans[span], &self.loads[span]);
        let mut points: Vec<PointLoad> = load.points.clone();
        points.sort_by(|p, q| p.at.total_cmp(&q.at));
        let mut candidates = vec![self.local_moment(span, 0.0), self.local_moment(span, length)];
        let (mut shear, mut previous) = (self.start_shear[span], 0.0);
        for stop in points.iter().map(|p| (p.at, p.force)).chain(std::iter::once((length, 0.0))) {
            if load.udl != 0.0 {
                let zero = previous + shear / load.udl;
                if zero > previous && zero < stop.0 {
                    candidates.push(self.local_moment(span, zero));
                }
            }
            candidates.push(self.local_moment(span, stop.0));
            shear -= load.udl * (stop.0 - previous) + stop.1;
            previous = stop.0;
        }
        candidates.into_iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), m| (lo.min(m), hi.max(m)))
    }

    /// 📊️ The moment line sampled at `per_span + 1` equidistant stations of every span as `[x, M]` pairs (a support appears once per adjacent span).
    pub fn diagram(&self, per_span: usize) -> Vec<[f64; 2]> {
        let steps = per_span.max(1);
        (0..self.spans.len()).flat_map(|j| (0..=steps).map(move |k| (j, self.spans[j] * k as f64 / steps as f64))).map(|(j, local)| [self.offsets[j] + local, self.local_moment(j, local)]).collect()
    }
}
