//! ⚡️ En1995 mutations — OpText/OpBinary via JSON tokens (hierarchical timber subject).

pub use crate::artifact_schema::mutations::En1995Mutation;

use protocol::{OpBinary, OpText};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl OpText for En1995Mutation {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        pack::json::from_json_str(line).map_err(|e| store::TextError::new(e.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl OpBinary for En1995Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "utf8", offset: 0, detail: e.to_string() })?;
        <Self as OpText>::parse_op(text).map_err(|e| protocol::ProtocolError::Malformed { what: "json", offset: 0, detail: e.to_string() })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
mod unit_tests {
    use super::*;

    /// 🎬️ One representative per addressing shape — root enum, member, member action, connection, connection action.
    fn demo_mutation_cases() -> Vec<En1995Mutation> {
        let base = crate::En1995Snapshot::compliant_building_beam();
        vec![
            En1995Mutation::ChangeAnnex(crate::mutations::set_snapshot::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
            En1995Mutation::InsertMember(crate::mutations::insert_member::InsertMember { index: 99, member: crate::TimberMember { id: "beam-B9".into(), ..base.members[0].clone() } }),
            En1995Mutation::RemoveMember(crate::mutations::remove_member::RemoveMember { index: 0 }),
            En1995Mutation::ChangeMemberRole(crate::mutations::change_member_role::ChangeMemberRole { member_id: base.members[0].id.clone(), new_value: crate::MemberRole::Column }),
            En1995Mutation::ChangeMemberStrengthClass(crate::mutations::change_member_strength_class::ChangeMemberStrengthClass { member_id: base.members[0].id.clone(), new_value: "GL32h".into() }),
            En1995Mutation::ChangeMemberSupport(crate::mutations::change_member_support::ChangeMemberSupport { member_id: base.members[0].id.clone(), new_value: crate::SupportType::Cantilever }),
            En1995Mutation::ChangeMemberB(crate::mutations::change_member_b::ChangeMemberB { member_id: base.members[0].id.clone(), new_value: 0.24 }),
            En1995Mutation::ChangeMemberH(crate::mutations::change_member_h::ChangeMemberH { member_id: base.members[0].id.clone(), new_value: 0.5 }),
            En1995Mutation::ChangeMemberBridgeCrowd(crate::mutations::change_member_bridge_crowd::ChangeMemberBridgeCrowd { member_id: base.members[0].id.clone(), new_value: 1.0 }),
            En1995Mutation::InsertMemberAction(crate::mutations::insert_member_action::InsertMemberAction { member_id: base.members[0].id.clone(), index: 99, action: crate::CharacteristicAction { id: "w".into(), ..base.members[0].actions[0].clone() } }),
            En1995Mutation::RemoveMemberAction(crate::mutations::remove_member_action::RemoveMemberAction { member_id: base.members[0].id.clone(), index: 0 }),
            En1995Mutation::ChangeMemberActionLoadDuration(crate::mutations::change_member_action_load_duration::ChangeMemberActionLoadDuration { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: "short".into() }),
            En1995Mutation::ChangeMemberActionQLine(crate::mutations::change_member_action_q_line::ChangeMemberActionQLine { member_id: base.members[0].id.clone(), action_id: base.members[0].actions[0].id.clone(), new_value: 3500.0 }),
            En1995Mutation::InsertConnection(crate::mutations::insert_connection::InsertConnection { index: 99, connection: crate::TimberConnection { id: "conn-C9".into(), ..base.connections[0].clone() } }),
            En1995Mutation::RemoveConnection(crate::mutations::remove_connection::RemoveConnection { index: 0 }),
            En1995Mutation::ChangeConnectionNumber(crate::mutations::change_connection_number::ChangeConnectionNumber { connection_id: base.connections[0].id.clone(), new_value: 12 }),
            En1995Mutation::ChangeConnectionSteelPlate(crate::mutations::change_connection_steel_plate::ChangeConnectionSteelPlate { connection_id: base.connections[0].id.clone(), new_value: true }),
            En1995Mutation::ChangeConnectionFUK(crate::mutations::change_connection_fuk::ChangeConnectionFUK { connection_id: base.connections[0].id.clone(), new_value: 500_000_000.0 }),
            En1995Mutation::InsertConnectionAction(crate::mutations::insert_connection_action::InsertConnectionAction { connection_id: base.connections[0].id.clone(), index: 99, action: crate::ConnectionAction { id: "w".into(), ..base.connections[0].actions[0].clone() } }),
            En1995Mutation::RemoveConnectionAction(crate::mutations::remove_connection_action::RemoveConnectionAction { connection_id: base.connections[0].id.clone(), index: 0 }),
            En1995Mutation::ChangeConnectionActionFK(crate::mutations::change_connection_action_fk::ChangeConnectionActionFK { connection_id: base.connections[0].id.clone(), action_id: base.connections[0].actions[0].id.clone(), new_value: 12000.0 }),
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
}
