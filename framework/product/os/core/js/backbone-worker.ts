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
 * - `PersistenceBinding.hub`: a `WebSocket` to `${baseUrl}/documents/{id}/ws`, speaking the exact
 *   `HubClientFrame`/`HubServerFrame` JSON the kernel module (`framework/core/rs`'s 🔖HubProtocol
 *   region) and the hub server (`framework/product/os/hub/rs/bin.rs`) use.
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
  const socket = new WebSocket(`${wsBase}/documents/${encodeURIComponent(state.config.documentId)}/ws`);
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
