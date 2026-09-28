# -*- coding: utf-8 -*-
"""S18 §14c: worker-level law of the document actor recovery (coordinator's laws 1–3 at the worker): in-flight batches return to the
outbox front in batch order, never twice; the mounted child retires as `actor-lost` and the recovery close reconnects at once; past
the policy's bound the typed `recovery-exhausted` fault is posted and no reconnect is asked for. Wires
`requestDocumentActorRecoveryV1` into the worker's test dependencies. Idempotent."""
import pathlib

OS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os")
WORKER = OS / "🔨️modules/🏪️store/👷️worker/🟦️.ts"
LAW = OS / "🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"

LAW_BLOCK = '''  describe("🚑️ document actor recovery (worker)", () => {
    it("returns in-flight batches to the outbox front in batch order, never twice, retires the child as actor-lost and reconnects at once; past the bound it posts the typed exhausted fault", () => {
      const requestRecovery = dependencies.requestDocumentActorRecoveryV1;
      const config: ArtifactActorConfig = { documentId: "d", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "local" };
      const state: ArtifactState = { ...newArtifactState(config, documentRuntimeKeyForConfig(config), { postMessage() {}, close() {} } as unknown as BroadcastChannel, "client-1"), actor: "local" };
      const envelope = (id: string) => ({ id }) as unknown as MutationEnvelope;
      const mount = () => {
        state.browserActorReservation = { close() {} } as unknown as ArtifactState["browserActorReservation"];
      };
      const priorSink = testSeams.workerPostTestSink;
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      try {
        state.pendingBatches.set(2, [envelope("c")]);
        state.pendingBatches.set(1, [envelope("a"), envelope("b")]);
        state.outbox = [envelope("d"), envelope("a"), envelope("e")];
        mount();
        requestRecovery(state, "action-unconfirmed");
        expect(state.outbox.map((entry) => entry.id)).toEqual(["a", "b", "c", "d", "e"]);
        expect(state.pendingBatches.size).toBe(0);
        expect(state.actorRecoveryRequested).toBe(true);
        expect(posted.map((message) => `${message.kind}:${"message" in message ? message.message : ""}`)).toEqual(["artifact-rebootstrap-required:actor-lost"]);
        for (let loss = 2; loss <= 3; loss += 1) {
          mount();
          state.actorRecoveryRequested = false;
          requestRecovery(state, "inbound-frame");
          expect(state.actorRecoveryRequested, `loss ${loss}`).toBe(true);
        }
        mount();
        state.actorRecoveryRequested = false;
        posted.length = 0;
        requestRecovery(state, "turn-failed");
        expect(state.actorRecovery.exhausted).toBe(true);
        expect(state.actorRecoveryRequested).toBe(false);
        expect(posted.map((message) => `${message.kind}:${"code" in message ? message.code : ""}`)).toEqual(["artifact-bootstrap-failed:recovery-exhausted"]);
        expect(state.outbox.map((entry) => entry.id)).toEqual(["a", "b", "c", "d", "e"]);
      } finally {
        testSeams.workerPostTestSink = priorSink;
      }
    });
  });

  describe("backbone-worker wire bridge", () => {'''

EDITS = [
    (WORKER, "  readonly newArtifactState: typeof newArtifactState;\n", "  readonly newArtifactState: typeof newArtifactState;\n  readonly requestDocumentActorRecoveryV1: typeof requestDocumentActorRecoveryV1;\n"),
    (LAW, '  describe("backbone-worker wire bridge", () => {', LAW_BLOCK),
]


def main() -> None:
    texts: dict[pathlib.Path, str] = {}
    for path, old, new in EDITS:
        text = texts.get(path) or path.read_text(encoding="utf-8")
        if new not in text:
            assert text.count(old) == 1, (path.name, old[:80])
            text = text.replace(old, new)
        texts[path] = text
    worker = texts[WORKER]
    marker = "newArtifactState, dropDocumentExecutionTargetLease"
    wired = "newArtifactState, requestDocumentActorRecoveryV1, dropDocumentExecutionTargetLease"
    if worker.count(wired) < 2:
        assert worker.count(marker) == 2, worker.count(marker)
        worker = worker.replace(marker, wired)
    texts[WORKER] = worker
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    print("ok")


main()
