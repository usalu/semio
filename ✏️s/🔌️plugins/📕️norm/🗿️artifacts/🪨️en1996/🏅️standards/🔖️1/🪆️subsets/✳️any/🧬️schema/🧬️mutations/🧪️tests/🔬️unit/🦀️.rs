use super::*;
use protocol::Mutation;

/// ⚖️ One value per `En1996Mutation` variant — the closed set the semantics/round-trip
/// tests iterate, mirroring `din16798`'s own `every_mutation()` fixture.
fn every_mutation() -> Vec<En1996Mutation> {
    vec![
        En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 12.5 }),
        En1996Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 250.0 }),
        En1996Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 42.0 }),
        En1996Mutation::ChangeHEdKn(change_h_ed_kn::ChangeHEdKn { new_h_ed_kn: 28.0 }),
        En1996Mutation::ChangeZMm3(change_z_mm3::ChangeZMm3 { new_z_mm3: 9_500_000.0 }),
        En1996Mutation::ChangeAreaMm2(change_area_mm2::ChangeAreaMm2 { new_area_mm2: 540_000.0 }),
        En1996Mutation::ChangeShearAreaMm2(change_shear_area_mm2::ChangeShearAreaMm2 { new_shear_area_mm2: 320_000.0 }),
        En1996Mutation::ChangeFKMpa(change_f_k_mpa::ChangeFKMpa { new_f_k_mpa: 6.5 }),
        En1996Mutation::ChangeFVkMpa(change_f_vk_mpa::ChangeFVkMpa { new_f_vk_mpa: 0.18 }),
        En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1996Mutation::ChangeMasonryClass(change_masonry_class::ChangeMasonryClass { new_masonry_class: crate::MasonryClass::Class4 }),
        En1996Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: crate::document::DesignSituation::Seismic }),
        En1996Mutation::ChangeMu(change_mu::ChangeMu { new_mu: 0.35 }),
        En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::ChangeWallThicknessMm { new_wall_thickness_mm: 300.0 }),
        En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin { new_fire_resistance_min: 90 }),
        En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit: "calcium_silicate".to_string() }),
        En1996Mutation::ChangeExposure(change_exposure::ChangeExposure { new_exposure: crate::part_2::ExposureClass::Mx3 }),
        En1996Mutation::ChangeMortar(change_mortar::ChangeMortar { new_mortar: crate::part_2::MortarClass::M10 }),
        En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: 15.0 }),
        En1996Mutation::ChangeStoreys(change_storeys::ChangeStoreys { new_storeys: 4 }),
        En1996Mutation::ChangeHEfMm(change_h_ef_mm::ChangeHEfMm { new_h_ef_mm: 2800.0 }),
        En1996Mutation::ChangeTEfMm(change_t_ef_mm::ChangeTEfMm { new_t_ef_mm: 200.0 }),
    ]
}

fn round_trip(base: &En1996Snapshot, mutation: &En1996Mutation) -> En1996Snapshot {
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
    assert_eq!(<En1996Mutation as protocol::SemanticMutation<En1996Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = En1996Snapshot::default();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::os_spr::testkit`), exercised against three structurally distinct
/// variants: an enum scalar (`change-annex`), a plain `f64` scalar (`change-m-ed-knm`), and a
/// `String` scalar (`change-unit`).
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = En1996Snapshot::default();
    let mutation = En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit: "calcium_silicate".to_string() }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_satisfies_the_inverse_and_absorb_laws() {
    let base = En1996Snapshot::default();
    let mutation = En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 12.5 });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1996Mutation::ChangeStoreys(change_storeys::ChangeStoreys { new_storeys: 4 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_unit_satisfies_the_inverse_and_absorb_laws() {
    let base = En1996Snapshot::default();
    let mutation = En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit: "calcium_silicate".to_string() });
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1996Mutation::ChangeFKMpa(change_f_k_mpa::ChangeFKMpa { new_f_k_mpa: 6.5 }).diff(&base).diff().clone();
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// this facet is entirely one verb family (root-scoped `change-<field>`) — see en1992's own
/// `🔖️OutcomeLaws` note for why `assert_missing_target_is_error`/`assert_outcome_policy_matrix`
/// don't apply/aren't landed yet.
#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_non_finite_is_fatal() {
    let base = En1996Snapshot::default();
    let mutation = En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: f64::NAN });
    let outcome = mutation.diff(&base);
    protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn change_masonry_class_same_value_is_no_op() {
    let base = En1996Snapshot::default();
    let mutation = En1996Mutation::ChangeMasonryClass(change_masonry_class::ChangeMasonryClass { new_masonry_class: base.masonry_class });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert_eq!(outcome.diff(), &En1996Diff::default());
}

#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_is_deterministic() {
    let base = En1996Snapshot::default();
    let mutation = En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 12.5 });
    protocol::os_spr::testkit::assert_outcome_deterministic(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws
