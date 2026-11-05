//! 🧬️ En1992 closed semantic mutation vocabulary — hierarchical RC structure subject.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1992_diff::En1992Diff;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES, ID_REMOVALS};

use semio_s_artifact_norm_en1992_mut_project::change_annex;
use semio_s_artifact_norm_en1992_mut_project::change_reliability_class;
use semio_s_artifact_norm_en1992_mut_project::change_title;
use semio_s_artifact_norm_en1992_mut_project::change_design_working_life;
use semio_s_artifact_norm_en1992_mut_project::change_delta_c_dev;
use semio_s_artifact_norm_en1992_mut_project::change_cement_type;
use semio_s_artifact_norm_en1992_mut_project::change_concrete_f_ck;
use semio_s_artifact_norm_en1992_mut_project::change_reinforcement_f_yk;
use semio_s_artifact_norm_en1992_mut_members::insert_member;
use semio_s_artifact_norm_en1992_mut_members::remove_member;
use semio_s_artifact_norm_en1992_mut_members::change_member_order;
use semio_s_artifact_norm_en1992_mut_members::change_member_width;
use semio_s_artifact_norm_en1992_mut_members::change_member_height;
use semio_s_artifact_norm_en1992_mut_members::change_member_effective_depth;
use semio_s_artifact_norm_en1992_mut_members::change_member_cover;
use semio_s_artifact_norm_en1992_mut_members::change_member_exposure;
use semio_s_artifact_norm_en1992_mut_members::change_member_span;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_bridge_delta_sigma_s;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_bridge_sigma_c;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_liquid_s_r_max;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_liquid_f_ct_eff;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_liquid_rho_p_eff;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_liquid_sigma_s;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_hd_over_h;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_udl;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_buckling_length;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_relative_humidity;
use semio_s_artifact_norm_en1992_mut_member_fields::change_member_bearing_width;
use semio_s_artifact_norm_en1992_mut_member_fields::{change_member_tightness, clear_member_tightness};
use semio_s_artifact_norm_en1992_mut_resistance::change_member_stirrup_spacing;
use semio_s_artifact_norm_en1992_mut_resistance::change_member_axis_distance;
use semio_s_artifact_norm_en1992_mut_resistance::change_member_fire_rating;
use semio_s_artifact_norm_en1992_mut_resistance::change_bar_layer_count;
use semio_s_artifact_norm_en1992_mut_resistance::change_bar_layer_diameter;
use semio_s_artifact_norm_en1992_mut_resistance::change_action_mk;
use semio_s_artifact_norm_en1992_mut_resistance::change_action_nk;
use semio_s_artifact_norm_en1992_mut_resistance::change_action_vk;
use semio_s_artifact_norm_en1992_mut_anchors::insert_anchor;
use semio_s_artifact_norm_en1992_mut_anchors::remove_anchor;
use semio_s_artifact_norm_en1992_mut_anchors::change_anchor_h_ef;
use semio_s_artifact_norm_en1992_mut_anchors::change_anchor_a_s;
use semio_s_artifact_norm_en1992_mut_concrete_grades::change_concrete_name;
use semio_s_artifact_norm_en1992_mut_reinforcement_grades::change_reinforcement_name;
use semio_s_artifact_norm_en1992_mut_reinforcement_grades::change_reinforcement_e_s;
use semio_s_artifact_norm_en1992_mut_reinforcement_grades::change_reinforcement_ductility;
use semio_s_artifact_norm_en1992_mut_reinforcement_grades::change_reinforcement_k;
use semio_s_artifact_norm_en1992_mut_reinforcement_grades::change_reinforcement_eps_uk;
use semio_s_artifact_norm_en1992_mut_prestress_steels::change_prestress_steel_name;
use semio_s_artifact_norm_en1992_mut_prestress_steels::change_prestress_steel_f_pk;
use semio_s_artifact_norm_en1992_mut_prestress_steels::change_prestress_steel_f_p01k;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_label_en;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_label_de;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_kind;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_concrete_grade_id;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_rebar_grade_id;
use semio_s_artifact_norm_en1992_mut_member_prestress_steel::change_member_prestress_steel_id;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_support;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_stirrups_diameter;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_stirrups_legs;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punch_column_width;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punch_column_depth;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punch_col_position;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_asw;
use semio_s_artifact_norm_en1992_mut_member_prestress_fire::change_member_prestress_force;
use semio_s_artifact_norm_en1992_mut_member_prestress_fire::change_member_prestress_area;
use semio_s_artifact_norm_en1992_mut_member_prestress_fire::change_member_prestress_ecc;
use semio_s_artifact_norm_en1992_mut_member_prestress_fire::change_member_prestress_loss_ratio;
use semio_s_artifact_norm_en1992_mut_member_prestress_fire::change_member_fire_slab_system;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_use_fem;
use semio_s_artifact_norm_en1992_mut_member_identity::change_member_deflection_sensitive;
use semio_s_artifact_norm_en1992_mut_bar_layers::change_bar_layer_position;
use semio_s_artifact_norm_en1992_mut_bar_layers::change_bar_layer_anchorage_length;
use semio_s_artifact_norm_en1992_mut_bar_layers::change_bar_layer_lap_length;
use semio_s_artifact_norm_en1992_mut_bar_layers::change_bar_layer_bond_condition;
use semio_s_artifact_norm_en1992_mut_bar_layers::change_bar_layer_aggregate_size;
use semio_s_artifact_norm_en1992_mut_bar_layers::change_bar_layer_lap_share;
use semio_s_artifact_norm_en1992_mut_member_actions_identity::change_action_kind;
use semio_s_artifact_norm_en1992_mut_member_actions_identity::change_action_category;
use semio_s_artifact_norm_en1992_mut_member_actions_identity::change_action_source;
use semio_s_artifact_norm_en1992_mut_member_actions_loads::change_action_g_k_line;
use semio_s_artifact_norm_en1992_mut_member_actions_loads::change_action_q_k_line;
use semio_s_artifact_norm_en1992_mut_member_actions_loads::change_action_point_force;
use semio_s_artifact_norm_en1992_mut_member_actions_loads::change_action_t_k;
use semio_s_artifact_norm_en1992_mut_member_actions_loads::change_action_v_k_punch;
use semio_s_artifact_norm_en1992_mut_geometry_section_bars::change_section_point_y;
use semio_s_artifact_norm_en1992_mut_geometry_section_bars::change_section_point_z;
use semio_s_artifact_norm_en1992_mut_geometry_section_bars::change_bar_point_layer_id;
use semio_s_artifact_norm_en1992_mut_geometry_section_bars::change_bar_point_y;
use semio_s_artifact_norm_en1992_mut_geometry_section_bars::change_bar_point_z;
use semio_s_artifact_norm_en1992_mut_geometry_section_bars::change_bar_point_diameter;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_tendon_point_x;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_tendon_point_y;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_tendon_point_z;
use semio_s_artifact_norm_en1992_mut_anchors_capacity::change_anchor_cracked;
use semio_s_artifact_norm_en1992_mut_anchors_capacity::change_anchor_f_uk;
use semio_s_artifact_norm_en1992_mut_anchors_capacity::change_anchor_f_yk;
use semio_s_artifact_norm_en1992_mut_anchors_capacity::change_anchor_d;
use semio_s_artifact_norm_en1992_mut_anchors_capacity::change_anchor_c1;
use semio_s_artifact_norm_en1992_mut_anchors_capacity::change_anchor_f_ck;
use semio_s_artifact_norm_en1992_mut_anchors_head::change_anchor_head_diameter;
use semio_s_artifact_norm_en1992_mut_anchor_actions_identity::change_anchor_action_kind;
use semio_s_artifact_norm_en1992_mut_anchor_actions_identity::change_anchor_action_category;
use semio_s_artifact_norm_en1992_mut_anchor_actions_identity::change_anchor_action_source;
use semio_s_artifact_norm_en1992_mut_anchor_actions_loads::change_anchor_action_g_k_line;
use semio_s_artifact_norm_en1992_mut_anchor_actions_loads::change_anchor_action_q_k_line;
use semio_s_artifact_norm_en1992_mut_anchor_actions_loads::change_anchor_action_point_force;
use semio_s_artifact_norm_en1992_mut_anchor_actions_resultants::change_anchor_action_m_k;
use semio_s_artifact_norm_en1992_mut_anchor_actions_resultants::change_anchor_action_n_k;
use semio_s_artifact_norm_en1992_mut_anchor_actions_resultants::change_anchor_action_v_k;
use semio_s_artifact_norm_en1992_mut_anchor_actions_resultants::change_anchor_action_t_k;
use semio_s_artifact_norm_en1992_mut_anchor_actions_resultants::change_anchor_action_v_k_punch;
use semio_s_artifact_norm_en1992_mut_grades_rows::insert_concrete_grade;
use semio_s_artifact_norm_en1992_mut_grades_rows::remove_concrete_grade;
use semio_s_artifact_norm_en1992_mut_grades_rows::insert_reinforcement_grade;
use semio_s_artifact_norm_en1992_mut_grades_rows::remove_reinforcement_grade;
use semio_s_artifact_norm_en1992_mut_grades_rows::insert_prestress_steel;
use semio_s_artifact_norm_en1992_mut_grades_rows::remove_prestress_steel;
use semio_s_artifact_norm_en1992_mut_member_lists_rows::insert_bar_layer;
use semio_s_artifact_norm_en1992_mut_member_lists_rows::remove_bar_layer;
use semio_s_artifact_norm_en1992_mut_member_lists_rows::insert_action;
use semio_s_artifact_norm_en1992_mut_member_lists_rows::remove_action;
use semio_s_artifact_norm_en1992_mut_member_lists_rows::insert_section_point;
use semio_s_artifact_norm_en1992_mut_member_lists_rows::remove_section_point;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::insert_bar_point;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::remove_bar_point;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::insert_tendon_point;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::remove_tendon_point;
use semio_s_artifact_norm_en1992_mut_anchor_lists_rows::insert_anchor_action;
use semio_s_artifact_norm_en1992_mut_anchor_lists_rows::remove_anchor_action;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1992_mut_placements_rows::insert_placement;
use semio_s_artifact_norm_en1992_mut_placements_rows::remove_placement;
use semio_s_artifact_norm_en1992_mut_design_cases::insert_case;
use semio_s_artifact_norm_en1992_mut_design_cases::remove_case;
use semio_s_artifact_norm_en1992_mut_design_cases::change_case_number;
use semio_s_artifact_norm_en1992_mut_design_cases::change_case_text;
use semio_s_artifact_norm_en1992_mut_design_cases::change_case_label_en;
use semio_s_artifact_norm_en1992_mut_design_cases::change_case_label_de;
use semio_s_artifact_norm_en1992_mut_design_cases::change_case_entry_id;
use semio_s_artifact_norm_en1992_mut_geometry_bar_fields::change_bar_point_length;
use semio_s_artifact_norm_en1992_mut_geometry_bar_fields::change_bar_point_bend;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_beta_method;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_moment_c1;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_moment_c2;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_x;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_y;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_width;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_depth;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::insert_opening;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::remove_opening;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_angle;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_count;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_spacing_first;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_spacing_radial;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_diameter;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::insert_stud_row;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::remove_stud_row;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1992Snapshot, diff = En1992Diff, schema = "s.norm.en1992", reference = En1992Snapshot, reference_diff = En1992Diff)]
pub enum En1992Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    ChangeTitle(change_title::ChangeTitle),
    ChangeDesignWorkingLife(change_design_working_life::ChangeDesignWorkingLife),
    ChangeDeltaCDev(change_delta_c_dev::ChangeDeltaCDev),
    ChangeCementType(change_cement_type::ChangeCementType),
    ChangeConcreteFCk(change_concrete_f_ck::ChangeConcreteFCk),
    ChangeReinforcementFYk(change_reinforcement_f_yk::ChangeReinforcementFYk),
    InsertMember(insert_member::InsertMember),
    RemoveMember(remove_member::RemoveMember),
    ChangeMemberOrder(change_member_order::ChangeMemberOrder),
    ChangeMemberWidth(change_member_width::ChangeMemberWidth),
    ChangeMemberHeight(change_member_height::ChangeMemberHeight),
    ChangeMemberEffectiveDepth(change_member_effective_depth::ChangeMemberEffectiveDepth),
    ChangeMemberCover(change_member_cover::ChangeMemberCover),
    ChangeMemberExposure(change_member_exposure::ChangeMemberExposure),
    ChangeMemberSpan(change_member_span::ChangeMemberSpan),
    ChangeMemberStirrupSpacing(change_member_stirrup_spacing::ChangeMemberStirrupSpacing),
    ChangeMemberAxisDistance(change_member_axis_distance::ChangeMemberAxisDistance),
    ChangeMemberFireRating(change_member_fire_rating::ChangeMemberFireRating),
    ChangeBarLayerCount(change_bar_layer_count::ChangeBarLayerCount),
    ChangeBarLayerDiameter(change_bar_layer_diameter::ChangeBarLayerDiameter),
    ChangeActionMk(change_action_mk::ChangeActionMk),
    ChangeActionNk(change_action_nk::ChangeActionNk),
    ChangeActionVk(change_action_vk::ChangeActionVk),
    InsertAnchor(insert_anchor::InsertAnchor),
    RemoveAnchor(remove_anchor::RemoveAnchor),
    ChangeAnchorHEf(change_anchor_h_ef::ChangeAnchorHEf),
    ChangeAnchorAs(change_anchor_a_s::ChangeAnchorAs),
    ChangeMemberRelativeHumidity(change_member_relative_humidity::ChangeMemberRelativeHumidity),
    ChangeMemberBearingWidth(change_member_bearing_width::ChangeMemberBearingWidth),
    ChangeMemberBucklingLength(change_member_buckling_length::ChangeMemberBucklingLength),
    ChangeMemberUdl(change_member_udl::ChangeMemberUdl),
    ChangeMemberHdOverH(change_member_hd_over_h::ChangeMemberHdOverH),
    ChangeMemberLiquidSigmaS(change_member_liquid_sigma_s::ChangeMemberLiquidSigmaS),
    ChangeMemberLiquidRhoPEff(change_member_liquid_rho_p_eff::ChangeMemberLiquidRhoPEff),
    ChangeMemberLiquidFCtEff(change_member_liquid_f_ct_eff::ChangeMemberLiquidFCtEff),
    ChangeMemberLiquidSRMax(change_member_liquid_s_r_max::ChangeMemberLiquidSRMax),
    ChangeMemberBridgeSigmaC(change_member_bridge_sigma_c::ChangeMemberBridgeSigmaC),
    ChangeMemberBridgeDeltaSigmaS(change_member_bridge_delta_sigma_s::ChangeMemberBridgeDeltaSigmaS),
    ChangeConcreteName(change_concrete_name::ChangeConcreteName),
    ChangeReinforcementName(change_reinforcement_name::ChangeReinforcementName),
    ChangeReinforcementES(change_reinforcement_e_s::ChangeReinforcementES),
    ChangeReinforcementDuctility(change_reinforcement_ductility::ChangeReinforcementDuctility),
    ChangeReinforcementK(change_reinforcement_k::ChangeReinforcementK),
    ChangeReinforcementEpsUk(change_reinforcement_eps_uk::ChangeReinforcementEpsUk),
    ChangePrestressSteelName(change_prestress_steel_name::ChangePrestressSteelName),
    ChangePrestressSteelFPk(change_prestress_steel_f_pk::ChangePrestressSteelFPk),
    ChangePrestressSteelFP01k(change_prestress_steel_f_p01k::ChangePrestressSteelFP01k),
    ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn),
    ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe),
    ChangeMemberKind(change_member_kind::ChangeMemberKind),
    ChangeMemberConcreteGradeId(change_member_concrete_grade_id::ChangeMemberConcreteGradeId),
    ChangeMemberRebarGradeId(change_member_rebar_grade_id::ChangeMemberRebarGradeId),
    ChangeMemberPrestressSteelId(change_member_prestress_steel_id::ChangeMemberPrestressSteelId),
    ChangeMemberSupport(change_member_support::ChangeMemberSupport),
    ChangeMemberStirrupsDiameter(change_member_stirrups_diameter::ChangeMemberStirrupsDiameter),
    ChangeMemberStirrupsLegs(change_member_stirrups_legs::ChangeMemberStirrupsLegs),
    ChangeMemberPunchColumnWidth(change_member_punch_column_width::ChangeMemberPunchColumnWidth),
    ChangeMemberPunchColumnDepth(change_member_punch_column_depth::ChangeMemberPunchColumnDepth),
    ChangeMemberPunchColPosition(change_member_punch_col_position::ChangeMemberPunchColPosition),
    ChangeMemberPunchingAsw(change_member_punching_asw::ChangeMemberPunchingAsw),
    ChangeMemberPrestressForce(change_member_prestress_force::ChangeMemberPrestressForce),
    ChangeMemberPrestressArea(change_member_prestress_area::ChangeMemberPrestressArea),
    ChangeMemberPrestressEcc(change_member_prestress_ecc::ChangeMemberPrestressEcc),
    ChangeMemberPrestressLossRatio(change_member_prestress_loss_ratio::ChangeMemberPrestressLossRatio),
    ChangeMemberFireSlabSystem(change_member_fire_slab_system::ChangeMemberFireSlabSystem),
    ChangeMemberUseFem(change_member_use_fem::ChangeMemberUseFem),
    ChangeMemberDeflectionSensitive(change_member_deflection_sensitive::ChangeMemberDeflectionSensitive),
    ChangeBarLayerPosition(change_bar_layer_position::ChangeBarLayerPosition),
    ChangeBarLayerAnchorageLength(change_bar_layer_anchorage_length::ChangeBarLayerAnchorageLength),
    ChangeBarLayerLapLength(change_bar_layer_lap_length::ChangeBarLayerLapLength),
    ChangeBarLayerBondCondition(change_bar_layer_bond_condition::ChangeBarLayerBondCondition),
    ChangeBarLayerAggregateSize(change_bar_layer_aggregate_size::ChangeBarLayerAggregateSize),
    ChangeBarLayerLapShare(change_bar_layer_lap_share::ChangeBarLayerLapShare),
    ChangeActionKind(change_action_kind::ChangeActionKind),
    ChangeActionCategory(change_action_category::ChangeActionCategory),
    ChangeActionSource(change_action_source::ChangeActionSource),
    ChangeActionGKLine(change_action_g_k_line::ChangeActionGKLine),
    ChangeActionQKLine(change_action_q_k_line::ChangeActionQKLine),
    ChangeActionPointForce(change_action_point_force::ChangeActionPointForce),
    ChangeActionTK(change_action_t_k::ChangeActionTK),
    ChangeActionVKPunch(change_action_v_k_punch::ChangeActionVKPunch),
    ChangeSectionPointY(change_section_point_y::ChangeSectionPointY),
    ChangeSectionPointZ(change_section_point_z::ChangeSectionPointZ),
    ChangeBarPointLayerId(change_bar_point_layer_id::ChangeBarPointLayerId),
    ChangeBarPointY(change_bar_point_y::ChangeBarPointY),
    ChangeBarPointZ(change_bar_point_z::ChangeBarPointZ),
    ChangeBarPointDiameter(change_bar_point_diameter::ChangeBarPointDiameter),
    ChangeTendonPointX(change_tendon_point_x::ChangeTendonPointX),
    ChangeTendonPointY(change_tendon_point_y::ChangeTendonPointY),
    ChangeTendonPointZ(change_tendon_point_z::ChangeTendonPointZ),
    ChangeAnchorCracked(change_anchor_cracked::ChangeAnchorCracked),
    ChangeAnchorFUk(change_anchor_f_uk::ChangeAnchorFUk),
    ChangeAnchorFYk(change_anchor_f_yk::ChangeAnchorFYk),
    ChangeAnchorD(change_anchor_d::ChangeAnchorD),
    ChangeAnchorC1(change_anchor_c1::ChangeAnchorC1),
    ChangeAnchorFCk(change_anchor_f_ck::ChangeAnchorFCk),
    ChangeAnchorHeadDiameter(change_anchor_head_diameter::ChangeAnchorHeadDiameter),
    ChangeAnchorActionKind(change_anchor_action_kind::ChangeAnchorActionKind),
    ChangeAnchorActionCategory(change_anchor_action_category::ChangeAnchorActionCategory),
    ChangeAnchorActionSource(change_anchor_action_source::ChangeAnchorActionSource),
    ChangeAnchorActionGKLine(change_anchor_action_g_k_line::ChangeAnchorActionGKLine),
    ChangeAnchorActionQKLine(change_anchor_action_q_k_line::ChangeAnchorActionQKLine),
    ChangeAnchorActionPointForce(change_anchor_action_point_force::ChangeAnchorActionPointForce),
    ChangeAnchorActionMK(change_anchor_action_m_k::ChangeAnchorActionMK),
    ChangeAnchorActionNK(change_anchor_action_n_k::ChangeAnchorActionNK),
    ChangeAnchorActionVK(change_anchor_action_v_k::ChangeAnchorActionVK),
    ChangeAnchorActionTK(change_anchor_action_t_k::ChangeAnchorActionTK),
    ChangeAnchorActionVKPunch(change_anchor_action_v_k_punch::ChangeAnchorActionVKPunch),
    InsertConcreteGrade(insert_concrete_grade::InsertConcreteGrade),
    RemoveConcreteGrade(remove_concrete_grade::RemoveConcreteGrade),
    InsertReinforcementGrade(insert_reinforcement_grade::InsertReinforcementGrade),
    RemoveReinforcementGrade(remove_reinforcement_grade::RemoveReinforcementGrade),
    InsertPrestressSteel(insert_prestress_steel::InsertPrestressSteel),
    RemovePrestressSteel(remove_prestress_steel::RemovePrestressSteel),
    InsertBarLayer(insert_bar_layer::InsertBarLayer),
    RemoveBarLayer(remove_bar_layer::RemoveBarLayer),
    InsertAction(insert_action::InsertAction),
    RemoveAction(remove_action::RemoveAction),
    InsertSectionPoint(insert_section_point::InsertSectionPoint),
    RemoveSectionPoint(remove_section_point::RemoveSectionPoint),
    InsertBarPoint(insert_bar_point::InsertBarPoint),
    RemoveBarPoint(remove_bar_point::RemoveBarPoint),
    InsertTendonPoint(insert_tendon_point::InsertTendonPoint),
    RemoveTendonPoint(remove_tendon_point::RemoveTendonPoint),
    InsertAnchorAction(insert_anchor_action::InsertAnchorAction),
    RemoveAnchorAction(remove_anchor_action::RemoveAnchorAction),
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
    ChangeMemberTightness(change_member_tightness::ChangeMemberTightness),
    ClearMemberTightness(clear_member_tightness::ClearMemberTightness),
    ChangeBarPointLength(change_bar_point_length::ChangeBarPointLength),
    ChangeBarPointBend(change_bar_point_bend::ChangeBarPointBend),
    ChangeMemberPunchingBetaMethod(change_member_punching_beta_method::ChangeMemberPunchingBetaMethod),
    ChangeMemberPunchingMomentC1(change_member_punching_moment_c1::ChangeMemberPunchingMomentC1),
    ChangeMemberPunchingMomentC2(change_member_punching_moment_c2::ChangeMemberPunchingMomentC2),
    ChangeOpeningX(change_opening_x::ChangeOpeningX),
    ChangeOpeningY(change_opening_y::ChangeOpeningY),
    ChangeOpeningWidth(change_opening_width::ChangeOpeningWidth),
    ChangeOpeningDepth(change_opening_depth::ChangeOpeningDepth),
    InsertOpening(insert_opening::InsertOpening),
    RemoveOpening(remove_opening::RemoveOpening),
    ChangeStudRowAngle(change_stud_row_angle::ChangeStudRowAngle),
    ChangeStudRowCount(change_stud_row_count::ChangeStudRowCount),
    ChangeStudRowSpacingFirst(change_stud_row_spacing_first::ChangeStudRowSpacingFirst),
    ChangeStudRowSpacingRadial(change_stud_row_spacing_radial::ChangeStudRowSpacingRadial),
    ChangeStudRowDiameter(change_stud_row_diameter::ChangeStudRowDiameter),
    InsertStudRow(insert_stud_row::InsertStudRow),
    RemoveStudRow(remove_stud_row::RemoveStudRow),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "change-title",
    "change-design-working-life",
    "change-delta-c-dev",
    "change-cement-type",
    "change-concrete-f-ck",
    "change-reinforcement-f-yk",
    "insert-member",
    "remove-member",
    "change-member-order",
    "change-member-width",
    "change-member-height",
    "change-member-effective-depth",
    "change-member-cover",
    "change-member-exposure",
    "change-member-span",
    "change-member-stirrup-spacing",
    "change-member-axis-distance",
    "change-member-fire-rating",
    "change-bar-layer-count",
    "change-bar-layer-diameter",
    "change-action-mk",
    "change-action-nk",
    "change-action-vk",
    "insert-anchor",
    "remove-anchor",
    "change-anchor-h-ef",
    "change-anchor-as",
    "change-member-relative-humidity",
    "change-member-bearing-width",
    "change-member-buckling-length",
    "change-member-udl",
    "change-member-hd-over-h",
    "change-member-liquid-sigma-s",
    "change-member-liquid-rho-p-eff",
    "change-member-liquid-f-ct-eff",
    "change-member-liquid-sr-max",
    "change-member-bridge-sigma-c",
    "change-member-bridge-delta-sigma-s",
    "change-concrete-name",
    "change-reinforcement-name",
    "change-reinforcement-es",
    "change-reinforcement-ductility",
    "change-reinforcement-k",
    "change-reinforcement-eps-uk",
    "change-prestress-steel-name",
    "change-prestress-steel-f-pk",
    "change-prestress-steel-fp01k",
    "change-member-label-en",
    "change-member-label-de",
    "change-member-kind",
    "change-member-concrete-grade-id",
    "change-member-rebar-grade-id",
    "change-member-prestress-steel-id",
    "change-member-support",
    "change-member-stirrups-diameter",
    "change-member-stirrups-legs",
    "change-member-punch-column-width",
    "change-member-punch-column-depth",
    "change-member-punch-col-position",
    "change-member-punching-asw",
    "change-member-prestress-force",
    "change-member-prestress-area",
    "change-member-prestress-ecc",
    "change-member-prestress-loss-ratio",
    "change-member-fire-slab-system",
    "change-member-use-fem",
    "change-member-deflection-sensitive",
    "change-bar-layer-position",
    "change-bar-layer-anchorage-length",
    "change-bar-layer-lap-length",
    "change-bar-layer-bond-condition",
    "change-bar-layer-aggregate-size",
    "change-bar-layer-lap-share",
    "change-action-kind",
    "change-action-category",
    "change-action-source",
    "change-action-gk-line",
    "change-action-qk-line",
    "change-action-point-force",
    "change-action-tk",
    "change-action-vk-punch",
    "change-section-point-y",
    "change-section-point-z",
    "change-bar-point-layer-id",
    "change-bar-point-y",
    "change-bar-point-z",
    "change-bar-point-diameter",
    "change-tendon-point-x",
    "change-tendon-point-y",
    "change-tendon-point-z",
    "change-anchor-cracked",
    "change-anchor-f-uk",
    "change-anchor-f-yk",
    "change-anchor-d",
    "change-anchor-c1",
    "change-anchor-f-ck",
    "change-anchor-head-diameter",
    "change-anchor-action-kind",
    "change-anchor-action-category",
    "change-anchor-action-source",
    "change-anchor-action-gk-line",
    "change-anchor-action-qk-line",
    "change-anchor-action-point-force",
    "change-anchor-action-mk",
    "change-anchor-action-nk",
    "change-anchor-action-vk",
    "change-anchor-action-tk",
    "change-anchor-action-vk-punch",
    "insert-concrete-grade",
    "remove-concrete-grade",
    "insert-reinforcement-grade",
    "remove-reinforcement-grade",
    "insert-prestress-steel",
    "remove-prestress-steel",
    "insert-bar-layer",
    "remove-bar-layer",
    "insert-action",
    "remove-action",
    "insert-section-point",
    "remove-section-point",
    "insert-bar-point",
    "remove-bar-point",
    "insert-tendon-point",
    "remove-tendon-point",
    "insert-anchor-action",
    "remove-anchor-action",
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
    "change-member-tightness",
    "clear-member-tightness",
    "change-bar-point-length",
    "change-bar-point-bend",
    "change-member-punching-beta-method",
    "change-member-punching-moment-c1",
    "change-member-punching-moment-c2",
    "change-opening-x",
    "change-opening-y",
    "change-opening-width",
    "change-opening-depth",
    "insert-opening",
    "remove-opening",
    "change-stud-row-angle",
    "change-stud-row-count",
    "change-stud-row-spacing-first",
    "change-stud-row-spacing-radial",
    "change-stud-row-diameter",
    "insert-stud-row",
    "remove-stud-row",
];

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#endregion 🧪️Tests

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1992_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1992_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1992_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1992_mutations_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1992Mutation;
    use semio_s_artifact_norm_en1992_model::En1992Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1992_mutation(base: &En1992Snapshot, mutation: &En1992Mutation) -> Result<(En1992Snapshot, Vec<String>), String> {
        let raised = <En1992Mutation as protocol::ColdMutation<En1992Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1992_mutation(mutation: &En1992Mutation, base: &En1992Snapshot) -> Result<Vec<En1992Mutation>, semio_framework_value::ValueError> {
        <En1992Mutation as protocol::ColdMutation<En1992Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
