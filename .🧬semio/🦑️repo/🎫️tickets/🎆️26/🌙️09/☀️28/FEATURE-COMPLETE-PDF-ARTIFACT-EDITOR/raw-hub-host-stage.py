"""🛰️ Stages the authenticated grouped Hub path in the normal Store worker socket and outbox."""

import hashlib
import json
import os
from pathlib import Path
import runpy

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
LOWER = runpy.run_path(str(TICKET / "raw-spr-transport-stage.py"))
WORKER = LOWER["WORKER"]
HOST = WORKER / "📦️atomic-hub-group/🟦️.ts"
WIRE = HOST.with_name("📡️wire.ts")
OUT = TICKET / "🗑️generated/raw-hub-host-overlay.json"


def stage():
    LOWER["stage"]()
    data = json.loads(LOWER["OUTPUT"].read_text(encoding="utf8"))
    pre, post = data["preimages"], data["candidate"]

    def write(path, value):
        full = ROOT / path
        pre[str(path)] = hashlib.sha256(full.read_bytes()).hexdigest() if full.exists() else None
        post[str(path)] = value

    def edit(path, anchor, replacement):
        current = post.get(str(path)) or (ROOT / path).read_text(encoding="utf8")
        assert current.count(anchor) == 1, (path, anchor, current.count(anchor))
        write(path, current.replace(anchor, replacement))

    links = [(marker, os.path.relpath(LOWER[source], HOST.parent)) for marker, source in
             (("ATOMIC_MODULE", "ASSEMBLY"), ("CAUSAL_MODULE", "BATCH"), ("BACKBONE_MODULE", "MESSAGE"))]
    for path, filename in ((HOST, "raw-hub-atomic-host.ts"), (WIRE, "raw-hub-atomic-wire.ts")):
        code = (TICKET / filename).read_text(encoding="utf8")
        for marker, link in links:
            code = code.replace(marker, link)
        write(path, code)

    def import_host(path, names):
        current = post.get(str(path)) or (ROOT / path).read_text(encoding="utf8")
        initial = current.splitlines()[0]
        edit(path, initial, f'import {{ {names} }} from "../📦️atomic-hub-group/🟦️.ts";\n' + initial)

    lifecycle = LOWER["LIFECYCLE"]
    opened = LOWER["OPEN"]
    teardown = LOWER["TEARDOWN"]
    connection = WORKER / "🔌️hub-connection/🟦️.ts"
    ack = WORKER / "📡️hub-frames/🟦️.ts"
    outbox = WORKER / "📮️hub-outbox/🟦️.ts"
    inbound = LOWER["INBOUND"]
    import_host(lifecycle, "relayAtomicHubPart")
    import_host(teardown, "retireAtomicHubDocument")
    import_host(connection, "deliverAtomicHubPacket, retireAtomicHubSocket")
    import_host(ack, "acknowledgeAtomicHubGroup")
    import_host(outbox, "flushPendingAtomicHubGroup")
    edit(lifecycle, '        if (hubBinding(state.config) !== null) throw new Error("atomic backbone: Hub binding does not admit grouped history");',
         """        if (hubBinding(state.config) !== null) {
          await relayAtomicHubPart(state, parsed);
          break;
        }""")
    edit(teardown, "  retireLocalAtomicDocumentFrames(state);", "  retireLocalAtomicDocumentFrames(state);\n  retireAtomicHubDocument(state);")
    edit(connection, "          const commandBatch = extractServerCommandsDocumentBackboneBatchExact(bytes);",
         """          if (await deliverAtomicHubPacket(state, socket, bytes)) return;
          const commandBatch = extractServerCommandsDocumentBackboneBatchExact(bytes);""")
    edit(connection, "    socket.onclose = () => {", "    socket.onclose = () => {\n      retireAtomicHubSocket(state, socket);")
    edit(ack, "    await handleAck(state, frame.Ack.batch_id, frame.Ack.stages);",
         """    if (acknowledgeAtomicHubGroup(state, frame.Ack.batch_id, frame.Ack.stages)) {
      flushMutationsToHubIfReady(state);
      return;
    }
    await handleAck(state, frame.Ack.batch_id, frame.Ack.stages);""")
    edit(outbox, "  if (!documentBackboneRelayReady(state) || state.outbox.length === 0 || state.pendingBatches.size > 0 || state.transientRefusal?.timer != null) return;",
         """  if (!documentBackboneRelayReady(state) || state.pendingBatches.size > 0 || state.transientRefusal?.timer != null) return;
  if (flushPendingAtomicHubGroup(state) || state.outbox.length === 0) return;""")
    edit(inbound, "type DocumentBrowserActorChild, browserActorColdStatus,",
         "type DocumentBrowserActorChild, browserActorColdStatus, browserActorRecord, browserActorTurnResult, DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT,")
    exchange = """            let result: BrowserActorChildValue | null = null;
            try {
              result = await this.hub.lanes.invokePoll(
                child,
                [{ tag: "message", val: { source: { tag: "shell", val: 0 }, payload: encodeDocumentBackboneControlV1(command) } }],
                null,
                () => {
                  if (this.session.child !== child || this.session.lifetime === null) throw new Error("actor-document-control.stale-child");
                  if (command.operation !== "retire") this.assertDocumentOwnerCurrent();
                },
                command.operation === "retire",
              );
              if (browserActorColdStatus(result).kind !== "idle" || this.hub.patches.captureUiPatch(result, this.session.lifetime!) !== null) throw new Error("actor-document-control.unexpected-turn-output");
              const routed = await this.hub.effects.routeTurnEffects(result, "control");
              if (routed.mutations !== 0) throw new Error("actor-document-control.data-before-receipt");
              return routed.receipts;
            } finally {
              if (result !== null) wipeBrowserActorValue(result);
            }"""
    edit(inbound, exchange, """            const events: BrowserActorChildValue[] = [{ tag: "message", val: { source: { tag: "shell", val: 0 }, payload: encodeDocumentBackboneControlV1(command) } }];
            for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
              const result = await this.hub.lanes.invokePoll(child, events, null, () => {
                if (this.session.child !== child || this.session.lifetime === null) throw new Error("actor-document-control.stale-child");
                if (command.operation !== "retire") this.assertDocumentOwnerCurrent();
              }, command.operation === "retire");
              try {
                if (browserActorColdStatus(result).kind !== "idle" || this.hub.patches.captureUiPatch(result, this.session.lifetime!) !== null) throw new Error("actor-document-control.unexpected-turn-output");
                const routed = await this.hub.effects.routeTurnEffects(result, "control");
                if (routed.mutations !== 0) throw new Error("actor-document-control.data-before-receipt");
                const more = browserActorRecord(browserActorTurnResult(result).status, "actor-document-control.invalid-status").tag === "more-work";
                if (routed.receipts.length !== 0 || !more || command.operation !== "retire") return routed.receipts;
              } finally {
                wipeBrowserActorValue(result);
              }
            }
            throw new Error("actor-document-control.retirement-turn-limit");""")

    OUT.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} Store worker authenticated atomic Hub postimages")


if __name__ == "__main__":
    stage()
