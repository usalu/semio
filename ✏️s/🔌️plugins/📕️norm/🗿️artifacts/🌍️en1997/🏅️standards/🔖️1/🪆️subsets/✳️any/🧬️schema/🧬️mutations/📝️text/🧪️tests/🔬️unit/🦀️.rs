
use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_v_ed_kn() {
    store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_design_approach() {
    store::os_store::test_support::assert_op_line_round_trip(&En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".to_string() }));
}

/// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<En1997Mutation> {
    vec![
        En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 }),
        En1997Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn: 95.0 }),
        En1997Mutation::ChangeFootingAreaM2(change_footing_area_m2::ChangeFootingAreaM2 { new_footing_area_m2: 2.4 }),
        En1997Mutation::ChangePhiDeg(change_phi_deg::ChangePhiDeg { new_phi_deg: 32.0 }),
        En1997Mutation::ChangeCKpa(change_c_kpa::ChangeCKpa { new_c_kpa: 5.0 }),
        En1997Mutation::ChangeGammaKnM3(change_gamma_kn_m3::ChangeGammaKnM3 { new_gamma_kn_m3: 19.0 }),
        En1997Mutation::ChangeBM(change_b_m::ChangeBM { new_b_m: 2.2 }),
        En1997Mutation::ChangeDFM(change_d_f_m::ChangeDFM { new_d_f_m: 1.8 }),
        En1997Mutation::ChangeESMpa(change_e_s_mpa::ChangeESMpa { new_e_s_mpa: 32_000.0 }),
        En1997Mutation::ChangeNu(change_nu::ChangeNu { new_nu: 0.32 }),
        En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".to_string() }),
        En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1997Mutation::ChangeSettlementLimitMm(change_settlement_limit_mm::ChangeSettlementLimitMm { new_settlement_limit_mm: 20.0 }),
        En1997Mutation::ChangeNPileEdKn(change_n_pile_ed_kn::ChangeNPileEdKn { new_n_pile_ed_kn: 900.0 }),
        En1997Mutation::ChangeAlphaS(change_alpha_s::ChangeAlphaS { new_alpha_s: 0.75 }),
        En1997Mutation::ChangePileDM(change_pile_d_m::ChangePileDM { new_pile_d_m: 0.65 }),
        En1997Mutation::ChangeQSKpa(change_q_s_kpa::ChangeQSKpa { new_q_s_kpa: 90.0 }),
        En1997Mutation::ChangePileLM(change_pile_l_m::ChangePileLM { new_pile_l_m: 14.0 }),
        En1997Mutation::ChangeQBKpa(change_q_b_kpa::ChangeQBKpa { new_q_b_kpa: 2700.0 }),
        En1997Mutation::ChangePileBaseAreaM2(change_pile_base_area_m2::ChangePileBaseAreaM2 { new_pile_base_area_m2: 0.33 }),
        En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::ChangePileNProfiles { new_pile_n_profiles: 3 }),
        En1997Mutation::ChangeZInvestigatedM(change_z_investigated_m::ChangeZInvestigatedM { new_z_investigated_m: 10.0 }),
    ]
}
