use super::*;
use protocol::Mutation;

/// ⚖️ One value per `En1992Mutation` variant — the closed set the semantics/round-trip tests
/// iterate, mirroring this ticket's din16798/vdi3805 precedents' own `every_mutation()` fixture.
fn every_mutation() -> Vec<En1992Mutation> {
    vec![
        En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1992Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 150.0 }),
        En1992Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 95.0 }),
        En1992Mutation::ChangeFCk(change_f_ck::ChangeFCk { new_f_ck: 35.0 }),
        En1992Mutation::ChangeBMm(change_b_mm::ChangeBMm { new_b_mm: 350.0 }),
        En1992Mutation::ChangeDMm(change_d_mm::ChangeDMm { new_d_mm: 500.0 }),
        En1992Mutation::ChangeASMm2(change_a_s_mm2::ChangeASMm2 { new_a_s_mm2: 1400.0 }),
        En1992Mutation::ChangeFYk(change_f_yk::ChangeFYk { new_f_yk: 550.0 }),
        En1992Mutation::ChangeRhoL(change_rho_l::ChangeRhoL { new_rho_l: 0.015 }),
        En1992Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 25.0 }),
        En1992Mutation::ChangePKn(change_p_kn::ChangePKn { new_p_kn: 50.0 }),
        En1992Mutation::ChangeACMm2(change_a_c_mm2::ChangeACMm2 { new_a_c_mm2: 150000.0 }),
        En1992Mutation::ChangeUseFem(change_use_fem::ChangeUseFem { new_use_fem: true }),
        En1992Mutation::ChangeSpanM(change_span_m::ChangeSpanM { new_span_m: 7.5 }),
        En1992Mutation::ChangeUdlKnM(change_udl_kn_m::ChangeUdlKnM { new_udl_kn_m: 24.0 }),
        En1992Mutation::ChangeFireRating(change_fire_rating::ChangeFireRating { new_fire_rating: crate::part_1_2::FireRating::R90 }),
        En1992Mutation::ChangeProvidedAxisDistanceMm(change_provided_axis_distance_mm::ChangeProvidedAxisDistanceMm { new_provided_axis_distance_mm: 40.0 }),
        En1992Mutation::ChangeBridgeSigmaCMpa(change_bridge_sigma_c_mpa::ChangeBridgeSigmaCMpa { new_bridge_sigma_c_mpa: 14.0 }),
        En1992Mutation::ChangeBridgeDeltaSigmaSMpa(change_bridge_delta_sigma_s_mpa::ChangeBridgeDeltaSigmaSMpa { new_bridge_delta_sigma_s_mpa: 120.0 }),
        En1992Mutation::ChangeTightnessClass(change_tightness_class::ChangeTightnessClass { new_tightness_class: crate::part_3::TightnessClass::Tc2 }),
        En1992Mutation::ChangeHdOverH(change_hd_over_h::ChangeHdOverH { new_hd_over_h: 12.0 }),
        En1992Mutation::ChangeLiquidSigmaSMpa(change_liquid_sigma_s_mpa::ChangeLiquidSigmaSMpa { new_liquid_sigma_s_mpa: 220.0 }),
        En1992Mutation::ChangeLiquidRhoPEff(change_liquid_rho_p_eff::ChangeLiquidRhoPEff { new_liquid_rho_p_eff: 0.012 }),
        En1992Mutation::ChangeLiquidFCtEffMpa(change_liquid_f_ct_eff_mpa::ChangeLiquidFCtEffMpa { new_liquid_f_ct_eff_mpa: 3.1 }),
        En1992Mutation::ChangeLiquidESMpa(change_liquid_e_s_mpa::ChangeLiquidESMpa { new_liquid_e_s_mpa: 205000.0 }),
        En1992Mutation::ChangeLiquidSRMaxMm(change_liquid_s_r_max_mm::ChangeLiquidSRMaxMm { new_liquid_s_r_max_mm: 275.0 }),
        En1992Mutation::ChangeAnchorHEfMm(change_anchor_h_ef_mm::ChangeAnchorHEfMm { new_anchor_h_ef_mm: 90.0 }),
        En1992Mutation::ChangeAnchorCracked(change_anchor_cracked::ChangeAnchorCracked { new_anchor_cracked: true }),
        En1992Mutation::ChangeAnchorFUkMpa(change_anchor_f_uk_mpa::ChangeAnchorFUkMpa { new_anchor_f_uk_mpa: 850.0 }),
        En1992Mutation::ChangeAnchorFYkMpa(change_anchor_f_yk_mpa::ChangeAnchorFYkMpa { new_anchor_f_yk_mpa: 680.0 }),
        En1992Mutation::ChangeAnchorASMm2(change_anchor_a_s_mm2::ChangeAnchorASMm2 { new_anchor_a_s_mm2: 94.3 }),
        En1992Mutation::ChangeAnchorDMm(change_anchor_d_mm::ChangeAnchorDMm { new_anchor_d_mm: 14.0 }),
        En1992Mutation::ChangeAnchorC1Mm(change_anchor_c1_mm::ChangeAnchorC1Mm { new_anchor_c1_mm: 120.0 }),
        En1992Mutation::ChangeAnchorNEdKn(change_anchor_n_ed_kn::ChangeAnchorNEdKn { new_anchor_n_ed_kn: 15.0 }),
        En1992Mutation::ChangeAnchorVEdKn(change_anchor_v_ed_kn::ChangeAnchorVEdKn { new_anchor_v_ed_kn: 8.0 }),
    ]
}

fn round_trip(base: &En1992Snapshot, mutation: &En1992Mutation) -> En1992Snapshot {
    let (forward, _messages) = vcs::apply_mutation(base, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(base) {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
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
    assert_eq!(<En1992Mutation as protocol::SemanticMutation<En1992Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = En1992Snapshot::default();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::testkit`), exercised against the three most structurally
/// distinct variants: the enum-typed scalar (`change-annex`), a typical `f64` scalar
/// (`change-m-ed-knm`), and a `bool` scalar (`change-use-fem`).
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = En1992Snapshot::default();
    let mutation = En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1992Mutation::ChangeFireRating(change_fire_rating::ChangeFireRating { new_fire_rating: crate::part_1_2::FireRating::R90 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_satisfies_the_inverse_and_absorb_laws() {
    let base = En1992Snapshot::default();
    let mutation = En1992Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 150.0 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1992Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 95.0 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_use_fem_satisfies_the_inverse_and_absorb_laws() {
    let base = En1992Snapshot::default();
    let mutation = En1992Mutation::ChangeUseFem(change_use_fem::ChangeUseFem { new_use_fem: true });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1992Mutation::ChangeAnchorCracked(change_anchor_cracked::ChangeAnchorCracked { new_anchor_cracked: true }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// this facet is entirely one verb family (root-scoped `change-<field>`), so one representative
/// Fatal/no-op/determinism check per structurally distinct field type stands in for "one per
/// verb family". `assert_missing_target_is_error` does not apply — every field is a document-root
/// scalar that always exists, never an id-keyed/addressable target. `assert_outcome_policy_matrix`
/// is not landed under that literal name yet (only the differently-shaped `assert_policy_matrix`
/// exists) — flagged for the coordinator/lane 1-D, not improvised around.
#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_non_finite_is_fatal() {
    let base = En1992Snapshot::default();
    let mutation = En1992Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: f64::NAN });
    let outcome = mutation.diff(&base);
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn change_annex_same_value_is_no_op() {
    let base = En1992Snapshot::default();
    let mutation = En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: base.annex });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert_eq!(outcome.diff(), &En1992Diff::default());
}

#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_is_deterministic() {
    let base = En1992Snapshot::default();
    let mutation = En1992Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 150.0 });
    protocol::os_spr::testkit::assert_outcome_deterministic(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws
