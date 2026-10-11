//! 🧬️ En1995 artifact — document mutation dispatch for the hierarchical timber subject
//! (members ⊃ characteristic actions, connections ⊃ connection actions).

use crate::{En1995Diff, En1995Snapshot};

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::EDIT_RULES;

//#region 🔖️Mutations
use super::change_annex;
use super::insert_member;
use super::remove_member;
use super::change_member_label_en;
use super::change_member_label_de;
use super::change_member_role;
use super::change_member_strength_class;
use super::change_member_service_class;
use super::change_member_support;
use super::change_member_b;
use super::change_member_h;
use super::change_member_span;
use super::change_member_support_length;
use super::change_member_bearing_length;
use super::change_member_buckling_length_y;
use super::change_member_buckling_length_z;
use super::change_member_restraint_spacing;
use super::change_member_notch_depth;
use super::change_member_notch_distance;
use super::change_member_m_crit;
use super::change_member_mass_kg_per_m;
use super::change_member_mass_kg_per_m2;
use super::change_member_damping_xi;
use super::change_member_fire_duration;
use super::change_member_bridge_n_obs;
use super::change_member_bridge_tl_years;
use super::change_member_bridge_beta;
use super::change_member_bridge_a;
use super::change_member_bridge_b;
use super::change_member_bridge_crowd;
use super::insert_member_action;
use super::remove_member_action;
use super::change_member_action_kind;
use super::change_member_action_category;
use super::change_member_load_duration;
use super::change_member_action_q_line;
use super::change_member_action_f_point;
use super::change_member_action_mk;
use super::change_member_action_vk;
use super::change_member_action_nk;
use super::change_member_action_ntk;
use super::change_member_action_fc90_k;
use super::insert_connection;
use super::remove_connection;
use super::change_connection_label_en;
use super::change_connection_label_de;
use super::change_connection_fastener_type;
use super::change_connection_strength_class;
use super::change_connection_service_class;
use super::change_connection_diameter;
use super::change_connection_number;
use super::change_connection_rows;
use super::change_connection_spacing;
use super::change_connection_edge_distance;
use super::change_connection_end_distance;
use super::change_connection_t1;
use super::change_connection_t2;
use super::change_connection_steel_plate;
use super::change_connection_plate_thickness;
use super::change_connection_shear_planes;
use super::change_connection_fuk;
use super::insert_connection_action;
use super::remove_connection_action;
use super::change_connection_action_kind;
use super::change_connection_load_duration;
use super::change_connection_action_fk;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1995Snapshot, diff = En1995Diff, schema = "s.norm.en1995")]
pub enum En1995Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
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
    ChangeMemberMCrit(change_member_m_crit::ChangeMemberMCrit),
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
}

/// 🏷️ Every kind in `#[derive(dsl::Mutations)]` declaration order — pinned by `kinds_match_the_enum_and_the_catalog`.
pub const KINDS: &[&str] = &[
    "change-annex",
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
    "change-member-m-crit",
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
];
//#endregion 🔖️Mutations


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;


//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
//#endregion 🧫️Vectors
