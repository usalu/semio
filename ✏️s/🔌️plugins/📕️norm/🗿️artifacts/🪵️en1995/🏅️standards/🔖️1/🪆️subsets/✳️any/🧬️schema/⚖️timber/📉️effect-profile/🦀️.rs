//! 📉️ Effect profile of the EN 1995 timber checks: the bending moment, shear force and deflection of a member along its length for its support and its line and point load, normalised so that the peak of every shape is one at the section the check assesses. See [`super`].

use super::*;

const XI_STAR: f64 = 0.421_507_718_9;

/// 📉️ Shapes and load shares of one member under one combination (`s = x / L` over the whole member, two spans for the continuous beam).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Profile {
    support: SupportType,
    span: f64,
    line: f64,
    point: f64,
}

impl Profile {
    /// 🧭️ The profile of `m` under the line load `line` (N/m) and the point load `point` (N); `None` without span or load.
    pub fn of(m: &TimberMember, line: f64, point: f64) -> Option<Self> {
        (m.span_m > 0.0 && (line.abs() > 0.0 || point.abs() > 0.0)).then_some(Self { support: m.support, span: m.span_m, line: line.abs(), point: point.abs() })
    }

    /// 🧭️ The profile of `m` for an action given as the forces `m_nm` (N·m, peak moment) and `v_n` (N, peak shear force): the line load `q` and the point load `P` at the middle of each span that carry exactly these two peaks, solved by statics (simply supported `q = 4 (V L - 2 M) / L²`, `P = 2 V - q L`; cantilever `q = 2 (V L - M) / L²`, `P = V - q L`; two spans `P = 4 (5 M / L - V)`, `q = 8 M / L² - 3 P / (2 L)`); `None` without span or moment and when the forces are no statically admissible line plus midspan load (a negative `q` or `P` beyond rounding).
    pub fn from_forces(m: &TimberMember, m_nm: f64, v_n: f64) -> Option<Self> {
        let (l, mm, v) = (m.span_m, m_nm.abs(), v_n.abs());
        if !(l > 0.0 && mm > 0.0) {
            return None;
        }
        let (q, p) = match m.support {
            SupportType::SimplySupported => {
                let q = 4.0 * (v * l - 2.0 * mm) / (l * l);
                (q, 2.0 * v - q * l)
            }
            SupportType::Cantilever => {
                let q = 2.0 * (v * l - mm) / (l * l);
                (q, v - q * l)
            }
            SupportType::ContinuousTwoSpan => {
                let p = 4.0 * (5.0 * mm / l - v);
                (8.0 * mm / (l * l) - 1.5 * p / l, p)
            }
        };
        let tolerance = 1e-9 * (v + mm / l);
        if q < -tolerance || p < -tolerance {
            return None;
        }
        Self::of(m, q.max(0.0), p.max(0.0))
    }

    /// ⚖️ The peak moments `(line load, point load)` in N·m that the shapes of [`Profile::moment`] are weighted with.
    pub fn moment_weights(&self) -> (f64, f64) {
        let l = self.span;
        match self.support {
            SupportType::SimplySupported => (self.line * l * l / 8.0, self.point * l / 4.0),
            SupportType::Cantilever => (self.line * l * l / 2.0, self.point * l),
            SupportType::ContinuousTwoSpan => (self.line * l * l / 8.0, 3.0 * self.point * l / 16.0),
        }
    }

    /// 📉️ `|M(s)| / |M|max`, the signed shapes of both loads superposed.
    pub fn moment(&self, s: f64) -> f64 {
        let (wu, wp) = self.moment_weights();
        let (u, p) = match self.support {
            SupportType::SimplySupported => (4.0 * s * (1.0 - s), 2.0 * s.min(1.0 - s)),
            SupportType::Cantilever => ((1.0 - s).powi(2), 1.0 - s),
            SupportType::ContinuousTwoSpan => {
                let xi = 2.0 * s.min(1.0 - s);
                (3.0 * xi - 4.0 * xi * xi, 5.0 / 3.0 * xi - 16.0 / 3.0 * (xi - 0.5).max(0.0))
            }
        };
        ((wu * u + wp * p) / (wu + wp)).abs()
    }

    /// ✂️ `|V(s)| / |V|max`.
    pub fn shear(&self, s: f64) -> f64 {
        let l = self.span;
        let (qu, qp, u, p) = match self.support {
            SupportType::SimplySupported => (self.line * l / 2.0, self.point / 2.0, (1.0 - 2.0 * s).abs(), 1.0),
            SupportType::Cantilever => (self.line * l, self.point, 1.0 - s, 1.0),
            SupportType::ContinuousTwoSpan => {
                let xi = 2.0 * s.min(1.0 - s);
                (5.0 * self.line * l / 8.0, 11.0 * self.point / 16.0, (0.375 - xi).abs() / 0.625, if xi < 0.5 { 5.0 / 11.0 } else { 1.0 })
            }
        };
        (qu * u + qp * p) / (qu + qp)
    }

    /// 🌊️ Deflection shape of the line load with peak 1.
    fn line_shape(&self, s: f64) -> f64 {
        match self.support {
            SupportType::SimplySupported => 16.0 / 5.0 * s * (1.0 - 2.0 * s * s + s.powi(3)),
            SupportType::Cantilever => s * s * (6.0 - 4.0 * s + s * s) / 3.0,
            SupportType::ContinuousTwoSpan => {
                let xi = 2.0 * s.min(1.0 - s);
                let g = |x: f64| x * (1.0 - 3.0 * x * x + 2.0 * x.powi(3));
                g(xi) / g(XI_STAR)
            }
        }
    }

    /// 🌊️ Deflection shape of the point load with peak 1.
    fn point_shape(&self, s: f64) -> f64 {
        match self.support {
            SupportType::SimplySupported => {
                let h = s.min(1.0 - s);
                h * (3.0 - 4.0 * h * h)
            }
            SupportType::Cantilever => s * s * (3.0 - s) / 2.0,
            SupportType::ContinuousTwoSpan => self.line_shape(s),
        }
    }

    /// 📍️ Section positions beyond the regular grid where a shape peaks (the field maxima of the continuous beam's deflection).
    pub fn peaks(&self) -> Vec<f64> {
        match self.support {
            SupportType::ContinuousTwoSpan => vec![XI_STAR / 2.0, 1.0 - XI_STAR / 2.0],
            _ => Vec::new(),
        }
    }
}

/// 🌊️ Deflection of `m` at `s` under the line load `q` and the point load `f`, with the peak coefficients of [`deflection_m`] so that the peak equals it exactly.
pub(super) fn deflection_at(m: &TimberMember, props: &TimberProperties, q: f64, f: f64, s: f64) -> f64 {
    let ei = props.e_0_mean * i_m4(m);
    let Some(profile) = Profile::of(m, 1.0, 0.0).filter(|_| ei > 0.0) else { return 0.0 };
    let l = m.span_m;
    let (cq, cf) = match m.support {
        SupportType::SimplySupported => (5.0 / 384.0, 1.0 / 48.0),
        SupportType::Cantilever => (1.0 / 8.0, 1.0 / 3.0),
        SupportType::ContinuousTwoSpan => (1.0 / 185.0, 1.0 / 100.0),
    };
    (q * cq * l.powi(4) * profile.line_shape(s) + f * cf * l.powi(3) * profile.point_shape(s)) / ei
}

/// 🔩️ Shear-lag shares `w_k = cosh(κ (u_k - 1/2)) / Σ w` of the `n` fasteners of one row along the force direction (`u_k = k / (n - 1)`), with `κ` chosen so that the largest share is `1 / n_ef`: the row of `n` fasteners then carries `F` with the effective number `n_ef` of EN 1995-1-1 8.1.2 (shares sum to one).
pub fn row_shares(n: usize, n_ef: f64) -> Vec<f64> {
    let n = n.max(1);
    if n == 1 {
        return vec![1.0];
    }
    let target = (n_ef / n as f64).clamp(1e-3, 1.0);
    let ratio = |kappa: f64| (0..n).map(|k| (kappa * (k as f64 / (n - 1) as f64 - 0.5)).cosh()).sum::<f64>() / n as f64 / (kappa / 2.0).cosh();
    let (mut lo, mut hi) = (0.0, 60.0);
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if ratio(mid) > target { lo = mid } else { hi = mid }
    }
    let kappa = 0.5 * (lo + hi);
    let raw: Vec<f64> = (0..n).map(|k| (kappa * (k as f64 / (n - 1) as f64 - 0.5)).cosh()).collect();
    let total: f64 = raw.iter().sum();
    raw.into_iter().map(|w| w / total).collect()
}
