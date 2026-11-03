//! 🧬️ EN 1999 mutations — hierarchical aluminium-structure vocabulary.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1999_diff::En1999Diff;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve, resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1999_mut_inputs::change_annex;
use semio_s_artifact_norm_en1999_mut_inputs::change_reliability_class;
use semio_s_artifact_norm_en1999_mut_members::insert_member;
use semio_s_artifact_norm_en1999_mut_members::remove_member;
use semio_s_artifact_norm_en1999_mut_members::change_member_n_ed;
use semio_s_artifact_norm_en1999_mut_members::change_member_m_y_ed;
use semio_s_artifact_norm_en1999_mut_members::change_member_buckling_length;
use semio_s_artifact_norm_en1999_mut_members::change_material_designation;
use semio_s_artifact_norm_en1999_mut_members::change_plate_thickness;
use semio_s_artifact_norm_en1999_mut_members::change_weld_throat;
use semio_s_artifact_norm_en1999_mut_material::change_material_product_form;
use semio_s_artifact_norm_en1999_mut_section::change_section_kind;
use semio_s_artifact_norm_en1999_mut_section::change_section_height;
use semio_s_artifact_norm_en1999_mut_section::change_section_width;
use semio_s_artifact_norm_en1999_mut_section::change_section_flange_thickness;
use semio_s_artifact_norm_en1999_mut_section::change_section_web_thickness;
use semio_s_artifact_norm_en1999_mut_section::change_section_outer_diameter;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_width;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_outstand;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_welded;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_weld_position;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_weld_process;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_weld_t_junction;
use semio_s_artifact_norm_en1999_mut_section_element::change_element_weld_direction;
use semio_s_artifact_norm_en1999_mut_section_element_stiffeners::change_element_stiffener_depth;
use semio_s_artifact_norm_en1999_mut_section_element_stiffeners::change_element_stiffener_thk;
use semio_s_artifact_norm_en1999_mut_member::change_member_section_id;
use semio_s_artifact_norm_en1999_mut_member::change_member_material_id;
use semio_s_artifact_norm_en1999_mut_member::change_member_length;
use semio_s_artifact_norm_en1999_mut_member::change_member_support;
use semio_s_artifact_norm_en1999_mut_member::change_member_ltb_length;
use semio_s_artifact_norm_en1999_mut_member::change_member_c1;
use semio_s_artifact_norm_en1999_mut_member::change_member_restrained_ltb;
use semio_s_artifact_norm_en1999_mut_member_web_details::change_member_bearing_length;
use semio_s_artifact_norm_en1999_mut_member_web_details::change_member_transverse_weld;
use semio_s_artifact_norm_en1999_mut_member_web_details::change_member_t_weld_offset;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_kind;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_category;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_source;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_g_k_line;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_q_k_line;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_v_y_k;
use semio_s_artifact_norm_en1999_mut_member_action::change_action_v_z_k;
use semio_s_artifact_norm_en1999_mut_member_action_components::change_action_m_z_k;
use semio_s_artifact_norm_en1999_mut_member_action_components::change_action_torsion_k;
use semio_s_artifact_norm_en1999_mut_member_action_components::change_action_local_load_k;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_member_id;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_material_id;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_kind;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_force_method;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_bolts_material;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_bolts_diameter;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_bolts_edge_distance;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_bolts_pitch;
use semio_s_artifact_norm_en1999_mut_connection_fasteners::change_conn_bolts_gauge;
use semio_s_artifact_norm_en1999_mut_connection_fasteners::change_conn_bolts_plate_thickness;
use semio_s_artifact_norm_en1999_mut_connection_fasteners::change_conn_welds_filler_alloy;
use semio_s_artifact_norm_en1999_mut_connection_fasteners::change_conn_welds_length;
use semio_s_artifact_norm_en1999_mut_connection_fasteners::change_conn_welds_beta_w;
use semio_s_artifact_norm_en1999_mut_connection_fasteners::change_conn_welds_haz_extent;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_kind;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_category;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_source;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_g_k_line;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_q_k_line;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_n_k;
use semio_s_artifact_norm_en1999_mut_connection_action::change_conn_action_v_y_k;
use semio_s_artifact_norm_en1999_mut_connection_action_components::change_conn_action_v_z_k;
use semio_s_artifact_norm_en1999_mut_connection_action_components::change_conn_action_m_y_k;
use semio_s_artifact_norm_en1999_mut_connection_action_components::change_conn_action_m_z_k;
use semio_s_artifact_norm_en1999_mut_connection_action_components::change_conn_action_torsion_k;
use semio_s_artifact_norm_en1999_mut_connection_action_components::change_conn_action_local_load_k;
use semio_s_artifact_norm_en1999_mut_fire_scenario::change_fire_member_id;
use semio_s_artifact_norm_en1999_mut_fire_scenario::change_fire_theta_a;
use semio_s_artifact_norm_en1999_mut_fire_scenario::change_fire_duration_s;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_member_id;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_detail_category;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_delta_sigma_c;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_delta_sigma_ed;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_n_cycles;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_m1;
use semio_s_artifact_norm_en1999_mut_fatigue_detail::change_fatigue_m2;
use semio_s_artifact_norm_en1999_mut_cold_formed::change_sheet_material_id;
use semio_s_artifact_norm_en1999_mut_cold_formed::change_sheet_thickness;
use semio_s_artifact_norm_en1999_mut_cold_formed::change_sheet_width;
use semio_s_artifact_norm_en1999_mut_cold_formed::change_sheet_span;
use semio_s_artifact_norm_en1999_mut_cold_formed::change_sheet_welded;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_kind;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_category;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_source;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_g_k_line;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_q_k_line;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_n_k;
use semio_s_artifact_norm_en1999_mut_cold_formed_action::change_sheet_action_v_y_k;
use semio_s_artifact_norm_en1999_mut_cold_formed_action_components::change_sheet_action_v_z_k;
use semio_s_artifact_norm_en1999_mut_cold_formed_action_components::change_sheet_action_m_y_k;
use semio_s_artifact_norm_en1999_mut_cold_formed_action_components::change_sheet_action_m_z_k;
use semio_s_artifact_norm_en1999_mut_cold_formed_action_components::change_sheet_action_torsion_k;
use semio_s_artifact_norm_en1999_mut_cold_formed_action_components::change_sheet_action_local_load_k;
use semio_s_artifact_norm_en1999_mut_shell::change_shell_material_id;
use semio_s_artifact_norm_en1999_mut_shell::change_shell_radius;
use semio_s_artifact_norm_en1999_mut_shell::change_shell_thickness;
use semio_s_artifact_norm_en1999_mut_shell::change_shell_length;
use semio_s_artifact_norm_en1999_mut_shell::change_shell_welded;
use semio_s_artifact_norm_en1999_mut_shell::change_shell_fabrication_class;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_kind;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_category;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_source;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_g_k_line;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_q_k_line;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_n_k;
use semio_s_artifact_norm_en1999_mut_shell_action::change_shell_action_v_y_k;
use semio_s_artifact_norm_en1999_mut_shell_action_components::change_shell_action_v_z_k;
use semio_s_artifact_norm_en1999_mut_shell_action_components::change_shell_action_m_y_k;
use semio_s_artifact_norm_en1999_mut_shell_action_components::change_shell_action_m_z_k;
use semio_s_artifact_norm_en1999_mut_shell_action_components::change_shell_action_torsion_k;
use semio_s_artifact_norm_en1999_mut_shell_action_components::change_shell_action_local_load_k;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_member_id;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_element_id;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_start;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_length;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_offset;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_throat;
use semio_s_artifact_norm_en1999_mut_weld_run::change_weld_run_process;
use semio_s_artifact_norm_en1999_mut_weld_run_process::change_weld_run_t_junction;
use semio_s_artifact_norm_en1999_mut_weld_run_process::change_weld_run_direction;
use semio_s_artifact_norm_en1999_mut_weld_run_process::change_weld_run_haz_extent;
use semio_s_artifact_norm_en1999_mut_outline_vertex::change_vertex_section_id;
use semio_s_artifact_norm_en1999_mut_outline_vertex::change_vertex_ring;
use semio_s_artifact_norm_en1999_mut_outline_vertex::change_vertex_element_id;
use semio_s_artifact_norm_en1999_mut_outline_vertex::change_vertex_y;
use semio_s_artifact_norm_en1999_mut_outline_vertex::change_vertex_z;
use semio_s_artifact_norm_en1999_mut_placement::change_placement_x;
use semio_s_artifact_norm_en1999_mut_placement::change_placement_y;
use semio_s_artifact_norm_en1999_mut_placement::change_placement_z;
use semio_s_artifact_norm_en1999_mut_placement::change_placement_yaw;
use semio_s_artifact_norm_en1999_mut_collection_rows::insert_material;
use semio_s_artifact_norm_en1999_mut_collection_rows::remove_material;
use semio_s_artifact_norm_en1999_mut_collection_rows::insert_section;
use semio_s_artifact_norm_en1999_mut_collection_rows::remove_section;
use semio_s_artifact_norm_en1999_mut_collection_rows::insert_connection;
use semio_s_artifact_norm_en1999_mut_collection_rows::remove_connection;
use semio_s_artifact_norm_en1999_mut_scenario_rows::insert_fire_scenario;
use semio_s_artifact_norm_en1999_mut_scenario_rows::remove_fire_scenario;
use semio_s_artifact_norm_en1999_mut_scenario_rows::insert_fatigue_detail;
use semio_s_artifact_norm_en1999_mut_scenario_rows::remove_fatigue_detail;
use semio_s_artifact_norm_en1999_mut_scenario_rows::insert_cold_formed_sheet;
use semio_s_artifact_norm_en1999_mut_scenario_rows::remove_cold_formed_sheet;
use semio_s_artifact_norm_en1999_mut_collection_rows::insert_shell;
use semio_s_artifact_norm_en1999_mut_collection_rows::remove_shell;
use semio_s_artifact_norm_en1999_mut_detail_rows::insert_weld_run;
use semio_s_artifact_norm_en1999_mut_detail_rows::remove_weld_run;
use semio_s_artifact_norm_en1999_mut_detail_rows::insert_outline_vertex;
use semio_s_artifact_norm_en1999_mut_detail_rows::remove_outline_vertex;
use semio_s_artifact_norm_en1999_mut_detail_rows::insert_placement;
use semio_s_artifact_norm_en1999_mut_detail_rows::remove_placement;
use semio_s_artifact_norm_en1999_mut_member::change_member_buckling_length_y;
use semio_s_artifact_norm_en1999_mut_member::change_member_buckling_length_t;
use semio_s_artifact_norm_en1999_mut_connection::change_conn_bolts_bolts_per_row;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_member_id;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_element_id;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_detail_category;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_detail_type;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_location_x;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_m1;
use semio_s_artifact_norm_en1999_mut_fatigue_category::change_category_m2;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_subject_id;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_position;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_x;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_bearing_length;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_web_height;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_web_angle;
use semio_s_artifact_norm_en1999_mut_patch_load::change_patch_corner_radius;
use semio_s_artifact_norm_en1999_mut_patch_load_load::change_patch_load;
use semio_s_artifact_norm_en1999_mut_ring_stiffener::change_ring_shell_id;
use semio_s_artifact_norm_en1999_mut_ring_stiffener::change_ring_x;
use semio_s_artifact_norm_en1999_mut_ring_stiffener::change_ring_depth;
use semio_s_artifact_norm_en1999_mut_ring_stiffener::change_ring_thickness;
use semio_s_artifact_norm_en1999_mut_ring_stiffener::change_ring_internal;
use semio_s_artifact_norm_en1999_mut_rows_fatigue_category::insert_fatigue_category;
use semio_s_artifact_norm_en1999_mut_rows_fatigue_category::remove_fatigue_category;
use semio_s_artifact_norm_en1999_mut_rows_patch_load::insert_patch_load;
use semio_s_artifact_norm_en1999_mut_rows_patch_load::remove_patch_load;
use semio_s_artifact_norm_en1999_mut_rows_ring_stiffener::insert_ring_stiffener;
use semio_s_artifact_norm_en1999_mut_rows_ring_stiffener::remove_ring_stiffener;
use semio_s_artifact_norm_en1999_mut_rows_member_action::insert_member_action;
use semio_s_artifact_norm_en1999_mut_rows_member_action::remove_member_action;
use semio_s_artifact_norm_en1999_mut_rows_connection_action::insert_connection_action;
use semio_s_artifact_norm_en1999_mut_rows_connection_action::remove_connection_action;
use semio_s_artifact_norm_en1999_mut_rows_section_element::insert_section_element;
use semio_s_artifact_norm_en1999_mut_rows_section_element::remove_section_element;
use semio_s_artifact_norm_en1999_mut_rows_sheet_action::insert_sheet_action;
use semio_s_artifact_norm_en1999_mut_rows_sheet_action::remove_sheet_action;
use semio_s_artifact_norm_en1999_mut_rows_shell_action::insert_shell_action;
use semio_s_artifact_norm_en1999_mut_rows_shell_action::remove_shell_action;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_transverse_girder_id;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_transverse_position;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_transverse_width;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_transverse_thickness;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_conn_load_eccentricity;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_conn_hole_stagger;
use semio_s_artifact_norm_en1999_mut_transverse_stiffener::change_transverse_sides;
use semio_s_artifact_norm_en1999_mut_rows_transverse_stiffener::insert_transverse_stiffener;
use semio_s_artifact_norm_en1999_mut_rows_transverse_stiffener::remove_transverse_stiffener;
use semio_s_artifact_norm_en1999_mut_members::change_bolt_count;

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1999Snapshot, diff = En1999Diff, schema = "norm.en1999", reference = En1999Snapshot, reference_diff = En1999Diff)]
pub enum En1999Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    InsertMember(insert_member::InsertMember),
    RemoveMember(remove_member::RemoveMember),
    ChangeMemberNEd(change_member_n_ed::ChangeMemberNEd),
    ChangeMemberMYEd(change_member_m_y_ed::ChangeMemberMYEd),
    ChangeMemberBucklingLength(change_member_buckling_length::ChangeMemberBucklingLength),
    ChangeMaterialDesignation(change_material_designation::ChangeMaterialDesignation),
    ChangePlateThickness(change_plate_thickness::ChangePlateThickness),
    ChangeWeldThroat(change_weld_throat::ChangeWeldThroat),
    ChangeBoltCount(change_bolt_count::ChangeBoltCount),
    ChangeMaterialProductForm(change_material_product_form::ChangeMaterialProductForm),
    ChangeSectionKind(change_section_kind::ChangeSectionKind),
    ChangeSectionHeight(change_section_height::ChangeSectionHeight),
    ChangeSectionWidth(change_section_width::ChangeSectionWidth),
    ChangeSectionFlangeThickness(change_section_flange_thickness::ChangeSectionFlangeThickness),
    ChangeSectionWebThickness(change_section_web_thickness::ChangeSectionWebThickness),
    ChangeSectionOuterDiameter(change_section_outer_diameter::ChangeSectionOuterDiameter),
    ChangeElementWidth(change_element_width::ChangeElementWidth),
    ChangeElementOutstand(change_element_outstand::ChangeElementOutstand),
    ChangeElementWelded(change_element_welded::ChangeElementWelded),
    ChangeElementWeldPosition(change_element_weld_position::ChangeElementWeldPosition),
    ChangeElementWeldProcess(change_element_weld_process::ChangeElementWeldProcess),
    ChangeElementWeldTJunction(change_element_weld_t_junction::ChangeElementWeldTJunction),
    ChangeElementWeldDirection(change_element_weld_direction::ChangeElementWeldDirection),
    ChangeElementStiffenerDepth(change_element_stiffener_depth::ChangeElementStiffenerDepth),
    ChangeElementStiffenerThk(change_element_stiffener_thk::ChangeElementStiffenerThk),
    ChangeMemberSectionId(change_member_section_id::ChangeMemberSectionId),
    ChangeMemberMaterialId(change_member_material_id::ChangeMemberMaterialId),
    ChangeMemberLength(change_member_length::ChangeMemberLength),
    ChangeMemberSupport(change_member_support::ChangeMemberSupport),
    ChangeMemberLtbLength(change_member_ltb_length::ChangeMemberLtbLength),
    ChangeMemberC1(change_member_c1::ChangeMemberC1),
    ChangeMemberRestrainedLtb(change_member_restrained_ltb::ChangeMemberRestrainedLtb),
    ChangeMemberBearingLength(change_member_bearing_length::ChangeMemberBearingLength),
    ChangeMemberTransverseWeld(change_member_transverse_weld::ChangeMemberTransverseWeld),
    ChangeMemberTWeldOffset(change_member_t_weld_offset::ChangeMemberTWeldOffset),
    ChangeActionKind(change_action_kind::ChangeActionKind),
    ChangeActionCategory(change_action_category::ChangeActionCategory),
    ChangeActionSource(change_action_source::ChangeActionSource),
    ChangeActionGKLine(change_action_g_k_line::ChangeActionGKLine),
    ChangeActionQKLine(change_action_q_k_line::ChangeActionQKLine),
    ChangeActionVYK(change_action_v_y_k::ChangeActionVYK),
    ChangeActionVZK(change_action_v_z_k::ChangeActionVZK),
    ChangeActionMZK(change_action_m_z_k::ChangeActionMZK),
    ChangeActionTorsionK(change_action_torsion_k::ChangeActionTorsionK),
    ChangeActionLocalLoadK(change_action_local_load_k::ChangeActionLocalLoadK),
    ChangeConnMemberId(change_conn_member_id::ChangeConnMemberId),
    ChangeConnMaterialId(change_conn_material_id::ChangeConnMaterialId),
    ChangeConnKind(change_conn_kind::ChangeConnKind),
    ChangeConnForceMethod(change_conn_force_method::ChangeConnForceMethod),
    ChangeConnBoltsMaterial(change_conn_bolts_material::ChangeConnBoltsMaterial),
    ChangeConnBoltsDiameter(change_conn_bolts_diameter::ChangeConnBoltsDiameter),
    ChangeConnBoltsEdgeDistance(change_conn_bolts_edge_distance::ChangeConnBoltsEdgeDistance),
    ChangeConnBoltsPitch(change_conn_bolts_pitch::ChangeConnBoltsPitch),
    ChangeConnBoltsGauge(change_conn_bolts_gauge::ChangeConnBoltsGauge),
    ChangeConnBoltsPlateThickness(change_conn_bolts_plate_thickness::ChangeConnBoltsPlateThickness),
    ChangeConnWeldsFillerAlloy(change_conn_welds_filler_alloy::ChangeConnWeldsFillerAlloy),
    ChangeConnWeldsLength(change_conn_welds_length::ChangeConnWeldsLength),
    ChangeConnWeldsBetaW(change_conn_welds_beta_w::ChangeConnWeldsBetaW),
    ChangeConnWeldsHazExtent(change_conn_welds_haz_extent::ChangeConnWeldsHazExtent),
    ChangeConnActionKind(change_conn_action_kind::ChangeConnActionKind),
    ChangeConnActionCategory(change_conn_action_category::ChangeConnActionCategory),
    ChangeConnActionSource(change_conn_action_source::ChangeConnActionSource),
    ChangeConnActionGKLine(change_conn_action_g_k_line::ChangeConnActionGKLine),
    ChangeConnActionQKLine(change_conn_action_q_k_line::ChangeConnActionQKLine),
    ChangeConnActionNK(change_conn_action_n_k::ChangeConnActionNK),
    ChangeConnActionVYK(change_conn_action_v_y_k::ChangeConnActionVYK),
    ChangeConnActionVZK(change_conn_action_v_z_k::ChangeConnActionVZK),
    ChangeConnActionMYK(change_conn_action_m_y_k::ChangeConnActionMYK),
    ChangeConnActionMZK(change_conn_action_m_z_k::ChangeConnActionMZK),
    ChangeConnActionTorsionK(change_conn_action_torsion_k::ChangeConnActionTorsionK),
    ChangeConnActionLocalLoadK(change_conn_action_local_load_k::ChangeConnActionLocalLoadK),
    ChangeFireMemberId(change_fire_member_id::ChangeFireMemberId),
    ChangeFireThetaA(change_fire_theta_a::ChangeFireThetaA),
    ChangeFireDurationS(change_fire_duration_s::ChangeFireDurationS),
    ChangeFatigueMemberId(change_fatigue_member_id::ChangeFatigueMemberId),
    ChangeFatigueDetailCategory(change_fatigue_detail_category::ChangeFatigueDetailCategory),
    ChangeFatigueDeltaSigmaC(change_fatigue_delta_sigma_c::ChangeFatigueDeltaSigmaC),
    ChangeFatigueDeltaSigmaEd(change_fatigue_delta_sigma_ed::ChangeFatigueDeltaSigmaEd),
    ChangeFatigueNCycles(change_fatigue_n_cycles::ChangeFatigueNCycles),
    ChangeFatigueM1(change_fatigue_m1::ChangeFatigueM1),
    ChangeFatigueM2(change_fatigue_m2::ChangeFatigueM2),
    ChangeSheetMaterialId(change_sheet_material_id::ChangeSheetMaterialId),
    ChangeSheetThickness(change_sheet_thickness::ChangeSheetThickness),
    ChangeSheetWidth(change_sheet_width::ChangeSheetWidth),
    ChangeSheetSpan(change_sheet_span::ChangeSheetSpan),
    ChangeSheetWelded(change_sheet_welded::ChangeSheetWelded),
    ChangeSheetActionKind(change_sheet_action_kind::ChangeSheetActionKind),
    ChangeSheetActionCategory(change_sheet_action_category::ChangeSheetActionCategory),
    ChangeSheetActionSource(change_sheet_action_source::ChangeSheetActionSource),
    ChangeSheetActionGKLine(change_sheet_action_g_k_line::ChangeSheetActionGKLine),
    ChangeSheetActionQKLine(change_sheet_action_q_k_line::ChangeSheetActionQKLine),
    ChangeSheetActionNK(change_sheet_action_n_k::ChangeSheetActionNK),
    ChangeSheetActionVYK(change_sheet_action_v_y_k::ChangeSheetActionVYK),
    ChangeSheetActionVZK(change_sheet_action_v_z_k::ChangeSheetActionVZK),
    ChangeSheetActionMYK(change_sheet_action_m_y_k::ChangeSheetActionMYK),
    ChangeSheetActionMZK(change_sheet_action_m_z_k::ChangeSheetActionMZK),
    ChangeSheetActionTorsionK(change_sheet_action_torsion_k::ChangeSheetActionTorsionK),
    ChangeSheetActionLocalLoadK(change_sheet_action_local_load_k::ChangeSheetActionLocalLoadK),
    ChangeShellMaterialId(change_shell_material_id::ChangeShellMaterialId),
    ChangeShellRadius(change_shell_radius::ChangeShellRadius),
    ChangeShellThickness(change_shell_thickness::ChangeShellThickness),
    ChangeShellLength(change_shell_length::ChangeShellLength),
    ChangeShellWelded(change_shell_welded::ChangeShellWelded),
    ChangeShellFabricationClass(change_shell_fabrication_class::ChangeShellFabricationClass),
    ChangeShellActionKind(change_shell_action_kind::ChangeShellActionKind),
    ChangeShellActionCategory(change_shell_action_category::ChangeShellActionCategory),
    ChangeShellActionSource(change_shell_action_source::ChangeShellActionSource),
    ChangeShellActionGKLine(change_shell_action_g_k_line::ChangeShellActionGKLine),
    ChangeShellActionQKLine(change_shell_action_q_k_line::ChangeShellActionQKLine),
    ChangeShellActionNK(change_shell_action_n_k::ChangeShellActionNK),
    ChangeShellActionVYK(change_shell_action_v_y_k::ChangeShellActionVYK),
    ChangeShellActionVZK(change_shell_action_v_z_k::ChangeShellActionVZK),
    ChangeShellActionMYK(change_shell_action_m_y_k::ChangeShellActionMYK),
    ChangeShellActionMZK(change_shell_action_m_z_k::ChangeShellActionMZK),
    ChangeShellActionTorsionK(change_shell_action_torsion_k::ChangeShellActionTorsionK),
    ChangeShellActionLocalLoadK(change_shell_action_local_load_k::ChangeShellActionLocalLoadK),
    ChangeWeldRunMemberId(change_weld_run_member_id::ChangeWeldRunMemberId),
    ChangeWeldRunElementId(change_weld_run_element_id::ChangeWeldRunElementId),
    ChangeWeldRunStart(change_weld_run_start::ChangeWeldRunStart),
    ChangeWeldRunLength(change_weld_run_length::ChangeWeldRunLength),
    ChangeWeldRunOffset(change_weld_run_offset::ChangeWeldRunOffset),
    ChangeWeldRunThroat(change_weld_run_throat::ChangeWeldRunThroat),
    ChangeWeldRunProcess(change_weld_run_process::ChangeWeldRunProcess),
    ChangeWeldRunTJunction(change_weld_run_t_junction::ChangeWeldRunTJunction),
    ChangeWeldRunDirection(change_weld_run_direction::ChangeWeldRunDirection),
    ChangeWeldRunHazExtent(change_weld_run_haz_extent::ChangeWeldRunHazExtent),
    ChangeVertexSectionId(change_vertex_section_id::ChangeVertexSectionId),
    ChangeVertexRing(change_vertex_ring::ChangeVertexRing),
    ChangeVertexElementId(change_vertex_element_id::ChangeVertexElementId),
    ChangeVertexY(change_vertex_y::ChangeVertexY),
    ChangeVertexZ(change_vertex_z::ChangeVertexZ),
    ChangePlacementX(change_placement_x::ChangePlacementX),
    ChangePlacementY(change_placement_y::ChangePlacementY),
    ChangePlacementZ(change_placement_z::ChangePlacementZ),
    ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw),
    InsertWeldRun(insert_weld_run::InsertWeldRun),
    RemoveWeldRun(remove_weld_run::RemoveWeldRun),
    InsertOutlineVertex(insert_outline_vertex::InsertOutlineVertex),
    RemoveOutlineVertex(remove_outline_vertex::RemoveOutlineVertex),
    InsertPlacement(insert_placement::InsertPlacement),
    RemovePlacement(remove_placement::RemovePlacement),
    ChangeMemberBucklingLengthY(change_member_buckling_length_y::ChangeMemberBucklingLengthY),
    ChangeMemberBucklingLengthT(change_member_buckling_length_t::ChangeMemberBucklingLengthT),
    ChangeConnBoltsBoltsPerRow(change_conn_bolts_bolts_per_row::ChangeConnBoltsBoltsPerRow),
    InsertMaterial(insert_material::InsertMaterial),
    RemoveMaterial(remove_material::RemoveMaterial),
    InsertSection(insert_section::InsertSection),
    RemoveSection(remove_section::RemoveSection),
    InsertConnection(insert_connection::InsertConnection),
    RemoveConnection(remove_connection::RemoveConnection),
    InsertFireScenario(insert_fire_scenario::InsertFireScenario),
    RemoveFireScenario(remove_fire_scenario::RemoveFireScenario),
    InsertFatigueDetail(insert_fatigue_detail::InsertFatigueDetail),
    RemoveFatigueDetail(remove_fatigue_detail::RemoveFatigueDetail),
    InsertColdFormedSheet(insert_cold_formed_sheet::InsertColdFormedSheet),
    RemoveColdFormedSheet(remove_cold_formed_sheet::RemoveColdFormedSheet),
    InsertShell(insert_shell::InsertShell),
    RemoveShell(remove_shell::RemoveShell),
    ChangeCategoryMemberId(change_category_member_id::ChangeCategoryMemberId),
    ChangeCategoryElementId(change_category_element_id::ChangeCategoryElementId),
    ChangeCategoryDetailCategory(change_category_detail_category::ChangeCategoryDetailCategory),
    ChangeCategoryDetailType(change_category_detail_type::ChangeCategoryDetailType),
    ChangeCategoryLocationX(change_category_location_x::ChangeCategoryLocationX),
    ChangeCategoryM1(change_category_m1::ChangeCategoryM1),
    ChangeCategoryM2(change_category_m2::ChangeCategoryM2),
    ChangePatchSubjectId(change_patch_subject_id::ChangePatchSubjectId),
    ChangePatchPosition(change_patch_position::ChangePatchPosition),
    ChangePatchX(change_patch_x::ChangePatchX),
    ChangePatchBearingLength(change_patch_bearing_length::ChangePatchBearingLength),
    ChangePatchWebHeight(change_patch_web_height::ChangePatchWebHeight),
    ChangePatchWebAngle(change_patch_web_angle::ChangePatchWebAngle),
    ChangePatchCornerRadius(change_patch_corner_radius::ChangePatchCornerRadius),
    ChangePatchLoad(change_patch_load::ChangePatchLoad),
    ChangeRingShellId(change_ring_shell_id::ChangeRingShellId),
    ChangeRingX(change_ring_x::ChangeRingX),
    ChangeRingDepth(change_ring_depth::ChangeRingDepth),
    ChangeRingThickness(change_ring_thickness::ChangeRingThickness),
    ChangeRingInternal(change_ring_internal::ChangeRingInternal),
    InsertFatigueCategory(insert_fatigue_category::InsertFatigueCategory),
    RemoveFatigueCategory(remove_fatigue_category::RemoveFatigueCategory),
    InsertPatchLoad(insert_patch_load::InsertPatchLoad),
    RemovePatchLoad(remove_patch_load::RemovePatchLoad),
    InsertRingStiffener(insert_ring_stiffener::InsertRingStiffener),
    RemoveRingStiffener(remove_ring_stiffener::RemoveRingStiffener),
    InsertMemberAction(insert_member_action::InsertMemberAction),
    RemoveMemberAction(remove_member_action::RemoveMemberAction),
    InsertConnectionAction(insert_connection_action::InsertConnectionAction),
    RemoveConnectionAction(remove_connection_action::RemoveConnectionAction),
    InsertSectionElement(insert_section_element::InsertSectionElement),
    RemoveSectionElement(remove_section_element::RemoveSectionElement),
    InsertSheetAction(insert_sheet_action::InsertSheetAction),
    RemoveSheetAction(remove_sheet_action::RemoveSheetAction),
    InsertShellAction(insert_shell_action::InsertShellAction),
    RemoveShellAction(remove_shell_action::RemoveShellAction),
    ChangeTransverseGirderId(change_transverse_girder_id::ChangeTransverseGirderId),
    ChangeTransversePosition(change_transverse_position::ChangeTransversePosition),
    ChangeTransverseWidth(change_transverse_width::ChangeTransverseWidth),
    ChangeTransverseThickness(change_transverse_thickness::ChangeTransverseThickness),
    ChangeConnLoadEccentricity(change_conn_load_eccentricity::ChangeConnLoadEccentricity),
    ChangeConnHoleStagger(change_conn_hole_stagger::ChangeConnHoleStagger),
    ChangeTransverseSides(change_transverse_sides::ChangeTransverseSides),
    InsertTransverseStiffener(insert_transverse_stiffener::InsertTransverseStiffener),
    RemoveTransverseStiffener(remove_transverse_stiffener::RemoveTransverseStiffener),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "insert-member",
    "remove-member",
    "change-member-n-ed",
    "change-member-my-ed",
    "change-member-buckling-length",
    "change-material-designation",
    "change-plate-thickness",
    "change-weld-throat",
    "change-bolt-count",
    "change-material-product-form",
    "change-section-kind",
    "change-section-height",
    "change-section-width",
    "change-section-flange-thickness",
    "change-section-web-thickness",
    "change-section-outer-diameter",
    "change-element-width",
    "change-element-outstand",
    "change-element-welded",
    "change-element-weld-position",
    "change-element-weld-process",
    "change-element-weld-t-junction",
    "change-element-weld-direction",
    "change-element-stiffener-depth",
    "change-element-stiffener-thk",
    "change-member-section-id",
    "change-member-material-id",
    "change-member-length",
    "change-member-support",
    "change-member-ltb-length",
    "change-member-c1",
    "change-member-restrained-ltb",
    "change-member-bearing-length",
    "change-member-transverse-weld",
    "change-member-t-weld-offset",
    "change-action-kind",
    "change-action-category",
    "change-action-source",
    "change-action-gk-line",
    "change-action-qk-line",
    "change-action-vyk",
    "change-action-vzk",
    "change-action-mzk",
    "change-action-torsion-k",
    "change-action-local-load-k",
    "change-conn-member-id",
    "change-conn-material-id",
    "change-conn-kind",
    "change-conn-force-method",
    "change-conn-bolts-material",
    "change-conn-bolts-diameter",
    "change-conn-bolts-edge-distance",
    "change-conn-bolts-pitch",
    "change-conn-bolts-gauge",
    "change-conn-bolts-plate-thickness",
    "change-conn-welds-filler-alloy",
    "change-conn-welds-length",
    "change-conn-welds-beta-w",
    "change-conn-welds-haz-extent",
    "change-conn-action-kind",
    "change-conn-action-category",
    "change-conn-action-source",
    "change-conn-action-gk-line",
    "change-conn-action-qk-line",
    "change-conn-action-nk",
    "change-conn-action-vyk",
    "change-conn-action-vzk",
    "change-conn-action-myk",
    "change-conn-action-mzk",
    "change-conn-action-torsion-k",
    "change-conn-action-local-load-k",
    "change-fire-member-id",
    "change-fire-theta-a",
    "change-fire-duration-s",
    "change-fatigue-member-id",
    "change-fatigue-detail-category",
    "change-fatigue-delta-sigma-c",
    "change-fatigue-delta-sigma-ed",
    "change-fatigue-n-cycles",
    "change-fatigue-m1",
    "change-fatigue-m2",
    "change-sheet-material-id",
    "change-sheet-thickness",
    "change-sheet-width",
    "change-sheet-span",
    "change-sheet-welded",
    "change-sheet-action-kind",
    "change-sheet-action-category",
    "change-sheet-action-source",
    "change-sheet-action-gk-line",
    "change-sheet-action-qk-line",
    "change-sheet-action-nk",
    "change-sheet-action-vyk",
    "change-sheet-action-vzk",
    "change-sheet-action-myk",
    "change-sheet-action-mzk",
    "change-sheet-action-torsion-k",
    "change-sheet-action-local-load-k",
    "change-shell-material-id",
    "change-shell-radius",
    "change-shell-thickness",
    "change-shell-length",
    "change-shell-welded",
    "change-shell-fabrication-class",
    "change-shell-action-kind",
    "change-shell-action-category",
    "change-shell-action-source",
    "change-shell-action-gk-line",
    "change-shell-action-qk-line",
    "change-shell-action-nk",
    "change-shell-action-vyk",
    "change-shell-action-vzk",
    "change-shell-action-myk",
    "change-shell-action-mzk",
    "change-shell-action-torsion-k",
    "change-shell-action-local-load-k",
    "change-weld-run-member-id",
    "change-weld-run-element-id",
    "change-weld-run-start",
    "change-weld-run-length",
    "change-weld-run-offset",
    "change-weld-run-throat",
    "change-weld-run-process",
    "change-weld-run-t-junction",
    "change-weld-run-direction",
    "change-weld-run-haz-extent",
    "change-vertex-section-id",
    "change-vertex-ring",
    "change-vertex-element-id",
    "change-vertex-y",
    "change-vertex-z",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-weld-run",
    "remove-weld-run",
    "insert-outline-vertex",
    "remove-outline-vertex",
    "insert-placement",
    "remove-placement",
    "change-member-buckling-length-y",
    "change-member-buckling-length-t",
    "change-conn-bolts-bolts-per-row",
    "insert-material",
    "remove-material",
    "insert-section",
    "remove-section",
    "insert-connection",
    "remove-connection",
    "insert-fire-scenario",
    "remove-fire-scenario",
    "insert-fatigue-detail",
    "remove-fatigue-detail",
    "insert-cold-formed-sheet",
    "remove-cold-formed-sheet",
    "insert-shell",
    "remove-shell",
    "change-category-member-id",
    "change-category-element-id",
    "change-category-detail-category",
    "change-category-detail-type",
    "change-category-location-x",
    "change-category-m1",
    "change-category-m2",
    "change-patch-subject-id",
    "change-patch-position",
    "change-patch-x",
    "change-patch-bearing-length",
    "change-patch-web-height",
    "change-patch-web-angle",
    "change-patch-corner-radius",
    "change-patch-load",
    "change-ring-shell-id",
    "change-ring-x",
    "change-ring-depth",
    "change-ring-thickness",
    "change-ring-internal",
    "insert-fatigue-category",
    "remove-fatigue-category",
    "insert-patch-load",
    "remove-patch-load",
    "insert-ring-stiffener",
    "remove-ring-stiffener",
    "insert-member-action",
    "remove-member-action",
    "insert-connection-action",
    "remove-connection-action",
    "insert-section-element",
    "remove-section-element",
    "insert-sheet-action",
    "remove-sheet-action",
    "insert-shell-action",
    "remove-shell-action",
    "change-transverse-girder-id",
    "change-transverse-position",
    "change-transverse-width",
    "change-transverse-thickness",
    "change-conn-load-eccentricity",
    "change-conn-hole-stagger",
    "change-transverse-sides",
    "insert-transverse-stiffener",
    "remove-transverse-stiffener",
];
//#endregion 🔖️Mutations

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️diff-algebra/🦀️.rs"]
mod diff_algebra;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1999_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1999_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1999_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1999_mutations_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1999Mutation;
    use semio_s_artifact_norm_en1999_model::En1999Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1999_mutation(base: &En1999Snapshot, mutation: &En1999Mutation) -> Result<(En1999Snapshot, Vec<String>), String> {
        let raised = <En1999Mutation as protocol::ColdMutation<En1999Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff_cold(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1999_mutation(mutation: &En1999Mutation, base: &En1999Snapshot) -> Result<Vec<En1999Mutation>, semio_framework_value::ValueError> {
        <En1999Mutation as protocol::ColdMutation<En1999Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
