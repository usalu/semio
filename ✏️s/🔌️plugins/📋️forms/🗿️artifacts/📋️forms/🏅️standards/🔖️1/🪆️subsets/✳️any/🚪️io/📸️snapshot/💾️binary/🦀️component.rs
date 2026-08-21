//! 📦️ Forms artifact — binary document surface + laws (constitutional: pack). Ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): `store::ArtifactPack
//! for FormsSnapshot` now lives HERE (moved from `🧬️schema/📸️snapshot`, which keeps only the struct)
//! — the composed `structure`/`results` child slots have no `flow::playbook` bridge equivalent, so
//! this facet hand-rolls the wire format directly. This component owns the real `encode_pack_with`/
//! `decode_pack_with` impl, the thin artifact-facing `encode`/`decode` wrappers, and the pack↔dsl
//! equivalence law plus the command-envelope round-trip law.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::forms::FormsSnapshot;
use store::PackError;

//#region 🔖️BinaryPrimitives
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
async fn write_opt_str_lp(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_lp(out, v);
        }
        None => out.push(0),
    }
}
async fn read_opt_str_lp(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("bad option tag {other}")),
    }
}
async fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
async fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
async fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
async fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

async fn encode_forms_snapshot_binary(s: &FormsSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.id);
    write_str_lp(&mut out, &s.version);
    write_opt_str_lp(&mut out, &s.title);
    write_child(&mut out, &s.structure);
    write_child(&mut out, &s.results);
    out
}
async fn decode_forms_snapshot_binary(bytes: &[u8]) -> Result<FormsSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    Ok(FormsSnapshot { schema: read_str_lp(&mut reader)?, id: read_str_lp(&mut reader)?, version: read_str_lp(&mut reader)?, title: read_opt_str_lp(&mut reader)?, structure: read_child(&mut reader)?, results: read_child(&mut reader)? })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
impl store::ArtifactPack for FormsSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_forms_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_forms_snapshot_binary(&inner).map_err(PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactPack

/// 📦️ Encodes a `FormsSnapshot` to its binary pack form.
pub async fn encode(document: &FormsSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `FormsSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<FormsSnapshot, PackError> {
    <FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::dsl;
    use crate::artifacts::forms::{forms_children_from_steps, FormStep, FORMS_DOCUMENT_SCHEMA};

    #[semio_framework_async_macros::async_test]
    async fn snapshot_pack_round_trips_with_composed_children() {
        let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
        let (structure, results) = forms_children_from_steps(&steps);
        let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, structure, results };
        let encoded = store::ArtifactPack::encode_pack(&snapshot);
        let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&encoded).expect("decodes");
        assert_eq!(decoded, snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn building_component_fixture_pack_agrees_with_dsl() {
        let spec = dsl::parse_playbook_example_dsl(dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).expect("📋️building-component.forms parses");
        store::os_store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_fixture_pack_agrees_with_dsl() {
        let spec = dsl::parse_playbook_example_dsl(dsl::DEFAULT_EXAMPLE_TEXT).expect("📋️default.forms parses");
        store::os_store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }

    #[semio_framework_async_macros::async_test]
    async fn onboarding_fixture_pack_agrees_with_dsl() {
        let spec = dsl::parse_playbook_example_dsl(dsl::ONBOARDING_EXAMPLE_TEXT).expect("📋️onboarding.forms parses");
        store::os_store::test_support::assert_dsl_pack_equivalence(&spec);
        let bytes = encode(&spec);
        assert_eq!(decode(&bytes).expect("decode"), spec);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `FormMutation`'s `CreateStep` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip laws (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::forms::{op::FormMutation, FormStep, FORMS_DOCUMENT_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let document = crate::artifacts::forms::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, vec![FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }]);
        let mut store: ArtifactStore<FormsSnapshot, FormMutation> = ArtifactStore::new(create_document_envelope(FORMS_DOCUMENT_SCHEMA, "forms-demo", document, None)).expect("valid artifact store fixture");
        let step = FormStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![FormMutation::CreateStep(crate::artifacts::forms::mutations::create_step::mutation::CreateStep { step, index: None })], description: None }).expect("apply");
        let edit: &Edit<FormMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<FormsSnapshot, FormMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
