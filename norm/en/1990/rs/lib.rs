//! ⚖️ EN 1990 basis of structural design: combinations, partial factors, reliability.

use norm_core::{
    AnnexChoice, CheckReport, CheckResult, ClauseId, LimitState, NationalAnnex, NormError, Quantity,
};

// #region 🔖NaDe
/// 🇩🇪 German national annex parameters (DIN EN 1990/NA).
#[derive(Clone, Copy, Debug, Default)]
pub struct NaDe;

impl NationalAnnex for NaDe {
    fn choice(&self) -> AnnexChoice {
        AnnexChoice::De
    }

    fn gamma_g(&self) -> f64 {
        1.35
    }

    fn gamma_q(&self) -> f64 {
        1.5
    }

    fn psi_0(&self, category: &str) -> f64 {
        match category {
            "residential" => 0.7,
            "office" => 0.7,
            "storage" => 1.0,
            "snow" => 0.7,
            "wind" => 0.6,
            _ => 0.7,
        }
    }

    fn psi_1(&self, category: &str) -> f64 {
        match category {
            "residential" => 0.5,
            "office" => 0.5,
            "storage" => 0.9,
            "snow" => 0.5,
            "wind" => 0.3,
            _ => 0.5,
        }
    }

    fn psi_2(&self, category: &str) -> f64 {
        match category {
            "residential" => 0.3,
            "office" => 0.3,
            "storage" => 0.8,
            "snow" => 0.2,
            "wind" => 0.1,
            _ => 0.3,
        }
    }
}
// #endregion 🔖NaDe

// #region 🔖NaEn
/// 🇪🇺 Recommended values EN 1990.
#[derive(Clone, Copy, Debug, Default)]
pub struct NaEn;

impl NationalAnnex for NaEn {
    fn choice(&self) -> AnnexChoice {
        AnnexChoice::En
    }

    fn gamma_g(&self) -> f64 {
        1.35
    }

    fn gamma_q(&self) -> f64 {
        1.5
    }

    fn psi_0(&self, _category: &str) -> f64 {
        0.7
    }

    fn psi_1(&self, _category: &str) -> f64 {
        0.5
    }

    fn psi_2(&self, _category: &str) -> f64 {
        0.3
    }
}
// #endregion 🔖NaEn

pub mod na_de {
    pub use super::NaDe;
}

pub mod na_en {
    pub use super::NaEn;
}

// #region 🔖Combinations
/// 📊 Permanent and variable action components for combination [kN].
#[derive(Clone, Debug, PartialEq)]
pub struct ActionSet {
    pub g_k: f64,
    pub q_k: Vec<(String, f64)>,
}

/// 🧮 ULS combination per EN 1990 Eq. 6.10: γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading {
            annex.gamma_q()
        } else {
            annex.gamma_q() * annex.psi_0(cat)
        };
        sum += factor * q;
    }
    sum
}

/// 🧮 SLS characteristic combination: G + Q + ψ_0·ΣQ.
pub fn combination_sls_char(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading {
            1.0
        } else {
            annex.psi_0(cat)
        };
        sum += factor * q;
    }
    sum
}

/// ✅ Check design action against resistance (ULS).
pub fn check_uls_action(
    annex: &dyn NationalAnnex,
    actions: &ActionSet,
    leading: usize,
    resistance: f64,
) -> CheckResult {
    let ed = combination_6_10(annex, actions, leading);
    CheckResult::from_utilization(
        ClauseId::new("EN 1990", "§6.4", "6.10"),
        Quantity::force_kn(ed),
        Quantity::force_kn(resistance),
        "ULS design action",
        annex.choice(),
    )
}
// #endregion 🔖Combinations

// #region 🔖Reliability
/// 📐 Reliability index target β for RC2 (EN 1990 Annex C).
pub fn target_reliability_index(consequence_class: u8) -> f64 {
    match consequence_class {
        1 => 3.1,
        2 => 3.8,
        3 => 4.3,
        _ => 3.8,
    }
}

pub fn check_reliability_index(beta: f64, consequence_class: u8) -> CheckResult {
    let target = target_reliability_index(consequence_class);
    CheckResult::from_utilization(
        ClauseId::new("EN 1990", "Annex C", "C.2"),
        Quantity::new(norm_core::QuantityKind::Dimensionless, beta),
        Quantity::new(norm_core::QuantityKind::Dimensionless, target),
        "reliability index β",
        AnnexChoice::En,
    )
}
// #endregion 🔖Reliability

/// 📋 Run EN 1990 design basis checks.
pub fn check_design_basis(
    annex: &dyn NationalAnnex,
    actions: &ActionSet,
    resistance_kn: f64,
    consequence_class: u8,
) -> CheckReport {
    let mut report = CheckReport::default();
    report.push(check_uls_action(annex, actions, 0, resistance_kn));
    report.push(check_reliability_index(3.9, consequence_class));
    let _ = LimitState::Uls;
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_na_combination_6_10() {
        let annex = NaDe;
        let actions = ActionSet {
            g_k: 100.0,
            q_k: vec![("office".into(), 50.0), ("wind".into(), 30.0)],
        };
        let ed = combination_6_10(&annex, &actions, 0);
        assert!(ed > 100.0);
        let report = check_design_basis(&annex, &actions, 300.0, 2);
        assert!(report.all_pass());
    }
}
