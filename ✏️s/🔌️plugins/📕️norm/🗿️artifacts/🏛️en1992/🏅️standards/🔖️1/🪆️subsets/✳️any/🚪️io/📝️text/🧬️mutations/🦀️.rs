//! ⚡️ EN 1992 mutation text wire — minimal codec for hierarchical vocabulary.

use crate::En1992Mutation;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_angle;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_count;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_spacing_first;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_spacing_radial;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_stud_row_diameter;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::insert_stud_row;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::remove_stud_row;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_x;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_y;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_width;
use semio_s_artifact_norm_en1992_mut_geometry_tendons_studs::change_opening_depth;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::insert_opening;
use semio_s_artifact_norm_en1992_mut_member_geometry_rows::remove_opening;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_beta_method;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_moment_c1;
use semio_s_artifact_norm_en1992_mut_member_shear::change_member_punching_moment_c2;
use semio_s_artifact_norm_en1992_mut_geometry_bar_fields::change_bar_point_length;
use semio_s_artifact_norm_en1992_mut_geometry_bar_fields::change_bar_point_bend;
use semio_s_artifact_norm_en1992_mut_design_cases::{insert_case, remove_case, change_case_number, change_case_text, change_case_label_en, change_case_label_de, change_case_entry_id};
use semio_s_artifact_norm_en1992_mut_placements::change_placement_x;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_y;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_z;
use semio_s_artifact_norm_en1992_mut_placements::change_placement_yaw;
use semio_s_artifact_norm_en1992_mut_placements_rows::insert_placement;
use semio_s_artifact_norm_en1992_mut_placements_rows::remove_placement;
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
use crate::*;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

fn enc_json<T: semio_framework_value::ToValue>(v: &T) -> String {
    semio_framework_pack_json::to_json_string(v)
}
fn enc_token<T: semio_framework_value::ToValue>(v: &T) -> String {
    enc_json(v).replace(' ', "\\u0020")
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(s, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

/// 🖨️ Print mutation as `kind key=value…` line.
pub fn print_op(op: &En1992Mutation) -> String {
    match op {
        En1992Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1992Mutation::ChangeReliabilityClass(p) => format!("change-reliability-class new-reliability-class={}", enc_json(&p.new_reliability_class)),
        En1992Mutation::ChangeTitle(p) => format!("change-title new-title={}", enc_json(&p.new_title)),
        En1992Mutation::ChangeDesignWorkingLife(p) => format!("change-design-working-life new-years={}", p.new_years),
        En1992Mutation::ChangeDeltaCDev(p) => format!("change-delta-c-dev new-delta-c-dev={}", p.new_delta_c_dev),
        En1992Mutation::ChangeCementType(p) => format!("change-cement-type new-cement-type={}", enc_json(&p.new_cement_type)),
        En1992Mutation::ChangeConcreteFCk(p) => format!("change-concrete-f-ck grade-id={} new-f-ck={}", enc_json(&p.grade_id), p.new_f_ck),
        En1992Mutation::ChangeReinforcementFYk(p) => format!("change-reinforcement-f-yk grade-id={} new-f-yk={}", enc_json(&p.grade_id), p.new_f_yk),
        En1992Mutation::InsertMember(p) => format!("insert-member index={} member={}", p.index, enc_json(&p.member)),
        En1992Mutation::RemoveMember(p) => format!("remove-member member-id={}", enc_json(&p.member_id)),
        En1992Mutation::ChangeMemberOrder(p) => format!("change-member-order from-index={} to-index={}", p.from_index, p.to_index),
        En1992Mutation::ChangeMemberWidth(p) => format!("change-member-width member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberHeight(p) => format!("change-member-height member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberEffectiveDepth(p) => format!("change-member-effective-depth member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberCover(p) => format!("change-member-cover member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberExposure(p) => format!("change-member-exposure member-id={} new-exposure={}", enc_json(&p.member_id), enc_json(&p.new_exposure)),
        En1992Mutation::ChangeMemberSpan(p) => format!("change-member-span member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberStirrupSpacing(p) => format!("change-member-stirrup-spacing member-id={} new-spacing={}", enc_json(&p.member_id), p.new_spacing),
        En1992Mutation::ChangeMemberAxisDistance(p) => format!("change-member-axis-distance member-id={} new-axis-distance={}", enc_json(&p.member_id), p.new_axis_distance),
        En1992Mutation::ChangeMemberFireRating(p) => format!("change-member-fire-rating member-id={} new-rating={}", enc_json(&p.member_id), enc_json(&p.new_rating)),
        En1992Mutation::ChangeBarLayerCount(p) => format!("change-bar-layer-count member-id={} layer-id={} new-count={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_count),
        En1992Mutation::ChangeBarLayerDiameter(p) => format!("change-bar-layer-diameter member-id={} layer-id={} new-diameter={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_diameter),
        En1992Mutation::ChangeActionMk(p) => format!("change-action-mk member-id={} action-id={} new-value={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_value),
        En1992Mutation::ChangeActionNk(p) => format!("change-action-nk member-id={} action-id={} new-value={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_value),
        En1992Mutation::ChangeActionVk(p) => format!("change-action-vk member-id={} action-id={} new-value={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_value),
        En1992Mutation::InsertAnchor(p) => format!("insert-anchor index={} anchor={}", p.index, enc_json(&p.anchor)),
        En1992Mutation::RemoveAnchor(p) => format!("remove-anchor anchor-id={}", enc_json(&p.anchor_id)),
        En1992Mutation::ChangeAnchorHEf(p) => format!("change-anchor-h-ef anchor-id={} new-value={}", enc_json(&p.anchor_id), p.new_value),
        En1992Mutation::ChangeAnchorAs(p) => format!("change-anchor-as anchor-id={} new-value={}", enc_json(&p.anchor_id), p.new_value),
        En1992Mutation::ChangeMemberBridgeDeltaSigmaS(p) => format!("change-member-bridge-delta-sigma-s member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberBridgeSigmaC(p) => format!("change-member-bridge-sigma-c member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberLiquidSRMax(p) => format!("change-member-liquid-sr-max member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberLiquidFCtEff(p) => format!("change-member-liquid-f-ct-eff member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberLiquidRhoPEff(p) => format!("change-member-liquid-rho-p-eff member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberLiquidSigmaS(p) => format!("change-member-liquid-sigma-s member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberHdOverH(p) => format!("change-member-hd-over-h member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberUdl(p) => format!("change-member-udl member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberBucklingLength(p) => format!("change-member-buckling-length member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberRelativeHumidity(p) => format!("change-member-relative-humidity member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberBearingWidth(p) => format!("change-member-bearing-width member-id={} new-value={}", enc_json(&p.member_id), p.new_value),
        En1992Mutation::ChangeMemberTightness(p) => format!("change-member-tightness member-id={} new-tightness={}", enc_json(&p.member_id), enc_json(&p.new_tightness)),
        En1992Mutation::ClearMemberTightness(p) => format!("clear-member-tightness member-id={}", enc_json(&p.member_id)),
        En1992Mutation::ChangeConcreteName(p) => format!("change-concrete-name grade-id={} new-name={}", enc_json(&p.grade_id), enc_json(&p.new_name)),
        En1992Mutation::ChangeReinforcementName(p) => format!("change-reinforcement-name grade-id={} new-name={}", enc_json(&p.grade_id), enc_json(&p.new_name)),
        En1992Mutation::ChangeReinforcementES(p) => format!("change-reinforcement-es grade-id={} new-e-s={}", enc_json(&p.grade_id), p.new_e_s),
        En1992Mutation::ChangeReinforcementDuctility(p) => format!("change-reinforcement-ductility grade-id={} new-ductility={}", enc_json(&p.grade_id), enc_json(&p.new_ductility)),
        En1992Mutation::ChangeReinforcementK(p) => format!("change-reinforcement-k grade-id={} new-k={}", enc_json(&p.grade_id), p.new_k),
        En1992Mutation::ChangeReinforcementEpsUk(p) => format!("change-reinforcement-eps-uk grade-id={} new-eps-uk={}", enc_json(&p.grade_id), p.new_eps_uk),
        En1992Mutation::ChangePrestressSteelName(p) => format!("change-prestress-steel-name grade-id={} new-name={}", enc_json(&p.grade_id), enc_json(&p.new_name)),
        En1992Mutation::ChangePrestressSteelFPk(p) => format!("change-prestress-steel-f-pk grade-id={} new-f-pk={}", enc_json(&p.grade_id), p.new_f_pk),
        En1992Mutation::ChangePrestressSteelFP01k(p) => format!("change-prestress-steel-fp01k grade-id={} new-f-p01k={}", enc_json(&p.grade_id), p.new_f_p01k),
        En1992Mutation::ChangeMemberLabelEn(p) => format!("change-member-label-en member-id={} new-label-en={}", enc_json(&p.member_id), enc_json(&p.new_label_en)),
        En1992Mutation::ChangeMemberLabelDe(p) => format!("change-member-label-de member-id={} new-label-de={}", enc_json(&p.member_id), enc_json(&p.new_label_de)),
        En1992Mutation::ChangeMemberKind(p) => format!("change-member-kind member-id={} new-kind={}", enc_json(&p.member_id), enc_json(&p.new_kind)),
        En1992Mutation::ChangeMemberConcreteGradeId(p) => format!("change-member-concrete-grade-id member-id={} new-concrete-grade-id={}", enc_json(&p.member_id), enc_json(&p.new_concrete_grade_id)),
        En1992Mutation::ChangeMemberRebarGradeId(p) => format!("change-member-rebar-grade-id member-id={} new-reinforcement-grade-id={}", enc_json(&p.member_id), enc_json(&p.new_reinforcement_grade_id)),
        En1992Mutation::ChangeMemberPrestressSteelId(p) => format!("change-member-prestress-steel-id member-id={} new-prestress-steel-id={}", enc_json(&p.member_id), enc_json(&p.new_prestress_steel_id)),
        En1992Mutation::ChangeMemberSupport(p) => format!("change-member-support member-id={} new-support={}", enc_json(&p.member_id), enc_json(&p.new_support)),
        En1992Mutation::ChangeMemberStirrupsDiameter(p) => format!("change-member-stirrups-diameter member-id={} new-stirrups-diameter={}", enc_json(&p.member_id), p.new_stirrups_diameter),
        En1992Mutation::ChangeMemberStirrupsLegs(p) => format!("change-member-stirrups-legs member-id={} new-stirrups-legs={}", enc_json(&p.member_id), p.new_stirrups_legs),
        En1992Mutation::ChangeMemberPunchColumnWidth(p) => format!("change-member-punch-column-width member-id={} new-punching-column-width={}", enc_json(&p.member_id), p.new_punching_column_width),
        En1992Mutation::ChangeMemberPunchColumnDepth(p) => format!("change-member-punch-column-depth member-id={} new-punching-column-depth={}", enc_json(&p.member_id), p.new_punching_column_depth),
        En1992Mutation::ChangeMemberPunchColPosition(p) => format!("change-member-punch-col-position member-id={} new-punching-column-position={}", enc_json(&p.member_id), enc_json(&p.new_punching_column_position)),
        En1992Mutation::ChangeMemberPunchingAsw(p) => format!("change-member-punching-asw member-id={} new-punching-asw={}", enc_json(&p.member_id), p.new_punching_asw),
        En1992Mutation::ChangeMemberPrestressForce(p) => format!("change-member-prestress-force member-id={} new-prestress-force={}", enc_json(&p.member_id), p.new_prestress_force),
        En1992Mutation::ChangeMemberPrestressArea(p) => format!("change-member-prestress-area member-id={} new-prestress-area={}", enc_json(&p.member_id), p.new_prestress_area),
        En1992Mutation::ChangeMemberPrestressEcc(p) => format!("change-member-prestress-ecc member-id={} new-prestress-eccentricity={}", enc_json(&p.member_id), p.new_prestress_eccentricity),
        En1992Mutation::ChangeMemberPrestressLossRatio(p) => format!("change-member-prestress-loss-ratio member-id={} new-prestress-loss-ratio={}", enc_json(&p.member_id), p.new_prestress_loss_ratio),
        En1992Mutation::ChangeMemberFireSlabSystem(p) => format!("change-member-fire-slab-system member-id={} new-fire-slab-system={}", enc_json(&p.member_id), enc_json(&p.new_fire_slab_system)),
        En1992Mutation::ChangeMemberUseFem(p) => format!("change-member-use-fem member-id={} new-use-fem={}", enc_json(&p.member_id), p.new_use_fem),
        En1992Mutation::ChangeMemberDeflectionSensitive(p) => format!("change-member-deflection-sensitive member-id={} new-deflection-sensitive={}", enc_json(&p.member_id), p.new_deflection_sensitive),
        En1992Mutation::ChangeBarLayerPosition(p) => format!("change-bar-layer-position member-id={} layer-id={} new-position={}", enc_json(&p.member_id), enc_json(&p.layer_id), enc_json(&p.new_position)),
        En1992Mutation::ChangeBarLayerAnchorageLength(p) => format!("change-bar-layer-anchorage-length member-id={} layer-id={} new-anchorage-length={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_anchorage_length),
        En1992Mutation::ChangeBarLayerLapLength(p) => format!("change-bar-layer-lap-length member-id={} layer-id={} new-lap-length={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_lap_length),
        En1992Mutation::ChangeBarLayerBondCondition(p) => format!("change-bar-layer-bond-condition member-id={} layer-id={} new-bond-condition={}", enc_json(&p.member_id), enc_json(&p.layer_id), enc_json(&p.new_bond_condition)),
        En1992Mutation::ChangeBarLayerAggregateSize(p) => format!("change-bar-layer-aggregate-size member-id={} layer-id={} new-aggregate-size={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_aggregate_size),
        En1992Mutation::ChangeBarLayerLapShare(p) => format!("change-bar-layer-lap-share member-id={} layer-id={} new-lap-share={}", enc_json(&p.member_id), enc_json(&p.layer_id), p.new_lap_share),
        En1992Mutation::ChangeActionKind(p) => format!("change-action-kind member-id={} action-id={} new-kind={}", enc_json(&p.member_id), enc_json(&p.action_id), enc_json(&p.new_kind)),
        En1992Mutation::ChangeActionCategory(p) => format!("change-action-category member-id={} action-id={} new-category={}", enc_json(&p.member_id), enc_json(&p.action_id), enc_json(&p.new_category)),
        En1992Mutation::ChangeActionSource(p) => format!("change-action-source member-id={} action-id={} new-source={}", enc_json(&p.member_id), enc_json(&p.action_id), enc_json(&p.new_source)),
        En1992Mutation::ChangeActionGKLine(p) => format!("change-action-gk-line member-id={} action-id={} new-g-k-line={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_g_k_line),
        En1992Mutation::ChangeActionQKLine(p) => format!("change-action-qk-line member-id={} action-id={} new-q-k-line={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_q_k_line),
        En1992Mutation::ChangeActionPointForce(p) => format!("change-action-point-force member-id={} action-id={} new-point-force={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_point_force),
        En1992Mutation::ChangeActionTK(p) => format!("change-action-tk member-id={} action-id={} new-t-k={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_t_k),
        En1992Mutation::ChangeActionVKPunch(p) => format!("change-action-vk-punch member-id={} action-id={} new-v-k-punch={}", enc_json(&p.member_id), enc_json(&p.action_id), p.new_v_k_punch),
        En1992Mutation::ChangeSectionPointY(p) => format!("change-section-point-y member-id={} section-point-id={} new-y={}", enc_json(&p.member_id), enc_json(&p.section_point_id), p.new_y),
        En1992Mutation::ChangeSectionPointZ(p) => format!("change-section-point-z member-id={} section-point-id={} new-z={}", enc_json(&p.member_id), enc_json(&p.section_point_id), p.new_z),
        En1992Mutation::ChangeBarPointLayerId(p) => format!("change-bar-point-layer-id member-id={} bar-id={} new-layer-id={}", enc_json(&p.member_id), enc_json(&p.bar_id), enc_json(&p.new_layer_id)),
        En1992Mutation::ChangeBarPointY(p) => format!("change-bar-point-y member-id={} bar-id={} new-y={}", enc_json(&p.member_id), enc_json(&p.bar_id), p.new_y),
        En1992Mutation::ChangeBarPointZ(p) => format!("change-bar-point-z member-id={} bar-id={} new-z={}", enc_json(&p.member_id), enc_json(&p.bar_id), p.new_z),
        En1992Mutation::ChangeBarPointDiameter(p) => format!("change-bar-point-diameter member-id={} bar-id={} new-diameter={}", enc_json(&p.member_id), enc_json(&p.bar_id), p.new_diameter),
        En1992Mutation::ChangeTendonPointX(p) => format!("change-tendon-point-x member-id={} tendon-point-id={} new-x={}", enc_json(&p.member_id), enc_json(&p.tendon_point_id), p.new_x),
        En1992Mutation::ChangeTendonPointY(p) => format!("change-tendon-point-y member-id={} tendon-point-id={} new-y={}", enc_json(&p.member_id), enc_json(&p.tendon_point_id), p.new_y),
        En1992Mutation::ChangeTendonPointZ(p) => format!("change-tendon-point-z member-id={} tendon-point-id={} new-z={}", enc_json(&p.member_id), enc_json(&p.tendon_point_id), p.new_z),
        En1992Mutation::ChangeAnchorCracked(p) => format!("change-anchor-cracked anchor-id={} new-cracked={}", enc_json(&p.anchor_id), p.new_cracked),
        En1992Mutation::ChangeAnchorFUk(p) => format!("change-anchor-f-uk anchor-id={} new-f-uk={}", enc_json(&p.anchor_id), p.new_f_uk),
        En1992Mutation::ChangeAnchorFYk(p) => format!("change-anchor-f-yk anchor-id={} new-f-yk={}", enc_json(&p.anchor_id), p.new_f_yk),
        En1992Mutation::ChangeAnchorD(p) => format!("change-anchor-d anchor-id={} new-d={}", enc_json(&p.anchor_id), p.new_d),
        En1992Mutation::ChangeAnchorC1(p) => format!("change-anchor-c1 anchor-id={} new-c1={}", enc_json(&p.anchor_id), p.new_c1),
        En1992Mutation::ChangeAnchorFCk(p) => format!("change-anchor-f-ck anchor-id={} new-f-ck={}", enc_json(&p.anchor_id), p.new_f_ck),
        En1992Mutation::ChangeAnchorHeadDiameter(p) => format!("change-anchor-head-diameter anchor-id={} new-head-diameter={}", enc_json(&p.anchor_id), p.new_head_diameter),
        En1992Mutation::ChangeAnchorActionKind(p) => format!("change-anchor-action-kind anchor-id={} action-id={} new-kind={}", enc_json(&p.anchor_id), enc_json(&p.action_id), enc_json(&p.new_kind)),
        En1992Mutation::ChangeAnchorActionCategory(p) => format!("change-anchor-action-category anchor-id={} action-id={} new-category={}", enc_json(&p.anchor_id), enc_json(&p.action_id), enc_json(&p.new_category)),
        En1992Mutation::ChangeAnchorActionSource(p) => format!("change-anchor-action-source anchor-id={} action-id={} new-source={}", enc_json(&p.anchor_id), enc_json(&p.action_id), enc_json(&p.new_source)),
        En1992Mutation::ChangeAnchorActionGKLine(p) => format!("change-anchor-action-gk-line anchor-id={} action-id={} new-g-k-line={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_g_k_line),
        En1992Mutation::ChangeAnchorActionQKLine(p) => format!("change-anchor-action-qk-line anchor-id={} action-id={} new-q-k-line={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_q_k_line),
        En1992Mutation::ChangeAnchorActionPointForce(p) => format!("change-anchor-action-point-force anchor-id={} action-id={} new-point-force={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_point_force),
        En1992Mutation::ChangeAnchorActionMK(p) => format!("change-anchor-action-mk anchor-id={} action-id={} new-m-k={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_m_k),
        En1992Mutation::ChangeAnchorActionNK(p) => format!("change-anchor-action-nk anchor-id={} action-id={} new-n-k={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_n_k),
        En1992Mutation::ChangeAnchorActionVK(p) => format!("change-anchor-action-vk anchor-id={} action-id={} new-v-k={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_v_k),
        En1992Mutation::ChangeAnchorActionTK(p) => format!("change-anchor-action-tk anchor-id={} action-id={} new-t-k={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_t_k),
        En1992Mutation::ChangeAnchorActionVKPunch(p) => format!("change-anchor-action-vk-punch anchor-id={} action-id={} new-v-k-punch={}", enc_json(&p.anchor_id), enc_json(&p.action_id), p.new_v_k_punch),
        En1992Mutation::InsertConcreteGrade(p) => format!("insert-concrete-grade index={} concrete-grade={}", p.index, enc_json(&p.concrete_grade)),
        En1992Mutation::RemoveConcreteGrade(p) => format!("remove-concrete-grade index={}", p.index),
        En1992Mutation::InsertReinforcementGrade(p) => format!("insert-reinforcement-grade index={} reinforcement-grade={}", p.index, enc_json(&p.reinforcement_grade)),
        En1992Mutation::RemoveReinforcementGrade(p) => format!("remove-reinforcement-grade index={}", p.index),
        En1992Mutation::InsertPrestressSteel(p) => format!("insert-prestress-steel index={} prestress-steel={}", p.index, enc_json(&p.prestress_steel)),
        En1992Mutation::RemovePrestressSteel(p) => format!("remove-prestress-steel index={}", p.index),
        En1992Mutation::InsertBarLayer(p) => format!("insert-bar-layer member-id={} index={} bar-layer={}", enc_json(&p.member_id), p.index, enc_json(&p.bar_layer)),
        En1992Mutation::RemoveBarLayer(p) => format!("remove-bar-layer member-id={} index={}", enc_json(&p.member_id), p.index),
        En1992Mutation::InsertAction(p) => format!("insert-action member-id={} index={} action={}", enc_json(&p.member_id), p.index, enc_json(&p.action)),
        En1992Mutation::RemoveAction(p) => format!("remove-action member-id={} index={}", enc_json(&p.member_id), p.index),
        En1992Mutation::InsertSectionPoint(p) => format!("insert-section-point member-id={} index={} section-point={}", enc_json(&p.member_id), p.index, enc_json(&p.section_point)),
        En1992Mutation::RemoveSectionPoint(p) => format!("remove-section-point member-id={} index={}", enc_json(&p.member_id), p.index),
        En1992Mutation::InsertBarPoint(p) => format!("insert-bar-point member-id={} index={} bar-point={}", enc_json(&p.member_id), p.index, enc_json(&p.bar_point)),
        En1992Mutation::RemoveBarPoint(p) => format!("remove-bar-point member-id={} index={}", enc_json(&p.member_id), p.index),
        En1992Mutation::InsertTendonPoint(p) => format!("insert-tendon-point member-id={} index={} tendon-point={}", enc_json(&p.member_id), p.index, enc_json(&p.tendon_point)),
        En1992Mutation::RemoveTendonPoint(p) => format!("remove-tendon-point member-id={} index={}", enc_json(&p.member_id), p.index),
        En1992Mutation::InsertAnchorAction(p) => format!("insert-anchor-action anchor-id={} index={} anchor-action={}", enc_json(&p.anchor_id), p.index, enc_json(&p.anchor_action)),
        En1992Mutation::RemoveAnchorAction(p) => format!("remove-anchor-action anchor-id={} index={}", enc_json(&p.anchor_id), p.index),
        En1992Mutation::ChangePlacementX(p) => format!("change-placement-x id={} new-x={}", enc_json(&p.id), p.new_x),
        En1992Mutation::ChangePlacementY(p) => format!("change-placement-y id={} new-y={}", enc_json(&p.id), p.new_y),
        En1992Mutation::ChangePlacementZ(p) => format!("change-placement-z id={} new-z={}", enc_json(&p.id), p.new_z),
        En1992Mutation::ChangePlacementYaw(p) => format!("change-placement-yaw id={} new-yaw={}", enc_json(&p.id), p.new_yaw),
        En1992Mutation::InsertPlacement(p) => format!("insert-placement index={} placement={}", p.index, enc_json(&p.placement)),
        En1992Mutation::RemovePlacement(p) => format!("remove-placement index={}", p.index),
        En1992Mutation::InsertCase(p) => format!("insert-case index={} case={}", p.index, enc_token(&p.case)),
        En1992Mutation::RemoveCase(p) => format!("remove-case id={}", enc_token(&p.id)),
        En1992Mutation::ChangeCaseNumber(p) => format!("change-case-number case-id={} key={} value={}", enc_token(&p.case_id), enc_token(&p.key), enc_token(&p.value)),
        En1992Mutation::ChangeCaseText(p) => format!("change-case-text case-id={} key={} value={}", enc_token(&p.case_id), enc_token(&p.key), enc_token(&p.value)),
        En1992Mutation::ChangeCaseLabelEn(p) => format!("change-case-label-en case-id={} new-value={}", enc_token(&p.case_id), enc_token(&p.new_value)),
        En1992Mutation::ChangeCaseLabelDe(p) => format!("change-case-label-de case-id={} new-value={}", enc_token(&p.case_id), enc_token(&p.new_value)),
        En1992Mutation::ChangeCaseEntryId(p) => format!("change-case-entry-id case-id={} new-value={}", enc_token(&p.case_id), enc_token(&p.new_value)),
        En1992Mutation::ChangeBarPointLength(p) => format!("change-bar-point-length member-id={} bar-id={} new-length={}", enc_json(&p.member_id), enc_json(&p.bar_id), p.new_length),
        En1992Mutation::ChangeBarPointBend(p) => format!("change-bar-point-bend member-id={} bar-id={} new-bend={}", enc_json(&p.member_id), enc_json(&p.bar_id), enc_json(&p.new_bend)),
        En1992Mutation::ChangeMemberPunchingBetaMethod(p) => format!("change-member-punching-beta-method member-id={} new-punching-beta-method={}", enc_json(&p.member_id), enc_json(&p.new_punching_beta_method)),
        En1992Mutation::ChangeMemberPunchingMomentC1(p) => format!("change-member-punching-moment-c1 member-id={} new-punching-moment-c1={}", enc_json(&p.member_id), p.new_punching_moment_c1),
        En1992Mutation::ChangeMemberPunchingMomentC2(p) => format!("change-member-punching-moment-c2 member-id={} new-punching-moment-c2={}", enc_json(&p.member_id), p.new_punching_moment_c2),
        En1992Mutation::ChangeOpeningX(p) => format!("change-opening-x member-id={} opening-id={} new-x={}", enc_json(&p.member_id), enc_json(&p.opening_id), p.new_x),
        En1992Mutation::ChangeOpeningY(p) => format!("change-opening-y member-id={} opening-id={} new-y={}", enc_json(&p.member_id), enc_json(&p.opening_id), p.new_y),
        En1992Mutation::ChangeOpeningWidth(p) => format!("change-opening-width member-id={} opening-id={} new-width={}", enc_json(&p.member_id), enc_json(&p.opening_id), p.new_width),
        En1992Mutation::ChangeOpeningDepth(p) => format!("change-opening-depth member-id={} opening-id={} new-depth={}", enc_json(&p.member_id), enc_json(&p.opening_id), p.new_depth),
        En1992Mutation::InsertOpening(p) => format!("insert-opening member-id={} index={} opening={}", enc_json(&p.member_id), p.index, enc_json(&p.opening)),
        En1992Mutation::RemoveOpening(p) => format!("remove-opening member-id={} index={}", enc_json(&p.member_id), p.index),
        En1992Mutation::ChangeStudRowAngle(p) => format!("change-stud-row-angle member-id={} stud-row-id={} new-angle={}", enc_json(&p.member_id), enc_json(&p.stud_row_id), p.new_angle),
        En1992Mutation::ChangeStudRowCount(p) => format!("change-stud-row-count member-id={} stud-row-id={} new-count={}", enc_json(&p.member_id), enc_json(&p.stud_row_id), p.new_count),
        En1992Mutation::ChangeStudRowSpacingFirst(p) => format!("change-stud-row-spacing-first member-id={} stud-row-id={} new-spacing-first={}", enc_json(&p.member_id), enc_json(&p.stud_row_id), p.new_spacing_first),
        En1992Mutation::ChangeStudRowSpacingRadial(p) => format!("change-stud-row-spacing-radial member-id={} stud-row-id={} new-spacing-radial={}", enc_json(&p.member_id), enc_json(&p.stud_row_id), p.new_spacing_radial),
        En1992Mutation::ChangeStudRowDiameter(p) => format!("change-stud-row-diameter member-id={} stud-row-id={} new-diameter={}", enc_json(&p.member_id), enc_json(&p.stud_row_id), p.new_diameter),
        En1992Mutation::InsertStudRow(p) => format!("insert-stud-row member-id={} index={} stud-row={}", enc_json(&p.member_id), p.index, enc_json(&p.stud_row)),
        En1992Mutation::RemoveStudRow(p) => format!("remove-stud-row member-id={} index={}", enc_json(&p.member_id), p.index),
    }
}

/// 📖️ Parse a single op line — JSON payload after first space for complex kinds.
pub fn parse_op(line: &str) -> Result<En1992Mutation, String> {
    let line = line.trim();
    let (kind, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::HashMap<String, String> = rest
        .split_whitespace()
        .filter_map(|tok| tok.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())))
        .collect();
    let get = |k: &str| args.get(k).cloned().ok_or_else(|| format!("missing {k}"));
    match kind {
        "change-annex" => Ok(En1992Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: dec_json(&get("new-annex")?)? })),
        "change-reliability-class" => Ok(En1992Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: dec_json(&get("new-reliability-class")?)? })),
        "change-title" => Ok(En1992Mutation::ChangeTitle(change_title::ChangeTitle { new_title: dec_json(&get("new-title")?)? })),
        "change-design-working-life" => Ok(En1992Mutation::ChangeDesignWorkingLife(change_design_working_life::ChangeDesignWorkingLife { new_years: get("new-years")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-delta-c-dev" => Ok(En1992Mutation::ChangeDeltaCDev(change_delta_c_dev::ChangeDeltaCDev { new_delta_c_dev: get("new-delta-c-dev")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-cement-type" => Ok(En1992Mutation::ChangeCementType(change_cement_type::ChangeCementType { new_cement_type: dec_json(&get("new-cement-type")?)? })),
        "change-concrete-f-ck" => Ok(En1992Mutation::ChangeConcreteFCk(change_concrete_f_ck::ChangeConcreteFCk { grade_id: dec_json(&get("grade-id")?)?, new_f_ck: get("new-f-ck")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-reinforcement-f-yk" => Ok(En1992Mutation::ChangeReinforcementFYk(change_reinforcement_f_yk::ChangeReinforcementFYk { grade_id: dec_json(&get("grade-id")?)?, new_f_yk: get("new-f-yk")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "remove-member" => Ok(En1992Mutation::RemoveMember(remove_member::RemoveMember { member_id: dec_json(&get("member-id")?)? })),
        "change-member-order" => Ok(En1992Mutation::ChangeMemberOrder(change_member_order::ChangeMemberOrder { from_index: get("from-index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, to_index: get("to-index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-member-width" => Ok(En1992Mutation::ChangeMemberWidth(change_member_width::ChangeMemberWidth { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-height" => Ok(En1992Mutation::ChangeMemberHeight(change_member_height::ChangeMemberHeight { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-effective-depth" => Ok(En1992Mutation::ChangeMemberEffectiveDepth(change_member_effective_depth::ChangeMemberEffectiveDepth { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-cover" => Ok(En1992Mutation::ChangeMemberCover(change_member_cover::ChangeMemberCover { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-bridge-delta-sigma-s" => Ok(En1992Mutation::ChangeMemberBridgeDeltaSigmaS(change_member_bridge_delta_sigma_s::ChangeMemberBridgeDeltaSigmaS { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-bridge-sigma-c" => Ok(En1992Mutation::ChangeMemberBridgeSigmaC(change_member_bridge_sigma_c::ChangeMemberBridgeSigmaC { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-liquid-sr-max" => Ok(En1992Mutation::ChangeMemberLiquidSRMax(change_member_liquid_s_r_max::ChangeMemberLiquidSRMax { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-liquid-f-ct-eff" => Ok(En1992Mutation::ChangeMemberLiquidFCtEff(change_member_liquid_f_ct_eff::ChangeMemberLiquidFCtEff { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-liquid-rho-p-eff" => Ok(En1992Mutation::ChangeMemberLiquidRhoPEff(change_member_liquid_rho_p_eff::ChangeMemberLiquidRhoPEff { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-liquid-sigma-s" => Ok(En1992Mutation::ChangeMemberLiquidSigmaS(change_member_liquid_sigma_s::ChangeMemberLiquidSigmaS { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-hd-over-h" => Ok(En1992Mutation::ChangeMemberHdOverH(change_member_hd_over_h::ChangeMemberHdOverH { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-udl" => Ok(En1992Mutation::ChangeMemberUdl(change_member_udl::ChangeMemberUdl { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-buckling-length" => Ok(En1992Mutation::ChangeMemberBucklingLength(change_member_buckling_length::ChangeMemberBucklingLength { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-relative-humidity" => Ok(En1992Mutation::ChangeMemberRelativeHumidity(change_member_relative_humidity::ChangeMemberRelativeHumidity { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-bearing-width" => Ok(En1992Mutation::ChangeMemberBearingWidth(change_member_bearing_width::ChangeMemberBearingWidth { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-tightness" => Ok(En1992Mutation::ChangeMemberTightness(change_member_tightness::ChangeMemberTightness { member_id: dec_json(&get("member-id")?)?, new_tightness: dec_json(&get("new-tightness")?)? })),
        "clear-member-tightness" => Ok(En1992Mutation::ClearMemberTightness(clear_member_tightness::ClearMemberTightness { member_id: dec_json(&get("member-id")?)? })),
        "change-member-exposure" => Ok(En1992Mutation::ChangeMemberExposure(change_member_exposure::ChangeMemberExposure { member_id: dec_json(&get("member-id")?)?, new_exposure: dec_json(&get("new-exposure")?)? })),
        "change-member-span" => Ok(En1992Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: dec_json(&get("member-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-stirrup-spacing" => Ok(En1992Mutation::ChangeMemberStirrupSpacing(change_member_stirrup_spacing::ChangeMemberStirrupSpacing { member_id: dec_json(&get("member-id")?)?, new_spacing: get("new-spacing")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-axis-distance" => Ok(En1992Mutation::ChangeMemberAxisDistance(change_member_axis_distance::ChangeMemberAxisDistance { member_id: dec_json(&get("member-id")?)?, new_axis_distance: get("new-axis-distance")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-fire-rating" => Ok(En1992Mutation::ChangeMemberFireRating(change_member_fire_rating::ChangeMemberFireRating { member_id: dec_json(&get("member-id")?)?, new_rating: dec_json(&get("new-rating")?)? })),
        "change-bar-layer-count" => Ok(En1992Mutation::ChangeBarLayerCount(change_bar_layer_count::ChangeBarLayerCount { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_count: get("new-count")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-bar-layer-diameter" => Ok(En1992Mutation::ChangeBarLayerDiameter(change_bar_layer_diameter::ChangeBarLayerDiameter { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_diameter: get("new-diameter")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-mk" => Ok(En1992Mutation::ChangeActionMk(change_action_mk::ChangeActionMk { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-nk" => Ok(En1992Mutation::ChangeActionNk(change_action_nk::ChangeActionNk { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-vk" => Ok(En1992Mutation::ChangeActionVk(change_action_vk::ChangeActionVk { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "remove-anchor" => Ok(En1992Mutation::RemoveAnchor(remove_anchor::RemoveAnchor { anchor_id: dec_json(&get("anchor-id")?)? })),
        "change-anchor-h-ef" => Ok(En1992Mutation::ChangeAnchorHEf(change_anchor_h_ef::ChangeAnchorHEf { anchor_id: dec_json(&get("anchor-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-as" => Ok(En1992Mutation::ChangeAnchorAs(change_anchor_a_s::ChangeAnchorAs { anchor_id: dec_json(&get("anchor-id")?)?, new_value: get("new-value")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "insert-member" => Ok(En1992Mutation::InsertMember(insert_member::InsertMember { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, member: dec_json(&get("member")?)? })),
        "insert-anchor" => Ok(En1992Mutation::InsertAnchor(insert_anchor::InsertAnchor { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, anchor: dec_json(&get("anchor")?)? })),
        "change-concrete-name" => Ok(En1992Mutation::ChangeConcreteName(change_concrete_name::ChangeConcreteName { grade_id: dec_json(&get("grade-id")?)?, new_name: dec_json(&get("new-name")?)? })),
        "change-reinforcement-name" => Ok(En1992Mutation::ChangeReinforcementName(change_reinforcement_name::ChangeReinforcementName { grade_id: dec_json(&get("grade-id")?)?, new_name: dec_json(&get("new-name")?)? })),
        "change-reinforcement-es" => Ok(En1992Mutation::ChangeReinforcementES(change_reinforcement_e_s::ChangeReinforcementES { grade_id: dec_json(&get("grade-id")?)?, new_e_s: get("new-e-s")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-reinforcement-ductility" => Ok(En1992Mutation::ChangeReinforcementDuctility(change_reinforcement_ductility::ChangeReinforcementDuctility { grade_id: dec_json(&get("grade-id")?)?, new_ductility: dec_json(&get("new-ductility")?)? })),
        "change-reinforcement-k" => Ok(En1992Mutation::ChangeReinforcementK(change_reinforcement_k::ChangeReinforcementK { grade_id: dec_json(&get("grade-id")?)?, new_k: get("new-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-reinforcement-eps-uk" => Ok(En1992Mutation::ChangeReinforcementEpsUk(change_reinforcement_eps_uk::ChangeReinforcementEpsUk { grade_id: dec_json(&get("grade-id")?)?, new_eps_uk: get("new-eps-uk")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-prestress-steel-name" => Ok(En1992Mutation::ChangePrestressSteelName(change_prestress_steel_name::ChangePrestressSteelName { grade_id: dec_json(&get("grade-id")?)?, new_name: dec_json(&get("new-name")?)? })),
        "change-prestress-steel-f-pk" => Ok(En1992Mutation::ChangePrestressSteelFPk(change_prestress_steel_f_pk::ChangePrestressSteelFPk { grade_id: dec_json(&get("grade-id")?)?, new_f_pk: get("new-f-pk")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-prestress-steel-fp01k" => Ok(En1992Mutation::ChangePrestressSteelFP01k(change_prestress_steel_f_p01k::ChangePrestressSteelFP01k { grade_id: dec_json(&get("grade-id")?)?, new_f_p01k: get("new-f-p01k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-label-en" => Ok(En1992Mutation::ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn { member_id: dec_json(&get("member-id")?)?, new_label_en: dec_json(&get("new-label-en")?)? })),
        "change-member-label-de" => Ok(En1992Mutation::ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe { member_id: dec_json(&get("member-id")?)?, new_label_de: dec_json(&get("new-label-de")?)? })),
        "change-member-kind" => Ok(En1992Mutation::ChangeMemberKind(change_member_kind::ChangeMemberKind { member_id: dec_json(&get("member-id")?)?, new_kind: dec_json(&get("new-kind")?)? })),
        "change-member-concrete-grade-id" => Ok(En1992Mutation::ChangeMemberConcreteGradeId(change_member_concrete_grade_id::ChangeMemberConcreteGradeId { member_id: dec_json(&get("member-id")?)?, new_concrete_grade_id: dec_json(&get("new-concrete-grade-id")?)? })),
        "change-member-rebar-grade-id" => Ok(En1992Mutation::ChangeMemberRebarGradeId(change_member_rebar_grade_id::ChangeMemberRebarGradeId { member_id: dec_json(&get("member-id")?)?, new_reinforcement_grade_id: dec_json(&get("new-reinforcement-grade-id")?)? })),
        "change-member-prestress-steel-id" => Ok(En1992Mutation::ChangeMemberPrestressSteelId(change_member_prestress_steel_id::ChangeMemberPrestressSteelId { member_id: dec_json(&get("member-id")?)?, new_prestress_steel_id: dec_json(&get("new-prestress-steel-id")?)? })),
        "change-member-support" => Ok(En1992Mutation::ChangeMemberSupport(change_member_support::ChangeMemberSupport { member_id: dec_json(&get("member-id")?)?, new_support: dec_json(&get("new-support")?)? })),
        "change-member-stirrups-diameter" => Ok(En1992Mutation::ChangeMemberStirrupsDiameter(change_member_stirrups_diameter::ChangeMemberStirrupsDiameter { member_id: dec_json(&get("member-id")?)?, new_stirrups_diameter: get("new-stirrups-diameter")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-stirrups-legs" => Ok(En1992Mutation::ChangeMemberStirrupsLegs(change_member_stirrups_legs::ChangeMemberStirrupsLegs { member_id: dec_json(&get("member-id")?)?, new_stirrups_legs: get("new-stirrups-legs")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-member-punch-column-width" => Ok(En1992Mutation::ChangeMemberPunchColumnWidth(change_member_punch_column_width::ChangeMemberPunchColumnWidth { member_id: dec_json(&get("member-id")?)?, new_punching_column_width: get("new-punching-column-width")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-punch-column-depth" => Ok(En1992Mutation::ChangeMemberPunchColumnDepth(change_member_punch_column_depth::ChangeMemberPunchColumnDepth { member_id: dec_json(&get("member-id")?)?, new_punching_column_depth: get("new-punching-column-depth")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-punch-col-position" => Ok(En1992Mutation::ChangeMemberPunchColPosition(change_member_punch_col_position::ChangeMemberPunchColPosition { member_id: dec_json(&get("member-id")?)?, new_punching_column_position: dec_json(&get("new-punching-column-position")?)? })),
        "change-member-punching-asw" => Ok(En1992Mutation::ChangeMemberPunchingAsw(change_member_punching_asw::ChangeMemberPunchingAsw { member_id: dec_json(&get("member-id")?)?, new_punching_asw: get("new-punching-asw")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-prestress-force" => Ok(En1992Mutation::ChangeMemberPrestressForce(change_member_prestress_force::ChangeMemberPrestressForce { member_id: dec_json(&get("member-id")?)?, new_prestress_force: get("new-prestress-force")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-prestress-area" => Ok(En1992Mutation::ChangeMemberPrestressArea(change_member_prestress_area::ChangeMemberPrestressArea { member_id: dec_json(&get("member-id")?)?, new_prestress_area: get("new-prestress-area")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-prestress-ecc" => Ok(En1992Mutation::ChangeMemberPrestressEcc(change_member_prestress_ecc::ChangeMemberPrestressEcc { member_id: dec_json(&get("member-id")?)?, new_prestress_eccentricity: get("new-prestress-eccentricity")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-prestress-loss-ratio" => Ok(En1992Mutation::ChangeMemberPrestressLossRatio(change_member_prestress_loss_ratio::ChangeMemberPrestressLossRatio { member_id: dec_json(&get("member-id")?)?, new_prestress_loss_ratio: get("new-prestress-loss-ratio")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-fire-slab-system" => Ok(En1992Mutation::ChangeMemberFireSlabSystem(change_member_fire_slab_system::ChangeMemberFireSlabSystem { member_id: dec_json(&get("member-id")?)?, new_fire_slab_system: dec_json(&get("new-fire-slab-system")?)? })),
        "change-member-use-fem" => Ok(En1992Mutation::ChangeMemberUseFem(change_member_use_fem::ChangeMemberUseFem { member_id: dec_json(&get("member-id")?)?, new_use_fem: get("new-use-fem")?.parse().map_err(|e: std::num::ParseBoolError| e.to_string())? })),
        "change-member-deflection-sensitive" => Ok(En1992Mutation::ChangeMemberDeflectionSensitive(change_member_deflection_sensitive::ChangeMemberDeflectionSensitive { member_id: dec_json(&get("member-id")?)?, new_deflection_sensitive: get("new-deflection-sensitive")?.parse().map_err(|e: std::num::ParseBoolError| e.to_string())? })),
        "change-bar-layer-position" => Ok(En1992Mutation::ChangeBarLayerPosition(change_bar_layer_position::ChangeBarLayerPosition { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_position: dec_json(&get("new-position")?)? })),
        "change-bar-layer-anchorage-length" => Ok(En1992Mutation::ChangeBarLayerAnchorageLength(change_bar_layer_anchorage_length::ChangeBarLayerAnchorageLength { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_anchorage_length: get("new-anchorage-length")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-layer-lap-length" => Ok(En1992Mutation::ChangeBarLayerLapLength(change_bar_layer_lap_length::ChangeBarLayerLapLength { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_lap_length: get("new-lap-length")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-layer-bond-condition" => Ok(En1992Mutation::ChangeBarLayerBondCondition(change_bar_layer_bond_condition::ChangeBarLayerBondCondition { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_bond_condition: dec_json(&get("new-bond-condition")?)? })),
        "change-bar-layer-aggregate-size" => Ok(En1992Mutation::ChangeBarLayerAggregateSize(change_bar_layer_aggregate_size::ChangeBarLayerAggregateSize { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_aggregate_size: get("new-aggregate-size")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-layer-lap-share" => Ok(En1992Mutation::ChangeBarLayerLapShare(change_bar_layer_lap_share::ChangeBarLayerLapShare { member_id: dec_json(&get("member-id")?)?, layer_id: dec_json(&get("layer-id")?)?, new_lap_share: get("new-lap-share")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-kind" => Ok(En1992Mutation::ChangeActionKind(change_action_kind::ChangeActionKind { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_kind: dec_json(&get("new-kind")?)? })),
        "change-action-category" => Ok(En1992Mutation::ChangeActionCategory(change_action_category::ChangeActionCategory { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_category: dec_json(&get("new-category")?)? })),
        "change-action-source" => Ok(En1992Mutation::ChangeActionSource(change_action_source::ChangeActionSource { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_source: dec_json(&get("new-source")?)? })),
        "change-action-gk-line" => Ok(En1992Mutation::ChangeActionGKLine(change_action_g_k_line::ChangeActionGKLine { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_g_k_line: get("new-g-k-line")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-qk-line" => Ok(En1992Mutation::ChangeActionQKLine(change_action_q_k_line::ChangeActionQKLine { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_q_k_line: get("new-q-k-line")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-point-force" => Ok(En1992Mutation::ChangeActionPointForce(change_action_point_force::ChangeActionPointForce { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_point_force: get("new-point-force")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-tk" => Ok(En1992Mutation::ChangeActionTK(change_action_t_k::ChangeActionTK { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_t_k: get("new-t-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-action-vk-punch" => Ok(En1992Mutation::ChangeActionVKPunch(change_action_v_k_punch::ChangeActionVKPunch { member_id: dec_json(&get("member-id")?)?, action_id: dec_json(&get("action-id")?)?, new_v_k_punch: get("new-v-k-punch")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-section-point-y" => Ok(En1992Mutation::ChangeSectionPointY(change_section_point_y::ChangeSectionPointY { member_id: dec_json(&get("member-id")?)?, section_point_id: dec_json(&get("section-point-id")?)?, new_y: get("new-y")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-section-point-z" => Ok(En1992Mutation::ChangeSectionPointZ(change_section_point_z::ChangeSectionPointZ { member_id: dec_json(&get("member-id")?)?, section_point_id: dec_json(&get("section-point-id")?)?, new_z: get("new-z")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-point-layer-id" => Ok(En1992Mutation::ChangeBarPointLayerId(change_bar_point_layer_id::ChangeBarPointLayerId { member_id: dec_json(&get("member-id")?)?, bar_id: dec_json(&get("bar-id")?)?, new_layer_id: dec_json(&get("new-layer-id")?)? })),
        "change-bar-point-y" => Ok(En1992Mutation::ChangeBarPointY(change_bar_point_y::ChangeBarPointY { member_id: dec_json(&get("member-id")?)?, bar_id: dec_json(&get("bar-id")?)?, new_y: get("new-y")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-point-z" => Ok(En1992Mutation::ChangeBarPointZ(change_bar_point_z::ChangeBarPointZ { member_id: dec_json(&get("member-id")?)?, bar_id: dec_json(&get("bar-id")?)?, new_z: get("new-z")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-point-diameter" => Ok(En1992Mutation::ChangeBarPointDiameter(change_bar_point_diameter::ChangeBarPointDiameter { member_id: dec_json(&get("member-id")?)?, bar_id: dec_json(&get("bar-id")?)?, new_diameter: get("new-diameter")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-tendon-point-x" => Ok(En1992Mutation::ChangeTendonPointX(change_tendon_point_x::ChangeTendonPointX { member_id: dec_json(&get("member-id")?)?, tendon_point_id: dec_json(&get("tendon-point-id")?)?, new_x: get("new-x")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-tendon-point-y" => Ok(En1992Mutation::ChangeTendonPointY(change_tendon_point_y::ChangeTendonPointY { member_id: dec_json(&get("member-id")?)?, tendon_point_id: dec_json(&get("tendon-point-id")?)?, new_y: get("new-y")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-tendon-point-z" => Ok(En1992Mutation::ChangeTendonPointZ(change_tendon_point_z::ChangeTendonPointZ { member_id: dec_json(&get("member-id")?)?, tendon_point_id: dec_json(&get("tendon-point-id")?)?, new_z: get("new-z")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-cracked" => Ok(En1992Mutation::ChangeAnchorCracked(change_anchor_cracked::ChangeAnchorCracked { anchor_id: dec_json(&get("anchor-id")?)?, new_cracked: get("new-cracked")?.parse().map_err(|e: std::num::ParseBoolError| e.to_string())? })),
        "change-anchor-f-uk" => Ok(En1992Mutation::ChangeAnchorFUk(change_anchor_f_uk::ChangeAnchorFUk { anchor_id: dec_json(&get("anchor-id")?)?, new_f_uk: get("new-f-uk")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-f-yk" => Ok(En1992Mutation::ChangeAnchorFYk(change_anchor_f_yk::ChangeAnchorFYk { anchor_id: dec_json(&get("anchor-id")?)?, new_f_yk: get("new-f-yk")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-d" => Ok(En1992Mutation::ChangeAnchorD(change_anchor_d::ChangeAnchorD { anchor_id: dec_json(&get("anchor-id")?)?, new_d: get("new-d")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-c1" => Ok(En1992Mutation::ChangeAnchorC1(change_anchor_c1::ChangeAnchorC1 { anchor_id: dec_json(&get("anchor-id")?)?, new_c1: get("new-c1")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-f-ck" => Ok(En1992Mutation::ChangeAnchorFCk(change_anchor_f_ck::ChangeAnchorFCk { anchor_id: dec_json(&get("anchor-id")?)?, new_f_ck: get("new-f-ck")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-head-diameter" => Ok(En1992Mutation::ChangeAnchorHeadDiameter(change_anchor_head_diameter::ChangeAnchorHeadDiameter { anchor_id: dec_json(&get("anchor-id")?)?, new_head_diameter: get("new-head-diameter")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-kind" => Ok(En1992Mutation::ChangeAnchorActionKind(change_anchor_action_kind::ChangeAnchorActionKind { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_kind: dec_json(&get("new-kind")?)? })),
        "change-anchor-action-category" => Ok(En1992Mutation::ChangeAnchorActionCategory(change_anchor_action_category::ChangeAnchorActionCategory { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_category: dec_json(&get("new-category")?)? })),
        "change-anchor-action-source" => Ok(En1992Mutation::ChangeAnchorActionSource(change_anchor_action_source::ChangeAnchorActionSource { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_source: dec_json(&get("new-source")?)? })),
        "change-anchor-action-gk-line" => Ok(En1992Mutation::ChangeAnchorActionGKLine(change_anchor_action_g_k_line::ChangeAnchorActionGKLine { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_g_k_line: get("new-g-k-line")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-qk-line" => Ok(En1992Mutation::ChangeAnchorActionQKLine(change_anchor_action_q_k_line::ChangeAnchorActionQKLine { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_q_k_line: get("new-q-k-line")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-point-force" => Ok(En1992Mutation::ChangeAnchorActionPointForce(change_anchor_action_point_force::ChangeAnchorActionPointForce { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_point_force: get("new-point-force")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-mk" => Ok(En1992Mutation::ChangeAnchorActionMK(change_anchor_action_m_k::ChangeAnchorActionMK { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_m_k: get("new-m-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-nk" => Ok(En1992Mutation::ChangeAnchorActionNK(change_anchor_action_n_k::ChangeAnchorActionNK { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_n_k: get("new-n-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-vk" => Ok(En1992Mutation::ChangeAnchorActionVK(change_anchor_action_v_k::ChangeAnchorActionVK { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_v_k: get("new-v-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-tk" => Ok(En1992Mutation::ChangeAnchorActionTK(change_anchor_action_t_k::ChangeAnchorActionTK { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_t_k: get("new-t-k")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-anchor-action-vk-punch" => Ok(En1992Mutation::ChangeAnchorActionVKPunch(change_anchor_action_v_k_punch::ChangeAnchorActionVKPunch { anchor_id: dec_json(&get("anchor-id")?)?, action_id: dec_json(&get("action-id")?)?, new_v_k_punch: get("new-v-k-punch")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "insert-concrete-grade" => Ok(En1992Mutation::InsertConcreteGrade(insert_concrete_grade::InsertConcreteGrade { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, concrete_grade: dec_json(&get("concrete-grade")?)? })),
        "remove-concrete-grade" => Ok(En1992Mutation::RemoveConcreteGrade(remove_concrete_grade::RemoveConcreteGrade { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-reinforcement-grade" => Ok(En1992Mutation::InsertReinforcementGrade(insert_reinforcement_grade::InsertReinforcementGrade { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, reinforcement_grade: dec_json(&get("reinforcement-grade")?)? })),
        "remove-reinforcement-grade" => Ok(En1992Mutation::RemoveReinforcementGrade(remove_reinforcement_grade::RemoveReinforcementGrade { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-prestress-steel" => Ok(En1992Mutation::InsertPrestressSteel(insert_prestress_steel::InsertPrestressSteel { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, prestress_steel: dec_json(&get("prestress-steel")?)? })),
        "remove-prestress-steel" => Ok(En1992Mutation::RemovePrestressSteel(remove_prestress_steel::RemovePrestressSteel { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-bar-layer" => Ok(En1992Mutation::InsertBarLayer(insert_bar_layer::InsertBarLayer { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, bar_layer: dec_json(&get("bar-layer")?)? })),
        "remove-bar-layer" => Ok(En1992Mutation::RemoveBarLayer(remove_bar_layer::RemoveBarLayer { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-action" => Ok(En1992Mutation::InsertAction(insert_action::InsertAction { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, action: dec_json(&get("action")?)? })),
        "remove-action" => Ok(En1992Mutation::RemoveAction(remove_action::RemoveAction { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-section-point" => Ok(En1992Mutation::InsertSectionPoint(insert_section_point::InsertSectionPoint { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, section_point: dec_json(&get("section-point")?)? })),
        "remove-section-point" => Ok(En1992Mutation::RemoveSectionPoint(remove_section_point::RemoveSectionPoint { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-bar-point" => Ok(En1992Mutation::InsertBarPoint(insert_bar_point::InsertBarPoint { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, bar_point: dec_json(&get("bar-point")?)? })),
        "remove-bar-point" => Ok(En1992Mutation::RemoveBarPoint(remove_bar_point::RemoveBarPoint { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-tendon-point" => Ok(En1992Mutation::InsertTendonPoint(insert_tendon_point::InsertTendonPoint { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, tendon_point: dec_json(&get("tendon-point")?)? })),
        "remove-tendon-point" => Ok(En1992Mutation::RemoveTendonPoint(remove_tendon_point::RemoveTendonPoint { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-anchor-action" => Ok(En1992Mutation::InsertAnchorAction(insert_anchor_action::InsertAnchorAction { anchor_id: dec_json(&get("anchor-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, anchor_action: dec_json(&get("anchor-action")?)? })),
        "remove-anchor-action" => Ok(En1992Mutation::RemoveAnchorAction(remove_anchor_action::RemoveAnchorAction { anchor_id: dec_json(&get("anchor-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-placement-x" => Ok(En1992Mutation::ChangePlacementX(change_placement_x::ChangePlacementX { id: dec_json(&get("id")?)?, new_x: get("new-x")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-placement-y" => Ok(En1992Mutation::ChangePlacementY(change_placement_y::ChangePlacementY { id: dec_json(&get("id")?)?, new_y: get("new-y")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-placement-z" => Ok(En1992Mutation::ChangePlacementZ(change_placement_z::ChangePlacementZ { id: dec_json(&get("id")?)?, new_z: get("new-z")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-placement-yaw" => Ok(En1992Mutation::ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw { id: dec_json(&get("id")?)?, new_yaw: get("new-yaw")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "insert-placement" => Ok(En1992Mutation::InsertPlacement(insert_placement::InsertPlacement { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, placement: dec_json(&get("placement")?)? })),
        "remove-placement" => Ok(En1992Mutation::RemovePlacement(remove_placement::RemovePlacement { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "insert-case" => Ok(En1992Mutation::InsertCase(insert_case::InsertCase { index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, case: dec_json(&get("case")?)? })),
        "remove-case" => Ok(En1992Mutation::RemoveCase(remove_case::RemoveCase { id: dec_json(&get("id")?)? })),
        "change-case-number" => Ok(En1992Mutation::ChangeCaseNumber(change_case_number::ChangeCaseNumber { case_id: dec_json(&get("case-id")?)?, key: dec_json(&get("key")?)?, value: dec_json(&get("value")?)? })),
        "change-case-text" => Ok(En1992Mutation::ChangeCaseText(change_case_text::ChangeCaseText { case_id: dec_json(&get("case-id")?)?, key: dec_json(&get("key")?)?, value: dec_json(&get("value")?)? })),
        "change-case-label-en" => Ok(En1992Mutation::ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn { case_id: dec_json(&get("case-id")?)?, new_value: dec_json(&get("new-value")?)? })),
        "change-case-label-de" => Ok(En1992Mutation::ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe { case_id: dec_json(&get("case-id")?)?, new_value: dec_json(&get("new-value")?)? })),
        "change-case-entry-id" => Ok(En1992Mutation::ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId { case_id: dec_json(&get("case-id")?)?, new_value: dec_json(&get("new-value")?)? })),
        "change-bar-point-length" => Ok(En1992Mutation::ChangeBarPointLength(change_bar_point_length::ChangeBarPointLength { member_id: dec_json(&get("member-id")?)?, bar_id: dec_json(&get("bar-id")?)?, new_length: get("new-length")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-bar-point-bend" => Ok(En1992Mutation::ChangeBarPointBend(change_bar_point_bend::ChangeBarPointBend { member_id: dec_json(&get("member-id")?)?, bar_id: dec_json(&get("bar-id")?)?, new_bend: dec_json(&get("new-bend")?)? })),
        "change-member-punching-beta-method" => Ok(En1992Mutation::ChangeMemberPunchingBetaMethod(change_member_punching_beta_method::ChangeMemberPunchingBetaMethod { member_id: dec_json(&get("member-id")?)?, new_punching_beta_method: dec_json(&get("new-punching-beta-method")?)? })),
        "change-member-punching-moment-c1" => Ok(En1992Mutation::ChangeMemberPunchingMomentC1(change_member_punching_moment_c1::ChangeMemberPunchingMomentC1 { member_id: dec_json(&get("member-id")?)?, new_punching_moment_c1: get("new-punching-moment-c1")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-member-punching-moment-c2" => Ok(En1992Mutation::ChangeMemberPunchingMomentC2(change_member_punching_moment_c2::ChangeMemberPunchingMomentC2 { member_id: dec_json(&get("member-id")?)?, new_punching_moment_c2: get("new-punching-moment-c2")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-opening-x" => Ok(En1992Mutation::ChangeOpeningX(change_opening_x::ChangeOpeningX { member_id: dec_json(&get("member-id")?)?, opening_id: dec_json(&get("opening-id")?)?, new_x: get("new-x")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-opening-y" => Ok(En1992Mutation::ChangeOpeningY(change_opening_y::ChangeOpeningY { member_id: dec_json(&get("member-id")?)?, opening_id: dec_json(&get("opening-id")?)?, new_y: get("new-y")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-opening-width" => Ok(En1992Mutation::ChangeOpeningWidth(change_opening_width::ChangeOpeningWidth { member_id: dec_json(&get("member-id")?)?, opening_id: dec_json(&get("opening-id")?)?, new_width: get("new-width")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-opening-depth" => Ok(En1992Mutation::ChangeOpeningDepth(change_opening_depth::ChangeOpeningDepth { member_id: dec_json(&get("member-id")?)?, opening_id: dec_json(&get("opening-id")?)?, new_depth: get("new-depth")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "insert-opening" => Ok(En1992Mutation::InsertOpening(insert_opening::InsertOpening { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, opening: dec_json(&get("opening")?)? })),
        "remove-opening" => Ok(En1992Mutation::RemoveOpening(remove_opening::RemoveOpening { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-stud-row-angle" => Ok(En1992Mutation::ChangeStudRowAngle(change_stud_row_angle::ChangeStudRowAngle { member_id: dec_json(&get("member-id")?)?, stud_row_id: dec_json(&get("stud-row-id")?)?, new_angle: get("new-angle")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-stud-row-count" => Ok(En1992Mutation::ChangeStudRowCount(change_stud_row_count::ChangeStudRowCount { member_id: dec_json(&get("member-id")?)?, stud_row_id: dec_json(&get("stud-row-id")?)?, new_count: get("new-count")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "change-stud-row-spacing-first" => Ok(En1992Mutation::ChangeStudRowSpacingFirst(change_stud_row_spacing_first::ChangeStudRowSpacingFirst { member_id: dec_json(&get("member-id")?)?, stud_row_id: dec_json(&get("stud-row-id")?)?, new_spacing_first: get("new-spacing-first")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-stud-row-spacing-radial" => Ok(En1992Mutation::ChangeStudRowSpacingRadial(change_stud_row_spacing_radial::ChangeStudRowSpacingRadial { member_id: dec_json(&get("member-id")?)?, stud_row_id: dec_json(&get("stud-row-id")?)?, new_spacing_radial: get("new-spacing-radial")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "change-stud-row-diameter" => Ok(En1992Mutation::ChangeStudRowDiameter(change_stud_row_diameter::ChangeStudRowDiameter { member_id: dec_json(&get("member-id")?)?, stud_row_id: dec_json(&get("stud-row-id")?)?, new_diameter: get("new-diameter")?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())? })),
        "insert-stud-row" => Ok(En1992Mutation::InsertStudRow(insert_stud_row::InsertStudRow { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, stud_row: dec_json(&get("stud-row")?)? })),
        "remove-stud-row" => Ok(En1992Mutation::RemoveStudRow(remove_stud_row::RemoveStudRow { member_id: dec_json(&get("member-id")?)?, index: get("index")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        other => Err(format!("unknown mutation kind {other}")),
    }
}

impl protocol::OpText for En1992Mutation {
    fn print_op(&self) -> String { print_op(self) }
    fn parse_op(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_op(text).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

#[allow(unused_imports)]
mod mutations_codec {
use crate::text::*;
use crate::*;
use semio_s_artifact_norm_en1992_diff::En1992Diff;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
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

/// 📥️ Decodes one committed mutation JSON document into [`En1992Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1992_mutation_json(text: &str) -> Result<En1992Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations_codec::*;
use semio_s_artifact_norm_en1992_mut_project::change_annex;
use semio_s_artifact_norm_en1992_mut_project::change_reliability_class;
use semio_s_artifact_norm_en1992_mut_project::change_title;
use semio_s_artifact_norm_en1992_mut_project::change_design_working_life;
use semio_s_artifact_norm_en1992_mut_project::change_delta_c_dev;
use semio_s_artifact_norm_en1992_mut_project::change_cement_type;
use semio_s_artifact_norm_en1992_mut_project::change_concrete_f_ck;
use semio_s_artifact_norm_en1992_mut_project::change_reinforcement_f_yk;
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
use semio_s_artifact_norm_en1992_mut_anchors::remove_anchor;
use semio_s_artifact_norm_en1992_mut_anchors::change_anchor_h_ef;
use semio_s_artifact_norm_en1992_mut_anchors::change_anchor_a_s;
use semio_s_artifact_norm_en1992_mut_members::insert_member;
use semio_s_artifact_norm_en1992_mut_anchors::insert_anchor;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
