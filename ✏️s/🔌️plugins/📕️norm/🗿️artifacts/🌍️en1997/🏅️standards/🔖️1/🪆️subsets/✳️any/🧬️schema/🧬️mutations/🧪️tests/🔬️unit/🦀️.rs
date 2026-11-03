use super::*;
use crate::document::AnnexChoice;
use crate::standards::v1::subsets::any::schema::snapshot::compliant_demo;
use protocol::{ColdMutation, Mutation};

/// ⚖️ One value per `En1997Mutation` variant — the closed set the semantics/round-trip
/// tests iterate.
fn typed_samples(demo: &semio_s_artifact_norm_en1997_model::En1997Snapshot) -> Vec<En1997Mutation> {
    let nested_footings = demo.footings.iter().find(|row| !row.load_cases.is_empty()).expect("demo row with nested rows");
    let nested_piles = demo.piles.iter().find(|row| !row.test_profiles.is_empty()).expect("demo row with nested rows");
    let nested_anchors = demo.anchors.iter().find(|row| !row.tests.is_empty()).expect("demo row with nested rows");
    let nested_lab_tests = demo.lab_tests.iter().find(|row| !row.points.is_empty()).expect("demo row with nested rows");
    vec![
        En1997Mutation::ChangeLayerSoilType(change_layer_soil_type::ChangeLayerSoilType { id: demo.layers[0].id.clone(), new_soil_type: semio_s_artifact_norm_en1997_model::SoilType::Clay }),
        En1997Mutation::ChangeLayerDepthTop(change_layer_depth_top::ChangeLayerDepthTop { id: demo.layers[0].id.clone(), new_depth_top: 1.5 }),
        En1997Mutation::ChangeLayerDepthBottom(change_layer_depth_bottom::ChangeLayerDepthBottom { id: demo.layers[0].id.clone(), new_depth_bottom: 1.5 }),
        En1997Mutation::ChangeLayerGamma(change_layer_gamma::ChangeLayerGamma { id: demo.layers[0].id.clone(), new_gamma: 1.5 }),
        En1997Mutation::ChangeLayerGammaPrime(change_layer_gamma_prime::ChangeLayerGammaPrime { id: demo.layers[0].id.clone(), new_gamma_prime: 1.5 }),
        En1997Mutation::ChangeLayerCohesionEffective(change_layer_cohesion_effective::ChangeLayerCohesionEffective { id: demo.layers[0].id.clone(), new_cohesion_effective: 1.5 }),
        En1997Mutation::ChangeLayerCohesionUndrained(change_layer_cohesion_undrained::ChangeLayerCohesionUndrained { id: demo.layers[0].id.clone(), new_cohesion_undrained: 1.5 }),
        En1997Mutation::ChangeLayerPoissonRatio(change_layer_poisson_ratio::ChangeLayerPoissonRatio { id: demo.layers[0].id.clone(), new_poisson_ratio: 1.5 }),
        En1997Mutation::ChangeLayerCptQc(change_layer_cpt_qc::ChangeLayerCptQc { id: demo.layers[0].id.clone(), new_cpt_qc: 1.5 }),
        En1997Mutation::ChangeLayerSptN(change_layer_spt_n::ChangeLayerSptN { id: demo.layers[0].id.clone(), new_spt_n: 1.5 }),
        En1997Mutation::ChangeFootingLength(change_footing_length::ChangeFootingLength { id: demo.footings[0].id.clone(), new_length: 1.5 }),
        En1997Mutation::ChangeFootingBaseInclDeg(change_footing_base_incl_deg::ChangeFootingBaseInclDeg { id: demo.footings[0].id.clone(), new_base_inclination_deg: 1.5 }),
        En1997Mutation::ChangeFootingSettlementLimit(change_footing_settlement_limit::ChangeFootingSettlementLimit { id: demo.footings[0].id.clone(), new_settlement_limit: 1.5 }),
        En1997Mutation::ChangeLoadCaseDesignSituation(change_load_case_design_situation::ChangeLoadCaseDesignSituation { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_design_situation: semio_s_artifact_norm_en1997_model::SituationChoice::Transient }),
        En1997Mutation::ChangeLoadCaseVerticalPerm(change_load_case_vertical_perm::ChangeLoadCaseVerticalPerm { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_vertical_permanent: 1.5 }),
        En1997Mutation::ChangeLoadCaseVerticalVar(change_load_case_vertical_var::ChangeLoadCaseVerticalVar { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_vertical_variable: 1.5 }),
        En1997Mutation::ChangeLoadCaseHorizPermanent(change_load_case_horiz_permanent::ChangeLoadCaseHorizPermanent { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_horizontal_permanent: 1.5 }),
        En1997Mutation::ChangeLoadCaseHorizVariable(change_load_case_horiz_variable::ChangeLoadCaseHorizVariable { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_horizontal_variable: 1.5 }),
        En1997Mutation::ChangeLoadCaseMomentPermanent(change_load_case_moment_permanent::ChangeLoadCaseMomentPermanent { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_moment_permanent: 1.5 }),
        En1997Mutation::ChangeLoadCaseMomentVariable(change_load_case_moment_variable::ChangeLoadCaseMomentVariable { footing_id: nested_footings.id.clone(), id: nested_footings.load_cases[0].id.clone(), new_moment_variable: 1.5 }),
        En1997Mutation::ChangePilePileType(change_pile_pile_type::ChangePilePileType { id: demo.piles[0].id.clone(), new_pile_type: semio_s_artifact_norm_en1997_model::PileType::Driven }),
        En1997Mutation::ChangePileDiameter(change_pile_diameter::ChangePileDiameter { id: demo.piles[0].id.clone(), new_diameter: 1.5 }),
        En1997Mutation::ChangePileAlphaS(change_pile_alpha_s::ChangePileAlphaS { id: demo.piles[0].id.clone(), new_alpha_s: 1.5 }),
        En1997Mutation::ChangePileUnitShaftResistance(change_pile_unit_shaft_resistance::ChangePileUnitShaftResistance { id: demo.piles[0].id.clone(), new_unit_shaft_resistance: 1.5 }),
        En1997Mutation::ChangePileUnitBaseResistance(change_pile_unit_base_resistance::ChangePileUnitBaseResistance { id: demo.piles[0].id.clone(), new_unit_base_resistance: 1.5 }),
        En1997Mutation::ChangePileCompressionPermanent(change_pile_compression_permanent::ChangePileCompressionPermanent { id: demo.piles[0].id.clone(), new_compression_permanent: 1.5 }),
        En1997Mutation::ChangePileCompressionVariable(change_pile_compression_variable::ChangePileCompressionVariable { id: demo.piles[0].id.clone(), new_compression_variable: 1.5 }),
        En1997Mutation::ChangePileTensionPermanent(change_pile_tension_permanent::ChangePileTensionPermanent { id: demo.piles[0].id.clone(), new_tension_permanent: 1.5 }),
        En1997Mutation::ChangePileTensionVariable(change_pile_tension_variable::ChangePileTensionVariable { id: demo.piles[0].id.clone(), new_tension_variable: 1.5 }),
        En1997Mutation::ChangeProfileShaftResistance(change_profile_shaft_resistance::ChangeProfileShaftResistance { pile_id: nested_piles.id.clone(), id: nested_piles.test_profiles[0].id.clone(), new_shaft_resistance: 1.5 }),
        En1997Mutation::ChangeProfileBaseResistance(change_profile_base_resistance::ChangeProfileBaseResistance { pile_id: nested_piles.id.clone(), id: nested_piles.test_profiles[0].id.clone(), new_base_resistance: 1.5 }),
        En1997Mutation::ChangeWallHeight(change_wall_height::ChangeWallHeight { id: demo.retaining_walls[0].id.clone(), new_height: 1.5 }),
        En1997Mutation::ChangeWallEmbedment(change_wall_embedment::ChangeWallEmbedment { id: demo.retaining_walls[0].id.clone(), new_embedment: 1.5 }),
        En1997Mutation::ChangeWallStemThickness(change_wall_stem_thickness::ChangeWallStemThickness { id: demo.retaining_walls[0].id.clone(), new_stem_thickness: 1.5 }),
        En1997Mutation::ChangeWallBackfillPhiDeg(change_wall_backfill_phi_deg::ChangeWallBackfillPhiDeg { id: demo.retaining_walls[0].id.clone(), new_backfill_phi_deg: 1.5 }),
        En1997Mutation::ChangeWallBackfillGamma(change_wall_backfill_gamma::ChangeWallBackfillGamma { id: demo.retaining_walls[0].id.clone(), new_backfill_gamma: 1.5 }),
        En1997Mutation::ChangeWallWallFrictionDeg(change_wall_wall_friction_deg::ChangeWallWallFrictionDeg { id: demo.retaining_walls[0].id.clone(), new_wall_friction_deg: 1.5 }),
        En1997Mutation::ChangeWallEarthPressureMode(change_wall_earth_pressure_mode::ChangeWallEarthPressureMode { id: demo.retaining_walls[0].id.clone(), new_earth_pressure_mode: semio_s_artifact_norm_en1997_model::EarthPressureMode::AtRest }),
        En1997Mutation::ChangeWallWallMovement(change_wall_wall_movement::ChangeWallWallMovement { id: demo.retaining_walls[0].id.clone(), new_wall_movement: semio_s_artifact_norm_en1997_model::WallMovement::Rigid }),
        En1997Mutation::ChangeWallOcr(change_wall_ocr::ChangeWallOcr { id: demo.retaining_walls[0].id.clone(), new_ocr: 1.5 }),
        En1997Mutation::ChangeWallConcreteGamma(change_wall_concrete_gamma::ChangeWallConcreteGamma { id: demo.retaining_walls[0].id.clone(), new_concrete_gamma: 1.5 }),
        En1997Mutation::ChangeWallSurcharge(change_wall_surcharge::ChangeWallSurcharge { id: demo.retaining_walls[0].id.clone(), new_surcharge: 1.5 }),
        En1997Mutation::ChangeWallVerticalPermanent(change_wall_vertical_permanent::ChangeWallVerticalPermanent { id: demo.retaining_walls[0].id.clone(), new_vertical_permanent: 1.5 }),
        En1997Mutation::ChangeWallHorizontalPermanent(change_wall_horizontal_permanent::ChangeWallHorizontalPermanent { id: demo.retaining_walls[0].id.clone(), new_horizontal_permanent: 1.5 }),
        En1997Mutation::ChangeSlopeHeight(change_slope_height::ChangeSlopeHeight { id: demo.slopes[0].id.clone(), new_height: 1.5 }),
        En1997Mutation::ChangeSlopeLength(change_slope_length::ChangeSlopeLength { id: demo.slopes[0].id.clone(), new_length: 1.5 }),
        En1997Mutation::ChangeSlopeGoverningLayerId(change_slope_governing_layer_id::ChangeSlopeGoverningLayerId { id: demo.slopes[0].id.clone(), new_governing_layer_id: "alt".into() }),
        En1997Mutation::ChangeSlopeCircleCentreX(change_slope_circle_centre_x::ChangeSlopeCircleCentreX { id: demo.slopes[0].id.clone(), new_circle_centre_x: 1.5 }),
        En1997Mutation::ChangeSlopeCircleCentreZ(change_slope_circle_centre_z::ChangeSlopeCircleCentreZ { id: demo.slopes[0].id.clone(), new_circle_centre_z: 1.5 }),
        En1997Mutation::ChangeSlopeCircleRadius(change_slope_circle_radius::ChangeSlopeCircleRadius { id: demo.slopes[0].id.clone(), new_circle_radius: 1.5 }),
        En1997Mutation::ChangeUpliftPermStabilizing(change_uplift_perm_stabilizing::ChangeUpliftPermStabilizing { id: demo.uplift_cases[0].id.clone(), new_permanent_stabilizing: 1.5 }),
        En1997Mutation::ChangeUpliftPermDestabilizing(change_uplift_perm_destabilizing::ChangeUpliftPermDestabilizing { id: demo.uplift_cases[0].id.clone(), new_permanent_destabilizing: 1.5 }),
        En1997Mutation::ChangeUpliftVarDestabilizing(change_uplift_var_destabilizing::ChangeUpliftVarDestabilizing { id: demo.uplift_cases[0].id.clone(), new_variable_destabilizing: 1.5 }),
        En1997Mutation::ChangeUpliftPorePressure(change_uplift_pore_pressure::ChangeUpliftPorePressure { id: demo.uplift_cases[0].id.clone(), new_pore_pressure: 1.5 }),
        En1997Mutation::ChangeUpliftTotalStress(change_uplift_total_stress::ChangeUpliftTotalStress { id: demo.uplift_cases[0].id.clone(), new_total_stress: 1.5 }),
        En1997Mutation::ChangeAnchorWallId(change_anchor_wall_id::ChangeAnchorWallId { id: demo.anchors[0].id.clone(), new_wall_id: "alt".into() }),
        En1997Mutation::ChangeAnchorTendon(change_anchor_tendon::ChangeAnchorTendon { id: demo.anchors[0].id.clone(), new_tendon: semio_s_artifact_norm_en1997_model::TendonType::Bar }),
        En1997Mutation::ChangeAnchorHeadDepth(change_anchor_head_depth::ChangeAnchorHeadDepth { id: demo.anchors[0].id.clone(), new_head_depth: 1.5 }),
        En1997Mutation::ChangeAnchorInclinationDeg(change_anchor_inclination_deg::ChangeAnchorInclinationDeg { id: demo.anchors[0].id.clone(), new_inclination_deg: 1.5 }),
        En1997Mutation::ChangeAnchorFreeLength(change_anchor_free_length::ChangeAnchorFreeLength { id: demo.anchors[0].id.clone(), new_free_length: 1.5 }),
        En1997Mutation::ChangeAnchorFixedLength(change_anchor_fixed_length::ChangeAnchorFixedLength { id: demo.anchors[0].id.clone(), new_fixed_length: 1.5 }),
        En1997Mutation::ChangeAnchorFixedDiameter(change_anchor_fixed_diameter::ChangeAnchorFixedDiameter { id: demo.anchors[0].id.clone(), new_fixed_diameter: 1.5 }),
        En1997Mutation::ChangeAnchorBondStress(change_anchor_bond_stress::ChangeAnchorBondStress { id: demo.anchors[0].id.clone(), new_bond_stress: 1.5 }),
        En1997Mutation::ChangeAnchorDesignForce(change_anchor_design_force::ChangeAnchorDesignForce { id: demo.anchors[0].id.clone(), new_design_force: 1.5 }),
        En1997Mutation::ChangeAnchorTendonYieldForce(change_anchor_tendon_yield_force::ChangeAnchorTendonYieldForce { id: demo.anchors[0].id.clone(), new_tendon_yield_force: 1.5 }),
        En1997Mutation::ChangeAnchorMinFreeLength(change_anchor_min_free_length::ChangeAnchorMinFreeLength { id: demo.anchors[0].id.clone(), new_min_free_length: 1.5 }),
        En1997Mutation::ChangeAnchorTestFactor(change_anchor_test_factor::ChangeAnchorTestFactor { id: demo.anchors[0].id.clone(), new_test_factor: 1.5 }),
        En1997Mutation::ChangeAnchorTestPullOutRes(change_anchor_test_pull_out_res::ChangeAnchorTestPullOutRes { anchor_id: nested_anchors.id.clone(), id: nested_anchors.tests[0].id.clone(), new_pull_out_resistance: 1.5 }),
        En1997Mutation::ChangeEmbWallKind(change_emb_wall_kind::ChangeEmbWallKind { id: demo.embedded_walls[0].id.clone(), new_kind: semio_s_artifact_norm_en1997_model::EmbeddedWallKind::Cantilever }),
        En1997Mutation::ChangeEmbWallRetainedHeight(change_emb_wall_retained_height::ChangeEmbWallRetainedHeight { id: demo.embedded_walls[0].id.clone(), new_retained_height: 1.5 }),
        En1997Mutation::ChangeEmbWallEmbedment(change_emb_wall_embedment::ChangeEmbWallEmbedment { id: demo.embedded_walls[0].id.clone(), new_embedment: 1.5 }),
        En1997Mutation::ChangeEmbWallPropDepth(change_emb_wall_prop_depth::ChangeEmbWallPropDepth { id: demo.embedded_walls[0].id.clone(), new_prop_depth: 1.5 }),
        En1997Mutation::ChangeEmbWallThickness(change_emb_wall_thickness::ChangeEmbWallThickness { id: demo.embedded_walls[0].id.clone(), new_thickness: 1.5 }),
        En1997Mutation::ChangeEmbWallBackfillGamma(change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma { id: demo.embedded_walls[0].id.clone(), new_backfill_gamma: 1.5 }),
        En1997Mutation::ChangeEmbWallPhiDeg(change_emb_wall_phi_deg::ChangeEmbWallPhiDeg { id: demo.embedded_walls[0].id.clone(), new_phi_deg: 1.5 }),
        En1997Mutation::ChangeEmbWallDeltaActiveDeg(change_emb_wall_delta_active_deg::ChangeEmbWallDeltaActiveDeg { id: demo.embedded_walls[0].id.clone(), new_delta_active_deg: 1.5 }),
        En1997Mutation::ChangeEmbWallDeltaPassiveDeg(change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg { id: demo.embedded_walls[0].id.clone(), new_delta_passive_deg: 1.5 }),
        En1997Mutation::ChangeEmbWallSurcharge(change_emb_wall_surcharge::ChangeEmbWallSurcharge { id: demo.embedded_walls[0].id.clone(), new_surcharge: 1.5 }),
        En1997Mutation::ChangeEmbWallUndrStrength(change_emb_wall_undr_strength::ChangeEmbWallUndrStrength { id: demo.embedded_walls[0].id.clone(), new_undrained_strength: 1.5 }),
        En1997Mutation::ChangePileGroupPileId(change_pile_group_pile_id::ChangePileGroupPileId { id: demo.pile_groups[0].id.clone(), new_pile_id: "alt".into() }),
        En1997Mutation::ChangePileGroupRows(change_pile_group_rows::ChangePileGroupRows { id: demo.pile_groups[0].id.clone(), new_rows: 2 }),
        En1997Mutation::ChangePileGroupColumns(change_pile_group_columns::ChangePileGroupColumns { id: demo.pile_groups[0].id.clone(), new_columns: 2 }),
        En1997Mutation::ChangePileGroupSpacing(change_pile_group_spacing::ChangePileGroupSpacing { id: demo.pile_groups[0].id.clone(), new_spacing: 1.5 }),
        En1997Mutation::ChangePileGroupCapVertical(change_pile_group_cap_vertical::ChangePileGroupCapVertical { id: demo.pile_groups[0].id.clone(), new_cap_vertical: 1.5 }),
        En1997Mutation::ChangePileGroupCapMomentX(change_pile_group_cap_moment_x::ChangePileGroupCapMomentX { id: demo.pile_groups[0].id.clone(), new_cap_moment_x: 1.5 }),
        En1997Mutation::ChangePileGroupCapMomentY(change_pile_group_cap_moment_y::ChangePileGroupCapMomentY { id: demo.pile_groups[0].id.clone(), new_cap_moment_y: 1.5 }),
        En1997Mutation::ChangePileGroupSingleRes(change_pile_group_single_res::ChangePileGroupSingleRes { id: demo.pile_groups[0].id.clone(), new_single_resistance: 1.5 }),
        En1997Mutation::ChangePileGroupShaftCohSum(change_pile_group_shaft_coh_sum::ChangePileGroupShaftCohSum { id: demo.pile_groups[0].id.clone(), new_shaft_cohesion_sum: 1.5 }),
        En1997Mutation::ChangePileGroupBaseCohesion(change_pile_group_base_cohesion::ChangePileGroupBaseCohesion { id: demo.pile_groups[0].id.clone(), new_base_cohesion: 1.5 }),
        En1997Mutation::ChangeLabTestLayerId(change_lab_test_layer_id::ChangeLabTestLayerId { id: demo.lab_tests[0].id.clone(), new_layer_id: "alt".into() }),
        En1997Mutation::ChangeLabTestKind(change_lab_test_kind::ChangeLabTestKind { id: demo.lab_tests[0].id.clone(), new_kind: semio_s_artifact_norm_en1997_model::LabTestKind::Triaxial }),
        En1997Mutation::ChangeLabTestVoidRatio(change_lab_test_void_ratio::ChangeLabTestVoidRatio { id: demo.lab_tests[0].id.clone(), new_void_ratio: 1.5 }),
        En1997Mutation::ChangeLabTestPreloadStress(change_lab_test_preload_stress::ChangeLabTestPreloadStress { id: demo.lab_tests[0].id.clone(), new_preload_stress: 1.5 }),
        En1997Mutation::ChangeLabTestDrainageHeight(change_lab_test_drainage_height::ChangeLabTestDrainageHeight { id: demo.lab_tests[0].id.clone(), new_drainage_height: 1.5 }),
        En1997Mutation::ChangeLabTestConsolTime(change_lab_test_consol_time::ChangeLabTestConsolTime { id: demo.lab_tests[0].id.clone(), new_consolidation_time: 1.5 }),
        En1997Mutation::ChangeLabPointNormalStress(change_lab_point_normal_stress::ChangeLabPointNormalStress { lab_test_id: nested_lab_tests.id.clone(), id: nested_lab_tests.points[0].id.clone(), new_normal_stress: 1.5 }),
        En1997Mutation::ChangeLabPointShearStress(change_lab_point_shear_stress::ChangeLabPointShearStress { lab_test_id: nested_lab_tests.id.clone(), id: nested_lab_tests.points[0].id.clone(), new_shear_stress: 1.5 }),
        En1997Mutation::ChangeWaterLayerId(change_water_layer_id::ChangeWaterLayerId { id: demo.layer_waters[0].id.clone(), new_layer_id: "alt".into() }),
        En1997Mutation::ChangeWaterPiezometricDepth(change_water_piezometric_depth::ChangeWaterPiezometricDepth { id: demo.layer_waters[0].id.clone(), new_piezometric_depth: 1.5 }),
        En1997Mutation::ChangeWaterPermeability(change_water_permeability::ChangeWaterPermeability { id: demo.layer_waters[0].id.clone(), new_permeability: 1.5 }),
        En1997Mutation::ChangeWaterHeadDifference(change_water_head_difference::ChangeWaterHeadDifference { id: demo.layer_waters[0].id.clone(), new_head_difference: 1.5 }),
        En1997Mutation::ChangeWaterFlowLength(change_water_flow_length::ChangeWaterFlowLength { id: demo.layer_waters[0].id.clone(), new_flow_length: 1.5 }),
        En1997Mutation::ChangeStructureId(change_structure_id::ChangeStructureId { new_structure_id: "alt".into() }),
        En1997Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { footing_id: nested_footings.id.clone(), index: 0, load_case: { let mut row = nested_footings.load_cases[0].clone(); row.id = "new".into(); row } }),
        En1997Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { footing_id: nested_footings.id.clone(), index: 0 }),
        En1997Mutation::InsertTestProfile(insert_test_profile::InsertTestProfile { pile_id: nested_piles.id.clone(), index: 0, test_profile: { let mut row = nested_piles.test_profiles[0].clone(); row.id = "new".into(); row } }),
        En1997Mutation::RemoveTestProfile(remove_test_profile::RemoveTestProfile { pile_id: nested_piles.id.clone(), index: 0 }),
        En1997Mutation::InsertAnchorTest(insert_anchor_test::InsertAnchorTest { anchor_id: nested_anchors.id.clone(), index: 0, anchor_test: { let mut row = nested_anchors.tests[0].clone(); row.id = "new".into(); row } }),
        En1997Mutation::RemoveAnchorTest(remove_anchor_test::RemoveAnchorTest { anchor_id: nested_anchors.id.clone(), index: 0 }),
        En1997Mutation::InsertLabPoint(insert_lab_point::InsertLabPoint { lab_test_id: nested_lab_tests.id.clone(), index: 0, lab_point: { let mut row = nested_lab_tests.points[0].clone(); row.id = "new".into(); row } }),
        En1997Mutation::RemoveLabPoint(remove_lab_point::RemoveLabPoint { lab_test_id: nested_lab_tests.id.clone(), index: 0 }),
    ]
}

fn every_mutation() -> Vec<En1997Mutation> {
    let demo = compliant_demo();
    let layer = demo.layers[0].clone();
    let footing = demo.footings[0].clone();
    let pile = demo.piles[0].clone();
    let retaining_wall_row = demo.retaining_walls[0].clone();
    let slope_row = demo.slopes[0].clone();
    let uplift_case_row = demo.uplift_cases[0].clone();
    let anchor_row = demo.anchors[0].clone();
    let embedded_retaining_wall_row = demo.embedded_walls[0].clone();
    let pile_group_row = demo.pile_groups[0].clone();
    let lab_test_row = demo.lab_tests[0].clone();
    let layer_water_row = demo.layer_waters[0].clone();
    let mut all = vec![
        En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::De }),
        En1997Mutation::ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory { new_geotechnical_category: 2 }),
        En1997Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation: semio_s_artifact_norm_en1997_model::SituationChoice::Persistent }),
        En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: semio_s_artifact_norm_en1997_model::ApproachChoice::Da2 }),
        En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level: -2.0 }),
        En1997Mutation::ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth { new_investigation_depth: 12.0 }),
        En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id: "f1".into(), new_width: 2.5 }),
        En1997Mutation::ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment { id: "f1".into(), new_embedment: 1.2 }),
        En1997Mutation::ChangePileLength(change_pile_length::ChangePileLength { id: "p1".into(), new_length: 14.0 }),
        En1997Mutation::ChangePileCount(change_pile_count::ChangePileCount { id: "p1".into(), new_count: 4 }),
        En1997Mutation::ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth { id: "w1".into(), new_base_width: 2.0 }),
        En1997Mutation::ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle { id: "s1".into(), new_angle_deg: 28.0 }),
        En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime { id: "L1".into(), new_phi_prime_deg: 32.0 }),
        En1997Mutation::ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus {
            id: "L1".into(),
            new_oedometric_modulus: 40e6,
        }),
        En1997Mutation::InsertLayer(insert_layer::InsertLayer { index: 0, layer: layer.clone() }),
        En1997Mutation::RemoveLayer(remove_layer::RemoveLayer { index: 0 }),
        En1997Mutation::InsertFooting(insert_footing::InsertFooting { index: 0, footing: footing.clone() }),
        En1997Mutation::RemoveFooting(remove_footing::RemoveFooting { index: 0 }),
        En1997Mutation::InsertPile(insert_pile::InsertPile { index: 0, pile: pile.clone() }),
        En1997Mutation::RemovePile(remove_pile::RemovePile { index: 0 }),
        En1997Mutation::InsertRetainingWall(insert_retaining_wall::InsertRetainingWall { index: 0, retaining_wall: retaining_wall_row.clone() }),
        En1997Mutation::RemoveRetainingWall(remove_retaining_wall::RemoveRetainingWall { index: 0 }),
        En1997Mutation::InsertSlope(insert_slope::InsertSlope { index: 0, slope: slope_row.clone() }),
        En1997Mutation::RemoveSlope(remove_slope::RemoveSlope { index: 0 }),
        En1997Mutation::InsertUpliftCase(insert_uplift_case::InsertUpliftCase { index: 0, uplift_case: uplift_case_row.clone() }),
        En1997Mutation::RemoveUpliftCase(remove_uplift_case::RemoveUpliftCase { index: 0 }),
        En1997Mutation::InsertAnchor(insert_anchor::InsertAnchor { index: 0, anchor: anchor_row.clone() }),
        En1997Mutation::RemoveAnchor(remove_anchor::RemoveAnchor { index: 0 }),
        En1997Mutation::InsertEmbeddedWall(insert_embedded_wall::InsertEmbeddedWall { index: 0, embedded_wall: embedded_retaining_wall_row.clone() }),
        En1997Mutation::RemoveEmbeddedWall(remove_embedded_wall::RemoveEmbeddedWall { index: 0 }),
        En1997Mutation::InsertPileGroup(insert_pile_group::InsertPileGroup { index: 0, pile_group: pile_group_row.clone() }),
        En1997Mutation::RemovePileGroup(remove_pile_group::RemovePileGroup { index: 0 }),
        En1997Mutation::InsertLabTest(insert_lab_test::InsertLabTest { index: 0, lab_test: lab_test_row.clone() }),
        En1997Mutation::RemoveLabTest(remove_lab_test::RemoveLabTest { index: 0 }),
        En1997Mutation::InsertLayerWater(insert_layer_water::InsertLayerWater { index: 0, layer_water: layer_water_row.clone() }),
        En1997Mutation::RemoveLayerWater(remove_layer_water::RemoveLayerWater { index: 0 }),
        En1997Mutation::InsertCase(insert_case::InsertCase { index: 0, case: demo.cases[0].clone() }),
        En1997Mutation::RemoveCase(remove_case::RemoveCase { id: "case-1".into() }),
        En1997Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: demo.cases[0].id.clone(), key: "m_dst_g".into(), value: Some(500_000.0) }),
        En1997Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id: demo.cases[0].id.clone(), key: "situation".into(), value: Some("bsT".into()) }),
        En1997Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: demo.cases[0].id.clone(), new_value: "Overturning".into() }),
        En1997Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: demo.cases[0].id.clone(), new_value: "Kippen".into() }),
        En1997Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: demo.cases[0].id.clone(), new_value: "UPL-01".into() }),
    ];
    all.extend(typed_samples(&demo));
    all
}

fn round_trip(base: &En1997Snapshot, mutation: &En1997Mutation) -> En1997Snapshot {
    let forward = vcs::apply_mutation(base, mutation).expect("valid mutation").0;
    let mut restored = forward.clone();
    for back in mutation.inverse(base).expect("valid retained mutation inverse fixture") {
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
    let base = compliant_demo();
    for mutation in every_mutation() {
        round_trip(&base, &mutation);
    }
}

//#region 🧪️MutationLaws
#[semio_framework_async_macros::async_test]
async fn change_annex_satisfies_the_inverse_and_absorb_laws() {
    let base = compliant_demo();
    let mutation = En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: semio_s_artifact_norm_en1997_model::ApproachChoice::Da1Str }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_footing_width_satisfies_the_inverse_and_absorb_laws() {
    let base = compliant_demo();
    let mutation = En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id: base.footings[0].id.clone(), new_width: 3.0 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level: -1.5 }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_design_approach_satisfies_the_inverse_and_absorb_laws() {
    let base = compliant_demo();
    let mutation = En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach: semio_s_artifact_norm_en1997_model::ApproachChoice::Da3 });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime { id: base.layers[0].id.clone(), new_phi_prime_deg: 35.0 }).diff(&base).diff().clone();
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws
