//! ⚖️ Equation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! `protocol::OpBinary for EquationMutation` is implemented directly in `crate::artifacts::equation::op`
//! (see that module's doc comment). This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.
//!
//! The app's typed `EquationCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `✏️editor/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::equation::op::EquationMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `EquationMutation` to its binary command form.
pub async fn encode_op(operation: &EquationMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `EquationMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<EquationMutation, protocol::ProtocolError> {
    EquationMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::equation::EquationSnapshot;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::change_graph_directed::mutation::ChangeGraphDirected;
        let operation = EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn math_document_text_round_trips_through_store() {
        use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::mutation::UpdateGraphAlgorithm;
        let initial = EquationSnapshot::default();
        let envelope = store::create_document_envelope(crate::artifacts::equation::MATH_DOCUMENT_SCHEMA, "math-demo", initial, None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        let mutation = UpdateGraphAlgorithm { new_algorithm: "components".into(), new_algorithm_seed: None };
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![EquationMutation::UpdateGraphAlgorithm(mutation)], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
