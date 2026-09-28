"""🚑️ C12 14c: a browser actor child that refuses an inbound hub frame is RECOVERED (S18's one recovery design:
`requestDocumentActorRecoveryV1(state, "inbound-frame")` — in-flight batches back to the outbox front, child retired `actor-lost`,
socket closed `actorLost`, reopen at once from the hub's checkpoint pair + tail) instead of rethrown as a "malformed hub frame" that
rejected the bootstrap, closed the socket and rebuilt the document from nothing (probe `c12short-8010`: 15 s / 60 s cuts → user2's
child faulted on the resumed frame → "fresh authoritative restore" → editors converged EMPTY). The frontier does not advance past a
frame the child never applied. Law beside S18's recovery law. Idempotent; `--dry-run`."""
import sys

WORKER = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
LAW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"
HUNKS = [
    (WORKER,
     "      else {\n        try { await reservation.receiveBackbone(message); }\n        catch (error) { reservation.close(); throw error; }\n      }\n",
     "      else {\n        try {\n          await reservation.receiveBackbone(message);\n        } catch (error) {\n"
     "          console.error(\"[backbone-worker] the browser actor child refused an inbound hub frame; reopening from the last confirmed state\", state.config.documentId, error);\n"
     "          requestDocumentActorRecoveryV1(state, \"inbound-frame\");\n          return;\n        }\n      }\n"),
    (LAW,
     '        expect(posted.map((message) => `${message.kind}:${"code" in message ? message.code : ""}`)).toEqual(["artifact-bootstrap-failed:recovery-exhausted"]);\n'
     '        expect(state.outbox.map((entry) => entry.id)).toEqual(["a", "b", "c", "d", "e"]);\n'
     '      } finally {\n        testSeams.workerPostTestSink = priorSink;\n      }\n    });\n',
     '        expect(posted.map((message) => `${message.kind}:${"code" in message ? message.code : ""}`)).toEqual(["artifact-bootstrap-failed:recovery-exhausted"]);\n'
     '        expect(state.outbox.map((entry) => entry.id)).toEqual(["a", "b", "c", "d", "e"]);\n'
     '      } finally {\n        testSeams.workerPostTestSink = priorSink;\n      }\n    });\n\n'
     '    it("recovers a child that refuses an inbound hub frame: actor-lost reopen, frontier kept, never a malformed-frame rebuild", async () => {\n'
     '      const batch = new Uint8Array(Buffer.from("01016d01640161000000017301aa016902bbcc03ffffffffffffffffff0105", "hex"));\n'
     '      const serverFrame = Uint8Array.from([0, 3, ...batch, 6, ...new TextEncoder().encode("remote"), 1, 100, 0, 1, 101, 0, ...new Array(32).fill(0)]);\n'
     '      const decoded = decodeServerFrame(serverFrame).frame;\n'
     '      const exactBatch = extractServerCommandsDocumentBackboneBatchExact(serverFrame);\n'
     '      if (typeof decoded === "string" || !("Commands" in decoded) || exactBatch === null) throw new Error("expected exact server Commands frame");\n'
     '      const config: ArtifactActorConfig = { documentId: "d", schema: "demo/v1", bindings: [{ kind: "hub", dataClass: "persistedShared", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "local" };\n'
     '      const state: ArtifactState = { ...newArtifactState(config, documentRuntimeKeyForConfig(config), { postMessage() {}, close() {} } as unknown as BroadcastChannel, "client-1"), actor: "local" };\n'
     '      const confirmed = { document_id: "d", head_edit_ordinal: 4, head_edit_id: "edit-4", last_commit_seq: 4, chain_hash: new Array(32).fill(4) };\n'
     '      state.frontier = confirmed;\n'
     '      state.pendingBatches.set(0, [{ id: "in-flight" } as unknown as MutationEnvelope]);\n'
     '      let refused = 0;\n'
     '      state.browserActorReservation = { close() {}, receiveBackbone: async () => { refused += 1; throw new Error("browser actor child: invocation rejected: invoke reactor/poll: guest fault"); } } as unknown as ArtifactState["browserActorReservation"];\n'
     '      const priorSink = testSeams.workerPostTestSink;\n'
     '      const posted: BackboneWorkerResponse[] = [];\n'
     '      testSeams.workerPostTestSink = (message) => posted.push(message);\n'
     '      try {\n'
     '        await handleHubFrame(state, decoded, null, null, exactBatch);\n'
     '      } finally {\n'
     '        testSeams.workerPostTestSink = priorSink;\n'
     '      }\n'
     '      expect(refused, "the child saw the frame once").toBe(1);\n'
     '      expect(state.actorRecoveryRequested, "the lost actor reopens").toBe(true);\n'
     '      expect(posted.map((message) => `${message.kind}:${"message" in message ? message.message : ""}`)).toEqual(["artifact-rebootstrap-required:actor-lost"]);\n'
     '      expect(state.frontier, "a frame the child never applied does not advance the frontier").toEqual(confirmed);\n'
     '      expect(state.outbox.map((entry) => entry.id), "the in-flight batch waits at the outbox front").toEqual(["in-flight"]);\n'
     '      expect(state.artifactRebootstrapRequired, "no authoritative rebuild").toBe(false);\n'
     '    });\n'),
]
dry = "--dry-run" in sys.argv
texts, problems = {}, []
for path, old, new in HUNKS:
    text = texts.get(path) or open(path, encoding="utf-8").read()
    if new in text:
        print("present", path.rsplit("/", 2)[-2])
        texts[path] = text
        continue
    if text.count(old) != 1:
        problems.append((path.rsplit("/", 2)[-2], text.count(old)))
        continue
    texts[path] = text.replace(old, new)
    print("planned", path.rsplit("/", 2)[-2])
print(f"{len(problems)} problems {problems}")
if problems:
    sys.exit(1)
if not dry:
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("written")
