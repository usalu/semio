"""✅️ C12 session 14b: TS twin of the Rust `settle_committed_envelopes` (parity scenario `lost-ack-committed-op-settles`) — a
`Commands` frame carrying an operation still unacknowledged proves the hub committed it, so it is never resent (a re-stamped resend is
refused as a replayed operation). Also drops the resume-fixture case no lease can express. One-off, idempotent."""
import json
import sys

WORKER = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
FIXTURE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json"

fixture = json.load(open(FIXTURE, encoding="utf-8"))
fixture["cases"] = [row for row in fixture["cases"] if row["case"] != "a checkpoint of another descriptor"]
with open(FIXTURE, "w", encoding="utf-8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=2)
    handle.write("\n")

text = open(WORKER, encoding="utf-8").read()
if "function settleCommittedEnvelopes(" in text:
    print("already applied")
    sys.exit(0)


def sub(old: str, new: str) -> None:
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


sub('''    const fresh = admitRemoteEnvelopes(state.ingestedMutationIds, frame.Commands.envelopes, wireEnvelopeId);
    if (fresh.length > 0 && commandBatch === null) throw new Error("document backbone: exact server command batch missing");''', '''    settleCommittedEnvelopes(state, frame.Commands.envelopes.map(wireEnvelopeId));
    const fresh = admitRemoteEnvelopes(state.ingestedMutationIds, frame.Commands.envelopes, wireEnvelopeId);
    if (fresh.length > 0 && commandBatch === null) throw new Error("document backbone: exact server command batch missing");''')

sub('''function requeuePendingBatches(state: ArtifactState): void {''', '''/** ✅️ Settles this replica's own operations the hub's log already holds: a `Commands` frame (a catch-up tail or a relay) that carries
 * an operation still in the outbox or a pending batch proves the hub committed it although its `Ack` was lost with the socket, so it
 * leaves both (an emptied batch leaves too), its retention is released, its echo stays suppressed, and it is never resent — a
 * resend is re-stamped and the hub refuses it as a replayed operation. TS twin of the Rust `settle_committed_envelopes`, both
 * replaying the parity scenario `lost-ack-committed-op-settles` (`🔄️sync/⚖️parity/🧫️fixtures`). */
function settleCommittedEnvelopes(state: ArtifactState, committedIds: readonly string[]): void {
  const committed = new Set(committedIds);
  const settled = new Set<MutationEnvelope>();
  const keep = (envelope: MutationEnvelope): boolean => {
    if (!committed.has(state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id)) return true;
    settled.add(envelope);
    return false;
  };
  state.outbox = state.outbox.filter(keep);
  for (const [batchId, batch] of [...state.pendingBatches]) {
    const kept = batch.filter(keep);
    if (kept.length === 0) state.pendingBatches.delete(batchId);
    else state.pendingBatches.set(batchId, kept);
  }
  if (settled.size === 0) return;
  releaseDocumentBackboneOwnership(state, [...settled]);
  state.pendingMutations = state.pendingMutations.filter((envelope) => !settled.has(envelope));
  setStatus(state, { pendingMutations: state.pendingMutations.length });
  flushMutationsToHubIfReady(state);
}

function requeuePendingBatches(state: ArtifactState): void {''')

open(WORKER, "w", encoding="utf-8").write(text)
print("applied")
