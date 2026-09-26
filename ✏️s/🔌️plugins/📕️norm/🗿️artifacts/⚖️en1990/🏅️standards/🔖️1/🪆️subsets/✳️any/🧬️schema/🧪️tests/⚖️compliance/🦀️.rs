use super::*;

fn sample_actions() -> ActionSet {
    ActionSet::new(100.0, vec![("office".into(), 50.0), ("wind".into(), 30.0)])
}

#[semio_framework_async_macros::async_test]
async fn de_na_combination_6_10_is_max_of_a_and_b() {
    let annex = NaDe;
    let actions = sample_actions();
    let ed = combination_6_10(&annex, &actions, 0);
    assert!((ed - 237.0).abs() < 1e-9);
    assert!((combination_6_10a(&annex, &actions, 0) - 237.0).abs() < 1e-9);
    assert!((combination_6_10b(&annex, &actions, 0) - 216.75).abs() < 1e-9);
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
    let actions = ActionSet::new(100.0, vec![("office".into(), 50.0), ("other".into(), 30.0)]);
    let de_ed = combination_6_10a(&de, &actions, 0);
    let en_ed = combination_6_10a(&en, &actions, 0);
    assert!((de_ed - 246.0).abs() < 1e-9);
    assert!((en_ed - 241.5).abs() < 1e-9);
    assert!(de_ed > en_ed);
}

#[semio_framework_async_macros::async_test]
async fn de_snow_high_altitude_psi() {
    assert_eq!(resolve_psi_category(AnnexChoice::De, "snow", 1200.0), "snow_high");
    assert_eq!(resolve_psi_category(AnnexChoice::De, "snow", 800.0), "snow");
    assert_eq!(resolve_psi_category(AnnexChoice::En, "snow", 1200.0), "snow");
    assert!((NaDe.psi_0("snow_high") - 0.7).abs() < 1e-9);
    assert!((NaDe.psi_0("snow") - 0.5).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn k_fi_by_consequence_class() {
    assert!((k_fi(1) - 0.9).abs() < 1e-9);
    assert!((k_fi(2) - 1.0).abs() < 1e-9);
    assert!((k_fi(3) - 1.1).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn beta_target_by_reliability_class_and_period() {
    assert!((target_reliability_index_for(1, 50.0) - 3.3).abs() < 1e-9);
    assert!((target_reliability_index_for(2, 50.0) - 3.8).abs() < 1e-9);
    assert!((target_reliability_index_for(3, 50.0) - 4.3).abs() < 1e-9);
    assert!((target_reliability_index_for(2, 1.0) - 4.7).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn combination_6_11_accidental() {
    let annex = NaDe;
    let mut actions = sample_actions();
    actions.a_d = 40.0;
    let ed = combination_6_11(&annex, &actions, 0);
    // G + A_d + ψ1*office + ψ2*wind = 100 + 40 + 0.5*50 + 0*30 = 165
    assert!((ed - 165.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn seismic_zero_is_not_applicable() {
    let annex = NaDe;
    let actions = sample_actions();
    let check = check_seismic_situation(&annex, &actions, 0.0, 300.0);
    assert_eq!(check.status, CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn de_vs_en_congregation_psi_tables() {
    let de = NaDe;
    let en = NaEn;
    assert!((de.psi_1("congregation") - 0.7).abs() < 1e-9);
    assert!((en.psi_1("congregation") - 0.7).abs() < 1e-9);
    assert!((de.psi_0("other") - 0.8).abs() < 1e-9);
    assert!((en.psi_0("other") - 0.7).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn de_na_gamma_m_and_xi() {
    let annex = NaDe;
    assert!((annex.gamma_m("concrete") - 1.5).abs() < 1e-9);
    assert!((annex.xi("permanent") - 0.85).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn seismic_combination_de_vs_en_diverge_on_other_psi_2() {
    let actions = ActionSet::new(100.0, vec![("other".into(), 50.0)]);
    let de_ed = combination_6_12b(&NaDe, &actions, 40.0);
    let en_ed = combination_6_12b(&NaEn, &actions, 40.0);
    assert!((de_ed - 165.0).abs() < 1e-9);
    assert!((en_ed - 155.0).abs() < 1e-9);
}
