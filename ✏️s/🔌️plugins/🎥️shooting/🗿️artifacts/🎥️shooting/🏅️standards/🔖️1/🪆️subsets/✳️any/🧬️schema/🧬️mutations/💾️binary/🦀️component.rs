//! ⚖️ Shooting artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for ShootingMutation` is implemented directly in `🔧️op/🦀️component.rs` (it needs
//! the `ShootingMutationDsl` mirror that lives alongside the operation enum). This component only adds
//! the thin artifact-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `ShootingCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🎥️shooting/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::shooting::schema::mutations::text::ShootingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `ShootingMutation` to its binary state-patch form.
pub fn encode_op(operation: &ShootingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `ShootingMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<ShootingMutation, protocol::ProtocolError> {
    ShootingMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::ShootingSnapshot;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = ShootingMutation::SetActiveShot(crate::artifacts::shooting::schema::mutations::set_active_shot::mutation::SetActiveShot { shot_id: Some("s1".into()) });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn shooting_document_text_round_trips_store_with_applied_operation() {
        use store::ArtifactCommand;

        let mut store = store::ArtifactStore::<ShootingSnapshot, ShootingMutation>::new(store::create_document_envelope(crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_snapshot(), None));
        let asset = crate::artifacts::shooting::ShootingAsset { id: "a1".into(), name: "Asset".into(), url: "/mesh/a1.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None };
        let create = crate::artifacts::shooting::schema::mutations::create_asset::mutation::CreateAsset { asset, index: Some(0) };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![ShootingMutation::CreateAsset(create)], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
