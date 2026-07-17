// #region Header
/**
 * @emoji 🧵 `backbone-worker.ts` — the browser-side actor twin of `framework/sync`'s Rust
 * `DocumentHost`. Runs in a dedicated Web Worker so all document-sync IO (hub WebSocket, folder
 * fetch/SSE polling, multi-tab convergence) happens off the main thread — the UI is never blocked.
 *
 * Protocol types ({@link BackboneWorkerRequest}/{@link BackboneWorkerResponse}/{@link DocumentEvent}/
 * {@link DocumentActorMsg}/{@link PersistenceBinding}) live in `./index.ts` so both this worker and
 * `os-shell.tsx` import the same shapes. This worker is deliberately dumb — it relays queues and
 * fans out `DocumentEvent`s, it never materializes a projection (that stays the plugin store's job,
 * same division of labor as the Rust actor's `ChannelBackbone`).
 *
 * Per-document responsibilities:
 * - `PersistenceBinding.hub`: a `WebSocket` to `${baseUrl}/studios/{studioId}/documents/{id}/ws`,
 *   speaking the exact `HubClientFrame`/`HubServerFrame` JSON the kernel module (`framework/core/rs`'s
 *   🔖HubProtocol region) and the hub server (`framework/product/os/hub/rs/bin.rs`) use.
 * - `PersistenceBinding.folder`: fetch/SSE against the dev middleware's `/semio-backbone` endpoint.
 *   The middleware's multi-document SSE watch endpoint (`GET /semio-backbone/watch?uri=`) is a
 *   dev-workflow deliverable (`framework/product/os/dev/script.ts`) that may land after this file;
 *   until an `EventSource` connects successfully this degrades to polling the envelope endpoint on
 *   an interval — a documented, functional fallback, not a silent gap.
 * - A `BroadcastChannel` per document id fans local ops out to other tabs open on the same document
 *   and ingests theirs, for same-machine multi-tab convergence independent of any server.
 */
// #endregion Header

import type { BackboneWorkerRequest, BackboneWorkerResponse, DocumentActorConfig, DocumentActorMsg, DocumentEvent, DocumentSyncStatus, HubClientFrame, HubServerFrame, OpEnvelope, PersistenceBinding, RemoteState } from "./index";

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
  pendingOps: OpEnvelope[];
  status: DocumentSyncStatus;
  sinceVersion: number;
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
  state.socket = socket;
  socket.onopen = () => {
    state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
    sendHubFrame(state, { kind: "hello", actor: state.config.actor, token: binding.token, sinceVersion: state.sinceVersion });
  };
  socket.onmessage = (messageEvent) => {
    try {
      handleHubFrame(state, JSON.parse(messageEvent.data as string) as HubServerFrame);
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

function sendHubFrame(state: DocumentState, frame: HubClientFrame): void {
  if (state.socket?.readyState === WebSocket.OPEN) state.socket.send(JSON.stringify(frame));
}

function handleHubFrame(state: DocumentState, frame: HubServerFrame): void {
  switch (frame.kind) {
    case "welcome":
      state.sinceVersion = frame.version;
      setRemote(state, { kind: "live", peerCount: frame.presence.length });
      if (frame.envelope != null) emitEvent(state.config.documentId, { kind: "snapshotReplaced", envelopeJson: JSON.stringify(frame.envelope) });
      if (frame.backlog.length > 0) emitEvent(state.config.documentId, { kind: "remoteOps", envelopes: frame.backlog });
      emitEvent(state.config.documentId, { kind: "presence", peers: frame.presence });
      break;
    case "ops":
      state.sinceVersion = frame.version;
      if (frame.origin !== state.config.actor) emitEvent(state.config.documentId, { kind: "remoteOps", envelopes: frame.envelopes });
      break;
    case "snapshotReplaced":
      state.sinceVersion = frame.version;
      emitEvent(state.config.documentId, { kind: "snapshotReplaced", envelopeJson: JSON.stringify(frame.envelope) });
      break;
    case "presence":
      emitEvent(state.config.documentId, { kind: "presence", peers: frame.peers });
      break;
    case "ack":
      state.pendingOps = state.pendingOps.filter((envelope) => envelope.id !== frame.opId);
      setStatus(state, { pendingOps: state.pendingOps.length });
      break;
    case "conflict":
      emitEvent(state.config.documentId, { kind: "conflict", message: frame.message });
      break;
    case "error":
      console.error("[backbone-worker] hub error", state.config.documentId, frame.message);
      break;
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
    pendingOps: [],
    status: { persisted: false, pendingOps: 0, remote: { kind: "detached" } },
    sinceVersion: 0,
    closed: false,
  };
  documents.set(config.documentId, state);
  channel.onmessage = (messageEvent) => {
    const envelopes = messageEvent.data as OpEnvelope[];
    if (Array.isArray(envelopes) && envelopes.length > 0) emitEvent(config.documentId, { kind: "remoteOps", envelopes });
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
    case "localOps": {
      if (message.envelopes.length === 0) break; // pure wake
      state.pendingOps.push(...message.envelopes);
      setStatus(state, { pendingOps: state.pendingOps.length });
      state.channel.postMessage(message.envelopes);
      sendHubFrame(state, { kind: "ops", envelopes: message.envelopes });
      const folder = folderBinding(state.config);
      // 📁 Folder persistence only understands whole-envelope snapshots today (`vcs::FolderSqliteStorage`
      // stores one json blob per document) — a local op still marks the document dirty so the next
      // `localSnapshot` (which every `store.dispatch` triggers via `flush_outbound`) persists it.
      if (folder) setStatus(state, { persisted: false });
      break;
    }
    case "localSnapshot": {
      const folder = folderBinding(state.config);
      if (folder) {
        try {
          await writeFolder(state, folder, message.envelopeJson);
          state.pendingOps = [];
          setStatus(state, { pendingOps: 0 });
        } catch (error) {
          console.error("[backbone-worker] folder write failed", state.config.documentId, error);
        }
      }
      const hub = hubBinding(state.config);
      if (hub) sendHubFrame(state, { kind: "putEnvelope", version: state.sinceVersion, envelope: JSON.parse(message.envelopeJson) });
      break;
    }
    case "presenceHeartbeat":
      sendHubFrame(state, { kind: "presence", peer: message.peer });
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
(self as unknown as DedicatedWorkerGlobalScope).onmessage = (messageEvent: MessageEvent<BackboneWorkerRequest>) => {
  const request = messageEvent.data;
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
};

post({ kind: "ready" });
//#endregion 🔖MessageBridge
