//! 📦️ VCS artifact — binary document surface + laws (was: constitutional `pack`).

use crate::artifacts::vcs::VcsDemoProjection;
use store::PackError;

/// 📦️ Encodes a `VcsDemoProjection` to its binary pack form.
pub fn encode(projection: &VcsDemoProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `VcsDemoProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<VcsDemoProjection, PackError> {
    <VcsDemoProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vcs::op::VcsDemoOperation;
    use crate::artifacts::vcs::VCS_DEMO_SCHEMA;

    #[test]
    fn vcs_demo_projection_dsl_pack_equivalence() {
        let projection = crate::artifacts::vcs::engine::empty_vcs_demo_projection();
        store::test_support::assert_dsl_pack_equivalence(&projection);
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `VcsDemoOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<VcsDemoProjection, VcsDemoOperation> =
            DocumentStore::new(create_document_envelope(VCS_DEMO_SCHEMA, "vcs-demo", crate::artifacts::vcs::engine::empty_vcs_demo_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![VcsDemoOperation::SetTitle { title: "Renamed".into() }], description: None }).expect("apply");
        let edit: &Edit<VcsDemoOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<VcsDemoProjection, VcsDemoOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
