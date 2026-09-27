"""📌️ H13 item 4 root fix (idempotent): every codec answer that parses a pair only to read it (the guest twin's
`print_mirror`, the zero-op `apply_ops` pass-through in both the guest twin and the store's linked-codec thunk) retires its
unadopted envelope entry by entry (`ArtifactEnvelope::retire_unadopted`) instead of dropping its owners. A POPULATED pair's
history ledgers carry a terminal-empty Drop witness, so `drop(envelope.into_owners())` aborted the guest on every pair
with an edit: a hub Check In validates its folded pair through `print-mirror`, so every writer/note Check In was refused
`codec-refused` ("artifact history ledger reached Drop before every exact entry owner was retired", vcs 🦀️.rs:622).
Plus two laws that read a populated pair through both codec tables (red without the fix).
usage: python3 h13-checkin-retire-patch.py [--dry-run]"""
import sys

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules"
PLUGIN = f"{ROOT}/🔌️plugin/🦀️.rs"
STORE = f"{ROOT}/🏪️store/🦀️.rs"
SURFACE = f"{ROOT}/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs"
STORE_TESTS = f"{ROOT}/🏪️store/🧪️tests/🔬️unit/🦀️.rs"

EDITS = [
    (PLUGIN,
     """        if mutations.is_empty() {
            let printed = store::print_document_pack(&envelope).await;
            drop(envelope.into_owners());
            return printed;
        }""",
     """        if mutations.is_empty() {
            let printed = store::print_document_pack(&envelope).await;
            envelope.retire_unadopted();
            return printed;
        }"""),
    (PLUGIN,
     """                let mirror = store::print_document_text(&envelope).await;
                drop(envelope.into_owners());
                mirror.map_err(|error| error.into_fault())""",
     """                let mirror = store::print_document_text(&envelope).await;
                envelope.retire_unadopted();
                mirror.map_err(|error| error.into_fault())"""),
    (STORE,
     """                    let printed = print_document_pack(&envelope).await;
                    drop(envelope.into_owners());
                    let files = printed?;""",
     """                    let printed = print_document_pack(&envelope).await;
                    envelope.retire_unadopted();
                    let files = printed?;"""),
    (STORE_TESTS,
     """    assert_eq!(history.edits.len(), 1, "one op in the batch must land exactly one edit, got {}", history.edits.len());
}
""",
     """    assert_eq!(history.edits.len(), 1, "one op in the batch must land exactly one edit, got {}", history.edits.len());
    let passed = (codec.apply_ops_binary)(&applied.0, &applied.1, &crate::os_spr::encode_ops_vec(&[])).await.expect("an empty batch passes a populated pair through");
    assert!(!passed.0.is_empty() && !passed.1.is_empty(), "an empty batch over a populated pair returns that pair");
    let mirror = (codec.print_mirror)(&applied.0, &applied.1).await.expect("a populated pair mirrors");
    assert!(mirror.ops.contains("set-n"), "the mirror prints the pair's edit: {}", mirror.ops);
}
"""),
    (SURFACE,
     """#[semio_framework_async_macros::async_test]
async fn viewer_rejects_every_contract_mutating_verb() {""",
     """/// 📌️ A hub Check In validates the pair it folded through the guest codec's `print-mirror`, and a zero-op `apply-ops`
/// batch passes a pair through: both read a POPULATED pair, whose unadopted envelope must be retired entry by entry —
/// dropping its owners aborted the guest on the history ledger's terminal-empty witness, so every writer and note Check In
/// was refused `codec-refused` (ticket 26/09/23, session 14).
#[semio_framework_async_macros::async_test]
async fn the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting() {
    let table = crate::app::artifact_codec_table::<EditorApp<SurfaceEditorFixture>>();
    let genesis = (table.genesis)("surface-codec").await.expect("genesis pair");
    let op = protocol::OpBinary::encode_op(&SurfaceMutation::from(SetSurfaceCount { value: 7 })).expect("encode set-surface-count");
    let populated = (table.apply_ops)(&genesis.pack, &genesis.spr, &store::os_spr::encode_ops_vec(&[op])).await.expect("one op lands one edit");
    let history = store::os_spr::decode_history(&populated.spr, &store::os_spr::DecodeOptions::default()).await.expect("populated history");
    assert_eq!(history.edits.len(), 1, "the pair carries one edit, so its history ledger is populated");
    let mirror = (table.print_mirror)(&populated.pack, &populated.spr).await.expect("a populated pair mirrors");
    assert!(mirror.ops.contains("set-surface-count"), "the mirror prints the pair's edit: {}", mirror.ops);
    let passed = (table.apply_ops)(&populated.pack, &populated.spr, &store::os_spr::encode_ops_vec(&[])).await.expect("an empty batch passes a populated pair through");
    assert!(!passed.pack.is_empty() && !passed.spr.is_empty(), "an empty batch over a populated pair returns that pair");
}

#[semio_framework_async_macros::async_test]
async fn viewer_rejects_every_contract_mutating_verb() {"""),
]

dry = "--dry-run" in sys.argv
texts = {}
pending = 0
for path, old, new in EDITS:
    text = texts.setdefault(path, open(path).read())
    if new in text:
        print(f"applied already: {path.rsplit('/', 3)[-3:]}")
        continue
    count = text.count(old)
    if count != 1:
        sys.exit(f"anchor matched {count}× in {path}: {old[:80]!r}")
    texts[path] = text.replace(old, new)
    pending += 1
    print(f"{'would apply' if dry else 'apply'}: {path.rsplit('/', 3)[-3:]}")
if not dry:
    for path, text in texts.items():
        open(path, "w").write(text)
print(f"{'dry run' if dry else 'done'}: {pending} pending")
