use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_annex() {
    store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_m_ed_knm() {
    store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeMEdKnm(change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 12.5 }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_unit() {
    store::os_store::test_support::assert_op_line_round_trip(&En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit: "calcium_silicate".into() }));
}

/// ⚖️ Every variant, not just the three hand-picked above — full-coverage `OpText` round trip
/// over the closed vocabulary, one sample value per field.
#[semio_framework_async_macros::async_test]
async fn every_variant_op_text_round_trips() {
    for mutation in every_mutation() {
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}

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
        En1996Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1996Mutation::ChangeMasonryClass(change_masonry_class::ChangeMasonryClass { new_masonry_class: MasonryClass::Class4 }),
        En1996Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: DesignSituation::Seismic }),
        En1996Mutation::ChangeMu(change_mu::ChangeMu { new_mu: 0.35 }),
        En1996Mutation::ChangeWallThicknessMm(change_wall_thickness_mm::ChangeWallThicknessMm { new_wall_thickness_mm: 300.0 }),
        En1996Mutation::ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin { new_fire_resistance_min: 90 }),
        En1996Mutation::ChangeUnit(change_unit::ChangeUnit { new_unit: "calcium_silicate".to_string() }),
        En1996Mutation::ChangeExposure(change_exposure::ChangeExposure { new_exposure: ExposureClass::Mx3 }),
        En1996Mutation::ChangeMortar(change_mortar::ChangeMortar { new_mortar: MortarClass::M10 }),
        En1996Mutation::ChangeBedJointThicknessMm(change_bed_joint_thickness_mm::ChangeBedJointThicknessMm { new_bed_joint_thickness_mm: 15.0 }),
        En1996Mutation::ChangeStoreys(change_storeys::ChangeStoreys { new_storeys: 4 }),
        En1996Mutation::ChangeHEfMm(change_h_ef_mm::ChangeHEfMm { new_h_ef_mm: 2800.0 }),
        En1996Mutation::ChangeTEfMm(change_t_ef_mm::ChangeTEfMm { new_t_ef_mm: 200.0 }),
    ]
}
