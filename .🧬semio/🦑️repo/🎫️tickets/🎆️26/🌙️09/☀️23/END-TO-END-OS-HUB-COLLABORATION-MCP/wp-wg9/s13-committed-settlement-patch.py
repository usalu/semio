#!/usr/bin/env python3
"""✅️ WG9 session 13 — a relink must never resend an operation the hub already committed, and a rollback never feeds a history
transition's empty inverse to a guest (kernel `🏪️store/🔄️sync/🦀️.rs`, native AND wasm32 document actors).

Measured live (run s13b, 7800 B3, wasm32 note, decoded frames): A's edit `edit-69c5…#0` was sent (+282.1 s) and committed (hub head 3,
B shows it) but its `Ack` was lost with the socket. On relink the hub's catch-up tail carried that very op; the actor re-queued it
from `pending_batches` anyway and RESENT it re-stamped (HLC logical 2 → 3), the hub refused it as a replayed operation, the rejection
rolled the batch back, `rollback_envelope` turned the batch's history transition (inverse = empty by design — a transition is undone
by a later transition) into an empty `semio.history.transition` payload, the guest's vcs module failed "truncated at offset 0", and
the shell retired the document (terminal fault).

1. `settle_committed_envelopes` (region 🔁️DocumentEchoSuppression, target-neutral): a `Commands` frame (tail or relay) carrying an
   operation still in the outbox or a pending batch proves the hub committed it → it leaves both (empty batches too) and its backbone
   retention is released; both actors call it before admission.
2. `rollback_envelope` answers `None` for a history transition, so no rollback path can build an undecodable payload.
Laws: the language-agnostic backbone-parity fixture gains `lost-ack-committed-op-settles` (queue → send → socket lost before the
Ack → relink → the tail carries the op → outbox and pending stay empty, no second send; red without the fix), walked by the native
actor's `backbone_parity_scenarios_match_neutral_fixture`; unit laws `a_committed_own_operation_settles_the_outbox_and_its_pending_batch`,
`both_actors_settle_committed_operations_before_admission`, `a_history_transition_has_no_inverse_rollback`; the existing rollback law
reads the `Some`.

Dry run by default; `--apply` writes. Every replacement asserts its anchor occurs exactly once.
"""

import difflib
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SYNC = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"
UNIT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs"
LAW = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️document-echo-suppression/🦀️.rs"
PARITY = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚖️parity/🧫️fixtures/🔣️.json"

SETTLE = '''/// @emoji ✅️ Settles this replica's own operations the hub's log already holds: a `Commands` frame (a catch-up tail or a relay)
/// that carries an operation still in `outbox` or a `pending` batch proves the hub committed it although its `Ack` was lost with the
/// socket, so it leaves both (a batch left empty leaves too) and is never resent — a resend is re-stamped and the hub refuses it as a
/// replayed operation (ticket 26/09/23 session 13, run s13b). Answers the settled envelopes, whose backbone retention the caller
/// releases.
pub fn settle_committed_envelopes(outbox: &mut Vec<MutationEnvelope>, pending: &mut std::collections::HashMap<u64, Vec<MutationEnvelope>>, committed: &[MutationEnvelope]) -> Vec<MutationEnvelope> {
    let committed: std::collections::HashSet<&str> = committed.iter().map(|envelope| envelope.mutation_id.0.as_str()).collect();
    let mut settled = Vec::new();
    let mut keep = |envelope: &MutationEnvelope| !committed.contains(envelope.mutation_id.0.as_str()) || {
        settled.push(envelope.clone());
        false
    };
    outbox.retain(&mut keep);
    pending.retain(|_, batch| {
        batch.retain(&mut keep);
        !batch.is_empty()
    });
    settled
}
//#endregion 🔁️DocumentEchoSuppression'''

SYNC_EDITS = [
    ("//#endregion 🔁️DocumentEchoSuppression", SETTLE),
    (
        """async fn rollback_envelope(envelope: &MutationEnvelope) -> MutationEnvelope {
    let undo_id = MutationId(format!("{}~undo", envelope.mutation_id.0));
    MutationEnvelope {""",
        """async fn rollback_envelope(envelope: &MutationEnvelope) -> Option<MutationEnvelope> {
    if crate::os_spr::is_history_transition(envelope) {
        return None;
    }
    let undo_id = MutationId(format!("{}~undo", envelope.mutation_id.0));
    Some(MutationEnvelope {""",
    ),
    (
        """        inverse: crate::os_spr::InverseMutation { schema: envelope.diff.schema.clone(), payload: envelope.diff.payload.clone() },
        timestamp: envelope.timestamp,
    }
}""",
        """        inverse: crate::os_spr::InverseMutation { schema: envelope.diff.schema.clone(), payload: envelope.diff.payload.clone() },
        timestamp: envelope.timestamp,
    })
}""",
    ),
    (
        """                    let persisted = &self.known_op_ids;
                    let fresh = admit_remote_envelopes(""",
        """                    let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                    self.document_backbone_retention.release(&settled);
                    note_authored_envelopes(&mut self.applied_op_ids, &settled);
                    let persisted = &self.known_op_ids;
                    let fresh = admit_remote_envelopes(""",
    ),
    (
        """                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion");
                        return;
                    }
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);""",
        """                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion");
                        return;
                    }
                    let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                    self.document_backbone_retention.release(&settled);
                    note_authored_envelopes(&mut self.applied_op_ids, &settled);
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);""",
    ),
]

ROLLBACK_LOOP_OLD = "rollbacks.push(rollback_envelope(envelope).await);"
ROLLBACK_LOOP_NEW = "rollbacks.extend(rollback_envelope(envelope).await);"

UNIT_EDITS = [
    (
        "    let rollback = rollback_envelope(&envelope).await;\n",
        "    let rollback = rollback_envelope(&envelope).await.expect(\"a domain operation rolls back by its inverse\");\n",
    ),
]

LAW_APPEND = '''

fn settlement_envelope(id: &str) -> MutationEnvelope {
    envelope(&serde_json::json!({ "mutationId": id, "actor": "hub.v1.author" }))
}

#[test]
fn a_committed_own_operation_settles_the_outbox_and_its_pending_batch() {
    let mut outbox = vec![settlement_envelope("edit-a#0"), settlement_envelope("transition-b")];
    let mut pending = std::collections::HashMap::from([(7_u64, vec![settlement_envelope("edit-c#0")]), (8_u64, vec![settlement_envelope("edit-d#0"), settlement_envelope("edit-e#0")])]);
    let tail = [settlement_envelope("edit-a#0"), settlement_envelope("edit-c#0"), settlement_envelope("edit-d#0"), settlement_envelope("edit-peer#0")];
    let mut settled: Vec<String> = super::settle_committed_envelopes(&mut outbox, &mut pending, &tail).into_iter().map(|envelope| envelope.mutation_id.0).collect();
    settled.sort();
    assert_eq!(settled, ["edit-a#0", "edit-c#0", "edit-d#0"], "every own operation the hub's log holds is settled, a peer's is not ours to settle");
    assert_eq!(outbox.iter().map(|envelope| envelope.mutation_id.0.as_str()).collect::<Vec<_>>(), ["transition-b"], "only the uncommitted work stays queued");
    assert!(!pending.contains_key(&7), "a batch the tail fully committed leaves");
    assert_eq!(pending[&8].iter().map(|envelope| envelope.mutation_id.0.as_str()).collect::<Vec<_>>(), ["edit-e#0"], "a partly committed batch keeps only its uncommitted rest");
}

#[test]
fn both_actors_settle_committed_operations_before_admission() {
    let source = include_str!("../../🦀️.rs");
    assert_eq!(source.matches("let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);").count(), 2, "native and wasm32 Commands arms settle");
    assert_eq!(source.matches("rollbacks.push(rollback_envelope(").count(), 0, "no rollback path pushes an unchecked rollback");
}

#[semio_framework_async_macros::async_test]
async fn a_history_transition_has_no_inverse_rollback() {
    let transition = crate::os_spr::HistoryTransition::Revert { mutation_ids: vec![MutationId("edit-a#0".into())] };
    let envelope = crate::os_spr::history_transition_envelope(&transition, &ArtifactId("artifact-echo".into()), &ActorId("hub.v1.author".into()), Vec::new(), crate::os_spr::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 });
    assert!(super::rollback_envelope(&envelope).await.is_none(), "a transition is undone by a later transition, never by its (empty) inverse");
}
'''


PARITY_SCENARIO = {
    "id": "lost-ack-committed-op-settles",
    "title": "A Committed Op Whose Ack Was Lost Settles From The Tail And Is Never Resent",
    "documentId": "parity-doc",
    "spaceId": "parity-space",
    "actor": "local-actor",
    "steps": [
        {"op": "localDispatch", "dispatch": {"kind": "queueMutation", "mutationId": "lost-1", "n": 1}},
        {"op": "localDispatch", "dispatch": {"kind": "connectSocket", "actor": "hub.v1.eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"}},
        {"op": "serverFrame", "frame": {"kind": "welcome", "sessionId": "s7", "resumeToken": "resume-7", "frontier": {"documentId": "parity-doc", "headEditOrdinal": 0, "headEditId": "genesis", "lastCommitSeq": 0, "chainHashByte": 1}, "bootstrap": "none"}},
        {"op": "serverFrame", "frame": {"kind": "session", "actor": "hub.v1.eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", "color": 2}},
        {"op": "expect", "expect": {"outboxMutationIds": [], "pendingBatchCount": 1}},
        {"op": "localDispatch", "dispatch": {"kind": "failConnection"}},
        {"op": "localDispatch", "dispatch": {"kind": "connectSocket", "actor": "hub.v1.eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"}},
        {"op": "serverFrame", "frame": {"kind": "welcome", "sessionId": "s8", "resumeToken": "resume-8", "frontier": {"documentId": "parity-doc", "headEditOrdinal": 1, "headEditId": "lost-1", "lastCommitSeq": 1, "chainHashByte": 2}, "bootstrap": "tail"}},
        {"op": "serverFrame", "frame": {"kind": "commands", "origin": "hub.catch-up", "envelopes": [{"mutationId": "lost-1", "n": 1}], "frontier": {"documentId": "parity-doc", "headEditOrdinal": 1, "headEditId": "lost-1", "lastCommitSeq": 1, "chainHashByte": 2}}},
        {"op": "expect", "expect": {"outboxMutationIds": [], "pendingBatchCount": 0, "frontierEditId": "lost-1"}},
        {"op": "serverFrame", "frame": {"kind": "session", "actor": "hub.v1.eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", "color": 2}},
        {"op": "expect", "expect": {"outboxMutationIds": [], "pendingBatchCount": 0, "socketActorConfirmed": True}},
    ],
}


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    apply = "--apply" in sys.argv
    plans = []
    sync_before = SYNC.read_text(encoding="utf-8")
    sync_after = replaced(SYNC, sync_before, SYNC_EDITS)
    loops = sync_after.count(ROLLBACK_LOOP_OLD)
    if loops < 4:
        sys.exit(f"expected every rollback loop (≥ 4), found {loops}")
    sync_after = sync_after.replace(ROLLBACK_LOOP_OLD, ROLLBACK_LOOP_NEW)
    plans.append((SYNC, sync_before, sync_after))
    unit_before = UNIT.read_text(encoding="utf-8")
    plans.append((UNIT, unit_before, replaced(UNIT, unit_before, UNIT_EDITS)))
    law_before = LAW.read_text(encoding="utf-8")
    if "settle_committed_envelopes" in law_before:
        sys.exit("law already carries the settlement laws")
    plans.append((LAW, law_before, law_before + LAW_APPEND))
    parity_before = PARITY.read_text(encoding="utf-8")
    parity = json.loads(parity_before)
    if any(row["id"] == PARITY_SCENARIO["id"] for row in parity["scenarios"]):
        sys.exit("parity fixture already carries the lost-ack scenario")
    parity["scenarios"].append(PARITY_SCENARIO)
    plans.append((PARITY, parity_before, json.dumps(parity, indent=2, ensure_ascii=False) + "\n"))
    for path, before, after in plans:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), path.name, path.name + " (patched)", n=1))
        if apply:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'APPLIED' if apply else 'DRY RUN'}: {len(plans)} files, {loops} rollback loops")


if __name__ == "__main__":
    main()
