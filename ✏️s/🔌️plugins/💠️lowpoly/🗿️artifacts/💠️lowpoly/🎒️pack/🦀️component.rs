//! 📦️ Lowpoly artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::lowpoly::LowpolyProjection;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `LowpolyProjection` to its binary pack form.
pub fn encode(projection: &LowpolyProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `LowpolyProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<LowpolyProjection, PackError> {
    <LowpolyProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::engine::default_projection;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let projection = default_projection();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }

    #[test]
    fn pack_round_trips_a_projection_with_a_painted_layer() {
        let mut projection = default_projection();
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn handcrafted_pack_protocol_uses_lwpl_domain_magic() {
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("0x894C57504C0D0A1A"));
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment Objects"));
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment PaintLayers"));
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment Projection"));
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `LowpolyMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::lowpoly::engine::default_projection;
        use crate::artifacts::lowpoly::op::{LowpolyMutation, LowpolyPaintLayerPatch};
        use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mut store: DocumentStore<LowpolyProjection, LowpolyMutation> = DocumentStore::new(create_document_envelope(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None));
        let operation = LowpolyMutation::PatchPaintLayer { object_id, index: 0, patch: LowpolyPaintLayerPatch { name: Some("Renamed Layer".into()), visible: None, opacity: None, blend_mode: None } };
        store.dispatch(DocumentCommand::Apply { mutations: vec![operation], description: None }).expect("apply");
        let edit: &Edit<LowpolyMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<LowpolyProjection, LowpolyMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
