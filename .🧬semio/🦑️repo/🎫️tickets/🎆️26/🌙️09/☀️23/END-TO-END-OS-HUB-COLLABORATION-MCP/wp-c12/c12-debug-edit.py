"""🐞️ C12 session 14b: inserts / removes temporary `[DEBUG] c12` worker logs for the link-shortage reopen diagnosis.
usage: python3 c12-debug-edit.py apply|remove"""
import sys
path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
text = open(path, encoding="utf-8").read()
edits = [
    ("        if (!suspendDocumentBrowserActorLink(state, binding)) dropDocumentExecutionTargetLease(state);\n",
     "        console.warn(\"[DEBUG] c12 close\", JSON.stringify({ reservation: state.browserActorReservation === null ? null : { suspended: state.browserActorReservation.suspended }, lease: state.executionTargetLease?.live ?? null, pair: state.verifiedColdPair !== null, pack: state.currentPack !== null, outbox: state.outbox.length, pending: state.pendingBatches.size }));\n"),
    ("  const suspended = suspendedDocumentBrowserActor(state);\n  if (suspended !== null && suspended.lease === lease",
     "  console.warn(\"[DEBUG] c12 activate\", JSON.stringify({ suspended: suspendedDocumentBrowserActor(state) !== null, sameLease: suspendedDocumentBrowserActor(state)?.lease === lease, sameSocket: state.socket === socket, ready: state.hubActorReady, actor: state.actor === lease.browserActorGrant()?.actorId, pair: state.verifiedColdPair !== null, pack: state.currentPack !== null, outbox: state.outbox.length }));\n"),
    ("  if (fields.browserActor.kind !== \"closed-browser-actor\" || state.currentPack !== null || state.verifiedColdPair !== null) return false;\n",
     "  console.warn(\"[DEBUG] c12 seed\", JSON.stringify({ kind: fields.browserActor.kind, pack: state.currentPack !== null, pair: state.verifiedColdPair !== null }));\n"),
    ("  if (!documentBackboneRelayReady(state) || state.outbox.length === 0 || state.pendingBatches.size > 0 || state.transientRefusal?.timer != null) return;\n  const limit",
     "  if (state.outbox.length > 0) console.warn(\"[DEBUG] c12 flush\", JSON.stringify({ outbox: state.outbox.length, ready: state.hubActorReady, open: state.socket?.readyState === WebSocket.OPEN, bootstrap: state.artifactBootstrap !== null, rebootstrap: state.artifactRebootstrapRequired, tail: state.requiredTailFrontier !== null, pack: state.currentPack !== null, frontier: state.frontier !== null, pair: state.verifiedColdPair !== null, pending: state.pendingBatches.size, refusal: state.transientRefusal?.timer != null }));\n"),
    ("  } catch {\n    return null;\n  }\n  const grantControl: ExecutionTargetReadControl = { signal: state.docAbort.signal, deadlineAtMs: Math.min(plan.expiresAtUnixMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent: assertOwner };\n  assertExecutionTargetRead(grantControl);\n  const exchange",
     "  try { const f = suspended.lease.fields() as unknown as Record<string, unknown>, p = receiptFreeFields(plan, { component: suspended.lease.fields().component.byteLength, descriptor: suspended.lease.fields().descriptor.byteLength, browserActor: suspended.lease.fields().browserActor.kind === \"closed-browser-actor\" ? (suspended.lease.fields().browserActor as { byteLength: number }).byteLength : undefined }) as unknown as Record<string, unknown>; console.warn(\"[DEBUG] c12 resume-diff\", JSON.stringify(Object.keys(f).filter((k) => JSON.stringify(f[k]) !== JSON.stringify(p[k])).map((k) => ({ k, lease: JSON.stringify(f[k]).slice(0, 700), plan: JSON.stringify(p[k]).slice(0, 700) })))); } catch (e) { console.warn(\"[DEBUG] c12 resume-diff failed\", String(e)); }\n"),
]
suspend_anchor = "  suspendLink(): boolean {\n    if (this.closed || this.child === null"
suspend_debug = "    console.warn(\"[DEBUG] c12 suspendLink\", JSON.stringify({ closed: this.closed, child: this.child !== null, cold: this.coldApplied !== null, patch: this.renderedUiPatch, backbone: this.documentBackboneReady, suspended: this.linkSuspended }));\n"
mode = sys.argv[1]
if mode == "apply":
    for anchor, line in edits:
        assert text.count(anchor) == 1, anchor[:70]
        if line in text: continue
        text = text.replace(anchor, line + anchor)
    if suspend_debug not in text:
        assert text.count(suspend_anchor) == 1
        text = text.replace(suspend_anchor, "  suspendLink(): boolean {\n" + suspend_debug + "    if (this.closed || this.child === null")
else:
    text = "".join(line for line in text.splitlines(keepends=True) if "[DEBUG] c12 " not in line)
open(path, "w", encoding="utf-8").write(text)
print(mode, text.count("[DEBUG] c12 "))
