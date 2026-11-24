//! 🧬️ En1995 artifact — document mutation dispatch for the hierarchical timber subject
//! (members ⊃ characteristic actions, connections ⊃ connection actions).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES, ID_REMOVALS};

//#region 🔖️Mutations
pub use semio_s_artifact_norm_en1995_mut_members::change_annex;
pub use semio_s_artifact_norm_en1995_mut_members::change_reliability_class;
pub use semio_s_artifact_norm_en1995_mut_members::insert_member;
pub use semio_s_artifact_norm_en1995_mut_members::remove_member;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_label_en;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_label_de;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_role;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_strength_class;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_service_class;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_support;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_b;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_h;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_span;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_support_length;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_bearing_length;
pub use semio_s_artifact_norm_en1995_mut_stability::change_member_buckling_length_y;
pub use semio_s_artifact_norm_en1995_mut_stability::change_member_buckling_length_z;
pub use semio_s_artifact_norm_en1995_mut_stability::change_member_restraint_spacing;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_notch_depth;
pub use semio_s_artifact_norm_en1995_mut_geometry::change_member_notch_distance;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_mass_kg_per_m;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_mass_kg_per_m2;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_damping_xi;
pub use semio_s_artifact_norm_en1995_mut_members::change_member_fire_duration;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_n_obs;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_tl_years;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_beta;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_a;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_b;
pub use semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_crowd;
pub use semio_s_artifact_norm_en1995_mut_actions::insert_member_action;
pub use semio_s_artifact_norm_en1995_mut_actions::remove_member_action;
pub use semio_s_artifact_norm_en1995_mut_actions::change_member_action_kind;
pub use semio_s_artifact_norm_en1995_mut_actions::change_member_action_category;
pub use semio_s_artifact_norm_en1995_mut_actions::change_member_load_duration;
pub use semio_s_artifact_norm_en1995_mut_actions::change_member_action_q_line;
pub use semio_s_artifact_norm_en1995_mut_actions::change_member_action_f_point;
pub use semio_s_artifact_norm_en1995_mut_effects::change_member_action_mk;
pub use semio_s_artifact_norm_en1995_mut_effects::change_member_action_vk;
pub use semio_s_artifact_norm_en1995_mut_effects::change_member_action_nk;
pub use semio_s_artifact_norm_en1995_mut_effects::change_member_action_ntk;
pub use semio_s_artifact_norm_en1995_mut_effects::change_member_action_fc90_k;
pub use semio_s_artifact_norm_en1995_mut_connections::insert_connection;
pub use semio_s_artifact_norm_en1995_mut_connections::remove_connection;
pub use semio_s_artifact_norm_en1995_mut_connections::change_connection_label_en;
pub use semio_s_artifact_norm_en1995_mut_connections::change_connection_label_de;
pub use semio_s_artifact_norm_en1995_mut_connections::change_connection_fastener_type;
pub use semio_s_artifact_norm_en1995_mut_connections::change_connection_strength_class;
pub use semio_s_artifact_norm_en1995_mut_connections::change_connection_service_class;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_diameter;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_number;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_rows;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_spacing;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_edge_distance;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_end_distance;
pub use semio_s_artifact_norm_en1995_mut_plates::change_connection_t1;
pub use semio_s_artifact_norm_en1995_mut_plates::change_connection_t2;
pub use semio_s_artifact_norm_en1995_mut_plates::change_connection_steel_plate;
pub use semio_s_artifact_norm_en1995_mut_plates::change_connection_plate_thickness;
pub use semio_s_artifact_norm_en1995_mut_plates::change_connection_shear_planes;
pub use semio_s_artifact_norm_en1995_mut_fasteners::change_connection_fuk;
pub use semio_s_artifact_norm_en1995_mut_joint_actions::insert_connection_action;
pub use semio_s_artifact_norm_en1995_mut_joint_actions::remove_connection_action;
pub use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_kind;
pub use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_load_duration;
pub use semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk;
pub use semio_s_artifact_norm_en1995_mut_design_cases::change_uls_equation;
pub use semio_s_artifact_norm_en1995_mut_design_cases::insert_case;
pub use semio_s_artifact_norm_en1995_mut_design_cases::remove_case;
pub use semio_s_artifact_norm_en1995_mut_design_cases::change_case_label_en;
pub use semio_s_artifact_norm_en1995_mut_design_cases::change_case_label_de;
pub use semio_s_artifact_norm_en1995_mut_design_cases::change_case_entry_id;
pub use semio_s_artifact_norm_en1995_mut_design_cases::change_case_number;
pub use semio_s_artifact_norm_en1995_mut_design_cases::change_case_text;
pub use semio_s_artifact_norm_en1995_mut_placements::change_placement_x;
pub use semio_s_artifact_norm_en1995_mut_placements::change_placement_y;
pub use semio_s_artifact_norm_en1995_mut_placements::change_placement_z;
pub use semio_s_artifact_norm_en1995_mut_placements::change_placement_yaw;
pub use semio_s_artifact_norm_en1995_mut_placements::insert_placement;
pub use semio_s_artifact_norm_en1995_mut_placements::remove_placement;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::insert_deck;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::remove_deck;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_lamination_product;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_bar_product;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_span;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_deck_width;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_deck_depth;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_lamination_width;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_bar_spacing;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_bar_diameter;
pub use semio_s_artifact_norm_en1995_mut_deck_geometry::change_deck_plate_side;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_initial_prestress;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_loss_relaxation;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_loss_creep;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_loss_moisture;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_bar_resistance;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_shear_stress;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_compression_c90;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_wheel_load;
pub use semio_s_artifact_norm_en1995_mut_deck_prestress::change_deck_wheel_contact;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::insert_diaphragm;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::remove_diaphragm;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_sheathing::change_diaphragm_kind;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_sheathing::change_diaphragm_method;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_sheathing::change_diaphragm_panel_width;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_sheathing::change_diaphragm_wall_height;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_sheathing::change_diaphragm_fastener_spacing;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_sheathing::change_diaphragm_panels;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_forces::change_diaphragm_fastener_capacity;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_forces::change_diaphragm_shear_ed;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_forces::change_diaphragm_hold_down;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_forces::change_diaphragm_force;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_forces::change_diaphragm_length;
pub use semio_s_artifact_norm_en1995_mut_diaphragm_forces::change_diaphragm_shear_resistance;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::insert_truss;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::remove_truss;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_kind;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_product;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_span;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_height;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_bays;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_chord_width;
pub use semio_s_artifact_norm_en1995_mut_truss_geometry::change_truss_chord_depth;
pub use semio_s_artifact_norm_en1995_mut_truss_forces::change_truss_web_width;
pub use semio_s_artifact_norm_en1995_mut_truss_forces::change_truss_web_depth;
pub use semio_s_artifact_norm_en1995_mut_truss_forces::change_truss_chord_force;
pub use semio_s_artifact_norm_en1995_mut_truss_forces::change_truss_restraint_length;
pub use semio_s_artifact_norm_en1995_mut_truss_forces::change_truss_bracing_capacity;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::insert_bracing;
pub use semio_s_artifact_norm_en1995_mut_structure_rows::remove_bracing;
pub use semio_s_artifact_norm_en1995_mut_bracing_layout::change_bracing_kind;
pub use semio_s_artifact_norm_en1995_mut_bracing_layout::change_bracing_member;
pub use semio_s_artifact_norm_en1995_mut_bracing_layout::change_bracing_stabilised_count;
pub use semio_s_artifact_norm_en1995_mut_bracing_layout::change_bracing_spacing;
pub use semio_s_artifact_norm_en1995_mut_bracing_layout::change_bracing_length;
pub use semio_s_artifact_norm_en1995_mut_bracing_forces::change_bracing_brace_width;
pub use semio_s_artifact_norm_en1995_mut_bracing_forces::change_bracing_brace_depth;
pub use semio_s_artifact_norm_en1995_mut_bracing_forces::change_bracing_compression;
pub use semio_s_artifact_norm_en1995_mut_bracing_forces::change_bracing_resistance;
pub use semio_s_artifact_norm_en1995_mut_bracing_forces::change_bracing_stiffness;
pub use semio_s_artifact_norm_en1995_mut_panel_layers::insert_panel_layer;
pub use semio_s_artifact_norm_en1995_mut_panel_layers::remove_panel_layer;
pub use semio_s_artifact_norm_en1995_mut_panel_layers::change_panel_layer_thickness;
pub use semio_s_artifact_norm_en1995_mut_panel_layers::change_panel_layer_orientation;
pub use semio_s_artifact_norm_en1995_mut_panel_layers::change_panel_layer_grade;
pub use semio_s_artifact_norm_en1995_mut_fastener_positions::insert_fastener_position;
pub use semio_s_artifact_norm_en1995_mut_fastener_positions::remove_fastener_position;
pub use semio_s_artifact_norm_en1995_mut_fastener_positions::change_fastener_position_x;
pub use semio_s_artifact_norm_en1995_mut_fastener_positions::change_fastener_position_y;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::insert_taper;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::remove_taper;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_member;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_kind;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_taper_angle;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_apex_depth;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_inner_radius;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_lamella;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_apex_angle;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_side;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_rods;
pub use semio_s_artifact_norm_en1995_mut_tapers::change_taper_rod_resistance;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::insert_cutout;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::remove_cutout;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_member;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_kind;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_face;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_distance;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_length;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_depth;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_centre_height;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_slope;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_rods;
pub use semio_s_artifact_norm_en1995_mut_cutouts::change_cutout_rod_resistance;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::insert_fastener;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::remove_fastener;
pub use semio_s_artifact_norm_en1995_mut_fastener_links::change_fastener_connection;
pub use semio_s_artifact_norm_en1995_mut_fastener_links::change_fastener_member;
pub use semio_s_artifact_norm_en1995_mut_fastener_links::change_fastener_columns;
pub use semio_s_artifact_norm_en1995_mut_fastener_links::change_fastener_lines;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_spacing_par;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_spacing_perp;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_end_distance;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_edge_distance;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_offset_x;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_offset_y;
pub use semio_s_artifact_norm_en1995_mut_fastener_distances::change_fastener_load_angle;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::insert_panel;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::remove_panel;
pub use semio_s_artifact_norm_en1995_mut_panels::change_panel_strength_class;
pub use semio_s_artifact_norm_en1995_mut_panels::change_panel_span;
pub use semio_s_artifact_norm_en1995_mut_panels::change_panel_width;
pub use semio_s_artifact_norm_en1995_mut_panels::change_panel_load;
pub use semio_s_artifact_norm_en1995_mut_panels::change_panel_fire_duration;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::insert_floor;
pub use semio_s_artifact_norm_en1995_mut_entity_rows::remove_floor;
pub use semio_s_artifact_norm_en1995_mut_floor_stiffness::change_floor_member;
pub use semio_s_artifact_norm_en1995_mut_floor_stiffness::change_floor_span;
pub use semio_s_artifact_norm_en1995_mut_floor_stiffness::change_floor_width;
pub use semio_s_artifact_norm_en1995_mut_floor_stiffness::change_floor_joist_spacing;
pub use semio_s_artifact_norm_en1995_mut_floor_stiffness::change_floor_ei_l;
pub use semio_s_artifact_norm_en1995_mut_floor_stiffness::change_floor_ei_b;
pub use semio_s_artifact_norm_en1995_mut_floor_loads::change_floor_mass;
pub use semio_s_artifact_norm_en1995_mut_floor_loads::change_floor_damping;
pub use semio_s_artifact_norm_en1995_mut_floor_loads::change_floor_permanent_load;
pub use semio_s_artifact_norm_en1995_mut_floor_loads::change_floor_imposed_load;
pub use semio_s_artifact_norm_en1995_mut_floor_loads::change_floor_psi_2;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1995Snapshot, diff = En1995Diff, schema = "s.norm-eurocode-materials.en1995", reference = En1995Snapshot, reference_diff = En1995Diff)]
pub enum En1995Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass),
    InsertMember(insert_member::InsertMember),
    RemoveMember(remove_member::RemoveMember),
    ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn),
    ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe),
    ChangeMemberRole(change_member_role::ChangeMemberRole),
    ChangeMemberStrengthClass(change_member_strength_class::ChangeMemberStrengthClass),
    ChangeMemberServiceClass(change_member_service_class::ChangeMemberServiceClass),
    ChangeMemberSupport(change_member_support::ChangeMemberSupport),
    ChangeMemberB(change_member_b::ChangeMemberB),
    ChangeMemberH(change_member_h::ChangeMemberH),
    ChangeMemberSpan(change_member_span::ChangeMemberSpan),
    ChangeMemberSupportLength(change_member_support_length::ChangeMemberSupportLength),
    ChangeMemberBearingLength(change_member_bearing_length::ChangeMemberBearingLength),
    ChangeMemberBucklingLengthY(change_member_buckling_length_y::ChangeMemberBucklingLengthY),
    ChangeMemberBucklingLengthZ(change_member_buckling_length_z::ChangeMemberBucklingLengthZ),
    ChangeMemberRestraintSpacing(change_member_restraint_spacing::ChangeMemberRestraintSpacing),
    ChangeMemberNotchDepth(change_member_notch_depth::ChangeMemberNotchDepth),
    ChangeMemberNotchDistance(change_member_notch_distance::ChangeMemberNotchDistance),
    ChangeMemberMassKgPerM(change_member_mass_kg_per_m::ChangeMemberMassKgPerM),
    ChangeMemberMassKgPerM2(change_member_mass_kg_per_m2::ChangeMemberMassKgPerM2),
    ChangeMemberDampingXi(change_member_damping_xi::ChangeMemberDampingXi),
    ChangeMemberFireDuration(change_member_fire_duration::ChangeMemberFireDuration),
    ChangeMemberBridgeNObs(change_member_bridge_n_obs::ChangeMemberBridgeNObs),
    ChangeMemberBridgeTLYears(change_member_bridge_tl_years::ChangeMemberBridgeTLYears),
    ChangeMemberBridgeBeta(change_member_bridge_beta::ChangeMemberBridgeBeta),
    ChangeMemberBridgeA(change_member_bridge_a::ChangeMemberBridgeA),
    ChangeMemberBridgeB(change_member_bridge_b::ChangeMemberBridgeB),
    ChangeMemberBridgeCrowd(change_member_bridge_crowd::ChangeMemberBridgeCrowd),
    InsertMemberAction(insert_member_action::InsertMemberAction),
    RemoveMemberAction(remove_member_action::RemoveMemberAction),
    ChangeMemberActionKind(change_member_action_kind::ChangeMemberActionKind),
    ChangeMemberActionCategory(change_member_action_category::ChangeMemberActionCategory),
    ChangeMemberLoadDuration(change_member_load_duration::ChangeMemberLoadDuration),
    ChangeMemberActionQLine(change_member_action_q_line::ChangeMemberActionQLine),
    ChangeMemberActionFPoint(change_member_action_f_point::ChangeMemberActionFPoint),
    ChangeMemberActionMK(change_member_action_mk::ChangeMemberActionMK),
    ChangeMemberActionVK(change_member_action_vk::ChangeMemberActionVK),
    ChangeMemberActionNK(change_member_action_nk::ChangeMemberActionNK),
    ChangeMemberActionNTK(change_member_action_ntk::ChangeMemberActionNTK),
    ChangeMemberActionFC90K(change_member_action_fc90_k::ChangeMemberActionFC90K),
    InsertConnection(insert_connection::InsertConnection),
    RemoveConnection(remove_connection::RemoveConnection),
    ChangeConnectionLabelEn(change_connection_label_en::ChangeConnectionLabelEn),
    ChangeConnectionLabelDe(change_connection_label_de::ChangeConnectionLabelDe),
    ChangeConnectionFastenerType(change_connection_fastener_type::ChangeConnectionFastenerType),
    ChangeConnectionStrengthClass(change_connection_strength_class::ChangeConnectionStrengthClass),
    ChangeConnectionServiceClass(change_connection_service_class::ChangeConnectionServiceClass),
    ChangeConnectionDiameter(change_connection_diameter::ChangeConnectionDiameter),
    ChangeConnectionNumber(change_connection_number::ChangeConnectionNumber),
    ChangeConnectionRows(change_connection_rows::ChangeConnectionRows),
    ChangeConnectionSpacing(change_connection_spacing::ChangeConnectionSpacing),
    ChangeConnectionEdgeDistance(change_connection_edge_distance::ChangeConnectionEdgeDistance),
    ChangeConnectionEndDistance(change_connection_end_distance::ChangeConnectionEndDistance),
    ChangeConnectionT1(change_connection_t1::ChangeConnectionT1),
    ChangeConnectionT2(change_connection_t2::ChangeConnectionT2),
    ChangeConnectionSteelPlate(change_connection_steel_plate::ChangeConnectionSteelPlate),
    ChangeConnectionPlateThickness(change_connection_plate_thickness::ChangeConnectionPlateThickness),
    ChangeConnectionShearPlanes(change_connection_shear_planes::ChangeConnectionShearPlanes),
    ChangeConnectionFUK(change_connection_fuk::ChangeConnectionFUK),
    InsertConnectionAction(insert_connection_action::InsertConnectionAction),
    RemoveConnectionAction(remove_connection_action::RemoveConnectionAction),
    ChangeConnectionActionKind(change_connection_action_kind::ChangeConnectionActionKind),
    ChangeConnectionLoadDuration(change_connection_load_duration::ChangeConnectionLoadDuration),
    ChangeConnectionActionFK(change_connection_action_fk::ChangeConnectionActionFK),
    ChangeUlsEquation(change_uls_equation::ChangeUlsEquation),
    InsertCase(insert_case::InsertCase),
    RemoveCase(remove_case::RemoveCase),
    ChangeCaseLabelEn(change_case_label_en::ChangeCaseLabelEn),
    ChangeCaseLabelDe(change_case_label_de::ChangeCaseLabelDe),
    ChangeCaseEntryId(change_case_entry_id::ChangeCaseEntryId),
    ChangeCaseNumber(change_case_number::ChangeCaseNumber),
    ChangeCaseText(change_case_text::ChangeCaseText),
    ChangePlacementX(change_placement_x::ChangePlacementX),
    ChangePlacementY(change_placement_y::ChangePlacementY),
    ChangePlacementZ(change_placement_z::ChangePlacementZ),
    ChangePlacementYaw(change_placement_yaw::ChangePlacementYaw),
    InsertPlacement(insert_placement::InsertPlacement),
    RemovePlacement(remove_placement::RemovePlacement),
    InsertDeck(insert_deck::InsertDeck),
    RemoveDeck(remove_deck::RemoveDeck),
    ChangeDeckLaminationProduct(change_deck_lamination_product::ChangeDeckLaminationProduct),
    ChangeDeckBarProduct(change_deck_bar_product::ChangeDeckBarProduct),
    ChangeDeckSpan(change_deck_span::ChangeDeckSpan),
    ChangeDeckDeckWidth(change_deck_deck_width::ChangeDeckDeckWidth),
    ChangeDeckDeckDepth(change_deck_deck_depth::ChangeDeckDeckDepth),
    ChangeDeckLaminationWidth(change_deck_lamination_width::ChangeDeckLaminationWidth),
    ChangeDeckBarSpacing(change_deck_bar_spacing::ChangeDeckBarSpacing),
    ChangeDeckBarDiameter(change_deck_bar_diameter::ChangeDeckBarDiameter),
    ChangeDeckPlateSide(change_deck_plate_side::ChangeDeckPlateSide),
    ChangeDeckInitialPrestress(change_deck_initial_prestress::ChangeDeckInitialPrestress),
    ChangeDeckLossRelaxation(change_deck_loss_relaxation::ChangeDeckLossRelaxation),
    ChangeDeckLossCreep(change_deck_loss_creep::ChangeDeckLossCreep),
    ChangeDeckLossMoisture(change_deck_loss_moisture::ChangeDeckLossMoisture),
    ChangeDeckBarResistance(change_deck_bar_resistance::ChangeDeckBarResistance),
    ChangeDeckShearStress(change_deck_shear_stress::ChangeDeckShearStress),
    ChangeDeckCompressionC90(change_deck_compression_c90::ChangeDeckCompressionC90),
    ChangeDeckWheelLoad(change_deck_wheel_load::ChangeDeckWheelLoad),
    ChangeDeckWheelContact(change_deck_wheel_contact::ChangeDeckWheelContact),
    InsertDiaphragm(insert_diaphragm::InsertDiaphragm),
    RemoveDiaphragm(remove_diaphragm::RemoveDiaphragm),
    ChangeDiaphragmKind(change_diaphragm_kind::ChangeDiaphragmKind),
    ChangeDiaphragmMethod(change_diaphragm_method::ChangeDiaphragmMethod),
    ChangeDiaphragmPanelWidth(change_diaphragm_panel_width::ChangeDiaphragmPanelWidth),
    ChangeDiaphragmWallHeight(change_diaphragm_wall_height::ChangeDiaphragmWallHeight),
    ChangeDiaphragmFastenerSpacing(change_diaphragm_fastener_spacing::ChangeDiaphragmFastenerSpacing),
    ChangeDiaphragmPanels(change_diaphragm_panels::ChangeDiaphragmPanels),
    ChangeDiaphragmFastenerCapacity(change_diaphragm_fastener_capacity::ChangeDiaphragmFastenerCapacity),
    ChangeDiaphragmShearEd(change_diaphragm_shear_ed::ChangeDiaphragmShearEd),
    ChangeDiaphragmHoldDown(change_diaphragm_hold_down::ChangeDiaphragmHoldDown),
    ChangeDiaphragmForce(change_diaphragm_force::ChangeDiaphragmForce),
    ChangeDiaphragmLength(change_diaphragm_length::ChangeDiaphragmLength),
    ChangeDiaphragmShearResistance(change_diaphragm_shear_resistance::ChangeDiaphragmShearResistance),
    InsertTruss(insert_truss::InsertTruss),
    RemoveTruss(remove_truss::RemoveTruss),
    ChangeTrussKind(change_truss_kind::ChangeTrussKind),
    ChangeTrussProduct(change_truss_product::ChangeTrussProduct),
    ChangeTrussSpan(change_truss_span::ChangeTrussSpan),
    ChangeTrussHeight(change_truss_height::ChangeTrussHeight),
    ChangeTrussBays(change_truss_bays::ChangeTrussBays),
    ChangeTrussChordWidth(change_truss_chord_width::ChangeTrussChordWidth),
    ChangeTrussChordDepth(change_truss_chord_depth::ChangeTrussChordDepth),
    ChangeTrussWebWidth(change_truss_web_width::ChangeTrussWebWidth),
    ChangeTrussWebDepth(change_truss_web_depth::ChangeTrussWebDepth),
    ChangeTrussChordForce(change_truss_chord_force::ChangeTrussChordForce),
    ChangeTrussRestraintLength(change_truss_restraint_length::ChangeTrussRestraintLength),
    ChangeTrussBracingCapacity(change_truss_bracing_capacity::ChangeTrussBracingCapacity),
    InsertBracing(insert_bracing::InsertBracing),
    RemoveBracing(remove_bracing::RemoveBracing),
    ChangeBracingKind(change_bracing_kind::ChangeBracingKind),
    ChangeBracingMember(change_bracing_member::ChangeBracingMember),
    ChangeBracingStabilisedCount(change_bracing_stabilised_count::ChangeBracingStabilisedCount),
    ChangeBracingSpacing(change_bracing_spacing::ChangeBracingSpacing),
    ChangeBracingLength(change_bracing_length::ChangeBracingLength),
    ChangeBracingBraceWidth(change_bracing_brace_width::ChangeBracingBraceWidth),
    ChangeBracingBraceDepth(change_bracing_brace_depth::ChangeBracingBraceDepth),
    ChangeBracingCompression(change_bracing_compression::ChangeBracingCompression),
    ChangeBracingResistance(change_bracing_resistance::ChangeBracingResistance),
    ChangeBracingStiffness(change_bracing_stiffness::ChangeBracingStiffness),
    InsertPanelLayer(insert_panel_layer::InsertPanelLayer),
    RemovePanelLayer(remove_panel_layer::RemovePanelLayer),
    ChangePanelLayerThickness(change_panel_layer_thickness::ChangePanelLayerThickness),
    ChangePanelLayerOrientation(change_panel_layer_orientation::ChangePanelLayerOrientation),
    ChangePanelLayerGrade(change_panel_layer_grade::ChangePanelLayerGrade),
    InsertFastenerPosition(insert_fastener_position::InsertFastenerPosition),
    RemoveFastenerPosition(remove_fastener_position::RemoveFastenerPosition),
    ChangeFastenerPositionX(change_fastener_position_x::ChangeFastenerPositionX),
    ChangeFastenerPositionY(change_fastener_position_y::ChangeFastenerPositionY),
    InsertTaper(insert_taper::InsertTaper),
    RemoveTaper(remove_taper::RemoveTaper),
    ChangeTaperMember(change_taper_member::ChangeTaperMember),
    ChangeTaperKind(change_taper_kind::ChangeTaperKind),
    ChangeTaperTaperAngle(change_taper_taper_angle::ChangeTaperTaperAngle),
    ChangeTaperApexDepth(change_taper_apex_depth::ChangeTaperApexDepth),
    ChangeTaperInnerRadius(change_taper_inner_radius::ChangeTaperInnerRadius),
    ChangeTaperLamella(change_taper_lamella::ChangeTaperLamella),
    ChangeTaperApexAngle(change_taper_apex_angle::ChangeTaperApexAngle),
    ChangeTaperSide(change_taper_side::ChangeTaperSide),
    ChangeTaperRods(change_taper_rods::ChangeTaperRods),
    ChangeTaperRodResistance(change_taper_rod_resistance::ChangeTaperRodResistance),
    InsertCutout(insert_cutout::InsertCutout),
    RemoveCutout(remove_cutout::RemoveCutout),
    ChangeCutoutMember(change_cutout_member::ChangeCutoutMember),
    ChangeCutoutKind(change_cutout_kind::ChangeCutoutKind),
    ChangeCutoutFace(change_cutout_face::ChangeCutoutFace),
    ChangeCutoutDistance(change_cutout_distance::ChangeCutoutDistance),
    ChangeCutoutLength(change_cutout_length::ChangeCutoutLength),
    ChangeCutoutDepth(change_cutout_depth::ChangeCutoutDepth),
    ChangeCutoutCentreHeight(change_cutout_centre_height::ChangeCutoutCentreHeight),
    ChangeCutoutSlope(change_cutout_slope::ChangeCutoutSlope),
    ChangeCutoutRods(change_cutout_rods::ChangeCutoutRods),
    ChangeCutoutRodResistance(change_cutout_rod_resistance::ChangeCutoutRodResistance),
    InsertFastener(insert_fastener::InsertFastener),
    RemoveFastener(remove_fastener::RemoveFastener),
    ChangeFastenerConnection(change_fastener_connection::ChangeFastenerConnection),
    ChangeFastenerMember(change_fastener_member::ChangeFastenerMember),
    ChangeFastenerColumns(change_fastener_columns::ChangeFastenerColumns),
    ChangeFastenerLines(change_fastener_lines::ChangeFastenerLines),
    ChangeFastenerSpacingPar(change_fastener_spacing_par::ChangeFastenerSpacingPar),
    ChangeFastenerSpacingPerp(change_fastener_spacing_perp::ChangeFastenerSpacingPerp),
    ChangeFastenerEndDistance(change_fastener_end_distance::ChangeFastenerEndDistance),
    ChangeFastenerEdgeDistance(change_fastener_edge_distance::ChangeFastenerEdgeDistance),
    ChangeFastenerOffsetX(change_fastener_offset_x::ChangeFastenerOffsetX),
    ChangeFastenerOffsetY(change_fastener_offset_y::ChangeFastenerOffsetY),
    ChangeFastenerLoadAngle(change_fastener_load_angle::ChangeFastenerLoadAngle),
    InsertPanel(insert_panel::InsertPanel),
    RemovePanel(remove_panel::RemovePanel),
    ChangePanelStrengthClass(change_panel_strength_class::ChangePanelStrengthClass),
    ChangePanelSpan(change_panel_span::ChangePanelSpan),
    ChangePanelWidth(change_panel_width::ChangePanelWidth),
    ChangePanelLoad(change_panel_load::ChangePanelLoad),
    ChangePanelFireDuration(change_panel_fire_duration::ChangePanelFireDuration),
    InsertFloor(insert_floor::InsertFloor),
    RemoveFloor(remove_floor::RemoveFloor),
    ChangeFloorMember(change_floor_member::ChangeFloorMember),
    ChangeFloorSpan(change_floor_span::ChangeFloorSpan),
    ChangeFloorWidth(change_floor_width::ChangeFloorWidth),
    ChangeFloorJoistSpacing(change_floor_joist_spacing::ChangeFloorJoistSpacing),
    ChangeFloorEiL(change_floor_ei_l::ChangeFloorEiL),
    ChangeFloorEiB(change_floor_ei_b::ChangeFloorEiB),
    ChangeFloorMass(change_floor_mass::ChangeFloorMass),
    ChangeFloorDamping(change_floor_damping::ChangeFloorDamping),
    ChangeFloorPermanentLoad(change_floor_permanent_load::ChangeFloorPermanentLoad),
    ChangeFloorImposedLoad(change_floor_imposed_load::ChangeFloorImposedLoad),
    ChangeFloorPsi2(change_floor_psi_2::ChangeFloorPsi2),
}

/// 🏷️ Every kind in `#[derive(dsl::Mutations)]` declaration order — pinned by `kinds_match_the_enum_and_the_catalog`.
pub const KINDS: &[&str] = &[
    "change-annex",
    "change-reliability-class",
    "insert-member",
    "remove-member",
    "change-member-label-en",
    "change-member-label-de",
    "change-member-role",
    "change-member-strength-class",
    "change-member-service-class",
    "change-member-support",
    "change-member-b",
    "change-member-h",
    "change-member-span",
    "change-member-support-length",
    "change-member-bearing-length",
    "change-member-buckling-length-y",
    "change-member-buckling-length-z",
    "change-member-restraint-spacing",
    "change-member-notch-depth",
    "change-member-notch-distance",
    "change-member-mass-kg-per-m",
    "change-member-mass-kg-per-m2",
    "change-member-damping-xi",
    "change-member-fire-duration",
    "change-member-bridge-n-obs",
    "change-member-bridge-tl-years",
    "change-member-bridge-beta",
    "change-member-bridge-a",
    "change-member-bridge-b",
    "change-member-bridge-crowd",
    "insert-member-action",
    "remove-member-action",
    "change-member-action-kind",
    "change-member-action-category",
    "change-member-load-duration",
    "change-member-action-q-line",
    "change-member-action-f-point",
    "change-member-action-mk",
    "change-member-action-vk",
    "change-member-action-nk",
    "change-member-action-ntk",
    "change-member-action-fc90-k",
    "insert-connection",
    "remove-connection",
    "change-connection-label-en",
    "change-connection-label-de",
    "change-connection-fastener-type",
    "change-connection-strength-class",
    "change-connection-service-class",
    "change-connection-diameter",
    "change-connection-number",
    "change-connection-rows",
    "change-connection-spacing",
    "change-connection-edge-distance",
    "change-connection-end-distance",
    "change-connection-t1",
    "change-connection-t2",
    "change-connection-steel-plate",
    "change-connection-plate-thickness",
    "change-connection-shear-planes",
    "change-connection-fuk",
    "insert-connection-action",
    "remove-connection-action",
    "change-connection-action-kind",
    "change-connection-load-duration",
    "change-connection-action-fk",
    "change-uls-equation",
    "insert-case",
    "remove-case",
    "change-case-label-en",
    "change-case-label-de",
    "change-case-entry-id",
    "change-case-number",
    "change-case-text",
    "change-placement-x",
    "change-placement-y",
    "change-placement-z",
    "change-placement-yaw",
    "insert-placement",
    "remove-placement",
    "insert-deck",
    "remove-deck",
    "change-deck-lamination-product",
    "change-deck-bar-product",
    "change-deck-span",
    "change-deck-deck-width",
    "change-deck-deck-depth",
    "change-deck-lamination-width",
    "change-deck-bar-spacing",
    "change-deck-bar-diameter",
    "change-deck-plate-side",
    "change-deck-initial-prestress",
    "change-deck-loss-relaxation",
    "change-deck-loss-creep",
    "change-deck-loss-moisture",
    "change-deck-bar-resistance",
    "change-deck-shear-stress",
    "change-deck-compression-c90",
    "change-deck-wheel-load",
    "change-deck-wheel-contact",
    "insert-diaphragm",
    "remove-diaphragm",
    "change-diaphragm-kind",
    "change-diaphragm-method",
    "change-diaphragm-panel-width",
    "change-diaphragm-wall-height",
    "change-diaphragm-fastener-spacing",
    "change-diaphragm-panels",
    "change-diaphragm-fastener-capacity",
    "change-diaphragm-shear-ed",
    "change-diaphragm-hold-down",
    "change-diaphragm-force",
    "change-diaphragm-length",
    "change-diaphragm-shear-resistance",
    "insert-truss",
    "remove-truss",
    "change-truss-kind",
    "change-truss-product",
    "change-truss-span",
    "change-truss-height",
    "change-truss-bays",
    "change-truss-chord-width",
    "change-truss-chord-depth",
    "change-truss-web-width",
    "change-truss-web-depth",
    "change-truss-chord-force",
    "change-truss-restraint-length",
    "change-truss-bracing-capacity",
    "insert-bracing",
    "remove-bracing",
    "change-bracing-kind",
    "change-bracing-member",
    "change-bracing-stabilised-count",
    "change-bracing-spacing",
    "change-bracing-length",
    "change-bracing-brace-width",
    "change-bracing-brace-depth",
    "change-bracing-compression",
    "change-bracing-resistance",
    "change-bracing-stiffness",
    "insert-panel-layer",
    "remove-panel-layer",
    "change-panel-layer-thickness",
    "change-panel-layer-orientation",
    "change-panel-layer-grade",
    "insert-fastener-position",
    "remove-fastener-position",
    "change-fastener-position-x",
    "change-fastener-position-y",
    "insert-taper",
    "remove-taper",
    "change-taper-member",
    "change-taper-kind",
    "change-taper-taper-angle",
    "change-taper-apex-depth",
    "change-taper-inner-radius",
    "change-taper-lamella",
    "change-taper-apex-angle",
    "change-taper-side",
    "change-taper-rods",
    "change-taper-rod-resistance",
    "insert-cutout",
    "remove-cutout",
    "change-cutout-member",
    "change-cutout-kind",
    "change-cutout-face",
    "change-cutout-distance",
    "change-cutout-length",
    "change-cutout-depth",
    "change-cutout-centre-height",
    "change-cutout-slope",
    "change-cutout-rods",
    "change-cutout-rod-resistance",
    "insert-fastener",
    "remove-fastener",
    "change-fastener-connection",
    "change-fastener-member",
    "change-fastener-columns",
    "change-fastener-lines",
    "change-fastener-spacing-par",
    "change-fastener-spacing-perp",
    "change-fastener-end-distance",
    "change-fastener-edge-distance",
    "change-fastener-offset-x",
    "change-fastener-offset-y",
    "change-fastener-load-angle",
    "insert-panel",
    "remove-panel",
    "change-panel-strength-class",
    "change-panel-span",
    "change-panel-width",
    "change-panel-load",
    "change-panel-fire-duration",
    "insert-floor",
    "remove-floor",
    "change-floor-member",
    "change-floor-span",
    "change-floor-width",
    "change-floor-joist-spacing",
    "change-floor-ei-l",
    "change-floor-ei-b",
    "change-floor-mass",
    "change-floor-damping",
    "change-floor-permanent-load",
    "change-floor-imposed-load",
    "change-floor-psi2",
];
//#endregion 🔖️Mutations

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1995_mutations_typescript::LEAF, graphql: semio_s_artifact_norm_en1995_mutations_graphql::LEAF, json_schema: semio_s_artifact_norm_en1995_mutations_json_schema::LEAF, proto: semio_s_artifact_norm_en1995_mutations_proto::LEAF };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1995Mutation;
    use semio_s_artifact_norm_en1995_model::En1995Snapshot;

/// 🧮️ Applies `mutation` to `base`, returning the next snapshot and every diagnostic the mutation raised.
pub fn apply_en1995_mutation(base: &En1995Snapshot, mutation: &En1995Mutation) -> Result<(En1995Snapshot, Vec<String>), String> {
    let outcome = <En1995Mutation as protocol::ColdMutation<En1995Snapshot>>::diff(mutation, base);
    let messages = outcome.messages().iter().map(|message| format!("{message:?}")).collect();
    let next = protocol::apply_diff_cold(outcome.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((next, messages))
}
/// ↩️ The mutation list that undoes `mutation` applied to `base`.
pub fn inverse_en1995_mutation(mutation: &En1995Mutation, base: &En1995Snapshot) -> Result<Vec<En1995Mutation>, semio_framework_value::ValueError> {
    Ok({
    <En1995Mutation as protocol::ColdMutation<En1995Snapshot>>::inverse(mutation, base)?

    })
}
}
pub use crate::mutation_bridge::*;

#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
pub mod binary;
