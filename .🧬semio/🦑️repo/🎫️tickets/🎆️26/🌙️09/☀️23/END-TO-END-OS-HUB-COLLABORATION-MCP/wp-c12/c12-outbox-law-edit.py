"""✂️ C12 session 14b: rewrites the transient-refusal / bounded-drain worker law for the ack-clocked outbox. One-off, idempotent."""
import sys
path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"
text = open(path, encoding="utf-8").read()
start = text.index('    it("keeps and resends a batch the hub refused for a transient reason, drains the outbox')
end_anchor = '    it("fences rebootstrap before mirror retirement and replays only the retained preexisting raw batch after exact catchup"'
end = text.index(end_anchor, start)
new = '''    it("keeps and resends a batch the hub refused for a transient reason, drains the outbox ack-clocked in bounded batches, never rebuilds", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      const originalBroadcastChannel = globalThis.BroadcastChannel;
      class BoundPortBroadcastChannel {
        onmessage: ((event: MessageEvent) => void) | null = null;
        postMessage(): void { throw new Error("bound document backbone must not echo before server authority"); }
        close(): void {}
      }
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = BoundPortBroadcastChannel;
      const actor = `hub.v1.${"5".repeat(64)}`;
      testSeams.documentSocketGrantTestIssue = async () => ({ schema: "semio.hub.document-socket-grant/v1", protocol: "semio.session.v1", actorId: actor, expiresAtMs: Number.MAX_SAFE_INTEGER });
      const documentId = "doc-ack-transient";
      const edit = (id: string, padding = 0) => encodeBackboneMessage({
        kind: "mutations",
        envelopes: encodeDocumentBackboneEnvelopeBatchExact([{ mutation_id: id, document_id: documentId, actor: "caller", dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: encodePackValue(id + "x".repeat(padding)) }, inverse: { schema: "demo/v1", payload: encodePackValue(null) }, timestamp: { actor: 1n, physical_ms: 2n, logical: 3n } }]),
      });
      const sentIds = (socket: FakeHubWebSocket) => socket.sent.flatMap((bytes) => {
        const decoded = decodeClientFrame(bytes).frame;
        return typeof decoded === "object" && decoded !== null && "Commands" in decoded ? [decoded.Commands.envelopes.map((envelope: { mutation_id: string }) => envelope.mutation_id)] : [];
      });
      const transient = Array.from(new TextEncoder().encode(JSON.stringify([{ level: "warning", code: "hub.unavailable", message: "DB I/O aggregate admission exhausted" }])));
      const refuse = (state: ArtifactState, batchId: number) => handleAck(state, batchId, [{ Applied: { outcome: { Rejected: { reason: "unavailable: DB I/O aggregate admission exhausted", messages: transient } } } }] as unknown as Parameters<typeof handleAck>[2]);
      const outcomes: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => outcomes.push(message);
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);
      try {
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller" });
        await flushSocketGrantTurns();
        const socket = FakeHubWebSocket.instances[0]!;
        socket.open();
        const state = artifactState(documentId, "space-1")!;
        installVerifiedDocumentBackbonePair(state);
        await handleHubFrame(state, { Session: { actor, color: 1 } });
        vi.useFakeTimers();
        const send = (id: string, padding = 0) => handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: edit(id, padding) } });
        for (const id of ["first-edit", "second-edit"]) send(id);
        expect(sentIds(socket), "one batch in flight: the next waits for its Ack").toEqual([["first-edit"]]);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual(["second-edit"]);
        await refuse(state, 0);
        expect(state.artifactRebootstrapRequired).toBe(false);
        expect(outcomes.some((message) => message.kind === "artifact-rebootstrap-required")).toBe(false);
        expect(outcomes.some((message) => message.kind === "event" && message.event.kind === "commandOutcome")).toBe(false);
        expect(state.outbox.map((envelope) => envelope.id), "the refused batch returns to the front").toEqual(["first-edit", "second-edit"]);
        expect(state.pendingMutations.map((envelope) => envelope.id)).toEqual(["first-edit", "second-edit"]);
        expect(state.transientRefusal?.batchLimit).toBe(1);
        send("third-edit");
        expect(state.outbox.map((envelope) => envelope.id), "a later edit waits behind the refused batch").toEqual(["first-edit", "second-edit", "third-edit"]);
        await vi.advanceTimersByTimeAsync(180);
        expect(sentIds(socket), "nothing goes out during the backoff (250 ms, jittered to 187.5)").toHaveLength(1);
        await vi.advanceTimersByTimeAsync(10);
        expect(sentIds(socket).slice(1), "the refused edit goes out again first, bounded to half the refused batch").toEqual([["first-edit"]]);
        await refuse(state, 1);
        await vi.advanceTimersByTimeAsync(370);
        expect(sentIds(socket), "a second refusal doubles the backoff (500 ms, jittered to 375)").toHaveLength(2);
        await vi.advanceTimersByTimeAsync(10);
        expect(sentIds(socket).slice(2)).toEqual([["first-edit"]]);
        await handleAck(state, 2, [{ Applied: { outcome: "Accepted" } }]);
        expect(state.transientRefusal, "an Accepted batch ends the refusal: the hub admits again").toBeNull();
        expect(sentIds(socket).slice(3), "the rest drains behind it, in order").toEqual([["second-edit", "third-edit"]]);
        await handleAck(state, 3, [{ Applied: { outcome: "Accepted" } }]);
        expect(state.outbox).toEqual([]);
        expect(state.pendingMutations).toEqual([]);
        state.hubActorReady = false;
        const queued = Array.from({ length: 20 }, (_, index) => `queued-${index}`);
        for (const id of queued) send(id);
        expect(state.outbox.map((envelope) => envelope.id)).toEqual(queued);
        state.hubActorReady = true;
        send("after-shortage");
        expect(sentIds(socket).slice(4), "a grown outbox drains one bounded batch at a time, in order").toEqual([queued.slice(0, 16)]);
        await handleAck(state, 4, [{ Applied: { outcome: "Accepted" } }]);
        expect(sentIds(socket).slice(5)).toEqual([[...queued.slice(16), "after-shortage"]]);
        await handleAck(state, 5, [{ Applied: { outcome: "Accepted" } }]);
        state.hubActorReady = false;
        for (const id of ["large-0", "large-1", "large-2"]) send(id, 100_000);
        state.hubActorReady = true;
        send("small-after-large");
        expect(sentIds(socket).slice(6), "a batch stays within half the declared batch bytes").toEqual([["large-0"]]);
        await handleAck(state, 6, [{ Applied: { outcome: "Accepted" } }]);
        expect(sentIds(socket).slice(7)).toEqual([["large-1"]]);
        await handleAck(state, 7, [{ Applied: { outcome: "Accepted" } }]);
        expect(sentIds(socket).slice(8)).toEqual([["large-2", "small-after-large"]]);
        send("behind-the-drop");
        expect(state.outbox.map((envelope) => envelope.id)).toEqual(["behind-the-drop"]);
        socket.close();
        expect(state.pendingBatches.size).toBe(0);
        expect(state.outbox.map((envelope) => envelope.id), "a dropped socket's unacked batch returns AHEAD of what queued behind it").toEqual(["large-2", "small-after-large", "behind-the-drop"]);
      } finally {
        closeArtifact(documentId, "space-1");
        vi.useRealTimers();
        randomSpy.mockRestore();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

'''
if text[start:end] == new:
    print("already applied"); sys.exit(0)
text = text[:start] + new + text[end:]
open(path, "w", encoding="utf-8").write(text)
print("applied")
