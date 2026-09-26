//! 🧬️ En1995 artifact — document mutation dispatch for the hierarchical timber subject
//! (members ⊃ characteristic actions, connections ⊃ connection actions).

use crate::{En1995Diff, En1995Snapshot};

//#region 🔖️Mutations
use super::set_snapshot;
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
use super::change_member_buckling_y;
use super::change_member_buckling_z;
use super::change_member_lateral_restraint;
use super::change_member_notch_depth;
use super::change_member_notch_distance;
use super::change_member_m_crit;
use super::change_member_mass_per_m;
use super::change_member_mass_per_m2;
use super::change_member_damping;
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
use super::change_member_action_load_duration;
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
use super::change_connection_steel_plate_thickness;
use super::change_connection_shear_planes;
use super::change_connection_fuk;
use super::insert_connection_action;
use super::remove_connection_action;
use super::change_connection_action_kind;
use super::change_connection_action_load_duration;
use super::change_connection_action_fk;

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1995Snapshot, diff = En1995Diff, schema = "s.norm.en1995")]
pub enum En1995Mutation {
    ChangeAnnex(set_snapshot::ChangeAnnex),
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
    ChangeMemberBucklingY(change_member_buckling_y::ChangeMemberBucklingY),
    ChangeMemberBucklingZ(change_member_buckling_z::ChangeMemberBucklingZ),
    ChangeMemberLateralRestraint(change_member_lateral_restraint::ChangeMemberLateralRestraint),
    ChangeMemberNotchDepth(change_member_notch_depth::ChangeMemberNotchDepth),
    ChangeMemberNotchDistance(change_member_notch_distance::ChangeMemberNotchDistance),
    ChangeMemberMCrit(change_member_m_crit::ChangeMemberMCrit),
    ChangeMemberMassPerM(change_member_mass_per_m::ChangeMemberMassPerM),
    ChangeMemberMassPerM2(change_member_mass_per_m2::ChangeMemberMassPerM2),
    ChangeMemberDamping(change_member_damping::ChangeMemberDamping),
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
    ChangeMemberActionLoadDuration(change_member_action_load_duration::ChangeMemberActionLoadDuration),
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
    ChangeConnectionSteelPlateThickness(change_connection_steel_plate_thickness::ChangeConnectionSteelPlateThickness),
    ChangeConnectionShearPlanes(change_connection_shear_planes::ChangeConnectionShearPlanes),
    ChangeConnectionFUK(change_connection_fuk::ChangeConnectionFUK),
    InsertConnectionAction(insert_connection_action::InsertConnectionAction),
    RemoveConnectionAction(remove_connection_action::RemoveConnectionAction),
    ChangeConnectionActionKind(change_connection_action_kind::ChangeConnectionActionKind),
    ChangeConnectionActionLoadDuration(change_connection_action_load_duration::ChangeConnectionActionLoadDuration),
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
    "change-member-buckling-y",
    "change-member-buckling-z",
    "change-member-lateral-restraint",
    "change-member-notch-depth",
    "change-member-notch-distance",
    "change-member-m-crit",
    "change-member-mass-per-m",
    "change-member-mass-per-m2",
    "change-member-damping",
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
    "change-member-action-load-duration",
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
    "change-connection-steel-plate-thickness",
    "change-connection-shear-planes",
    "change-connection-fuk",
    "insert-connection-action",
    "remove-connection-action",
    "change-connection-action-kind",
    "change-connection-action-load-duration",
    "change-connection-action-fk",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1995Mutation {
    /// 🔀 Derives the ordered mutation list carrying `base` to `target` — removals (reverse index order),
    /// insertions, then one `change-*` per differing scalar, recursing into each item's action table.
    pub fn from_snapshot(base: &En1995Snapshot, target: &En1995Snapshot) -> Vec<Self> {
        let mut out = Vec::new();
        if base.annex != target.annex {
            out.push(En1995Mutation::ChangeAnnex(set_snapshot::ChangeAnnex { new_annex: target.annex }));
        }
        if base.members != target.members {
            let base_ids: std::collections::BTreeSet<_> = base.members.iter().map(|m| m.id.clone()).collect();
            let target_ids: std::collections::BTreeSet<_> = target.members.iter().map(|m| m.id.clone()).collect();
            for (index, m) in base.members.iter().enumerate().rev() {
                if !target_ids.contains(&m.id) {
                    out.push(En1995Mutation::RemoveMember(remove_member::RemoveMember { index }));
                }
            }
            for (index, m) in target.members.iter().enumerate() {
                if !base_ids.contains(&m.id) {
                    out.push(En1995Mutation::InsertMember(insert_member::InsertMember { index, member: m.clone() }));
                }
            }
            for t in &target.members {
                if let Some(b) = base.members.iter().find(|m| m.id == t.id) {
                    if b.label_en != t.label_en {
                        out.push(En1995Mutation::ChangeMemberLabelEn(change_member_label_en::ChangeMemberLabelEn { member_id: t.id.clone(), new_value: t.label_en.clone() }));
                    }
                    if b.label_de != t.label_de {
                        out.push(En1995Mutation::ChangeMemberLabelDe(change_member_label_de::ChangeMemberLabelDe { member_id: t.id.clone(), new_value: t.label_de.clone() }));
                    }
                    if b.role != t.role {
                        out.push(En1995Mutation::ChangeMemberRole(change_member_role::ChangeMemberRole { member_id: t.id.clone(), new_value: t.role }));
                    }
                    if b.strength_class != t.strength_class {
                        out.push(En1995Mutation::ChangeMemberStrengthClass(change_member_strength_class::ChangeMemberStrengthClass { member_id: t.id.clone(), new_value: t.strength_class.clone() }));
                    }
                    if b.service_class != t.service_class {
                        out.push(En1995Mutation::ChangeMemberServiceClass(change_member_service_class::ChangeMemberServiceClass { member_id: t.id.clone(), new_value: t.service_class }));
                    }
                    if b.support != t.support {
                        out.push(En1995Mutation::ChangeMemberSupport(change_member_support::ChangeMemberSupport { member_id: t.id.clone(), new_value: t.support }));
                    }
                    if (b.b_m - t.b_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberB(change_member_b::ChangeMemberB { member_id: t.id.clone(), new_value: t.b_m }));
                    }
                    if (b.h_m - t.h_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberH(change_member_h::ChangeMemberH { member_id: t.id.clone(), new_value: t.h_m }));
                    }
                    if (b.span_m - t.span_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberSpan(change_member_span::ChangeMemberSpan { member_id: t.id.clone(), new_value: t.span_m }));
                    }
                    if (b.support_length_m - t.support_length_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberSupportLength(change_member_support_length::ChangeMemberSupportLength { member_id: t.id.clone(), new_value: t.support_length_m }));
                    }
                    if (b.bearing_length_m - t.bearing_length_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBearingLength(change_member_bearing_length::ChangeMemberBearingLength { member_id: t.id.clone(), new_value: t.bearing_length_m }));
                    }
                    if (b.buckling_length_y_m - t.buckling_length_y_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBucklingY(change_member_buckling_y::ChangeMemberBucklingY { member_id: t.id.clone(), new_value: t.buckling_length_y_m }));
                    }
                    if (b.buckling_length_z_m - t.buckling_length_z_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBucklingZ(change_member_buckling_z::ChangeMemberBucklingZ { member_id: t.id.clone(), new_value: t.buckling_length_z_m }));
                    }
                    if (b.lateral_restraint_spacing_m - t.lateral_restraint_spacing_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberLateralRestraint(change_member_lateral_restraint::ChangeMemberLateralRestraint { member_id: t.id.clone(), new_value: t.lateral_restraint_spacing_m }));
                    }
                    if (b.notch_depth_m - t.notch_depth_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberNotchDepth(change_member_notch_depth::ChangeMemberNotchDepth { member_id: t.id.clone(), new_value: t.notch_depth_m }));
                    }
                    if (b.notch_distance_m - t.notch_distance_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberNotchDistance(change_member_notch_distance::ChangeMemberNotchDistance { member_id: t.id.clone(), new_value: t.notch_distance_m }));
                    }
                    if (b.m_crit_nm - t.m_crit_nm).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberMCrit(change_member_m_crit::ChangeMemberMCrit { member_id: t.id.clone(), new_value: t.m_crit_nm }));
                    }
                    if (b.mass_kg_per_m - t.mass_kg_per_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberMassPerM(change_member_mass_per_m::ChangeMemberMassPerM { member_id: t.id.clone(), new_value: t.mass_kg_per_m }));
                    }
                    if (b.mass_kg_per_m2 - t.mass_kg_per_m2).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberMassPerM2(change_member_mass_per_m2::ChangeMemberMassPerM2 { member_id: t.id.clone(), new_value: t.mass_kg_per_m2 }));
                    }
                    if (b.damping_xi - t.damping_xi).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberDamping(change_member_damping::ChangeMemberDamping { member_id: t.id.clone(), new_value: t.damping_xi }));
                    }
                    if (b.fire_duration_s - t.fire_duration_s).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberFireDuration(change_member_fire_duration::ChangeMemberFireDuration { member_id: t.id.clone(), new_value: t.fire_duration_s }));
                    }
                    if (b.bridge_n_obs - t.bridge_n_obs).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBridgeNObs(change_member_bridge_n_obs::ChangeMemberBridgeNObs { member_id: t.id.clone(), new_value: t.bridge_n_obs }));
                    }
                    if (b.bridge_t_l_years - t.bridge_t_l_years).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBridgeTLYears(change_member_bridge_tl_years::ChangeMemberBridgeTLYears { member_id: t.id.clone(), new_value: t.bridge_t_l_years }));
                    }
                    if (b.bridge_beta - t.bridge_beta).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBridgeBeta(change_member_bridge_beta::ChangeMemberBridgeBeta { member_id: t.id.clone(), new_value: t.bridge_beta }));
                    }
                    if (b.bridge_a - t.bridge_a).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBridgeA(change_member_bridge_a::ChangeMemberBridgeA { member_id: t.id.clone(), new_value: t.bridge_a }));
                    }
                    if (b.bridge_b - t.bridge_b).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBridgeB(change_member_bridge_b::ChangeMemberBridgeB { member_id: t.id.clone(), new_value: t.bridge_b }));
                    }
                    if (b.bridge_crowd_per_m2 - t.bridge_crowd_per_m2).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeMemberBridgeCrowd(change_member_bridge_crowd::ChangeMemberBridgeCrowd { member_id: t.id.clone(), new_value: t.bridge_crowd_per_m2 }));
                    }
                    if b.actions != t.actions {
                        let member_id = t.id.clone();
                        let base_ids: std::collections::BTreeSet<_> = b.actions.iter().map(|a| a.id.clone()).collect();
                        let target_ids: std::collections::BTreeSet<_> = t.actions.iter().map(|a| a.id.clone()).collect();
                        for (index, a) in b.actions.iter().enumerate().rev() {
                            if !target_ids.contains(&a.id) {
                                out.push(En1995Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { member_id: member_id.clone(), index }));
                            }
                        }
                        for (index, a) in t.actions.iter().enumerate() {
                            if !base_ids.contains(&a.id) {
                                out.push(En1995Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { member_id: member_id.clone(), index, action: a.clone() }));
                            }
                        }
                        for t in &t.actions {
                            if let Some(b) = b.actions.iter().find(|a| a.id == t.id) {
                            if b.kind != t.kind {
                                out.push(En1995Mutation::ChangeMemberActionKind(change_member_action_kind::ChangeMemberActionKind { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.kind.clone() }));
                            }
                            if b.category != t.category {
                                out.push(En1995Mutation::ChangeMemberActionCategory(change_member_action_category::ChangeMemberActionCategory { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.category.clone() }));
                            }
                            if b.load_duration != t.load_duration {
                                out.push(En1995Mutation::ChangeMemberActionLoadDuration(change_member_action_load_duration::ChangeMemberActionLoadDuration { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.load_duration.clone() }));
                            }
                            if (b.q_line_n_per_m - t.q_line_n_per_m).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionQLine(change_member_action_q_line::ChangeMemberActionQLine { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.q_line_n_per_m }));
                            }
                            if (b.f_point_n - t.f_point_n).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionFPoint(change_member_action_f_point::ChangeMemberActionFPoint { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.f_point_n }));
                            }
                            if (b.m_k_nm - t.m_k_nm).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionMK(change_member_action_mk::ChangeMemberActionMK { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.m_k_nm }));
                            }
                            if (b.v_k_n - t.v_k_n).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionVK(change_member_action_vk::ChangeMemberActionVK { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.v_k_n }));
                            }
                            if (b.n_k_n - t.n_k_n).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionNK(change_member_action_nk::ChangeMemberActionNK { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.n_k_n }));
                            }
                            if (b.n_t_k_n - t.n_t_k_n).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionNTK(change_member_action_ntk::ChangeMemberActionNTK { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.n_t_k_n }));
                            }
                            if (b.f_c90_k_n - t.f_c90_k_n).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeMemberActionFC90K(change_member_action_fc90_k::ChangeMemberActionFC90K { member_id: member_id.clone(), action_id: t.id.clone(), new_value: t.f_c90_k_n }));
                            }
                            }
                        }
                    }
                }
            }
        }
        if base.connections != target.connections {
            let base_ids: std::collections::BTreeSet<_> = base.connections.iter().map(|c| c.id.clone()).collect();
            let target_ids: std::collections::BTreeSet<_> = target.connections.iter().map(|c| c.id.clone()).collect();
            for (index, c) in base.connections.iter().enumerate().rev() {
                if !target_ids.contains(&c.id) {
                    out.push(En1995Mutation::RemoveConnection(remove_connection::RemoveConnection { index }));
                }
            }
            for (index, c) in target.connections.iter().enumerate() {
                if !base_ids.contains(&c.id) {
                    out.push(En1995Mutation::InsertConnection(insert_connection::InsertConnection { index, connection: c.clone() }));
                }
            }
            for t in &target.connections {
                if let Some(b) = base.connections.iter().find(|c| c.id == t.id) {
                    if b.label_en != t.label_en {
                        out.push(En1995Mutation::ChangeConnectionLabelEn(change_connection_label_en::ChangeConnectionLabelEn { connection_id: t.id.clone(), new_value: t.label_en.clone() }));
                    }
                    if b.label_de != t.label_de {
                        out.push(En1995Mutation::ChangeConnectionLabelDe(change_connection_label_de::ChangeConnectionLabelDe { connection_id: t.id.clone(), new_value: t.label_de.clone() }));
                    }
                    if b.fastener_type != t.fastener_type {
                        out.push(En1995Mutation::ChangeConnectionFastenerType(change_connection_fastener_type::ChangeConnectionFastenerType { connection_id: t.id.clone(), new_value: t.fastener_type.clone() }));
                    }
                    if b.strength_class != t.strength_class {
                        out.push(En1995Mutation::ChangeConnectionStrengthClass(change_connection_strength_class::ChangeConnectionStrengthClass { connection_id: t.id.clone(), new_value: t.strength_class.clone() }));
                    }
                    if b.service_class != t.service_class {
                        out.push(En1995Mutation::ChangeConnectionServiceClass(change_connection_service_class::ChangeConnectionServiceClass { connection_id: t.id.clone(), new_value: t.service_class }));
                    }
                    if (b.diameter_m - t.diameter_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionDiameter(change_connection_diameter::ChangeConnectionDiameter { connection_id: t.id.clone(), new_value: t.diameter_m }));
                    }
                    if b.number != t.number {
                        out.push(En1995Mutation::ChangeConnectionNumber(change_connection_number::ChangeConnectionNumber { connection_id: t.id.clone(), new_value: t.number }));
                    }
                    if b.rows != t.rows {
                        out.push(En1995Mutation::ChangeConnectionRows(change_connection_rows::ChangeConnectionRows { connection_id: t.id.clone(), new_value: t.rows }));
                    }
                    if (b.spacing_m - t.spacing_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionSpacing(change_connection_spacing::ChangeConnectionSpacing { connection_id: t.id.clone(), new_value: t.spacing_m }));
                    }
                    if (b.edge_distance_m - t.edge_distance_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionEdgeDistance(change_connection_edge_distance::ChangeConnectionEdgeDistance { connection_id: t.id.clone(), new_value: t.edge_distance_m }));
                    }
                    if (b.end_distance_m - t.end_distance_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionEndDistance(change_connection_end_distance::ChangeConnectionEndDistance { connection_id: t.id.clone(), new_value: t.end_distance_m }));
                    }
                    if (b.t1_m - t.t1_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionT1(change_connection_t1::ChangeConnectionT1 { connection_id: t.id.clone(), new_value: t.t1_m }));
                    }
                    if (b.t2_m - t.t2_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionT2(change_connection_t2::ChangeConnectionT2 { connection_id: t.id.clone(), new_value: t.t2_m }));
                    }
                    if b.steel_plate != t.steel_plate {
                        out.push(En1995Mutation::ChangeConnectionSteelPlate(change_connection_steel_plate::ChangeConnectionSteelPlate { connection_id: t.id.clone(), new_value: t.steel_plate }));
                    }
                    if (b.steel_plate_thickness_m - t.steel_plate_thickness_m).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionSteelPlateThickness(change_connection_steel_plate_thickness::ChangeConnectionSteelPlateThickness { connection_id: t.id.clone(), new_value: t.steel_plate_thickness_m }));
                    }
                    if b.shear_planes != t.shear_planes {
                        out.push(En1995Mutation::ChangeConnectionShearPlanes(change_connection_shear_planes::ChangeConnectionShearPlanes { connection_id: t.id.clone(), new_value: t.shear_planes }));
                    }
                    if (b.f_u_k - t.f_u_k).abs() > f64::EPSILON {
                        out.push(En1995Mutation::ChangeConnectionFUK(change_connection_fuk::ChangeConnectionFUK { connection_id: t.id.clone(), new_value: t.f_u_k }));
                    }
                    if b.actions != t.actions {
                        let connection_id = t.id.clone();
                        let base_ids: std::collections::BTreeSet<_> = b.actions.iter().map(|a| a.id.clone()).collect();
                        let target_ids: std::collections::BTreeSet<_> = t.actions.iter().map(|a| a.id.clone()).collect();
                        for (index, a) in b.actions.iter().enumerate().rev() {
                            if !target_ids.contains(&a.id) {
                                out.push(En1995Mutation::RemoveConnectionAction(remove_connection_action::RemoveConnectionAction { connection_id: connection_id.clone(), index }));
                            }
                        }
                        for (index, a) in t.actions.iter().enumerate() {
                            if !base_ids.contains(&a.id) {
                                out.push(En1995Mutation::InsertConnectionAction(insert_connection_action::InsertConnectionAction { connection_id: connection_id.clone(), index, action: a.clone() }));
                            }
                        }
                        for t in &t.actions {
                            if let Some(b) = b.actions.iter().find(|a| a.id == t.id) {
                            if b.kind != t.kind {
                                out.push(En1995Mutation::ChangeConnectionActionKind(change_connection_action_kind::ChangeConnectionActionKind { connection_id: connection_id.clone(), action_id: t.id.clone(), new_value: t.kind.clone() }));
                            }
                            if b.load_duration != t.load_duration {
                                out.push(En1995Mutation::ChangeConnectionActionLoadDuration(change_connection_action_load_duration::ChangeConnectionActionLoadDuration { connection_id: connection_id.clone(), action_id: t.id.clone(), new_value: t.load_duration.clone() }));
                            }
                            if (b.f_k_n - t.f_k_n).abs() > f64::EPSILON {
                                out.push(En1995Mutation::ChangeConnectionActionFK(change_connection_action_fk::ChangeConnectionActionFK { connection_id: connection_id.clone(), action_id: t.id.clone(), new_value: t.f_k_n }));
                            }
                            }
                        }
                    }
                }
            }
        }
        out
    }
}
//#endregion 🔖️FromSnapshot

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
