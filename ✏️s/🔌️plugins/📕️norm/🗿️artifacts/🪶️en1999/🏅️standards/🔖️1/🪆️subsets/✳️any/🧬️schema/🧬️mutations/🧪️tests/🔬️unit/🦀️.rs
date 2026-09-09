use super::*;
use protocol::Mutation;

/// ⚖️ One value per `En1999Mutation` variant — the closed set the semantics/round-trip
/// tests iterate.
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
        En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
    ]
}

fn round_trip(base: &En1999Snapshot, mutation: &En1999Mutation) -> En1999Snapshot {
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
    assert_eq!(<En1999Mutation as protocol::SemanticMutation<En1999Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = En1999Snapshot::default();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

#[semio_framework_async_macros::async_test]
async fn from_snapshot_round_trips_via_full_document_replacement() {
    let base = En1999Snapshot::default();
    let mut target = En1999Snapshot::default();
    let _ = &mut target;
    let mut projected = base.clone();
    for mutation in En1999Mutation::from_snapshot(&target) {
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
    let base = En1999Snapshot::default();
    let mutation = En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: "aw6082t6".to_string() }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_n_ed_kn_satisfies_the_inverse_and_absorb_laws() {
    let base = En1999Snapshot::default();
    let mutation = En1999Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 95.0 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1999Mutation::ChangeNCycles(change_n_cycles::ChangeNCycles { new_n_cycles: 600_000.0 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_alloy_satisfies_the_inverse_and_absorb_laws() {
    let base = En1999Snapshot::default();
    let mutation = En1999Mutation::ChangeAlloy(change_alloy::ChangeAlloy { new_alloy: "aw6082t6".to_string() });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1999Mutation::ChangeChi(change_chi::ChangeChi { new_chi: 0.8 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws
