//! 🧱️ EN 1998-3:2005 with AC1, Annex A.4.2 and A.4.3 strengthening models.

/// 🧮️ Applies (A.17)–(A.20) to the capacities of the monolithic section before the overall Annex A factor.
pub fn concrete_jacket(shear_n: f64, moment_nm: f64, yield_rotation: f64, ultimate_rotation: f64) -> Option<[f64; 4]> {
    if ![shear_n, moment_nm, yield_rotation, ultimate_rotation].into_iter().all(positive) || ultimate_rotation < yield_rotation {
        return None;
    }
    let result = [0.9 * shear_n, moment_nm, 1.05 * yield_rotation, ultimate_rotation];
    result.into_iter().all(f64::is_finite).then_some(result)
}

/// 🪢️ The geometry and design strength of a steel jacket operating within its elastic range.
pub struct SteelJacket {
    pub depth_m: f64,
    pub thickness_m: f64,
    pub width_spacing_ratio: f64,
    pub strut_deg: f64,
    pub strap_deg: f64,
    pub design_yield_pa: f64,
}

impl SteelJacket {
    /// 🧮️ The additive steel jacket shear resistance under (A.21), using half the design yield strength.
    pub fn shear_contribution_n(self) -> Option<f64> {
        if ![self.depth_m, self.thickness_m, self.design_yield_pa].into_iter().all(positive)
            || !(0.0 < self.width_spacing_ratio && self.width_spacing_ratio <= 1.0)
            || !(0.0 < self.strut_deg && self.strut_deg < 90.0)
            || !(0.0 < self.strap_deg && self.strap_deg <= 90.0)
        {
            return None;
        }
        let strut = self.strut_deg.to_radians();
        let strap = self.strap_deg.to_radians();
        let force = self.depth_m * self.thickness_m * self.width_spacing_ratio * self.design_yield_pa
            * (strap.sin() / strut.tan() + strap.cos());
        positive(force).then_some(force)
    }
}

/// 📐️ Steel jacket length and two opposing bolt-row counts for the checked loading direction.
pub struct SpliceClamp {
    pub splice_m: f64,
    pub jacket_m: f64,
    pub near_rows: f64,
    pub far_rows: f64,
    pub at_column_base: bool,
    pub base_pattern_confirmed: bool,
}

impl SpliceClamp {
    /// 🧮️ Returns the minimum length and the clamping result under A.4.3.3.
    pub fn evaluate(self) -> Option<(f64, bool)> {
        if ![self.splice_m, self.jacket_m].into_iter().all(positive)
            || ![self.near_rows, self.far_rows].into_iter().all(|n| n.is_finite() && n >= 0.0 && n.fract() == 0.0)
        {
            return None;
        }
        let minimum = 1.5 * self.splice_m;
        let length_passes = self.jacket_m >= minimum || minimum - self.jacket_m <= f64::EPSILON * minimum;
        let bolts_pass = self.near_rows >= 2.0 && self.far_rows >= 2.0 && (!self.at_column_base || self.base_pattern_confirmed);
        minimum.is_finite().then_some((minimum, length_passes && bolts_pass))
    }
}

fn positive(value: f64) -> bool { value.is_finite() && value > 0.0 }
