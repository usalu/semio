//! 🧬️ En1994 artifact — document mutation dispatch for the composite structure subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1994_mut_project::change_annex;
use semio_s_artifact_norm_en1994_mut_project::change_reliability_class;
use semio_s_artifact_norm_en1994_mut_project::change_structure_kind;
use semio_s_artifact_norm_en1994_mut_project::change_steel_fy_pa;
use semio_s_artifact_norm_en1994_mut_project::change_fire_rating;
use semio_s_artifact_norm_en1994_mut_project::change_insulation_thickness_m;
use semio_s_artifact_norm_en1994_mut_project::change_fatigue_detail;
use semio_s_artifact_norm_en1994_mut_beams::insert_beam;
use semio_s_artifact_norm_en1994_mut_beams::remove_beam;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_action_q_area_pa;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_span_m;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_slab_thickness_m;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_transverse_as;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_construction;
use semio_s_artifact_norm_en1994_mut_columns_slabs::insert_column;
use semio_s_artifact_norm_en1994_mut_columns_slabs::remove_column;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_column_action_force_n;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_column_kind;
use semio_s_artifact_norm_en1994_mut_columns_slabs::insert_slab;
use semio_s_artifact_norm_en1994_mut_columns_slabs::remove_slab;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_slab_action_q_area_pa;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_slab_thickness_m;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_spacing_m;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_support;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_rebar_product;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_concrete_f_ck_pa;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_concrete_e_cm_pa;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_ltb_length_m;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_hogging_rebar_m2_per_m;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_bar_spacing_m;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_crack_width_limit_m;
use semio_s_artifact_norm_en1994_mut_beam_fields::change_beam_fatigue_cycles;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_designation;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_height_m;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_width_m;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_web_thickness_m;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_tf_m;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_area_m2;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_w_pl_y_m3;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_second_moment_m4;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_shear_area_m2;
use semio_s_artifact_norm_en1994_mut_beam_steel::change_beam_steel_root_radius_m;
use semio_s_artifact_norm_en1994_mut_beam_sheeting::change_beam_sheeting_profile;
use semio_s_artifact_norm_en1994_mut_beam_sheeting::change_beam_sheeting_height_m;
use semio_s_artifact_norm_en1994_mut_beam_sheeting::change_beam_sheeting_rib_width_m;
use semio_s_artifact_norm_en1994_mut_beam_sheeting::change_beam_sheeting_thickness_m;
use semio_s_artifact_norm_en1994_mut_beam_sheeting::change_beam_sheeting_ribs_parallel;
use semio_s_artifact_norm_en1994_mut_beam_actions::change_beam_action_kind;
use semio_s_artifact_norm_en1994_mut_beam_actions::change_beam_action_category;
use semio_s_artifact_norm_en1994_mut_beam_actions::change_beam_action_stage;
use semio_s_artifact_norm_en1994_mut_beam_actions::change_beam_action_point_force_n;
use semio_s_artifact_norm_en1994_mut_beam_actions::change_beam_action_delta_sigma_pa;
use semio_s_artifact_norm_en1994_mut_beam_actions::change_beam_action_delta_tau_pa;
use semio_s_artifact_norm_en1994_mut_column_section::change_column_length_m;
use semio_s_artifact_norm_en1994_mut_column_section::change_column_outer_size_m;
use semio_s_artifact_norm_en1994_mut_column_section::change_column_wall_thickness_m;
use semio_s_artifact_norm_en1994_mut_column_section::change_column_steel_area_m2;
use semio_s_artifact_norm_en1994_mut_column_section::change_column_steel_f_y_pa;
use semio_s_artifact_norm_en1994_mut_column_section::change_column_concrete_area_m2;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_concrete_f_ck_pa;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_rebar_area_m2;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_rebar_f_yk_pa;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_second_moment_m4;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_buckling_curve;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_rebar_product;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_action_kind;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_action_category;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_action_stage;
use semio_s_artifact_norm_en1994_mut_column_material::change_column_action_moment_nm;
use semio_s_artifact_norm_en1994_mut_slab_fields::change_slab_span_m;
use semio_s_artifact_norm_en1994_mut_slab_fields::change_slab_support;
use semio_s_artifact_norm_en1994_mut_slab_fields::change_slab_f_ck_pa;
use semio_s_artifact_norm_en1994_mut_slab_fields::change_slab_m_factor;
use semio_s_artifact_norm_en1994_mut_slab_fields::change_slab_k_factor;
use semio_s_artifact_norm_en1994_mut_slab_fields::change_slab_reinforcement_m2_per_m;
use semio_s_artifact_norm_en1994_mut_slab_sheeting::change_slab_sheeting_profile;
use semio_s_artifact_norm_en1994_mut_slab_sheeting::change_slab_sheeting_height_m;
use semio_s_artifact_norm_en1994_mut_slab_sheeting::change_slab_sheeting_rib_width_m;
use semio_s_artifact_norm_en1994_mut_slab_sheeting::change_slab_sheeting_thickness_m;
use semio_s_artifact_norm_en1994_mut_slab_sheeting::change_slab_sheeting_ribs_parallel;
use semio_s_artifact_norm_en1994_mut_slab_actions::change_slab_action_kind;
use semio_s_artifact_norm_en1994_mut_slab_actions::change_slab_action_category;
use semio_s_artifact_norm_en1994_mut_slab_actions::change_slab_action_stage;
use semio_s_artifact_norm_en1994_mut_slab_actions::change_slab_action_point_force_n;
use semio_s_artifact_norm_en1994_mut_slab_actions::change_slab_action_delta_sigma_pa;
use semio_s_artifact_norm_en1994_mut_slab_actions::change_slab_action_delta_tau_pa;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_adjacent_span_m;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_patch_bearing_length_m;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_bridge_lanes;
use semio_s_artifact_norm_en1994_mut_column_design::change_column_introduced_force_n;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_cracked_length_fraction;
use semio_s_artifact_norm_en1994_mut_column_design::change_column_section_height_m;
use semio_s_artifact_norm_en1994_mut_column_design::change_column_section_width_m;
use semio_s_artifact_norm_en1994_mut_column_design::change_column_section_tw_m;
use semio_s_artifact_norm_en1994_mut_column_design::change_column_section_tf_m;
use semio_s_artifact_norm_en1994_mut_column_design::change_column_rebar_cover_m;
use semio_s_artifact_norm_en1994_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1994_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1994_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1994_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1994_mut_placements::insert_placement;
use semio_s_artifact_norm_en1994_mut_placements::remove_placement;
use semio_s_artifact_norm_en1994_mut_stud_rows::insert_stud;
use semio_s_artifact_norm_en1994_mut_stud_rows::remove_stud;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_stud_cross_spacing_m;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_stud_edge_distance_m;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_transverse_as_top;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_end_plate_width_m;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_end_plate_height_m;
use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_end_zone_length_m;
use semio_s_artifact_norm_en1994_mut_stud_rows::change_stud_f_u_pa;
use semio_s_artifact_norm_en1994_mut_stud_rows::change_stud_height_m;
use semio_s_artifact_norm_en1994_mut_stud_rows::change_stud_diameter_m;
use semio_s_artifact_norm_en1994_mut_stud_rows::change_stud_x_m;
use semio_s_artifact_norm_en1994_mut_beam_action_rows::insert_beam_action;
use semio_s_artifact_norm_en1994_mut_beam_action_rows::remove_beam_action;
use semio_s_artifact_norm_en1994_mut_column_action_rows::insert_column_action;
use semio_s_artifact_norm_en1994_mut_column_action_rows::remove_column_action;
use semio_s_artifact_norm_en1994_mut_slab_action_rows::insert_slab_action;
use semio_s_artifact_norm_en1994_mut_slab_action_rows::remove_slab_action;
use semio_s_artifact_norm_en1994_mut_beam_action_position::change_beam_action_x_m;
use semio_s_artifact_norm_en1994_mut_slab_action_position::change_slab_action_x_m;
use semio_s_artifact_norm_en1994_mut_slab_action_position::change_slab_action_y_m;
use semio_s_artifact_norm_en1994_mut_design_cases::insert_case;
use semio_s_artifact_norm_en1994_mut_design_cases::remove_case;
use semio_s_artifact_norm_en1994_mut_design_cases::change_case_number;
use semio_s_artifact_norm_en1994_mut_design_cases::change_case_text;
use semio_s_artifact_norm_en1994_mut_design_cases::change_case_label_en;
use semio_s_artifact_norm_en1994_mut_design_cases::change_case_label_de;
use semio_s_artifact_norm_en1994_mut_design_cases::change_case_entry_id;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1994Snapshot, diff = En1994Diff, schema = "s.norm.en1994", reference = En1994Snapshot, reference_diff = En1994Diff)]
pub enum En1994Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    ChangeStructureKind(change_structure_kind::ChangeStructureKind),
    ChangeSteelFYPa(change_steel_fy_pa::ChangeSteelFYPa),
    ChangeFireRating(change_fire_rating::ChangeFireRating),
    ChangeInsulationThicknessM(change_insulation_thickness_m::ChangeInsulationThicknessM),
    ChangeFatigueDetail(change_fatigue_detail::ChangeFatigueDetail),
    InsertBeam(insert_beam::InsertBeam),
    RemoveBeam(remove_beam::RemoveBeam),
    ChangeBeamActionQAreaPa(change_beam_action_q_area_pa::ChangeBeamActionQAreaPa),
    ChangeBeamSpanM(change_beam_span_m::ChangeBeamSpanM),
    ChangeBeamSlabThicknessM(change_beam_slab_thickness_m::ChangeBeamSlabThicknessM),
    ChangeBeamTransverseAs(change_beam_transverse_as::ChangeBeamTransverseAs),
    ChangeBeamConstruction(change_beam_construction::ChangeBeamConstruction),
    InsertColumn(insert_column::InsertColumn),
    RemoveColumn(remove_column::RemoveColumn),
    ChangeColumnActionForceN(change_column_action_force_n::ChangeColumnActionForceN),
    ChangeColumnKind(change_column_kind::ChangeColumnKind),
    InsertSlab(insert_slab::InsertSlab),
    RemoveSlab(remove_slab::RemoveSlab),
    ChangeSlabActionQAreaPa(change_slab_action_q_area_pa::ChangeSlabActionQAreaPa),
    ChangeSlabThicknessM(change_slab_thickness_m::ChangeSlabThicknessM),
    ChangeBeamSpacingM(change_beam_spacing_m::ChangeBeamSpacingM),
    ChangeBeamSupport(change_beam_support::ChangeBeamSupport),
    ChangeBeamRebarProduct(change_beam_rebar_product::ChangeBeamRebarProduct),
    ChangeBeamConcreteFCkPa(change_beam_concrete_f_ck_pa::ChangeBeamConcreteFCkPa),
    ChangeBeamConcreteECmPa(change_beam_concrete_e_cm_pa::ChangeBeamConcreteECmPa),
    ChangeBeamLtbLengthM(change_beam_ltb_length_m::ChangeBeamLtbLengthM),
    ChangeBeamHoggingRebarM2PerM(change_beam_hogging_rebar_m2_per_m::ChangeBeamHoggingRebarM2PerM),
    ChangeBeamBarSpacingM(change_beam_bar_spacing_m::ChangeBeamBarSpacingM),
    ChangeBeamCrackWidthLimitM(change_beam_crack_width_limit_m::ChangeBeamCrackWidthLimitM),
    ChangeBeamFatigueCycles(change_beam_fatigue_cycles::ChangeBeamFatigueCycles),
    ChangeBeamSteelDesignation(change_beam_steel_designation::ChangeBeamSteelDesignation),
    ChangeBeamSteelHeightM(change_beam_steel_height_m::ChangeBeamSteelHeightM),
    ChangeBeamSteelWidthM(change_beam_steel_width_m::ChangeBeamSteelWidthM),
    ChangeBeamSteelWebThicknessM(change_beam_steel_web_thickness_m::ChangeBeamSteelWebThicknessM),
    ChangeBeamSteelTfM(change_beam_steel_tf_m::ChangeBeamSteelTfM),
    ChangeBeamSteelAreaM2(change_beam_steel_area_m2::ChangeBeamSteelAreaM2),
    ChangeBeamSteelWPlYM3(change_beam_steel_w_pl_y_m3::ChangeBeamSteelWPlYM3),
    ChangeBeamSteelSecondMomentM4(change_beam_steel_second_moment_m4::ChangeBeamSteelSecondMomentM4),
    ChangeBeamSteelShearAreaM2(change_beam_steel_shear_area_m2::ChangeBeamSteelShearAreaM2),
    ChangeBeamSteelRootRadiusM(change_beam_steel_root_radius_m::ChangeBeamSteelRootRadiusM),
    ChangeBeamSheetingProfile(change_beam_sheeting_profile::ChangeBeamSheetingProfile),
    ChangeBeamSheetingHeightM(change_beam_sheeting_height_m::ChangeBeamSheetingHeightM),
    ChangeBeamSheetingRibWidthM(change_beam_sheeting_rib_width_m::ChangeBeamSheetingRibWidthM),
    ChangeBeamSheetingThicknessM(change_beam_sheeting_thickness_m::ChangeBeamSheetingThicknessM),
    ChangeBeamSheetingRibsParallel(change_beam_sheeting_ribs_parallel::ChangeBeamSheetingRibsParallel),
    ChangeBeamActionKind(change_beam_action_kind::ChangeBeamActionKind),
    ChangeBeamActionCategory(change_beam_action_category::ChangeBeamActionCategory),
    ChangeBeamActionStage(change_beam_action_stage::ChangeBeamActionStage),
    ChangeBeamActionPointForceN(change_beam_action_point_force_n::ChangeBeamActionPointForceN),
    ChangeBeamActionDeltaSigmaPa(change_beam_action_delta_sigma_pa::ChangeBeamActionDeltaSigmaPa),
    ChangeBeamActionDeltaTauPa(change_beam_action_delta_tau_pa::ChangeBeamActionDeltaTauPa),
    ChangeColumnLengthM(change_column_length_m::ChangeColumnLengthM),
    ChangeColumnOuterSizeM(change_column_outer_size_m::ChangeColumnOuterSizeM),
    ChangeColumnWallThicknessM(change_column_wall_thickness_m::ChangeColumnWallThicknessM),
    ChangeColumnSteelAreaM2(change_column_steel_area_m2::ChangeColumnSteelAreaM2),
    ChangeColumnSteelFYPa(change_column_steel_f_y_pa::ChangeColumnSteelFYPa),
    ChangeColumnConcreteAreaM2(change_column_concrete_area_m2::ChangeColumnConcreteAreaM2),
    ChangeColumnConcreteFCkPa(change_column_concrete_f_ck_pa::ChangeColumnConcreteFCkPa),
    ChangeColumnRebarAreaM2(change_column_rebar_area_m2::ChangeColumnRebarAreaM2),
    ChangeColumnRebarFYkPa(change_column_rebar_f_yk_pa::ChangeColumnRebarFYkPa),
    ChangeColumnSecondMomentM4(change_column_second_moment_m4::ChangeColumnSecondMomentM4),
    ChangeColumnBucklingCurve(change_column_buckling_curve::ChangeColumnBucklingCurve),
    ChangeColumnRebarProduct(change_column_rebar_product::ChangeColumnRebarProduct),
    ChangeColumnActionKind(change_column_action_kind::ChangeColumnActionKind),
    ChangeColumnActionCategory(change_column_action_category::ChangeColumnActionCategory),
    ChangeColumnActionStage(change_column_action_stage::ChangeColumnActionStage),
    ChangeColumnActionMomentNm(change_column_action_moment_nm::ChangeColumnActionMomentNm),
    ChangeSlabSpanM(change_slab_span_m::ChangeSlabSpanM),
    ChangeSlabSupport(change_slab_support::ChangeSlabSupport),
    ChangeSlabFCkPa(change_slab_f_ck_pa::ChangeSlabFCkPa),
    ChangeSlabMFactor(change_slab_m_factor::ChangeSlabMFactor),
    ChangeSlabKFactor(change_slab_k_factor::ChangeSlabKFactor),
    ChangeSlabReinforcementM2PerM(change_slab_reinforcement_m2_per_m::ChangeSlabReinforcementM2PerM),
    ChangeSlabSheetingProfile(change_slab_sheeting_profile::ChangeSlabSheetingProfile),
    ChangeSlabSheetingHeightM(change_slab_sheeting_height_m::ChangeSlabSheetingHeightM),
    ChangeSlabSheetingRibWidthM(change_slab_sheeting_rib_width_m::ChangeSlabSheetingRibWidthM),
    ChangeSlabSheetingThicknessM(change_slab_sheeting_thickness_m::ChangeSlabSheetingThicknessM),
    ChangeSlabSheetingRibsParallel(change_slab_sheeting_ribs_parallel::ChangeSlabSheetingRibsParallel),
    ChangeSlabActionKind(change_slab_action_kind::ChangeSlabActionKind),
    ChangeSlabActionCategory(change_slab_action_category::ChangeSlabActionCategory),
    ChangeSlabActionStage(change_slab_action_stage::ChangeSlabActionStage),
    ChangeSlabActionPointForceN(change_slab_action_point_force_n::ChangeSlabActionPointForceN),
    ChangeSlabActionDeltaSigmaPa(change_slab_action_delta_sigma_pa::ChangeSlabActionDeltaSigmaPa),
    ChangeSlabActionDeltaTauPa(change_slab_action_delta_tau_pa::ChangeSlabActionDeltaTauPa),
    ChangeBeamAdjacentSpanM(change_beam_adjacent_span_m::ChangeBeamAdjacentSpanM),
    ChangeBeamPatchBearingLengthM(change_beam_patch_bearing_length_m::ChangeBeamPatchBearingLengthM),
    ChangeBeamBridgeLanes(change_beam_bridge_lanes::ChangeBeamBridgeLanes),
    ChangeColumnIntroducedForceN(change_column_introduced_force_n::ChangeColumnIntroducedForceN),
    ChangeBeamCrackedLengthFraction(change_beam_cracked_length_fraction::ChangeBeamCrackedLengthFraction),
    ChangeColumnSectionHeightM(change_column_section_height_m::ChangeColumnSectionHeightM),
    ChangeColumnSectionWidthM(change_column_section_width_m::ChangeColumnSectionWidthM),
    ChangeColumnSectionTwM(change_column_section_tw_m::ChangeColumnSectionTwM),
    ChangeColumnSectionTfM(change_column_section_tf_m::ChangeColumnSectionTfM),
    ChangeColumnRebarCoverM(change_column_rebar_cover_m::ChangeColumnRebarCoverM),
    ChangePlacementX(change_placement_x::ChangePlacementX),
    ChangePlacementY(change_placement_y::ChangePlacementY),
    ChangePlacementZ(change_placement_z::ChangePlacementZ),
    ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw),
    InsertPlacement(insert_placement::InsertPlacement),
    RemovePlacement(remove_placement::RemovePlacement),
    InsertStud(insert_stud::InsertStud),
    RemoveStud(remove_stud::RemoveStud),
    ChangeBeamStudCrossSpacingM(change_beam_stud_cross_spacing_m::ChangeBeamStudCrossSpacingM),
    ChangeBeamStudEdgeDistanceM(change_beam_stud_edge_distance_m::ChangeBeamStudEdgeDistanceM),
    ChangeBeamTransverseAsTop(change_beam_transverse_as_top::ChangeBeamTransverseAsTop),
    ChangeBeamEndPlateWidthM(change_beam_end_plate_width_m::ChangeBeamEndPlateWidthM),
    ChangeBeamEndPlateHeightM(change_beam_end_plate_height_m::ChangeBeamEndPlateHeightM),
    ChangeBeamEndZoneLengthM(change_beam_end_zone_length_m::ChangeBeamEndZoneLengthM),
    ChangeStudFUPa(change_stud_f_u_pa::ChangeStudFUPa),
    ChangeStudHeightM(change_stud_height_m::ChangeStudHeightM),
    ChangeStudDiameterM(change_stud_diameter_m::ChangeStudDiameterM),
    ChangeStudXM(change_stud_x_m::ChangeStudXM),
    InsertBeamAction(insert_beam_action::InsertBeamAction),
    RemoveBeamAction(remove_beam_action::RemoveBeamAction),
    InsertColumnAction(insert_column_action::InsertColumnAction),
    RemoveColumnAction(remove_column_action::RemoveColumnAction),
    InsertSlabAction(insert_slab_action::InsertSlabAction),
    RemoveSlabAction(remove_slab_action::RemoveSlabAction),
    ChangeBeamActionXM(change_beam_action_x_m::ChangeBeamActionXM),
    ChangeSlabActionXM(change_slab_action_x_m::ChangeSlabActionXM),
    ChangeSlabActionYM(change_slab_action_y_m::ChangeSlabActionYM),
    InsertCase(insert_case::InsertCase),
    RemoveCase(remove_case::RemoveCase),
    ChangeCaseNumber(change_case_number::ChangeCaseNumber),
    ChangeCaseText(change_case_text::ChangeCaseText),
    ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn),
    ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe),
    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "change-structure-kind",
    "change-steel-fy-pa",
    "change-fire-rating",
    "change-insulation-thickness-m",
    "change-fatigue-detail",
    "insert-beam",
    "remove-beam",
    "change-beam-action-q-area-pa",
    "change-beam-span-m",
    "change-beam-slab-thickness-m",
    "change-beam-transverse-as",
    "change-beam-construction",
    "insert-column",
    "remove-column",
    "change-column-action-force-n",
    "change-column-kind",
    "insert-slab",
    "remove-slab",
    "change-slab-action-q-area-pa",
    "change-slab-thickness-m",
    "change-beam-spacing-m",
    "change-beam-support",
    "change-beam-rebar-product",
    "change-beam-concrete-f-ck-pa",
    "change-beam-concrete-e-cm-pa",
    "change-beam-ltb-length-m",
    "change-beam-hogging-rebar-m2-per-m",
    "change-beam-bar-spacing-m",
    "change-beam-crack-width-limit-m",
    "change-beam-fatigue-cycles",
    "change-beam-steel-designation",
    "change-beam-steel-height-m",
    "change-beam-steel-width-m",
    "change-beam-steel-web-thickness-m",
    "change-beam-steel-tf-m",
    "change-beam-steel-area-m2",
    "change-beam-steel-w-pl-ym3",
    "change-beam-steel-second-moment-m4",
    "change-beam-steel-shear-area-m2",
    "change-beam-steel-root-radius-m",
    "change-beam-sheeting-profile",
    "change-beam-sheeting-height-m",
    "change-beam-sheeting-rib-width-m",
    "change-beam-sheeting-thickness-m",
    "change-beam-sheeting-ribs-parallel",
    "change-beam-action-kind",
    "change-beam-action-category",
    "change-beam-action-stage",
    "change-beam-action-point-force-n",
    "change-beam-action-delta-sigma-pa",
    "change-beam-action-delta-tau-pa",
    "change-column-length-m",
    "change-column-outer-size-m",
    "change-column-wall-thickness-m",
    "change-column-steel-area-m2",
    "change-column-steel-fy-pa",
    "change-column-concrete-area-m2",
    "change-column-concrete-f-ck-pa",
    "change-column-rebar-area-m2",
    "change-column-rebar-f-yk-pa",
    "change-column-second-moment-m4",
    "change-column-buckling-curve",
    "change-column-rebar-product",
    "change-column-action-kind",
    "change-column-action-category",
    "change-column-action-stage",
    "change-column-action-moment-nm",
    "change-slab-span-m",
    "change-slab-support",
    "change-slab-f-ck-pa",
    "change-slab-m-factor",
    "change-slab-k-factor",
    "change-slab-reinforcement-m2-per-m",
    "change-slab-sheeting-profile",
    "change-slab-sheeting-height-m",
    "change-slab-sheeting-rib-width-m",
    "change-slab-sheeting-thickness-m",
    "change-slab-sheeting-ribs-parallel",
    "change-slab-action-kind",
    "change-slab-action-category",
    "change-slab-action-stage",
    "change-slab-action-point-force-n",
    "change-slab-action-delta-sigma-pa",
    "change-slab-action-delta-tau-pa",
    "change-beam-adjacent-span-m",
    "change-beam-patch-bearing-length-m",
    "change-beam-bridge-lanes",
    "change-column-introduced-force-n",
    "change-beam-cracked-length-fraction",
    "change-column-section-height-m",
    "change-column-section-width-m",
    "change-column-section-tw-m",
    "change-column-section-tf-m",
    "change-column-rebar-cover-m",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "insert-stud",
    "remove-stud",
    "change-beam-stud-cross-spacing-m",
    "change-beam-stud-edge-distance-m",
    "change-beam-transverse-as-top",
    "change-beam-end-plate-width-m",
    "change-beam-end-plate-height-m",
    "change-beam-end-zone-length-m",
    "change-stud-fu-pa",
    "change-stud-height-m",
    "change-stud-diameter-m",
    "change-stud-xm",
    "insert-beam-action",
    "remove-beam-action",
    "insert-column-action",
    "remove-column-action",
    "insert-slab-action",
    "remove-slab-action",
    "change-beam-action-xm",
    "change-slab-action-xm",
    "change-slab-action-ym",
    "insert-case",
    "remove-case",
    "change-case-number",
    "change-case-text",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
];
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️Tests

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
mod testkit;
#[cfg(any(test, feature = "testkit"))]
pub use testkit::demo_mutation_cases;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1994_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1994_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1994_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1994_mutations_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1994Mutation;
    use semio_s_artifact_norm_en1994_model::En1994Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1994_mutation(base: &En1994Snapshot, mutation: &En1994Mutation) -> Result<(En1994Snapshot, Vec<String>), String> {
        let raised = <En1994Mutation as protocol::ColdMutation<En1994Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1994_mutation(mutation: &En1994Mutation, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
        <En1994Mutation as protocol::ColdMutation<En1994Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
