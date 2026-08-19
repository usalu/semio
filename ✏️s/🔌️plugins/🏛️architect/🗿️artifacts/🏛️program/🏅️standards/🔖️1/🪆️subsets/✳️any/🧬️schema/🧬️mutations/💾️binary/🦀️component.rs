//! ⚖️ Architect program artifact — state-patch-representation wire codec (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::program::schema::mutations::text::ProgramMutation;
use protocol::OpBinary;

/// 📡️ Encodes an Architect operation for transport or persistence.
pub async fn encode_op(operation: &ProgramMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📥️ Decodes an Architect operation from its transport representation.
pub async fn decode_op(bytes: &[u8]) -> Result<ProgramMutation, protocol::ProtocolError> {
    ProgramMutation::decode_op(bytes)
}

//#region 🧪️Tests
/// 🧷️ The pre-migration `operation_rows_keep_their_pre_migration_bytes` pinned-hex test is
/// deliberately not carried forward: it existed to catch an ACCIDENTAL serde-shape drift on
/// `ProgramMutation`, but the semantic-mutations overhaul is a deliberate, comprehensive vocabulary
/// replacement (the old generic per-collection wrap variants no longer exist), so pinning the old
/// bytes would just assert the migration didn't happen. Round-trip coverage below stands in.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::EntityId;

    #[test]
    async fn disconnect_adjacency_round_trips_through_the_binary_codec() {
        let operation = ProgramMutation::DisconnectAdjacency(super::super::disconnect_adjacency::mutation::DisconnectAdjacency { id: EntityId("adjacency-1".into()) });
        assert_eq!(decode_op(&encode_op(&operation).expect("encode")).expect("decode"), operation);
    }

    #[test]
    async fn delete_program_element_round_trips_through_the_binary_codec() {
        let operation = ProgramMutation::DeleteProgramElement(super::super::delete_program_element::mutation::DeleteProgramElement { id: EntityId("element-1".into()) });
        assert_eq!(decode_op(&encode_op(&operation).expect("encode")).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
