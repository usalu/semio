//! 📏️ Welded steel pipe strain limits shared by the two EN 1998-4:2006 ground movement checks.

/// 🧮 Allowable extreme tensile and compressive strains, expressed as positive fractions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PipeStrainLimits {
    pub tensile: f64,
    pub compressive: f64,
}

/// 📐 Derive the §6.5.2(2)P material and local buckling limits for a circular pipe.
pub fn welded_pipe_strain_limits(radius_m: f64, thickness_m: f64) -> Option<PipeStrainLimits> {
    if !radius_m.is_finite() || !thickness_m.is_finite() || thickness_m <= 0.0 || thickness_m >= radius_m {
        return None;
    }
    let compressive = (0.20 * (thickness_m / radius_m)).min(0.01);
    (compressive > 0.0).then_some(PipeStrainLimits { tensile: 0.03, compressive })
}
