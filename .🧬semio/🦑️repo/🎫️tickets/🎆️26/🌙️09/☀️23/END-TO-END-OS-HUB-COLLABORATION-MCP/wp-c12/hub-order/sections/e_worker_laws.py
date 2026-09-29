# 🧪️ Section E: the worker laws — a frame reaches the store in the hub's order, a decided batch is committed, nothing rebuilds.
WORKER_LAWS = "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"

edit(WORKER_LAWS, """  describe("remote operations folded over pending local ones", () => {
    it("rebuilds an actor-bound document from the hub once its own operations are accepted after another human's were folded over them", async () => {""",
     """  describe("remote operations folded over pending local ones", () => {
    it("hands another human's operations and this human's commit to the actor in the hub's order, and never rebuilds the document", async () => {""", "worker law hub-order title")
edit(WORKER_LAWS, """          expect(state.remoteFoldedOverLocal, documentId).toBe(interleaved);
          posted.length = 0;
          await handleHubFrame(state, { Ack: { batch_id: 7, stages: [{ Applied: { outcome: "Accepted" } }], frontier: frontier(interleaved ? 2 : 1, `${documentId}:local-1`) } } as unknown as Parameters<typeof handleHubFrame>[1], null, null, null);
          expect(state.pendingMutations, documentId).toEqual([]);
          expect(posted.filter((message) => message.kind === "event" && message.event.kind === "commandOutcome").map((message) => (message as { event: { outcome: unknown } }).event.outcome), documentId).toEqual([{ kind: "accepted" }]);
          expect(posted.some((message) => message.kind === "artifact-rebootstrap-required"), documentId).toBe(interleaved);
          expect(state.remoteFoldedOverLocal, documentId).toBe(false);""", """          const { decodeBackboneMessage } = await import("../../🟦️.ts");
          posted.length = 0;
          await handleHubFrame(state, { Ack: { batch_id: 7, stages: [{ Applied: { outcome: "Accepted" } }], frontier: frontier(interleaved ? 2 : 1, `${documentId}:local-1`) } } as unknown as Parameters<typeof handleHubFrame>[1], null, null, null);
          expect(state.pendingMutations, documentId).toEqual([]);
          expect(posted.filter((message) => message.kind === "event" && message.event.kind === "commandOutcome").map((message) => (message as { event: { outcome: unknown } }).event.outcome), documentId).toEqual([{ kind: "accepted" }]);
          expect(state.browserActorBackboneBeforeReservation.map((message) => decodeBackboneMessage(message).kind), documentId).toEqual(interleaved ? ["sequenced", "committed"] : ["committed"]);
          expect(decodeBackboneMessage(state.browserActorBackboneBeforeReservation.at(-1)!), documentId).toEqual({ kind: "committed", opIds: [`${documentId}:local-1`] });
          expect(posted.some((message) => message.kind === "artifact-rebootstrap-required"), documentId).toBe(false);""", "worker law commit follows the remote run, no rebuild")

edit(WORKER_LAWS, """      expect(response.event.message).toEqual(encodeBackboneMessage({ kind: "mutations", envelopes: batch }));
      expect(parseDocumentBackboneMessage(response.event.message).envelopes[0]?.timestamp.physical_ms).toBe(0xffff_ffff_ffff_ffffn);""", """      const { parseDocumentBackboneInboundMessage } = await import("../../🟦️.ts");
      expect(response.event.message).toEqual(encodeBackboneMessage({ kind: "sequenced", envelopes: batch }));
      expect(parseDocumentBackboneInboundMessage(response.event.message).envelopes[0]?.timestamp.physical_ms).toBe(0xffff_ffff_ffff_ffffn);""", "worker law a hub batch reaches the store sequenced")
edit(WORKER_LAWS, """            expect(state.browserActorBackboneBeforeReservation.map((message) => Array.from(message)), documentId).toEqual([Array.from(encodeBackboneMessage({ kind: "mutations", envelopes: batch }))]);""",
     """            expect(state.browserActorBackboneBeforeReservation.map((message) => Array.from(message)), documentId).toEqual([Array.from(encodeBackboneMessage({ kind: "sequenced", envelopes: batch }))]);""", "worker law a tail before the child is kept sequenced")
