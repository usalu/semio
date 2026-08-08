//! 📦️ VCS artifact — binary document surface + laws (was: constitutional `pack`).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


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
    use crate::artifacts::vcs::op::VcsDemoMutation;
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
    /// `VcsDemoMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<VcsDemoProjection, VcsDemoMutation> =
            DocumentStore::new(create_document_envelope(VCS_DEMO_SCHEMA, "vcs-demo", crate::artifacts::vcs::engine::empty_vcs_demo_projection(), None));
        store.dispatch(DocumentCommand::Apply { mutations: vec![VcsDemoMutation::SetTitle { title: "Renamed".into() }], description: None }).expect("apply");
        let edit: &Edit<VcsDemoMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<VcsDemoProjection, VcsDemoMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
