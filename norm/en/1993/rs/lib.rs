//! 🔩 EN 1993 design of steel structures.

use norm_core::{AnnexChoice, CheckReport, CheckResult, ClauseId, Quantity};

pub mod na_de {
    pub use norm_en_1990::na_de::NaDe;
}

macro_rules! steel_section_check {
    ($mod:ident, $part:expr, $section:expr) => {
        pub mod $mod {
            use super::*;

            pub fn check_cross_section(n_ed_kn: f64, n_rd_kn: f64, annex: AnnexChoice) -> CheckResult {
                CheckResult::from_utilization(
                    ClauseId::new("EN 1993", $part, $section),
                    Quantity::force_kn(n_ed_kn),
                    Quantity::force_kn(n_rd_kn),
                    "cross-section ULS",
                    annex,
                )
            }

            pub fn check_member_buckling(n_ed: f64, n_b_rd: f64, annex: AnnexChoice) -> CheckResult {
                CheckResult::from_utilization(
                    ClauseId::new("EN 1993", $part, "buckling"),
                    Quantity::force_kn(n_ed),
                    Quantity::force_kn(n_b_rd),
                    "member buckling",
                    annex,
                )
            }
        }
    };
}

// #region 🔖Part1
steel_section_check!(part_1_1, "-1-1", "§6.2");
steel_section_check!(part_1_2, "-1-2", "fire");
steel_section_check!(part_1_3, "-1-3", "fatigue");
steel_section_check!(part_1_4, "-1-4", "silos");
steel_section_check!(part_1_5, "-1-5", "piling");
steel_section_check!(part_1_6, "-1-6", "crane");
steel_section_check!(part_1_7, "-1-7", "aluminium");
steel_section_check!(part_1_8, "-1-8", "joints");
steel_section_check!(part_1_9, "-1-9", "tension");
steel_section_check!(part_1_10, "-1-10", "material");
steel_section_check!(part_1_11, "-1-11", "hollow");
steel_section_check!(part_1_12, "-1-12", "high_strength");
// #endregion 🔖Part1

// #region 🔖Part2
pub mod part_2 {
    use super::*;

    pub fn check_steel_bridge(n_ed: f64, n_rd: f64) -> CheckResult {
        part_1_1::check_cross_section(n_ed, n_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part2

// #region 🔖Part3
pub mod part_3 {
    use super::*;

    pub fn check_tower_buckling(n_ed: f64, n_b_rd: f64) -> CheckResult {
        part_1_1::check_member_buckling(n_ed, n_b_rd, AnnexChoice::En)
    }
}
// #endregion 🔖Part3

// #region 🔖Part4
pub mod part_4 {
    use super::*;
}
// #endregion 🔖Part4

// #region 🔖Part5
pub mod part_5 {
    use super::*;
}
// #endregion 🔖Part5

// #region 🔖Part6
pub mod part_6 {
    use super::*;
}
// #endregion 🔖Part6

/// 📐 Plastic section modulus W_pl resistance M_c,Rd [kNm].
pub fn bending_resistance_knm(w_pl_mm3: f64, f_y_mpa: f64) -> f64 {
    w_pl_mm3 * f_y_mpa / 1.15 / 1_000_000.0
}

/// 📐 Compression buckling N_b,Rd [kN] simplified.
pub fn buckling_resistance_kn(a_mm2: f64, f_y_mpa: f64, chi: f64) -> f64 {
    chi * a_mm2 * f_y_mpa / 1.15 / 1000.0
}

/// 📋 I-section member check.
pub fn check_steel_member(
    n_ed_kn: f64,
    m_ed_knm: f64,
    a_mm2: f64,
    w_pl_mm3: f64,
    f_y_mpa: f64,
    chi: f64,
) -> CheckReport {
    let annex = AnnexChoice::De;
    let n_rd = a_mm2 * f_y_mpa / 1.15 / 1000.0;
    let n_b_rd = buckling_resistance_kn(a_mm2, f_y_mpa, chi);
    let m_rd = bending_resistance_knm(w_pl_mm3, f_y_mpa);
    let mut report = CheckReport::default();
    report.push(part_1_1::check_cross_section(n_ed_kn, n_rd, annex));
    report.push(part_1_1::check_member_buckling(n_ed_kn, n_b_rd, annex));
    report.push(CheckResult::from_utilization(
        ClauseId::new("EN 1993-1-1", "§6.2.5", "6.2.5"),
        Quantity::new(norm_core::QuantityKind::Moment, m_ed_knm * 1_000_000.0),
        Quantity::new(norm_core::QuantityKind::Moment, m_rd * 1_000_000.0),
        "bending ULS",
        annex,
    ));
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steel_member_e2e() {
        let report = check_steel_member(500.0, 150.0, 5000.0, 500_000.0, 355.0, 0.75);
        assert!(!report.checks.is_empty());
    }
}
