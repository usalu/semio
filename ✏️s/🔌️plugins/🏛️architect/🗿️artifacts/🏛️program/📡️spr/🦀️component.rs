//! ⚖️ Architect program artifact — state-patch-representation wire codec (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::program::op::ProgramMutation;
use protocol::OpBinary;

/// 📡️ Encodes an Architect operation for transport or persistence.
pub fn encode_op(operation: &ProgramMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📥️ Decodes an Architect operation from its transport representation.
pub fn decode_op(bytes: &[u8]) -> Result<ProgramMutation, protocol::ProtocolError> {
    ProgramMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::kernel::EntityId;
    use protocol::CollectionMutation;

    #[test]
    fn clear_adjacency_round_trips_through_the_binary_codec() {
        let operation = ProgramMutation::ClearAdjacency { id: EntityId("adjacency-1".into()) };
        assert_eq!(decode_op(&encode_op(&operation).expect("encode")).expect("decode"), operation);
    }

    #[test]
    fn a_collection_operation_round_trips_through_the_binary_codec() {
        let operation = ProgramMutation::Elements(CollectionMutation::Remove { id: EntityId("element-1".into()) });
        assert_eq!(decode_op(&encode_op(&operation).expect("encode")).expect("decode"), operation);
    }

    /// 🧷️ Pins the exact pre-migration bytes of the JSON-bridge op codec — copied verbatim out of the
    /// ticket's `🧪️wire-baseline-before.txt`, so a future refactor of `ProgramMutation`'s serde shape
    /// cannot silently change the on-the-wire representation.
    #[test]
    fn operation_rows_keep_their_pre_migration_bytes() {
        let hex = |operation: &ProgramMutation| encode_op(operation).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&ProgramMutation::ClearAdjacency { id: EntityId("adjacency-1".into()) }), "7b226f7065726174696f6e223a22636c65617241646a6163656e6379222c226964223a2261646a6163656e63792d31227d");
        assert_eq!(
            hex(&ProgramMutation::Elements(CollectionMutation::Remove { id: EntityId("element-1".into()) })),
            "7b226f7065726174696f6e223a22656c656d656e7473222c226b696e64223a2272656d6f7665222c226964223a22656c656d656e742d31227d"
        );
        assert_eq!(
            hex(&ProgramMutation::Elements(CollectionMutation::Move { id: EntityId("element-1".into()), to_index: 2 })),
            "7b226f7065726174696f6e223a22656c656d656e7473222c226b696e64223a226d6f7665222c226964223a22656c656d656e742d31222c22746f223a327d"
        );
    }
}
//#endregion 🧪️Tests
