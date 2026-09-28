"""🔎️ C12 14c: TEMPORARY `[DEBUG] c12` probes in the worker — why a reloaded document's edits never leave the tab (collab STEP 6/8:
"Check In (1)" pending, 0 Commands at the peer). Logs the relay gate on every locally authored batch and on Welcome/Session.
usage: python3 c12-debug-gate-edit.py [--revert]   (idempotent both ways)"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
HELPER = """function c12DebugGate(state: ArtifactState, where: string): void {
  let pairCurrent = false;
  try { state.verifiedColdPair?.assertCurrent(); pairCurrent = state.verifiedColdPair !== null; } catch { pairCurrent = false; }
  console.warn(`[DEBUG] c12 gate ${where} ${state.config.documentId} ${JSON.stringify({ actorReady: state.hubActorReady, socket: state.socket?.readyState ?? null, bootstrap: state.artifactBootstrap !== null, rebootstrap: state.artifactRebootstrapRequired, requiredTail: state.requiredTailFrontier !== null, pack: state.currentPack !== null, spr: state.currentSpr !== null, frontier: state.frontier !== null, pair: state.verifiedColdPair !== null, pairCurrent, lease: state.executionTargetLease?.live ?? null, reservation: state.browserActorReservation !== null, outbox: state.outbox.length, inFlight: state.pendingBatches.size, transient: state.transientRefusal?.timer != null, closed: state.closed })}`);
}

"""
HUNKS = [
    ("function relayMutationsToHub(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {\n",
     HELPER + "function relayMutationsToHub(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {\n"),
    ("  queueOutbox(state, envelopes);\n  flushMutationsToHubIfReady(state);\n}\n",
     "  queueOutbox(state, envelopes);\n  flushMutationsToHubIfReady(state);\n  c12DebugGate(state, \"relay\");\n}\n"),
    ("  if (\"Welcome\" in frame) {\n    requeuePendingBatches(state);\n",
     "  if (\"Welcome\" in frame) {\n    c12DebugGate(state, `welcome-${typeof frame.Welcome.bootstrap === \"string\" ? frame.Welcome.bootstrap : \"object\"}`);\n    requeuePendingBatches(state);\n"),
    ("    state.presenceAuthority = presenceCandidate?.socket === state.socket ? presenceCandidate : null;\n    flushMutationsToHubIfReady(state);\n",
     "    state.presenceAuthority = presenceCandidate?.socket === state.socket ? presenceCandidate : null;\n    flushMutationsToHubIfReady(state);\n    c12DebugGate(state, \"session\");\n"),
]

text = open(PATH, encoding="utf-8").read()
revert = "--revert" in sys.argv
for old, new in HUNKS:
    source, target = (new, old) if revert else (old, new)
    if target in text and source not in text:
        continue
    assert text.count(source) == 1, (revert, source[:60], text.count(source))
    text = text.replace(source, target)
open(PATH, "w", encoding="utf-8").write(text)
print("reverted" if revert else "applied", text.count("[DEBUG] c12"))
