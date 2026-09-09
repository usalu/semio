use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_n_ed_kn() {
    store::os_store::test_support::assert_op_line_round_trip(&En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 95.0 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    store::os_store::test_support::assert_op_line_round_trip(&En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_alloy() {
    store::os_store::test_support::assert_op_line_round_trip(&En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: "aw6082t6".to_string() }));
}

/// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<En1999Mutation> {
    vec![
        En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 95.0 }),
        En1999Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 5.0 }),
        En1999Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: 1300.0 }),
        En1999Mutation::ChangeWElMm3(change_w_el_mm3::ChangeWElMm3 { new_w_el_mm3: 26_000.0 }),
        En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: "aw6082t6".to_string() }),
        En1999Mutation::ChangeChi(change_chi::ChangeChi { new_chi: 0.8 }),
        En1999Mutation::ChangeITMm4(change_i_t_mm4::ChangeITMm4 { new_i_t_mm4: 5400.0 }),
        En1999Mutation::ChangeLCrMm(change_l_cr_mm::ChangeLCrMm { new_l_cr_mm: 3200.0 }),
        En1999Mutation::ChangeThetaC(change_theta_c::ChangeThetaC { new_theta_c: 180.0 }),
        En1999Mutation::ChangeDeltaSigmaEd(change_delta_sigma_ed::ChangeDeltaSigmaEd { new_delta_sigma_ed: 50.0 }),
        En1999Mutation::ChangeDeltaSigmaC(change_delta_sigma_c::ChangeDeltaSigmaC { new_delta_sigma_c: 80.0 }),
        En1999Mutation::ChangeFatigueM(change_fatigue_m::ChangeFatigueM { new_fatigue_m: 5.0 }),
        En1999Mutation::ChangeNCycles(change_n_cycles::ChangeNCycles { new_n_cycles: 600_000.0 }),
        En1999Mutation::ChangeVWeldEdKn(change_v_weld_ed_kn::ChangeVWeldEdKn { new_v_weld_ed_kn: 28.0 }),
        En1999Mutation::ChangeWeldThroatMm(change_weld_throat_mm::ChangeWeldThroatMm { new_weld_throat_mm: 5.0 }),
        En1999Mutation::ChangeWeldLengthMm(change_weld_length_mm::ChangeWeldLengthMm { new_weld_length_mm: 140.0 }),
        En1999Mutation::ChangeBetaW(change_beta_w::ChangeBetaW { new_beta_w: 0.7 }),
        En1999Mutation::ChangeSheetBMm(change_sheet_b_mm::ChangeSheetBMm { new_sheet_b_mm: 220.0 }),
        En1999Mutation::ChangeSheetTMm(change_sheet_t_mm::ChangeSheetTMm { new_sheet_t_mm: 2.5 }),
        En1999Mutation::ChangeSheetKSigma(change_sheet_k_sigma::ChangeSheetKSigma { new_sheet_k_sigma: 4.2 }),
        En1999Mutation::ChangeSheetWElMm3(change_sheet_w_el_mm3::ChangeSheetWElMm3 { new_sheet_w_el_mm3: 8500.0 }),
        En1999Mutation::ChangeSheetMEdKnm(change_sheet_m_ed_knm::ChangeSheetMEdKnm { new_sheet_m_ed_knm: 0.6 }),
        En1999Mutation::ChangeShellTMm(change_shell_t_mm::ChangeShellTMm { new_shell_t_mm: 4.5 }),
        En1999Mutation::ChangeShellRMm(change_shell_r_mm::ChangeShellRMm { new_shell_r_mm: 520.0 }),
        En1999Mutation::ChangeSigmaEdShellMpa(change_sigma_ed_shell_mpa::ChangeSigmaEdShellMpa { new_sigma_ed_shell_mpa: 160.0 }),
        En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
    ]
}
