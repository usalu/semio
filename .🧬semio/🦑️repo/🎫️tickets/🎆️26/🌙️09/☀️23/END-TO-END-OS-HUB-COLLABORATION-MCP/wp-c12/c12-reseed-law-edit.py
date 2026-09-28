"""✂️ C12 session 14b: worker law — a socket that closes under an unsuspendable mounted child retires it, forgets its document and
reopens like a first open. One-off, idempotent."""
import sys
path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"
text = open(path, encoding="utf-8").read()
title = "retires an unsuspendable mounted child when its socket closes, forgets its document and reopens like a first open"
if title in text:
    print("already applied"); sys.exit(0)
anchor = '    it("fences rebootstrap before mirror retirement and replays only the retained preexisting raw batch after exact catchup"'
assert text.count(anchor) == 1
law = '''    it("''' + title + '''", async () => {
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
      const actor = `hub.v1.${"6".repeat(64)}`;
      testSeams.documentSocketGrantTestIssue = async () => ({ schema: "semio.hub.document-socket-grant/v1", protocol: "semio.session.v1", actorId: actor, expiresAtMs: Number.MAX_SAFE_INTEGER });
      const edit = (documentId: string, id: string) => encodeBackboneMessage({
        kind: "mutations",
        envelopes: encodeDocumentBackboneEnvelopeBatchExact([{ mutation_id: id, document_id: documentId, actor: "caller", dependencies: [], observed: null, target: [], diff: { schema: "demo/v1", payload: encodePackValue(id) }, inverse: { schema: "demo/v1", payload: encodePackValue(null) }, timestamp: { actor: 1n, physical_ms: 2n, logical: 3n } }]),
      });
      const hello = (socket: FakeHubWebSocket) => {
        const decoded = decodeClientFrame(socket.sent[0]!).frame;
        if (typeof decoded !== "object" || decoded === null || !("SocketHelloV1" in decoded)) throw new Error("expected SocketHelloV1");
        return decoded.SocketHelloV1;
      };
      const posted: BackboneWorkerResponse[] = [];
      testSeams.workerPostTestSink = (message) => posted.push(message);
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);
      vi.useFakeTimers();
      try {
        const documentId = "doc-retire-mounted";
        openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller" });
        await flushSocketGrantTurns();
        const socket = FakeHubWebSocket.instances.at(-1)!;
        socket.open();
        const state = artifactState(documentId, "space-1")!;
        const baseline = installVerifiedDocumentBackbonePair(state, { head_edit_ordinal: 7, head_edit_id: "edit-7", last_commit_seq: 7 });
        state.resumeToken = "resume-before-the-cut";
        await handleHubFrame(state, { Session: { actor, color: 1 } });
        let childClosed = false;
        state.browserActorReservation = { suspended: false, suspendLink: () => false, close: () => { childClosed = true; state.browserActorReservation = null; } } as unknown as ArtifactState["browserActorReservation"];
        for (const id of ["edit-in-flight", "edit-queued"]) handleTsRequest({ kind: "send", documentId, clientInstanceId: state.openClientInstanceId, message: { kind: "documentBackbone", message: edit(documentId, id) } });
        expect(state.pendingBatches.size).toBe(1);
        expect(state.frontier).toEqual(baseline);
        socket.close();
        expect(childClosed, "the unsuspendable child is retired").toBe(true);
        expect({ pair: state.verifiedColdPair, lease: state.executionTargetLease, pack: state.currentPack, spr: state.currentSpr, frontier: state.frontier, resumeToken: state.resumeToken }, "the document the child held is forgotten").toEqual({ pair: null, lease: null, pack: null, spr: null, frontier: null, resumeToken: null });
        expect(state.outbox.map((envelope) => envelope.id), "nothing unsent is lost, in send order").toEqual(["edit-in-flight", "edit-queued"]);
        expect([...state.ingestedMutationIds].sort(), "their echo stays suppressed").toEqual(["edit-in-flight", "edit-queued"]);
        expect(posted.filter((message) => message.kind === "artifact-rebootstrap-required"), "the Shell discards the stale surface").toEqual([expect.objectContaining({ documentId, retryable: true })]);
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS + 1);
        const next = FakeHubWebSocket.instances.at(-1)!;
        expect(next).not.toBe(socket);
        next.open();
        expect(hello(next), "the next connection opens like a first one").toMatchObject({ frontier: null, resume_token: null });
        closeArtifact(documentId, "space-1");

        const plainId = "doc-retire-unmounted";
        openArtifact({ documentId: plainId, schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller" });
        await flushSocketGrantTurns();
        const plainSocket = FakeHubWebSocket.instances.at(-1)!;
        plainSocket.open();
        const plain = artifactState(plainId, "space-1")!;
        const plainFrontier = { document_id: plainId, head_edit_ordinal: 3, head_edit_id: "edit-3", last_commit_seq: 3, chain_hash: new Array(32).fill(3) };
        plain.frontier = plainFrontier;
        plain.resumeToken = "resume-plain";
        const postedBefore = posted.length;
        plainSocket.close();
        expect({ frontier: plain.frontier, resumeToken: plain.resumeToken }, "a document without a mounted child resumes from where it was").toEqual({ frontier: plainFrontier, resumeToken: "resume-plain" });
        expect(posted.slice(postedBefore).some((message) => message.kind === "artifact-rebootstrap-required")).toBe(false);
        closeArtifact(plainId, "space-1");
      } finally {
        closeArtifact("doc-retire-mounted", "space-1");
        closeArtifact("doc-retire-unmounted", "space-1");
        vi.useRealTimers();
        randomSpy.mockRestore();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        (globalThis as unknown as { BroadcastChannel: unknown }).BroadcastChannel = originalBroadcastChannel;
      }
    });

'''
text = text.replace(anchor, law + anchor)
open(path, "w", encoding="utf-8").write(text)
print("applied")
