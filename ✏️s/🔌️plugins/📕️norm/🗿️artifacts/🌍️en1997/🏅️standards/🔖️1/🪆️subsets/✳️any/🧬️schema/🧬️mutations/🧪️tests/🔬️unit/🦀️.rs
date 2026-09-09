use super::*;
use protocol::Mutation;

/// ⚖️ One value per `En1997Mutation` variant — the closed set the semantics/round-trip
/// tests iterate.
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
        En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
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

fn round_trip(base: &En1997Snapshot, mutation: &En1997Mutation) -> En1997Snapshot {
    let forward = vcs::apply_mutation(base, mutation).expect("valid mutation").0;
    let mut restored = forward.clone();
    for back in mutation.inverse(base) {
        restored = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation").0;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<En1997Mutation as protocol::SemanticMutation<En1997Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = En1997Snapshot::default();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn from_snapshot_round_trips_via_full_document_replacement() {
    let base = En1997Snapshot::default();
    let mut target = En1997Snapshot::default();
    let _ = &mut target;
    let mut projected = base.clone();
    for mutation in En1997Mutation::from_snapshot(&target) {
        projected = vcs::apply_mutation(&projected, &mutation).expect("snapshot mutation applies").0;
    }
    assert_eq!(projected, target, "from_snapshot must reconstruct every persistent field");
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
/// variants.
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = En1997Snapshot::default();
    let mutation = En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".to_string() }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_v_ed_kn_satisfies_the_inverse_and_absorb_laws() {
    let base = En1997Snapshot::default();
    let mutation = En1997Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangePileNProfiles(change_pile_n_profiles::ChangePileNProfiles { new_pile_n_profiles: 3 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_design_approach_satisfies_the_inverse_and_absorb_laws() {
    let base = En1997Snapshot::default();
    let mutation = En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: "da2".to_string() });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangePhiDeg(change_phi_deg::ChangePhiDeg { new_phi_deg: 32.0 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws
