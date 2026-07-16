// #region Header
/** @emoji 🖥️ `@semio-tech/framework-os-core` — minimal JS surface for OS program registration until full port lands. */
// #endregion Header

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

export function osOutPort(resourceKind: string, id = "out", label = "Out") {
  return { id, label, resourceKind };
}

export function osInPort(resourceKind: string, id: string, label: string, required = false) {
  return { id, label, resourceKind, required };
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

export function parseRemoteBackboneUri(uri: string): { readonly hostPort: string; readonly documentId: string } | null {
  if (!uri.startsWith("remote://")) return null;
  const rest = uri.slice("remote://".length);
  const slash = rest.indexOf("/");
  if (slash <= 0) return null;
  return { hostPort: rest.slice(0, slash), documentId: rest.slice(slash + 1) };
}

export function buildRemoteBackboneUri(hostPort: string, documentId: string): string {
  return `remote://${hostPort}/${documentId}`;
}

export function buildFileBackboneUri(path: string): string {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `file://${normalized}`;
}

export function buildFolderBackboneUri(path: string): string {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `folder://${normalized}`;
}

export async function readBackboneEnvelope(uri: string): Promise<string | null> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) return null;
    const response = await fetch(`http://${remote.hostPort}/documents/${encodeURIComponent(remote.documentId)}/envelope`);
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
    const current = await fetch(`http://${remote.hostPort}/documents/${encodeURIComponent(remote.documentId)}/envelope`);
    const version = current.ok ? Number(((await current.json()) as { version?: number }).version ?? 0) : 0;
    const response = await fetch(`http://${remote.hostPort}/documents/${encodeURIComponent(remote.documentId)}/envelope`, {
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

export type BackboneMessage = { readonly kind: "snapshot"; readonly envelopeJson: string } | { readonly kind: "ops"; readonly envelopes?: readonly BackboneOpEnvelope[] };

/**
 * 🔀 Mirrors `vcs::storage_send` — applies an incoming backbone message on top of a previously
 * stored envelope: a `snapshot` message overwrites, an `ops` message appends into `vcs.edits`
 * deduped by id. This is the canonical implementation; the dev host shim's generated JS
 * (`hostShimSource` in `framework/product/os/dev/script.ts`) hand-ports the same algorithm and
 * must be kept in sync until a build-time inlining step exists.
 */
export function applyBackboneMessage(storedEnvelopeJson: string | null, messageJson: string): string {
  const message = JSON.parse(messageJson) as BackboneMessage;
  if (message.kind === "snapshot") return message.envelopeJson;
  if (message.kind === "ops") {
    if (storedEnvelopeJson == null) throw new Error("cannot append ops before a snapshot exists");
    const envelope = JSON.parse(storedEnvelopeJson) as { vcs?: { edits?: unknown[] } };
    const edits = envelope?.vcs?.edits;
    if (!Array.isArray(edits)) throw new Error("stored envelope missing vcs.edits");
    const seen = new Set(edits.map((edit) => (edit as { id?: unknown })?.id).filter((id): id is string => typeof id === "string"));
    for (const opEnvelope of message.envelopes ?? []) {
      const editJson = opEnvelope?.diff?.payload;
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

export type FrameworkSyncUtilityLeaf = {
  readonly id: string;
  readonly kind: "toggle";
  readonly iconId: string;
  readonly label?: string;
  readonly text?: string;
  readonly title?: string;
  readonly order?: number;
  readonly pressed?: boolean;
  readonly category: "sync";
  readonly controllerId: typeof FRAMEWORK_SYNC_CONTROLLER_ID;
  readonly action: string;
  readonly args?: unknown;
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

//#region 🔖SyncProtocol
/**
 * 🔁 TS mirror of `framework/sync`'s Rust actor protocol (`DocumentActorConfig`/`DocumentActorMsg`/
 * `DocumentEvent`/`DocumentSyncStatus`/`RemoteState`/`PersistenceBinding`) — the wire/postMessage
 * shapes `backbone-worker.ts` speaks, kept camelCase-tag-identical to the Rust side (`#[serde(tag =
 * "kind", rename_all = "camelCase")]`) so a shared JSON fixture suite (`framework/sync/fixtures/`)
 * stays plausible across both runtimes even though this file is a deliberately dumb TS twin (no
 * materialization — it only relays queues, exactly like the Rust actor's `ChannelBackbone` side).
 */
export type OpEnvelope = {
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
};

/** 📨 Client→server hub wire frames — mirrors Rust `HubClientFrame` byte-for-byte. */
export type HubClientFrame =
  | { readonly kind: "hello"; readonly actor: string; readonly token?: string; readonly sinceVersion: number }
  | { readonly kind: "ops"; readonly envelopes: readonly OpEnvelope[] }
  | { readonly kind: "putEnvelope"; readonly version: number; readonly envelope: unknown }
  | { readonly kind: "presence"; readonly peer: DocumentPresencePeer }
  | { readonly kind: "bye" };

/** 📬 Server→client hub wire frames — mirrors Rust `HubServerFrame` byte-for-byte. */
export type HubServerFrame =
  | { readonly kind: "welcome"; readonly version: number; readonly envelope?: unknown; readonly presence: readonly DocumentPresencePeer[]; readonly backlog: readonly OpEnvelope[] }
  | { readonly kind: "ops"; readonly version: number; readonly envelopes: readonly OpEnvelope[]; readonly origin: string }
  | { readonly kind: "snapshotReplaced"; readonly version: number; readonly envelope: unknown }
  | { readonly kind: "presence"; readonly peers: readonly DocumentPresencePeer[] }
  | { readonly kind: "ack"; readonly opId: string; readonly version: number }
  | { readonly kind: "conflict"; readonly message: string }
  | { readonly kind: "error"; readonly message: string };

/** 🗃️ A durable place a document synchronizes with — mirrors Rust `PersistenceBinding`. */
export type PersistenceBinding = { readonly kind: "folder"; readonly path: string } | { readonly kind: "hub"; readonly baseUrl: string; readonly token?: string };

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
  | { readonly kind: "localOps"; readonly envelopes: readonly OpEnvelope[] }
  | { readonly kind: "localSnapshot"; readonly envelopeJson: string }
  | { readonly kind: "presenceHeartbeat"; readonly peer: DocumentPresencePeer }
  | { readonly kind: "externalChanged" }
  | { readonly kind: "detach" };

/** 📶 Connection state of a document's remote (hub) transport — mirrors Rust `RemoteState`. */
export type RemoteState = { readonly kind: "detached" } | { readonly kind: "connecting" } | { readonly kind: "live"; readonly peerCount: number } | { readonly kind: "backoff"; readonly retryInMs: number };

/** 🚦 Sync health snapshot for status badges — mirrors Rust `DocumentSyncStatus`. */
export type DocumentSyncStatus = {
  readonly persisted: boolean;
  readonly pendingOps: number;
  readonly remote: RemoteState;
};

/** ⚠️ A structural sync conflict — loosely typed pending a full mirror of `vcs::StudioConflict`; the
 * shell only needs enough to render a conflict card / offer "fork alternative" vs "take theirs". */
export type SyncConflict = { readonly message?: string } & Record<string, unknown>;

/** 📬 Actor→subscriber events — mirrors Rust `DocumentEvent`. */
export type DocumentEvent =
  | { readonly kind: "remoteOps"; readonly envelopes: readonly OpEnvelope[] }
  | { readonly kind: "snapshotReplaced"; readonly envelopeJson: string }
  | ({ readonly kind: "status" } & DocumentSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly DocumentPresencePeer[] }
  | ({ readonly kind: "conflict" } & SyncConflict);

/** 📤 Main thread → `backbone-worker.ts` messages. */
export type BackboneWorkerRequest = ({ readonly kind: "open" } & DocumentActorConfig) | { readonly kind: "close"; readonly documentId: string } | { readonly kind: "send"; readonly documentId: string; readonly message: DocumentActorMsg };

/** 📥 `backbone-worker.ts` → main thread messages. */
export type BackboneWorkerResponse = { readonly kind: "event"; readonly documentId: string; readonly event: DocumentEvent } | { readonly kind: "ready" };
//#endregion 🔖SyncProtocol

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/framework-os-core program registration", () => {
    it("builds baseline resources and ports", () => {
      expect(osBaselineResource("document", "note-1", "Note 1")).toEqual({ kind: "document", id: "note-1", label: "Note 1" });
      expect(osOutPort("document")).toEqual({ id: "out", label: "Out", resourceKind: "document" });
      expect(osInPort("document", "in", "In", true)).toEqual({ id: "in", label: "In", resourceKind: "document", required: true });
    });

    it("merges program definitions and registers vcs handlers without throwing", () => {
      expect(() => mergeOsProgramDefinition("draw-play", { id: "draw-play" })).not.toThrow();
      let called = false;
      registerAppVcsHandler(() => {
        called = true;
      });
      expect(called).toBe(false);
    });
  });

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
      expect(buildRemoteBackboneUri("localhost:1234", "doc-1")).toBe("remote://localhost:1234/doc-1");
      expect(parseRemoteBackboneUri("remote://localhost:1234/doc-1")).toEqual({ hostPort: "localhost:1234", documentId: "doc-1" });
      expect(parseRemoteBackboneUri("file:///tmp/a.json")).toBeNull();
    });

    it("derives a backbone ref from a uri", () => {
      expect(documentBackboneRef("folder:///tmp")).toEqual({ kind: "folder", uri: "folder:///tmp" });
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

    it("exposes the shared backbone endpoint path", () => {
      expect(BACKBONE_ENDPOINT_PATH).toBe("/semio-backbone");
    });

    it("applies a snapshot message by overwriting the stored envelope", () => {
      const messageJson = JSON.stringify({ kind: "snapshot", envelopeJson: '{"vcs":{"edits":[]}}' });
      expect(applyBackboneMessage(null, messageJson)).toBe('{"vcs":{"edits":[]}}');
    });

    it("applies an ops message by appending deduped edits into vcs.edits", () => {
      const stored = JSON.stringify({ vcs: { edits: [{ id: "e1" }] } });
      const messageJson = JSON.stringify({
        kind: "ops",
        envelopes: [{ diff: { payload: { id: "e1" } } }, { diff: { payload: { id: "e2" } } }],
      });
      const result = JSON.parse(applyBackboneMessage(stored, messageJson)) as { vcs: { edits: Array<{ id: string }> } };
      expect(result.vcs.edits.map((edit) => edit.id)).toEqual(["e1", "e2"]);
    });

    it("throws when applying an ops message before a snapshot exists", () => {
      const messageJson = JSON.stringify({ kind: "ops", envelopes: [] });
      expect(() => applyBackboneMessage(null, messageJson)).toThrow("cannot append ops before a snapshot exists");
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
}
//#endregion 🧪Tests
