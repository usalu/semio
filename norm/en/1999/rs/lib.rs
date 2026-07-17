//! ✨ EN 1999 design of aluminium structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

macro_rules! alu_part {
    ($mod:ident, $part:expr) => {
        pub mod $mod {
            use super::*;

            pub fn check_cross_section(n_ed: f64, n_rd: f64, annex: AnnexChoice) -> CheckResult {
                CheckResult::from_utilization(
                    ClauseId::new("EN 1999", $part, "§6.2"),
                    Quantity::force_kn(n_ed),
                    Quantity::force_kn(n_rd),
                    "aluminium cross-section ULS",
                    annex,
                )
            }

            pub fn check_buckling(n_ed: f64, n_b_rd: f64, annex: AnnexChoice) -> CheckResult {
                CheckResult::from_utilization(
                    ClauseId::new("EN 1999", $part, "§6.3"),
                    Quantity::force_kn(n_ed),
                    Quantity::force_kn(n_b_rd),
                    "aluminium buckling ULS",
                    annex,
                )
            }
        }
    };
}

// #region 🔖Parts
alu_part!(part_1_1, "-1-1");
alu_part!(part_1_2, "-1-2");
alu_part!(part_1_3, "-1-3");
alu_part!(part_1_4, "-1-4");
alu_part!(part_1_5, "-1-5");
// #endregion 🔖Parts

/// 📐 Aluminium bending resistance M_c,Rd [kNm].
pub fn bending_resistance_knm(w_el_mm3: f64, f_0_2_mpa: f64, gamma_m: f64) -> f64 {
    w_el_mm3 * f_0_2_mpa / gamma_m / 1_000_000.0
}

/// 📐 Buckling resistance with χ factor.
pub fn buckling_resistance_kn(a_mm2: f64, f_0_2_mpa: f64, chi: f64, gamma_m: f64) -> f64 {
    chi * a_mm2 * f_0_2_mpa / gamma_m / 1000.0
}

/// 📋 Aluminium member check.
pub fn check_aluminium_member(
    n_ed_kn: f64,
    m_ed_knm: f64,
    a_mm2: f64,
    w_el_mm3: f64,
    f_0_2: f64,
    chi: f64,
) -> CheckReport {
    let gamma_m = 1.1;
    let annex = AnnexChoice::De;
    let n_rd = a_mm2 * f_0_2 / gamma_m / 1000.0;
    let n_b_rd = buckling_resistance_kn(a_mm2, f_0_2, chi, gamma_m);
    let m_rd = bending_resistance_knm(w_el_mm3, f_0_2, gamma_m);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_buckling(n_ed_kn, n_b_rd, annex));
    report.push(CheckResult::from_utilization(
        ClauseId::new("EN 1999-1-1", "§6.2.5", "6.2.5"),
        Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
        Quantity::new(norm_core::QuantityKind::Moment, m_rd * 1_000_000.0),
        "aluminium bending ULS",
        annex,
    ));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aluminium_member_e2e() {
        let report = check_aluminium_member(80.0, 12.0, 1200.0, 15_000.0, 240.0, 0.8);
        assert!(!report.checks.is_empty());
    }
}
