//! 📦️ Forms artifact — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for FormSpec` is implemented directly in the shared `playbook` kernel crate; see
//! `🗿️artifacts/📋️forms/🦀️component.rs` for why. This component only adds the thin artifact-facing
//! `encode`/`decode` wrappers plus the pack↔dsl equivalence law and the command-envelope round-trip law.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::forms::FormSpec;
use store::PackError;

/// 📦️ Encodes a `FormSpec` to its binary pack form.
pub fn encode(document: &FormSpec) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `FormSpec` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FormSpec, PackError> {
    <FormSpec as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::dsl;

    #[test]
    fn building_component_fixture_pack_agrees_with_dsl() {
        let spec = dsl::parse_dsl(dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
        store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }

    #[test]
    fn default_fixture_pack_agrees_with_dsl() {
        let spec = dsl::parse_dsl(dsl::DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
        store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }

    #[test]
    fn onboarding_fixture_pack_agrees_with_dsl() {
        let spec = dsl::parse_dsl(dsl::ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
        store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `FormOperation`'s `AddStep` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing dsl/pack round-trip laws (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::forms::{op::FormOperation, FormStep, FORMS_DOCUMENT_SCHEMA};
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let document = FormSpec { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, steps: vec![FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }] };
        let mut store: DocumentStore<FormSpec, FormOperation> = DocumentStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms-demo", document, None));
        let step = FormStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        store.dispatch(DocumentCommand::Apply { operations: vec![FormOperation::AddStep { step, index: None }], description: None }).expect("apply");
        let edit: &Edit<FormOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<FormSpec, FormOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
