#!/usr/bin/env python3
"""🩹️ S5-STORE wave RR (found by the cross-editor acceptance law, raster family, 2026-10-06 01:17; design §23):
a command the store REFUSES aborted the guest of every editor whose snapshot is fail-closed.

`ArtifactStore::replay_mutations` held its working projection, the command's operations and their inverses as plain locals.
Only the refused-inverse branch retired them; an encode failure (`?`), a failing diff (`?`) and a rejected apply
(`return Err(VcsError::Rejected { .. })`) let them fall to drop glue, and a projection that owns a fail-closed root (raster's
owned map) panics there — `drop_glue::<RasterSnapshot>` ← `replay_mutations` ← `apply_command` ← `dispatch`. Its three callers
had the same shape after a successful replay: every `?` between the replay and the adoption dropped the same owners bare.

The fix is ONE mechanism, not patched returns: `ScratchOwner<T>` holds a scratch owner and retires it through its
technology's cold retirement on EVERY exit path until `adopt` hands it to the store.

  * `replay_mutations` holds projection / operations / inverses as `ScratchOwner`s and RETURNS them as such;
  * `apply_command`, `open_transaction_edit`, `append_transaction` adopt each part where the store takes it (the edit
    `apply_command` builds is a `ScratchOwner` too until the ledger takes it);
  * `replace_current_retained` retires the projection it was offered when it refuses.

The fold seams (`ReplayProjection`) and the history replays (`EditReplay`, `EditReplayResult`, `EffectiveOperation`) already
retire on drop: `replay_mutations` was the one replay with bare locals.

Parts: `rule` (store), `law` (kernel law over a fail-closed fixture + its corpus and schema, mounted in the store unit tests).
The body of `replay_mutations` is replaced as one span guarded by its digest; every other hunk is keyed on an anchor with an
exact expected count. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-replay-retires-on-every-exit.py [--check] [--only rule|law] [--emit <dir>] [--revert-rule]

`--revert-rule` restores the store exactly (hunks swapped back, the old `replay_mutations` span read from the sidecar the apply wrote).
"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE_DIR = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
STORE = STORE_DIR / "🦀️.rs"
UNIT = STORE_DIR / "🧪️tests/🔬️unit/🦀️.rs"
LAW = STORE_DIR / "🧪️tests/🧪️replay-retirement/🦀️.rs"
CORPUS = STORE_DIR / "🧫️fixtures/🧫️replay-retirement/🔣️.json"
SCHEMA = STORE_DIR / "🧬️schema/🔣️replay-retirement/🔣️.json"

REPLAY_START = "    async fn replay_mutations(&mut self, pre_snapshot: &P, mutations: Vec<Mutation>"
REPLAY_END = "    /// 🕹️ Parses `command_text` via [`parse_command`]"
REPLAY_OLD_DIGEST = "c85c0d3b3ebb3252cfc518aa3b07a1bf4e56e3440a326e90a81f88353d6bab69"
REPLAY_NEW = '''    async fn replay_mutations(&mut self, pre_snapshot: &P, mutations: Vec<Mutation>, transaction: Option<&protocol::TransactionRef>) -> Result<(ScratchOwner<Vec<Mutation>>, ScratchOwner<Vec<Mutation>>, Vec<MutationMeta>, ScratchOwner<P>, Vec<crate::os_spr::MutationMessage>), VcsError> {
        let mut snapshot = ScratchOwner::new(pre_snapshot.clone(), retire_replayed_projection::<P, Mutation>);
        let forwards = ScratchOwner::new(mutations, retire_scratch_operations::<P, Mutation>);
        let mut inverse = ScratchOwner::new(Vec::new(), retire_scratch_operations::<P, Mutation>);
        let mut candidate_clock = self.clock;
        let mut mutation_meta = Vec::with_capacity(forwards.len());
        let mut messages = Vec::new();
        for op_index in 0..forwards.len() {
            let encoded = forwards[op_index].encode_op().map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
            let mut back = forwards[op_index].inverse(&*snapshot).map_err(VcsError::InverseRefused)?;
            back.reverse();
            inverse.extend(back);
            let authored_timestamp = forwards[op_index].timestamp();
            let timestamp = match authored_timestamp {
                Some(timestamp) => {
                    candidate_clock.merge(&timestamp);
                    timestamp
                }
                None => {
                    candidate_clock.tick(now_ms());
                    candidate_clock
                }
            };
            let authored_id = forwards[op_index].mutation_id();
            let mutation_id = match authored_id {
                Some(id) => id,
                None => MutationId(mint_mutation_id(&encoded, (candidate_clock.actor, candidate_clock.physical_ms, candidate_clock.logical)).await),
            };
            let mutation = &forwards[op_index];
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: mutation.dependencies(),
                base_version: mutation.base_version().map_or(0, |version| version.0),
                author_id: Some(mutation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp,
                undo_policy: mutation.undo_policy(),
                payload_hash: Some(crate::os_spr::PayloadHash(*semio_framework_hash::hash(&encoded).as_bytes())),
                semantic_kind: Some(operation_semantic_kind::<P, Mutation>(mutation, &self.envelope.schema)),
                label: Some(mutation.descriptor().display_name.to_string()),
                group_id: None,
                origin: Default::default(),
                transaction: transaction.cloned(),
            });
            let outcome = mutation.diff(&*snapshot).stamp_op_index(op_index as u32);
            let (diff, op_messages) = outcome.into_parts();
            messages.extend(op_messages);
            let applied = diff.apply(&*snapshot);
            MutationDiff::retire_cold(diff);
            let next = applied?;
            retire_replayed_projection::<P, Mutation>(std::mem::replace(&mut *snapshot, next));
        }
        if let Some(level) = crate::os_spr::worst_level(&messages) {
            if self.merge_policy.rejects(level) {
                return Err(VcsError::Rejected { policy: self.merge_policy, messages });
            }
        }
        self.clock = candidate_clock;
        Ok((forwards, inverse, mutation_meta, snapshot, messages))
    }

'''

SCRATCH_OWNER = '''/// 🩹️ A scratch owner — a working projection, the operations of a command, an edit not yet in the ledger — that leaves
/// through its technology's cold retirement on EVERY exit path until [`Self::adopt`] hands it to the store. An artifact whose
/// projection or operation owns a fail-closed root aborts the guest on a bare drop, so a command the store refuses (a rejected
/// apply, a refused inverse, an encode failure, a failing diff, a saturated ledger) must never let what it built reach `Drop`.
/// Holding it here makes that structural: a new early return cannot miss it.
struct ScratchOwner<T> {
    owner: Option<T>,
    retire: fn(T),
}

impl<T> ScratchOwner<T> {
    fn new(owner: T, retire: fn(T)) -> Self {
        Self { owner: Some(owner), retire }
    }

    /// 🤝️ Hands the owner to the store, which owns its retirement from here on.
    fn adopt(mut self) -> T {
        self.owner.take().expect("a scratch owner is adopted exactly once")
    }
}

impl<T> std::ops::Deref for ScratchOwner<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.owner.as_ref().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> std::ops::DerefMut for ScratchOwner<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.owner.as_mut().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> Drop for ScratchOwner<T> {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            (self.retire)(owner);
        }
    }
}

'''
SCRATCH_ANCHOR = "/// 🧊️ Cold-retires scratch operations (a rebased or discarded inverse): an operation may own a\n"

# (name, expected count, old, new)
STORE_HUNKS = [
    ("store:replay-doc", 1, "    /// via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.\n" + REPLAY_START, "    /// via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.\n    ///\n    /// 🩹️ Everything the replay builds — its working projection, the command's operations, their inverses — is a\n    /// [`ScratchOwner`]: a refusal on ANY exit retires all of it cold, and the caller adopts each part where the store takes it.\n" + REPLAY_START),
    ("store:append-adopts", 1, "        edit.forwards.extend(forwards);\n        edit.inverse.extend(inverse);\n", "        edit.forwards.extend(forwards.adopt());\n        edit.inverse.extend(inverse.adopt());\n"),
    ("store:current-adopts", 3, "        self.replace_current_retained(Arc::new(post))?;\n", "        self.replace_current_retained(Arc::new(post.adopt()))?;\n"),
    ("store:fingerprint", 2, "        let forwards_fingerprint = semio_framework_pack_json::to_json_string(&forwards).into_bytes();\n", "        let forwards_fingerprint = semio_framework_pack_json::to_json_string(&*forwards).into_bytes();\n"),
    (
        "store:open-edit-adopts",
        1,
        "            actor,\n            forwards,\n            inverse,\n            mutation_meta,\n            verb: self.authoring_verb.clone(),\n            line: self.envelope.active_alternative_id.clone(),\n            sequence_number: self.edit_sequence,\n            started_at,\n            finished_at: None,\n        };\n",
        "            actor,\n            forwards: forwards.adopt(),\n            inverse: inverse.adopt(),\n            mutation_meta,\n            verb: self.authoring_verb.clone(),\n            line: self.envelope.active_alternative_id.clone(),\n            sequence_number: self.edit_sequence,\n            started_at,\n            finished_at: None,\n        };\n",
    ),
    (
        "store:apply-edit-scratch",
        1,
        "        let mut edit = Edit {\n            id: mint_edit_id(self.clock.actor, self.edit_sequence, &forwards_fingerprint).await,\n            actor,\n            forwards,\n            inverse,\n            mutation_meta,\n            verb: self.authoring_verb.clone(),\n            line: self.envelope.active_alternative_id.clone(),\n            sequence_number: self.edit_sequence,\n            started_at,\n            finished_at: Some(now_iso()),\n        };\n        stamp_primary_operation_identity(&mut edit);\n        let edit_id = edit.id.clone();\n        let operations = self.operation_envelopes(&edit)?;\n        let tail = self.folded_tail(&edit);\n        let actor = edit.actor.clone();\n        self.insert_reserved_edit_history(reservation, edit)?;\n",
        "        let mut edit = ScratchOwner::new(\n            Edit {\n                id: mint_edit_id(self.clock.actor, self.edit_sequence, &forwards_fingerprint).await,\n                actor,\n                forwards: forwards.adopt(),\n                inverse: inverse.adopt(),\n                mutation_meta,\n                verb: self.authoring_verb.clone(),\n                line: self.envelope.active_alternative_id.clone(),\n                sequence_number: self.edit_sequence,\n                started_at,\n                finished_at: Some(now_iso()),\n            },\n            |edit| retire_scratch_edits::<P, Mutation>([edit]),\n        );\n        stamp_primary_operation_identity(&mut *edit);\n        let edit_id = edit.id.clone();\n        let operations = self.operation_envelopes(&*edit)?;\n        let tail = self.folded_tail(&*edit);\n        let actor = edit.actor.clone();\n        self.insert_reserved_edit_history(reservation, edit.adopt())?;\n",
    ),
    ("store:current-guard", 1, "        if Arc::ptr_eq(&self.current, &next) {\n            return Ok(());\n        }\n        let previous_is_tail = ", "        if Arc::ptr_eq(&self.current, &next) {\n            return Ok(());\n        }\n        let next = ScratchOwner::new(next, retire_shared_projection::<P, Mutation>);\n        let previous_is_tail = "),
    ("store:current-replace", 1, "        let previous = std::mem::replace(&mut *self.current, next);\n", "        let previous = std::mem::replace(&mut *self.current, next.adopt());\n"),
    ("store:scratch-owner", 1, SCRATCH_ANCHOR, SCRATCH_OWNER + SCRATCH_ANCHOR),
]

MOUNT_OLD = '#[cfg(test)]\n#[path = "../🧪️viewer-head/🦀️.rs"]\nmod viewer_head_tests;\n'
MOUNT_NEW = MOUNT_OLD + '\n#[cfg(test)]\n#[path = "../🧪️replay-retirement/🦀️.rs"]\nmod replay_retirement_tests;\n'

LAW_SOURCE = r'''//! 🩹️ Store law of scratch retirement (design §23, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): a command the store
//! refuses retires everything its replay built — the working projection, the command's operations, the inverses it derived —
//! through the technology's cold retirement, and drops nothing bare. An artifact whose projection or operation owns a
//! fail-closed root aborts the guest on a bare drop, so this is what keeps a refused edit — exactly what conflict resolution
//! produces — from killing an editor. Artifact-agnostic: the fixture is a demo operation and a demo diff with a fail-closed
//! owner's discipline whose technology counts every projection it produces and every scratch projection retired through it;
//! the cases are the language-agnostic corpus `🧫️fixtures/🧫️replay-retirement`.
use super::*;

//#region 🧰️Fixture
std::thread_local! {
    static OPERATIONS_RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static OPERATIONS_DROPPED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static INVERSES_DERIVED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static PROJECTIONS_APPLIED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static PROJECTIONS_RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn count(counter: &'static std::thread::LocalKey<std::cell::Cell<usize>>) {
    counter.with(|cell| cell.set(cell.get() + 1));
}

/// 📊️ `[operations retired cold, operations dropped bare, inverses derived, projections applied, projections retired]` so far
/// on this thread.
fn tally() -> [usize; 5] {
    [&OPERATIONS_RETIRED, &OPERATIONS_DROPPED, &INVERSES_DERIVED, &PROJECTIONS_APPLIED, &PROJECTIONS_RETIRED].map(|counter| counter.with(std::cell::Cell::get))
}

/// 🧨️ Where a [`FailClosedOp`] makes its replay fail.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Fault {
    None,
    Encode,
    Inverse,
    Apply,
    Message,
}

/// 🧿️ A demo operation with a fail-closed owner's discipline: its cold retirement is counted, and a bare drop — what a
/// refused command used to do — is counted too, so a law can prove none happened.
#[derive(Clone, Debug, PartialEq)]
struct FailClosedOp {
    operation: DemoMutation,
    fault: Fault,
    live: bool,
}

impl FailClosedOp {
    fn of(operation: DemoMutation, fault: Fault) -> Self {
        Self { operation, fault, live: true }
    }
}

impl Drop for FailClosedOp {
    fn drop(&mut self) {
        if self.live {
            count(&OPERATIONS_DROPPED);
        }
    }
}

impl ToValue for FailClosedOp {
    fn to_value(&self) -> DslValue {
        self.operation.to_value()
    }
}

impl FromValue for FailClosedOp {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoMutation::from_value(value).map(|operation| Self::of(operation, Fault::None))
    }
}

impl OpBinary for FailClosedOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        if self.fault == Fault::Encode {
            return Err(crate::os_spr::ProtocolError::LimitExceeded("the fixture refuses to encode this operation"));
        }
        self.operation.encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        DemoMutation::decode_op(bytes).map(|operation| Self::of(operation, Fault::None))
    }
}

impl OpText for FailClosedOp {
    fn print_op(&self) -> String {
        self.operation.print_op()
    }

    fn parse_op(line: &str) -> Result<Self, TextError> {
        DemoMutation::parse_op(line).map(|operation| Self::of(operation, Fault::None))
    }
}

/// 🧮️ The demo diff with a refusal switch. Its technology counts every projection it produces and every scratch projection
/// retired through it, so a law can prove each projection a replay built left through [`MutationDiff::retire_projection`].
#[derive(Clone, Debug, Default, PartialEq)]
struct FailClosedDiff {
    inner: DemoDiff,
    refuse: bool,
}

impl ToValue for FailClosedDiff {
    fn to_value(&self) -> DslValue {
        self.inner.to_value()
    }
}

impl FromValue for FailClosedDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoDiff::from_value(value).map(|inner| Self { inner, refuse: false })
    }
}

impl MutationDiff<DemoSnapshot> for FailClosedDiff {
    fn apply(&self, base: &DemoSnapshot) -> crate::os_spr::MutationApplyResult<DemoSnapshot> {
        if self.refuse {
            return Err(crate::os_spr::MutationApplyError { code: "fixture.refused".into(), message: "the fixture refuses to apply this diff".into(), target: Vec::new() });
        }
        count(&PROJECTIONS_APPLIED);
        self.inner.apply(base)
    }

    fn absorb(&mut self, other: Self) {
        self.refuse |= other.refuse;
        self.inner.absorb(other.inner);
    }

    fn retire_projection(projection: DemoSnapshot) {
        count(&PROJECTIONS_RETIRED);
        drop(projection);
    }
}

impl Mutation<DemoSnapshot> for FailClosedOp {
    type Diff = FailClosedDiff;
    const DESCRIPTORS: &'static [crate::os_spr::MutationLeafDescriptor] = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static crate::os_spr::MutationLeafDescriptor {
        self.operation.descriptor()
    }

    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<Self::Diff> {
        if self.fault == Fault::Message {
            return crate::os_spr::MutationOutcome::error("fixture.refused", "the fixture reports an error for this operation", ["n"]);
        }
        crate::os_spr::MutationOutcome::new(FailClosedDiff { inner: self.operation.diff(base).into_parts().0, refuse: self.fault == Fault::Apply })
    }

    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        if self.fault == Fault::Inverse {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "the fixture has no inverse for this operation".to_string()));
        }
        let derived = self.operation.inverse(base)?;
        Ok(derived
            .into_iter()
            .map(|operation| {
                count(&INVERSES_DERIVED);
                Self::of(operation, Fault::None)
            })
            .collect())
    }

    fn retire_cold(mut self) {
        self.live = false;
        count(&OPERATIONS_RETIRED);
    }
}

impl MemberStoreOwner<FailClosedOp> for DemoSnapshot {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> DocumentStoreOwners<Self, FailClosedOp> {
        DocumentStoreOwners::new(Arc::new(DemoSnapshotRetirementFactory), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, FailClosedOp>::new()))
    }
}
//#endregion 🧰️Fixture

//#region 🧪️Laws
/// 🩹️ LAW: a refused apply retires everything its replay built. For every corpus case — an encode failure, a refused
/// inverse, a failing diff and an error the merge policy rejects, at the first, a middle and the last operation of a command
/// — the store answers the refusal the corpus names; every operation of the command and every inverse derived from it
/// retired cold and none reached a bare `Drop`; the working projection and every projection an operation produced retired
/// through the technology; and the store shows exactly what it showed. Afterwards the store still applies an edit, and it
/// closes to its terminal-empty witness when the test ends.
#[semio_framework_async_macros::async_test]
async fn a_refused_apply_retires_everything_its_replay_built() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️replay-retirement/🔣️.json")).expect("the corpus parses");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, FailClosedOp>("demo/v1", "replay-retirement", DemoSnapshot { n: Some(0) }, None)).await;
    store.set_merge_policy(crate::os_spr::MergePolicy::Normal);
    store.dispatch(ArtifactCommand::Apply { mutations: vec![FailClosedOp::of(DemoMutation::SetN(SetN { n: 1 }), Fault::None)], transaction: None }).await.expect("a clean edit applies");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("a case name");
        let operations = usize::try_from(case["operations"].as_u64().expect("an operation count")).expect("a usize");
        let at = usize::try_from(case["at"].as_u64().expect("the failing operation")).expect("a usize");
        let fault = match case["fault"].as_str().expect("a fault") {
            "encode" => Fault::Encode,
            "inverse" => Fault::Inverse,
            "apply" => Fault::Apply,
            "message" => Fault::Message,
            other => panic!("unknown corpus fault {other}"),
        };
        let mutations: Vec<FailClosedOp> = (0..operations).map(|index| FailClosedOp::of(DemoMutation::SetN(SetN { n: 10 + index as i32 }), if index == at { fault } else { Fault::None })).collect();
        let shown = (store.snapshot_ref().n, store.applied_edit_ids().len(), store.content_revision_now());
        let before = tally();
        let refusal = match store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }).await {
            Ok(_) => panic!("{name}: the command is refused"),
            Err(VcsError::ValidationFailed(_)) => "validation-failed",
            Err(VcsError::InverseRefused(_)) => "inverse-refused",
            Err(VcsError::MutationApply(_)) => "mutation-apply",
            Err(VcsError::Rejected { .. }) => "rejected",
            Err(other) => panic!("{name}: an unexpected refusal {other}"),
        };
        let after = tally();
        let [retired, dropped, derived, applied, projections] = [0, 1, 2, 3, 4].map(|index| after[index] - before[index]);
        assert_eq!(refusal, case["refusal"].as_str().expect("a refusal"), "{name}: the refusal");
        assert_eq!(applied as u64, case["applied"].as_u64().expect("applied operations"), "{name}: operations applied before the refusal");
        assert_eq!(dropped, 0, "{name}: no operation reached a bare Drop");
        assert_eq!(retired, operations + derived, "{name}: every operation of the command and every inverse derived from it retired cold");
        assert_eq!(projections, applied + 1, "{name}: the working projection and every projection an operation produced retired through the technology");
        assert_eq!((store.snapshot_ref().n, store.applied_edit_ids().len(), store.content_revision_now()), shown, "{name}: the store shows what it showed");
    }
    store.dispatch(ArtifactCommand::Apply { mutations: vec![FailClosedOp::of(DemoMutation::SetN(SetN { n: 2 }), Fault::None)], transaction: None }).await.expect("the store still applies an edit");
    assert_eq!((store.snapshot_ref().n, store.applied_edit_ids().len()), (Some(2), 2));
}
//#endregion 🧪️Laws
'''

CORPUS_SOURCE = '''{
  "$schema": "../../🧬️schema/🔣️replay-retirement/🔣️.json",
  "schema": "semio.store.replay-retirement.v1",
  "cases": [
    { "name": "an-encode-failure-at-the-first-operation", "operations": 3, "fault": "encode", "at": 0, "refusal": "validation-failed", "applied": 0 },
    { "name": "an-encode-failure-at-the-last-operation", "operations": 3, "fault": "encode", "at": 2, "refusal": "validation-failed", "applied": 2 },
    { "name": "a-refused-inverse-in-the-middle", "operations": 3, "fault": "inverse", "at": 1, "refusal": "inverse-refused", "applied": 1 },
    { "name": "a-failing-diff-at-the-first-operation", "operations": 3, "fault": "apply", "at": 0, "refusal": "mutation-apply", "applied": 0 },
    { "name": "a-failing-diff-at-the-last-operation", "operations": 3, "fault": "apply", "at": 2, "refusal": "mutation-apply", "applied": 2 },
    { "name": "an-error-the-merge-policy-rejects", "operations": 3, "fault": "message", "at": 1, "refusal": "rejected", "applied": 3 },
    { "name": "a-single-operation-the-merge-policy-rejects", "operations": 1, "fault": "message", "at": 0, "refusal": "rejected", "applied": 1 }
  ]
}
'''

SCHEMA_SOURCE = '''{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "semio.store.replay-retirement.v1",
  "title": "Replay retirement corpus",
  "description": "Language-agnostic cases of the store's scratch-retirement law: a command of `operations` operations whose operation `at` fails by `fault` is refused as `refusal` after `applied` operations were applied to the working projection; everything the replay built is retired through the technology and nothing reaches a bare drop.",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "cases"],
  "properties": {
    "$schema": { "type": "string" },
    "schema": { "const": "semio.store.replay-retirement.v1" },
    "cases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["name", "operations", "fault", "at", "refusal", "applied"],
        "properties": {
          "name": { "type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$" },
          "operations": { "type": "integer", "minimum": 1 },
          "fault": { "description": "How the operation `at` fails: its binary encoding, its inverse, the application of its diff, or an error-level message the merge policy rejects.", "enum": ["encode", "inverse", "apply", "message"] },
          "at": { "type": "integer", "minimum": 0 },
          "refusal": { "description": "The typed refusal the store answers.", "enum": ["validation-failed", "inverse-refused", "mutation-apply", "rejected"] },
          "applied": { "description": "Operations applied to the working projection before the refusal.", "type": "integer", "minimum": 0 }
        }
      }
    }
  }
}
'''


SIDECAR = Path(__file__).resolve().parent / "🗑️generated/s5-store/rr-replay-body-before.txt"


def revert_rule():
    text = STORE.read_text(encoding="utf-8")
    if "struct ScratchOwner<T>" not in text:
        print("pending: none")
        return
    for name, expected, old, new in STORE_HUNKS:
        if text.count(new) != expected:
            raise SystemExit(f"{name}: applied form occurs {text.count(new)} times (expected {expected}): restore by hand")
    start, end = text.index(REPLAY_START), text.index(REPLAY_END)
    text = text[:start] + SIDECAR.read_text(encoding="utf-8") + text[end:]
    for name, _, old, new in reversed(STORE_HUNKS):
        text = text.replace(new, old)
    start, end = text.index(REPLAY_START), text.index(REPLAY_END)
    if hashlib.sha256(text[start:end].encode()).hexdigest() != REPLAY_OLD_DIGEST:
        raise SystemExit("the restored replay_mutations does not match its digest: nothing written")
    STORE.write_text(text, encoding="utf-8")
    print("reverted the rule")


def main():
    arguments = sys.argv[1:]
    if "--revert-rule" in arguments:
        return revert_rule()
    only = arguments[arguments.index("--only") + 1] if "--only" in arguments else None
    emit = Path(arguments[arguments.index("--emit") + 1]) if "--emit" in arguments else None
    writes, pending, before = {}, [], None
    if only in (None, "rule"):
        text = STORE.read_text(encoding="utf-8")
        if "struct ScratchOwner<T>" not in text:
            start, end = text.index(REPLAY_START), text.index(REPLAY_END)
            before = text[start:end]
            digest = hashlib.sha256(before.encode()).hexdigest()
            if digest != REPLAY_OLD_DIGEST:
                raise SystemExit(f"store:replay-body: replay_mutations changed (digest {digest}): re-derive the wave")
            for name, expected, old, new in STORE_HUNKS:
                if text.count(old) != expected:
                    raise SystemExit(f"{name}: anchor occurs {text.count(old)} times (expected {expected}): re-derive the wave")
            for name, _, old, new in STORE_HUNKS:
                text = text.replace(old, new)
                pending.append(name)
            start, end = text.index(REPLAY_START), text.index(REPLAY_END)
            text = text[:start] + REPLAY_NEW + text[end:]
            pending.append("store:replay-body")
            writes[STORE] = text
    if only in (None, "law"):
        unit = UNIT.read_text(encoding="utf-8")
        if MOUNT_NEW not in unit:
            if unit.count(MOUNT_OLD) != 1:
                raise SystemExit(f"law:mount: anchor occurs {unit.count(MOUNT_OLD)} times (expected 1): re-derive the wave")
            writes[UNIT] = unit.replace(MOUNT_OLD, MOUNT_NEW)
            pending.append("law:mount")
        for path, source, name in ((LAW, LAW_SOURCE, "law:file"), (CORPUS, CORPUS_SOURCE, "law:corpus"), (SCHEMA, SCHEMA_SOURCE, "law:schema")):
            if not path.exists() or path.read_text(encoding="utf-8") != source:
                writes[path] = source
                pending.append(name)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if emit is not None:
        emit.mkdir(parents=True, exist_ok=True)
        for index, (path, text) in enumerate(writes.items()):
            (emit / f"{index}-{path.parent.name}{path.suffix}").write_text(text, encoding="utf-8")
        print(f"emitted {len(writes)} files to {emit}")
        return
    if "--check" in arguments or not pending:
        return
    if before is not None:
        SIDECAR.parent.mkdir(parents=True, exist_ok=True)
        SIDECAR.write_text(before, encoding="utf-8")
    for path, text in writes.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    print(f"applied {len(pending)} parts in {len(writes)} files")


main()
