"""📐 Aligns the existing database command/recovery consumers with actual paged causal owners."""

import re
import hashlib
import json
from pathlib import Path

BASE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db")
BACKING = BASE / "🔄️sync/🎟️hello-admission-and-backing/🦀️.rs"
ARTIFACT = BASE / "🗿️artifact"
CONTRACT = ARTIFACT / "🧬️contract"
RECEIPTS = CONTRACT / "🧾️receipts-and-checkpoints/🦀️.rs"
RECOVERY = CONTRACT / "🛡️durable-group-recovery/🦀️.rs"
PATHMAP = CONTRACT / "🗺️pathmap-diff-and-bridge/🦀️.rs"
UNDO = ARTIFACT / "⚙️engine/↩️engine-undo-durable-group-query-and-preview/🦀️.rs"
ENGINE = ARTIFACT / "⚙️engine/🦀️.rs"
TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated/raw-hub-paged-db-baseline.json"


def stage(root, edit):
    if "host.advance(grant)" in (root / RECOVERY).read_text(encoding="utf8"):
        edit(RECOVERY, "match host.advance(grant) {", """match {
                let mut report_progress = |_| true;
                let mut receiving = store::MemberNativeDecodeControl::new(grant.maximum_capacity_bytes, &mut report_progress);
                host.advance(grant, &mut receiving)
            } {""")
    if "pub async fn encode_pathmap_payload(" in (root / PATHMAP).read_text(encoding="utf8"):
        assert "fn database_sync_hello_retire_payload(" in (root / BACKING).read_text(encoding="utf8")
        assert "store::StoreMutationFor<ParentP>" in (root / RECOVERY).read_text(encoding="utf8")
        return
    anchor = """pub(super) struct DatabaseSyncHelloEnvelopeClose {"""
    edit(BACKING, anchor, """/// 🧹️ Retires one bounded suffix or one empty backing page from the original causal payload.
fn database_sync_hello_retire_payload(owner: &mut protocol::CausalPayload) -> bool {
    let bytes = owner.retained_bytes_mut();
    if !bytes.is_empty() {
        for _ in 0..bytes.len().min(DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES) { let _ = bytes.pop(); }
        return true;
    }
    if bytes.allocated_bytes() == 0 { return false; }
    let extent = bytes.next_release_allocation_bytes().expect("admitted paged causal backing extent");
    let released = bytes.release_empty_page_leased(extent).expect("paid paged causal release");
    assert!(released.progressed);
    true
}

""" + anchor)
    for field in ("diff", "inverse"):
        edit(BACKING, f"database_sync_hello_retire_bytes(&mut owner.{field}.payload)",
             f"database_sync_hello_retire_payload(&mut owner.{field}.payload)")
        edit(BACKING, f"envelope.{field}.payload.capacity()", f"envelope.{field}.payload.retained_bytes().allocated_bytes()")
        edit(PATHMAP, f"decode_pathmap(&{field}.payload)", f"decode_pathmap(&{field}.payload.to_vec_owner())")
    for file in (RECEIPTS, RECOVERY):
        lines = (root / file).read_text(encoding="utf8")
        for owner in ("Parent", "Drawing", "Value"):
            for field, bounds in (
                (owner + "P", "store::StoreState + pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot"),
                (owner + "Mutation", f"store::StoreMutationFor<{owner}P> + store::os_store::serialization::ArtifactCanonicalJsonTree"),
            ):
                values = set(re.findall(rf"\b{field}: ([^\n,]+),", lines))
                assert len(values) == 2, (file, field, values)
                for prior in values:
                    anchor = f"{field}: {prior},"
                    edit(file, anchor, f"{field}: {bounds},", count=lines.count(anchor))
    edit(PATHMAP, "/// 🎯️ Inverse of " + chr(96) + "encode_pathmap" + chr(96), """/// 📥️ Admits a path-map value to the canonical paged bytes of an outgoing causal envelope.
pub async fn encode_pathmap_payload(value: &DslValue) -> Result<protocol::CausalPayload, DbError> {
    protocol::CausalPayload::try_from_slice(&encode_pathmap(value).await).map_err(|error| DbError::InvalidArgument(error.to_string()))
}

/// 🎯️ Inverse of """ + chr(96) + "encode_pathmap" + chr(96))
    edit(PATHMAP, "payload: encode_pathmap(&forward).await", "payload: encode_pathmap_payload(&forward).await?")
    edit(PATHMAP, "payload: encode_pathmap(&backward).await", "payload: encode_pathmap_payload(&backward).await?")
    edit(CONTRACT / "🦀️.rs", "decode_pathmap_json, diff_entries, encode_pathmap, encode_pathmap_json,",
         "decode_pathmap_json, diff_entries, encode_pathmap, encode_pathmap_json, encode_pathmap_payload,")
    edit(ENGINE, "durable_group_touch, encode_pathmap, entries_to_value", "durable_group_touch, encode_pathmap, encode_pathmap_payload, entries_to_value")
    edit(UNDO, "payload: encode_pathmap(&entries_to_value(&dsl_entries)).await",
         "payload: encode_pathmap_payload(&entries_to_value(&dsl_entries)).await?")
    edit(UNDO, "payload: encode_pathmap(&semio_framework_value::DslValue::Object(vec![])).await",
         "payload: encode_pathmap_payload(&semio_framework_value::DslValue::Object(vec![])).await?")


def baseline():
    preimages = {}
    candidate = {}

    def edit(path, anchor, replacement, *, count=1):
        before = (ROOT / path).read_bytes()
        value = candidate.get(str(path), before.decode("utf8"))
        assert value.count(anchor) == count, (path, anchor, value.count(anchor))
        preimages[str(path)] = hashlib.sha256(before).hexdigest()
        candidate[str(path)] = value.replace(anchor, replacement)

    stage(ROOT, edit)
    OUT.write_text(json.dumps({"preimages": preimages, "candidate": candidate}, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(candidate)} native DB baseline-only postimages")


if __name__ == "__main__":
    baseline()
