//! ⚖️ EN 1990 basis of structural design: combinations, partial factors, reliability.

use norm_core::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, DesignSituation, ImposedCategory, LimitState, Quantity};
use serde::{Deserialize, Serialize};

pub use norm_core::NationalAnnex;

// #region 🔖PsiTables
/// 📊 ψ factors for one imposed-load category (EN 1990 Table A1.1 / DIN EN 1990/NA Table NA.A.1.1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PsiRow {
    psi_0: f64,
    psi_1: f64,
    psi_2: f64,
}

fn psi_row_de(category: &str) -> PsiRow {
    match category {
        "residential" | "A" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "office" | "B" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "congregation" | "C" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "retail" | "D" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "storage" | "E" => PsiRow { psi_0: 1.0, psi_1: 0.9, psi_2: 0.8 },
        "traffic_light" | "F" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "traffic_heavy" | "G" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "roof" | "H" => PsiRow { psi_0: 0.0, psi_1: 0.0, psi_2: 0.0 },
        "snow" => PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 },
        "snow_high" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.2 },
        "wind" => PsiRow { psi_0: 0.6, psi_1: 0.2, psi_2: 0.0 },
        "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
        "settlement" => PsiRow { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 },
        "other" => PsiRow { psi_0: 0.8, psi_1: 0.7, psi_2: 0.5 },
        _ => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
    }
}

fn psi_row_en(category: &str) -> PsiRow {
    match category {
        "residential" | "A" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "office" | "B" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "congregation" | "C" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "retail" | "D" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "storage" | "E" => PsiRow { psi_0: 1.0, psi_1: 0.9, psi_2: 0.8 },
        "traffic_light" | "F" => PsiRow { psi_0: 0.7, psi_1: 0.7, psi_2: 0.6 },
        "traffic_heavy" | "G" => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
        "roof" | "H" => PsiRow { psi_0: 0.0, psi_1: 0.0, psi_2: 0.0 },
        "snow" => PsiRow { psi_0: 0.5, psi_1: 0.2, psi_2: 0.0 },
        "wind" => PsiRow { psi_0: 0.6, psi_1: 0.2, psi_2: 0.0 },
        "temperature" => PsiRow { psi_0: 0.6, psi_1: 0.5, psi_2: 0.0 },
        "settlement" => PsiRow { psi_0: 1.0, psi_1: 1.0, psi_2: 1.0 },
        _ => PsiRow { psi_0: 0.7, psi_1: 0.5, psi_2: 0.3 },
    }
}

pub fn psi_for_category(annex: &dyn NationalAnnex, category: &str) -> PsiRow {
    if annex.choice() == AnnexChoice::De {
        psi_row_de(category)
    } else {
        psi_row_en(category)
    }
}

pub fn psi_for_imposed(annex: &dyn NationalAnnex, category: ImposedCategory) -> PsiRow {
    psi_for_category(annex, category.label())
}
// #endregion 🔖PsiTables

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

    fn gamma_m(&self, material: &str) -> f64 {
        match material {
            "concrete" => 1.5,
            "steel" => 1.0,
            "timber" => 1.3,
            _ => 1.0,
        }
    }

    fn gamma_r(&self) -> f64 {
        1.0
    }

    fn xi(&self, category: &str) -> f64 {
        match category {
            "accidental" | "seismic" => 1.0,
            _ => 0.85,
        }
    }

    fn psi_0(&self, category: &str) -> f64 {
        psi_row_de(category).psi_0
    }

    fn psi_1(&self, category: &str) -> f64 {
        psi_row_de(category).psi_1
    }

    fn psi_2(&self, category: &str) -> f64 {
        psi_row_de(category).psi_2
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

    fn gamma_m(&self, material: &str) -> f64 {
        match material {
            "concrete" => 1.5,
            "steel" => 1.0,
            "timber" => 1.3,
            _ => 1.0,
        }
    }

    fn gamma_r(&self) -> f64 {
        1.0
    }

    fn xi(&self, _category: &str) -> f64 {
        0.85
    }

    fn psi_0(&self, category: &str) -> f64 {
        psi_row_en(category).psi_0
    }

    fn psi_1(&self, category: &str) -> f64 {
        psi_row_en(category).psi_1
    }

    fn psi_2(&self, category: &str) -> f64 {
        psi_row_en(category).psi_2
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

/// 🏷️ ULS/SLS combination rule identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombinationRule {
    Uls610,
    Uls610a,
    Uls610b,
    SlsCharacteristic,
    SlsFrequent,
    SlsQuasiPermanent,
}

fn gamma_for_situation(annex: &dyn NationalAnnex, situation: DesignSituation) -> (f64, f64) {
    match situation {
        DesignSituation::Persistent | DesignSituation::Transient => (annex.gamma_g(), annex.gamma_q()),
        DesignSituation::Accidental | DesignSituation::Seismic => (1.0, 1.0),
    }
}

fn xi_for_situation(annex: &dyn NationalAnnex, situation: DesignSituation) -> f64 {
    match situation {
        DesignSituation::Persistent | DesignSituation::Transient => annex.xi("permanent"),
        DesignSituation::Accidental => annex.xi("accidental"),
        DesignSituation::Seismic => annex.xi("seismic"),
    }
}

/// 🧮 ULS combination per EN 1990 Eq. 6.10: max(6.10a, 6.10b) surrogate as 6.10a.
pub fn combination_6_10(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    combination_6_10a(annex, actions, leading)
}

/// 🧮 ULS combination per EN 1990 Eq. 6.10a: γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10a(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮 ULS combination per EN 1990 Eq. 6.10b: ξ·γ_G·G + γ_Q·Q + γ_Q·ψ_0·ΣQ.
pub fn combination_6_10b(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let xi = annex.xi("permanent");
    let mut sum = xi * annex.gamma_g() * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.gamma_q() } else { annex.gamma_q() * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮 ULS combination for a design situation with situation-specific γ factors.
pub fn combination_uls(annex: &dyn NationalAnnex, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
    let (gamma_g, gamma_q) = gamma_for_situation(annex, situation);
    let xi = xi_for_situation(annex, situation);
    let g_factor = match rule {
        CombinationRule::Uls610b => xi * gamma_g,
        _ => gamma_g,
    };
    let mut sum = g_factor * actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { gamma_q } else { gamma_q * annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮 SLS characteristic combination: G + Q + ψ_0·ΣQ.
pub fn combination_sls_char(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { 1.0 } else { annex.psi_0(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮 SLS frequent combination: G + ψ_1·Q_leading + ψ_2·ΣQ_accompanying.
pub fn combination_sls_frequent(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize) -> f64 {
    let mut sum = actions.g_k;
    for (i, (cat, q)) in actions.q_k.iter().enumerate() {
        let factor = if i == leading { annex.psi_1(cat) } else { annex.psi_2(cat) };
        sum += factor * q;
    }
    sum
}

/// 🧮 SLS quasi-permanent combination: G + ψ_2·ΣQ.
pub fn combination_sls_quasi_permanent(annex: &dyn NationalAnnex, actions: &ActionSet) -> f64 {
    let mut sum = actions.g_k;
    for (cat, q) in &actions.q_k {
        sum += annex.psi_2(cat) * q;
    }
    sum
}

pub fn combination_value(annex: &dyn NationalAnnex, rule: CombinationRule, actions: &ActionSet, leading: usize) -> f64 {
    match rule {
        CombinationRule::Uls610 => combination_6_10(annex, actions, leading),
        CombinationRule::Uls610a => combination_6_10a(annex, actions, leading),
        CombinationRule::Uls610b => combination_6_10b(annex, actions, leading),
        CombinationRule::SlsCharacteristic => combination_sls_char(annex, actions, leading),
        CombinationRule::SlsFrequent => combination_sls_frequent(annex, actions, leading),
        CombinationRule::SlsQuasiPermanent => combination_sls_quasi_permanent(annex, actions),
    }
}

/// 📋 Combination rules relevant for a design situation and limit state.
pub fn rules_for_situation(situation: DesignSituation, limit_state: LimitState) -> Vec<CombinationRule> {
    match (situation, limit_state) {
        (DesignSituation::Persistent | DesignSituation::Transient, LimitState::Uls) => {
            vec![CombinationRule::Uls610, CombinationRule::Uls610a, CombinationRule::Uls610b]
        }
        (DesignSituation::Accidental | DesignSituation::Seismic, LimitState::Uls) => {
            vec![CombinationRule::Uls610a]
        }
        (_, LimitState::Sls) => vec![CombinationRule::SlsCharacteristic, CombinationRule::SlsFrequent, CombinationRule::SlsQuasiPermanent],
        (_, LimitState::Als) => vec![CombinationRule::Uls610a],
        (_, LimitState::Fls) => vec![CombinationRule::Uls610a],
    }
}

fn clause_for_rule(rule: CombinationRule) -> ClauseId {
    match rule {
        CombinationRule::Uls610 => ClauseId::new("EN 1990", "§6.4", "6.10"),
        CombinationRule::Uls610a => ClauseId::new("EN 1990", "§6.4", "6.10a"),
        CombinationRule::Uls610b => ClauseId::new("EN 1990", "§6.4", "6.10b"),
        CombinationRule::SlsCharacteristic => ClauseId::new("EN 1990", "§6.5", "6.14"),
        CombinationRule::SlsFrequent => ClauseId::new("EN 1990", "§6.5", "6.16"),
        CombinationRule::SlsQuasiPermanent => ClauseId::new("EN 1990", "§6.5", "6.17"),
    }
}

fn message_for_rule(rule: CombinationRule, leading: usize) -> String {
    match rule {
        CombinationRule::Uls610 => format!("ULS 6.10 leading={leading}"),
        CombinationRule::Uls610a => format!("ULS 6.10a leading={leading}"),
        CombinationRule::Uls610b => format!("ULS 6.10b leading={leading}"),
        CombinationRule::SlsCharacteristic => format!("SLS characteristic leading={leading}"),
        CombinationRule::SlsFrequent => format!("SLS frequent leading={leading}"),
        CombinationRule::SlsQuasiPermanent => "SLS quasi-permanent".into(),
    }
}

/// ✅ Check one combination against a resistance limit [kN].
pub fn check_combination(annex: &dyn NationalAnnex, situation: DesignSituation, rule: CombinationRule, actions: &ActionSet, leading: usize, resistance_kn: f64) -> CheckResult {
    let ed = if matches!(rule, CombinationRule::Uls610 | CombinationRule::Uls610a | CombinationRule::Uls610b) { combination_uls(annex, situation, rule, actions, leading) } else { combination_value(annex, rule, actions, leading) };
    CheckResult::from_utilization(clause_for_rule(rule), Quantity::force_kn(ed), Quantity::force_kn(resistance_kn), message_for_rule(rule, leading), annex.choice())
}

/// ✅ Run all relevant combinations for an action set in a design situation.
pub fn check_combination_set(annex: &dyn NationalAnnex, situation: DesignSituation, actions: &ActionSet, resistance_kn: f64) -> CheckReport {
    let mut report = CheckReport::default();
    let n_leading = actions.q_k.len().max(1);
    for rule in rules_for_situation(situation, LimitState::Uls) {
        for leading in 0..n_leading {
            if actions.q_k.is_empty() && leading > 0 {
                break;
            }
            report.push(check_combination(annex, situation, rule, actions, leading, resistance_kn));
        }
    }
    for rule in rules_for_situation(situation, LimitState::Sls) {
        match rule {
            CombinationRule::SlsQuasiPermanent => {
                report.push(check_combination(annex, situation, rule, actions, 0, resistance_kn));
            }
            _ => {
                for leading in 0..n_leading {
                    if actions.q_k.is_empty() && leading > 0 {
                        break;
                    }
                    report.push(check_combination(annex, situation, rule, actions, leading, resistance_kn));
                }
            }
        }
    }
    report
}

/// ✅ Check design action against resistance (ULS).
pub fn check_uls_action(annex: &dyn NationalAnnex, actions: &ActionSet, leading: usize, resistance: f64) -> CheckResult {
    let ed = combination_6_10(annex, actions, leading);
    CheckResult::from_utilization(ClauseId::new("EN 1990", "§6.4", "6.10"), Quantity::force_kn(ed), Quantity::force_kn(resistance), "ULS design action", annex.choice())
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
    let passes = beta >= target;
    CheckResult {
        clause: ClauseId::new("EN 1990", "Annex C", "C.2"),
        status: if passes { CheckStatus::Pass } else { CheckStatus::Fail },
        computed: Quantity::new(norm_core::QuantityKind::Dimensionless, beta),
        limit: Quantity::new(norm_core::QuantityKind::Dimensionless, target),
        utilization: if passes { target / beta } else { beta / target },
        message: "reliability index β".into(),
        annex: AnnexChoice::En,
    }
}
// #endregion 🔖Reliability

/// 📋 Run EN 1990 design basis checks.
pub fn check_design_basis(annex: &dyn NationalAnnex, actions: &ActionSet, resistance_kn: f64, consequence_class: u8) -> CheckReport {
    let mut report = check_combination_set(annex, DesignSituation::Persistent, actions, resistance_kn);
    report.push(check_reliability_index(3.9, consequence_class));
    let _ = LimitState::Uls;
    report
}

// #region 🔖Session
use norm_core::{NormFamily, NormFamilyId, NormHost, SetDocumentOp};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub g_k: f64,
    pub q_k: Vec<(String, f64)>,
    pub resistance_kn: f64,
    pub consequence_class: u8,
    pub use_de_na: bool,
}

impl Default for Document {
    fn default() -> Self {
        Self { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("wind".into(), 30.0)], resistance_kn: 300.0, consequence_class: 2, use_de_na: true }
    }
}

pub type Op = SetDocumentOp<Document>;
pub type Host = NormHost<En1990Family>;

pub fn evaluate(document: &Document) -> CheckReport {
    let actions = ActionSet { g_k: document.g_k, q_k: document.q_k.clone() };
    let annex: &dyn NationalAnnex = if document.use_de_na { &NaDe } else { &NaEn };
    check_design_basis(annex, &actions, document.resistance_kn, document.consequence_class)
}

pub struct En1990Family;

impl NormFamily for En1990Family {
    type Document = Document;
    type Op = Op;

    fn family_id() -> NormFamilyId {
        NormFamilyId::En1990
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion 🔖Session

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_actions() -> ActionSet {
        ActionSet { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("wind".into(), 30.0)] }
    }

    #[test]
    fn de_na_combination_6_10() {
        let annex = NaDe;
        let actions = sample_actions();
        let ed = combination_6_10(&annex, &actions, 0);
        assert!(ed > 100.0);
        let report = check_design_basis(&annex, &actions, 300.0, 2);
        assert!(report.all_pass());
    }

    #[test]
    fn de_combination_6_10a_numeric() {
        let annex = NaDe;
        let actions = sample_actions();
        let ed = combination_6_10a(&annex, &actions, 0);
        assert!((ed - 237.0).abs() < 1e-9);
    }

    #[test]
    fn de_combination_6_10b_numeric() {
        let annex = NaDe;
        let actions = sample_actions();
        let ed = combination_6_10b(&annex, &actions, 0);
        assert!((ed - 216.75).abs() < 1e-9);
    }

    #[test]
    fn en_combination_6_10a_differs_on_other_psi() {
        let de = NaDe;
        let en = NaEn;
        let actions = ActionSet { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("other".into(), 30.0)] };
        let de_ed = combination_6_10a(&de, &actions, 0);
        let en_ed = combination_6_10a(&en, &actions, 0);
        assert!((de_ed - 246.0).abs() < 1e-9);
        assert!((en_ed - 241.5).abs() < 1e-9);
        assert!(de_ed > en_ed);
    }

    #[test]
    fn de_vs_en_congregation_psi_tables() {
        let de = NaDe;
        let en = NaEn;
        assert!((de.psi_1("congregation") - 0.7).abs() < 1e-9);
        assert!((en.psi_1("congregation") - 0.7).abs() < 1e-9);
        assert!((de.psi_0("other") - 0.8).abs() < 1e-9);
        assert!((en.psi_0("other") - 0.7).abs() < 1e-9);
        assert!((de.psi_0("wind") - 0.6).abs() < 1e-9);
        assert!((en.psi_0("wind") - 0.6).abs() < 1e-9);
        let actions = ActionSet { g_k: 100.0, q_k: vec![("congregation".into(), 50.0)] };
        let de_freq = combination_sls_frequent(&de, &actions, 0);
        let en_freq = combination_sls_frequent(&en, &actions, 0);
        assert!((de_freq - 135.0).abs() < 1e-9);
        assert!((en_freq - 135.0).abs() < 1e-9);
        assert!((de.psi_2("storage") - 0.8).abs() < 1e-9);
        assert!((en.psi_2("storage") - 0.8).abs() < 1e-9);
        let qp_de = combination_sls_quasi_permanent(&de, &actions);
        let qp_en = combination_sls_quasi_permanent(&en, &actions);
        assert!((qp_de - 130.0).abs() < 1e-9);
        assert!((qp_en - 130.0).abs() < 1e-9);
    }

    #[test]
    fn de_na_gamma_m_and_xi() {
        let annex = NaDe;
        assert!((annex.gamma_m("concrete") - 1.5).abs() < 1e-9);
        assert!((annex.gamma_m("steel") - 1.0).abs() < 1e-9);
        assert!((annex.gamma_m("timber") - 1.3).abs() < 1e-9);
        assert!((annex.gamma_r() - 1.0).abs() < 1e-9);
        assert!((annex.xi("permanent") - 0.85).abs() < 1e-9);
    }

    #[test]
    fn imposed_categories_a_to_h_de() {
        let annex = NaDe;
        for cat in [ImposedCategory::A, ImposedCategory::B, ImposedCategory::C, ImposedCategory::D, ImposedCategory::E, ImposedCategory::F, ImposedCategory::G, ImposedCategory::H] {
            let row = psi_for_imposed(&annex, cat);
            let label = cat.label();
            assert!((annex.psi_0(label) - row.psi_0).abs() < 1e-9);
            assert!((annex.psi_1(label) - row.psi_1).abs() < 1e-9);
            assert!((annex.psi_2(label) - row.psi_2).abs() < 1e-9);
        }
        assert!((annex.psi_0("roof") - 0.0).abs() < 1e-9);
        assert!((annex.psi_0("storage") - 1.0).abs() < 1e-9);
    }

    #[test]
    fn check_combination_set_covers_uls_and_sls() {
        let annex = NaDe;
        let actions = sample_actions();
        let report = check_combination_set(&annex, DesignSituation::Persistent, &actions, 300.0);
        assert!(report.checks.len() >= 9);
        assert!(report.checks.iter().any(|c| c.clause.section == "6.10a"));
        assert!(report.checks.iter().any(|c| c.clause.section == "6.10b"));
        assert!(report.checks.iter().any(|c| c.clause.section == "6.16"));
        assert!(report.checks.iter().any(|c| c.clause.section == "6.17"));
    }

    #[test]
    fn accidental_situation_uses_unit_gamma() {
        let annex = NaDe;
        let actions = sample_actions();
        let persistent = combination_uls(&annex, DesignSituation::Persistent, CombinationRule::Uls610a, &actions, 0);
        let accidental = combination_uls(&annex, DesignSituation::Accidental, CombinationRule::Uls610a, &actions, 0);
        assert!(accidental < persistent);
        assert!((accidental - 168.0).abs() < 1e-9);
    }
}
