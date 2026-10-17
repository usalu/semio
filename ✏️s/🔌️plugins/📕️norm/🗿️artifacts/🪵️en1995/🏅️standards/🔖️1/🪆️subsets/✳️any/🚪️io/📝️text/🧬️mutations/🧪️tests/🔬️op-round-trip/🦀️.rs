//! 🧪️ `unit_tests` — moved out of `📝️text/🦀️.rs` into its canonical test implementation.
use crate::mutations::*;

/// 🎬️ One representative per addressing shape — root enum, member, member action, connection, connection action.
fn demo_mutation_cases() -> Vec<En1995Mutation> {
    let base = semio_s_artifact_norm_en1995_model::En1995Snapshot::compliant_building_beam();
    vec![
        En1995Mutation::ChangeAnnex(semio_s_artifact_norm_en1995_mut_members::change_annex::ChangeAnnex { new_annex: semio_s_plugin_norm_compliance::AnnexChoice::En }),
        En1995Mutation::InsertMember(semio_s_artifact_norm_en1995_mut_members::insert_member::InsertMember { index: 99, member: semio_s_artifact_norm_en1995_model::TimberMember { id: "beam-B9".into(), ..base.members[0].clone() } }),
        En1995Mutation::RemoveMember(semio_s_artifact_norm_en1995_mut_members::remove_member::RemoveMember { index: 0 }),
        En1995Mutation::ChangeMemberRole(semio_s_artifact_norm_en1995_mut_members::change_member_role::ChangeMemberRole { member_id: base.members[0].id.clone(), new_value: semio_s_artifact_norm_en1995_model::MemberRole::Column }),
        En1995Mutation::ChangeMemberStrengthClass(semio_s_artifact_norm_en1995_mut_members::change_member_strength_class::ChangeMemberStrengthClass { member_id: base.members[0].id.clone(), new_value: "GL32h".into() }),
        En1995Mutation::ChangeMemberSupport(semio_s_artifact_norm_en1995_mut_members::change_member_support::ChangeMemberSupport { member_id: base.members[0].id.clone(), new_value: semio_s_artifact_norm_en1995_model::SupportType::Cantilever }),
        En1995Mutation::ChangeMemberB(semio_s_artifact_norm_en1995_mut_geometry::change_member_b::ChangeMemberB { member_id: base.members[0].id.clone(), new_value: 0.24 }),
        En1995Mutation::ChangeMemberH(semio_s_artifact_norm_en1995_mut_geometry::change_member_h::ChangeMemberH { member_id: base.members[0].id.clone(), new_value: 0.5 }),
        En1995Mutation::ChangeMemberBridgeCrowd(semio_s_artifact_norm_en1995_mut_footbridges::change_member_bridge_crowd::ChangeMemberBridgeCrowd { member_id: base.members[0].id.clone(), new_value: 1.0 }),
        En1995Mutation::InsertMemberAction(semio_s_artifact_norm_en1995_mut_actions::insert_member_action::InsertMemberAction { member_id: base.members[0].id.clone(), index: 99, action: semio_s_artifact_norm_en1995_model::CharacteristicAction { id: "w".into(), ..base.members[0].actions[0].clone() } }),
        En1995Mutation::RemoveMemberAction(semio_s_artifact_norm_en1995_mut_actions::remove_member_action::RemoveMemberAction { member_id: base.members[0].id.clone(), index: 0 }),
        En1995Mutation::ChangeMemberLoadDuration(semio_s_artifact_norm_en1995_mut_actions::change_member_load_duration::ChangeMemberLoadDuration { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: "short".into() }),
        En1995Mutation::ChangeMemberActionQLine(semio_s_artifact_norm_en1995_mut_actions::change_member_action_q_line::ChangeMemberActionQLine { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 3500.0 }),
        En1995Mutation::InsertConnection(semio_s_artifact_norm_en1995_mut_connections::insert_connection::InsertConnection { index: 99, connection: semio_s_artifact_norm_en1995_model::TimberConnection { id: "conn-C9".into(), ..base.connections[0].clone() } }),
        En1995Mutation::RemoveConnection(semio_s_artifact_norm_en1995_mut_connections::remove_connection::RemoveConnection { index: 0 }),
        En1995Mutation::ChangeConnectionNumber(semio_s_artifact_norm_en1995_mut_fasteners::change_connection_number::ChangeConnectionNumber { connection_id: base.connections[0].id.clone(), new_value: 12 }),
        En1995Mutation::ChangeConnectionSteelPlate(semio_s_artifact_norm_en1995_mut_plates::change_connection_steel_plate::ChangeConnectionSteelPlate { connection_id: base.connections[0].id.clone(), new_value: true }),
        En1995Mutation::ChangeConnectionFUK(semio_s_artifact_norm_en1995_mut_fasteners::change_connection_fuk::ChangeConnectionFUK { connection_id: base.connections[0].id.clone(), new_value: 500_000_000.0 }),
        En1995Mutation::InsertConnectionAction(semio_s_artifact_norm_en1995_mut_joint_actions::insert_connection_action::InsertConnectionAction { connection_id: base.connections[0].id.clone(), index: 99, action: semio_s_artifact_norm_en1995_model::ConnectionAction { id: "w".into(), ..base.connections[0].actions[0].clone() } }),
        En1995Mutation::RemoveConnectionAction(semio_s_artifact_norm_en1995_mut_joint_actions::remove_connection_action::RemoveConnectionAction { connection_id: base.connections[0].id.clone(), index: 0 }),
        En1995Mutation::ChangeConnectionActionFK(semio_s_artifact_norm_en1995_mut_joint_actions::change_connection_action_fk::ChangeConnectionActionFK { connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: 12000.0 }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = <En1995Mutation as OpText>::print_op(&mutation);
        assert!(!printed.contains('\n'), "print_op must be one line");
        let parsed = <En1995Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed: {e}"));
        assert_eq!(parsed, mutation);
        let encoded = <En1995Mutation as OpBinary>::encode_op(&mutation).unwrap();
        let decoded = <En1995Mutation as OpBinary>::decode_op(&encoded).unwrap();
        assert_eq!(decoded, mutation);
    }
}
