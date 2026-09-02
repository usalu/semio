//! 🎒️ CAD artifact — the binary document surface: `encode`/`decode` over the derive-generated
//! `store::ArtifactPack`, and the law that pack and dsl are two projections of the same `CadSnapshot`.

use crate::artifacts::cad::CadSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `CadSnapshot` to its binary pack form.
pub fn encode(document: &CadSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `CadSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<CadSnapshot, PackError> {
    <CadSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::testkit::sample_scene;

    #[semio_framework_async_macros::async_test]
    async fn cad_scene_round_trips_through_pack() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&sample_scene());
        let bytes = encode(&sample_scene());
        assert_eq!(decode(&bytes).expect("decode"), sample_scene());
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `CadMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::cad::mutations::create_shape_model::mutation::CreateShapeModel;
        use crate::artifacts::cad::op::CadMutation;
        use crate::artifacts::cad::{empty_cad_snapshot, testkit::sample_model_child, CAD_DOCUMENT_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<CadSnapshot, CadMutation> = ArtifactStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad-demo", empty_cad_snapshot(), None)).await.expect("valid artifact store fixture");
        let sample = sample_model_child("command-envelope-1");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() })], description: None }).await.expect("apply");
        let edit: &Edit<CadMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<CadSnapshot, CadMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
