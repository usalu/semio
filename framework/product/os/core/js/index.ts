// #region Header
/**
 * 🖥️ `@semio-tech/framework-os-core` — JS sync/backbone protocol surface (backbone URIs, document
 * envelopes, `backbone-worker.ts` request/response wire types, `PersistenceBinding`/`OperationEnvelope`,
 * {@link buildFrameworkSyncUtilities}) consumed by `framework/renderer/react/index.tsx` and
 * `framework/product/os/dev/script.ts`. The OS kernel's *stateful* logic (operation application, program
 * registry) is Rust/wasm-only, hosted by the s-plugin wasm — this file is not a JS port of that. The
 * one exception is {@link planMediaFlow}: a pure, side-effect-free scheduling function has no state
 * to keep in sync with a live wasm host, so it's hand-mirrored here against the Rust `plan_media_flow`
 * (`framework/product/os/core/rs/lib.rs`) with shared fixtures (`framework/product/os/core/fixtures/`)
 * asserting parity. This file still exposes a small legacy `osBaselineResource`/
 * `mergeOsProgramDefinition`/`registerAppVcsHandler` app-registration shim kept alive only because
 * `compose/client/lib/sketchpad/js/index.ts` still calls it; do not extend that shim further.
 */
// #endregion Header

import type { UtilityLeaf } from "@semio-tech/framework-core";

export type OsProgramResourceMap = Readonly<Record<string, { readonly kind: string; readonly id: string; readonly label: string }>>;

const programDefinitions = new Map<string, unknown>();
const vcsHandlers = new Set<() => void>();

export function osBaselineResource(kind: string, id: string, label: string) {
  return { kind, id, label };
}

export function mergeOsProgramDefinition(programId: string, definition: unknown, resources?: OsProgramResourceMap): void {
  programDefinitions.set(programId, { definition, resources });
}

export function registerAppVcsHandler(handler: () => void): void {
  vcsHandlers.add(handler);
}

//#region 🔖Backbone
export const FRAMEWORK_SYNC_CONTROLLER_ID = "framework.sync";

/** 🛰️ Dev-server-proxied backbone endpoint path for `file://`/`folder://` uris; shared with the dev host shim (`framework/product/os/dev/script.ts`) so both stay in sync on the same literal. */
export const BACKBONE_ENDPOINT_PATH = "/semio-backbone";

export type BackboneKind = "file" | "folder" | "remote" | "unknown";

export type DocumentBackboneRef = {
  readonly kind: BackboneKind;
  readonly uri: string;
};

export function backboneKindFromUri(uri: string): BackboneKind {
  if (uri.startsWith("file://")) return "file";
  if (uri.startsWith("folder://")) return "folder";
  if (uri.startsWith("remote://")) return "remote";
  return "unknown";
}

export function documentBackboneRef(uri: string): DocumentBackboneRef {
  return { kind: backboneKindFromUri(uri), uri };
}

export function parseRemoteBackboneUri(uri: string): { readonly hostPort: string; readonly studioId: string; readonly documentId: string } | null {
  if (!uri.startsWith("remote://")) return null;
  const rest = uri.slice("remote://".length);
  const firstSlash = rest.indexOf("/");
  if (firstSlash <= 0) return null;
  const secondSlash = rest.indexOf("/", firstSlash + 1);
  if (secondSlash <= 0) return null;
  return { hostPort: rest.slice(0, firstSlash), studioId: rest.slice(firstSlash + 1, secondSlash), documentId: rest.slice(secondSlash + 1) };
}

export function buildRemoteBackboneUri(hostPort: string, studioId: string, documentId: string): string {
  return `remote://${hostPort}/${studioId}/${documentId}`;
}

export function buildFileBackboneUri(path: string): string {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `file://${normalized}`;
}

export function buildFolderBackboneUri(path: string): string {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `folder://${normalized}`;
}

function remoteEnvelopeUrl(remote: { readonly hostPort: string; readonly studioId: string; readonly documentId: string }): string {
  return `http://${remote.hostPort}/studios/${encodeURIComponent(remote.studioId)}/documents/${encodeURIComponent(remote.documentId)}/envelope`;
}

export async function readBackboneEnvelope(uri: string): Promise<string | null> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) return null;
    const response = await fetch(remoteEnvelopeUrl(remote));
    if (response.status === 404) return null;
    if (!response.ok) throw new Error(`remote backbone read failed (${response.status})`);
    const body = (await response.json()) as { envelope?: unknown };
    return JSON.stringify(body.envelope ?? body);
  }
  const response = await fetch(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`);
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`backbone read failed (${response.status})`);
  return response.text();
}

export async function writeBackboneEnvelope(uri: string, envelopeJson: string): Promise<void> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) throw new Error(`invalid remote backbone uri: ${uri}`);
    const current = await fetch(remoteEnvelopeUrl(remote));
    const version = current.ok ? Number(((await current.json()) as { version?: number }).version ?? 0) : 0;
    const response = await fetch(remoteEnvelopeUrl(remote), {
      method: "PUT",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ version, envelope: JSON.parse(envelopeJson) }),
    });
    if (!response.ok) throw new Error(`remote backbone write failed (${response.status})`);
    console.log("[DEBUG] remote backbone synced", uri);
    return;
  }
  const response = await fetch(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`, {
    method: "PUT",
    headers: { "content-type": "application/json" },
    body: envelopeJson,
  });
  if (!response.ok) throw new Error(`backbone write failed (${response.status})`);
  console.log("[DEBUG] backbone synced", uri);
}

export function documentFromEnvelopeJson(envelopeJson: string): unknown {
  const parsed = JSON.parse(envelopeJson) as { projection?: unknown; document?: unknown; vcs?: unknown };
  if (parsed.projection != null) return parsed.projection;
  if (parsed.document != null) return parsed.document;
  return parsed;
}

export function wrapDocumentEnvelope(document: unknown, documentId: string, uri: string): string {
  if (document && typeof document === "object" && "vcs" in (document as Record<string, unknown>)) {
    const envelope = { ...(document as Record<string, unknown>), backbone: documentBackboneRef(uri) };
    return JSON.stringify(envelope);
  }
  return JSON.stringify({
    schema: "document/v1",
    id: documentId,
    projection: document,
    vcs: { edits: [], changes: [], checkpoints: [], alternatives: [], operations: [] },
    backbone: documentBackboneRef(uri),
  });
}

//#region 🔀ApplyBackboneMessage
export type BackboneOpEnvelope = { readonly diff?: { readonly payload?: { readonly id?: string } & Record<string, unknown> } };

export type BackboneMessage = { readonly kind: "snapshot"; readonly envelopeJson: string } | { readonly kind: "operations"; readonly envelopes?: readonly BackboneOpEnvelope[] };

/**
 * 🔀 Mirrors `vcs::storage_send` — applies an incoming backbone message on top of a previously
 * stored envelope: a `snapshot` message overwrites, an `operations` message appends into `vcs.edits`
 * deduped by id. This is the canonical implementation; the dev host shim's generated JS
 * (`hostShimSource` in `framework/product/os/dev/script.ts`) hand-ports the same algorithm and
 * must be kept in sync until a build-time inlining step exists.
 */
export function applyBackboneMessage(storedEnvelopeJson: string | null, messageJson: string): string {
  const message = JSON.parse(messageJson) as BackboneMessage;
  if (message.kind === "snapshot") return message.envelopeJson;
  if (message.kind === "operations") {
    if (storedEnvelopeJson == null) throw new Error("cannot append operations before a snapshot exists");
    const envelope = JSON.parse(storedEnvelopeJson) as { vcs?: { edits?: unknown[] } };
    const edits = envelope?.vcs?.edits;
    if (!Array.isArray(edits)) throw new Error("stored envelope missing vcs.edits");
    const seen = new Set(edits.map((edit) => (edit as { id?: unknown })?.id).filter((id): id is string => typeof id === "string"));
    for (const operationEnvelope of message.envelopes ?? []) {
      const editJson = operationEnvelope?.diff?.payload;
      const id = editJson?.id;
      if (typeof id === "string") {
        if (seen.has(id)) continue;
        seen.add(id);
      }
      edits.push(editJson);
    }
    return JSON.stringify(envelope);
  }
  throw new Error(`unsupported backbone message kind: ${(message as { kind: string }).kind}`);
}
//#endregion 🔀ApplyBackboneMessage

/** 🍃 Sync-controller-scoped toggle leaf — narrows the canonical {@link UtilityLeaf} `"toggle"` variant instead of duplicating its fields. */
export type FrameworkSyncUtilityLeaf = Extract<UtilityLeaf, { readonly kind: "toggle" }> & {
  readonly category: "sync";
  readonly controllerId: typeof FRAMEWORK_SYNC_CONTROLLER_ID;
  readonly action: string;
};

export function buildFrameworkSyncUtilities(activeUri: string | null): readonly FrameworkSyncUtilityLeaf[] {
  const activeKind = activeUri ? backboneKindFromUri(activeUri) : null;
  const pressed = (kind: BackboneKind) => activeKind === kind;
  return [
    { id: "framework.sync.file", kind: "toggle", iconId: "file-json", label: "File", category: "sync", pressed: pressed("file"), order: 0, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectFile" },
    { id: "framework.sync.folder", kind: "toggle", iconId: "folder", label: "Folder", category: "sync", pressed: pressed("folder"), order: 1, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectFolder" },
    { id: "framework.sync.remote", kind: "toggle", iconId: "cloud", label: "Remote", category: "sync", pressed: pressed("remote"), order: 2, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectRemote" },
  ];
}
//#endregion 🔖Backbone

//#region 🔖DesktopWindowChrome
/** 🪟 IPC channel names for the desktop window chrome controls (minimize/maximize/close) — shared literal between a host's `ipcMain.handle` registration and the renderer's `invoke` bridge. */
export const DESKTOP_WINDOW_CONTROL_CHANNELS = {
  minimize: "framework.window.minimize",
  maximize: "framework.window.maximize",
  close: "framework.window.close",
} as const;

/** 🎛️ Renderer-facing surface for the three desktop window chrome controls. */
export type DesktopWindowControls = { minimize(): Promise<unknown>; maximize(): Promise<unknown>; close(): Promise<unknown> };

/**
 * 🔌 Registers host-side handlers for {@link DESKTOP_WINDOW_CONTROL_CHANNELS} against a structural
 * `ipc.handle`-shaped port — no `electron` types leak into this signature; a real Electron app wires
 * its `ipcMain`/`BrowserWindow` in at the call site. `maximize` toggles based on `isMaximized()`;
 * a null `focusedWindow()` is a no-operation.
 */
export function registerDesktopWindowControlHandlers(
  ipc: { handle(channel: string, fn: () => void): void },
  focusedWindow: () => { minimize(): void; isMaximized(): boolean; maximize(): void; unmaximize(): void; close(): void } | null,
): void {
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.minimize, () => {
    focusedWindow()?.minimize();
  });
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.maximize, () => {
    const window = focusedWindow();
    if (!window) return;
    if (window.isMaximized()) window.unmaximize();
    else window.maximize();
  });
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.close, () => {
    focusedWindow()?.close();
  });
}

/** 🌉 Renderer-side {@link DesktopWindowControls} backed by a structural `invoke`-shaped port (e.g. `electron`'s `ipcRenderer.invoke`). */
export function desktopWindowControlsBridge(invoke: (channel: string) => Promise<unknown>): DesktopWindowControls {
  return {
    minimize: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.minimize),
    maximize: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.maximize),
    close: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.close),
  };
}
//#endregion 🔖DesktopWindowChrome

//#region 🔖Blob
/** 📦 Dev-server-proxied content-addressed blob endpoint: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=` (raw
 * bytes body, returns `{"hash":"..."}`) and `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response).
 * Shared with the dev host shim (`framework/product/os/dev/script.ts`'s `hostShimSource`) and the
 * browser blob cache (`backbone-worker.ts`) so all three stay in sync on the same literal. Backed by
 * `vcs::BlobStore`'s native counterpart; a hub-backed route is a later ticket. */
export const BLOB_ENDPOINT_PATH = "/semio-blob";
//#endregion 🔖Blob

//#region 🔖SyncProtocol
/**
 * 🔁 TS mirror of `framework/sync`'s Rust actor protocol (`DocumentActorConfig`/`DocumentActorMsg`/
 * `DocumentEvent`/`DocumentSyncStatus`/`RemoteState`/`PersistenceBinding`) — the wire/postMessage
 * shapes `backbone-worker.ts` speaks, kept camelCase-tag-identical to the Rust side (`#[serde(tag =
 * "kind", rename_all = "camelCase")]`) so a shared JSON fixture suite (`framework/sync/fixtures/`)
 * stays plausible across both runtimes even though this file is a deliberately dumb TS twin (no
 * materialization — it only relays queues, exactly like the Rust actor's `ChannelBackbone` side).
 */
export type OperationEnvelope = {
  readonly id: string;
  readonly actor: string;
  readonly document: string;
  readonly schemaVersion: string;
  readonly deps?: readonly string[];
  readonly payloadHash: string;
  readonly diff: { readonly schemaId: string; readonly payload: unknown };
  readonly inverse: {
    readonly targetOperation: string;
    readonly inverseDiff: { readonly schemaId: string; readonly payload: unknown };
    readonly baseVersion: number;
    readonly dependencies?: readonly string[];
    readonly undoPolicy: string;
  };
};

/** 📡 Wire-protocol presence identity — distinct from the UI-rendering {@link PresencePeer} scene prop. */
export type DocumentPresencePeer = {
  readonly actor: string;
  readonly label?: string;
  readonly selectionJson?: string;
  readonly connectedAtMs: number;
  readonly userId?: string;
  readonly role?: string;
  readonly cursor?: { readonly x: number; readonly y: number };
  readonly viewport?: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly dragGhostJson?: string;
};

/** 📨 Client→server hub wire frames — mirrors Rust `HubClientFrame` byte-for-byte. */
export type HubClientFrame =
  | { readonly kind: "hello"; readonly actor: string; readonly token?: string; readonly sinceVersion: number }
  | { readonly kind: "operations"; readonly envelopes: readonly OperationEnvelope[] }
  | { readonly kind: "putEnvelope"; readonly version: number; readonly envelope: unknown }
  | { readonly kind: "presence"; readonly peer: DocumentPresencePeer }
  | { readonly kind: "bye" };

/** 📬 Server→client hub wire frames — mirrors Rust `HubServerFrame` byte-for-byte. */
export type HubServerFrame =
  | { readonly kind: "welcome"; readonly version: number; readonly envelope?: unknown; readonly presence: readonly DocumentPresencePeer[]; readonly backlog: readonly OperationEnvelope[] }
  | { readonly kind: "operations"; readonly version: number; readonly envelopes: readonly OperationEnvelope[]; readonly origin: string }
  | { readonly kind: "snapshotReplaced"; readonly version: number; readonly envelope: unknown }
  | { readonly kind: "presence"; readonly peers: readonly DocumentPresencePeer[] }
  | { readonly kind: "ack"; readonly operationId: string; readonly version: number }
  | { readonly kind: "conflict"; readonly message: string }
  | { readonly kind: "error"; readonly message: string };

/** 🗃️ A durable place a document synchronizes with — mirrors Rust `PersistenceBinding`. */
export type PersistenceBinding = { readonly kind: "folder"; readonly path: string } | { readonly kind: "hub"; readonly baseUrl: string; readonly studioId: string; readonly token?: string };

/** 🧾 Everything the worker needs to open one document's actor — mirrors `DocumentActorConfig`. */
export type DocumentActorConfig = {
  readonly documentId: string;
  readonly schema: string;
  readonly bindings: readonly PersistenceBinding[];
  readonly watchExternal?: boolean;
  readonly actor: string;
};

/** 📨 Caller→actor control messages — mirrors Rust `DocumentActorMsg`. */
export type DocumentActorMsg =
  | { readonly kind: "localOperations"; readonly envelopes: readonly OperationEnvelope[] }
  | { readonly kind: "localSnapshot"; readonly envelopeJson: string }
  | { readonly kind: "presenceHeartbeat"; readonly peer: DocumentPresencePeer }
  | { readonly kind: "externalChanged" }
  | { readonly kind: "detach" };

/** 📶 Connection state of a document's remote (hub) transport — mirrors Rust `RemoteState`. */
export type RemoteState = { readonly kind: "detached" } | { readonly kind: "connecting" } | { readonly kind: "live"; readonly peerCount: number } | { readonly kind: "backoff"; readonly retryInMs: number };

/** 🚦 Sync health snapshot for status badges — mirrors Rust `DocumentSyncStatus`. */
export type DocumentSyncStatus = {
  readonly persisted: boolean;
  readonly pendingOperations: number;
  readonly remote: RemoteState;
};

/** ⚠️ A structural sync conflict — loosely typed pending a full mirror of `vcs::StudioConflict`; the
 * shell only needs enough to render a conflict card / offer "fork alternative" vs "take theirs". */
export type SyncConflict = { readonly message?: string } & Record<string, unknown>;

/** 📬 Actor→subscriber events — mirrors Rust `DocumentEvent`. */
export type DocumentEvent =
  | { readonly kind: "remoteOperations"; readonly envelopes: readonly OperationEnvelope[] }
  | { readonly kind: "snapshotReplaced"; readonly envelopeJson: string }
  | ({ readonly kind: "status" } & DocumentSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly DocumentPresencePeer[] }
  | ({ readonly kind: "conflict" } & SyncConflict);

/** 📤 Main thread → `backbone-worker.ts` messages. */
export type BackboneWorkerRequest = ({ readonly kind: "open" } & DocumentActorConfig) | { readonly kind: "close"; readonly documentId: string } | { readonly kind: "send"; readonly documentId: string; readonly message: DocumentActorMsg };

/** 📥 `backbone-worker.ts` → main thread messages. */
export type BackboneWorkerResponse = { readonly kind: "event"; readonly documentId: string; readonly event: DocumentEvent } | { readonly kind: "ready" };
//#endregion 🔖SyncProtocol

//#region 🔖MediaFlow
/**
 * 🎬 TS mirror of `media_graph::{OsMediaPort,OsMediaGraphNode,OsMediaGraphEdge,OsMediaGraph}` (Rust,
 * `framework/product/os/core/rs/lib.rs`) — camelCase-field-identical (Rust: `#[serde(rename_all =
 * "camelCase")]`). See this file's header for why only this pure-planner slice is hand-mirrored.
 */
export type OsMediaPort = {
  readonly id: string;
  readonly resourceKind: string;
  readonly direction: string;
};

export type OsMediaGraphNode = {
  readonly id: string;
  readonly instanceId: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly inputs: readonly OsMediaPort[];
  readonly outputs: readonly OsMediaPort[];
};

export type OsMediaGraphEdge = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
};

export type OsMediaGraph = {
  readonly schema: string;
  readonly nodes: readonly OsMediaGraphNode[];
  readonly edges: readonly OsMediaGraphEdge[];
};

/** 🚚 TS twin of Rust `MediaFlowDelivery`. */
export type MediaFlowDelivery = {
  readonly edgeId: string;
  readonly producerInstanceId: string;
  readonly producerPortId: string;
  readonly consumerInstanceId: string;
  readonly consumerPortId: string;
};

/**
 * 🧭 TS twin of Rust `media_flow_topological_node_order` — DFS post-order reversed into a
 * topological node order (source before target); deterministic purely from `graph.nodes`/
 * `graph.edges` insertion order, so it matches the Rust side edge-for-edge.
 */
function mediaFlowTopologicalNodeOrder(graph: OsMediaGraph): readonly string[] {
  const adjacency = new Map<string, string[]>();
  for (const edge of graph.edges) {
    const targets = adjacency.get(edge.sourceNodeId) ?? [];
    targets.push(edge.targetNodeId);
    adjacency.set(edge.sourceNodeId, targets);
  }
  const visited = new Set<string>();
  const order: string[] = [];
  const dfs = (nodeId: string): void => {
    if (visited.has(nodeId)) return;
    visited.add(nodeId);
    for (const next of adjacency.get(nodeId) ?? []) dfs(next);
    order.push(nodeId);
  };
  for (const node of graph.nodes) dfs(node.id);
  order.reverse();
  return order;
}

/**
 * 🚚 TS twin of Rust `plan_media_flow` — plans one {@link MediaFlowDelivery} per edge in the
 * downstream closure of `dirtyInstanceIds`, propagating dirtiness onto each edge's consumer instance
 * so multi-hop chains (A→B→C) resolve in a single topological pass. Pure/side-effect-free.
 */
export function planMediaFlow(graph: OsMediaGraph, dirtyInstanceIds: ReadonlySet<string>): readonly MediaFlowDelivery[] {
  const nodeById = new Map<string, OsMediaGraphNode>(graph.nodes.map((node) => [node.id, node]));
  const edgesBySource = new Map<string, OsMediaGraphEdge[]>();
  for (const edge of graph.edges) {
    const edges = edgesBySource.get(edge.sourceNodeId) ?? [];
    edges.push(edge);
    edgesBySource.set(edge.sourceNodeId, edges);
  }
  const order = mediaFlowTopologicalNodeOrder(graph);
  const dirty = new Set(dirtyInstanceIds);
  const deliveries: MediaFlowDelivery[] = [];
  for (const nodeId of order) {
    const node = nodeById.get(nodeId);
    if (!node || !dirty.has(node.instanceId)) continue;
    for (const edge of edgesBySource.get(nodeId) ?? []) {
      const targetNode = nodeById.get(edge.targetNodeId);
      if (!targetNode) continue;
      deliveries.push({
        edgeId: edge.id,
        producerInstanceId: node.instanceId,
        producerPortId: edge.sourcePortId,
        consumerInstanceId: targetNode.instanceId,
        consumerPortId: edge.targetPortId,
      });
      dirty.add(targetNode.instanceId);
    }
  }
  return deliveries;
}
//#endregion 🔖MediaFlow

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/framework-os-core backbone", () => {
    it("classifies backbone uri kinds", () => {
      expect(backboneKindFromUri("file:///tmp/a.json")).toBe("file");
      expect(backboneKindFromUri("folder:///tmp")).toBe("folder");
      expect(backboneKindFromUri("remote://host:1234/doc-1")).toBe("remote");
      expect(backboneKindFromUri("other://x")).toBe("unknown");
    });

    it("builds and parses backbone uris", () => {
      expect(buildFileBackboneUri("tmp/a.json")).toBe("file:///tmp/a.json");
      expect(buildFolderBackboneUri("tmp")).toBe("folder:///tmp");
      expect(buildRemoteBackboneUri("localhost:1234", "studio-1", "doc-1")).toBe("remote://localhost:1234/studio-1/doc-1");
      expect(parseRemoteBackboneUri("remote://localhost:1234/studio-1/doc-1")).toEqual({ hostPort: "localhost:1234", studioId: "studio-1", documentId: "doc-1" });
      expect(parseRemoteBackboneUri("remote://localhost:1234/doc-1")).toBeNull();
      expect(parseRemoteBackboneUri("file:///tmp/a.json")).toBeNull();
    });

    it("wraps and unwraps document envelopes", () => {
      const envelopeJson = wrapDocumentEnvelope({ nodes: [] }, "doc-1", "file:///tmp/a.json");
      const envelope = JSON.parse(envelopeJson) as { schema: string; id: string; projection: unknown; backbone: unknown };
      expect(envelope.schema).toBe("document/v1");
      expect(envelope.id).toBe("doc-1");
      expect(documentFromEnvelopeJson(envelopeJson)).toEqual({ nodes: [] });
    });

    it("preserves an existing vcs envelope instead of re-wrapping it", () => {
      const existing = { vcs: { edits: [], changes: [], checkpoints: [], alternatives: [], operations: [] }, projection: { a: 1 } };
      const envelopeJson = wrapDocumentEnvelope(existing, "doc-1", "file:///tmp/a.json");
      const envelope = JSON.parse(envelopeJson) as { projection: unknown; vcs: unknown };
      expect(envelope.projection).toEqual({ a: 1 });
    });

    it("applies a snapshot message by overwriting the stored envelope", () => {
      const messageJson = JSON.stringify({ kind: "snapshot", envelopeJson: '{"vcs":{"edits":[]}}' });
      expect(applyBackboneMessage(null, messageJson)).toBe('{"vcs":{"edits":[]}}');
    });

    it("applies an operations message by appending deduped edits into vcs.edits", () => {
      const stored = JSON.stringify({ vcs: { edits: [{ id: "e1" }] } });
      const messageJson = JSON.stringify({
        kind: "operations",
        envelopes: [{ diff: { payload: { id: "e1" } } }, { diff: { payload: { id: "e2" } } }],
      });
      const result = JSON.parse(applyBackboneMessage(stored, messageJson)) as { vcs: { edits: Array<{ id: string }> } };
      expect(result.vcs.edits.map((edit) => edit.id)).toEqual(["e1", "e2"]);
    });

    it("throws when applying an operations message before a snapshot exists", () => {
      const messageJson = JSON.stringify({ kind: "operations", envelopes: [] });
      expect(() => applyBackboneMessage(null, messageJson)).toThrow("cannot append operations before a snapshot exists");
    });

    it("throws on an unsupported backbone message kind", () => {
      const messageJson = JSON.stringify({ kind: "bogus" });
      expect(() => applyBackboneMessage(null, messageJson)).toThrow("unsupported backbone message kind: bogus");
    });

    it("builds sync utilities reflecting the active backbone kind", () => {
      const utilities = buildFrameworkSyncUtilities("folder:///tmp");
      expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
      expect(utilities.find((utility) => utility.id === "framework.sync.folder")?.pressed).toBe(true);
      expect(utilities.find((utility) => utility.id === "framework.sync.file")?.pressed).toBe(false);
    });
  });

  describe("@semio-tech/framework-os-core media flow", () => {
    const mediaNode = (id: string, instanceId: string): OsMediaGraphNode => ({
      id,
      instanceId,
      x: 0,
      y: 0,
      width: 160,
      height: 72,
      inputs: [{ id: `${instanceId}:in`, resourceKind: "2d.drawing", direction: "in" }],
      outputs: [{ id: `${instanceId}:out`, resourceKind: "2d.drawing", direction: "out" }],
    });

    it("plans a single delivery across one dirty edge", () => {
      const graph: OsMediaGraph = {
        schema: "s.media-graph",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in" }],
      };
      const deliveries = planMediaFlow(graph, new Set(["app-1"]));
      expect(deliveries).toEqual([{ edgeId: "edge-1", producerInstanceId: "app-1", producerPortId: "app-1:out", consumerInstanceId: "app-2", consumerPortId: "app-2:in" }]);
    });

    it("plans a chain in topological order when only the root is dirty", () => {
      const graph: OsMediaGraph = {
        schema: "s.media-graph",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2"), mediaNode("node-3", "app-3")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in" },
          { id: "edge-bc", sourceNodeId: "node-2", sourcePortId: "app-2:out", targetNodeId: "node-3", targetPortId: "app-3:in" },
        ],
      };
      const deliveries = planMediaFlow(graph, new Set(["app-1"]));
      expect(deliveries.map((delivery) => delivery.edgeId)).toEqual(["edge-ab", "edge-bc"]);
    });

    it("plans a diamond with one delivery per incoming edge", () => {
      const graph: OsMediaGraph = {
        schema: "s.media-graph",
        nodes: [mediaNode("node-1", "app-a"), mediaNode("node-2", "app-b"), mediaNode("node-3", "app-c"), mediaNode("node-4", "app-d")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-2", targetPortId: "app-b:in" },
          { id: "edge-ac", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-3", targetPortId: "app-c:in" },
          { id: "edge-bd", sourceNodeId: "node-2", sourcePortId: "app-b:out", targetNodeId: "node-4", targetPortId: "app-d:in" },
          { id: "edge-cd", sourceNodeId: "node-3", sourcePortId: "app-c:out", targetNodeId: "node-4", targetPortId: "app-d:in" },
        ],
      };
      const deliveries = planMediaFlow(graph, new Set(["app-a"]));
      const edgeIds = deliveries.map((delivery) => delivery.edgeId);
      expect(edgeIds).toHaveLength(4);
      expect(edgeIds.indexOf("edge-bd")).toBeGreaterThan(edgeIds.indexOf("edge-ab"));
      expect(edgeIds.indexOf("edge-cd")).toBeGreaterThan(edgeIds.indexOf("edge-ac"));
    });

    it("plans nothing when no instance is dirty", () => {
      const graph: OsMediaGraph = {
        schema: "s.media-graph",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in" }],
      };
      expect(planMediaFlow(graph, new Set())).toEqual([]);
    });

    it("plans nothing for a dirty node with no outgoing edges", () => {
      const graph: OsMediaGraph = { schema: "s.media-graph", nodes: [mediaNode("node-1", "app-1")], edges: [] };
      expect(planMediaFlow(graph, new Set(["app-1"]))).toEqual([]);
    });

    // 🔬 Shared fixtures replay (`framework/product/os/core/fixtures/*.json`) — the same files drive
    // the Rust harness's `media_flow_fixtures_match_expected_deliveries` test. Node builtins are
    // imported dynamically inside this vitest-only block so they never reach the browser bundle (this
    // whole `if (import.meta.vitest)` block is stripped from production builds).
    it("matches the Rust plan_media_flow across shared fixtures", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const fixturesDir = join(dirname(fileURLToPath(import.meta.url)), "..", "fixtures");
      const files = readdirSync(fixturesDir).filter((file) => file.endsWith(".json"));
      expect(files.length).toBeGreaterThanOrEqual(5);
      for (const file of files) {
        const fixture = JSON.parse(readFileSync(join(fixturesDir, file), "utf8")) as {
          name: string;
          graph: OsMediaGraph;
          dirtyInstanceIds: readonly string[];
          expectedDeliveries: readonly MediaFlowDelivery[];
        };
        const deliveries = planMediaFlow(fixture.graph, new Set(fixture.dirtyInstanceIds));
        expect(deliveries).toEqual(fixture.expectedDeliveries);
      }
    });
  });
}
//#endregion 🧪Tests
