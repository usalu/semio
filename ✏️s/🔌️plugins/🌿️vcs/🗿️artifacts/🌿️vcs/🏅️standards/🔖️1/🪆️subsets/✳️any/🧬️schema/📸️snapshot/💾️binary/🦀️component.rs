//! 📦️ VCS artifact — binary document surface + laws (was: constitutional `pack`).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::vcs::VcsSnapshot;
use store::PackError;

/// 📦️ Encodes a `VcsSnapshot` to its binary pack form.
pub fn encode(projection: &VcsSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(projection)
}

/// 📖️ Decodes a `VcsSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<VcsSnapshot, PackError> {
    <VcsSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vcs::op::VcsDemoMutation;
    use crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA;

    #[test]
    fn vcs_demo_projection_dsl_pack_equivalence() {
        let projection = crate::artifacts::vcs::engine::empty_vcs_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&projection);
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
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<VcsSnapshot, VcsDemoMutation> =
            ArtifactStore::new(create_document_envelope(VCS_DOCUMENT_SCHEMA, "vcs-demo", crate::artifacts::vcs::engine::empty_vcs_snapshot(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::vcs::mutations::rename_vcs("Renamed".into())], description: None }).expect("apply");
        let edit: &Edit<VcsDemoMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<VcsSnapshot, VcsDemoMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
