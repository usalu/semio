"""🔗 Stages atomic backbone fragments beside the exact native causal decoder."""

import hashlib
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
REPLICATION = Path("🧰️framework/🔨️modules/📡️replication")
OWNER = REPLICATION / "🚪️io/💾️binary/🔗️causal/🦀️.rs"
MODULE = OWNER.with_name("🧩️atomic-group.rs")
MANIFEST = REPLICATION / "🔗️causal/📦️packages/🦀️rust/Cargo.toml"
SPR = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️.rs")
OUT = TICKET / "🗑️generated/raw-spr-group-overlay.json"

DECODE = """/// 🧷️ Decodes a separately sealed original group while preserving the ordinary per-operation limits.
pub fn decode_verified_atomic_backbone(
    bytes: &atomic_group::VerifiedAtomicBackbone<'_>, document: &str, actor: &str,
) -> Result<VerifiedAtomicEdit, semio_framework_replication_error::ProtocolError> {
    decode_atomic_backbone(bytes, document, Some(actor))
}

/// 📡️ Opens a sealed edit on a port whose remote authors were authenticated by the native Hub.
pub fn decode_verified_atomic_backbone_at_document(
    bytes: &atomic_group::VerifiedAtomicBackbone<'_>, document: &str,
) -> Result<VerifiedAtomicEdit, semio_framework_replication_error::ProtocolError> {
    decode_atomic_backbone(bytes, document, None)
}

fn decode_atomic_backbone(
    bytes: &(impl ByteReadSource + ?Sized), document: &str, actor: Option<&str>,
) -> Result<VerifiedAtomicEdit, semio_framework_replication_error::ProtocolError> {
    if !atomic_group::AtomicBackboneSender::matches_prepared(bytes) {
        return Err(document_backbone_batch_malformed(0, "atomic-group-prefix"));
    }
    let mut position = atomic_group::PREPARED_ATOMIC_EDIT_TAG.len();
    let edit_id = read_document_backbone_text(bytes, &mut position, DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, "edit-id")?;
    if edit_id.is_empty() { return Err(document_backbone_batch_malformed(0, "atomic-group-edit")); }
    let batch = semio_framework_value::paged::ByteReadRange::new(bytes, position, bytes.byte_len() - position)
        .ok_or_else(|| document_backbone_batch_malformed(position, "atomic-group-range"))?;
    let ceiling = DocumentBackboneBatchLimitsV1 {
        maximum_bytes: atomic_group::ATOMIC_BACKBONE_MAXIMUM_BYTES,
        maximum_payload_bytes: atomic_group::ATOMIC_BACKBONE_MAXIMUM_BYTES,
        ..Default::default()
    };
    let events = decode_document_backbone_envelopes_bounded(&batch, ceiling, ceiling)?;
    let Some((author, transaction)) = events.first().and_then(|event| event.transaction.as_ref().map(|tx| (&event.actor, tx))) else {
        return Err(document_backbone_batch_malformed(0, "atomic-group-transaction"));
    };
    let mut ids = std::collections::HashSet::with_capacity(events.len());
    if author.0.is_empty() || actor.is_some_and(|expected| author.0.as_str() != expected) || transaction.id.is_empty() ||
        !atomic_group::is_atomic_backbone_tool(&transaction.tool) || events.iter().any(|event|
        event.document_id.0 != document || &event.actor != author || event.transaction.as_ref() != Some(transaction)
        || event.mutation_id.0.is_empty() || !ids.insert(event.mutation_id.0.as_str())
    ) {
        return Err(document_backbone_batch_malformed(0, "atomic-group-binding"));
    }
    Ok(VerifiedAtomicEdit { edit_id, events })
}

/// 📜️ Preserves the authoring Store edit identity with its complete operation sequence.
pub struct VerifiedAtomicEdit {
    edit_id: String,
    events: Vec<MutationEnvelope>,
}

impl VerifiedAtomicEdit {
    pub fn edit_id(&self) -> &str { &self.edit_id }
    pub fn events(&self) -> &[MutationEnvelope] { &self.events }
    pub fn into_parts(self) -> (String, Vec<MutationEnvelope>) { (self.edit_id, self.events) }
}

"""


def stage():
    pre = {}
    post = {}

    def edit(path, anchor, replacement):
        raw = (ROOT / path).read_bytes()
        pre[str(path)] = hashlib.sha256(raw).hexdigest()
        text = post.get(str(path), raw.decode("utf8"))
        assert text.count(anchor) == 1, (path, anchor)
        post[str(path)] = text.replace(anchor, replacement)

    edit(OWNER, "pub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES", '#[path = "🧩️atomic-group.rs"]\npub mod atomic_group;\n\npub const DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES')
    edit(OWNER, "    let ceiling = DocumentBackboneBatchLimitsV1::default();\n    if limits.maximum_bytes",
         """    decode_document_backbone_envelopes_bounded(bytes, limits, Default::default())
}

""" + DECODE + """fn decode_document_backbone_envelopes_bounded(
    bytes: &(impl ByteReadSource+?Sized), limits: DocumentBackboneBatchLimitsV1, ceiling: DocumentBackboneBatchLimitsV1,
) -> Result<Vec<MutationEnvelope>, semio_framework_replication_error::ProtocolError> {
    if limits.maximum_bytes""")
    edit(OWNER, "        let diff_payload = read_document_backbone_payload(bytes, &mut position, limits.maximum_payload_bytes.saturating_sub(total_payload_bytes))?;",
         """        let operation_limit = DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES.min(limits.maximum_payload_bytes.saturating_sub(total_payload_bytes));
        let diff_payload = read_document_backbone_payload(bytes, &mut position, operation_limit)?;""")
    edit(OWNER, "let inverse_payload = read_document_backbone_payload(bytes, &mut position, limits.maximum_payload_bytes.saturating_sub(total_payload_bytes))?;",
         "let inverse_payload = read_document_backbone_payload(bytes, &mut position, operation_limit - diff_payload.len())?;")
    edit(MANIFEST, "semio-framework-value.workspace = true\n", "semio-framework-value.workspace = true\nsemio-framework-hash.workspace = true\n")
    edit(SPR, "pub use crate::io::binary::causal::{decode_document_backbone_envelopes_exact,",
         "pub use crate::io::binary::causal::{atomic_group, VerifiedAtomicEdit, decode_verified_atomic_backbone, decode_verified_atomic_backbone_at_document, decode_document_backbone_envelopes_exact,")
    assert not (ROOT / MODULE).exists()
    pre[str(MODULE)] = None
    post[str(MODULE)] = (TICKET / "raw-spr-group.rs").read_text(encoding="utf8")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps({"preimages": pre, "candidate": post}, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} atomic backbone product postimages")


if __name__ == "__main__":
    stage()
