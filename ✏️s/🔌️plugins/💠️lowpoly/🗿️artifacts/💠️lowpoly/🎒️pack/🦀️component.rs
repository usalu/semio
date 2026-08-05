//! 📦️ Lowpoly artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::lowpoly::LowpolyProjection;
use store::PackError;

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
    use crate::artifacts::lowpoly::dsl::{parse_dsl, LOWPOLY_EXAMPLE_TEXT};

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let projection = parse_dsl(LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses");
        store::test_support::assert_dsl_pack_equivalence(&projection);
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }

    #[test]
    fn pack_round_trips_a_projection_with_a_painted_layer() {
        let mut projection = parse_dsl(LOWPOLY_EXAMPLE_TEXT).expect("default projection DSL parses");
        projection.objects[0].paint_layers[0].pixels[0] = 7;
        projection.objects[0].paint_layers[0].pixels[1] = 9;
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `LowpolyOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::lowpoly::engine::default_projection;
        use crate::artifacts::lowpoly::op::{LowpolyOperation, LowpolyPaintLayerPatch};
        use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let projection = default_projection();
        let object_id = projection.objects[0].id.clone();
        let mut store: DocumentStore<LowpolyProjection, LowpolyOperation> = DocumentStore::new(create_document_envelope(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None));
        let operation = LowpolyOperation::PatchPaintLayer { object_id, index: 0, patch: LowpolyPaintLayerPatch { name: Some("Renamed Layer".into()), visible: None, opacity: None, blend_mode: None } };
        store.dispatch(DocumentCommand::Apply { operations: vec![operation], description: None }).expect("apply");
        let edit: &Edit<LowpolyOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<LowpolyProjection, LowpolyOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
