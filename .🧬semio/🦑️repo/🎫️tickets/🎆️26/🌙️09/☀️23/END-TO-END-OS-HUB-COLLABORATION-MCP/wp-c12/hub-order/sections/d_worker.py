# 👷️ Section D: the worker hands each hub frame to the store in the hub's order (other authors' operations `sequenced`, this
# replica's own settled ones `committed`), marks every decided batch `committed` before any correction, hands the hub's
# rollback of a refused batch over `sequenced`, and no longer rebuilds a document on a reorder — the store rebases its
# undecided operations itself.

edit(WORKER, """  parseDocumentBackboneMessage,
  parseDocumentSocketGrantReceiptV1,""", """  parseDocumentBackboneInboundMessage,
  parseDocumentBackboneMessage,
  parseDocumentSocketGrantReceiptV1,""", "worker imports store-bound admission")

edit(WORKER, """  /** 🔀️ Another human's operations were folded in while this shell's own operations were still unacknowledged. The hub
   * orders those remote operations BEFORE the pending local ones, but the local fold applied them after, so once the local
   * ones are accepted the document is rebuilt from the hub's authoritative pair: measured as two humans each showing the
   * OTHER's value for the same field forever after a short link loss (🎫️ 26/09/23 C10). */
  remoteFoldedOverLocal: boolean;
""", "", "worker drops the reorder backstop field")
edit(WORKER, """    browserActorBackboneBeforeReservation: [],
    remoteFoldedOverLocal: false,
""", """    browserActorBackboneBeforeReservation: [],
""", "worker drops the reorder backstop initializer")
edit(WORKER, """async function requireArtifactRebootstrap(state: ArtifactState): Promise<void> {
  state.remoteFoldedOverLocal = false;
  const owner""", """async function requireArtifactRebootstrap(state: ArtifactState): Promise<void> {
  const owner""", "worker rebootstrap no longer resets the backstop")

edit(WORKER, """  private retainBackboneBeforeBinding(bytes: Uint8Array): void {
    const parsed = parseDocumentBackboneMessage(bytes);""", """  private retainBackboneBeforeBinding(bytes: Uint8Array): void {
    const parsed = parseDocumentBackboneInboundMessage(bytes);""", "worker child retains store-bound messages")
edit(WORKER, """      this.retainBackboneBeforeBinding(bytes);
      return;
    }
    const parsed = parseDocumentBackboneMessage(bytes);""", """      this.retainBackboneBeforeBinding(bytes);
      return;
    }
    const parsed = parseDocumentBackboneInboundMessage(bytes);""", "worker child receives store-bound messages")

edit(WORKER, """function documentBackboneMessage(envelopes: readonly ExactWireMutationEnvelope[]): Uint8Array {
  return encodeBackboneMessage({ kind: "mutations", envelopes: encodeDocumentBackboneEnvelopeBatchExact(envelopes) });
}

function documentBackboneMessageFromDomain(state: ArtifactState, envelopes: readonly MutationEnvelope[]): Uint8Array {
  return documentBackboneMessage(envelopes.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope ?? exactWireEnvelope(toWireEnvelope(envelope, nextWireTimestamp(state)))));
}

function emitMutationEvent(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  if (hubBinding(state.config)) emitEvent(state, { kind: "documentBackbone", message: documentBackboneMessageFromDomain(state, envelopes) });
  else emitEvent(state, { kind: "remoteMutations", envelopes });
}""", """/** 📨️ One store-bound batch: `sequenced` when the hub placed it (its order is final), `mutations` otherwise. */
function documentBackboneMessage(envelopes: readonly ExactWireMutationEnvelope[], kind: "mutations" | "sequenced"): Uint8Array {
  return encodeBackboneMessage({ kind, envelopes: encodeDocumentBackboneEnvelopeBatchExact(envelopes) });
}

function documentBackboneMessageFromDomain(state: ArtifactState, envelopes: readonly MutationEnvelope[], kind: "mutations" | "sequenced"): Uint8Array {
  return documentBackboneMessage(envelopes.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope ?? exactWireEnvelope(toWireEnvelope(envelope, nextWireTimestamp(state)))), kind);
}

function emitMutationEvent(state: ArtifactState, envelopes: readonly MutationEnvelope[], kind: "mutations" | "sequenced"): void {
  if (envelopes.length === 0) return;
  if (hubBinding(state.config)) emitEvent(state, { kind: "documentBackbone", message: documentBackboneMessageFromDomain(state, envelopes, kind) });
  else emitEvent(state, { kind: "remoteMutations", envelopes });
}""", "worker store-bound batches name their order")
edit(WORKER, """  if (state.browserActorReservation === null) emitMutationEvent(state, replacement === null ? rollback : [...rollback, replacement]);""",
     """  if (state.browserActorReservation === null) emitMutationEvent(state, replacement === null ? rollback : [...rollback, replacement], "sequenced");""", "worker hands the hub's correction over in the hub's order")
edit(WORKER, """      const retainedBackbone = state.outbox.length === 0 ? [] : [documentBackboneMessageFromDomain(state, state.outbox)];""",
     """      const retainedBackbone = state.outbox.length === 0 ? [] : [documentBackboneMessageFromDomain(state, state.outbox, "mutations")];""", "worker replays its own queue into a rebuilt child as mutations")
edit(WORKER, """    if (state.outbox.length > 0 && coldOwner === null) {
      emitMutationEvent(state, [...state.outbox]);""", """    if (state.outbox.length > 0 && coldOwner === null) {
      emitMutationEvent(state, [...state.outbox], "mutations");""", "worker replays its own queue into a rebuilt instance as mutations")

edit(WORKER, """    releaseDocumentBackboneOwnership(state, sent);
    const sentIds = new Set(sent.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !sentIds.has(envelope.id));

    let ackOutcome: CommandAckOutcome;
    let reorder = false;
    if (outcome === "Accepted") {
      if (state.transientRefusal?.timer != null) clearTimeout(state.transientRefusal.timer);
      state.transientRefusal = null;
      ackOutcome = { kind: "accepted" };
      reorder = state.remoteFoldedOverLocal && state.pendingMutations.length === 0 && documentAwaitsBrowserActor(state);
    } else if""", """    const committedIds = sent.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id);
    releaseDocumentBackboneOwnership(state, sent);
    const sentIds = new Set(sent.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !sentIds.has(envelope.id));
    if ((outcome === "Accepted" || state.browserActorReservation === null) && !(await forwardDocumentBackboneV1(state, encodeBackboneMessage({ kind: "committed", opIds: committedIds })))) return;

    let ackOutcome: CommandAckOutcome;
    if (outcome === "Accepted") {
      if (state.transientRefusal?.timer != null) clearTimeout(state.transientRefusal.timer);
      state.transientRefusal = null;
      ackOutcome = { kind: "accepted" };
    } else if""", "worker commits a decided batch before any correction the store takes in place")
edit(WORKER, """    setStatus(state, { pendingMutations: state.pendingMutations.length });
    emitEvent(state, { kind: "commandOutcome", batchId, outcome: ackOutcome });
    if (reorder) await requireArtifactRebootstrap(state);
  }""", """    setStatus(state, { pendingMutations: state.pendingMutations.length });
    emitEvent(state, { kind: "commandOutcome", batchId, outcome: ackOutcome });
  }""", "worker drops the reorder rebuild")

edit(WORKER, """function settleCommittedEnvelopes(state: ArtifactState, committedIds: readonly string[]): void {
  const committed = new Set(committedIds);
  const settled = new Set<MutationEnvelope>();
  const keep = (envelope: MutationEnvelope): boolean => {
    if (!committed.has(state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id)) return true;
    settled.add(envelope);
    return false;
  };""", """function settleCommittedEnvelopes(state: ArtifactState, committedIds: readonly string[]): ReadonlySet<string> {
  const committed = new Set(committedIds);
  const settled = new Set<MutationEnvelope>();
  const settledIds = new Set<string>();
  const keep = (envelope: MutationEnvelope): boolean => {
    const id = state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id;
    if (!committed.has(id)) return true;
    settled.add(envelope);
    settledIds.add(id);
    return false;
  };""", "worker settle answers the settled ids")
edit(WORKER, """  if (settled.size === 0) return;
  releaseDocumentBackboneOwnership(state, [...settled]);
  state.pendingMutations = state.pendingMutations.filter((envelope) => !settled.has(envelope));
  setStatus(state, { pendingMutations: state.pendingMutations.length });
  flushMutationsToHubIfReady(state);
}""", """  if (settled.size === 0) return settledIds;
  releaseDocumentBackboneOwnership(state, [...settled]);
  state.pendingMutations = state.pendingMutations.filter((envelope) => !settled.has(envelope));
  setStatus(state, { pendingMutations: state.pendingMutations.length });
  flushMutationsToHubIfReady(state);
  return settledIds;
}

/** 🧭️ One run of a hub `Commands` frame, in the frame's order — which is the hub's order. */
export type HubOrderSegmentV1 = Readonly<{ kind: "committed" | "sequenced"; ids: readonly string[] }>;

/** 🧭️ Splits a hub frame's operation ids into its ordered runs: `settled` ids (this replica's own operations the hub already
 * holds — a lost ack's settle) become `committed` runs, `fresh` ids (other authors' operations) `sequenced` runs, every other
 * id (an operation this replica already holds) is skipped. The store folds every `sequenced` run before its operations the hub
 * has not decided yet, so a frame must reach it run by run. Rust twin: `hub_order_segments` (`🔄️sync`); both replay the
 * parity scenarios `hub-order-*` (`🔄️sync/⚖️parity/🧫️fixtures/🔣️.json`). */
export function hubOrderSegmentsV1(frame: readonly string[], settled: ReadonlySet<string>, fresh: ReadonlySet<string>): HubOrderSegmentV1[] {
  const segments: { kind: "committed" | "sequenced"; ids: string[] }[] = [];
  for (const id of frame) {
    const kind = settled.has(id) ? "committed" : fresh.has(id) ? "sequenced" : null;
    if (kind === null) continue;
    const last = segments.at(-1);
    if (last?.kind === kind) last.ids.push(id);
    else segments.push({ kind, ids: [id] });
  }
  return segments;
}

/** 📡️ Hands one store-bound message to the document's store: its browser actor child (held until the child binds), or the
 * Shell's local instance. A child that refuses it reopens from the last confirmed state; answers whether it was taken. */
async function forwardDocumentBackboneV1(state: ArtifactState, message: Uint8Array): Promise<boolean> {
  const reservation = state.browserActorReservation;
  if (reservation === null && documentAwaitsBrowserActor(state)) retainBackboneBeforeBrowserActor(state, message);
  else if (reservation === null) emitEvent(state, { kind: "documentBackbone", message });
  else {
    try {
      await reservation.receiveBackbone(message);
    } catch (error) {
      console.error("[backbone-worker] the browser actor child refused an inbound hub frame; reopening from the last confirmed state", state.config.documentId, error);
      requestDocumentActorRecoveryV1(state, "inbound-frame");
      return false;
    }
  }
  return true;
}""", "worker hub-order segments and store-bound forwarding")

edit(WORKER, """    settleCommittedEnvelopes(state, frame.Commands.envelopes.map(wireEnvelopeId));
    const fresh = admitRemoteEnvelopes(state.ingestedMutationIds, frame.Commands.envelopes, wireEnvelopeId);
    if (fresh.length > 0 && commandBatch === null) throw new Error("document backbone: exact server command batch missing");
    if (fresh.length > 0 && commandBatch !== null) {
      if (state.pendingMutations.length > 0) state.remoteFoldedOverLocal = true;
      const admitted = new Set(fresh.map(wireEnvelopeId)),
        batch = fresh.length === frame.Commands.envelopes.length ? commandBatch : encodeDocumentBackboneEnvelopeBatchExact(decodeDocumentBackboneEnvelopeBatchExact(commandBatch).filter((envelope) => admitted.has(envelope.mutation_id))),
        message = encodeBackboneMessage({ kind: "mutations", envelopes: batch }),
        reservation = state.browserActorReservation;
      if (reservation === null && documentAwaitsBrowserActor(state)) retainBackboneBeforeBrowserActor(state, message);
      else if (reservation === null) emitEvent(state, { kind: "documentBackbone", message });
      else {
        try {
          await reservation.receiveBackbone(message);
        } catch (error) {
          console.error("[backbone-worker] the browser actor child refused an inbound hub frame; reopening from the last confirmed state", state.config.documentId, error);
          requestDocumentActorRecoveryV1(state, "inbound-frame");
          return;
        }
      }
    }""", """    const frameIds = frame.Commands.envelopes.map(wireEnvelopeId);
    const settled = settleCommittedEnvelopes(state, frameIds);
    const fresh = admitRemoteEnvelopes(state.ingestedMutationIds, frame.Commands.envelopes, wireEnvelopeId);
    if (fresh.length > 0 && commandBatch === null) throw new Error("document backbone: exact server command batch missing");
    const segments = hubOrderSegmentsV1(frameIds, settled, new Set(fresh.map(wireEnvelopeId)));
    const decoded = segments.some((segment) => segment.kind === "sequenced" && segment.ids.length !== frameIds.length) ? decodeDocumentBackboneEnvelopeBatchExact(commandBatch!) : null;
    for (const segment of segments) {
      const run = new Set(segment.ids);
      const message =
        segment.kind === "committed"
          ? encodeBackboneMessage({ kind: "committed", opIds: segment.ids })
          : encodeBackboneMessage({ kind: "sequenced", envelopes: decoded === null ? commandBatch! : encodeDocumentBackboneEnvelopeBatchExact(decoded.filter((envelope) => run.has(envelope.mutation_id))) });
      if (!(await forwardDocumentBackboneV1(state, message))) return;
    }""", "worker forwards a frame in hub order")

edit(WORKER, """/** 🔁️ Rebuilds this document from the hub's authoritative state (a remote fold over pending local operations, a hub
 * `RebootstrapRequired`): drops""", """/** 🔁️ Rebuilds this document from the hub's authoritative state (the hub's correction of an actor-bound document's refused
 * batch, a hub `RebootstrapRequired`): drops""", "worker rebuild doc names its remaining causes")
edit(STORE_DIR + "🧬️schema/🔁️document-rebuild-welcome/🔣️.json", "(a rebootstrap after a remote fold over pending local operations, or a hub `RebootstrapRequired`)",
     "(a rebootstrap after the hub corrects an actor-bound document's refused batch, or a hub `RebootstrapRequired`)", "rebuild welcome schema names its remaining causes")
edit(WORKER, """/** 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage — mirrors the Rust actor's
 * `handle_ack`. `pendingMutations` (the UI-facing "unconfirmed" count) is trimmed by id, the same
 * way the old per-operation `ack` frame used to. */""", """/** 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage — mirrors the Rust actor's
 * `handle_ack`. `pendingMutations` (the UI-facing "unconfirmed" count) is trimmed by id, the same
 * way the old per-operation `ack` frame used to. The store learns the hub's decision (`committed`) before any correction it
 * takes in place; a browser actor's refused or transformed batch is corrected by the document's rebuild instead, which
 * discards the actor, so nothing reaches it. */""", "worker ack doc names the commit")
