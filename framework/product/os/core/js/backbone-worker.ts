// #region Header
/**
 * @emoji 🧵 `backbone-worker.ts` — thin loader for the Rust WASM `store_worker`
 * actor (`store/worker/rs`). When the wasm package is unavailable (vitest/node), falls
 * back to the embedded TypeScript actor twin so dev workflows keep working.
 */
// #endregion Header

import type { BackboneWorkerRequest, BackboneWorkerResponse, ClientFrame, CommandAckOutcome, DocumentActorConfig, DocumentActorMsg, DocumentEvent, DocumentPresencePeer, DocumentSyncStatus, OperationEnvelope, PersistenceBinding, RemoteState, ServerFrame, WireAckStage, WireFrontierSummary, WireLane, WireOperationEnvelope } from "./index";
import { decodeClientFrame, decodeServerFrame, encodeClientFrame, encodeServerFrame } from "./index";

type RustWorkerHost = {
  handleRequestJson(json: string): void;
  postReady(): void;
};

let rustHost: RustWorkerHost | null = null;

// 🧵 Built as a variable rather than a string literal so Rollup's static import analysis — including
// the separate sub-build `vite:worker-import-meta-url` runs for this very file — can't see a resolvable
// specifier at all and leaves the `import()` genuinely dynamic; a real bundler-visible specifier here
// (even `@vite-ignore`d) still gets probed by that sub-build and, since the package is never actually
// published, either fails the build outright or emits a phantom `__vite-browser-external-*.js` chunk
// reference that 404s in production. Left dynamic, the browser's native module loader simply rejects
// the unresolvable bare specifier at runtime, which the `catch` below already treats as "unavailable".
const RUST_SYNC_WORKER_MODULE_SPECIFIER = "@semio-tech/store-worker";

async function ensureRustHost(): Promise<RustWorkerHost | null> {
  if (rustHost) return rustHost;
  if (typeof WebAssembly === "undefined") return null;
  try {
    const module = await import(RUST_SYNC_WORKER_MODULE_SPECIFIER);
    await module.default();
    rustHost = new module.BackboneWorkerHost() as RustWorkerHost;
    return rustHost;
  } catch {
    return null;
  }
}

const rustHostPromise = ensureRustHost();

(self as unknown as DedicatedWorkerGlobalScope).onmessage = (messageEvent: MessageEvent<BackboneWorkerRequest>) => {
  void rustHostPromise.then((host) => {
    if (host) {
      host.handleRequestJson(JSON.stringify(messageEvent.data));
      return;
    }
    handleTsRequest(messageEvent.data);
  });
};

void rustHostPromise.then((host) => {
  if (host) host.postReady();
  else post({ kind: "ready" });
});

//#region 🔖TsFallback

//#region 🔖Constants
/** 🛰️ Must match `framework/product/os/core/js/index.ts`'s `BACKBONE_ENDPOINT_PATH`. */
const FOLDER_ENDPOINT_PATH = "/semio-backbone";
const FOLDER_POLL_INTERVAL_MS = 1_500;
const HUB_RECONNECT_MIN_MS = 500;
const HUB_RECONNECT_MAX_MS = 30_000;
//#endregion 🔖Constants

//#region 🔖DocumentState
type DocumentState = {
  config: DocumentActorConfig;
  channel: BroadcastChannel;
  socket: WebSocket | null;
  pollTimer: ReturnType<typeof setInterval> | null;
  reconnectTimer: ReturnType<typeof setTimeout> | null;
  reconnectDelayMs: number;
  pendingOperations: OperationEnvelope[];
  status: DocumentSyncStatus;
  /** 🏔️ Last frontier the hub reported (`Welcome.server_frontier` / `Commands.frontier` /
   * `Ack.frontier`) — the wire-v2 replacement for the old `sinceVersion: number` counter. */
  frontier: WireFrontierSummary | null;
  /** 🎟️ The hub's last `Welcome.resume_token`, echoed back on the next `hello` after a reconnect. */
  resumeToken: string | null;
  /** 🧺 Outbound `Commands` batches awaiting an `Ack`, keyed by `batch_id`. */
  pendingBatches: Map<number, OperationEnvelope[]>;
  nextBatchId: number;
  /** ⏰ Logical tick counter for {@link nextWireTimestamp} on every outbound wire envelope. */
  hlcCounter: number;
  closed: boolean;
};

const documents = new Map<string, DocumentState>();

function post(message: BackboneWorkerResponse): void {
  (self as unknown as DedicatedWorkerGlobalScope).postMessage(message);
}

function emitEvent(documentId: string, event: DocumentEvent): void {
  post({ kind: "event", documentId, event });
}

function setStatus(state: DocumentState, patch: Partial<DocumentSyncStatus>): void {
  state.status = { ...state.status, ...patch };
  emitEvent(state.config.documentId, { kind: "status", ...state.status });
}

function setRemote(state: DocumentState, remote: RemoteState): void {
  setStatus(state, { remote });
}

function folderBinding(config: DocumentActorConfig): Extract<PersistenceBinding, { kind: "folder" }> | null {
  const binding = config.bindings.find((entry): entry is Extract<PersistenceBinding, { kind: "folder" }> => entry.kind === "folder");
  return binding ?? null;
}

function hubBinding(config: DocumentActorConfig): Extract<PersistenceBinding, { kind: "hub" }> | null {
  const binding = config.bindings.find((entry): entry is Extract<PersistenceBinding, { kind: "hub" }> => entry.kind === "hub");
  return binding ?? null;
}
//#endregion 🔖DocumentState

//#region 🔖WireBridge
/** 🧮 A stable, deterministic 32-bit seed for an actor id string, for `WireOperationEnvelope.
 * timestamp.actor` — the TS twin of the Rust actor's `actor_seed` (`framework/sync/rs/lib.rs`
 * `🔖WireBridge`). Not cryptographic, just a cheap deterministic fold — matches the Rust side's own
 * `DefaultHasher`-based approach in spirit (both are wire-local ordering metadata, never round-
 * tripped back into an app-level {@link OperationEnvelope}). */
function actorSeed(actor: string): number {
  let hash = 0;
  for (let index = 0; index < actor.length; index++) {
    hash = (Math.imul(hash, 31) + actor.charCodeAt(index)) | 0;
  }
  return hash >>> 0;
}

/** ⏰ Advances `state.hlcCounter` and stamps a fresh wire timestamp for an outbound envelope —
 * the TS twin of the Rust actor's `next_timestamp`. */
function nextWireTimestamp(state: DocumentState): WireOperationEnvelope["timestamp"] {
  state.hlcCounter += 1;
  return { actor: actorSeed(state.config.actor), physical_ms: Date.now(), logical: state.hlcCounter };
}

/** #️⃣ A cheap, non-cryptographic FNV-1a-style digest for {@link toWireEnvelope}'s placeholder
 * `payloadHash` on the way back through {@link fromWireEnvelope}. This TS fallback never verifies
 * `payloadHash` against anything (it's a "deliberately dumb" relay twin — see this file's header
 * doc — the real content-addressed check happens Rust-side via `semio_framework_hash::hash_bytes`
 * once the wasm actor is available), so a real blake3 dependency isn't worth adding here just to
 * fill an otherwise-unused field. */
function placeholderPayloadHash(payload: unknown): string {
  const json = JSON.stringify(payload) ?? "null";
  let hash = 0x811c9dc5;
  for (let index = 0; index < json.length; index++) {
    hash ^= json.charCodeAt(index);
    hash = Math.imul(hash, 0x01000193);
  }
  return (hash >>> 0).toString(16).padStart(8, "0");
}

/** 🌉 Converts this fallback's local, camelCase {@link OperationEnvelope} into the snake_case
 * {@link WireOperationEnvelope} `protocol_wire::ClientFrame::Commands`/`ServerFrame::Commands`
 * carry — the TS twin of the Rust actor's `to_wire_envelope`. */
function toWireEnvelope(envelope: OperationEnvelope, timestamp: WireOperationEnvelope["timestamp"]): WireOperationEnvelope {
  return {
    operation_id: envelope.id,
    document_id: envelope.document,
    actor: envelope.actor,
    dependencies: [...(envelope.deps ?? [])],
    diff: { schema: envelope.diff.schemaId, payload: envelope.diff.payload },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, inverse_diff: envelope.inverse.inverseDiff.payload },
    timestamp,
  };
}

/** 🌉 The inverse of {@link toWireEnvelope} — the TS twin of the Rust actor's `from_wire_envelope`.
 * `baseVersion` is recovered from the payload's own `sequenceNumber` (this actor's payloads are
 * always edit-shaped JSON), mirroring the Rust side's identical recovery. */
function fromWireEnvelope(envelope: WireOperationEnvelope): OperationEnvelope {
  const payload = envelope.diff.payload;
  const sequenceNumber = payload !== null && typeof payload === "object" && "sequenceNumber" in payload ? Number((payload as Record<string, unknown>).sequenceNumber) : 0;
  return {
    id: envelope.operation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: placeholderPayloadHash(payload),
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: {
      targetOperation: envelope.operation_id,
      inverseDiff: { schemaId: envelope.inverse.schema, payload: envelope.inverse.inverse_diff },
      baseVersion: Number.isFinite(sequenceNumber) ? Math.max(0, sequenceNumber) : 0,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
  };
}

/** ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed `inverse` —
 * the TS twin of the Rust actor's `rollback_envelope` (see that function's doc comment for why
 * replaying the envelope's own inverse, rather than calling into typed operation-inverse machinery,
 * is the right move for this schema-agnostic relay). */
function rollbackEnvelope(envelope: OperationEnvelope): OperationEnvelope {
  const undoId = `${envelope.id}~undo`;
  return {
    id: undoId,
    actor: envelope.actor,
    document: envelope.document,
    schemaVersion: envelope.schemaVersion,
    deps: [envelope.id],
    payloadHash: placeholderPayloadHash(envelope.inverse.inverseDiff.payload),
    diff: { schemaId: envelope.inverse.inverseDiff.schemaId, payload: envelope.inverse.inverseDiff.payload },
    inverse: { targetOperation: undoId, inverseDiff: { schemaId: envelope.diff.schemaId, payload: envelope.diff.payload }, baseVersion: envelope.inverse.baseVersion, dependencies: [], undoPolicy: envelope.inverse.undoPolicy },
  };
}
//#endregion 🔖WireBridge

//#region 🔖Folder
function folderEnvelopeUrl(binding: Extract<PersistenceBinding, { kind: "folder" }>, documentId: string): string {
  return `${FOLDER_ENDPOINT_PATH}?uri=${encodeURIComponent(`folder://${binding.path}`)}&documentId=${encodeURIComponent(documentId)}`;
}

async function pollFolderOnce(state: DocumentState, binding: Extract<PersistenceBinding, { kind: "folder" }>): Promise<void> {
  try {
    const response = await fetch(folderEnvelopeUrl(binding, state.config.documentId));
    if (response.status === 404) return;
    if (!response.ok) throw new Error(`folder backbone read failed (${response.status})`);
    const envelopeJson = await response.text();
    emitEvent(state.config.documentId, { kind: "snapshotReplaced", envelopeJson });
    setStatus(state, { persisted: true });
  } catch (error) {
    console.error("[backbone-worker] folder poll failed", state.config.documentId, error);
  }
}

/** 👁️ Best-effort external-change watch: tries the dev middleware's SSE endpoint first (see header
 * doc), and only falls back to interval polling if that connection never opens — so once the
 * middleware side (`framework/product/os/dev/script.ts`) lands, this upgrades itself automatically. */
function watchFolder(state: DocumentState, binding: Extract<PersistenceBinding, { kind: "folder" }>): void {
  let sseOpened = false;
  try {
    const source = new EventSource(`${FOLDER_ENDPOINT_PATH}/watch?uri=${encodeURIComponent(`folder://${binding.path}`)}`);
    source.onopen = () => {
      sseOpened = true;
    };
    source.onmessage = () => {
      void pollFolderOnce(state, binding);
    };
    source.onerror = () => {
      if (!sseOpened) {
        source.close();
        startFolderPolling(state, binding);
      }
    };
  } catch {
    startFolderPolling(state, binding);
  }
  // 🛟 Always poll at a slow cadence too, even when SSE is live, as a self-healing fallback.
  startFolderPolling(state, binding);
}

function startFolderPolling(state: DocumentState, binding: Extract<PersistenceBinding, { kind: "folder" }>): void {
  if (state.pollTimer != null) return;
  state.pollTimer = setInterval(() => void pollFolderOnce(state, binding), FOLDER_POLL_INTERVAL_MS);
  void pollFolderOnce(state, binding);
}

async function writeFolder(state: DocumentState, binding: Extract<PersistenceBinding, { kind: "folder" }>, envelopeJson: string): Promise<void> {
  const response = await fetch(folderEnvelopeUrl(binding, state.config.documentId), {
    method: "PUT",
    headers: { "content-type": "application/json" },
    body: envelopeJson,
  });
  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);
  setStatus(state, { persisted: true });
}
//#endregion 🔖Folder

//#region 🔖Hub
function connectHub(state: DocumentState, binding: Extract<PersistenceBinding, { kind: "hub" }>): void {
  if (state.closed) return;
  setRemote(state, { kind: "connecting" });
  const wsBase = binding.baseUrl.replace(/^http/, "ws");
  const socket = new WebSocket(`${wsBase}/studios/${encodeURIComponent(binding.studioId)}/documents/${encodeURIComponent(state.config.documentId)}/ws`);
  // 🎞️ Binary frames (`protocol_wire`), not JSON text — see this file's header + `WireBridge` region.
  socket.binaryType = "arraybuffer";
  state.socket = socket;
  socket.onopen = () => {
    state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
    sendWireFrame(state, {
      Hello: {
        wire_version: 1,
        protocol_version: 1,
        schema: state.config.schema,
        // 🔖 No client-side schema pack hashing wired this wave (matches the Rust actor's identical
        // placeholder — see `framework/sync/rs/lib.rs` `try_connect_hub`); the hub is JSON-only until
        // CW6 anyway, so this is never validated yet.
        pack_schema_hash: new Array(32).fill(0),
        actor: state.config.actor,
        token: binding.token ?? null,
        resume_token: state.resumeToken,
        frontier: state.frontier,
      },
    }, "command");
  };
  socket.onmessage = (messageEvent) => {
    try {
      const bytes = new Uint8Array(messageEvent.data as ArrayBuffer);
      handleHubFrame(state, decodeServerFrame(bytes).frame);
    } catch (error) {
      console.error("[backbone-worker] malformed hub frame", state.config.documentId, error);
    }
  };
  socket.onclose = () => {
    if (state.socket === socket) state.socket = null;
    if (state.closed) return;
    setRemote(state, { kind: "backoff", retryInMs: state.reconnectDelayMs });
    state.reconnectTimer = setTimeout(() => connectHub(state, binding), state.reconnectDelayMs);
    state.reconnectDelayMs = Math.min(state.reconnectDelayMs * 2, HUB_RECONNECT_MAX_MS);
  };
  socket.onerror = () => socket.close();
}

function sendWireFrame(state: DocumentState, frame: ClientFrame, lane: WireLane): void {
  if (state.socket?.readyState === WebSocket.OPEN) state.socket.send(encodeClientFrame(frame, lane));
}

/** 🧺 Builds + sends one `Commands` batch, tracking it in `pendingBatches` for
 * {@link handleAck}. Mirrors the Rust actor's `relay_operations_to_hub`. */
function relayOperationsToHub(state: DocumentState, envelopes: readonly OperationEnvelope[]): void {
  if (state.socket?.readyState !== WebSocket.OPEN || envelopes.length === 0) return;
  const batchId = state.nextBatchId;
  state.nextBatchId += 1;
  const wireEnvelopes = envelopes.map((envelope) => toWireEnvelope(envelope, nextWireTimestamp(state)));
  state.pendingBatches.set(batchId, [...envelopes]);
  sendWireFrame(state, { Commands: { batch_id: batchId, envelopes: wireEnvelopes } }, "command");
}

/** 📮 Resolves one outbound `Commands` batch's terminal `Applied` stage — mirrors the Rust actor's
 * `handle_ack`. `pendingOperations` (the UI-facing "unconfirmed" count) is trimmed by id, the same
 * way the old per-operation `ack` frame used to. */
function handleAck(state: DocumentState, batchId: number, stages: readonly WireAckStage[]): void {
  for (const stage of stages) {
    if (typeof stage !== "object" || !("Applied" in stage)) continue;
    const sent = state.pendingBatches.get(batchId);
    state.pendingBatches.delete(batchId);
    if (!sent) continue;
    const sentIds = new Set(sent.map((envelope) => envelope.id));
    state.pendingOperations = state.pendingOperations.filter((envelope) => !sentIds.has(envelope.id));

    const outcome = stage.Applied.outcome;
    let ackOutcome: CommandAckOutcome;
    if (outcome === "Accepted") {
      ackOutcome = { kind: "accepted" };
    } else if ("Transformed" in outcome) {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      if (rollbacks.length > 0) emitEvent(state.config.documentId, { kind: "remoteOperations", envelopes: rollbacks });
      const converted = fromWireEnvelope(outcome.Transformed.envelope);
      emitEvent(state.config.documentId, { kind: "remoteOperations", envelopes: [converted] });
      ackOutcome = { kind: "transformed" };
    } else {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      if (rollbacks.length > 0) emitEvent(state.config.documentId, { kind: "remoteOperations", envelopes: rollbacks });
      ackOutcome = { kind: "rejected", reason: outcome.Rejected.reason };
    }
    setStatus(state, { pendingOperations: state.pendingOperations.length });
    emitEvent(state.config.documentId, { kind: "commandOutcome", batchId, outcome: ackOutcome });
  }
}

function handleHubFrame(state: DocumentState, frame: ServerFrame): void {
  if (typeof frame === "string") return; // no unit-variant `ServerFrame` exists today; defensive.
  if ("Welcome" in frame) {
    state.resumeToken = frame.Welcome.resume_token;
    state.frontier = frame.Welcome.server_frontier;
    // 📡 `Welcome` no longer carries a presence roster (wire v2 splits it into its own `Presence`
    // frame) — `peerCount` is corrected once that frame arrives.
    setRemote(state, { kind: "live", peerCount: 0 });
    // 📦 Pack-based snapshot bootstrap (`Welcome.bootstrap.Snapshot`): no client-side pack decoder
    // wired this wave (db/pack integration is a CW6+ hub-rebuild concern, mirrors the Rust actor's
    // identical deferral) — accepted and ignored; catch-up relies on the hub's follow-up `Commands`.
    return;
  }
  if ("SnapshotChunk" in frame || "SnapshotDone" in frame) {
    // 📦 See the `Welcome.bootstrap.Snapshot` note above — accepted and ignored.
    return;
  }
  if ("Commands" in frame) {
    state.frontier = frame.Commands.frontier;
    if (frame.Commands.origin !== state.config.actor) {
      const envelopes = frame.Commands.envelopes.map(fromWireEnvelope);
      emitEvent(state.config.documentId, { kind: "remoteOperations", envelopes });
    }
    return;
  }
  if ("Ack" in frame) {
    state.frontier = frame.Ack.frontier;
    handleAck(state, frame.Ack.batch_id, frame.Ack.stages);
    return;
  }
  if ("Preview" in frame) {
    if (frame.Preview.actor !== state.config.actor) emitEvent(state.config.documentId, { kind: "preview", actor: frame.Preview.actor, key: frame.Preview.key, seq: frame.Preview.seq, payload: frame.Preview.payload });
    return;
  }
  if ("Presence" in frame) {
    // 📡 `ServerFrame::Presence.peers` is opaque JSON on the wire (`Vec<serde_json::Value>`);
    // trusted-cast to the locally-known peer shape, same trust boundary the Rust actor's
    // `presence_from_json` (`serde_json::from_value`) crosses.
    emitEvent(state.config.documentId, { kind: "presence", peers: frame.Presence.peers as DocumentPresencePeer[] });
    return;
  }
  if ("CreditGrant" in frame) {
    // 🪙 Command-lane credit-based flow control: no client-side backpressure implemented this wave
    // (scope is frame plumbing, not congestion control) — accepted and ignored.
    return;
  }
  if ("Error" in frame) {
    emitEvent(state.config.documentId, { kind: "conflict", message: frame.Error.message });
  }
}
//#endregion 🔖Hub

//#region 🔖BlobCache
/** 📦 Must match `framework/product/os/core/js/index.ts`'s `BLOB_ENDPOINT_PATH`. A hub-backed fallback
 * (for documents synced through a hub rather than a dev folder) is 0G's job once that route exists —
 * this worker only ever talks to the dev middleware today. */
const BLOB_ENDPOINT_PATH = "/semio-blob";

const BLOB_CACHE_DB_NAME = "semio-blob-cache";
const BLOB_CACHE_DB_VERSION = 1;
const BLOB_CACHE_STORE_NAME = "semio-blobs";
const BLOB_CACHE_LAST_ACCESSED_INDEX = "lastAccessedAt";
/** 💾 IndexedDB eviction budget for the browser blob cache. 512 MiB comfortably fits a working set of
 * document media (images/audio/small video clips) without risking the browser's own storage-pressure
 * eviction of the whole origin; raise this once real usage data says otherwise. */
const BLOB_CACHE_BUDGET_BYTES = 512 * 1024 * 1024;

type CachedBlobRecord = { hash: string; mediaType: string; size: number; bytes: ArrayBuffer; lastAccessedAt: number };

function idbRequest<T>(request: IDBRequest): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result as T);
    request.onerror = () => reject(request.error ?? new Error("indexeddb request failed"));
  });
}

function openBlobCacheDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(BLOB_CACHE_DB_NAME, BLOB_CACHE_DB_VERSION);
    request.onupgradeneeded = () => {
      const db = request.result;
      if (!db.objectStoreNames.contains(BLOB_CACHE_STORE_NAME)) {
        const store = db.createObjectStore(BLOB_CACHE_STORE_NAME, { keyPath: "hash" });
        store.createIndex(BLOB_CACHE_LAST_ACCESSED_INDEX, "lastAccessedAt");
      }
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("failed to open blob cache database"));
  });
}

let blobCacheDbPromise: Promise<IDBDatabase> | null = null;
function blobCacheDb(): Promise<IDBDatabase> {
  if (!blobCacheDbPromise) blobCacheDbPromise = openBlobCacheDb();
  return blobCacheDbPromise;
}

/** 🧮 Running cache size, lazily seeded from a full scan on first use and kept in sync by
 * {@link writeCachedBlob}/eviction from then on — avoids a cursor sum on every put. */
let cachedTotalBytes: number | null = null;

async function blobCacheTotalBytes(db: IDBDatabase): Promise<number> {
  if (cachedTotalBytes != null) return cachedTotalBytes;
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readonly");
  const records = await idbRequest<CachedBlobRecord[]>(tx.objectStore(BLOB_CACHE_STORE_NAME).getAll());
  cachedTotalBytes = records.reduce((sum, record) => sum + record.size, 0);
  return cachedTotalBytes;
}

/** ♻️ Evicts least-recently-accessed entries (via the `lastAccessedAt` index, ascending order) until
 * the running total drops back under {@link BLOB_CACHE_BUDGET_BYTES}. */
async function evictBlobCacheOverBudget(db: IDBDatabase): Promise<void> {
  let total = await blobCacheTotalBytes(db);
  if (total <= BLOB_CACHE_BUDGET_BYTES) return;
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readwrite");
  const index = tx.objectStore(BLOB_CACHE_STORE_NAME).index(BLOB_CACHE_LAST_ACCESSED_INDEX);
  await new Promise<void>((resolve, reject) => {
    const cursorRequest = index.openCursor();
    cursorRequest.onsuccess = () => {
      const cursor = cursorRequest.result;
      if (!cursor || total <= BLOB_CACHE_BUDGET_BYTES) {
        resolve();
        return;
      }
      const record = cursor.value as CachedBlobRecord;
      total -= record.size;
      cursor.delete();
      cursor.continue();
    };
    cursorRequest.onerror = () => reject(cursorRequest.error ?? new Error("blob cache eviction cursor failed"));
  });
  cachedTotalBytes = total;
}

async function readCachedBlob(hash: string): Promise<CachedBlobRecord | null> {
  const db = await blobCacheDb();
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readonly");
  const record = await idbRequest<CachedBlobRecord | undefined>(tx.objectStore(BLOB_CACHE_STORE_NAME).get(hash));
  return record ?? null;
}

async function writeCachedBlob(record: CachedBlobRecord): Promise<void> {
  const db = await blobCacheDb();
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readwrite");
  await idbRequest(tx.objectStore(BLOB_CACHE_STORE_NAME).put(record));
  cachedTotalBytes = (cachedTotalBytes ?? (await blobCacheTotalBytes(db))) + record.size;
  await evictBlobCacheOverBudget(db);
}

/** 📥 Reads a blob by hash — cache-first (bumping `lastAccessedAt` for LRU), falling back to the dev
 * server's `GET ${BLOB_ENDPOINT_PATH}/:hash` on a miss and populating the cache. Nothing outside this
 * worker calls this yet (no plugin/UI surface consumes blobs today), so it stays internal rather than
 * growing {@link BackboneWorkerRequest}/{@link BackboneWorkerResponse} with variants nothing sends. */
async function getCachedBlob(hash: string): Promise<{ bytes: Uint8Array; mediaType: string } | null> {
  const cached = await readCachedBlob(hash);
  if (cached) {
    void writeCachedBlob({ ...cached, lastAccessedAt: Date.now() });
    return { bytes: new Uint8Array(cached.bytes), mediaType: cached.mediaType };
  }
  const response = await fetch(`${BLOB_ENDPOINT_PATH}/${encodeURIComponent(hash)}`);
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`blob fetch failed (${response.status})`);
  const mediaType = response.headers.get("content-type") ?? "application/octet-stream";
  const buffer = await response.arrayBuffer();
  await writeCachedBlob({ hash, mediaType, size: buffer.byteLength, bytes: buffer, lastAccessedAt: Date.now() });
  return { bytes: new Uint8Array(buffer), mediaType };
}

/** 📤 Writes a blob to the dev server's content-addressed store, caching it locally under the hash the
 * server returns (content-addressing means the caller can't pick the cache key up front). */
async function putCachedBlob(bytes: Uint8Array, mediaType: string): Promise<string> {
  const response = await fetch(`${BLOB_ENDPOINT_PATH}?mediaType=${encodeURIComponent(mediaType)}`, {
    method: "PUT",
    headers: { "content-type": "application/octet-stream" },
    body: bytes,
  });
  if (!response.ok) throw new Error(`blob put failed (${response.status})`);
  const { hash } = (await response.json()) as { hash: string };
  await writeCachedBlob({ hash, mediaType, size: bytes.byteLength, bytes: bytes.slice().buffer, lastAccessedAt: Date.now() });
  return hash;
}

// 🧷 Referenced defensively so `getCachedBlob`/`putCachedBlob` aren't flagged unused before a
// plugin/UI surface calls into them — both are the intended entry points once one does.
void getCachedBlob;
void putCachedBlob;
//#endregion 🔖BlobCache

//#region 🔖Lifecycle
function openDocument(config: DocumentActorConfig): void {
  closeDocument(config.documentId);
  const channel = new BroadcastChannel(`semio-doc-${config.documentId}`);
  const state: DocumentState = {
    config,
    channel,
    socket: null,
    pollTimer: null,
    reconnectTimer: null,
    reconnectDelayMs: HUB_RECONNECT_MIN_MS,
    pendingOperations: [],
    status: { persisted: false, pendingOperations: 0, remote: { kind: "detached" } },
    frontier: null,
    resumeToken: null,
    pendingBatches: new Map(),
    nextBatchId: 0,
    hlcCounter: 0,
    closed: false,
  };
  documents.set(config.documentId, state);
  channel.onmessage = (messageEvent) => {
    const envelopes = messageEvent.data as OperationEnvelope[];
    if (Array.isArray(envelopes) && envelopes.length > 0) emitEvent(config.documentId, { kind: "remoteOperations", envelopes });
  };
  const folder = folderBinding(config);
  if (folder) {
    if (config.watchExternal !== false) watchFolder(state, folder);
    else void pollFolderOnce(state, folder);
  }
  const hub = hubBinding(config);
  if (hub) connectHub(state, hub);
  emitEvent(config.documentId, { kind: "status", ...state.status });
}

function closeDocument(documentId: string): void {
  const state = documents.get(documentId);
  if (!state) return;
  state.closed = true;
  state.socket?.close();
  if (state.pollTimer != null) clearInterval(state.pollTimer);
  if (state.reconnectTimer != null) clearTimeout(state.reconnectTimer);
  state.channel.close();
  documents.delete(documentId);
}

async function handleLocalMsg(state: DocumentState, message: DocumentActorMsg): Promise<void> {
  switch (message.kind) {
    case "localOperations": {
      if (message.envelopes.length === 0) break; // pure wake
      state.pendingOperations.push(...message.envelopes);
      setStatus(state, { pendingOperations: state.pendingOperations.length });
      state.channel.postMessage(message.envelopes);
      relayOperationsToHub(state, message.envelopes);
      const folder = folderBinding(state.config);
      // 📁 Folder persistence only understands whole-envelope snapshots today (`vcs::FolderSqliteStorage`
      // stores one json blob per document) — a local operation still marks the document dirty so the next
      // `localSnapshot` (which every `store.dispatch` triggers via `flush_outbound`) persists it.
      if (folder) setStatus(state, { persisted: false });
      break;
    }
    case "localSnapshot": {
      const folder = folderBinding(state.config);
      if (folder) {
        try {
          await writeFolder(state, folder, message.envelopeJson);
          state.pendingOperations = [];
          setStatus(state, { pendingOperations: 0 });
        } catch (error) {
          console.error("[backbone-worker] folder write failed", state.config.documentId, error);
        }
      }
      // 📸 No client -> hub whole-envelope push exists in wire v2 (`ClientFrame` has no snapshot-put
      // variant, only causally-ordered `Commands`) — mirrors the Rust actor's identical deferral
      // (`framework/sync/rs/lib.rs` `drain_and_relay`'s `BackboneMessage::Snapshot` arm) rather than
      // a bug here; the folder write above still persists it.
      break;
    }
    case "presenceHeartbeat":
      sendWireFrame(state, { Presence: { peer: message.peer } }, "preview");
      break;
    case "publishPreview":
      sendWireFrame(state, { PreviewPublish: { key: message.key, seq: message.seq, payload: message.payload } }, "preview");
      break;
    case "externalChanged": {
      const folder = folderBinding(state.config);
      if (folder) void pollFolderOnce(state, folder);
      break;
    }
    case "detach":
      closeDocument(state.config.documentId);
      break;
  }
}
//#endregion 🔖Lifecycle

//#region 🔖MessageBridge
function handleTsRequest(request: BackboneWorkerRequest): void {
  switch (request.kind) {
    case "open":
      openDocument(request);
      break;
    case "close":
      closeDocument(request.documentId);
      break;
    case "send": {
      const state = documents.get(request.documentId);
      if (state) void handleLocalMsg(state, request.message);
      break;
    }
  }
}
//#endregion 🔖MessageBridge
//#endregion 🔖TsFallback

//#region 🧪Tests
// 🧵 Whole block stripped from production builds (see this file's header doc) — `node:*` imports
// below are dynamic specifically so they never get bundled into the actual browser Worker script.
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  function sampleEnvelope(): OperationEnvelope {
    return {
      id: "edit-1",
      actor: "actor-1",
      document: "doc-1",
      schemaVersion: "demo/v1",
      deps: [],
      payloadHash: "unused-in-this-fallback",
      diff: { schemaId: "demo/v1", payload: { n: 5, sequenceNumber: 1 } },
      inverse: { targetOperation: "edit-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
    };
  }

  describe("backbone-worker wire bridge", () => {
    it("round-trips an OperationEnvelope through toWireEnvelope/fromWireEnvelope", () => {
      const envelope = sampleEnvelope();
      const wire = toWireEnvelope(envelope, { actor: 1, physical_ms: 2, logical: 3 });
      expect(wire.operation_id).toBe(envelope.id);
      expect(wire.document_id).toBe(envelope.document);
      expect(wire.actor).toBe(envelope.actor);
      expect(wire.diff.payload).toEqual(envelope.diff.payload);

      const recovered = fromWireEnvelope(wire);
      expect(recovered.id).toBe(envelope.id);
      expect(recovered.document).toBe(envelope.document);
      expect(recovered.diff.payload).toEqual(envelope.diff.payload);
      expect(recovered.inverse.inverseDiff.payload).toEqual(envelope.inverse.inverseDiff.payload);
    });

    it("rollbackEnvelope synthesizes an undo from the original inverse", () => {
      const envelope = sampleEnvelope();
      const rollback = rollbackEnvelope(envelope);
      expect(rollback.deps).toEqual([envelope.id]);
      expect(rollback.diff.payload).toEqual(envelope.inverse.inverseDiff.payload);
      expect(rollback.id).not.toBe(envelope.id);
    });

    it("encodeClientFrame/decodeClientFrame round-trip a Hello frame", () => {
      const frame: ClientFrame = { Hello: { wire_version: 1, protocol_version: 1, schema: "demo/v1", pack_schema_hash: new Array(32).fill(0), actor: "actor-1", token: null, resume_token: null, frontier: null } };
      const bytes = encodeClientFrame(frame, "command");
      const decoded = decodeClientFrame(bytes);
      expect(decoded.lane).toBe("command");
      expect(decoded.frame).toEqual(frame);
    });

    // 🎬 Shared fixtures: the exact same bytes `framework/sync/rs/lib.rs`'s
    // `wire_fixtures_stay_byte_identical_across_rust_and_ts` test generates and verifies Rust-side.
    // Decoding them here, then re-encoding the decoded value and diffing against the original bytes,
    // proves the TS codec agrees with `protocol_wire`'s Rust codec byte-for-byte, not just shape-wise.
    it("decodes the Rust-generated binary wire fixtures byte-identically", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const fixturesDir = join(dirname(fileURLToPath(import.meta.url)), "../../../../../store/sync/fixtures/wire");

      const helloBytes = new Uint8Array(readFileSync(join(fixturesDir, "client-hello.bin")));
      const hello = decodeClientFrame(helloBytes);
      expect(hello.lane).toBe("command");
      if (typeof hello.frame === "string" || !("Hello" in hello.frame)) throw new Error("expected a Hello frame");
      expect(hello.frame.Hello.schema).toBe("demo/v1");
      expect(hello.frame.Hello.actor).toBe("actor-1");
      expect(encodeClientFrame(hello.frame, "command")).toEqual(helloBytes);

      const commandsBytes = new Uint8Array(readFileSync(join(fixturesDir, "client-commands.bin")));
      const commands = decodeClientFrame(commandsBytes);
      if (typeof commands.frame === "string" || !("Commands" in commands.frame)) throw new Error("expected a Commands frame");
      expect(commands.frame.Commands.envelopes).toHaveLength(1);
      expect(commands.frame.Commands.envelopes[0]?.diff.payload).toEqual({ n: 5, sequenceNumber: 1 });
      expect(encodeClientFrame(commands.frame, "command")).toEqual(commandsBytes);

      const welcomeBytes = new Uint8Array(readFileSync(join(fixturesDir, "server-welcome.bin")));
      const welcome = decodeServerFrame(welcomeBytes);
      if (typeof welcome.frame === "string" || !("Welcome" in welcome.frame)) throw new Error("expected a Welcome frame");
      expect(welcome.frame.Welcome.resume_token).toBe("resume-1");
      expect(encodeServerFrame(welcome.frame, "command")).toEqual(welcomeBytes);

      const ackBytes = new Uint8Array(readFileSync(join(fixturesDir, "server-ack.bin")));
      const ack = decodeServerFrame(ackBytes);
      if (typeof ack.frame === "string" || !("Ack" in ack.frame)) throw new Error("expected an Ack frame");
      expect(ack.frame.Ack.batch_id).toBe(1);
      expect(ack.frame.Ack.stages).toHaveLength(3);
      expect(encodeServerFrame(ack.frame, "command")).toEqual(ackBytes);
    });
  });
}
//#endregion 🧪Tests
