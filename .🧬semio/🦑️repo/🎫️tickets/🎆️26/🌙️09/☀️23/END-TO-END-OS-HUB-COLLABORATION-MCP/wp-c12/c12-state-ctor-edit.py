"""🌱️ C12 session 14c: ONE ArtifactState constructor (`newArtifactState`) shared by `openArtifact` and every law's hand-built state
(the u64 Commands law and the parity `seedState`), so a new state field can never be missing from a test state again (C13 relay,
17:0x: the u64 law's literal lacked `outbox`/`pendingBatches`/`exactLocalEnvelopes` once `settleCommittedEnvelopes` landed).
Idempotent; `--dry-run` prints the plan only; every anchor must match exactly once or nothing is written."""
import sys

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/"
WORKER = ROOT + "🔨️modules/🏪️store/👷️worker/🟦️.ts"
LAW = ROOT + "🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"
PARITY = ROOT + "🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts"

FIELDS = """    pendingSocketActorId: null,
    channel,
    socket: null,
    presenceAuthority: null,
    docAbort: new AbortController(),
    executionTargetOpen: null,
    executionTargetLease: null,
    browserActorReservation: null,
    browserActorBackboneBeforeReservation: [],
    remoteFoldedOverLocal: false,
    link: { kind: "linked" },
    linkShortageTimer: null,
    transientRefusal: null,
    browserActorViewState: null,
    sanityPollTimer: null,
    watchHealthy: false,
    revalidateFolder: async () => {},
    reconnectDelayMs: HUB_RECONNECT_MIN_MS,
    outbox: [],
    pendingMutations: [],
    exactLocalEnvelopes: new WeakMap(),
    pendingDocumentBackboneBytes: 0,
    pendingDocumentBackboneMessages: 0,
    status: { persisted: false, pendingMutations: 0, remote: { kind: "detached" } },
    frontier: null,
    pendingResumeToken: null,
    requiredTailFrontier: null,
    artifactBootstrap: null,
    artifactBootstrapOwner: null,
    artifactBootstrapDeadlineMs: null,
    artifactBootstrapDeadlineTimer: null,
    artifactRebootstrapOwner: null,
    artifactRebootstrapDeadlineMs: null,
    artifactRebootstrapDeadlineTimer: null,
    artifactRebootstrapRequired: false,
    artifactBootstrapProgress: [],
    canonicalFolderMirror: null,
    verifiedColdPair: null,
    currentPack: null,
    currentSpr: null,
    hubFrameChain: Promise.resolve(),
    resumeToken: null,
    sessionColor: null,
    pendingBatches: new Map(),
    nextBatchId: 0,
    ingestedMutationIds: new Set(),
    hlcCounter: 0,
    closed: false,
  };
"""

OPEN_OLD = """  const channel = new BroadcastChannel(`semio-doc-${runtimeKey}`);
  const state: ArtifactState = {
    runtimeKey,
    config,
    openClientInstanceId: request.clientInstanceId ?? crypto.randomUUID(),
    actor: hub === null ? config.actor : "",
    hubActorReady: hub === null,
""" + FIELDS.replace("    revalidateFolder: async () => {},\n", "    revalidateFolder: async () => {}, // 🔧 replaced below once a folder binding exists.\n")

OPEN_NEW = """  const channel = new BroadcastChannel(`semio-doc-${runtimeKey}`);
  const state = newArtifactState(config, runtimeKey, channel, request.clientInstanceId ?? crypto.randomUUID());
"""

CTOR = """/** 🌱️ A just-opened document's runtime state: nothing linked, nothing pending, no pair, no lease. The ONE constructor —
 * {@link openArtifact} and every law's hand-built state go through it, so a new field is never missing from a test state.
 * `revalidateFolder` is a no-op until {@link openArtifact} binds a folder. */
function newArtifactState(config: ArtifactActorConfig, runtimeKey: string, channel: BroadcastChannel, openClientInstanceId: string): ArtifactState {
  const hub = hubBinding(config);
  return {
    runtimeKey,
    config,
    openClientInstanceId,
    actor: hub === null ? config.actor : "",
    hubActorReady: hub === null,
""" + FIELDS.replace("  };\n", "  };\n}\n") + "\n"

OPEN_HEAD = "function openArtifact(request: ArtifactActorConfig & { readonly clientInstanceId?: string }): void {\n"

TYPE_OLD = "  readonly documentRuntimeKeyForConfig: typeof documentRuntimeKeyForConfig;\n"
TYPE_NEW = TYPE_OLD + "  readonly newArtifactState: typeof newArtifactState;\n"

DEPS_OLD = "documentRuntimeKeyForConfig, documentRuntimeKeyV1, driveInferencePort,"
DEPS_NEW = "documentRuntimeKeyForConfig, documentRuntimeKeyV1, driveInferencePort, newArtifactState,"

LAW_OLD = """      const state = { config, actor: "local", openClientInstanceId: "client-1", artifactBootstrap: null, artifactRebootstrapRequired: false, frontier: null, requiredTailFrontier: null, executionTargetLease: null, pendingMutations: [], outbox: [], pendingBatches: new Map(), exactLocalEnvelopes: new WeakMap(), browserActorReservation: null, ingestedMutationIds: new Set<string>() } as unknown as ArtifactState;
"""
LAW_NEW = """      const state: ArtifactState = { ...newArtifactState(config, documentRuntimeKeyForConfig(config), { postMessage() {}, close() {} } as unknown as BroadcastChannel, "client-1"), actor: "local" };
"""
LAW_DESTRUCTURE_OLD = " documentRuntimeKeyForConfig,"
LAW_DESTRUCTURE_NEW = " documentRuntimeKeyForConfig, newArtifactState,"

PARITY_HEAD = """  const runtimeKey = deps.documentRuntimeKeyForConfig(config);
  const state = {
    runtimeKey,
    config,
    openClientInstanceId: "parity-client",
    actor: "",
    hubActorReady: false,
    pendingSocketActorId: null,
    channel: { postMessage() {}, close() {} } as unknown as BroadcastChannel,
"""
PARITY_TAIL = """    closed: false,
  } as unknown as ArtifactState;
  deps.artifacts.set(runtimeKey, state);
"""
PARITY_NEW = """  const runtimeKey = deps.documentRuntimeKeyForConfig(config);
  const state = deps.newArtifactState(config, runtimeKey, { postMessage() {}, close() {} } as unknown as BroadcastChannel, "parity-client");
  deps.artifacts.set(runtimeKey, state);
"""

dry = "--dry-run" in sys.argv
plan, problems, writes = [], [], {}


def once(text, old, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: anchor matches {count}×")
    return count == 1


worker = open(WORKER, encoding="utf-8").read()
if "function newArtifactState(" in worker:
    plan.append("worker: already applied")
else:
    ok = once(worker, OPEN_OLD, "worker openArtifact literal") & once(worker, OPEN_HEAD, "worker openArtifact head") & once(worker, TYPE_OLD, "worker deps type")
    if worker.count(DEPS_OLD) != 2:
        problems.append(f"worker deps literals: anchor matches {worker.count(DEPS_OLD)}× (want 2)")
        ok = False
    if ok:
        worker = worker.replace(OPEN_OLD, OPEN_NEW).replace(OPEN_HEAD, CTOR + OPEN_HEAD).replace(TYPE_OLD, TYPE_NEW).replace(DEPS_OLD, DEPS_NEW)
        writes[WORKER] = worker
        plan.append("worker: newArtifactState + openArtifact + deps type + 2 deps literals")

law = open(LAW, encoding="utf-8").read()
if LAW_NEW in law:
    plan.append("u64 law: already applied")
else:
    head = law.split("\n", 11)[10]
    ok = once(law, LAW_OLD, "u64 law literal") and head.count(LAW_DESTRUCTURE_OLD) == 1 and "newArtifactState" not in head
    if not ok:
        problems.append("u64 law destructure anchor")
    else:
        lines = law.split("\n")
        lines[10] = lines[10].replace(LAW_DESTRUCTURE_OLD, LAW_DESTRUCTURE_NEW)
        writes[LAW] = "\n".join(lines).replace(LAW_OLD, LAW_NEW)
        plan.append("u64 law: state from newArtifactState")

parity = open(PARITY, encoding="utf-8").read()
if PARITY_NEW in parity:
    plan.append("parity seedState: already applied")
elif once(parity, PARITY_HEAD, "parity head") and once(parity, PARITY_TAIL, "parity tail"):
    start = parity.index(PARITY_HEAD)
    end = parity.index(PARITY_TAIL) + len(PARITY_TAIL)
    writes[PARITY] = parity[:start] + PARITY_NEW + parity[end:]
    plan.append(f"parity seedState: literal ({parity[start:end].count(chr(10))} lines) → newArtifactState")

print("\n".join(plan))
print(f"{len(problems)} problems" + ("".join(f"\n  {p}" for p in problems)))
if problems:
    sys.exit(1)
if not dry:
    for path, text in writes.items():
        open(path, "w", encoding="utf-8").write(text)
    print(f"wrote {len(writes)} files")
