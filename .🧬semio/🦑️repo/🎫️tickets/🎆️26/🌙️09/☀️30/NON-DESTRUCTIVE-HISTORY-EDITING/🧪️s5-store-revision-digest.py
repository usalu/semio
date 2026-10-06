#!/usr/bin/env python3
"""🔏️ S5-STORE wave C, store half (design §22.28, channel 23): `sequence_number` leaves the revision digest.

An edit's `sequence_number` is where the edit stands in ONE replica's ledger: a remote merge renumbers the applied edits by
position, a history-log reload derives it from the stored order, an edit of a line the replica does not stand on keeps the
number it arrived with. It was hashed into the edit's revision record, so what a content revision named depended on the path
a replica took to its log. The revision identity now covers the edit's value WITHOUT that member:

  * single-operation edit: `record("edit", [id, JSON(edit minus sequenceNumber)])`, members otherwise in the same order;
  * chained edit: `record("edit-chained", [...])` without its 4-byte `sequence` part, everything else unchanged;
  * the one-item sealer streams exactly that JSON (its Edit field table loses the member), so sealer digest == store digest;
  * the remote merge's renumbering no longer dirties revision records before the insertion point.

`Edit.sequence_number` stays on the type, the wire, `.spr` and `.ops`; the preparation authority still binds
`next_sequence_number`. No persisted or wire byte changes: the revision digest is replica-local.

Parts: `rule` (store, sealer, the five Rust test oracles that equate canonical bytes with `edit.to_value()`), `law` (the
viewer-head corpus law). S5-CHANNEL reseals the canonical-edit fixtures, schema and TS twins in the same wave.

Every hunk is keyed on an anchor that must occur exactly once. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-revision-digest.py [--check] [--only rule|law] [--revert] [--emit <dir>]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE_DIR = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
STORE = f"{STORE_DIR}/🦀️.rs"
SEALER = f"{STORE_DIR}/🧵️canonical-edit/🦀️.rs"
UNIT = f"{STORE_DIR}/🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs"
BORROWED = f"{STORE_DIR}/🧵️canonical-edit/🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs"
READER = f"{STORE_DIR}/🧵️canonical-edit/📖️reader/🧪️tests/📖️reader/🦀️.rs"
VIEWER = f"{STORE_DIR}/🧪️tests/🧪️viewer-head/🦀️.rs"

RULE = [
    (
        STORE,
        "store:revision-value",
        '''    /// 🔏️ Revision identity of one edit — a pure function of the edit. A single-operation edit hashes its canonical JSON
    /// (exactly what the one-item byte sealer streams); a coalesced edit that grew past one operation hashes its header
    /// fields and one running chain per operation list ([`EditDigestChains`]), which an amend extends in O(appended).
    fn edit_digest<Mutation: ToValue>(edit: &Edit<Mutation>) -> [u8; 32] {
''',
        '''    /// 🥽️ What an edit's revision identity covers: its value without `sequenceNumber`. That member is where the edit stands
    /// in ONE replica's ledger — a remote merge renumbers it, a reload derives it from the stored order — so two replicas
    /// holding the same edit at different positions, and a replica and its own reload, name the same edit (design §22.28).
    fn revision_value<Mutation: ToValue>(edit: &Edit<Mutation>) -> DslValue {
        match edit.to_value() {
            DslValue::Object(mut members) => {
                members.retain(|(key, _)| key != "sequenceNumber");
                DslValue::Object(members)
            }
            value => value,
        }
    }

    /// 🔏️ Revision identity of one edit — a pure function of [`Self::revision_value`]. A single-operation edit hashes that
    /// value's canonical JSON (exactly what the one-item byte sealer streams); a coalesced edit that grew past one operation
    /// hashes its header fields and one running chain per operation list ([`EditDigestChains`]), which an amend extends in
    /// O(appended).
    fn edit_digest<Mutation: ToValue>(edit: &Edit<Mutation>) -> [u8; 32] {
''',
    ),
    (
        STORE,
        "store:single-operation",
        '''            let encoded = semio_framework_pack_json::to_json_string(edit).into_bytes();
            return (Self::hash_record(b"edit", &[edit.id.as_bytes(), &encoded]), None);
''',
        '''            let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&Self::revision_value(edit))).into_bytes();
            return (Self::hash_record(b"edit", &[edit.id.as_bytes(), &encoded]), None);
''',
    ),
    (
        STORE,
        "store:chained",
        '''                edit.actor.as_deref().unwrap_or_default().as_bytes(),
                &edit.sequence_number.to_be_bytes(),
                edit.started_at.as_bytes(),
''',
        '''                edit.actor.as_deref().unwrap_or_default().as_bytes(),
                edit.started_at.as_bytes(),
''',
    ),
    (
        STORE,
        "store:renumber",
        '''        let mut renumbered = k;
        for edit in self.envelope.vcs.edits.iter_mut() {
            if let Some(index) = positions.get(edit.id.as_str()).copied().filter(|index| edit.sequence_number != *index as i32 + 1) {
                edit.sequence_number = index as i32 + 1;
                renumbered = renumbered.min(index);
            }
        }
        drop(positions);
        self.edit_sequence = self.applied_edit_ids.len() as i32;
        self.replace_tail_undo_cache_retained(None)?;
        self.replace_current_retained(state)?;
        self.revision_dirty_from = Some(self.revision_dirty_from.map_or(renumbered, |dirty| dirty.min(renumbered)));
''',
        '''        for edit in self.envelope.vcs.edits.iter_mut() {
            if let Some(index) = positions.get(edit.id.as_str()) {
                edit.sequence_number = *index as i32 + 1;
            }
        }
        drop(positions);
        self.edit_sequence = self.applied_edit_ids.len() as i32;
        self.replace_tail_undo_cache_retained(None)?;
        self.replace_current_retained(state)?;
        self.revision_dirty_from = Some(self.revision_dirty_from.map_or(k, |dirty| dirty.min(k)));
''',
    ),
    (
        SEALER,
        "sealer:cursor-doc",
        '''}

enum CanonicalEditNode<'a, M> {
''',
        '''}

/// 🪺️ Typed cursor over what an edit's revision identity covers (`CursorRevisionAccumulator::revision_value`): every member
/// of the edit but `sequenceNumber`, its position in one replica's ledger.
enum CanonicalEditNode<'a, M> {
''',
    ),
    (
        SEALER,
        "sealer:fields",
        '''                fields[..10].copy_from_slice(&[
                    ("id", true),
                    ("actor", edit.actor.is_some()),
                    ("forwards", true),
                    ("inverse", true),
                    ("mutationMeta", !edit.mutation_meta.is_empty()),
                    ("verb", edit.verb.is_some()),
                    ("sequenceNumber", true),
                    ("startedAt", true),
                    ("finishedAt", edit.finished_at.is_some()),
                    ("line", true),
                ]);
''',
        '''                fields[..9].copy_from_slice(&[
                    ("id", true),
                    ("actor", edit.actor.is_some()),
                    ("forwards", true),
                    ("inverse", true),
                    ("mutationMeta", !edit.mutation_meta.is_empty()),
                    ("verb", edit.verb.is_some()),
                    ("startedAt", true),
                    ("finishedAt", edit.finished_at.is_some()),
                    ("line", true),
                ]);
''',
    ),
    (
        SEALER,
        "sealer:children",
        '''                6 => Self::Scalar(N::I64(i64::from(edit.sequence_number))),
                7 => Self::Scalar(N::String(&edit.started_at)),
                8 => Self::Scalar(N::String(edit.finished_at.as_deref().ok_or_else(invalid_path)?)),
                9 => Self::Scalar(edit.line.as_deref().map_or(N::Null, N::String)),
''',
        '''                6 => Self::Scalar(N::String(&edit.started_at)),
                7 => Self::Scalar(N::String(edit.finished_at.as_deref().ok_or_else(invalid_path)?)),
                8 => Self::Scalar(edit.line.as_deref().map_or(N::Null, N::String)),
''',
    ),
    (
        UNIT,
        "unit:oracle",
        '''    let oracle = serde_json::to_vec(&test_support::SerdeValue(&edit.to_value())).unwrap();
''',
        '''    let oracle = serde_json::to_vec(&test_support::SerdeValue(&CursorRevisionAccumulator::revision_value(&edit))).unwrap();
''',
    ),
    (
        UNIT,
        "unit:origins-oracle",
        '''            let expected = serde_json::to_vec(&test_support::SerdeValue(&edit.to_value())).unwrap();
''',
        '''            let expected = serde_json::to_vec(&test_support::SerdeValue(&CursorRevisionAccumulator::revision_value(&edit))).unwrap();
''',
    ),
    (
        BORROWED,
        "borrowed:oracle",
        '''        let expected = serde_json::to_vec(&test_support::SerdeValue(&owner.edit.as_ref().unwrap().as_ref().to_value())).unwrap();
''',
        '''        let expected = serde_json::to_vec(&test_support::SerdeValue(&crate::os_store::component::CursorRevisionAccumulator::revision_value(owner.edit.as_ref().unwrap().as_ref()))).unwrap();
''',
    ),
    (
        READER,
        "reader:oracle",
        '''        let expected = serde_json::to_vec(&test_support::SerdeValue(&reader.owned.root.as_ref().unwrap().as_ref().to_value())).unwrap();
''',
        '''        let expected = serde_json::to_vec(&test_support::SerdeValue(&crate::os_store::component::CursorRevisionAccumulator::revision_value(reader.owned.root.as_ref().unwrap().as_ref()))).unwrap();
''',
    ),
    (
        READER,
        "reader:failed-prefix-oracle",
        '''            let expected = serde_json::to_string(&test_support::SerdeValue(&oracle.to_value())).unwrap();
''',
        '''            let expected = serde_json::to_string(&test_support::SerdeValue(&crate::os_store::component::CursorRevisionAccumulator::revision_value(&oracle))).unwrap();
''',
    ),
]

LAW = [
    (
        VIEWER,
        "law:revision",
        '''//#endregion 🧪️Corpus

//#region 🧪️FinalizeLaws
''',
        '''//#endregion 🧪️Corpus

//#region 🧪️RevisionLaws
/// 🧲️ LAW (design §22.28): a content revision names what a head holds, never where its edits stand in one replica's ledger.
/// For every corpus case a replica takes the whole log in every arrival order while it stands on each line of the case — it
/// steps onto a line the moment that line is listed, so the other lines' edits reach it while it shows something else.
/// Whatever line it stood on and whatever order the events came in, it names ONE content revision per line, and its own
/// persisted pair restores exactly the revision it showed.
#[semio_framework_async_macros::async_test]
async fn a_content_revision_names_a_head_whatever_line_its_replica_stood_on() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️viewer-head/🔣️.json")).expect("the corpus parses");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("a case name");
        let initial = case["initial"]["n"].as_i64().map(|n| i32::try_from(n).expect("an i32"));
        let mut replicas = [Replica { store: fresh(name, initial).await, authored: Vec::new() }, Replica { store: fresh(name, initial).await, authored: Vec::new() }];
        for step in case["steps"].as_array().expect("steps") {
            act(&mut replicas, replica_index(&step["at"]), &step["do"]).await;
        }
        let [a, b] = &mut replicas;
        deliver(&mut a.store, b.store.event_log().expect("log")).await;
        let log = a.store.event_log().expect("the whole log");
        let lines: Vec<&str> = case["converged"]["lines"].as_object().expect("lines").keys().map(String::as_str).collect();
        let mut arrivals = vec![log.clone(), log.iter().rev().cloned().collect::<Vec<_>>()];
        arrivals.extend((1..log.len()).map(|rotation| {
            let mut rotated = log.clone();
            rotated.rotate_left(rotation);
            rotated
        }));
        let mut named: BTreeMap<&str, BTreeMap<[u8; 32], String>> = BTreeMap::new();
        for (arrival, events) in arrivals.iter().enumerate() {
            for stand in &lines {
                let mut replica = fresh(name, initial).await;
                let mut standing = *stand == "trunk";
                for event in events.iter().cloned() {
                    replica.ingest_remote(event).await.expect("a replica ingests a shared event");
                    if !standing && replica.envelope().vcs.alternatives.iter().any(|alternative| alternative.name == *stand) {
                        let alternative_id = line_id(&replica, stand);
                        replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id }).await.expect("a replica steps onto a listed line");
                        standing = true;
                    }
                }
                assert!(standing && replica.dag.pending_is_empty(), "{name} arrival {arrival}: the replica stood on {stand} and every event found its dependencies");
                for line in &lines {
                    let alternative_id = line_id(&replica, line);
                    replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id }).await.expect("a local switch");
                    let revision = replica.content_revision_now();
                    assert_eq!(reloaded(&replica, "spr").await.content_revision_now(), revision, "{name} arrival {arrival}, stood on {stand}: the persisted pair restores the revision {line} showed");
                    let positions = replica.envelope().vcs.edits.iter().map(|edit| edit.sequence_number.to_string()).collect::<Vec<_>>().join(",");
                    named.entry(*line).or_default().entry(revision).or_insert_with(|| format!("arrival {arrival} stood on {stand}, ledger positions {positions}"));
                }
            }
        }
        for (line, revisions) in &named {
            assert_eq!(revisions.len(), 1, "{name}: every replica names one content revision for {line}: {:?}", revisions.values().collect::<Vec<_>>());
        }
    }
}
//#endregion 🧪️RevisionLaws

//#region 🧪️FinalizeLaws
''',
    ),
]


def main():
    arguments = sys.argv[1:]
    only = arguments[arguments.index("--only") + 1] if "--only" in arguments else None
    emit = Path(arguments[arguments.index("--emit") + 1]) if "--emit" in arguments else None
    revert = "--revert" in arguments
    hunks = [hunk for part, part_hunks in (("rule", RULE), ("law", LAW)) if only in (None, part) for hunk in part_hunks]
    texts, pending = {}, []
    for path, name, old, new in hunks:
        text = texts.setdefault(path, (ROOT / path).read_text(encoding="utf-8"))
        before, after = (new, old) if revert else (old, new)
        if text.count(after) == 1 and text.count(before) == 0:
            continue
        if text.count(before) != 1:
            raise SystemExit(f"{name}: anchor occurs {text.count(before)} times in {path} (expected 1): re-derive the wave")
        texts[path] = text.replace(before, after)
        pending.append(name)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if emit is not None:
        emit.mkdir(parents=True, exist_ok=True)
        for index, (path, text) in enumerate(texts.items()):
            (emit / f"{index}-{Path(path).parent.name}.rs").write_text(text, encoding="utf-8")
        print(f"emitted {len(texts)} files to {emit}")
        return
    if "--check" in arguments or not pending:
        return
    for path, text in texts.items():
        (ROOT / path).write_text(text, encoding="utf-8")
    print(f"applied {len(pending)} hunks in {len(texts)} files")


main()
