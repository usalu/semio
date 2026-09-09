use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_seismic_zone() {
    store::os_store::test_support::assert_op_line_round_trip(&En1998Mutation::ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone { new_seismic_zone: 3 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_ground_type() {
    store::os_store::test_support::assert_op_line_round_trip(&En1998Mutation::ChangeGroundType(change_ground_type::ChangeGroundType { new_ground_type: "c".to_string() }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_multiple_resisting_systems() {
    store::os_store::test_support::assert_op_line_round_trip(&En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::ChangeMultipleResistingSystems { new_multiple_resisting_systems: false }));
}

/// ⚖️ Every variant, not just the hand-picked ones above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

fn every_mutation() -> Vec<En1998Mutation> {
    vec![
        En1998Mutation::ChangeSeismicZone(change_seismic_zone::ChangeSeismicZone { new_seismic_zone: 3 }),
        En1998Mutation::ChangeGroundType(change_ground_type::ChangeGroundType { new_ground_type: "c".to_string() }),
        En1998Mutation::ChangeImportanceClass(change_importance_class::ChangeImportanceClass { new_importance_class: "cc3".to_string() }),
        En1998Mutation::ChangeStructuralSystem(change_structural_system::ChangeStructuralSystem { new_structural_system: "wall_dcm".to_string() }),
        En1998Mutation::ChangeT1S(change_t1_s::ChangeT1S { new_t1_s: 0.35 }),
        En1998Mutation::ChangeMassT(change_mass_t::ChangeMassT { new_mass_t: 550.0 }),
        En1998Mutation::ChangeVRdKn(change_v_rd_kn::ChangeVRdKn { new_v_rd_kn: 850.0 }),
        En1998Mutation::ChangeDriftMm(change_drift_mm::ChangeDriftMm { new_drift_mm: 22.0 }),
        En1998Mutation::ChangeHeightM(change_height_m::ChangeHeightM { new_height_m: 14.0 }),
        En1998Mutation::ChangeMultipleResistingSystems(change_multiple_resisting_systems::ChangeMultipleResistingSystems { new_multiple_resisting_systems: false }),
        En1998Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: "en".to_string() }),
        En1998Mutation::ChangeEnAGr(change_en_a_gr::ChangeEnAGr { new_en_a_gr: 0.2 }),
        En1998Mutation::ChangeEnGroundType(change_en_ground_type::ChangeEnGroundType { new_en_ground_type: "c".to_string() }),
        En1998Mutation::ChangeEnSpectrumType(change_en_spectrum_type::ChangeEnSpectrumType { new_en_spectrum_type: "type2".to_string() }),
        En1998Mutation::ChangePeriodRatio(change_period_ratio::ChangePeriodRatio { new_period_ratio: 1.8 }),
        En1998Mutation::ChangeBridgeVRdKn(change_bridge_v_rd_kn::ChangeBridgeVRdKn { new_bridge_v_rd_kn: 650.0 }),
        En1998Mutation::ChangeBearingDEdMm(change_bearing_d_ed_mm::ChangeBearingDEdMm { new_bearing_d_ed_mm: 130.0 }),
        En1998Mutation::ChangeBearingDRdMm(change_bearing_d_rd_mm::ChangeBearingDRdMm { new_bearing_d_rd_mm: 260.0 }),
        En1998Mutation::ChangeRetrofitKnowledgeLevel(change_retrofit_knowledge_level::ChangeRetrofitKnowledgeLevel { new_retrofit_knowledge_level: "kl3".to_string() }),
        En1998Mutation::ChangeRetrofitLimitState(change_retrofit_limit_state::ChangeRetrofitLimitState { new_retrofit_limit_state: "near_collapse".to_string() }),
        En1998Mutation::ChangeRetrofitEDKn(change_retrofit_e_d_kn::ChangeRetrofitEDKn { new_retrofit_e_d_kn: 270.0 }),
        En1998Mutation::ChangeRetrofitRKKn(change_retrofit_r_k_kn::ChangeRetrofitRKKn { new_retrofit_r_k_kn: 420.0 }),
        En1998Mutation::ChangeRetrofitGammaEl(change_retrofit_gamma_el::ChangeRetrofitGammaEl { new_retrofit_gamma_el: 1.15 }),
        En1998Mutation::ChangeSiloHeightM(change_silo_height_m::ChangeSiloHeightM { new_silo_height_m: 11.0 }),
        En1998Mutation::ChangeSiloRadiusM(change_silo_radius_m::ChangeSiloRadiusM { new_silo_radius_m: 5.5 }),
        En1998Mutation::ChangeSiloNRdKn(change_silo_n_rd_kn::ChangeSiloNRdKn { new_silo_n_rd_kn: 520.0 }),
        En1998Mutation::ChangeSiloVEdKn(change_silo_v_ed_kn::ChangeSiloVEdKn { new_silo_v_ed_kn: 190.0 }),
        En1998Mutation::ChangeSiloVRdKn(change_silo_v_rd_kn::ChangeSiloVRdKn { new_silo_v_rd_kn: 320.0 }),
        En1998Mutation::ChangeSiloQNominal(change_silo_q_nominal::ChangeSiloQNominal { new_silo_q_nominal: 2.2 }),
        En1998Mutation::ChangeTankHeightM(change_tank_height_m::ChangeTankHeightM { new_tank_height_m: 9.0 }),
        En1998Mutation::ChangeTankRadiusM(change_tank_radius_m::ChangeTankRadiusM { new_tank_radius_m: 4.5 }),
        En1998Mutation::ChangeTankMassT(change_tank_mass_t::ChangeTankMassT { new_tank_mass_t: 320.0 }),
        En1998Mutation::ChangeTankVRdKn(change_tank_v_rd_kn::ChangeTankVRdKn { new_tank_v_rd_kn: 420.0 }),
        En1998Mutation::ChangeTowerMEdKnm(change_tower_m_ed_knm::ChangeTowerMEdKnm { new_tower_m_ed_knm: 1300.0 }),
        En1998Mutation::ChangeTowerMRdKnm(change_tower_m_rd_knm::ChangeTowerMRdKnm { new_tower_m_rd_knm: 2600.0 }),
        En1998Mutation::ChangeTowerIsChimney(change_tower_is_chimney::ChangeTowerIsChimney { new_tower_is_chimney: false }),
        En1998Mutation::ChangeTowerQNominal(change_tower_q_nominal::ChangeTowerQNominal { new_tower_q_nominal: 2.8 }),
        En1998Mutation::ChangeTowerMassT(change_tower_mass_t::ChangeTowerMassT { new_tower_mass_t: 85.0 }),
        En1998Mutation::ChangeFoundationAreaM2(change_foundation_area_m2::ChangeFoundationAreaM2 { new_foundation_area_m2: 110.0 }),
        En1998Mutation::ChangeFoundationPRdKpa(change_foundation_p_rd_kpa::ChangeFoundationPRdKpa { new_foundation_p_rd_kpa: 520.0 }),
        En1998Mutation::ChangeFoundationHEdKn(change_foundation_h_ed_kn::ChangeFoundationHEdKn { new_foundation_h_ed_kn: 160.0 }),
        En1998Mutation::ChangeFoundationHRdKn(change_foundation_h_rd_kn::ChangeFoundationHRdKn { new_foundation_h_rd_kn: 420.0 }),
        En1998Mutation::ChangeKFoundation(change_k_foundation::ChangeKFoundation { new_k_foundation: 520_000.0 }),
        En1998Mutation::ChangeKSoil(change_k_soil::ChangeKSoil { new_k_soil: 210_000.0 }),
        En1998Mutation::ChangeWallHeightM(change_wall_height_m::ChangeWallHeightM { new_wall_height_m: 4.5 }),
        En1998Mutation::ChangeWallPhiDeg(change_wall_phi_deg::ChangeWallPhiDeg { new_wall_phi_deg: 32.0 }),
        En1998Mutation::ChangeWallSoilGammaKnM3(change_wall_soil_gamma_kn_m3::ChangeWallSoilGammaKnM3 { new_wall_soil_gamma_kn_m3: 19.0 }),
        En1998Mutation::ChangeWallR(change_wall_r::ChangeWallR { new_wall_r: 1.4 }),
        En1998Mutation::ChangeWallHRdKn(change_wall_h_rd_kn::ChangeWallHRdKn { new_wall_h_rd_kn: 160.0 }),
    ]
}
