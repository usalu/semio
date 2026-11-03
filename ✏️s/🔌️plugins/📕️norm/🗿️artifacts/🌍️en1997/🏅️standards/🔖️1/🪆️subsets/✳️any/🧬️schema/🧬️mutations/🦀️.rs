//! 🧬️ En1997 closed semantic mutation vocabulary for the geotechnical project subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

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
use semio_s_artifact_norm_en1997_mut_rows::insert_retaining_wall;
use semio_s_artifact_norm_en1997_mut_rows::remove_retaining_wall;
use semio_s_artifact_norm_en1997_mut_rows::insert_slope;
use semio_s_artifact_norm_en1997_mut_rows::remove_slope;
use semio_s_artifact_norm_en1997_mut_structures::insert_uplift_case;
use semio_s_artifact_norm_en1997_mut_structures::remove_uplift_case;
use semio_s_artifact_norm_en1997_mut_structures::insert_anchor;
use semio_s_artifact_norm_en1997_mut_structures::remove_anchor;
use semio_s_artifact_norm_en1997_mut_structures::insert_embedded_wall;
use semio_s_artifact_norm_en1997_mut_structures::remove_embedded_wall;
use semio_s_artifact_norm_en1997_mut_investigation::insert_pile_group;
use semio_s_artifact_norm_en1997_mut_investigation::remove_pile_group;
use semio_s_artifact_norm_en1997_mut_investigation::insert_lab_test;
use semio_s_artifact_norm_en1997_mut_investigation::remove_lab_test;
use semio_s_artifact_norm_en1997_mut_investigation::insert_layer_water;
use semio_s_artifact_norm_en1997_mut_investigation::remove_layer_water;
use semio_s_artifact_norm_en1997_mut_investigation::insert_characteristic_basis;
use semio_s_artifact_norm_en1997_mut_investigation::remove_characteristic_basis;
use semio_s_artifact_norm_en1997_mut_investigation::insert_ground_investigation;
use semio_s_artifact_norm_en1997_mut_investigation::remove_ground_investigation;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_soil_type;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_depth_top;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_depth_bottom;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_gamma;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_gamma_prime;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_cohesion_effective;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_cohesion_undrained;
use semio_s_artifact_norm_en1997_mut_layer_investigation::change_layer_poisson_ratio;
use semio_s_artifact_norm_en1997_mut_layer_investigation::change_layer_cpt_qc;
use semio_s_artifact_norm_en1997_mut_layer_investigation::change_layer_spt_n;
use semio_s_artifact_norm_en1997_mut_footing::change_footing_length;
use semio_s_artifact_norm_en1997_mut_footing::change_footing_base_incl_deg;
use semio_s_artifact_norm_en1997_mut_footing::change_footing_settlement_limit;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_design_situation;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_vertical_perm;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_vertical_var;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_horiz_permanent;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_horiz_variable;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_moment_permanent;
use semio_s_artifact_norm_en1997_mut_load_case::change_load_case_moment_variable;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_pile_type;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_diameter;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_alpha_s;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_unit_shaft_resistance;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_unit_base_resistance;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_compression_permanent;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_compression_variable;
use semio_s_artifact_norm_en1997_mut_pile_tension::change_pile_tension_permanent;
use semio_s_artifact_norm_en1997_mut_pile_tension::change_pile_tension_variable;
use semio_s_artifact_norm_en1997_mut_profile::change_profile_shaft_resistance;
use semio_s_artifact_norm_en1997_mut_profile::change_profile_base_resistance;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_height;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_embedment;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_stem_thickness;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_backfill_phi_deg;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_backfill_gamma;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_wall_friction_deg;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_earth_pressure_mode;
use semio_s_artifact_norm_en1997_mut_wall_loads::change_wall_wall_movement;
use semio_s_artifact_norm_en1997_mut_wall_loads::change_wall_ocr;
use semio_s_artifact_norm_en1997_mut_wall_loads::change_wall_concrete_gamma;
use semio_s_artifact_norm_en1997_mut_wall_loads::change_wall_surcharge;
use semio_s_artifact_norm_en1997_mut_wall_loads::change_wall_vertical_permanent;
use semio_s_artifact_norm_en1997_mut_wall_loads::change_wall_horizontal_permanent;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_height;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_length;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_governing_layer_id;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_circle_centre_x;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_circle_centre_z;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_circle_radius;
use semio_s_artifact_norm_en1997_mut_uplift::change_uplift_perm_stabilizing;
use semio_s_artifact_norm_en1997_mut_uplift::change_uplift_perm_destabilizing;
use semio_s_artifact_norm_en1997_mut_uplift::change_uplift_var_destabilizing;
use semio_s_artifact_norm_en1997_mut_uplift::change_uplift_pore_pressure;
use semio_s_artifact_norm_en1997_mut_uplift::change_uplift_total_stress;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_wall_id;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_tendon;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_head_depth;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_inclination_deg;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_free_length;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_fixed_length;
use semio_s_artifact_norm_en1997_mut_anchor::change_anchor_fixed_diameter;
use semio_s_artifact_norm_en1997_mut_anchor_proof::change_anchor_bond_stress;
use semio_s_artifact_norm_en1997_mut_anchor_proof::change_anchor_design_force;
use semio_s_artifact_norm_en1997_mut_anchor_proof::change_anchor_tendon_yield_force;
use semio_s_artifact_norm_en1997_mut_anchor_proof::change_anchor_min_free_length;
use semio_s_artifact_norm_en1997_mut_anchor_proof::change_anchor_test_factor;
use semio_s_artifact_norm_en1997_mut_anchor_test::change_anchor_test_pull_out_res;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_kind;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_retained_height;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_embedment;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_prop_depth;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_thickness;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_backfill_gamma;
use semio_s_artifact_norm_en1997_mut_embedded_wall::change_emb_wall_phi_deg;
use semio_s_artifact_norm_en1997_mut_embedded_wall_pressure::change_emb_wall_delta_active_deg;
use semio_s_artifact_norm_en1997_mut_embedded_wall_pressure::change_emb_wall_delta_passive_deg;
use semio_s_artifact_norm_en1997_mut_embedded_wall_pressure::change_emb_wall_surcharge;
use semio_s_artifact_norm_en1997_mut_embedded_wall_pressure::change_emb_wall_undr_strength;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_pile_id;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_rows;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_columns;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_spacing;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_cap_vertical;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_cap_moment_x;
use semio_s_artifact_norm_en1997_mut_pile_group::change_pile_group_cap_moment_y;
use semio_s_artifact_norm_en1997_mut_pile_group_resistance::change_pile_group_single_res;
use semio_s_artifact_norm_en1997_mut_pile_group_resistance::change_pile_group_shaft_coh_sum;
use semio_s_artifact_norm_en1997_mut_pile_group_resistance::change_pile_group_base_cohesion;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_layer_id;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_kind;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_void_ratio;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_preload_stress;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_drainage_height;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_consol_time;
use semio_s_artifact_norm_en1997_mut_lab_point::change_lab_point_normal_stress;
use semio_s_artifact_norm_en1997_mut_lab_point::change_lab_point_shear_stress;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_layer_id;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_piezometric_depth;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_permeability;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_head_difference;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_flow_length;
use semio_s_artifact_norm_en1997_mut_design::change_structure_id;
use semio_s_artifact_norm_en1997_mut_nested_rows::insert_load_case;
use semio_s_artifact_norm_en1997_mut_nested_rows::remove_load_case;
use semio_s_artifact_norm_en1997_mut_nested_rows::insert_test_profile;
use semio_s_artifact_norm_en1997_mut_nested_rows::remove_test_profile;
use semio_s_artifact_norm_en1997_mut_nested_rows::insert_anchor_test;
use semio_s_artifact_norm_en1997_mut_nested_rows::remove_anchor_test;
use semio_s_artifact_norm_en1997_mut_nested_rows::insert_lab_point;
use semio_s_artifact_norm_en1997_mut_nested_rows::remove_lab_point;
use semio_s_artifact_norm_en1997_mut_nested_rows::insert_ground_investigation_reading;
use semio_s_artifact_norm_en1997_mut_nested_rows::remove_ground_investigation_reading;
use semio_s_artifact_norm_en1997_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1997_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1997_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1997_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1997_mut_placements::insert_placement;
use semio_s_artifact_norm_en1997_mut_placements::remove_placement;
use semio_s_artifact_norm_en1997_mut_design_cases::insert_case;
use semio_s_artifact_norm_en1997_mut_design_cases::remove_case;
use semio_s_artifact_norm_en1997_mut_design_cases::change_case_number;
use semio_s_artifact_norm_en1997_mut_design_cases::change_case_text;
use semio_s_artifact_norm_en1997_mut_design_cases::change_case_label_en;
use semio_s_artifact_norm_en1997_mut_design_cases::change_case_label_de;
use semio_s_artifact_norm_en1997_mut_design_cases::change_case_entry_id;
use semio_s_artifact_norm_en1997_mut_slope::change_slope_width;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_bending_stiffness_ei;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_horiz_bed_modulus;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_lateral_head_load;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_seismic_kh;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_seismic_kv;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_elastic_modulus;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_settling_thickness;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_neg_friction_beta;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_dynamic_test_count;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_dynamic_res_mean;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_dynamic_res_min;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_hammer_weight;
use semio_s_artifact_norm_en1997_mut_pile_tension::change_pile_drop_height;
use semio_s_artifact_norm_en1997_mut_pile_tension::change_pile_set_per_blow;
use semio_s_artifact_norm_en1997_mut_pile_tension::change_pile_elastic_compression;
use semio_s_artifact_norm_en1997_mut_pile_tension::change_pile_hammer_efficiency;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_reinforcement_spacing;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_reinforcement_width;
use semio_s_artifact_norm_en1997_mut_wall::change_wall_rebar_friction;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_compression_index;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_swelling_index;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_stress_increment;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_sec_compr_index;
use semio_s_artifact_norm_en1997_mut_lab_test::change_lab_test_design_life;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_well_radius;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_well_water_depth;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_influence_radius;
use semio_s_artifact_norm_en1997_mut_layer_water::change_water_well_capacity;
use semio_s_artifact_norm_en1997_mut_pile::change_pile_concrete_class;
use semio_s_artifact_norm_en1997_mut_ground_investigation::change_ground_investigation_position_x;
use semio_s_artifact_norm_en1997_mut_ground_investigation::change_ground_investigation_position_y;
use semio_s_artifact_norm_en1997_mut_ground_investigation::change_ground_investigation_level;
use semio_s_artifact_norm_en1997_mut_ground_investigation::change_ground_investigation_depth;
use semio_s_artifact_norm_en1997_mut_ground_investigation::change_ground_investigation_kind;
use semio_s_artifact_norm_en1997_mut_ground_investigation_reading::change_ground_reading_depth;
use semio_s_artifact_norm_en1997_mut_ground_investigation_reading::change_ground_reading_measurement;
use semio_s_artifact_norm_en1997_mut_characteristic_basis::change_characteristic_basis_layer_id;
use semio_s_artifact_norm_en1997_mut_characteristic_basis::change_characteristic_basis_parameter;
use semio_s_artifact_norm_en1997_mut_characteristic_basis::change_characteristic_basis_basis;
use semio_s_artifact_norm_en1997_mut_layer::change_layer_cpt_nkt;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1997Snapshot, diff = En1997Diff, schema = "norm.en1997", reference = En1997Snapshot, reference_diff = En1997Diff)]
pub enum En1997Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeGeotechnicalCategory(change_geotechnical_category::ChangeGeotechnicalCategory),
    ChangeDesignSituation(change_design_situation::ChangeDesignSituation),
    ChangeDesignApproach(change_design_approach::ChangeDesignApproach),
    ChangeGroundwaterLevel(change_groundwater_level::ChangeGroundwaterLevel),
    ChangeInvestigationDepth(change_investigation_depth::ChangeInvestigationDepth),
    ChangeFootingWidth(change_footing_width::ChangeFootingWidth),
    ChangeFootingEmbedment(change_footing_embedment::ChangeFootingEmbedment),
    ChangePileLength(change_pile_length::ChangePileLength),
    ChangePileCount(change_pile_count::ChangePileCount),
    ChangeWallBaseWidth(change_wall_base_width::ChangeWallBaseWidth),
    ChangeSlopeAngle(change_slope_angle::ChangeSlopeAngle),
    ChangeLayerPhiPrime(change_layer_phi_prime::ChangeLayerPhiPrime),
    ChangeLayerOedometricModulus(change_layer_oedometric_modulus::ChangeLayerOedometricModulus),
    InsertLayer(insert_layer::InsertLayer),
    RemoveLayer(remove_layer::RemoveLayer),
    InsertFooting(insert_footing::InsertFooting),
    RemoveFooting(remove_footing::RemoveFooting),
    InsertPile(insert_pile::InsertPile),
    RemovePile(remove_pile::RemovePile),
    InsertRetainingWall(insert_retaining_wall::InsertRetainingWall),
    RemoveRetainingWall(remove_retaining_wall::RemoveRetainingWall),
    InsertSlope(insert_slope::InsertSlope),
    RemoveSlope(remove_slope::RemoveSlope),
    InsertUpliftCase(insert_uplift_case::InsertUpliftCase),
    RemoveUpliftCase(remove_uplift_case::RemoveUpliftCase),
    InsertAnchor(insert_anchor::InsertAnchor),
    RemoveAnchor(remove_anchor::RemoveAnchor),
    InsertEmbeddedWall(insert_embedded_wall::InsertEmbeddedWall),
    RemoveEmbeddedWall(remove_embedded_wall::RemoveEmbeddedWall),
    InsertPileGroup(insert_pile_group::InsertPileGroup),
    RemovePileGroup(remove_pile_group::RemovePileGroup),
    InsertLabTest(insert_lab_test::InsertLabTest),
    RemoveLabTest(remove_lab_test::RemoveLabTest),
    InsertLayerWater(insert_layer_water::InsertLayerWater),
    RemoveLayerWater(remove_layer_water::RemoveLayerWater),
    InsertCharacteristicBasis(insert_characteristic_basis::InsertCharacteristicBasis),
    RemoveCharacteristicBasis(remove_characteristic_basis::RemoveCharacteristicBasis),
    InsertGroundInvestigation(insert_ground_investigation::InsertGroundInvestigation),
    RemoveGroundInvestigation(remove_ground_investigation::RemoveGroundInvestigation),
    ChangeLayerSoilType(change_layer_soil_type::ChangeLayerSoilType),
    ChangeLayerDepthTop(change_layer_depth_top::ChangeLayerDepthTop),
    ChangeLayerDepthBottom(change_layer_depth_bottom::ChangeLayerDepthBottom),
    ChangeLayerGamma(change_layer_gamma::ChangeLayerGamma),
    ChangeLayerGammaPrime(change_layer_gamma_prime::ChangeLayerGammaPrime),
    ChangeLayerCohesionEffective(change_layer_cohesion_effective::ChangeLayerCohesionEffective),
    ChangeLayerCohesionUndrained(change_layer_cohesion_undrained::ChangeLayerCohesionUndrained),
    ChangeLayerPoissonRatio(change_layer_poisson_ratio::ChangeLayerPoissonRatio),
    ChangeLayerCptQc(change_layer_cpt_qc::ChangeLayerCptQc),
    ChangeLayerSptN(change_layer_spt_n::ChangeLayerSptN),
    ChangeFootingLength(change_footing_length::ChangeFootingLength),
    ChangeFootingBaseInclDeg(change_footing_base_incl_deg::ChangeFootingBaseInclDeg),
    ChangeFootingSettlementLimit(change_footing_settlement_limit::ChangeFootingSettlementLimit),
    ChangeLoadCaseDesignSituation(change_load_case_design_situation::ChangeLoadCaseDesignSituation),
    ChangeLoadCaseVerticalPerm(change_load_case_vertical_perm::ChangeLoadCaseVerticalPerm),
    ChangeLoadCaseVerticalVar(change_load_case_vertical_var::ChangeLoadCaseVerticalVar),
    ChangeLoadCaseHorizPermanent(change_load_case_horiz_permanent::ChangeLoadCaseHorizPermanent),
    ChangeLoadCaseHorizVariable(change_load_case_horiz_variable::ChangeLoadCaseHorizVariable),
    ChangeLoadCaseMomentPermanent(change_load_case_moment_permanent::ChangeLoadCaseMomentPermanent),
    ChangeLoadCaseMomentVariable(change_load_case_moment_variable::ChangeLoadCaseMomentVariable),
    ChangePilePileType(change_pile_pile_type::ChangePilePileType),
    ChangePileDiameter(change_pile_diameter::ChangePileDiameter),
    ChangePileAlphaS(change_pile_alpha_s::ChangePileAlphaS),
    ChangePileUnitShaftResistance(change_pile_unit_shaft_resistance::ChangePileUnitShaftResistance),
    ChangePileUnitBaseResistance(change_pile_unit_base_resistance::ChangePileUnitBaseResistance),
    ChangePileCompressionPermanent(change_pile_compression_permanent::ChangePileCompressionPermanent),
    ChangePileCompressionVariable(change_pile_compression_variable::ChangePileCompressionVariable),
    ChangePileTensionPermanent(change_pile_tension_permanent::ChangePileTensionPermanent),
    ChangePileTensionVariable(change_pile_tension_variable::ChangePileTensionVariable),
    ChangeProfileShaftResistance(change_profile_shaft_resistance::ChangeProfileShaftResistance),
    ChangeProfileBaseResistance(change_profile_base_resistance::ChangeProfileBaseResistance),
    ChangeWallHeight(change_wall_height::ChangeWallHeight),
    ChangeWallEmbedment(change_wall_embedment::ChangeWallEmbedment),
    ChangeWallStemThickness(change_wall_stem_thickness::ChangeWallStemThickness),
    ChangeWallBackfillPhiDeg(change_wall_backfill_phi_deg::ChangeWallBackfillPhiDeg),
    ChangeWallBackfillGamma(change_wall_backfill_gamma::ChangeWallBackfillGamma),
    ChangeWallWallFrictionDeg(change_wall_wall_friction_deg::ChangeWallWallFrictionDeg),
    ChangeWallEarthPressureMode(change_wall_earth_pressure_mode::ChangeWallEarthPressureMode),
    ChangeWallWallMovement(change_wall_wall_movement::ChangeWallWallMovement),
    ChangeWallOcr(change_wall_ocr::ChangeWallOcr),
    ChangeWallConcreteGamma(change_wall_concrete_gamma::ChangeWallConcreteGamma),
    ChangeWallSurcharge(change_wall_surcharge::ChangeWallSurcharge),
    ChangeWallVerticalPermanent(change_wall_vertical_permanent::ChangeWallVerticalPermanent),
    ChangeWallHorizontalPermanent(change_wall_horizontal_permanent::ChangeWallHorizontalPermanent),
    ChangeSlopeHeight(change_slope_height::ChangeSlopeHeight),
    ChangeSlopeLength(change_slope_length::ChangeSlopeLength),
    ChangeSlopeGoverningLayerId(change_slope_governing_layer_id::ChangeSlopeGoverningLayerId),
    ChangeSlopeCircleCentreX(change_slope_circle_centre_x::ChangeSlopeCircleCentreX),
    ChangeSlopeCircleCentreZ(change_slope_circle_centre_z::ChangeSlopeCircleCentreZ),
    ChangeSlopeCircleRadius(change_slope_circle_radius::ChangeSlopeCircleRadius),
    ChangeUpliftPermStabilizing(change_uplift_perm_stabilizing::ChangeUpliftPermStabilizing),
    ChangeUpliftPermDestabilizing(change_uplift_perm_destabilizing::ChangeUpliftPermDestabilizing),
    ChangeUpliftVarDestabilizing(change_uplift_var_destabilizing::ChangeUpliftVarDestabilizing),
    ChangeUpliftPorePressure(change_uplift_pore_pressure::ChangeUpliftPorePressure),
    ChangeUpliftTotalStress(change_uplift_total_stress::ChangeUpliftTotalStress),
    ChangeAnchorWallId(change_anchor_wall_id::ChangeAnchorWallId),
    ChangeAnchorTendon(change_anchor_tendon::ChangeAnchorTendon),
    ChangeAnchorHeadDepth(change_anchor_head_depth::ChangeAnchorHeadDepth),
    ChangeAnchorInclinationDeg(change_anchor_inclination_deg::ChangeAnchorInclinationDeg),
    ChangeAnchorFreeLength(change_anchor_free_length::ChangeAnchorFreeLength),
    ChangeAnchorFixedLength(change_anchor_fixed_length::ChangeAnchorFixedLength),
    ChangeAnchorFixedDiameter(change_anchor_fixed_diameter::ChangeAnchorFixedDiameter),
    ChangeAnchorBondStress(change_anchor_bond_stress::ChangeAnchorBondStress),
    ChangeAnchorDesignForce(change_anchor_design_force::ChangeAnchorDesignForce),
    ChangeAnchorTendonYieldForce(change_anchor_tendon_yield_force::ChangeAnchorTendonYieldForce),
    ChangeAnchorMinFreeLength(change_anchor_min_free_length::ChangeAnchorMinFreeLength),
    ChangeAnchorTestFactor(change_anchor_test_factor::ChangeAnchorTestFactor),
    ChangeAnchorTestPullOutRes(change_anchor_test_pull_out_res::ChangeAnchorTestPullOutRes),
    ChangeEmbWallKind(change_emb_wall_kind::ChangeEmbWallKind),
    ChangeEmbWallRetainedHeight(change_emb_wall_retained_height::ChangeEmbWallRetainedHeight),
    ChangeEmbWallEmbedment(change_emb_wall_embedment::ChangeEmbWallEmbedment),
    ChangeEmbWallPropDepth(change_emb_wall_prop_depth::ChangeEmbWallPropDepth),
    ChangeEmbWallThickness(change_emb_wall_thickness::ChangeEmbWallThickness),
    ChangeEmbWallBackfillGamma(change_emb_wall_backfill_gamma::ChangeEmbWallBackfillGamma),
    ChangeEmbWallPhiDeg(change_emb_wall_phi_deg::ChangeEmbWallPhiDeg),
    ChangeEmbWallDeltaActiveDeg(change_emb_wall_delta_active_deg::ChangeEmbWallDeltaActiveDeg),
    ChangeEmbWallDeltaPassiveDeg(change_emb_wall_delta_passive_deg::ChangeEmbWallDeltaPassiveDeg),
    ChangeEmbWallSurcharge(change_emb_wall_surcharge::ChangeEmbWallSurcharge),
    ChangeEmbWallUndrStrength(change_emb_wall_undr_strength::ChangeEmbWallUndrStrength),
    ChangePileGroupPileId(change_pile_group_pile_id::ChangePileGroupPileId),
    ChangePileGroupRows(change_pile_group_rows::ChangePileGroupRows),
    ChangePileGroupColumns(change_pile_group_columns::ChangePileGroupColumns),
    ChangePileGroupSpacing(change_pile_group_spacing::ChangePileGroupSpacing),
    ChangePileGroupCapVertical(change_pile_group_cap_vertical::ChangePileGroupCapVertical),
    ChangePileGroupCapMomentX(change_pile_group_cap_moment_x::ChangePileGroupCapMomentX),
    ChangePileGroupCapMomentY(change_pile_group_cap_moment_y::ChangePileGroupCapMomentY),
    ChangePileGroupSingleRes(change_pile_group_single_res::ChangePileGroupSingleRes),
    ChangePileGroupShaftCohSum(change_pile_group_shaft_coh_sum::ChangePileGroupShaftCohSum),
    ChangePileGroupBaseCohesion(change_pile_group_base_cohesion::ChangePileGroupBaseCohesion),
    ChangeLabTestLayerId(change_lab_test_layer_id::ChangeLabTestLayerId),
    ChangeLabTestKind(change_lab_test_kind::ChangeLabTestKind),
    ChangeLabTestVoidRatio(change_lab_test_void_ratio::ChangeLabTestVoidRatio),
    ChangeLabTestPreloadStress(change_lab_test_preload_stress::ChangeLabTestPreloadStress),
    ChangeLabTestDrainageHeight(change_lab_test_drainage_height::ChangeLabTestDrainageHeight),
    ChangeLabTestConsolTime(change_lab_test_consol_time::ChangeLabTestConsolTime),
    ChangeLabPointNormalStress(change_lab_point_normal_stress::ChangeLabPointNormalStress),
    ChangeLabPointShearStress(change_lab_point_shear_stress::ChangeLabPointShearStress),
    ChangeWaterLayerId(change_water_layer_id::ChangeWaterLayerId),
    ChangeWaterPiezometricDepth(change_water_piezometric_depth::ChangeWaterPiezometricDepth),
    ChangeWaterPermeability(change_water_permeability::ChangeWaterPermeability),
    ChangeWaterHeadDifference(change_water_head_difference::ChangeWaterHeadDifference),
    ChangeWaterFlowLength(change_water_flow_length::ChangeWaterFlowLength),
    ChangeStructureId(change_structure_id::ChangeStructureId),
    InsertLoadCase(insert_load_case::InsertLoadCase),
    RemoveLoadCase(remove_load_case::RemoveLoadCase),
    InsertTestProfile(insert_test_profile::InsertTestProfile),
    RemoveTestProfile(remove_test_profile::RemoveTestProfile),
    InsertAnchorTest(insert_anchor_test::InsertAnchorTest),
    RemoveAnchorTest(remove_anchor_test::RemoveAnchorTest),
    InsertLabPoint(insert_lab_point::InsertLabPoint),
    RemoveLabPoint(remove_lab_point::RemoveLabPoint),
    InsertGroundInvestigationReading(insert_ground_investigation_reading::InsertGroundInvestigationReading),
    RemoveGroundInvestigationReading(remove_ground_investigation_reading::RemoveGroundInvestigationReading),
    ChangePlacementX(change_placement_x::ChangePlacementX),
    ChangePlacementY(change_placement_y::ChangePlacementY),
    ChangePlacementZ(change_placement_z::ChangePlacementZ),
    ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw),
    InsertPlacement(insert_placement::InsertPlacement),
    RemovePlacement(remove_placement::RemovePlacement),
    InsertCase(insert_case::InsertCase),
    RemoveCase(remove_case::RemoveCase),
    ChangeCaseNumber(change_case_number::ChangeCaseNumber),
    ChangeCaseText(change_case_text::ChangeCaseText),
    ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn),
    ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe),
    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),
    ChangeSlopeWidth(change_slope_width::ChangeSlopeWidth),
    ChangePileBendingStiffnessEi(change_pile_bending_stiffness_ei::ChangePileBendingStiffnessEi),
    ChangePileHorizBedModulus(change_pile_horiz_bed_modulus::ChangePileHorizBedModulus),
    ChangePileLateralHeadLoad(change_pile_lateral_head_load::ChangePileLateralHeadLoad),
    ChangeWallSeismicKh(change_wall_seismic_kh::ChangeWallSeismicKh),
    ChangeWallSeismicKv(change_wall_seismic_kv::ChangeWallSeismicKv),

    ChangePileElasticModulus(change_pile_elastic_modulus::ChangePileElasticModulus),
    ChangePileSettlingThickness(change_pile_settling_thickness::ChangePileSettlingThickness),
    ChangePileNegFrictionBeta(change_pile_neg_friction_beta::ChangePileNegFrictionBeta),
    ChangePileDynamicTestCount(change_pile_dynamic_test_count::ChangePileDynamicTestCount),
    ChangePileDynamicResMean(change_pile_dynamic_res_mean::ChangePileDynamicResMean),
    ChangePileDynamicResMin(change_pile_dynamic_res_min::ChangePileDynamicResMin),
    ChangePileHammerWeight(change_pile_hammer_weight::ChangePileHammerWeight),
    ChangePileDropHeight(change_pile_drop_height::ChangePileDropHeight),
    ChangePileSetPerBlow(change_pile_set_per_blow::ChangePileSetPerBlow),
    ChangePileElasticCompression(change_pile_elastic_compression::ChangePileElasticCompression),
    ChangePileHammerEfficiency(change_pile_hammer_efficiency::ChangePileHammerEfficiency),
    ChangeWallReinforcementSpacing(change_wall_reinforcement_spacing::ChangeWallReinforcementSpacing),
    ChangeWallReinforcementWidth(change_wall_reinforcement_width::ChangeWallReinforcementWidth),
    ChangeWallRebarFriction(change_wall_rebar_friction::ChangeWallRebarFriction),
    ChangeLabTestCompressionIndex(change_lab_test_compression_index::ChangeLabTestCompressionIndex),
    ChangeLabTestSwellingIndex(change_lab_test_swelling_index::ChangeLabTestSwellingIndex),
    ChangeLabTestStressIncrement(change_lab_test_stress_increment::ChangeLabTestStressIncrement),
    ChangeLabTestSecComprIndex(change_lab_test_sec_compr_index::ChangeLabTestSecComprIndex),
    ChangeLabTestDesignLife(change_lab_test_design_life::ChangeLabTestDesignLife),
    ChangeWaterWellRadius(change_water_well_radius::ChangeWaterWellRadius),
    ChangeWaterWellWaterDepth(change_water_well_water_depth::ChangeWaterWellWaterDepth),
    ChangeWaterInfluenceRadius(change_water_influence_radius::ChangeWaterInfluenceRadius),
    ChangeWaterWellCapacity(change_water_well_capacity::ChangeWaterWellCapacity),
    ChangePileConcreteClass(change_pile_concrete_class::ChangePileConcreteClass),
    ChangeGroundInvestigationPositionX(change_ground_investigation_position_x::ChangeGroundInvestigationPositionX),
    ChangeGroundInvestigationPositionY(change_ground_investigation_position_y::ChangeGroundInvestigationPositionY),
    ChangeGroundInvestigationLevel(change_ground_investigation_level::ChangeGroundInvestigationLevel),
    ChangeGroundInvestigationDepth(change_ground_investigation_depth::ChangeGroundInvestigationDepth),
    ChangeGroundInvestigationKind(change_ground_investigation_kind::ChangeGroundInvestigationKind),
    ChangeGroundReadingDepth(change_ground_reading_depth::ChangeGroundReadingDepth),
    ChangeGroundReadingMeasurement(change_ground_reading_measurement::ChangeGroundReadingMeasurement),
    ChangeCharacteristicBasisLayerId(change_characteristic_basis_layer_id::ChangeCharacteristicBasisLayerId),
    ChangeCharacteristicBasisParameter(change_characteristic_basis_parameter::ChangeCharacteristicBasisParameter),
    ChangeCharacteristicBasisBasis(change_characteristic_basis_basis::ChangeCharacteristicBasisBasis),
    ChangeLayerCptNkt(change_layer_cpt_nkt::ChangeLayerCptNkt),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-geotechnical-category",
    "change-design-situation",
    "change-design-approach",
    "change-groundwater-level",
    "change-investigation-depth",
    "change-footing-width",
    "change-footing-embedment",
    "change-pile-length",
    "change-pile-count",
    "change-wall-base-width",
    "change-slope-angle",
    "change-layer-phi-prime",
    "change-layer-oedometric-modulus",
    "insert-layer",
    "remove-layer",
    "insert-footing",
    "remove-footing",
    "insert-pile",
    "remove-pile",
    "insert-retaining-wall",
    "remove-retaining-wall",
    "insert-slope",
    "remove-slope",
    "insert-uplift-case",
    "remove-uplift-case",
    "insert-anchor",
    "remove-anchor",
    "insert-embedded-wall",
    "remove-embedded-wall",
    "insert-pile-group",
    "remove-pile-group",
    "insert-lab-test",
    "remove-lab-test",
    "insert-layer-water",
    "remove-layer-water",
    "insert-characteristic-basis",
    "remove-characteristic-basis",
    "insert-ground-investigation",
    "remove-ground-investigation",
    "change-layer-soil-type",
    "change-layer-depth-top",
    "change-layer-depth-bottom",
    "change-layer-gamma",
    "change-layer-gamma-prime",
    "change-layer-cohesion-effective",
    "change-layer-cohesion-undrained",
    "change-layer-poisson-ratio",
    "change-layer-cpt-qc",
    "change-layer-spt-n",
    "change-footing-length",
    "change-footing-base-incl-deg",
    "change-footing-settlement-limit",
    "change-load-case-design-situation",
    "change-load-case-vertical-perm",
    "change-load-case-vertical-var",
    "change-load-case-horiz-permanent",
    "change-load-case-horiz-variable",
    "change-load-case-moment-permanent",
    "change-load-case-moment-variable",
    "change-pile-pile-type",
    "change-pile-diameter",
    "change-pile-alpha-s",
    "change-pile-unit-shaft-resistance",
    "change-pile-unit-base-resistance",
    "change-pile-compression-permanent",
    "change-pile-compression-variable",
    "change-pile-tension-permanent",
    "change-pile-tension-variable",
    "change-profile-shaft-resistance",
    "change-profile-base-resistance",
    "change-wall-height",
    "change-wall-embedment",
    "change-wall-stem-thickness",
    "change-wall-backfill-phi-deg",
    "change-wall-backfill-gamma",
    "change-wall-wall-friction-deg",
    "change-wall-earth-pressure-mode",
    "change-wall-wall-movement",
    "change-wall-ocr",
    "change-wall-concrete-gamma",
    "change-wall-surcharge",
    "change-wall-vertical-permanent",
    "change-wall-horizontal-permanent",
    "change-slope-height",
    "change-slope-length",
    "change-slope-governing-layer-id",
    "change-slope-circle-centre-x",
    "change-slope-circle-centre-z",
    "change-slope-circle-radius",
    "change-uplift-perm-stabilizing",
    "change-uplift-perm-destabilizing",
    "change-uplift-var-destabilizing",
    "change-uplift-pore-pressure",
    "change-uplift-total-stress",
    "change-anchor-wall-id",
    "change-anchor-tendon",
    "change-anchor-head-depth",
    "change-anchor-inclination-deg",
    "change-anchor-free-length",
    "change-anchor-fixed-length",
    "change-anchor-fixed-diameter",
    "change-anchor-bond-stress",
    "change-anchor-design-force",
    "change-anchor-tendon-yield-force",
    "change-anchor-min-free-length",
    "change-anchor-test-factor",
    "change-anchor-test-pull-out-res",
    "change-emb-wall-kind",
    "change-emb-wall-retained-height",
    "change-emb-wall-embedment",
    "change-emb-wall-prop-depth",
    "change-emb-wall-thickness",
    "change-emb-wall-backfill-gamma",
    "change-emb-wall-phi-deg",
    "change-emb-wall-delta-active-deg",
    "change-emb-wall-delta-passive-deg",
    "change-emb-wall-surcharge",
    "change-emb-wall-undr-strength",
    "change-pile-group-pile-id",
    "change-pile-group-rows",
    "change-pile-group-columns",
    "change-pile-group-spacing",
    "change-pile-group-cap-vertical",
    "change-pile-group-cap-moment-x",
    "change-pile-group-cap-moment-y",
    "change-pile-group-single-res",
    "change-pile-group-shaft-coh-sum",
    "change-pile-group-base-cohesion",
    "change-lab-test-layer-id",
    "change-lab-test-kind",
    "change-lab-test-void-ratio",
    "change-lab-test-preload-stress",
    "change-lab-test-drainage-height",
    "change-lab-test-consol-time",
    "change-lab-point-normal-stress",
    "change-lab-point-shear-stress",
    "change-water-layer-id",
    "change-water-piezometric-depth",
    "change-water-permeability",
    "change-water-head-difference",
    "change-water-flow-length",
    "change-structure-id",
    "insert-load-case",
    "remove-load-case",
    "insert-test-profile",
    "remove-test-profile",
    "insert-anchor-test",
    "remove-anchor-test",
    "insert-lab-point",
    "remove-lab-point",
    "insert-ground-investigation-reading",
    "remove-ground-investigation-reading",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-slope-width",
    "change-pile-bending-stiffness-ei",
    "change-pile-horiz-bed-modulus",
    "change-pile-lateral-head-load",
    "change-wall-seismic-kh",
    "change-wall-seismic-kv",

    "change-pile-elastic-modulus",
    "change-pile-settling-thickness",
    "change-pile-neg-friction-beta",
    "change-pile-dynamic-test-count",
    "change-pile-dynamic-res-mean",
    "change-pile-dynamic-res-min",
    "change-pile-hammer-weight",
    "change-pile-drop-height",
    "change-pile-set-per-blow",
    "change-pile-elastic-compression",
    "change-pile-hammer-efficiency",
    "change-wall-reinforcement-spacing",
    "change-wall-reinforcement-width",
    "change-wall-rebar-friction",
    "change-lab-test-compression-index",
    "change-lab-test-swelling-index",
    "change-lab-test-stress-increment",
    "change-lab-test-sec-compr-index",
    "change-lab-test-design-life",
    "change-water-well-radius",
    "change-water-well-water-depth",
    "change-water-influence-radius",
    "change-water-well-capacity",
    "change-pile-concrete-class",
    "change-ground-investigation-position-x",
    "change-ground-investigation-position-y",
    "change-ground-investigation-level",
    "change-ground-investigation-depth",
    "change-ground-investigation-kind",
    "change-ground-reading-depth",
    "change-ground-reading-measurement",
    "change-characteristic-basis-layer-id",
    "change-characteristic-basis-parameter",
    "change-characteristic-basis-basis",
    "change-layer-cpt-nkt",
];
//#endregion 🔖️Mutations

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1997_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1997_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1997_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1997_mutations_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1997Mutation;
    use semio_s_artifact_norm_en1997_model::En1997Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1997_mutation(base: &En1997Snapshot, mutation: &En1997Mutation) -> Result<(En1997Snapshot, Vec<String>), String> {
        let raised = <En1997Mutation as protocol::ColdMutation<En1997Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1997_mutation(mutation: &En1997Mutation, base: &En1997Snapshot) -> Result<Vec<En1997Mutation>, semio_framework_value::ValueError> {
        <En1997Mutation as protocol::ColdMutation<En1997Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
