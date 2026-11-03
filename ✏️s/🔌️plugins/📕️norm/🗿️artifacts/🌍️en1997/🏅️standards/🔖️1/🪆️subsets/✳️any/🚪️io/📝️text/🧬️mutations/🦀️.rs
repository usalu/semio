//! 🔧️ En1997 artifact — OpText/OpBinary codecs for `En1997Mutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_plugin_norm_compliance::AnnexChoice;
use crate::En1997Mutation;
use semio_s_artifact_norm_en1997_mut_design::{change_annex, change_design_approach, change_design_situation, change_geotechnical_category, change_groundwater_level, change_investigation_depth};
use semio_s_artifact_norm_en1997_mut_placements::{change_placement_x, change_placement_y, change_placement_yaw, change_placement_z, insert_placement, remove_placement};
use semio_s_artifact_norm_en1997_mut_foundations::{change_footing_embedment, change_footing_width, change_pile_count, change_pile_length, change_wall_base_width, insert_footing, insert_pile, remove_footing, remove_pile};
use semio_s_artifact_norm_en1997_mut_ground::{change_layer_oedometric_modulus, change_layer_phi_prime, change_slope_angle, insert_layer, remove_layer};
use semio_s_artifact_norm_en1997_mut_rows::{insert_retaining_wall, remove_retaining_wall, insert_slope, remove_slope};
use semio_s_artifact_norm_en1997_mut_structures::{insert_uplift_case, remove_uplift_case, insert_anchor, remove_anchor, insert_embedded_wall, remove_embedded_wall};
use semio_s_artifact_norm_en1997_mut_investigation::{insert_pile_group, remove_pile_group, insert_lab_test, remove_lab_test, insert_layer_water, remove_layer_water, insert_ground_investigation, remove_ground_investigation, insert_characteristic_basis, remove_characteristic_basis};
use semio_s_artifact_norm_en1997_mut_anchor::{change_anchor_fixed_diameter, change_anchor_fixed_length, change_anchor_free_length, change_anchor_head_depth, change_anchor_inclination_deg, change_anchor_tendon, change_anchor_wall_id};
use semio_s_artifact_norm_en1997_mut_anchor_proof::{change_anchor_bond_stress, change_anchor_design_force, change_anchor_min_free_length, change_anchor_tendon_yield_force, change_anchor_test_factor};
use semio_s_artifact_norm_en1997_mut_anchor_test::{change_anchor_test_pull_out_res};
use semio_s_artifact_norm_en1997_mut_embedded_wall::{change_emb_wall_backfill_gamma, change_emb_wall_embedment, change_emb_wall_kind, change_emb_wall_phi_deg, change_emb_wall_prop_depth, change_emb_wall_retained_height, change_emb_wall_thickness};
use semio_s_artifact_norm_en1997_mut_embedded_wall_pressure::{change_emb_wall_delta_active_deg, change_emb_wall_delta_passive_deg, change_emb_wall_surcharge, change_emb_wall_undr_strength};
use semio_s_artifact_norm_en1997_mut_footing::{change_footing_base_incl_deg, change_footing_length, change_footing_settlement_limit};
use semio_s_artifact_norm_en1997_mut_lab_point::{change_lab_point_normal_stress, change_lab_point_shear_stress};
use semio_s_artifact_norm_en1997_mut_lab_test::{change_lab_test_consol_time, change_lab_test_drainage_height, change_lab_test_kind, change_lab_test_layer_id, change_lab_test_preload_stress, change_lab_test_void_ratio};
use semio_s_artifact_norm_en1997_mut_layer::{change_layer_cohesion_effective, change_layer_cohesion_undrained, change_layer_depth_bottom, change_layer_depth_top, change_layer_gamma, change_layer_gamma_prime, change_layer_soil_type};
use semio_s_artifact_norm_en1997_mut_layer_investigation::{change_layer_cpt_qc, change_layer_poisson_ratio, change_layer_spt_n};
use semio_s_artifact_norm_en1997_mut_layer_water::{change_water_flow_length, change_water_head_difference, change_water_layer_id, change_water_permeability, change_water_piezometric_depth};
use semio_s_artifact_norm_en1997_mut_load_case::{change_load_case_design_situation, change_load_case_horiz_permanent, change_load_case_horiz_variable, change_load_case_moment_permanent, change_load_case_moment_variable, change_load_case_vertical_perm, change_load_case_vertical_var};
use semio_s_artifact_norm_en1997_mut_pile::{change_pile_alpha_s, change_pile_compression_permanent, change_pile_compression_variable, change_pile_diameter, change_pile_pile_type, change_pile_unit_base_resistance, change_pile_unit_shaft_resistance};
use semio_s_artifact_norm_en1997_mut_pile_group::{change_pile_group_cap_moment_x, change_pile_group_cap_moment_y, change_pile_group_cap_vertical, change_pile_group_columns, change_pile_group_pile_id, change_pile_group_rows, change_pile_group_spacing};
use semio_s_artifact_norm_en1997_mut_pile_group_resistance::{change_pile_group_base_cohesion, change_pile_group_shaft_coh_sum, change_pile_group_single_res};
use semio_s_artifact_norm_en1997_mut_pile_tension::{change_pile_tension_permanent, change_pile_tension_variable};
use semio_s_artifact_norm_en1997_mut_profile::{change_profile_base_resistance, change_profile_shaft_resistance};
use semio_s_artifact_norm_en1997_mut_slope::{change_slope_circle_centre_x, change_slope_circle_centre_z, change_slope_circle_radius, change_slope_governing_layer_id, change_slope_height, change_slope_length};
use semio_s_artifact_norm_en1997_mut_uplift::{change_uplift_perm_destabilizing, change_uplift_perm_stabilizing, change_uplift_pore_pressure, change_uplift_total_stress, change_uplift_var_destabilizing};
use semio_s_artifact_norm_en1997_mut_wall::{change_wall_backfill_gamma, change_wall_backfill_phi_deg, change_wall_earth_pressure_mode, change_wall_embedment, change_wall_height, change_wall_stem_thickness, change_wall_wall_friction_deg};
use semio_s_artifact_norm_en1997_mut_wall_loads::{change_wall_concrete_gamma, change_wall_horizontal_permanent, change_wall_ocr, change_wall_surcharge, change_wall_vertical_permanent, change_wall_wall_movement};
use semio_s_artifact_norm_en1997_mut_design::{change_structure_id};
use semio_s_artifact_norm_en1997_mut_nested_rows::{insert_anchor_test, insert_lab_point, insert_load_case, insert_test_profile, remove_anchor_test, remove_lab_point, remove_load_case, remove_test_profile, insert_ground_investigation_reading, remove_ground_investigation_reading};
use semio_s_artifact_norm_en1997_mut_design_cases::{insert_case, remove_case, change_case_number, change_case_text, change_case_label_en, change_case_label_de, change_case_entry_id};
use semio_s_artifact_norm_en1997_mut_slope::{change_slope_width};
use semio_s_artifact_norm_en1997_mut_pile::{change_pile_bending_stiffness_ei, change_pile_horiz_bed_modulus, change_pile_lateral_head_load};
use semio_s_artifact_norm_en1997_mut_wall::{change_wall_seismic_kh, change_wall_seismic_kv};
use semio_s_artifact_norm_en1997_mut_lab_test::{change_lab_test_compression_index, change_lab_test_design_life, change_lab_test_sec_compr_index, change_lab_test_stress_increment, change_lab_test_swelling_index};
use semio_s_artifact_norm_en1997_mut_layer_water::{change_water_influence_radius, change_water_well_capacity, change_water_well_radius, change_water_well_water_depth};
use semio_s_artifact_norm_en1997_mut_pile::{change_pile_dynamic_res_mean, change_pile_dynamic_res_min, change_pile_dynamic_test_count, change_pile_elastic_modulus, change_pile_hammer_weight, change_pile_neg_friction_beta, change_pile_settling_thickness};
use semio_s_artifact_norm_en1997_mut_pile_tension::{change_pile_drop_height, change_pile_elastic_compression, change_pile_hammer_efficiency, change_pile_set_per_blow};
use semio_s_artifact_norm_en1997_mut_wall::{change_wall_rebar_friction, change_wall_reinforcement_spacing, change_wall_reinforcement_width};
use semio_s_artifact_norm_en1997_mut_pile::{change_pile_concrete_class};
use semio_s_artifact_norm_en1997_mut_layer::{change_layer_cpt_nkt};
use semio_s_artifact_norm_en1997_model::{PileConcreteClass, CaseRow, Placement, AnchorTest, FoundationLoadCase, LabPoint, PileTestProfile, Anchor, EmbeddedWall, LabTest, LayerWater, Pile, PileGroup, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase, Investigation, InvestigationReading, CharacteristicBasis};
use semio_s_artifact_norm_en1997_mut_characteristic_basis::{change_characteristic_basis_basis, change_characteristic_basis_layer_id, change_characteristic_basis_parameter};
use semio_s_artifact_norm_en1997_model::{CharacteristicParameter, ParameterBasis, PileConcreteClass, CaseRow, Placement, AnchorTest, FoundationLoadCase, LabPoint, PileTestProfile, Anchor, EmbeddedWall, LabTest, LayerWater, Pile, PileGroup, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase, Investigation, InvestigationReading, CharacteristicBasis};
use semio_s_artifact_norm_en1997_mut_ground_investigation::{change_ground_investigation_depth, change_ground_investigation_level, change_ground_investigation_kind, change_ground_investigation_position_x, change_ground_investigation_position_y};
use semio_s_artifact_norm_en1997_mut_ground_investigation_reading::{change_ground_reading_depth, change_ground_reading_measurement};
use semio_s_artifact_norm_en1997_model::{InvestigationKind, PileConcreteClass, CaseRow, Placement, AnchorTest, FoundationLoadCase, LabPoint, PileTestProfile, Anchor, EmbeddedWall, LabTest, LayerWater, Pile, PileGroup, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase, Investigation, InvestigationReading};
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `En1997Mutation`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum En1997MutationDsl {
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeGeotechnicalCategory { new_geotechnical_category: u8 },
    ChangeDesignSituation { new_design_situation: String },
    ChangeDesignApproach { new_design_approach: String },
    ChangeGroundwaterLevel { new_groundwater_level: f64 },
    ChangeInvestigationDepth { new_investigation_depth: f64 },
    ChangeFootingWidth { id: String, new_width: f64 },
    ChangeFootingEmbedment { id: String, new_embedment: f64 },
    ChangePileLength { id: String, new_length: f64 },
    ChangePileCount { id: String, new_count: u32 },
    ChangeWallBaseWidth { id: String, new_base_width: f64 },
    ChangeSlopeAngle { id: String, new_angle_deg: f64 },
    ChangeLayerPhiPrime { id: String, new_phi_prime_deg: f64 },
    ChangeLayerOedometricModulus { id: String, new_oedometric_modulus: f64 },
    InsertLayer {
        index: usize,
        #[dsl(block)]
        layer: SoilLayer,
    },
    RemoveLayer { index: usize },
    InsertFooting {
        index: usize,
        #[dsl(block)]
        footing: SpreadFoundation,
    },
    RemoveFooting { index: usize },
    InsertPile {
        index: usize,
        #[dsl(block)]
        pile: Pile,
    },
    RemovePile { index: usize },
    InsertRetainingWall {
        index: usize,
        #[dsl(block)]
        retaining_wall: RetainingWall,
    },
    RemoveRetainingWall { index: usize },
    InsertSlope {
        index: usize,
        #[dsl(block)]
        slope: Slope,
    },
    RemoveSlope { index: usize },
    InsertUpliftCase {
        index: usize,
        #[dsl(block)]
        uplift_case: UpliftCase,
    },
    RemoveUpliftCase { index: usize },
    InsertAnchor {
        index: usize,
        #[dsl(block)]
        anchor: Anchor,
    },
    RemoveAnchor { index: usize },
    InsertEmbeddedWall {
        index: usize,
        #[dsl(block)]
        embedded_wall: EmbeddedWall,
    },
    RemoveEmbeddedWall { index: usize },
    InsertPileGroup {
        index: usize,
        #[dsl(block)]
        pile_group: PileGroup,
    },
    RemovePileGroup { index: usize },
    InsertLabTest {
        index: usize,
        #[dsl(block)]
        lab_test: LabTest,
    },
    RemoveLabTest { index: usize },
    InsertLayerWater {
        index: usize,
        #[dsl(block)]
        layer_water: LayerWater,
    },
    RemoveLayerWater { index: usize },
    InsertCharacteristicBasis {
        index: usize,
        #[dsl(block)]
        characteristic_basis: CharacteristicBasis,
    },
    RemoveCharacteristicBasis { index: usize },
    InsertGroundInvestigation {
        index: usize,
        #[dsl(block)]
        investigation: Investigation,
    },
    RemoveGroundInvestigation { index: usize },
    ChangeLayerSoilType { id: String, new_soil_type: String },
    ChangeLayerDepthTop { id: String, new_depth_top: f64 },
    ChangeLayerDepthBottom { id: String, new_depth_bottom: f64 },
    ChangeLayerGamma { id: String, new_gamma: f64 },
    ChangeLayerGammaPrime { id: String, new_gamma_prime: f64 },
    ChangeLayerCohesionEffective { id: String, new_cohesion_effective: f64 },
    ChangeLayerCohesionUndrained { id: String, new_cohesion_undrained: f64 },
    ChangeLayerPoissonRatio { id: String, new_poisson_ratio: f64 },
    ChangeLayerCptQc { id: String, new_cpt_qc: f64 },
    ChangeLayerSptN { id: String, new_spt_n: f64 },
    ChangeFootingLength { id: String, new_length: f64 },
    ChangeFootingBaseInclDeg { id: String, new_base_inclination_deg: f64 },
    ChangeFootingSettlementLimit { id: String, new_settlement_limit: f64 },
    ChangeLoadCaseDesignSituation { footing_id: String, id: String, new_design_situation: String },
    ChangeLoadCaseVerticalPerm { footing_id: String, id: String, new_vertical_permanent: f64 },
    ChangeLoadCaseVerticalVar { footing_id: String, id: String, new_vertical_variable: f64 },
    ChangeLoadCaseHorizPermanent { footing_id: String, id: String, new_horizontal_permanent: f64 },
    ChangeLoadCaseHorizVariable { footing_id: String, id: String, new_horizontal_variable: f64 },
    ChangeLoadCaseMomentPermanent { footing_id: String, id: String, new_moment_permanent: f64 },
    ChangeLoadCaseMomentVariable { footing_id: String, id: String, new_moment_variable: f64 },
    ChangePilePileType { id: String, new_pile_type: String },
    ChangePileDiameter { id: String, new_diameter: f64 },
    ChangePileAlphaS { id: String, new_alpha_s: f64 },
    ChangePileUnitShaftResistance { id: String, new_unit_shaft_resistance: f64 },
    ChangePileUnitBaseResistance { id: String, new_unit_base_resistance: f64 },
    ChangePileCompressionPermanent { id: String, new_compression_permanent: f64 },
    ChangePileCompressionVariable { id: String, new_compression_variable: f64 },
    ChangePileTensionPermanent { id: String, new_tension_permanent: f64 },
    ChangePileTensionVariable { id: String, new_tension_variable: f64 },
    ChangeProfileShaftResistance { pile_id: String, id: String, new_shaft_resistance: f64 },
    ChangeProfileBaseResistance { pile_id: String, id: String, new_base_resistance: f64 },
    ChangeWallHeight { id: String, new_height: f64 },
    ChangeWallEmbedment { id: String, new_embedment: f64 },
    ChangeWallStemThickness { id: String, new_stem_thickness: f64 },
    ChangeWallBackfillPhiDeg { id: String, new_backfill_phi_deg: f64 },
    ChangeWallBackfillGamma { id: String, new_backfill_gamma: f64 },
    ChangeWallWallFrictionDeg { id: String, new_wall_friction_deg: f64 },
    ChangeWallEarthPressureMode { id: String, new_earth_pressure_mode: String },
    ChangeWallWallMovement { id: String, new_wall_movement: String },
    ChangeWallOcr { id: String, new_ocr: f64 },
    ChangeWallConcreteGamma { id: String, new_concrete_gamma: f64 },
    ChangeWallSurcharge { id: String, new_surcharge: f64 },
    ChangeWallVerticalPermanent { id: String, new_vertical_permanent: f64 },
    ChangeWallHorizontalPermanent { id: String, new_horizontal_permanent: f64 },
    ChangeSlopeHeight { id: String, new_height: f64 },
    ChangeSlopeLength { id: String, new_length: f64 },
    ChangeSlopeGoverningLayerId { id: String, new_governing_layer_id: String },
    ChangeSlopeCircleCentreX { id: String, new_circle_centre_x: f64 },
    ChangeSlopeCircleCentreZ { id: String, new_circle_centre_z: f64 },
    ChangeSlopeCircleRadius { id: String, new_circle_radius: f64 },
    ChangeUpliftPermStabilizing { id: String, new_permanent_stabilizing: f64 },
    ChangeUpliftPermDestabilizing { id: String, new_permanent_destabilizing: f64 },
    ChangeUpliftVarDestabilizing { id: String, new_variable_destabilizing: f64 },
    ChangeUpliftPorePressure { id: String, new_pore_pressure: f64 },
    ChangeUpliftTotalStress { id: String, new_total_stress: f64 },
    ChangeAnchorWallId { id: String, new_wall_id: String },
    ChangeAnchorTendon { id: String, new_tendon: String },
    ChangeAnchorHeadDepth { id: String, new_head_depth: f64 },
    ChangeAnchorInclinationDeg { id: String, new_inclination_deg: f64 },
    ChangeAnchorFreeLength { id: String, new_free_length: f64 },
    ChangeAnchorFixedLength { id: String, new_fixed_length: f64 },
    ChangeAnchorFixedDiameter { id: String, new_fixed_diameter: f64 },
    ChangeAnchorBondStress { id: String, new_bond_stress: f64 },
    ChangeAnchorDesignForce { id: String, new_design_force: f64 },
    ChangeAnchorTendonYieldForce { id: String, new_tendon_yield_force: f64 },
    ChangeAnchorMinFreeLength { id: String, new_min_free_length: f64 },
    ChangeAnchorTestFactor { id: String, new_test_factor: f64 },
    ChangeAnchorTestPullOutRes { anchor_id: String, id: String, new_pull_out_resistance: f64 },
    ChangeEmbWallKind { id: String, new_kind: String },
    ChangeEmbWallRetainedHeight { id: String, new_retained_height: f64 },
    ChangeEmbWallEmbedment { id: String, new_embedment: f64 },
    ChangeEmbWallPropDepth { id: String, new_prop_depth: f64 },
    ChangeEmbWallThickness { id: String, new_thickness: f64 },
    ChangeEmbWallBackfillGamma { id: String, new_backfill_gamma: f64 },
    ChangeEmbWallPhiDeg { id: String, new_phi_deg: f64 },
    ChangeEmbWallDeltaActiveDeg { id: String, new_delta_active_deg: f64 },
    ChangeEmbWallDeltaPassiveDeg { id: String, new_delta_passive_deg: f64 },
    ChangeEmbWallSurcharge { id: String, new_surcharge: f64 },
    ChangeEmbWallUndrStrength { id: String, new_undrained_strength: f64 },
    ChangePileGroupPileId { id: String, new_pile_id: String },
    ChangePileGroupRows { id: String, new_rows: u32 },
    ChangePileGroupColumns { id: String, new_columns: u32 },
    ChangePileGroupSpacing { id: String, new_spacing: f64 },
    ChangePileGroupCapVertical { id: String, new_cap_vertical: f64 },
    ChangePileGroupCapMomentX { id: String, new_cap_moment_x: f64 },
    ChangePileGroupCapMomentY { id: String, new_cap_moment_y: f64 },
    ChangePileGroupSingleRes { id: String, new_single_resistance: f64 },
    ChangePileGroupShaftCohSum { id: String, new_shaft_cohesion_sum: f64 },
    ChangePileGroupBaseCohesion { id: String, new_base_cohesion: f64 },
    ChangeLabTestLayerId { id: String, new_layer_id: String },
    ChangeLabTestKind { id: String, new_kind: String },
    ChangeLabTestVoidRatio { id: String, new_void_ratio: f64 },
    ChangeLabTestPreloadStress { id: String, new_preload_stress: f64 },
    ChangeLabTestDrainageHeight { id: String, new_drainage_height: f64 },
    ChangeLabTestConsolTime { id: String, new_consolidation_time: f64 },
    ChangeLabPointNormalStress { lab_test_id: String, id: String, new_normal_stress: f64 },
    ChangeLabPointShearStress { lab_test_id: String, id: String, new_shear_stress: f64 },
    ChangeWaterLayerId { id: String, new_layer_id: String },
    ChangeWaterPiezometricDepth { id: String, new_piezometric_depth: f64 },
    ChangeWaterPermeability { id: String, new_permeability: f64 },
    ChangeWaterHeadDifference { id: String, new_head_difference: f64 },
    ChangeWaterFlowLength { id: String, new_flow_length: f64 },
    ChangeStructureId { new_structure_id: String },
    InsertLoadCase {
        footing_id: String,
        index: usize,
        #[dsl(block)]
        load_case: FoundationLoadCase,
    },
    RemoveLoadCase { footing_id: String, index: usize },
    InsertTestProfile {
        pile_id: String,
        index: usize,
        #[dsl(block)]
        test_profile: PileTestProfile,
    },
    RemoveTestProfile { pile_id: String, index: usize },
    InsertAnchorTest {
        anchor_id: String,
        index: usize,
        #[dsl(block)]
        anchor_test: AnchorTest,
    },
    RemoveAnchorTest { anchor_id: String, index: usize },
    InsertLabPoint {
        lab_test_id: String,
        index: usize,
        #[dsl(block)]
        lab_point: LabPoint,
    },
    RemoveLabPoint { lab_test_id: String, index: usize },
    InsertGroundInvestigationReading {
        investigation_id: String,
        index: usize,
        #[dsl(block)]
        investigation_reading: InvestigationReading,
    },
    RemoveGroundInvestigationReading { investigation_id: String, index: usize },
    ChangePlacementX { id: String, new_x: f64 },
    ChangePlacementY { id: String, new_y: f64 },
    ChangePlacementZ { id: String, new_z: f64 },
    ChangePlacementYaw { id: String, new_yaw: f64 },
    InsertPlacement {
        index: usize,
        #[dsl(block)]
        item: Placement,
    },
    RemovePlacement { index: usize },
    InsertCase {
        index: usize,
        #[dsl(block)]
        case: CaseRow,
    },
    RemoveCase { id: String },
    ChangeCaseNumber { case_id: String, key: String, value: Option<f64> },
    ChangeCaseText { case_id: String, key: String, value: Option<String> },
    ChangeCaseLabelEn { case_id: String, new_value: String },
    ChangeCaseLabelDe { case_id: String, new_value: String },
    ChangeCaseEntryId { case_id: String, new_value: String },
    ChangeSlopeWidth { id: String, new_width: f64 },
    ChangePileBendingStiffnessEi { id: String, new_bending_stiffness_ei: f64 },
    ChangePileHorizBedModulus { id: String, new_horizontal_subgrade_modulus: f64 },
    ChangePileLateralHeadLoad { id: String, new_lateral_head_load: f64 },
    ChangeWallSeismicKh { id: String, new_seismic_kh: f64 },
    ChangeWallSeismicKv { id: String, new_seismic_kv: f64 },

    ChangePileElasticModulus { id: String, new_elastic_modulus: f64 },
    ChangePileSettlingThickness { id: String, new_settling_thickness: f64 },
    ChangePileNegFrictionBeta { id: String, new_negative_friction_beta: f64 },
    ChangePileDynamicTestCount { id: String, new_dynamic_test_count: f64 },
    ChangePileDynamicResMean { id: String, new_dynamic_resistance_mean: f64 },
    ChangePileDynamicResMin { id: String, new_dynamic_resistance_min: f64 },
    ChangePileHammerWeight { id: String, new_hammer_weight: f64 },
    ChangePileDropHeight { id: String, new_drop_height: f64 },
    ChangePileSetPerBlow { id: String, new_set_per_blow: f64 },
    ChangePileElasticCompression { id: String, new_elastic_compression: f64 },
    ChangePileHammerEfficiency { id: String, new_hammer_efficiency: f64 },
    ChangeWallReinforcementSpacing { id: String, new_reinforcement_spacing: f64 },
    ChangeWallReinforcementWidth { id: String, new_reinforcement_width: f64 },
    ChangeWallRebarFriction { id: String, new_reinforcement_friction: f64 },
    ChangeLabTestCompressionIndex { id: String, new_compression_index: f64 },
    ChangeLabTestSwellingIndex { id: String, new_swelling_index: f64 },
    ChangeLabTestStressIncrement { id: String, new_stress_increment: f64 },
    ChangeLabTestSecComprIndex { id: String, new_secondary_compression_index: f64 },
    ChangeLabTestDesignLife { id: String, new_design_life: f64 },
    ChangeWaterWellRadius { id: String, new_well_radius: f64 },
    ChangeWaterWellWaterDepth { id: String, new_well_water_depth: f64 },
    ChangeWaterInfluenceRadius { id: String, new_influence_radius: f64 },
    ChangeWaterWellCapacity { id: String, new_well_capacity: f64 },
    ChangePileConcreteClass { id: String, new_concrete_class: PileConcreteClass },
    ChangeGroundInvestigationPositionX { id: String, new_position_x: f64 },
    ChangeGroundInvestigationPositionY { id: String, new_position_y: f64 },
    ChangeGroundInvestigationLevel { id: String, new_ground_level: f64 },
    ChangeGroundInvestigationDepth { id: String, new_depth: f64 },
    ChangeGroundInvestigationKind { id: String, new_kind: InvestigationKind },
    ChangeGroundReadingDepth { investigation_id: String, id: String, new_depth: f64 },
    ChangeGroundReadingMeasurement { investigation_id: String, id: String, new_measurement: f64 },
    ChangeCharacteristicBasisLayerId { id: String, new_layer_id: String },
    ChangeCharacteristicBasisParameter { id: String, new_parameter: CharacteristicParameter },
    ChangeCharacteristicBasisBasis { id: String, new_basis: ParameterBasis },
    ChangeLayerCptNkt { id: String, new_cpt_nkt: f64 },
}

impl OpText for En1997MutationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

//#endregion 🔖️HandcraftedOpCodecs

pub fn en1997_mutation_to_dsl(mutation: &En1997Mutation) -> En1997MutationDsl {
    match mutation {
        En1997Mutation::ChangeAnnex(payload) => En1997MutationDsl::ChangeAnnex { new_annex: payload.new_annex },
        En1997Mutation::ChangeGeotechnicalCategory(payload) => En1997MutationDsl::ChangeGeotechnicalCategory { new_geotechnical_category: payload.new_geotechnical_category },
        En1997Mutation::ChangeDesignSituation(payload) => En1997MutationDsl::ChangeDesignSituation { new_design_situation: payload.new_design_situation.clone() },
        En1997Mutation::ChangeDesignApproach(payload) => En1997MutationDsl::ChangeDesignApproach { new_design_approach: payload.new_design_approach.clone() },
        En1997Mutation::ChangeGroundwaterLevel(payload) => En1997MutationDsl::ChangeGroundwaterLevel { new_groundwater_level: payload.new_groundwater_level },
        En1997Mutation::ChangeInvestigationDepth(payload) => En1997MutationDsl::ChangeInvestigationDepth { new_investigation_depth: payload.new_investigation_depth },
        En1997Mutation::ChangeFootingWidth(payload) => En1997MutationDsl::ChangeFootingWidth { id: payload.id.clone(), new_width: payload.new_width },
        En1997Mutation::ChangeFootingEmbedment(payload) => En1997MutationDsl::ChangeFootingEmbedment { id: payload.id.clone(), new_embedment: payload.new_embedment },
        En1997Mutation::ChangePileLength(payload) => En1997MutationDsl::ChangePileLength { id: payload.id.clone(), new_length: payload.new_length },
        En1997Mutation::ChangePileCount(payload) => En1997MutationDsl::ChangePileCount { id: payload.id.clone(), new_count: payload.new_count },
        En1997Mutation::ChangeWallBaseWidth(payload) => En1997MutationDsl::ChangeWallBaseWidth { id: payload.id.clone(), new_base_width: payload.new_base_width },
        En1997Mutation::ChangeSlopeAngle(payload) => En1997MutationDsl::ChangeSlopeAngle { id: payload.id.clone(), new_angle_deg: payload.new_angle_deg },
        En1997Mutation::ChangeLayerPhiPrime(payload) => En1997MutationDsl::ChangeLayerPhiPrime { id: payload.id.clone(), new_phi_prime_deg: payload.new_phi_prime_deg },
        En1997Mutation::ChangeLayerOedometricModulus(payload) => En1997MutationDsl::ChangeLayerOedometricModulus { id: payload.id.clone(), new_oedometric_modulus: payload.new_oedometric_modulus },
        En1997Mutation::InsertLayer(payload) => En1997MutationDsl::InsertLayer { index: payload.index, layer: payload.layer.clone() },
        En1997Mutation::RemoveLayer(payload) => En1997MutationDsl::RemoveLayer { index: payload.index },
        En1997Mutation::InsertFooting(payload) => En1997MutationDsl::InsertFooting { index: payload.index, footing: payload.footing.clone() },
        En1997Mutation::RemoveFooting(payload) => En1997MutationDsl::RemoveFooting { index: payload.index },
        En1997Mutation::InsertPile(payload) => En1997MutationDsl::InsertPile { index: payload.index, pile: payload.pile.clone() },
        En1997Mutation::RemovePile(payload) => En1997MutationDsl::RemovePile { index: payload.index },
        En1997Mutation::InsertRetainingWall(payload) => En1997MutationDsl::InsertRetainingWall { index: payload.index, retaining_wall: payload.retaining_wall.clone() },
        En1997Mutation::RemoveRetainingWall(payload) => En1997MutationDsl::RemoveRetainingWall { index: payload.index },
        En1997Mutation::InsertSlope(payload) => En1997MutationDsl::InsertSlope { index: payload.index, slope: payload.slope.clone() },
        En1997Mutation::RemoveSlope(payload) => En1997MutationDsl::RemoveSlope { index: payload.index },
        En1997Mutation::InsertUpliftCase(payload) => En1997MutationDsl::InsertUpliftCase { index: payload.index, uplift_case: payload.uplift_case.clone() },
        En1997Mutation::RemoveUpliftCase(payload) => En1997MutationDsl::RemoveUpliftCase { index: payload.index },
        En1997Mutation::InsertAnchor(payload) => En1997MutationDsl::InsertAnchor { index: payload.index, anchor: payload.anchor.clone() },
        En1997Mutation::RemoveAnchor(payload) => En1997MutationDsl::RemoveAnchor { index: payload.index },
        En1997Mutation::InsertEmbeddedWall(payload) => En1997MutationDsl::InsertEmbeddedWall { index: payload.index, embedded_wall: payload.embedded_wall.clone() },
        En1997Mutation::RemoveEmbeddedWall(payload) => En1997MutationDsl::RemoveEmbeddedWall { index: payload.index },
        En1997Mutation::InsertPileGroup(payload) => En1997MutationDsl::InsertPileGroup { index: payload.index, pile_group: payload.pile_group.clone() },
        En1997Mutation::RemovePileGroup(payload) => En1997MutationDsl::RemovePileGroup { index: payload.index },
        En1997Mutation::InsertLabTest(payload) => En1997MutationDsl::InsertLabTest { index: payload.index, lab_test: payload.lab_test.clone() },
        En1997Mutation::RemoveLabTest(payload) => En1997MutationDsl::RemoveLabTest { index: payload.index },
        En1997Mutation::InsertLayerWater(payload) => En1997MutationDsl::InsertLayerWater { index: payload.index, layer_water: payload.layer_water.clone() },
        En1997Mutation::RemoveLayerWater(payload) => En1997MutationDsl::RemoveLayerWater { index: payload.index },
        En1997Mutation::InsertCharacteristicBasis(payload) => En1997MutationDsl::InsertCharacteristicBasis { index: payload.index, characteristic_basis: payload.characteristic_basis.clone() },
        En1997Mutation::RemoveCharacteristicBasis(payload) => En1997MutationDsl::RemoveCharacteristicBasis { index: payload.index },
        En1997Mutation::InsertGroundInvestigation(payload) => En1997MutationDsl::InsertGroundInvestigation { index: payload.index, investigation: payload.investigation.clone() },
        En1997Mutation::RemoveGroundInvestigation(payload) => En1997MutationDsl::RemoveGroundInvestigation { index: payload.index },
        En1997Mutation::ChangeLayerSoilType(payload) => En1997MutationDsl::ChangeLayerSoilType { id: payload.id.clone(), new_soil_type: payload.new_soil_type.clone() },
        En1997Mutation::ChangeLayerDepthTop(payload) => En1997MutationDsl::ChangeLayerDepthTop { id: payload.id.clone(), new_depth_top: payload.new_depth_top },
        En1997Mutation::ChangeLayerDepthBottom(payload) => En1997MutationDsl::ChangeLayerDepthBottom { id: payload.id.clone(), new_depth_bottom: payload.new_depth_bottom },
        En1997Mutation::ChangeLayerGamma(payload) => En1997MutationDsl::ChangeLayerGamma { id: payload.id.clone(), new_gamma: payload.new_gamma },
        En1997Mutation::ChangeLayerGammaPrime(payload) => En1997MutationDsl::ChangeLayerGammaPrime { id: payload.id.clone(), new_gamma_prime: payload.new_gamma_prime },
        En1997Mutation::ChangeLayerCohesionEffective(payload) => En1997MutationDsl::ChangeLayerCohesionEffective { id: payload.id.clone(), new_cohesion_effective: payload.new_cohesion_effective },
        En1997Mutation::ChangeLayerCohesionUndrained(payload) => En1997MutationDsl::ChangeLayerCohesionUndrained { id: payload.id.clone(), new_cohesion_undrained: payload.new_cohesion_undrained },
        En1997Mutation::ChangeLayerPoissonRatio(payload) => En1997MutationDsl::ChangeLayerPoissonRatio { id: payload.id.clone(), new_poisson_ratio: payload.new_poisson_ratio },
        En1997Mutation::ChangeLayerCptQc(payload) => En1997MutationDsl::ChangeLayerCptQc { id: payload.id.clone(), new_cpt_qc: payload.new_cpt_qc },
        En1997Mutation::ChangeLayerSptN(payload) => En1997MutationDsl::ChangeLayerSptN { id: payload.id.clone(), new_spt_n: payload.new_spt_n },
        En1997Mutation::ChangeFootingLength(payload) => En1997MutationDsl::ChangeFootingLength { id: payload.id.clone(), new_length: payload.new_length },
        En1997Mutation::ChangeFootingBaseInclDeg(payload) => En1997MutationDsl::ChangeFootingBaseInclDeg { id: payload.id.clone(), new_base_inclination_deg: payload.new_base_inclination_deg },
        En1997Mutation::ChangeFootingSettlementLimit(payload) => En1997MutationDsl::ChangeFootingSettlementLimit { id: payload.id.clone(), new_settlement_limit: payload.new_settlement_limit },
        En1997Mutation::ChangeLoadCaseDesignSituation(payload) => En1997MutationDsl::ChangeLoadCaseDesignSituation { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_design_situation: payload.new_design_situation.clone() },
        En1997Mutation::ChangeLoadCaseVerticalPerm(payload) => En1997MutationDsl::ChangeLoadCaseVerticalPerm { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_vertical_permanent: payload.new_vertical_permanent },
        En1997Mutation::ChangeLoadCaseVerticalVar(payload) => En1997MutationDsl::ChangeLoadCaseVerticalVar { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_vertical_variable: payload.new_vertical_variable },
        En1997Mutation::ChangeLoadCaseHorizPermanent(payload) => En1997MutationDsl::ChangeLoadCaseHorizPermanent { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_horizontal_permanent: payload.new_horizontal_permanent },
        En1997Mutation::ChangeLoadCaseHorizVariable(payload) => En1997MutationDsl::ChangeLoadCaseHorizVariable { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_horizontal_variable: payload.new_horizontal_variable },
        En1997Mutation::ChangeLoadCaseMomentPermanent(payload) => En1997MutationDsl::ChangeLoadCaseMomentPermanent { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_moment_permanent: payload.new_moment_permanent },
        En1997Mutation::ChangeLoadCaseMomentVariable(payload) => En1997MutationDsl::ChangeLoadCaseMomentVariable { footing_id: payload.footing_id.clone(), id: payload.id.clone(), new_moment_variable: payload.new_moment_variable },
        En1997Mutation::ChangePilePileType(payload) => En1997MutationDsl::ChangePilePileType { id: payload.id.clone(), new_pile_type: payload.new_pile_type.clone() },
        En1997Mutation::ChangePileDiameter(payload) => En1997MutationDsl::ChangePileDiameter { id: payload.id.clone(), new_diameter: payload.new_diameter },
        En1997Mutation::ChangePileAlphaS(payload) => En1997MutationDsl::ChangePileAlphaS { id: payload.id.clone(), new_alpha_s: payload.new_alpha_s },
        En1997Mutation::ChangePileUnitShaftResistance(payload) => En1997MutationDsl::ChangePileUnitShaftResistance { id: payload.id.clone(), new_unit_shaft_resistance: payload.new_unit_shaft_resistance },
        En1997Mutation::ChangePileUnitBaseResistance(payload) => En1997MutationDsl::ChangePileUnitBaseResistance { id: payload.id.clone(), new_unit_base_resistance: payload.new_unit_base_resistance },
        En1997Mutation::ChangePileCompressionPermanent(payload) => En1997MutationDsl::ChangePileCompressionPermanent { id: payload.id.clone(), new_compression_permanent: payload.new_compression_permanent },
        En1997Mutation::ChangePileCompressionVariable(payload) => En1997MutationDsl::ChangePileCompressionVariable { id: payload.id.clone(), new_compression_variable: payload.new_compression_variable },
        En1997Mutation::ChangePileTensionPermanent(payload) => En1997MutationDsl::ChangePileTensionPermanent { id: payload.id.clone(), new_tension_permanent: payload.new_tension_permanent },
        En1997Mutation::ChangePileTensionVariable(payload) => En1997MutationDsl::ChangePileTensionVariable { id: payload.id.clone(), new_tension_variable: payload.new_tension_variable },
        En1997Mutation::ChangeProfileShaftResistance(payload) => En1997MutationDsl::ChangeProfileShaftResistance { pile_id: payload.pile_id.clone(), id: payload.id.clone(), new_shaft_resistance: payload.new_shaft_resistance },
        En1997Mutation::ChangeProfileBaseResistance(payload) => En1997MutationDsl::ChangeProfileBaseResistance { pile_id: payload.pile_id.clone(), id: payload.id.clone(), new_base_resistance: payload.new_base_resistance },
        En1997Mutation::ChangeWallHeight(payload) => En1997MutationDsl::ChangeWallHeight { id: payload.id.clone(), new_height: payload.new_height },
        En1997Mutation::ChangeWallEmbedment(payload) => En1997MutationDsl::ChangeWallEmbedment { id: payload.id.clone(), new_embedment: payload.new_embedment },
        En1997Mutation::ChangeWallStemThickness(payload) => En1997MutationDsl::ChangeWallStemThickness { id: payload.id.clone(), new_stem_thickness: payload.new_stem_thickness },
        En1997Mutation::ChangeWallBackfillPhiDeg(payload) => En1997MutationDsl::ChangeWallBackfillPhiDeg { id: payload.id.clone(), new_backfill_phi_deg: payload.new_backfill_phi_deg },
        En1997Mutation::ChangeWallBackfillGamma(payload) => En1997MutationDsl::ChangeWallBackfillGamma { id: payload.id.clone(), new_backfill_gamma: payload.new_backfill_gamma },
        En1997Mutation::ChangeWallWallFrictionDeg(payload) => En1997MutationDsl::ChangeWallWallFrictionDeg { id: payload.id.clone(), new_wall_friction_deg: payload.new_wall_friction_deg },
        En1997Mutation::ChangeWallEarthPressureMode(payload) => En1997MutationDsl::ChangeWallEarthPressureMode { id: payload.id.clone(), new_earth_pressure_mode: payload.new_earth_pressure_mode.clone() },
        En1997Mutation::ChangeWallWallMovement(payload) => En1997MutationDsl::ChangeWallWallMovement { id: payload.id.clone(), new_wall_movement: payload.new_wall_movement.clone() },
        En1997Mutation::ChangeWallOcr(payload) => En1997MutationDsl::ChangeWallOcr { id: payload.id.clone(), new_ocr: payload.new_ocr },
        En1997Mutation::ChangeWallConcreteGamma(payload) => En1997MutationDsl::ChangeWallConcreteGamma { id: payload.id.clone(), new_concrete_gamma: payload.new_concrete_gamma },
        En1997Mutation::ChangeWallSurcharge(payload) => En1997MutationDsl::ChangeWallSurcharge { id: payload.id.clone(), new_surcharge: payload.new_surcharge },
        En1997Mutation::ChangeWallVerticalPermanent(payload) => En1997MutationDsl::ChangeWallVerticalPermanent { id: payload.id.clone(), new_vertical_permanent: payload.new_vertical_permanent },
        En1997Mutation::ChangeWallHorizontalPermanent(payload) => En1997MutationDsl::ChangeWallHorizontalPermanent { id: payload.id.clone(), new_horizontal_permanent: payload.new_horizontal_permanent },
        En1997Mutation::ChangeSlopeHeight(payload) => En1997MutationDsl::ChangeSlopeHeight { id: payload.id.clone(), new_height: payload.new_height },
        En1997Mutation::ChangeSlopeLength(payload) => En1997MutationDsl::ChangeSlopeLength { id: payload.id.clone(), new_length: payload.new_length },
        En1997Mutation::ChangeSlopeGoverningLayerId(payload) => En1997MutationDsl::ChangeSlopeGoverningLayerId { id: payload.id.clone(), new_governing_layer_id: payload.new_governing_layer_id.clone() },
        En1997Mutation::ChangeSlopeCircleCentreX(payload) => En1997MutationDsl::ChangeSlopeCircleCentreX { id: payload.id.clone(), new_circle_centre_x: payload.new_circle_centre_x },
        En1997Mutation::ChangeSlopeCircleCentreZ(payload) => En1997MutationDsl::ChangeSlopeCircleCentreZ { id: payload.id.clone(), new_circle_centre_z: payload.new_circle_centre_z },
        En1997Mutation::ChangeSlopeCircleRadius(payload) => En1997MutationDsl::ChangeSlopeCircleRadius { id: payload.id.clone(), new_circle_radius: payload.new_circle_radius },
        En1997Mutation::ChangeUpliftPermStabilizing(payload) => En1997MutationDsl::ChangeUpliftPermStabilizing { id: payload.id.clone(), new_permanent_stabilizing: payload.new_permanent_stabilizing },
        En1997Mutation::ChangeUpliftPermDestabilizing(payload) => En1997MutationDsl::ChangeUpliftPermDestabilizing { id: payload.id.clone(), new_permanent_destabilizing: payload.new_permanent_destabilizing },
        En1997Mutation::ChangeUpliftVarDestabilizing(payload) => En1997MutationDsl::ChangeUpliftVarDestabilizing { id: payload.id.clone(), new_variable_destabilizing: payload.new_variable_destabilizing },
        En1997Mutation::ChangeUpliftPorePressure(payload) => En1997MutationDsl::ChangeUpliftPorePressure { id: payload.id.clone(), new_pore_pressure: payload.new_pore_pressure },
        En1997Mutation::ChangeUpliftTotalStress(payload) => En1997MutationDsl::ChangeUpliftTotalStress { id: payload.id.clone(), new_total_stress: payload.new_total_stress },
        En1997Mutation::ChangeAnchorWallId(payload) => En1997MutationDsl::ChangeAnchorWallId { id: payload.id.clone(), new_wall_id: payload.new_wall_id.clone() },
        En1997Mutation::ChangeAnchorTendon(payload) => En1997MutationDsl::ChangeAnchorTendon { id: payload.id.clone(), new_tendon: payload.new_tendon.clone() },
        En1997Mutation::ChangeAnchorHeadDepth(payload) => En1997MutationDsl::ChangeAnchorHeadDepth { id: payload.id.clone(), new_head_depth: payload.new_head_depth },
        En1997Mutation::ChangeAnchorInclinationDeg(payload) => En1997MutationDsl::ChangeAnchorInclinationDeg { id: payload.id.clone(), new_inclination_deg: payload.new_inclination_deg },
        En1997Mutation::ChangeAnchorFreeLength(payload) => En1997MutationDsl::ChangeAnchorFreeLength { id: payload.id.clone(), new_free_length: payload.new_free_length },
        En1997Mutation::ChangeAnchorFixedLength(payload) => En1997MutationDsl::ChangeAnchorFixedLength { id: payload.id.clone(), new_fixed_length: payload.new_fixed_length },
        En1997Mutation::ChangeAnchorFixedDiameter(payload) => En1997MutationDsl::ChangeAnchorFixedDiameter { id: payload.id.clone(), new_fixed_diameter: payload.new_fixed_diameter },
        En1997Mutation::ChangeAnchorBondStress(payload) => En1997MutationDsl::ChangeAnchorBondStress { id: payload.id.clone(), new_bond_stress: payload.new_bond_stress },
        En1997Mutation::ChangeAnchorDesignForce(payload) => En1997MutationDsl::ChangeAnchorDesignForce { id: payload.id.clone(), new_design_force: payload.new_design_force },
        En1997Mutation::ChangeAnchorTendonYieldForce(payload) => En1997MutationDsl::ChangeAnchorTendonYieldForce { id: payload.id.clone(), new_tendon_yield_force: payload.new_tendon_yield_force },
        En1997Mutation::ChangeAnchorMinFreeLength(payload) => En1997MutationDsl::ChangeAnchorMinFreeLength { id: payload.id.clone(), new_min_free_length: payload.new_min_free_length },
        En1997Mutation::ChangeAnchorTestFactor(payload) => En1997MutationDsl::ChangeAnchorTestFactor { id: payload.id.clone(), new_test_factor: payload.new_test_factor },
        En1997Mutation::ChangeAnchorTestPullOutRes(payload) => En1997MutationDsl::ChangeAnchorTestPullOutRes { anchor_id: payload.anchor_id.clone(), id: payload.id.clone(), new_pull_out_resistance: payload.new_pull_out_resistance },
        En1997Mutation::ChangeEmbWallKind(payload) => En1997MutationDsl::ChangeEmbWallKind { id: payload.id.clone(), new_kind: payload.new_kind.clone() },
        En1997Mutation::ChangeEmbWallRetainedHeight(payload) => En1997MutationDsl::ChangeEmbWallRetainedHeight { id: payload.id.clone(), new_retained_height: payload.new_retained_height },
        En1997Mutation::ChangeEmbWallEmbedment(payload) => En1997MutationDsl::ChangeEmbWallEmbedment { id: payload.id.clone(), new_embedment: payload.new_embedment },
        En1997Mutation::ChangeEmbWallPropDepth(payload) => En1997MutationDsl::ChangeEmbWallPropDepth { id: payload.id.clone(), new_prop_depth: payload.new_prop_depth },
        En1997Mutation::ChangeEmbWallThickness(payload) => En1997MutationDsl::ChangeEmbWallThickness { id: payload.id.clone(), new_thickness: payload.new_thickness },
        En1997Mutation::ChangeEmbWallBackfillGamma(payload) => En1997MutationDsl::ChangeEmbWallBackfillGamma { id: payload.id.clone(), new_backfill_gamma: payload.new_backfill_gamma },
        En1997Mutation::ChangeEmbWallPhiDeg(payload) => En1997MutationDsl::ChangeEmbWallPhiDeg { id: payload.id.clone(), new_phi_deg: payload.new_phi_deg },
        En1997Mutation::ChangeEmbWallDeltaActiveDeg(payload) => En1997MutationDsl::ChangeEmbWallDeltaActiveDeg { id: payload.id.clone(), new_delta_active_deg: payload.new_delta_active_deg },
        En1997Mutation::ChangeEmbWallDeltaPassiveDeg(payload) => En1997MutationDsl::ChangeEmbWallDeltaPassiveDeg { id: payload.id.clone(), new_delta_passive_deg: payload.new_delta_passive_deg },
        En1997Mutation::ChangeEmbWallSurcharge(payload) => En1997MutationDsl::ChangeEmbWallSurcharge { id: payload.id.clone(), new_surcharge: payload.new_surcharge },
        En1997Mutation::ChangeEmbWallUndrStrength(payload) => En1997MutationDsl::ChangeEmbWallUndrStrength { id: payload.id.clone(), new_undrained_strength: payload.new_undrained_strength },
        En1997Mutation::ChangePileGroupPileId(payload) => En1997MutationDsl::ChangePileGroupPileId { id: payload.id.clone(), new_pile_id: payload.new_pile_id.clone() },
        En1997Mutation::ChangePileGroupRows(payload) => En1997MutationDsl::ChangePileGroupRows { id: payload.id.clone(), new_rows: payload.new_rows },
        En1997Mutation::ChangePileGroupColumns(payload) => En1997MutationDsl::ChangePileGroupColumns { id: payload.id.clone(), new_columns: payload.new_columns },
        En1997Mutation::ChangePileGroupSpacing(payload) => En1997MutationDsl::ChangePileGroupSpacing { id: payload.id.clone(), new_spacing: payload.new_spacing },
        En1997Mutation::ChangePileGroupCapVertical(payload) => En1997MutationDsl::ChangePileGroupCapVertical { id: payload.id.clone(), new_cap_vertical: payload.new_cap_vertical },
        En1997Mutation::ChangePileGroupCapMomentX(payload) => En1997MutationDsl::ChangePileGroupCapMomentX { id: payload.id.clone(), new_cap_moment_x: payload.new_cap_moment_x },
        En1997Mutation::ChangePileGroupCapMomentY(payload) => En1997MutationDsl::ChangePileGroupCapMomentY { id: payload.id.clone(), new_cap_moment_y: payload.new_cap_moment_y },
        En1997Mutation::ChangePileGroupSingleRes(payload) => En1997MutationDsl::ChangePileGroupSingleRes { id: payload.id.clone(), new_single_resistance: payload.new_single_resistance },
        En1997Mutation::ChangePileGroupShaftCohSum(payload) => En1997MutationDsl::ChangePileGroupShaftCohSum { id: payload.id.clone(), new_shaft_cohesion_sum: payload.new_shaft_cohesion_sum },
        En1997Mutation::ChangePileGroupBaseCohesion(payload) => En1997MutationDsl::ChangePileGroupBaseCohesion { id: payload.id.clone(), new_base_cohesion: payload.new_base_cohesion },
        En1997Mutation::ChangeLabTestLayerId(payload) => En1997MutationDsl::ChangeLabTestLayerId { id: payload.id.clone(), new_layer_id: payload.new_layer_id.clone() },
        En1997Mutation::ChangeLabTestKind(payload) => En1997MutationDsl::ChangeLabTestKind { id: payload.id.clone(), new_kind: payload.new_kind.clone() },
        En1997Mutation::ChangeLabTestVoidRatio(payload) => En1997MutationDsl::ChangeLabTestVoidRatio { id: payload.id.clone(), new_void_ratio: payload.new_void_ratio },
        En1997Mutation::ChangeLabTestPreloadStress(payload) => En1997MutationDsl::ChangeLabTestPreloadStress { id: payload.id.clone(), new_preload_stress: payload.new_preload_stress },
        En1997Mutation::ChangeLabTestDrainageHeight(payload) => En1997MutationDsl::ChangeLabTestDrainageHeight { id: payload.id.clone(), new_drainage_height: payload.new_drainage_height },
        En1997Mutation::ChangeLabTestConsolTime(payload) => En1997MutationDsl::ChangeLabTestConsolTime { id: payload.id.clone(), new_consolidation_time: payload.new_consolidation_time },
        En1997Mutation::ChangeLabPointNormalStress(payload) => En1997MutationDsl::ChangeLabPointNormalStress { lab_test_id: payload.lab_test_id.clone(), id: payload.id.clone(), new_normal_stress: payload.new_normal_stress },
        En1997Mutation::ChangeLabPointShearStress(payload) => En1997MutationDsl::ChangeLabPointShearStress { lab_test_id: payload.lab_test_id.clone(), id: payload.id.clone(), new_shear_stress: payload.new_shear_stress },
        En1997Mutation::ChangeWaterLayerId(payload) => En1997MutationDsl::ChangeWaterLayerId { id: payload.id.clone(), new_layer_id: payload.new_layer_id.clone() },
        En1997Mutation::ChangeWaterPiezometricDepth(payload) => En1997MutationDsl::ChangeWaterPiezometricDepth { id: payload.id.clone(), new_piezometric_depth: payload.new_piezometric_depth },
        En1997Mutation::ChangeWaterPermeability(payload) => En1997MutationDsl::ChangeWaterPermeability { id: payload.id.clone(), new_permeability: payload.new_permeability },
        En1997Mutation::ChangeWaterHeadDifference(payload) => En1997MutationDsl::ChangeWaterHeadDifference { id: payload.id.clone(), new_head_difference: payload.new_head_difference },
        En1997Mutation::ChangeWaterFlowLength(payload) => En1997MutationDsl::ChangeWaterFlowLength { id: payload.id.clone(), new_flow_length: payload.new_flow_length },
        En1997Mutation::ChangeStructureId(payload) => En1997MutationDsl::ChangeStructureId { new_structure_id: payload.new_structure_id.clone() },
        En1997Mutation::InsertLoadCase(payload) => En1997MutationDsl::InsertLoadCase { footing_id: payload.footing_id.clone(), index: payload.index, load_case: payload.load_case.clone() },
        En1997Mutation::RemoveLoadCase(payload) => En1997MutationDsl::RemoveLoadCase { footing_id: payload.footing_id.clone(), index: payload.index },
        En1997Mutation::InsertTestProfile(payload) => En1997MutationDsl::InsertTestProfile { pile_id: payload.pile_id.clone(), index: payload.index, test_profile: payload.test_profile.clone() },
        En1997Mutation::RemoveTestProfile(payload) => En1997MutationDsl::RemoveTestProfile { pile_id: payload.pile_id.clone(), index: payload.index },
        En1997Mutation::InsertAnchorTest(payload) => En1997MutationDsl::InsertAnchorTest { anchor_id: payload.anchor_id.clone(), index: payload.index, anchor_test: payload.anchor_test.clone() },
        En1997Mutation::RemoveAnchorTest(payload) => En1997MutationDsl::RemoveAnchorTest { anchor_id: payload.anchor_id.clone(), index: payload.index },
        En1997Mutation::InsertLabPoint(payload) => En1997MutationDsl::InsertLabPoint { lab_test_id: payload.lab_test_id.clone(), index: payload.index, lab_point: payload.lab_point.clone() },
        En1997Mutation::RemoveLabPoint(payload) => En1997MutationDsl::RemoveLabPoint { lab_test_id: payload.lab_test_id.clone(), index: payload.index },
        En1997Mutation::InsertGroundInvestigationReading(payload) => En1997MutationDsl::InsertGroundInvestigationReading { investigation_id: payload.investigation_id.clone(), index: payload.index, investigation_reading: payload.investigation_reading.clone() },
        En1997Mutation::RemoveGroundInvestigationReading(payload) => En1997MutationDsl::RemoveGroundInvestigationReading { investigation_id: payload.investigation_id.clone(), index: payload.index },
        En1997Mutation::ChangePlacementX(payload) => En1997MutationDsl::ChangePlacementX { id: payload.id.clone(), new_x: payload.new_x },
        En1997Mutation::ChangePlacementY(payload) => En1997MutationDsl::ChangePlacementY { id: payload.id.clone(), new_y: payload.new_y },
        En1997Mutation::ChangePlacementZ(payload) => En1997MutationDsl::ChangePlacementZ { id: payload.id.clone(), new_z: payload.new_z },
        En1997Mutation::ChangePlacementYaw(payload) => En1997MutationDsl::ChangePlacementYaw { id: payload.id.clone(), new_yaw: payload.new_yaw },
        En1997Mutation::InsertPlacement(payload) => En1997MutationDsl::InsertPlacement { index: payload.index, item: payload.item.clone() },
        En1997Mutation::RemovePlacement(payload) => En1997MutationDsl::RemovePlacement { index: payload.index },
        En1997Mutation::InsertCase(payload) => En1997MutationDsl::InsertCase { index: payload.index, case: payload.case.clone() },
        En1997Mutation::RemoveCase(payload) => En1997MutationDsl::RemoveCase { id: payload.id.clone() },
        En1997Mutation::ChangeCaseNumber(payload) => En1997MutationDsl::ChangeCaseNumber { case_id: payload.case_id.clone(), key: payload.key.clone(), value: payload.value },
        En1997Mutation::ChangeCaseText(payload) => En1997MutationDsl::ChangeCaseText { case_id: payload.case_id.clone(), key: payload.key.clone(), value: payload.value.clone() },
        En1997Mutation::ChangeCaseLabelEn(payload) => En1997MutationDsl::ChangeCaseLabelEn { case_id: payload.case_id.clone(), new_value: payload.new_value.clone() },
        En1997Mutation::ChangeCaseLabelDe(payload) => En1997MutationDsl::ChangeCaseLabelDe { case_id: payload.case_id.clone(), new_value: payload.new_value.clone() },
        En1997Mutation::ChangeCaseEntryId(payload) => En1997MutationDsl::ChangeCaseEntryId { case_id: payload.case_id.clone(), new_value: payload.new_value.clone() },
        En1997Mutation::ChangeSlopeWidth(payload) => En1997MutationDsl::ChangeSlopeWidth { id: payload.id.clone(), new_width: payload.new_width },
        En1997Mutation::ChangePileBendingStiffnessEi(payload) => En1997MutationDsl::ChangePileBendingStiffnessEi { id: payload.id.clone(), new_bending_stiffness_ei: payload.new_bending_stiffness_ei },
        En1997Mutation::ChangePileHorizBedModulus(payload) => En1997MutationDsl::ChangePileHorizBedModulus { id: payload.id.clone(), new_horizontal_subgrade_modulus: payload.new_horizontal_subgrade_modulus },
        En1997Mutation::ChangePileLateralHeadLoad(payload) => En1997MutationDsl::ChangePileLateralHeadLoad { id: payload.id.clone(), new_lateral_head_load: payload.new_lateral_head_load },
        En1997Mutation::ChangeWallSeismicKh(payload) => En1997MutationDsl::ChangeWallSeismicKh { id: payload.id.clone(), new_seismic_kh: payload.new_seismic_kh },
        En1997Mutation::ChangeWallSeismicKv(payload) => En1997MutationDsl::ChangeWallSeismicKv { id: payload.id.clone(), new_seismic_kv: payload.new_seismic_kv },
        En1997Mutation::ChangePileElasticModulus(payload) => En1997MutationDsl::ChangePileElasticModulus { id: payload.id.clone(), new_elastic_modulus: payload.new_elastic_modulus },
        En1997Mutation::ChangePileSettlingThickness(payload) => En1997MutationDsl::ChangePileSettlingThickness { id: payload.id.clone(), new_settling_thickness: payload.new_settling_thickness },
        En1997Mutation::ChangePileNegFrictionBeta(payload) => En1997MutationDsl::ChangePileNegFrictionBeta { id: payload.id.clone(), new_negative_friction_beta: payload.new_negative_friction_beta },
        En1997Mutation::ChangePileDynamicTestCount(payload) => En1997MutationDsl::ChangePileDynamicTestCount { id: payload.id.clone(), new_dynamic_test_count: payload.new_dynamic_test_count },
        En1997Mutation::ChangePileDynamicResMean(payload) => En1997MutationDsl::ChangePileDynamicResMean { id: payload.id.clone(), new_dynamic_resistance_mean: payload.new_dynamic_resistance_mean },
        En1997Mutation::ChangePileDynamicResMin(payload) => En1997MutationDsl::ChangePileDynamicResMin { id: payload.id.clone(), new_dynamic_resistance_min: payload.new_dynamic_resistance_min },
        En1997Mutation::ChangePileHammerWeight(payload) => En1997MutationDsl::ChangePileHammerWeight { id: payload.id.clone(), new_hammer_weight: payload.new_hammer_weight },
        En1997Mutation::ChangePileDropHeight(payload) => En1997MutationDsl::ChangePileDropHeight { id: payload.id.clone(), new_drop_height: payload.new_drop_height },
        En1997Mutation::ChangePileSetPerBlow(payload) => En1997MutationDsl::ChangePileSetPerBlow { id: payload.id.clone(), new_set_per_blow: payload.new_set_per_blow },
        En1997Mutation::ChangePileElasticCompression(payload) => En1997MutationDsl::ChangePileElasticCompression { id: payload.id.clone(), new_elastic_compression: payload.new_elastic_compression },
        En1997Mutation::ChangePileHammerEfficiency(payload) => En1997MutationDsl::ChangePileHammerEfficiency { id: payload.id.clone(), new_hammer_efficiency: payload.new_hammer_efficiency },
        En1997Mutation::ChangeWallReinforcementSpacing(payload) => En1997MutationDsl::ChangeWallReinforcementSpacing { id: payload.id.clone(), new_reinforcement_spacing: payload.new_reinforcement_spacing },
        En1997Mutation::ChangeWallReinforcementWidth(payload) => En1997MutationDsl::ChangeWallReinforcementWidth { id: payload.id.clone(), new_reinforcement_width: payload.new_reinforcement_width },
        En1997Mutation::ChangeWallRebarFriction(payload) => En1997MutationDsl::ChangeWallRebarFriction { id: payload.id.clone(), new_reinforcement_friction: payload.new_reinforcement_friction },
        En1997Mutation::ChangeLabTestCompressionIndex(payload) => En1997MutationDsl::ChangeLabTestCompressionIndex { id: payload.id.clone(), new_compression_index: payload.new_compression_index },
        En1997Mutation::ChangeLabTestSwellingIndex(payload) => En1997MutationDsl::ChangeLabTestSwellingIndex { id: payload.id.clone(), new_swelling_index: payload.new_swelling_index },
        En1997Mutation::ChangeLabTestStressIncrement(payload) => En1997MutationDsl::ChangeLabTestStressIncrement { id: payload.id.clone(), new_stress_increment: payload.new_stress_increment },
        En1997Mutation::ChangeLabTestSecComprIndex(payload) => En1997MutationDsl::ChangeLabTestSecComprIndex { id: payload.id.clone(), new_secondary_compression_index: payload.new_secondary_compression_index },
        En1997Mutation::ChangeLabTestDesignLife(payload) => En1997MutationDsl::ChangeLabTestDesignLife { id: payload.id.clone(), new_design_life: payload.new_design_life },
        En1997Mutation::ChangeWaterWellRadius(payload) => En1997MutationDsl::ChangeWaterWellRadius { id: payload.id.clone(), new_well_radius: payload.new_well_radius },
        En1997Mutation::ChangeWaterWellWaterDepth(payload) => En1997MutationDsl::ChangeWaterWellWaterDepth { id: payload.id.clone(), new_well_water_depth: payload.new_well_water_depth },
        En1997Mutation::ChangeWaterInfluenceRadius(payload) => En1997MutationDsl::ChangeWaterInfluenceRadius { id: payload.id.clone(), new_influence_radius: payload.new_influence_radius },
        En1997Mutation::ChangeWaterWellCapacity(payload) => En1997MutationDsl::ChangeWaterWellCapacity { id: payload.id.clone(), new_well_capacity: payload.new_well_capacity },
        En1997Mutation::ChangePileConcreteClass(payload) => En1997MutationDsl::ChangePileConcreteClass { id: payload.id.clone(), new_concrete_class: payload.new_concrete_class },
        En1997Mutation::ChangeGroundInvestigationPositionX(payload) => En1997MutationDsl::ChangeGroundInvestigationPositionX { id: payload.id.clone(), new_position_x: payload.new_position_x },
        En1997Mutation::ChangeGroundInvestigationPositionY(payload) => En1997MutationDsl::ChangeGroundInvestigationPositionY { id: payload.id.clone(), new_position_y: payload.new_position_y },
        En1997Mutation::ChangeGroundInvestigationLevel(payload) => En1997MutationDsl::ChangeGroundInvestigationLevel { id: payload.id.clone(), new_ground_level: payload.new_ground_level },
        En1997Mutation::ChangeGroundInvestigationDepth(payload) => En1997MutationDsl::ChangeGroundInvestigationDepth { id: payload.id.clone(), new_depth: payload.new_depth },
        En1997Mutation::ChangeGroundInvestigationKind(payload) => En1997MutationDsl::ChangeGroundInvestigationKind { id: payload.id.clone(), new_kind: payload.new_kind },
        En1997Mutation::ChangeGroundReadingDepth(payload) => En1997MutationDsl::ChangeGroundReadingDepth { investigation_id: payload.investigation_id.clone(), id: payload.id.clone(), new_depth: payload.new_depth },
        En1997Mutation::ChangeGroundReadingMeasurement(payload) => En1997MutationDsl::ChangeGroundReadingMeasurement { investigation_id: payload.investigation_id.clone(), id: payload.id.clone(), new_measurement: payload.new_measurement },
        En1997Mutation::ChangeCharacteristicBasisLayerId(payload) => En1997MutationDsl::ChangeCharacteristicBasisLayerId { id: payload.id.clone(), new_layer_id: payload.new_layer_id.clone() },
        En1997Mutation::ChangeCharacteristicBasisParameter(payload) => En1997MutationDsl::ChangeCharacteristicBasisParameter { id: payload.id.clone(), new_parameter: payload.new_parameter },
        En1997Mutation::ChangeCharacteristicBasisBasis(payload) => En1997MutationDsl::ChangeCharacteristicBasisBasis { id: payload.id.clone(), new_basis: payload.new_basis },
        En1997Mutation::ChangeLayerCptNkt(payload) => En1997MutationDsl::ChangeLayerCptNkt { id: payload.id.clone(), new_cpt_nkt: payload.new_cpt_nkt },
    }
}

pub fn en1997_mutation_from_dsl(mutation: En1997MutationDsl) -> En1997Mutation {
    match mutation {
        En1997MutationDsl::ChangeAnnex { new_annex } => En1997Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex }),
        En1997MutationDsl::ChangeGeotechnicalCategory { new_geotechnical_category } => {
            En1997Mutation::ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory { new_geotechnical_category })
        }
        En1997MutationDsl::ChangeDesignSituation { new_design_situation } => En1997Mutation::ChangeDesignSituation(change_design_situation::ChangeDesignSituation { new_design_situation }),
        En1997MutationDsl::ChangeDesignApproach { new_design_approach } => En1997Mutation::ChangeDesignApproach(change_design_approach::ChangeDesignApproach { new_design_approach }),
        En1997MutationDsl::ChangeGroundwaterLevel { new_groundwater_level } => En1997Mutation::ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel { new_groundwater_level }),
        En1997MutationDsl::ChangeInvestigationDepth { new_investigation_depth } => {
            En1997Mutation::ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth { new_investigation_depth })
        }
        En1997MutationDsl::ChangeFootingWidth { id, new_width } => En1997Mutation::ChangeFootingWidth(change_footing_width::ChangeFootingWidth { id, new_width }),
        En1997MutationDsl::ChangeFootingEmbedment { id, new_embedment } => En1997Mutation::ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment { id, new_embedment }),
        En1997MutationDsl::ChangePileLength { id, new_length } => En1997Mutation::ChangePileLength(change_pile_length::ChangePileLength { id, new_length }),
        En1997MutationDsl::ChangePileCount { id, new_count } => En1997Mutation::ChangePileCount(change_pile_count::ChangePileCount { id, new_count }),
        En1997MutationDsl::ChangeWallBaseWidth { id, new_base_width } => En1997Mutation::ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth { id, new_base_width }),
        En1997MutationDsl::ChangeSlopeAngle { id, new_angle_deg } => En1997Mutation::ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle { id, new_angle_deg }),
        En1997MutationDsl::ChangeLayerPhiPrime { id, new_phi_prime_deg } => En1997Mutation::ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime { id, new_phi_prime_deg }),
        En1997MutationDsl::ChangeLayerOedometricModulus { id, new_oedometric_modulus } => {
            En1997Mutation::ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus { id, new_oedometric_modulus })
        }
        En1997MutationDsl::InsertLayer { index, layer } => En1997Mutation::InsertLayer(insert_layer::InsertLayer { index, layer }),
        En1997MutationDsl::RemoveLayer { index } => En1997Mutation::RemoveLayer(remove_layer::RemoveLayer { index }),
        En1997MutationDsl::InsertFooting { index, footing } => En1997Mutation::InsertFooting(insert_footing::InsertFooting { index, footing }),
        En1997MutationDsl::RemoveFooting { index } => En1997Mutation::RemoveFooting(remove_footing::RemoveFooting { index }),
        En1997MutationDsl::InsertPile { index, pile } => En1997Mutation::InsertPile(insert_pile::InsertPile { index, pile }),
        En1997MutationDsl::RemovePile { index } => En1997Mutation::RemovePile(remove_pile::RemovePile { index }),
        En1997MutationDsl::InsertRetainingWall { index, retaining_wall } => En1997Mutation::InsertRetainingWall(insert_retaining_wall::InsertRetainingWall { index, retaining_wall }),
        En1997MutationDsl::RemoveRetainingWall { index } => En1997Mutation::RemoveRetainingWall(remove_retaining_wall::RemoveRetainingWall { index }),
        En1997MutationDsl::InsertSlope { index, slope } => En1997Mutation::InsertSlope(insert_slope::InsertSlope { index, slope }),
        En1997MutationDsl::RemoveSlope { index } => En1997Mutation::RemoveSlope(remove_slope::RemoveSlope { index }),
        En1997MutationDsl::InsertUpliftCase { index, uplift_case } => En1997Mutation::InsertUpliftCase(insert_uplift_case::InsertUpliftCase { index, uplift_case }),
        En1997MutationDsl::RemoveUpliftCase { index } => En1997Mutation::RemoveUpliftCase(remove_uplift_case::RemoveUpliftCase { index }),
        En1997MutationDsl::InsertAnchor { index, anchor } => En1997Mutation::InsertAnchor(insert_anchor::InsertAnchor { index, anchor }),
        En1997MutationDsl::RemoveAnchor { index } => En1997Mutation::RemoveAnchor(remove_anchor::RemoveAnchor { index }),
        En1997MutationDsl::InsertEmbeddedWall { index, embedded_wall } => En1997Mutation::InsertEmbeddedWall(insert_embedded_wall::InsertEmbeddedWall { index, embedded_wall }),
        En1997MutationDsl::RemoveEmbeddedWall { index } => En1997Mutation::RemoveEmbeddedWall(remove_embedded_wall::RemoveEmbeddedWall { index }),
        En1997MutationDsl::InsertPileGroup { index, pile_group } => En1997Mutation::InsertPileGroup(insert_pile_group::InsertPileGroup { index, pile_group }),
        En1997MutationDsl::RemovePileGroup { index } => En1997Mutation::RemovePileGroup(remove_pile_group::RemovePileGroup { index }),
        En1997MutationDsl::InsertLabTest { index, lab_test } => En1997Mutation::InsertLabTest(insert_lab_test::InsertLabTest { index, lab_test }),
        En1997MutationDsl::RemoveLabTest { index } => En1997Mutation::RemoveLabTest(remove_lab_test::RemoveLabTest { index }),
        En1997MutationDsl::InsertLayerWater { index, layer_water } => En1997Mutation::InsertLayerWater(insert_layer_water::InsertLayerWater { index, layer_water }),
        En1997MutationDsl::RemoveLayerWater { index } => En1997Mutation::RemoveLayerWater(remove_layer_water::RemoveLayerWater { index }),
        En1997MutationDsl::InsertCharacteristicBasis { index, characteristic_basis } => En1997Mutation::InsertCharacteristicBasis(insert_characteristic_basis::InsertCharacteristicBasis { index, characteristic_basis }),
        En1997MutationDsl::RemoveCharacteristicBasis { index } => En1997Mutation::RemoveCharacteristicBasis(remove_characteristic_basis::RemoveCharacteristicBasis { index }),
        En1997MutationDsl::InsertGroundInvestigation { index, investigation } => En1997Mutation::InsertGroundInvestigation(insert_ground_investigation::InsertGroundInvestigation { index, investigation }),
        En1997MutationDsl::RemoveGroundInvestigation { index } => En1997Mutation::RemoveGroundInvestigation(remove_ground_investigation::RemoveGroundInvestigation { index }),
        En1997MutationDsl::ChangeLayerSoilType { id, new_soil_type } => En1997Mutation::ChangeLayerSoilType(change_layer_soil_type::ChangeLayerSoilType { id, new_soil_type }),
        En1997MutationDsl::ChangeLayerDepthTop { id, new_depth_top } => En1997Mutation::ChangeLayerDepthTop(change_layer_depth_top::ChangeLayerDepthTop { id, new_depth_top }),
        En1997MutationDsl::ChangeLayerDepthBottom { id, new_depth_bottom } => En1997Mutation::ChangeLayerDepthBottom(change_layer_depth_bottom::ChangeLayerDepthBottom { id, new_depth_bottom }),
        En1997MutationDsl::ChangeLayerGamma { id, new_gamma } => En1997Mutation::ChangeLayerGamma(change_layer_gamma::ChangeLayerGamma { id, new_gamma }),
        En1997MutationDsl::ChangeLayerGammaPrime { id, new_gamma_prime } => En1997Mutation::ChangeLayerGammaPrime(change_layer_gamma_prime::ChangeLayerGammaPrime { id, new_gamma_prime }),
        En1997MutationDsl::ChangeLayerCohesionEffective { id, new_cohesion_effective } => En1997Mutation::ChangeLayerCohesionEffective(change_layer_cohesion_effective::ChangeLayerCohesionEffective { id, new_cohesion_effective }),
        En1997MutationDsl::ChangeLayerCohesionUndrained { id, new_cohesion_undrained } => En1997Mutation::ChangeLayerCohesionUndrained(change_layer_cohesion_undrained::ChangeLayerCohesionUndrained { id, new_cohesion_undrained }),
        En1997MutationDsl::ChangeLayerPoissonRatio { id, new_poisson_ratio } => En1997Mutation::ChangeLayerPoissonRatio(change_layer_poisson_ratio::ChangeLayerPoissonRatio { id, new_poisson_ratio }),
        En1997MutationDsl::ChangeLayerCptQc { id, new_cpt_qc } => En1997Mutation::ChangeLayerCptQc(change_layer_cpt_qc::ChangeLayerCptQc { id, new_cpt_qc }),
        En1997MutationDsl::ChangeLayerSptN { id, new_spt_n } => En1997Mutation::ChangeLayerSptN(change_layer_spt_n::ChangeLayerSptN { id, new_spt_n }),
        En1997MutationDsl::ChangeFootingLength { id, new_length } => En1997Mutation::ChangeFootingLength(change_footing_length::ChangeFootingLength { id, new_length }),
        En1997MutationDsl::ChangeFootingBaseInclDeg { id, new_base_inclination_deg } => En1997Mutation::ChangeFootingBaseInclDeg(change_footing_base_incl_deg::ChangeFootingBaseInclDeg { id, new_base_inclination_deg }),
        En1997MutationDsl::ChangeFootingSettlementLimit { id, new_settlement_limit } => En1997Mutation::ChangeFootingSettlementLimit(change_footing_settlement_limit::ChangeFootingSettlementLimit { id, new_settlement_limit }),
        En1997MutationDsl::ChangeLoadCaseDesignSituation { footing_id, id, new_design_situation } => En1997Mutation::ChangeLoadCaseDesignSituation(change_load_case_design_situation::ChangeLoadCaseDesignSituation { footing_id, id, new_design_situation }),
        En1997MutationDsl::ChangeLoadCaseVerticalPerm { footing_id, id, new_vertical_permanent } => En1997Mutation::ChangeLoadCaseVerticalPerm(change_load_case_vertical_perm::ChangeLoadCaseVerticalPerm { footing_id, id, new_vertical_permanent }),
        En1997MutationDsl::ChangeLoadCaseVerticalVar { footing_id, id, new_vertical_variable } => En1997Mutation::ChangeLoadCaseVerticalVar(change_load_case_vertical_var::ChangeLoadCaseVerticalVar { footing_id, id, new_vertical_variable }),
        En1997MutationDsl::ChangeLoadCaseHorizPermanent { footing_id, id, new_horizontal_permanent } => En1997Mutation::ChangeLoadCaseHorizPermanent(change_load_case_horiz_permanent::ChangeLoadCaseHorizPermanent { footing_id, id, new_horizontal_permanent }),
        En1997MutationDsl::ChangeLoadCaseHorizVariable { footing_id, id, new_horizontal_variable } => En1997Mutation::ChangeLoadCaseHorizVariable(change_load_case_horiz_variable::ChangeLoadCaseHorizVariable { footing_id, id, new_horizontal_variable }),
        En1997MutationDsl::ChangeLoadCaseMomentPermanent { footing_id, id, new_moment_permanent } => En1997Mutation::ChangeLoadCaseMomentPermanent(change_load_case_moment_permanent::ChangeLoadCaseMomentPermanent { footing_id, id, new_moment_permanent }),
        En1997MutationDsl::ChangeLoadCaseMomentVariable { footing_id, id, new_moment_variable } => En1997Mutation::ChangeLoadCaseMomentVariable(change_load_case_moment_variable::ChangeLoadCaseMomentVariable { footing_id, id, new_moment_variable }),
        En1997MutationDsl::ChangePilePileType { id, new_pile_type } => En1997Mutation::ChangePilePileType(change_pile_pile_type::ChangePilePileType { id, new_pile_type }),
        En1997MutationDsl::ChangePileDiameter { id, new_diameter } => En1997Mutation::ChangePileDiameter(change_pile_diameter::ChangePileDiameter { id, new_diameter }),
        En1997MutationDsl::ChangePileAlphaS { id, new_alpha_s } => En1997Mutation::ChangePileAlphaS(change_pile_alpha_s::ChangePileAlphaS { id, new_alpha_s }),
        En1997MutationDsl::ChangePileUnitShaftResistance { id, new_unit_shaft_resistance } => En1997Mutation::ChangePileUnitShaftResistance(change_pile_unit_shaft_resistance::ChangePileUnitShaftResistance { id, new_unit_shaft_resistance }),
        En1997MutationDsl::ChangePileUnitBaseResistance { id, new_unit_base_resistance } => En1997Mutation::ChangePileUnitBaseResistance(change_pile_unit_base_resistance::ChangePileUnitBaseResistance { id, new_unit_base_resistance }),
        En1997MutationDsl::ChangePileCompressionPermanent { id, new_compression_permanent } => En1997Mutation::ChangePileCompressionPermanent(change_pile_compression_permanent::ChangePileCompressionPermanent { id, new_compression_permanent }),
        En1997MutationDsl::ChangePileCompressionVariable { id, new_compression_variable } => En1997Mutation::ChangePileCompressionVariable(change_pile_compression_variable::ChangePileCompressionVariable { id, new_compression_variable }),
        En1997MutationDsl::ChangePileTensionPermanent { id, new_tension_permanent } => En1997Mutation::ChangePileTensionPermanent(change_pile_tension_permanent::ChangePileTensionPermanent { id, new_tension_permanent }),
        En1997MutationDsl::ChangePileTensionVariable { id, new_tension_variable } => En1997Mutation::ChangePileTensionVariable(change_pile_tension_variable::ChangePileTensionVariable { id, new_tension_variable }),
        En1997MutationDsl::ChangeProfileShaftResistance { pile_id, id, new_shaft_resistance } => En1997Mutation::ChangeProfileShaftResistance(change_profile_shaft_resistance::ChangeProfileShaftResistance { pile_id, id, new_shaft_resistance }),
        En1997MutationDsl::ChangeProfileBaseResistance { pile_id, id, new_base_resistance } => En1997Mutation::ChangeProfileBaseResistance(change_profile_base_resistance::ChangeProfileBaseResistance { pile_id, id, new_base_resistance }),
        En1997MutationDsl::ChangeWallHeight { id, new_height } => En1997Mutation::ChangeWallHeight(change_wall_height::ChangeWallHeight { id, new_height }),
        En1997MutationDsl::ChangeWallEmbedment { id, new_embedment } => En1997Mutation::ChangeWallEmbedment(change_wall_embedment::ChangeWallEmbedment { id, new_embedment }),
        En1997MutationDsl::ChangeWallStemThickness { id, new_stem_thickness } => En1997Mutation::ChangeWallStemThickness(change_wall_stem_thickness::ChangeWallStemThickness { id, new_stem_thickness }),
        En1997MutationDsl::ChangeWallBackfillPhiDeg { id, new_backfill_phi_deg } => En1997Mutation::ChangeWallBackfillPhiDeg(change_wall_backfill_phi_deg::ChangeWallBackfillPhiDeg { id, new_backfill_phi_deg }),
        En1997MutationDsl::ChangeWallBackfillGamma { id, new_backfill_gamma } => En1997Mutation::ChangeWallBackfillGamma(change_wall_backfill_gamma::ChangeWallBackfillGamma { id, new_backfill_gamma }),
        En1997MutationDsl::ChangeWallWallFrictionDeg { id, new_wall_friction_deg } => En1997Mutation::ChangeWallWallFrictionDeg(change_wall_wall_friction_deg::ChangeWallWallFrictionDeg { id, new_wall_friction_deg }),
        En1997MutationDsl::ChangeWallEarthPressureMode { id, new_earth_pressure_mode } => En1997Mutation::ChangeWallEarthPressureMode(change_wall_earth_pressure_mode::ChangeWallEarthPressureMode { id, new_earth_pressure_mode }),
        En1997MutationDsl::ChangeWallWallMovement { id, new_wall_movement } => En1997Mutation::ChangeWallWallMovement(change_wall_wall_movement::ChangeWallWallMovement { id, new_wall_movement }),
        En1997MutationDsl::ChangeWallOcr { id, new_ocr } => En1997Mutation::ChangeWallOcr(change_wall_ocr::ChangeWallOcr { id, new_ocr }),
        En1997MutationDsl::ChangeWallConcreteGamma { id, new_concrete_gamma } => En1997Mutation::ChangeWallConcreteGamma(change_wall_concrete_gamma::ChangeWallConcreteGamma { id, new_concrete_gamma }),
        En1997MutationDsl::ChangeWallSurcharge { id, new_surcharge } => En1997Mutation::ChangeWallSurcharge(change_wall_surcharge::ChangeWallSurcharge { id, new_surcharge }),
        En1997MutationDsl::ChangeWallVerticalPermanent { id, new_vertical_permanent } => En1997Mutation::ChangeWallVerticalPermanent(change_wall_vertical_permanent::ChangeWallVerticalPermanent { id, new_vertical_permanent }),
        En1997MutationDsl::ChangeWallHorizontalPermanent { id, new_horizontal_permanent } => En1997Mutation::ChangeWallHorizontalPermanent(change_wall_horizontal_permanent::ChangeWallHorizontalPermanent { id, new_horizontal_permanent }),
        En1997MutationDsl::ChangeSlopeHeight { id, new_height } => En1997Mutation::ChangeSlopeHeight(change_slope_height::ChangeSlopeHeight { id, new_height }),
        En1997MutationDsl::ChangeSlopeLength { id, new_length } => En1997Mutation::ChangeSlopeLength(change_slope_length::ChangeSlopeLength { id, new_length }),
        En1997MutationDsl::ChangeSlopeGoverningLayerId { id, new_governing_layer_id } => En1997Mutation::ChangeSlopeGoverningLayerId(change_slope_governing_layer_id::ChangeSlopeGoverningLayerId { id, new_governing_layer_id }),
        En1997MutationDsl::ChangeSlopeCircleCentreX { id, new_circle_centre_x } => En1997Mutation::ChangeSlopeCircleCentreX(change_slope_circle_centre_x::ChangeSlopeCircleCentreX { id, new_circle_centre_x }),
        En1997MutationDsl::ChangeSlopeCircleCentreZ { id, new_circle_centre_z } => En1997Mutation::ChangeSlopeCircleCentreZ(change_slope_circle_centre_z::ChangeSlopeCircleCentreZ { id, new_circle_centre_z }),
        En1997MutationDsl::ChangeSlopeCircleRadius { id, new_circle_radius } => En1997Mutation::ChangeSlopeCircleRadius(change_slope_circle_radius::ChangeSlopeCircleRadius { id, new_circle_radius }),
        En1997MutationDsl::ChangeUpliftPermStabilizing { id, new_permanent_stabilizing } => En1997Mutation::ChangeUpliftPermStabilizing(change_uplift_perm_stabilizing::ChangeUpliftPermStabilizing { id, new_permanent_stabilizing }),
        En1997MutationDsl::ChangeUpliftPermDestabilizing { id, new_permanent_destabilizing } => En1997Mutation::ChangeUpliftPermDestabilizing(change_uplift_perm_destabilizing::ChangeUpliftPermDestabilizing { id, new_permanent_destabilizing }),
        En1997MutationDsl::ChangeUpliftVarDestabilizing { id, new_variable_destabilizing } => En1997Mutation::ChangeUpliftVarDestabilizing(change_uplift_var_destabilizing::ChangeUpliftVarDestabilizing { id, new_variable_destabilizing }),
        En1997MutationDsl::ChangeUpliftPorePressure { id, new_pore_pressure } => En1997Mutation::ChangeUpliftPorePressure(change_uplift_pore_pressure::ChangeUpliftPorePressure { id, new_pore_pressure }),
        En1997MutationDsl::ChangeUpliftTotalStress { id, new_total_stress } => En1997Mutation::ChangeUpliftTotalStress(change_uplift_total_stress::ChangeUpliftTotalStress { id, new_total_stress }),
        En1997MutationDsl::ChangeAnchorWallId { id, new_wall_id } => En1997Mutation::ChangeAnchorWallId(change_anchor_wall_id::ChangeAnchorWallId { id, new_wall_id }),
        En1997MutationDsl::ChangeAnchorTendon { id, new_tendon } => En1997Mutation::ChangeAnchorTendon(change_anchor_tendon::ChangeAnchorTendon { id, new_tendon }),
        En1997MutationDsl::ChangeAnchorHeadDepth { id, new_head_depth } => En1997Mutation::ChangeAnchorHeadDepth(change_anchor_head_depth::ChangeAnchorHeadDepth { id, new_head_depth }),
        En1997MutationDsl::ChangeAnchorInclinationDeg { id, new_inclination_deg } => En1997Mutation::ChangeAnchorInclinationDeg(change_anchor_inclination_deg::ChangeAnchorInclinationDeg { id, new_inclination_deg }),
        En1997MutationDsl::ChangeAnchorFreeLength { id, new_free_length } => En1997Mutation::ChangeAnchorFreeLength(change_anchor_free_length::ChangeAnchorFreeLength { id, new_free_length }),
        En1997MutationDsl::ChangeAnchorFixedLength { id, new_fixed_length } => En1997Mutation::ChangeAnchorFixedLength(change_anchor_fixed_length::ChangeAnchorFixedLength { id, new_fixed_length }),
        En1997MutationDsl::ChangeAnchorFixedDiameter { id, new_fixed_diameter } => En1997Mutation::ChangeAnchorFixedDiameter(change_anchor_fixed_diameter::ChangeAnchorFixedDiameter { id, new_fixed_diameter }),
        En1997MutationDsl::ChangeAnchorBondStress { id, new_bond_stress } => En1997Mutation::ChangeAnchorBondStress(change_anchor_bond_stress::ChangeAnchorBondStress { id, new_bond_stress }),
        En1997MutationDsl::ChangeAnchorDesignForce { id, new_design_force } => En1997Mutation::ChangeAnchorDesignForce(change_anchor_design_force::ChangeAnchorDesignForce { id, new_design_force }),
        En1997MutationDsl::ChangeAnchorTendonYieldForce { id, new_tendon_yield_force } => En1997Mutation::ChangeAnchorTendonYieldForce(change_anchor_tendon_yield_force::ChangeAnchorTendonYieldForce { id, new_tendon_yield_force }),
        En1997MutationDsl::ChangeAnchorMinFreeLength { id, new_min_free_length } => En1997Mutation::ChangeAnchorMinFreeLength(change_anchor_min_free_length::ChangeAnchorMinFreeLength { id, new_min_free_length }),
        En1997MutationDsl::ChangeAnchorTestFactor { id, new_test_factor } => En1997Mutation::ChangeAnchorTestFactor(change_anchor_test_factor::ChangeAnchorTestFactor { id, new_test_factor }),
        En1997MutationDsl::ChangeAnchorTestPullOutRes { anchor_id, id, new_pull_out_resistance } => En1997Mutation::ChangeAnchorTestPullOutRes(change_anchor_test_pull_out_res::ChangeAnchorTestPullOutRes { anchor_id, id, new_pull_out_resistance }),
        En1997MutationDsl::ChangeEmbWallKind { id, new_kind } => En1997Mutation::ChangeEmbWallKind(change_emb_wall_kind::ChangeEmbWallKind { id, new_kind }),
        En1997MutationDsl::ChangeEmbWallRetainedHeight { id, new_retained_height } => En1997Mutation::ChangeEmbWallRetainedHeight(change_emb_wall_retained_height::ChangeEmbWallRetainedHeight { id, new_retained_height }),
        En1997MutationDsl::ChangeEmbWallEmbedment { id, new_embedment } => En1997Mutation::ChangeEmbWallEmbedment(change_emb_wall_embedment::ChangeEmbWallEmbedment { id, new_embedment }),
        En1997MutationDsl::ChangeEmbWallPropDepth { id, new_prop_depth } => En1997Mutation::ChangeEmbWallPropDepth(change_emb_wall_prop_depth::ChangeEmbWallPropDepth { id, new_prop_depth }),
        En1997MutationDsl::ChangeEmbWallThickness { id, new_thickness } => En1997Mutation::ChangeEmbWallThickness(change_emb_wall_thickness::ChangeEmbWallThickness { id, new_thickness }),
        En1997MutationDsl::ChangeEmbWallBackfillGamma { id, new_backfill_gamma } => En1997Mutation::ChangeEmbWallBackfillGamma(change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma { id, new_backfill_gamma }),
        En1997MutationDsl::ChangeEmbWallPhiDeg { id, new_phi_deg } => En1997Mutation::ChangeEmbWallPhiDeg(change_emb_wall_phi_deg::ChangeEmbWallPhiDeg { id, new_phi_deg }),
        En1997MutationDsl::ChangeEmbWallDeltaActiveDeg { id, new_delta_active_deg } => En1997Mutation::ChangeEmbWallDeltaActiveDeg(change_emb_wall_delta_active_deg::ChangeEmbWallDeltaActiveDeg { id, new_delta_active_deg }),
        En1997MutationDsl::ChangeEmbWallDeltaPassiveDeg { id, new_delta_passive_deg } => En1997Mutation::ChangeEmbWallDeltaPassiveDeg(change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg { id, new_delta_passive_deg }),
        En1997MutationDsl::ChangeEmbWallSurcharge { id, new_surcharge } => En1997Mutation::ChangeEmbWallSurcharge(change_emb_wall_surcharge::ChangeEmbWallSurcharge { id, new_surcharge }),
        En1997MutationDsl::ChangeEmbWallUndrStrength { id, new_undrained_strength } => En1997Mutation::ChangeEmbWallUndrStrength(change_emb_wall_undr_strength::ChangeEmbWallUndrStrength { id, new_undrained_strength }),
        En1997MutationDsl::ChangePileGroupPileId { id, new_pile_id } => En1997Mutation::ChangePileGroupPileId(change_pile_group_pile_id::ChangePileGroupPileId { id, new_pile_id }),
        En1997MutationDsl::ChangePileGroupRows { id, new_rows } => En1997Mutation::ChangePileGroupRows(change_pile_group_rows::ChangePileGroupRows { id, new_rows }),
        En1997MutationDsl::ChangePileGroupColumns { id, new_columns } => En1997Mutation::ChangePileGroupColumns(change_pile_group_columns::ChangePileGroupColumns { id, new_columns }),
        En1997MutationDsl::ChangePileGroupSpacing { id, new_spacing } => En1997Mutation::ChangePileGroupSpacing(change_pile_group_spacing::ChangePileGroupSpacing { id, new_spacing }),
        En1997MutationDsl::ChangePileGroupCapVertical { id, new_cap_vertical } => En1997Mutation::ChangePileGroupCapVertical(change_pile_group_cap_vertical::ChangePileGroupCapVertical { id, new_cap_vertical }),
        En1997MutationDsl::ChangePileGroupCapMomentX { id, new_cap_moment_x } => En1997Mutation::ChangePileGroupCapMomentX(change_pile_group_cap_moment_x::ChangePileGroupCapMomentX { id, new_cap_moment_x }),
        En1997MutationDsl::ChangePileGroupCapMomentY { id, new_cap_moment_y } => En1997Mutation::ChangePileGroupCapMomentY(change_pile_group_cap_moment_y::ChangePileGroupCapMomentY { id, new_cap_moment_y }),
        En1997MutationDsl::ChangePileGroupSingleRes { id, new_single_resistance } => En1997Mutation::ChangePileGroupSingleRes(change_pile_group_single_res::ChangePileGroupSingleRes { id, new_single_resistance }),
        En1997MutationDsl::ChangePileGroupShaftCohSum { id, new_shaft_cohesion_sum } => En1997Mutation::ChangePileGroupShaftCohSum(change_pile_group_shaft_coh_sum::ChangePileGroupShaftCohSum { id, new_shaft_cohesion_sum }),
        En1997MutationDsl::ChangePileGroupBaseCohesion { id, new_base_cohesion } => En1997Mutation::ChangePileGroupBaseCohesion(change_pile_group_base_cohesion::ChangePileGroupBaseCohesion { id, new_base_cohesion }),
        En1997MutationDsl::ChangeLabTestLayerId { id, new_layer_id } => En1997Mutation::ChangeLabTestLayerId(change_lab_test_layer_id::ChangeLabTestLayerId { id, new_layer_id }),
        En1997MutationDsl::ChangeLabTestKind { id, new_kind } => En1997Mutation::ChangeLabTestKind(change_lab_test_kind::ChangeLabTestKind { id, new_kind }),
        En1997MutationDsl::ChangeLabTestVoidRatio { id, new_void_ratio } => En1997Mutation::ChangeLabTestVoidRatio(change_lab_test_void_ratio::ChangeLabTestVoidRatio { id, new_void_ratio }),
        En1997MutationDsl::ChangeLabTestPreloadStress { id, new_preload_stress } => En1997Mutation::ChangeLabTestPreloadStress(change_lab_test_preload_stress::ChangeLabTestPreloadStress { id, new_preload_stress }),
        En1997MutationDsl::ChangeLabTestDrainageHeight { id, new_drainage_height } => En1997Mutation::ChangeLabTestDrainageHeight(change_lab_test_drainage_height::ChangeLabTestDrainageHeight { id, new_drainage_height }),
        En1997MutationDsl::ChangeLabTestConsolTime { id, new_consolidation_time } => En1997Mutation::ChangeLabTestConsolTime(change_lab_test_consol_time::ChangeLabTestConsolTime { id, new_consolidation_time }),
        En1997MutationDsl::ChangeLabPointNormalStress { lab_test_id, id, new_normal_stress } => En1997Mutation::ChangeLabPointNormalStress(change_lab_point_normal_stress::ChangeLabPointNormalStress { lab_test_id, id, new_normal_stress }),
        En1997MutationDsl::ChangeLabPointShearStress { lab_test_id, id, new_shear_stress } => En1997Mutation::ChangeLabPointShearStress(change_lab_point_shear_stress::ChangeLabPointShearStress { lab_test_id, id, new_shear_stress }),
        En1997MutationDsl::ChangeWaterLayerId { id, new_layer_id } => En1997Mutation::ChangeWaterLayerId(change_water_layer_id::ChangeWaterLayerId { id, new_layer_id }),
        En1997MutationDsl::ChangeWaterPiezometricDepth { id, new_piezometric_depth } => En1997Mutation::ChangeWaterPiezometricDepth(change_water_piezometric_depth::ChangeWaterPiezometricDepth { id, new_piezometric_depth }),
        En1997MutationDsl::ChangeWaterPermeability { id, new_permeability } => En1997Mutation::ChangeWaterPermeability(change_water_permeability::ChangeWaterPermeability { id, new_permeability }),
        En1997MutationDsl::ChangeWaterHeadDifference { id, new_head_difference } => En1997Mutation::ChangeWaterHeadDifference(change_water_head_difference::ChangeWaterHeadDifference { id, new_head_difference }),
        En1997MutationDsl::ChangeWaterFlowLength { id, new_flow_length } => En1997Mutation::ChangeWaterFlowLength(change_water_flow_length::ChangeWaterFlowLength { id, new_flow_length }),
        En1997MutationDsl::ChangeStructureId { new_structure_id } => En1997Mutation::ChangeStructureId(change_structure_id::ChangeStructureId { new_structure_id }),
        En1997MutationDsl::InsertLoadCase { footing_id, index, load_case } => En1997Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { footing_id, index, load_case }),
        En1997MutationDsl::RemoveLoadCase { footing_id, index } => En1997Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { footing_id, index }),
        En1997MutationDsl::InsertTestProfile { pile_id, index, test_profile } => En1997Mutation::InsertTestProfile(insert_test_profile::InsertTestProfile { pile_id, index, test_profile }),
        En1997MutationDsl::RemoveTestProfile { pile_id, index } => En1997Mutation::RemoveTestProfile(remove_test_profile::RemoveTestProfile { pile_id, index }),
        En1997MutationDsl::InsertAnchorTest { anchor_id, index, anchor_test } => En1997Mutation::InsertAnchorTest(insert_anchor_test::InsertAnchorTest { anchor_id, index, anchor_test }),
        En1997MutationDsl::RemoveAnchorTest { anchor_id, index } => En1997Mutation::RemoveAnchorTest(remove_anchor_test::RemoveAnchorTest { anchor_id, index }),
        En1997MutationDsl::InsertLabPoint { lab_test_id, index, lab_point } => En1997Mutation::InsertLabPoint(insert_lab_point::InsertLabPoint { lab_test_id, index, lab_point }),
        En1997MutationDsl::RemoveLabPoint { lab_test_id, index } => En1997Mutation::RemoveLabPoint(remove_lab_point::RemoveLabPoint { lab_test_id, index }),
        En1997MutationDsl::InsertGroundInvestigationReading { investigation_id, index, investigation_reading } => En1997Mutation::InsertGroundInvestigationReading(insert_ground_investigation_reading::InsertGroundInvestigationReading { investigation_id, index, investigation_reading }),
        En1997MutationDsl::RemoveGroundInvestigationReading { investigation_id, index } => En1997Mutation::RemoveGroundInvestigationReading(remove_ground_investigation_reading::RemoveGroundInvestigationReading { investigation_id, index }),
        En1997MutationDsl::ChangePlacementX { id, new_x } => En1997Mutation::ChangePlacementX(change_placement_x::ChangePlacementX { id, new_x }),
        En1997MutationDsl::ChangePlacementY { id, new_y } => En1997Mutation::ChangePlacementY(change_placement_y::ChangePlacementY { id, new_y }),
        En1997MutationDsl::ChangePlacementZ { id, new_z } => En1997Mutation::ChangePlacementZ(change_placement_z::ChangePlacementZ { id, new_z }),
        En1997MutationDsl::ChangePlacementYaw { id, new_yaw } => En1997Mutation::ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw { id, new_yaw }),
        En1997MutationDsl::InsertPlacement { index, item } => En1997Mutation::InsertPlacement(insert_placement::InsertPlacement { index, item }),
        En1997MutationDsl::RemovePlacement { index } => En1997Mutation::RemovePlacement(remove_placement::RemovePlacement { index }),
        En1997MutationDsl::InsertCase { index, case } => En1997Mutation::InsertCase(insert_case::InsertCase { index, case }),
        En1997MutationDsl::RemoveCase { id } => En1997Mutation::RemoveCase(remove_case::RemoveCase { id }),
        En1997MutationDsl::ChangeCaseNumber { case_id, key, value } => En1997Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id, key, value }),
        En1997MutationDsl::ChangeCaseText { case_id, key, value } => En1997Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id, key, value }),
        En1997MutationDsl::ChangeCaseLabelEn { case_id, new_value } => En1997Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id, new_value }),
        En1997MutationDsl::ChangeCaseLabelDe { case_id, new_value } => En1997Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id, new_value }),
        En1997MutationDsl::ChangeCaseEntryId { case_id, new_value } => En1997Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id, new_value }),
        En1997MutationDsl::ChangeSlopeWidth { id, new_width } => En1997Mutation::ChangeSlopeWidth(change_slope_width::ChangeSlopeWidth { id, new_width }),
        En1997MutationDsl::ChangePileBendingStiffnessEi { id, new_bending_stiffness_ei } => En1997Mutation::ChangePileBendingStiffnessEi(change_pile_bending_stiffness_ei::ChangePileBendingStiffnessEi { id, new_bending_stiffness_ei }),
        En1997MutationDsl::ChangePileHorizBedModulus { id, new_horizontal_subgrade_modulus } => En1997Mutation::ChangePileHorizBedModulus(change_pile_horiz_bed_modulus::ChangePileHorizBedModulus { id, new_horizontal_subgrade_modulus }),
        En1997MutationDsl::ChangePileLateralHeadLoad { id, new_lateral_head_load } => En1997Mutation::ChangePileLateralHeadLoad(change_pile_lateral_head_load::ChangePileLateralHeadLoad { id, new_lateral_head_load }),
        En1997MutationDsl::ChangeWallSeismicKh { id, new_seismic_kh } => En1997Mutation::ChangeWallSeismicKh(change_wall_seismic_kh::ChangeWallSeismicKh { id, new_seismic_kh }),
        En1997MutationDsl::ChangeWallSeismicKv { id, new_seismic_kv } => En1997Mutation::ChangeWallSeismicKv(change_wall_seismic_kv::ChangeWallSeismicKv { id, new_seismic_kv }),
        En1997MutationDsl::ChangePileElasticModulus { id, new_elastic_modulus } => En1997Mutation::ChangePileElasticModulus(change_pile_elastic_modulus::ChangePileElasticModulus { id, new_elastic_modulus }),
        En1997MutationDsl::ChangePileSettlingThickness { id, new_settling_thickness } => En1997Mutation::ChangePileSettlingThickness(change_pile_settling_thickness::ChangePileSettlingThickness { id, new_settling_thickness }),
        En1997MutationDsl::ChangePileNegFrictionBeta { id, new_negative_friction_beta } => En1997Mutation::ChangePileNegFrictionBeta(change_pile_neg_friction_beta::ChangePileNegFrictionBeta { id, new_negative_friction_beta }),
        En1997MutationDsl::ChangePileDynamicTestCount { id, new_dynamic_test_count } => En1997Mutation::ChangePileDynamicTestCount(change_pile_dynamic_test_count::ChangePileDynamicTestCount { id, new_dynamic_test_count }),
        En1997MutationDsl::ChangePileDynamicResMean { id, new_dynamic_resistance_mean } => En1997Mutation::ChangePileDynamicResMean(change_pile_dynamic_res_mean::ChangePileDynamicResMean { id, new_dynamic_resistance_mean }),
        En1997MutationDsl::ChangePileDynamicResMin { id, new_dynamic_resistance_min } => En1997Mutation::ChangePileDynamicResMin(change_pile_dynamic_res_min::ChangePileDynamicResMin { id, new_dynamic_resistance_min }),
        En1997MutationDsl::ChangePileHammerWeight { id, new_hammer_weight } => En1997Mutation::ChangePileHammerWeight(change_pile_hammer_weight::ChangePileHammerWeight { id, new_hammer_weight }),
        En1997MutationDsl::ChangePileDropHeight { id, new_drop_height } => En1997Mutation::ChangePileDropHeight(change_pile_drop_height::ChangePileDropHeight { id, new_drop_height }),
        En1997MutationDsl::ChangePileSetPerBlow { id, new_set_per_blow } => En1997Mutation::ChangePileSetPerBlow(change_pile_set_per_blow::ChangePileSetPerBlow { id, new_set_per_blow }),
        En1997MutationDsl::ChangePileElasticCompression { id, new_elastic_compression } => En1997Mutation::ChangePileElasticCompression(change_pile_elastic_compression::ChangePileElasticCompression { id, new_elastic_compression }),
        En1997MutationDsl::ChangePileHammerEfficiency { id, new_hammer_efficiency } => En1997Mutation::ChangePileHammerEfficiency(change_pile_hammer_efficiency::ChangePileHammerEfficiency { id, new_hammer_efficiency }),
        En1997MutationDsl::ChangeWallReinforcementSpacing { id, new_reinforcement_spacing } => En1997Mutation::ChangeWallReinforcementSpacing(change_wall_reinforcement_spacing::ChangeWallReinforcementSpacing { id, new_reinforcement_spacing }),
        En1997MutationDsl::ChangeWallReinforcementWidth { id, new_reinforcement_width } => En1997Mutation::ChangeWallReinforcementWidth(change_wall_reinforcement_width::ChangeWallReinforcementWidth { id, new_reinforcement_width }),
        En1997MutationDsl::ChangeWallRebarFriction { id, new_reinforcement_friction } => En1997Mutation::ChangeWallRebarFriction(change_wall_rebar_friction::ChangeWallRebarFriction { id, new_reinforcement_friction }),
        En1997MutationDsl::ChangeLabTestCompressionIndex { id, new_compression_index } => En1997Mutation::ChangeLabTestCompressionIndex(change_lab_test_compression_index::ChangeLabTestCompressionIndex { id, new_compression_index }),
        En1997MutationDsl::ChangeLabTestSwellingIndex { id, new_swelling_index } => En1997Mutation::ChangeLabTestSwellingIndex(change_lab_test_swelling_index::ChangeLabTestSwellingIndex { id, new_swelling_index }),
        En1997MutationDsl::ChangeLabTestStressIncrement { id, new_stress_increment } => En1997Mutation::ChangeLabTestStressIncrement(change_lab_test_stress_increment::ChangeLabTestStressIncrement { id, new_stress_increment }),
        En1997MutationDsl::ChangeLabTestSecComprIndex { id, new_secondary_compression_index } => En1997Mutation::ChangeLabTestSecComprIndex(change_lab_test_sec_compr_index::ChangeLabTestSecComprIndex { id, new_secondary_compression_index }),
        En1997MutationDsl::ChangeLabTestDesignLife { id, new_design_life } => En1997Mutation::ChangeLabTestDesignLife(change_lab_test_design_life::ChangeLabTestDesignLife { id, new_design_life }),
        En1997MutationDsl::ChangeWaterWellRadius { id, new_well_radius } => En1997Mutation::ChangeWaterWellRadius(change_water_well_radius::ChangeWaterWellRadius { id, new_well_radius }),
        En1997MutationDsl::ChangeWaterWellWaterDepth { id, new_well_water_depth } => En1997Mutation::ChangeWaterWellWaterDepth(change_water_well_water_depth::ChangeWaterWellWaterDepth { id, new_well_water_depth }),
        En1997MutationDsl::ChangeWaterInfluenceRadius { id, new_influence_radius } => En1997Mutation::ChangeWaterInfluenceRadius(change_water_influence_radius::ChangeWaterInfluenceRadius { id, new_influence_radius }),
        En1997MutationDsl::ChangeWaterWellCapacity { id, new_well_capacity } => En1997Mutation::ChangeWaterWellCapacity(change_water_well_capacity::ChangeWaterWellCapacity { id, new_well_capacity }),
        En1997MutationDsl::ChangePileConcreteClass { id, new_concrete_class } => En1997Mutation::ChangePileConcreteClass(change_pile_concrete_class::ChangePileConcreteClass { id, new_concrete_class }),
        En1997MutationDsl::ChangeGroundInvestigationPositionX { id, new_position_x } => En1997Mutation::ChangeGroundInvestigationPositionX(change_ground_investigation_position_x::ChangeGroundInvestigationPositionX { id, new_position_x }),
        En1997MutationDsl::ChangeGroundInvestigationPositionY { id, new_position_y } => En1997Mutation::ChangeGroundInvestigationPositionY(change_ground_investigation_position_y::ChangeGroundInvestigationPositionY { id, new_position_y }),
        En1997MutationDsl::ChangeGroundInvestigationLevel { id, new_ground_level } => En1997Mutation::ChangeGroundInvestigationLevel(change_ground_investigation_level::ChangeGroundInvestigationLevel { id, new_ground_level }),
        En1997MutationDsl::ChangeGroundInvestigationDepth { id, new_depth } => En1997Mutation::ChangeGroundInvestigationDepth(change_ground_investigation_depth::ChangeGroundInvestigationDepth { id, new_depth }),
        En1997MutationDsl::ChangeGroundInvestigationKind { id, new_kind } => En1997Mutation::ChangeGroundInvestigationKind(change_ground_investigation_kind::ChangeGroundInvestigationKind { id, new_kind }),
        En1997MutationDsl::ChangeGroundReadingDepth { investigation_id, id, new_depth } => En1997Mutation::ChangeGroundReadingDepth(change_ground_reading_depth::ChangeGroundReadingDepth { investigation_id, id, new_depth }),
        En1997MutationDsl::ChangeGroundReadingMeasurement { investigation_id, id, new_measurement } => En1997Mutation::ChangeGroundReadingMeasurement(change_ground_reading_measurement::ChangeGroundReadingMeasurement { investigation_id, id, new_measurement }),
        En1997MutationDsl::ChangeCharacteristicBasisLayerId { id, new_layer_id } => En1997Mutation::ChangeCharacteristicBasisLayerId(change_characteristic_basis_layer_id::ChangeCharacteristicBasisLayerId { id, new_layer_id }),
        En1997MutationDsl::ChangeCharacteristicBasisParameter { id, new_parameter } => En1997Mutation::ChangeCharacteristicBasisParameter(change_characteristic_basis_parameter::ChangeCharacteristicBasisParameter { id, new_parameter }),
        En1997MutationDsl::ChangeCharacteristicBasisBasis { id, new_basis } => En1997Mutation::ChangeCharacteristicBasisBasis(change_characteristic_basis_basis::ChangeCharacteristicBasisBasis { id, new_basis }),
        En1997MutationDsl::ChangeLayerCptNkt { id, new_cpt_nkt } => En1997Mutation::ChangeLayerCptNkt(change_layer_cpt_nkt::ChangeLayerCptNkt { id, new_cpt_nkt }),
    }
}

impl OpText for En1997Mutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(en1997_mutation_from_dsl(<En1997MutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <En1997MutationDsl as OpText>::print_op(&en1997_mutation_to_dsl(self))
    }
}

//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::*;
use crate::*;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_artifact_norm_en1997_mut_design::change_annex;
use semio_s_artifact_norm_en1997_mut_design::change_geotechnical_category;
use semio_s_artifact_norm_en1997_mut_design::change_design_situation;
use semio_s_artifact_norm_en1997_mut_design::change_design_approach;
use semio_s_artifact_norm_en1997_mut_design::change_groundwater_level;
use semio_s_artifact_norm_en1997_mut_design::change_investigation_depth;
use semio_s_artifact_norm_en1997_mut_foundations::change_footing_width;
use semio_s_artifact_norm_en1997_mut_foundations::change_footing_embedment;
use semio_s_artifact_norm_en1997_mut_foundations::change_pile_length;
use semio_s_artifact_norm_en1997_mut_foundations::change_pile_count;
use semio_s_artifact_norm_en1997_mut_foundations::change_wall_base_width;
use semio_s_artifact_norm_en1997_mut_ground::change_slope_angle;
use semio_s_artifact_norm_en1997_mut_ground::change_layer_phi_prime;
use semio_s_artifact_norm_en1997_mut_ground::change_layer_oedometric_modulus;
use semio_s_artifact_norm_en1997_mut_ground::insert_layer;
use semio_s_artifact_norm_en1997_mut_ground::remove_layer;
use semio_s_artifact_norm_en1997_mut_foundations::insert_footing;
use semio_s_artifact_norm_en1997_mut_foundations::remove_footing;
use semio_s_artifact_norm_en1997_mut_foundations::insert_pile;
use semio_s_artifact_norm_en1997_mut_foundations::remove_pile;

/// 📥️ Decodes one committed mutation JSON document into [`En1997Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1997_mutation_json(text: &str) -> Result<En1997Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
