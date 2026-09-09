use super::*;

/// ⚖️ Every variant — full-coverage `OpText` round trip over the closed vocabulary, one sample
/// value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<En1995Mutation> {
    vec![
        En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1995Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 25.0 }),
        En1995Mutation::ChangeNEdKn(change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 50.0 }),
        En1995Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 15.0 }),
        En1995Mutation::ChangeWMm3(change_w_mm3::ChangeWMm3 { new_w_mm3: 1_000_000.0 }),
        En1995Mutation::ChangeAMm2(change_a_mm2::ChangeAMm2 { new_a_mm2: 20_000.0 }),
        En1995Mutation::ChangeBMm(change_b_mm::ChangeBMm { new_b_mm: 200.0 }),
        En1995Mutation::ChangeHMm(change_h_mm::ChangeHMm { new_h_mm: 300.0 }),
        En1995Mutation::ChangeFMK(change_f_m_k::ChangeFMK { new_f_m_k: 24.0 }),
        En1995Mutation::ChangeFC0K(change_f_c_0_k::ChangeFC0K { new_f_c_0_k: 21.0 }),
        En1995Mutation::ChangeServiceClass(change_service_class::ChangeServiceClass { new_service_class: "sc1".into() }),
        En1995Mutation::ChangeLoadDuration(change_load_duration::ChangeLoadDuration { new_load_duration: "medium".into() }),
        En1995Mutation::ChangeMCritKnm(change_m_crit_knm::ChangeMCritKnm { new_m_crit_knm: 80.0 }),
        En1995Mutation::ChangeFEdKn(change_f_ed_kn::ChangeFEdKn { new_f_ed_kn: 18.0 }),
        En1995Mutation::ChangeAEfMm2(change_a_ef_mm2::ChangeAEfMm2 { new_a_ef_mm2: 12_000.0 }),
        En1995Mutation::ChangeFVK(change_f_v_k::ChangeFVK { new_f_v_k: 4.0 }),
        En1995Mutation::ChangeFireDurationMin(change_fire_duration_min::ChangeFireDurationMin { new_fire_duration_min: 30.0 }),
        En1995Mutation::ChangeSectionDepthMm(change_section_depth_mm::ChangeSectionDepthMm { new_section_depth_mm: 300.0 }),
        En1995Mutation::ChangeAVertMS2(change_a_vert_m_s2::ChangeAVertMS2 { new_a_vert_m_s2: 0.3 }),
        En1995Mutation::ChangeNCyclesBridge(change_n_cycles_bridge::ChangeNCyclesBridge { new_n_cycles_bridge: 500_000.0 }),
    ]
}
