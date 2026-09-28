"""🌱️ C12 window-3 set for L1's T3 train — one causal identity per history node in every store initializer (collab-e2e STEP 8 on
7800/p24: "document-archive-replacement.initializer-failed … duplicate mutation id").

Root cause: a replica folds every operation of the hub's tail as its own remote edit NAMED AFTER ITS MUTATION ID
(`store::edit_from_operation_envelope`: `Edit { id: envelope.mutation_id, mutation_meta: [{ mutation_id: envelope.mutation_id }] }`).
Every plugin store initializer's SeedHistory seeded the entry id (lane 0) AND each forward's mutation id (lane 1) as two nodes — for a
remote edit that is the same id twice, and the causal owner refuses the whole document. The framework's own hydrations already skip an
operation id equal to its entry id; eight plugin copies and the SDK's bounded initializer did not. Fix = ONE rule in the store runtime
(`ArtifactStoreInitializationRuntime::seed_edit_operation`: an operation named like its own entry is that entry's node, never a second
one; a repeat across entries still refuses) used by every initializer and both hydrations; law = the captured 17-envelope tail (four
edits, Check In, eleven two-author edits, second Check In) folded, printed as a pack, parsed and initialized through writer's store
initializer.

Idempotent: new files are copied from `tree/` (refused when a different file exists); every hunk replaces one exact anchor that must
occur exactly once (a hunk whose replacement is present counts as applied).
usage: python3 c12-seed-history-patch.py [--apply]   (default: dry run; exit 1 when any hunk or file cannot be placed)"""
import os
import sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
TREE = os.path.join(HERE, "tree")
APPLY = "--apply" in sys.argv

STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
COMPOSITION = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs"
WRITER_ANY = "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any"
WRITER_BIN = WRITER_ANY + "/🚪️io/🧬️mutations/💾️binary"

PLAIN = 'if let Err(error) = runtime.seed_mutation(id) {'
PLAIN_NEW = 'if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {'
MATCHED = 'match runtime.seed_mutation(id) {'
MATCHED_NEW = 'match runtime.seed_edit_operation(&entry.id, id) {'

LAW = r'''
//#region 🔖️HubTailInitialization
/// 🔁️ The hub's catch-up tail of STEP 8 (ticket 26/09/23 C12, captured from hub 7800): a replica folds each operation as a remote edit
/// named after its mutation id. Parsed from the fixture's verbatim wire fields.
fn hub_tail_envelope(value: &serde_json::Value) -> protocol::MutationEnvelope {
    let text = |node: &serde_json::Value| node.as_str().expect("hub tail fixture string").to_string();
    let bytes = |node: &serde_json::Value| node["payload"].as_array().expect("hub tail payload bytes").iter().map(|byte| u8::try_from(byte.as_u64().expect("hub tail byte")).expect("hub tail byte fits u8")).collect::<Vec<u8>>();
    let number = |node: &serde_json::Value| node.as_u64().expect("hub tail timestamp field");
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(text(&value["mutation_id"])),
        document_id: protocol::ArtifactId(text(&value["document_id"])),
        actor: protocol::ActorId(text(&value["actor"])),
        dependencies: value["dependencies"].as_array().expect("hub tail dependencies").iter().map(|id| protocol::MutationId(text(id))).collect(),
        observed: value["observed"].as_str().map(|id| protocol::MutationId(id.to_string())),
        target: value["target"].as_array().expect("hub tail target").iter().map(text).collect(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(text(&value["diff"]["schema"])), payload: bytes(&value["diff"]) },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(text(&value["inverse"]["schema"])), payload: bytes(&value["inverse"]) },
        timestamp: protocol::HybridLogicalTimestamp { actor: number(&value["timestamp"]["actor"]), physical_ms: number(&value["timestamp"]["physical_ms"]), logical: number(&value["timestamp"]["logical"]) },
    }
}

/// 🌱️ LAW: a document a replica folded from the hub's tail — beside its own local edits, STEP 8's mixed shape — initializes again through this store initializer. Every folded operation is
/// an edit whose id IS its only operation's mutation id — one causal node, which SeedHistory used to seed twice and refuse as
/// `duplicate mutation id` (collab-e2e STEP 8: the reload after a Check In showed "Document restore failed … initializer-failed").
/// The initialized candidate must be the folded document.
#[semio_framework_async_macros::async_test]
async fn a_document_folded_from_the_hub_tail_initializes_again() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json")).expect("hub tail fixture decodes");
    let document_id = fixture["documentId"].as_str().expect("hub tail document id");
    let tail = fixture["envelopes"].as_array().expect("hub tail envelopes");
    assert_eq!(tail.len(), 17, "the captured tail: four edits, a Check In, eleven two-author edits, a second Check In");
    let mut folded = new_writer_store(store::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, document_id, schema::empty_writer_snapshot(), None)).await.expect("valid writer store fixture");
    folded.dispatch(store::ArtifactCommand::Apply { mutations: vec![schema::mutations::edit_text("user1 typed before the hub tail".to_string())], description: None }).await.expect("a locally authored edit before the tail");
    for value in tail {
        folded.ingest_remote(hub_tail_envelope(value)).await.expect("every operation and transition of the hub tail folds");
    }
    folded.dispatch(store::ArtifactCommand::Apply { mutations: vec![schema::mutations::edit_text("user1 typed after the hub tail".to_string())], description: None }).await.expect("a locally authored edit after the tail");
    let (remote, local) = folded.envelope().vcs.edits.iter().fold((0, 0), |(remote, local), edit| if edit.mutation_meta.first().and_then(|meta| meta.mutation_id.as_ref()).is_some_and(|id| id.0 == edit.id) { (remote + 1, local) } else { (remote, local + 1) });
    assert_eq!((remote, local), (15, 2), "STEP 8's mixed shape: every folded operation is an edit named after its own mutation id, the replica's own edits are local (entry id + operation id)");
    let live = folded.snapshot().expect("folded writer snapshot");
    let files = store::print_document_pack(folded.envelope()).await.expect("print the folded document pack");
    let parsed: store::ParsedDocumentText<WriterSnapshot, WriterMutation> = store::parse_document_pack(&files.pack, &files.spr).await.expect("parse the folded document pack");
    let operation = semio_framework_job::OperationId(403);
    let generation = semio_framework_job::Generation(13);
    let mut authority = WriterStoreInitializationAuthority::new(parsed.into_envelope(), operation, generation);
    let outcome = drive_writer_initializer(&mut authority, operation, generation);
    assert!(matches!(outcome, semio_framework_job::StepOutcome::Complete(_)), "the folded hub tail must initialize, not fault (duplicate mutation id)");
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Writer candidate");
    assert_eq!(candidate.snapshot().expect("initialized writer snapshot"), live, "the initialized document is the folded one");
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_writer_candidate(candidate);
}
//#endregion 🔖️HubTailInitialization
'''

HUNKS = [
    # 🌱️ the ONE rule, in the store runtime every initializer and hydration seeds through.
    (STORE + "/🦀️.rs",
     "    pub fn seed_mutation(&mut self, id: MutationId) -> Result<(), String> {\n",
     "    /// @emoji 🌱️ Seeds operation `id` of history entry `entry_id` into the causal owner. An edit a replica folded from a peer is\n"
     "    /// named after its only operation (`edit_from_operation_envelope`), so an operation id equal to its own entry id is the node\n"
     "    /// the entry already seeded — one node, never two; a repeat across entries still refuses as a duplicate.\n"
     "    pub fn seed_edit_operation(&mut self, entry_id: &str, id: MutationId) -> Result<(), String> {\n"
     "        if id.0 == entry_id {\n"
     "            return Ok(());\n"
     "        }\n"
     "        self.seed_mutation(id)\n"
     "    }\n\n"
     "    pub fn seed_mutation(&mut self, id: MutationId) -> Result<(), String> {\n"),
    (STORE + "/🧾️document/📜️history/💧️hydration/🦀️.rs",
     'if id.0 != edit_id && self.runtime.as_mut().expect("hydration runtime remains retained").seed_mutation(id.clone()).is_err() {',
     'if self.runtime.as_mut().expect("hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {'),
    (STORE + "/🎚️config/📥️retained/🦀️.rs",
     'if id.0 != edit_id && self.runtime.as_mut().expect("config hydration runtime remains retained").seed_mutation(id.clone()).is_err() {',
     'if self.runtime.as_mut().expect("config hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {'),
    (COMPOSITION,
     "if id.0 != edit.id && runtime.seed_mutation(id.clone()).is_err() {",
     "if runtime.seed_edit_operation(&edit.id, id.clone()).is_err() {"),
    (SDK,
     "runtime.seed_mutation(id).map(|()| BoundedStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 })",
     "runtime.seed_edit_operation(&entry.id, id).map(|()| BoundedStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 })"),
    # 🧩️ the eight plugin store initializers that copied the SDK's SeedHistory.
    (WRITER_BIN + "/🦀️.rs", PLAIN, PLAIN_NEW),
    ("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", PLAIN, PLAIN_NEW),
    ("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs", PLAIN, PLAIN_NEW),
    ("✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs", PLAIN, PLAIN_NEW),
    ("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", PLAIN, PLAIN_NEW),
    ("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", MATCHED, MATCHED_NEW),
    ("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", MATCHED, MATCHED_NEW),
    ("✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", MATCHED, MATCHED_NEW),
    # 🧪️ the law, appended to writer's store-initializer unit tests (fixture in `tree/`).
    (WRITER_BIN + "/🧪️tests/🔬️unit/🦀️.rs",
     '    assert!(outcome.is_err(), "the fixture panic must reach this caller as an unwind");\n}\n',
     '    assert!(outcome.is_err(), "the fixture panic must reach this caller as an unwind");\n}\n' + LAW),
]


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def main():
    problems, planned = [], []
    for root, _, files in os.walk(TREE):
        for name in files:
            source = os.path.join(root, name)
            relative = os.path.relpath(source, TREE)
            target = os.path.join(REPO, relative)
            content = open(source, encoding="utf-8").read()
            if os.path.exists(target):
                planned.append(("file-present", relative)) if open(target, encoding="utf-8").read() == content else problems.append(("file-differs", relative))
            else:
                planned.append(("file-new", relative))
    texts = {}
    for path, old, new in HUNKS:
        if not os.path.exists(os.path.join(REPO, path)):
            problems.append(("missing-file", path))
            continue
        text = texts.get(path) or read(path)
        if new in text and old not in text.replace(new, ""):
            planned.append(("hunk-present", path))
            continue
        if new.startswith(old) and "fn a_document_folded_from_the_hub_tail_initializes_again" in new and "fn a_document_folded_from_the_hub_tail_initializes_again" in text:
            planned.append(("law-present", path + " (version deltas: c12-seed-history-amend.py)"))
            continue
        count = text.count(old)
        if count != 1:
            problems.append((f"anchor-count-{count}", f"{path} :: {old[:80]!r}"))
            continue
        texts[path] = text.replace(old, new, 1)
        planned.append(("hunk", path))
    for kind, what in planned:
        print(f"OK   {kind:13} {what}")
    for kind, what in problems:
        print(f"FAIL {kind:13} {what}")
    print(f"{len(planned)} planned, {len(problems)} problem(s), mode={'apply' if APPLY else 'dry-run'}")
    if problems:
        sys.exit(1)
    if not APPLY:
        return
    for root, _, files in os.walk(TREE):
        for name in files:
            source = os.path.join(root, name)
            target = os.path.join(REPO, os.path.relpath(source, TREE))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(source, encoding="utf-8") as src, open(target, "w", encoding="utf-8") as dst:
                dst.write(src.read())
    for path, text in texts.items():
        with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
            handle.write(text)


main()
