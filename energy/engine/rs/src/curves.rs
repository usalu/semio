//! 📈 Equipment performance curves: polynomials, biquadratics, triquadratics, lookup tables.

use crate::error::{Diagnostics, Error, Severity};
pub use crate::num::{biquadratic, lerp, poly_eval, LookupTable2D};

// #region 🔖CurveKind
/// 📈 Polynomial curve degree for performance functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CurveDegree {
    Constant,
    Linear,
    Quadratic,
    Cubic,
    Biquadratic,
    Triquadratic,
}
// #endregion 🔖CurveKind

// #region 🔖PerformanceCurve
/// 📈 Part-load performance curve for fans, coils, and plant equipment.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PerformanceCurve {
    Constant(f64),
    Linear { x1: f64, y1: f64, x2: f64, y2: f64 },
    Quadratic { coeffs: [f64; 3] },
    Cubic { coeffs: [f64; 4] },
    Quartic { coeffs: [f64; 5] },
    Biquadratic { coeffs: [f64; 6] },
    Triquadratic { coeffs: [f64; 10] },
    Table(CurveLookupTable2D),
}

impl PerformanceCurve {
    /// 📊 Evaluate 1-D curve at normalized load `x`.
    pub fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Constant(v) => *v,
            Self::Linear { x1, y1, x2, y2 } => lerp(x, *x1, *x2, *y1, *y2),
            Self::Quadratic { coeffs } => poly_eval(coeffs, x),
            Self::Cubic { coeffs } => poly_eval(coeffs, x),
            Self::Quartic { coeffs } => poly_eval(coeffs, x),
            Self::Table(table) => {
                let y_mid = table.inner.y.get(table.inner.y.len() / 2).copied().unwrap_or(0.0);
                table.evaluate(x, y_mid)
            }
            Self::Biquadratic { .. } | Self::Triquadratic { .. } => self.evaluate_2d(x, 0.0),
        }
    }

    /// 📊 Evaluate 2-D biquadratic, triquadratic, or table curve.
    pub fn evaluate_2d(&self, x: f64, y: f64) -> f64 {
        match self {
            Self::Biquadratic { coeffs } => biquadratic(*coeffs, x, y),
            Self::Triquadratic { coeffs } => triquadratic(*coeffs, x, y),
            Self::Table(table) => table.evaluate(x, y),
            other => other.evaluate(x),
        }
    }

    /// 📊 Part-load ratio clamped to [0, 1].
    pub fn part_load(&self, load: f64, rated: f64) -> f64 {
        if rated.abs() < 1e-9 {
            return 0.0;
        }
        (load / rated).clamp(0.0, 1.0)
    }

    /// 📐 Curve polynomial degree.
    pub fn degree(&self) -> CurveDegree {
        match self {
            Self::Constant(_) => CurveDegree::Constant,
            Self::Linear { .. } => CurveDegree::Linear,
            Self::Quadratic { .. } => CurveDegree::Quadratic,
            Self::Cubic { .. } => CurveDegree::Cubic,
            Self::Biquadratic { .. } => CurveDegree::Biquadratic,
            Self::Triquadratic { .. } => CurveDegree::Triquadratic,
            Self::Quartic { .. } => CurveDegree::Cubic,
            Self::Table(_) => CurveDegree::Biquadratic,
        }
    }
}
// #endregion 🔖PerformanceCurve

// #region 🔖Triquadratic
/// 📐 Triquadratic f(x,y) = Σ cᵢⱼ xⁱ yʲ for i+j ≤ 2 plus x²y² cross term.
pub fn triquadratic(c: [f64; 10], x: f64, y: f64) -> f64 {
    c[0] + c[1] * x + c[2] * x * x + c[3] * y + c[4] * y * y + c[5] * x * y + c[6] * x * x * y + c[7] * x * y * y + c[8] * x * x * y * y + c[9] * x * x * x
}
// #endregion 🔖Triquadratic

// #region 🔖LookupWrapper
/// 📊 Validated 2-D lookup table wrapper with named axes.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurveLookupTable2D {
    pub name: String,
    pub inner: LookupTable2D,
}

impl CurveLookupTable2D {
    pub fn new(name: impl Into<String>, inner: LookupTable2D) -> Self {
        Self { name: name.into(), inner }
    }

    pub fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.inner.evaluate(x, y)
    }
}

impl From<LookupTable2D> for CurveLookupTable2D {
    fn from(inner: LookupTable2D) -> Self {
        Self::new("lookup", inner)
    }
}
// #endregion 🔖LookupWrapper

// #region 🔖Validation
/// ✅ Validate curve coefficients and lookup grid consistency.
pub fn validate_curve(curve: &PerformanceCurve) -> Result<(), Diagnostics> {
    let mut diag = Diagnostics::default();
    match curve {
        PerformanceCurve::Quadratic { coeffs } => {
            if coeffs.iter().all(|c| c.abs() < 1e-15) {
                diag.push(Error::severe("quadratic curve has all-zero coefficients"));
            }
        }
        PerformanceCurve::Cubic { coeffs } => {
            if coeffs.iter().all(|c| c.abs() < 1e-15) {
                diag.push(Error::severe("polynomial curve has all-zero coefficients"));
            }
        }
        PerformanceCurve::Quartic { coeffs } => {
            if coeffs.iter().all(|c| c.abs() < 1e-15) {
                diag.push(Error::severe("polynomial curve has all-zero coefficients"));
            }
        }
        PerformanceCurve::Linear { x1, x2, .. } if (x1 - x2).abs() < 1e-12 => {
            diag.push(Error::severe("linear curve has coincident x knots"));
        }
        PerformanceCurve::Table(table) => {
            validate_lookup_table(&table.inner, &mut diag);
        }
        _ => {}
    }
    if diag.messages.iter().any(|m| m.severity == Severity::Severe) {
        Err(diag)
    } else {
        Ok(())
    }
}

/// ✅ Validate lookup table grid dimensions and monotonic axes.
pub fn validate_lookup_table(table: &LookupTable2D, diag: &mut Diagnostics) {
    if table.x.len() < 2 || table.y.len() < 2 {
        diag.push(Error::severe("lookup table must have at least 2 x and 2 y values"));
    }
    if table.values.len() != table.y.len() {
        diag.push(Error::severe("lookup table row count must match y axis length"));
    }
    for (i, row) in table.values.iter().enumerate() {
        if row.len() != table.x.len() {
            diag.push(Error::severe(format!("lookup table row {i} width must match x axis length")));
        }
    }
    if !is_monotonic(&table.x) {
        diag.push(Error::warning("lookup table x axis is not monotonically increasing"));
    }
    if !is_monotonic(&table.y) {
        diag.push(Error::warning("lookup table y axis is not monotonically increasing"));
    }
}

fn is_monotonic(vals: &[f64]) -> bool {
    vals.windows(2).all(|w| w[1] > w[0])
}
// #endregion 🔖Validation

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_curve_midpoint() {
        let c = PerformanceCurve::Linear { x1: 0.0, y1: 0.0, x2: 1.0, y2: 10.0 };
        assert!((c.evaluate(0.5) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn quadratic_curve_evaluates() {
        let c = PerformanceCurve::Quadratic { coeffs: [1.0, 2.0, 3.0] };
        assert!((c.evaluate(2.0) - 17.0).abs() < 1e-9);
    }

    #[test]
    fn biquadratic_at_origin() {
        let c = PerformanceCurve::Biquadratic { coeffs: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
        assert!((c.evaluate_2d(2.0, 3.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn triquadratic_includes_cross_terms() {
        let c = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        assert!((triquadratic(c, 2.0, 3.0) - 6.0).abs() < 1e-9);
    }

    #[test]
    fn lookup_wrapper_evaluates() {
        let table = CurveLookupTable2D::new("test", LookupTable2D { x: vec![0.0, 1.0], y: vec![0.0, 1.0], values: vec![vec![0.0, 10.0], vec![0.0, 20.0]] });
        let curve = PerformanceCurve::Table(table);
        assert!((curve.evaluate_2d(1.0, 1.0) - 20.0).abs() < 1e-9);
    }

    #[test]
    fn validate_rejects_degenerate_linear() {
        let c = PerformanceCurve::Linear { x1: 1.0, y1: 0.0, x2: 1.0, y2: 5.0 };
        assert!(validate_curve(&c).is_err());
    }

    #[test]
    fn validate_accepts_valid_quadratic() {
        let c = PerformanceCurve::Quadratic { coeffs: [0.5, 0.1, 0.0] };
        assert!(validate_curve(&c).is_ok());
    }

    #[test]
    fn part_load_clamps() {
        let c = PerformanceCurve::Constant(1.0);
        assert!((c.part_load(150.0, 100.0) - 1.0).abs() < 1e-9);
    }
}
