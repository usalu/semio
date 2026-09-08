
use super::*;

fn sample_actions() -> ActionSet {
    ActionSet { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("wind".into(), 30.0)] }
}

#[semio_framework_async_macros::async_test]
async fn de_na_combination_6_10() {
    let annex = NaDe;
    let actions = sample_actions();
    let ed = combination_6_10(&annex, &actions, 0);
    assert!(ed > 100.0);
    let report = check_design_basis(&annex, &actions, 300.0, 2);
    assert!(report.all_pass());
}

#[semio_framework_async_macros::async_test]
async fn de_combination_6_10a_numeric() {
    let annex = NaDe;
    let actions = sample_actions();
    let ed = combination_6_10a(&annex, &actions, 0);
    assert!((ed - 237.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn de_combination_6_10b_numeric() {
    let annex = NaDe;
    let actions = sample_actions();
    let ed = combination_6_10b(&annex, &actions, 0);
    assert!((ed - 216.75).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn en_combination_6_10a_differs_on_other_psi() {
    let de = NaDe;
    let en = NaEn;
    let actions = ActionSet { g_k: 100.0, q_k: vec![("office".into(), 50.0), ("other".into(), 30.0)] };
    let de_ed = combination_6_10a(&de, &actions, 0);
    let en_ed = combination_6_10a(&en, &actions, 0);
    assert!((de_ed - 246.0).abs() < 1e-9);
    assert!((en_ed - 241.5).abs() < 1e-9);
    assert!(de_ed > en_ed);
}

#[semio_framework_async_macros::async_test]
async fn de_vs_en_congregation_psi_tables() {
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

#[semio_framework_async_macros::async_test]
async fn de_na_gamma_m_and_xi() {
    let annex = NaDe;
    assert!((annex.gamma_m("concrete") - 1.5).abs() < 1e-9);
    assert!((annex.gamma_m("steel") - 1.0).abs() < 1e-9);
    assert!((annex.gamma_m("timber") - 1.3).abs() < 1e-9);
    assert!((annex.gamma_r() - 1.0).abs() < 1e-9);
    assert!((annex.xi("permanent") - 0.85).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn imposed_categories_a_to_h_de() {
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

#[semio_framework_async_macros::async_test]
async fn check_combination_set_covers_uls_and_sls() {
    let annex = NaDe;
    let actions = sample_actions();
    let report = check_combination_set(&annex, DesignSituation::Persistent, &actions, 300.0);
    assert!(report.checks.len() >= 9);
    assert!(report.checks.iter().any(|c| c.clause.section == "6.10a"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.10b"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.16"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.17"));
}

#[semio_framework_async_macros::async_test]
async fn accidental_situation_uses_unit_gamma() {
    let annex = NaDe;
    let actions = sample_actions();
    let persistent = combination_uls(&annex, DesignSituation::Persistent, CombinationRule::Uls610a, &actions, 0);
    let accidental = combination_uls(&annex, DesignSituation::Accidental, CombinationRule::Uls610a, &actions, 0);
    assert!(accidental < persistent);
    assert!((accidental - 168.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn check_design_basis_covers_all_situations() {
    let annex = NaDe;
    let actions = sample_actions();
    let report = check_design_basis(&annex, &actions, 300.0, 2);
    let persistent = check_combination_set(&annex, DesignSituation::Persistent, &actions, 300.0);
    let accidental = check_combination_set(&annex, DesignSituation::Accidental, &actions, 300.0);
    let seismic = check_combination_set(&annex, DesignSituation::Seismic, &actions, 300.0);
    assert_eq!(report.checks.len(), persistent.checks.len() + accidental.checks.len() + seismic.checks.len() + 1);
}

#[semio_framework_async_macros::async_test]
async fn seismic_combination_de_vs_en_diverge_on_other_psi_2() {
    let actions = ActionSet { g_k: 100.0, q_k: vec![("other".into(), 50.0)] };
    let de_ed = combination_6_12b(&NaDe, &actions, 40.0);
    let en_ed = combination_6_12b(&NaEn, &actions, 40.0);
    assert!((de_ed - 165.0).abs() < 1e-9);
    assert!((en_ed - 155.0).abs() < 1e-9);
    assert!(de_ed > en_ed);
}
