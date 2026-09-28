"""✂️ C12 session 14b: rewrites the worker's hub outbox drain (ack-clocked, envelope + byte bounded, in-flight batches requeued
at the front). One-off, idempotent: refuses when the anchors are missing, no-ops when already applied."""
import sys
path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
text = open(path, encoding="utf-8").read()
start = text.index("/** 📦️ Most envelopes one `Commands` batch carries while the outbox drains")
end_anchor = "  state.pendingBatches.set(batchId, [...envelopes]);\n  sendWireFrame(state, { Commands: { batch_id: batchId, envelopes: wireEnvelopes } }, \"command\");\n}\n"
end = text.index(end_anchor, start) + len(end_anchor)
new = '''/** 📦️ Bounds of one `Commands` batch. The outbox drains ack-clocked — one batch in flight per document, the next after its `Ack` —
 * in batches of at most this many envelopes and {@link HUB_OUTBOX_BATCH_BYTES} wire bytes, so a queue that grew during a
 * connection shortage never reaches the hub as one oversized batch (ticket 26/09/23 C12, run `c12short3`: a whole-outbox batch
 * was refused `DB I/O aggregate admission exhausted` and every keystroke typed during the cut was lost), and nothing is ever sent
 * behind a batch whose outcome is unknown: a transiently refused batch goes out again FIRST, and a later batch whose envelopes
 * depend on it can never reach the hub before it (the hub refuses those for good: `dependency names an unknown edit`). */
const HUB_OUTBOX_BATCH_ENVELOPES = 16;
/** 📦️ Wire bytes of one drained batch: half the declared batch maximum, as the Rust store's `announce_history`. One envelope larger
 * than this still goes out alone. */
const HUB_OUTBOX_BATCH_BYTES = DOCUMENT_BACKBONE_BATCH_LIMITS.maximumBytes / 2;
/** ⏳️ Resend delay after a transient refusal: doubled per consecutive refusal between these bounds, jittered. */
const HUB_TRANSIENT_REFUSAL_MIN_MS = 250;
const HUB_TRANSIENT_REFUSAL_MAX_MS = 5_000;

/** 🚿️ Sends the outbox's next bounded batch once the socket is live, no batch awaits its `Ack` and no transient refusal is backing
 * off. Called on every relay, `Ack`, authenticated `Session`, catch-up completion, link restore and refusal timer. */
function flushMutationsToHubIfReady(state: ArtifactState): void {
  if (!documentBackboneRelayReady(state) || state.outbox.length === 0 || state.pendingBatches.size > 0 || state.transientRefusal?.timer != null) return;
  const limit = state.transientRefusal?.batchLimit ?? HUB_OUTBOX_BATCH_ENVELOPES;
  const envelopes: MutationEnvelope[] = [];
  const wireEnvelopes: WireMutationEnvelope[] = [];
  let bytes = 0;
  for (const envelope of state.outbox) {
    if (envelopes.length === limit) break;
    const wire = hubWireEnvelope(state, envelope);
    const size = wire.mutation_id.length + wire.document_id.length + wire.actor.length + wire.diff.schema.length + wire.diff.payload.length + wire.inverse.schema.length + wire.inverse.payload.length + wire.dependencies.reduce((sum, dependency) => sum + dependency.length, 0) + wire.target.reduce((sum, segment) => sum + segment.length, 0);
    if (envelopes.length > 0 && bytes + size > HUB_OUTBOX_BATCH_BYTES) break;
    bytes += size;
    envelopes.push(envelope);
    wireEnvelopes.push(wire);
  }
  state.outbox.splice(0, envelopes.length);
  const batchId = state.nextBatchId;
  state.nextBatchId += 1;
  state.pendingBatches.set(batchId, envelopes);
  sendWireFrame(state, { Commands: { batch_id: batchId, envelopes: wireEnvelopes } }, "command");
}

/** 🌉️ One outbound envelope as this socket's actor sends it: the exact causal envelope a bound port authored, re-stamped, or the TS
 * twin's {@link toWireEnvelope}. */
function hubWireEnvelope(state: ArtifactState, envelope: MutationEnvelope): WireMutationEnvelope {
  const exact = state.exactLocalEnvelopes.get(envelope)?.envelope;
  const timestamp = nextWireTimestamp(state);
  if (exact === undefined) return toWireEnvelope(envelope, timestamp, state.actor);
  return {
    mutation_id: exact.mutation_id,
    document_id: exact.document_id,
    actor: state.actor,
    dependencies: [...exact.dependencies],
    observed: exact.observed,
    target: [...exact.target],
    diff: { schema: exact.diff.schema, payload: Array.from(exact.diff.payload) },
    inverse: { schema: exact.inverse.schema, payload: Array.from(exact.inverse.payload) },
    timestamp,
  };
}

/** ⏮️ Puts envelopes that were sent but never applied back at the FRONT of the outbox, in their send order: they precede everything
 * queued behind them. */
function returnToOutboxFront(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  const ids = new Set(envelopes.map((envelope) => envelope.id));
  state.outbox = [...envelopes, ...state.outbox.filter((envelope) => !ids.has(envelope.id))];
}

/** ⏳️ Keeps a batch the hub refused for a transient reason: its envelopes return to the front of the outbox (nothing was applied,
 * nothing is rolled back or rebuilt), the next drain is bounded to half the refused batch and resent after a jittered backoff. */
function resendAfterTransientRefusal(state: ArtifactState, sent: readonly MutationEnvelope[]): void {
  returnToOutboxFront(state, sent);
  const previous = state.transientRefusal;
  if (previous?.timer != null) clearTimeout(previous.timer);
  const backoffMs = Math.min(HUB_TRANSIENT_REFUSAL_MAX_MS, Math.max(HUB_TRANSIENT_REFUSAL_MIN_MS, (previous?.backoffMs ?? 0) * 2));
  const refusal: NonNullable<ArtifactState["transientRefusal"]> = { timer: null, backoffMs, batchLimit: Math.max(1, Math.ceil(sent.length / 2)) };
  refusal.timer = setTimeout(() => {
    refusal.timer = null;
    if (state.transientRefusal === refusal) flushMutationsToHubIfReady(state);
  }, backoffMs / 2 + Math.random() * (backoffMs / 2));
  state.transientRefusal = refusal;
  setStatus(state, { pendingMutations: state.pendingMutations.length });
}

/** 🧺️ Hands locally authored envelopes to the hub: they join the outbox and {@link flushMutationsToHubIfReady} drains it.
 * Mirrors the Rust actor's `relay_operations_to_hub`. Finding 5: a closed socket no longer no-ops silently — the envelopes wait
 * in {@link ArtifactState.outbox}, and {@link handleHubFrame}'s authenticated `Session` branch drains that outbox only after the
 * grant actor is proven, so nothing is lost or sent pre-authority. */
function relayMutationsToHub(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  // 🔒️ A verified read-only execution target rejects publication locally, before a worker frame or
  // an outbox entry exists. Server authorization stays an independent fence.
  if (state.executionTargetLease !== null && state.executionTargetLease.live && !state.executionTargetLease.fields().grant.write) {
    releaseDocumentBackboneOwnership(state, envelopes);
    const refusedIds = new Set(envelopes.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !refusedIds.has(envelope.id));
    state.outbox = state.outbox.filter((envelope) => !refusedIds.has(envelope.id));
    setStatus(state, { pendingMutations: state.pendingMutations.length });
    rejectReadOnlyExecutionTarget(state, envelopes);
    return;
  }
  queueOutbox(state, envelopes);
  flushMutationsToHubIfReady(state);
}
'''
if text[start:end] == new:
    print("already applied"); sys.exit(0)
text = text[:start] + new + text[end:]
old_requeue = '''function requeuePendingBatches(state: ArtifactState): void {
  const batches = [...state.pendingBatches.entries()].sort(([left], [right]) => left - right);
  state.pendingBatches.clear();
  for (const [, envelopes] of batches) queueOutbox(state, envelopes);
}'''
new_requeue = '''function requeuePendingBatches(state: ArtifactState): void {
  const batches = [...state.pendingBatches.entries()].sort(([left], [right]) => left - right);
  state.pendingBatches.clear();
  returnToOutboxFront(state, batches.flatMap(([, envelopes]) => envelopes));
}'''
if text.count(old_requeue) != 1: sys.exit("requeue anchor missing")
text = text.replace(old_requeue, new_requeue)
old_import = "import { ArtifactBootstrapAssembler, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, DOCUMENT_BACKBONE_RETENTION_LIMITS,"
if text.count(old_import) != 1: sys.exit("import anchor missing")
text = text.replace(old_import, "import { ArtifactBootstrapAssembler, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, DOCUMENT_BACKBONE_BATCH_LIMITS, DOCUMENT_BACKBONE_RETENTION_LIMITS,")
open(path, "w", encoding="utf-8").write(text)
print("applied")
