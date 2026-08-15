//! ⚖️ Playground artifact — state-patch-representation wire codec + laws.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `PlaygroundMutation` to its binary state-patch form.
pub fn encode_op(operation: &PlaygroundMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `PlaygroundMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<PlaygroundMutation, protocol::ProtocolError> {
    PlaygroundMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::change_schema::mutation::ChangeSchema;
        let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground.custom".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
