use super::*;
use protocol::Mutation;

/// ⚖️ One value per `En1995Mutation` variant — the closed set the semantics/round-trip tests
/// iterate, mirroring this ticket's `en1992`/`en1993` precedent's own `every_mutation()` fixture.
fn every_mutation() -> Vec<En1995Mutation> {
    vec![
        En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 999.0 }),
        En1995Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 111.0 }),
        En1995Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 77.0 }),
        En1995Mutation::ChangeWMm3(change_w_mm3::ChangeWMm3 { new_w_mm3: 2_000_000.0 }),
        En1995Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: 30_000.0 }),
        En1995Mutation::ChangeBMm(change_b_mm::ChangeBMm { new_b_mm: 250.0 }),
        En1995Mutation::ChangeHMm(change_h_mm::ChangeHMm { new_h_mm: 400.0 }),
        En1995Mutation::ChangeFMK(change_f_m_k::ChangeFMK { new_f_m_k: 28.0 }),
        En1995Mutation::ChangeFC0K(change_f_c_0_k::ChangeFC0K { new_f_c_0_k: 24.0 }),
        En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class: "sc2".into() }),
        En1995Mutation::ChangeLoadDuration(change_load_duration::ChangeLoadDuration { new_load_duration: "short".into() }),
        En1995Mutation::ChangeMCritKnm(change_m_crit_knm::ChangeMCritKnm { new_m_crit_knm: 95.0 }),
        En1995Mutation::ChangeFEdKn(change_f_ed_kn::ChangeFEdKn { new_f_ed_kn: 22.0 }),
        En1995Mutation::ChangeAEfMm2(change_a_ef_mm2::ChangeAEfMm2 { new_a_ef_mm2: 14_000.0 }),
        En1995Mutation::ChangeFVK(change_f_v_k::ChangeFVK { new_f_v_k: 4.5 }),
        En1995Mutation::ChangeFireDurationMin(change_fire_duration_min::ChangeFireDurationMin { new_fire_duration_min: 60.0 }),
        En1995Mutation::ChangeSectionDepthMm(change_section_depth_mm::ChangeSectionDepthMm { new_section_depth_mm: 350.0 }),
        En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::ChangeAVertMS2 { new_a_vert_m_s2: 0.5 }),
        En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::ChangeNCyclesBridge { new_n_cycles_bridge: 750_000.0 }),
    ]
}

fn round_trip(base: &En1995Snapshot, mutation: &En1995Mutation) -> En1995Snapshot {
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
    assert_eq!(<En1995Mutation as protocol::SemanticMutation<En1995Snapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

#[semio_framework_async_macros::async_test]
async fn every_variant_round_trips_via_inverse() {
    let base = En1995Snapshot::default();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `protocol::os_spr::protocol_laws`), exercised against the three most structurally
/// distinct variants: the enum-typed scalar (`change-annex`), a typical `f64` scalar
/// (`change-m-ed-knm`), and a `String` scalar (`change-service-class`).
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = En1995Snapshot::default();
    let mutation = En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class: "sc2".into() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_m_ed_knm_satisfies_the_inverse_and_absorb_laws() {
    let base = En1995Snapshot::default();
    let mutation = En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 999.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1995Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 77.0 }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
#[semio_framework_async_macros::async_test]
async fn change_service_class_satisfies_the_inverse_and_absorb_laws() {
    let base = En1995Snapshot::default();
    let mutation = En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class: "sc2".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1995Mutation::ChangeLoadDuration(change_load_duration::ChangeLoadDuration { new_load_duration: "short".into() }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws
