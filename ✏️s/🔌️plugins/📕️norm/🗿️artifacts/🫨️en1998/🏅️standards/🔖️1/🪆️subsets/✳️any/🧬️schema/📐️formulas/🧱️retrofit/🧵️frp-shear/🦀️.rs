//! 🧵️ EN 1998-3:2005 Annex A.4.4.2 shear contribution in N and mm.

#[derive(Clone, Copy)]
pub enum Arrangement {
    Closed { corner_radius_mm: f64, web_width_mm: f64, ultimate_mpa: f64 },
    OpenU,
    Sides,
}

#[derive(Clone, Copy)]
pub enum Layout {
    Strips { width_mm: f64, spacing_mm: f64 },
    Sheet { web_height_mm: f64 },
}

#[derive(Clone, Copy)]
pub struct FrpRectangular {
    pub depth_mm: f64,
    pub thickness_mm: f64,
    pub strut_deg: f64,
    pub fibre_deg: f64,
    pub modulus_mpa: f64,
    pub tensile_mpa: f64,
    pub debonding_factor: f64,
    pub arrangement: Arrangement,
    pub layout: Layout,
}

pub struct FrpContribution {
    pub bond_length_mm: f64,
    pub debonding_mpa: f64,
    pub effective_mpa: f64,
    pub shear_n: f64,
}

impl FrpRectangular {
    /// 🧮️ Evaluates the applicable rectangular-section branch of (A.22)–(A.31).
    pub fn evaluate(self) -> Option<FrpContribution> {
        if ![self.depth_mm, self.thickness_mm, self.modulus_mpa, self.tensile_mpa, self.debonding_factor].into_iter().all(positive)
            || !(0.0 < self.strut_deg && self.strut_deg < 90.0)
            || !(0.0 < self.fibre_deg && self.fibre_deg <= 90.0)
        {
            return None;
        }
        let z = 0.9 * self.depth_mm;
        let theta = self.strut_deg.to_radians();
        let beta = self.fibre_deg.to_radians();
        let (width, spacing) = match self.layout {
            Layout::Strips { width_mm, spacing_mm } => (width_mm, spacing_mm),
            Layout::Sheet { web_height_mm } => {
                if !positive(web_height_mm) {
                    return None;
                }
                let width = z.min(web_height_mm) * (theta + beta).sin() / theta.sin();
                (width, width)
            }
        };
        if !positive(width) || !positive(spacing) || width > spacing {
            return None;
        }
        let coverage = width / spacing;
        let k_b = (1.5 * (2.0 - coverage) / (1.0 + width / 100.0)).sqrt();
        let fdd = (0.6 * self.modulus_mpa * self.tensile_mpa * k_b / self.thickness_mm).sqrt() / self.debonding_factor;
        let tau_max = 1.8 * self.tensile_mpa * k_b;
        let bond_length = (self.modulus_mpa * self.thickness_mm / (4.0 * tau_max).sqrt()).sqrt();
        let bond_fraction = bond_length * beta.sin() / z;
        let k = 1.0 - 2.0 / std::f64::consts::PI;
        let effective = match self.arrangement {
            Arrangement::Closed { corner_radius_mm, web_width_mm, ultimate_mpa } => {
                let corner = corner_radius_mm / web_width_mm;
                if !positive(web_width_mm) || !positive(ultimate_mpa) || !(0.0..=0.5).contains(&corner) {
                    return None;
                }
                let added = ((0.2 + 1.6 * corner) * ultimate_mpa - fdd).max(0.0);
                fdd * (1.0 - k * bond_fraction / 2.0) + 0.5 * added * (1.0 - bond_fraction)
            }
            Arrangement::OpenU => fdd * (1.0 - k * bond_fraction),
            Arrangement::Sides => {
                let remaining = z - bond_length * beta.sin();
                if !positive(remaining) {
                    return None;
                }
                let extra = (k_b / 3.0) / (fdd / self.modulus_mpa) * beta.sin();
                let equivalent = remaining + extra;
                fdd * equivalent / z * (1.0 - (k * extra / equivalent).sqrt()).powi(2)
            }
        };
        let projection = match self.arrangement {
            Arrangement::Sides => beta.sin() / theta.sin() * coverage,
            _ => coverage.powi(2) * (beta.sin() / theta.tan() + beta.cos()),
        };
        let shear = z * effective * 2.0 * self.thickness_mm * projection;
        [bond_length, fdd, effective, shear].into_iter().all(positive).then_some(FrpContribution {
            bond_length_mm: bond_length,
            debonding_mpa: fdd,
            effective_mpa: effective,
            shear_n: shear,
        })
    }
}

/// 🧮️ Calculates the fully wrapped circular-section shear contribution using (A.32).
pub fn circular_shear_n(diameter_mm: f64, thickness_mm: f64, modulus_mpa: f64) -> Option<f64> {
    if ![diameter_mm, thickness_mm, modulus_mpa].into_iter().all(positive) {
        return None;
    }
    let area = std::f64::consts::PI * diameter_mm.powi(2) / 4.0;
    let ratio = 4.0 * thickness_mm / diameter_mm;
    let shear = 0.5 * area * ratio * modulus_mpa * 0.004;
    positive(shear).then_some(shear)
}

fn positive(value: f64) -> bool { value.is_finite() && value > 0.0 }
