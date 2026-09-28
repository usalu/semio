"""✂️ C12 session 14b: a dropped execution-target lease forgets the document its child held (pack, spr, frontier, resume token,
ingested ids), and a mounted child replaced while the document stays open tells the Shell (`artifact-rebootstrap-required`).
One-off, idempotent."""
import sys
path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
text = open(path, encoding="utf-8").read()
if "function forgetMountedDocument(" in text:
    print("already applied"); sys.exit(0)
def sub(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:90])
    text = text.replace(old, new)
sub('''function dropDocumentExecutionTargetLease(state: ArtifactState): void {
  dropVerifiedColdDocumentPair(state);
  const mirror = state.canonicalFolderMirror;
  state.canonicalFolderMirror = null;
  if (mirror) void retireFolderCanonicalBootstrapMirror(mirror);
  state.browserActorReservation?.close();
  state.executionTargetLease?.drop();
  state.executionTargetLease = null;
  state.browserActorBackboneBeforeReservation = [];
}
''', '''/** 🗑️ Drops the execution-target lease with its actor child, and with them the document that child held
 * ({@link forgetMountedDocument}). */
function dropDocumentExecutionTargetLease(state: ArtifactState): void {
  const mounted = state.verifiedColdPair !== null;
  dropVerifiedColdDocumentPair(state);
  const mirror = state.canonicalFolderMirror;
  state.canonicalFolderMirror = null;
  if (mirror) void retireFolderCanonicalBootstrapMirror(mirror);
  state.browserActorReservation?.close();
  state.executionTargetLease?.drop();
  state.executionTargetLease = null;
  state.browserActorBackboneBeforeReservation = [];
  if (mounted) forgetMountedDocument(state);
}

/** 🧹️ Forgets the document an actor child held once its cold pair is gone: the pack and spr published from that pair, the frontier
 * and resume token the child reached, and the ids it ingested — the operations still unacknowledged stay noted, since their echo
 * must never apply twice. The next connection then opens like a first one: its `SocketHelloV1` names no frontier, the hub answers
 * with the tail from its active checkpoint, and the next child is seeded from that checkpoint's canonical pair. A kept pack made
 * {@link seedColdPairFromCanonicalCheckpoint} skip the seed and left the next child without a document — "verifying" forever, every
 * edit refused `owner-mismatch` (ticket 26/09/23 C12, run `c12short5`: a 5 s link cut whose reconnect named a newer plan). */
function forgetMountedDocument(state: ArtifactState): void {
  state.currentPack = null;
  state.currentSpr = null;
  state.frontier = null;
  state.resumeToken = null;
  state.artifactBootstrapProgress = [];
  state.ingestedMutationIds.clear();
  const unacknowledged = [...[...state.pendingBatches.values()].flat(), ...state.outbox];
  noteAuthoredEnvelopeIds(state.ingestedMutationIds, unacknowledged.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id));
}

/** 🔁️ Retires the document's actor child while the document stays open — its socket closed and the child could not be suspended, or
 * a reconnect's plan named another execution target than the suspended child's — and, when a child was mounted, tells the Shell to
 * discard its stale surface exactly as a rebuild does (`artifact-rebootstrap-required`): the connection that follows seeds a fresh
 * child from the hub's active checkpoint and replays the unsent operations on top. */
function retireMountedDocumentChild(state: ArtifactState): void {
  const mounted = state.verifiedColdPair !== null || state.browserActorReservation !== null;
  dropDocumentExecutionTargetLease(state);
  if (!mounted || state.closed || state.docAbort.signal.aborted || state.artifactRebootstrapRequired) return;
  const scope = artifactScope(state);
  post({ kind: "artifact-rebootstrap-required", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), message: "rebootstrap-required", retryable: true });
}
''')
sub('''        if (!suspendDocumentBrowserActorLink(state, binding)) dropDocumentExecutionTargetLease(state);
''', '''        if (!suspendDocumentBrowserActorLink(state, binding)) retireMountedDocumentChild(state);
''')
sub('''    if (resumed !== null) return resumed;
    assertOwner.adopt();
  }
  dropDocumentExecutionTargetLease(state);
''', '''    if (resumed !== null) return resumed;
    assertOwner.adopt();
  }
  retireMountedDocumentChild(state);
''')
sub('''  abortArtifactBootstrap(state);
  dropVerifiedColdDocumentPair(state);
  state.currentPack = null;
  state.currentSpr = null;
  state.frontier = null;
  state.resumeToken = null;
  state.artifactBootstrapProgress = [];
  state.ingestedMutationIds.clear();
  setRemote(state, { kind: "connecting" });
''', '''  abortArtifactBootstrap(state);
  dropVerifiedColdDocumentPair(state);
  forgetMountedDocument(state);
  setRemote(state, { kind: "connecting" });
''')
open(path, "w", encoding="utf-8").write(text)
print("applied")
