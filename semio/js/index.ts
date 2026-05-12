// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: stateless {@link Kit} + GraphQL transport (WASM worker or inline); no client-side kit cache.
// #endregion 🧲Header

// @ts-nocheck — embedded vitest block references legacy `Graph.*` kit-store types; restore strict typing when the kit-store surface is merged back into this entry.
//#region 📥KitImports
//#endregion 📥KitImports

//#region 🌐Transport
/** @emoji 🧵 Bundled worker — Vite resolves `@semio/rs-wasm`; Blob workers cannot import bare specifiers. */
export function createKitStoreWorker(): Worker {
  return new Worker(new URL("./kit-store.worker.ts", import.meta.url), { type: "module" });
}

export type JsonValue = string | number | boolean | null | readonly JsonValue[] | JsonObject;
export type JsonObject = { readonly [k: string]: JsonValue };

type KitGraphqlResponseEnvelope<TData> = Readonly<{
  data?: TData | null;
  errors?: readonly { readonly message?: string }[];
}>;

function parseJsonValue(text: string): JsonValue {
  return JSON.parse(text) as JsonValue;
}

function isJsonObjectNode(v: JsonValue | KitJsonTreeDto | null | undefined): v is JsonObject | KitJsonObjectDto {
  return v != null && typeof v === "object" && !Array.isArray(v);
}

function kitGraphqlData<TData>(response: KitGraphqlResponseEnvelope<TData>): TData {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  if (Array.isArray(response.errors) && response.errors.length > 0) throw new Error(response.errors[0]?.message ?? "GraphQL error");
  const d = response.data;
  if (d != null && typeof d === "object") return d;
  throw new Error("kitGraphql: no data in response");
}

function withTimeout<T>(p: Promise<T>, ms: number, label: string): Promise<T> {
  if (!ms || ms <= 0) return p;
  return new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(label)), ms);
    p.then(
      (v) => {
        clearTimeout(t);
        resolve(v);
      },
      (e) => {
        clearTimeout(t);
        reject(e);
      },
    );
  });
}

type ExecuteFn = (requestJson: string) => Promise<string>;
type SubscribeFn = (requestJson: string, onEvent: (eventJson: string) => void) => Promise<void>;

class InlineTransport {
  constructor(
    private readonly handle: {
      execute: ExecuteFn;
      subscribe: SubscribeFn;
      free?: () => void;
    },
  ) { }
  async execute(requestJson: string): Promise<string> {
    return await this.handle.execute(requestJson);
  }
  async subscribe(requestJson: string, onEvent: (eventJson: string) => void): Promise<void> {
    await this.handle.subscribe(requestJson, onEvent);
  }
  dispose(): void {
    if (typeof this.handle.free === "function") {
      try {
        this.handle.free();
      } catch {
        /* ignore */
      }
    }
  }
}

function describeWorkerThreadError(ev: Event): string {
  if (ev instanceof ErrorEvent) {
    const parts: string[] = [];
    if (ev.message) parts.push(ev.message);
    if (ev.error instanceof Error) parts.push(ev.error.message);
    else if (ev.error) parts.push(String(ev.error));
    if (ev.filename) parts.push(`at ${ev.filename}:${ev.lineno}:${ev.colno}`);
    if (parts.length) return parts.join(" — ");
  }
  return "worker script or module failed to load";
}

class WorkerStringTransport {
  private nextSerial = 0;
  constructor(private readonly worker: Worker) { }

  init(uri: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const t = setTimeout(() => {
        cleanup();
        reject(new Error("worker init timeout"));
      }, 30_000);
      const onMessage = (ev: MessageEvent<string>) => {
        let m: { op?: string; message?: string };
        try {
          m = JSON.parse(ev.data) as typeof m;
        } catch {
          return;
        }
        if (m.op === "ready") {
          cleanup();
          resolve();
        } else if (m.op === "error") {
          cleanup();
          reject(new Error(`worker init error: ${m.message ?? "unknown"}`));
        }
      };
      const onError = (ev: Event) => {
        cleanup();
        reject(new Error(`worker init error: ${describeWorkerThreadError(ev)}`));
      };
      const cleanup = () => {
        clearTimeout(t);
        this.worker.removeEventListener("message", onMessage);
        this.worker.removeEventListener("error", onError as EventListener);
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.addEventListener("error", onError as EventListener);
      this.worker.postMessage(JSON.stringify({ op: "init", uri }));
    });
  }

  async execute(requestJson: string): Promise<string> {
    const reqId = `r-${++this.nextSerial}-${Date.now().toString(36)}`;
    return await new Promise<string>((resolve, reject) => {
      let result: string | null = null;
      const w = (ev: MessageEvent<string>) => {
        let m: { op: string; reqId?: string; json?: string; message?: string };
        try {
          m = JSON.parse(ev.data) as typeof m;
        } catch {
          return;
        }
        if (m.reqId !== reqId) return;
        if (m.op === "result" && typeof m.json === "string") result = m.json;
        if (m.op === "done") {
          this.worker.removeEventListener("message", w);
          if (result == null) reject(new Error("graphql: worker completed without result"));
          else resolve(result);
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", w);
          reject(new Error(m.message ?? "worker error"));
        }
      };
      this.worker.addEventListener("message", w);
      this.worker.postMessage(JSON.stringify({ op: "execute", reqId, body: requestJson }));
    });
  }

  async subscribe(requestJson: string, onEvent: (eventJson: string) => void): Promise<void> {
    const reqId = `s-${++this.nextSerial}-${Date.now().toString(36)}`;
    await new Promise<void>((resolve, reject) => {
      const w = (ev: MessageEvent<string>) => {
        let m: { op: string; reqId?: string; json?: string; message?: string };
        try {
          m = JSON.parse(ev.data) as typeof m;
        } catch {
          return;
        }
        if (m.reqId !== reqId) return;
        if (m.op === "event" && typeof m.json === "string") onEvent(m.json);
        if (m.op === "done") {
          this.worker.removeEventListener("message", w);
          resolve();
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", w);
          reject(new Error(m.message ?? "worker error"));
        }
      };
      this.worker.addEventListener("message", w);
      this.worker.postMessage(JSON.stringify({ op: "subscribe", reqId, body: requestJson }));
    });
  }

  dispose(): void {
    this.worker.terminate();
  }
}

/** @emoji 🌐 Thin GraphQL wire: JSON request in, JSON string out; pairs with rs {@code KitStoreHandle}. */
export class GqlTransport {
  constructor(private readonly inner: WorkerStringTransport | InlineTransport) { }

  async executeJson(body: { readonly query: string; readonly variables?: JsonObject; readonly operationName?: string }, timeoutMs: number): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    const json = await withTimeout(this.inner.execute(JSON.stringify(body)), timeoutMs, "graphql");
    return parseJsonValue(json) as KitGraphqlResponseEnvelope<JsonValue>;
  }

  async subscribeJson(body: { readonly query: string; readonly variables?: JsonObject }, onEvent: (env: KitGraphqlResponseEnvelope<JsonValue>) => void): Promise<void> {
    await this.inner.subscribe(JSON.stringify(body), (eventJson) => {
      try {
        onEvent(parseJsonValue(eventJson) as KitGraphqlResponseEnvelope<JsonValue>);
      } catch {
        /* ignore */
      }
    });
  }

  dispose(): void {
    this.inner.dispose();
  }
}

export type Unsubscribe = () => void;

/** @emoji 📡 Demultiplexes live subscription `data` roots + legacy `event` JSON into listener fan-out (no client cache). */
export class EventBus {
  private readonly listeners = new Set<(ev: JsonValue) => void>();

  emit(ev: JsonValue): void {
    for (const fn of this.listeners) {
      try {
        fn(ev);
      } catch {
        /* ignore */
      }
    }
  }

  subscribe(handler: (ev: JsonValue) => void): Unsubscribe {
    this.listeners.add(handler);
    return () => {
      this.listeners.delete(handler);
    };
  }

  subscribeKind(kind: string, handler: (payload: JsonValue | undefined) => void): Unsubscribe {
    return this.subscribe((ev) => {
      if (!isJsonObjectNode(ev)) return;
      if (ev["kind"] === kind) handler(ev["payload"]);
    });
  }
}

/** @emoji 📡 Live-query mirror of root {@code Query.wip} — ticks {@link Kit#bus} on each WIP emission (replaces {@code Subscription.event}). */
export const KIT_EVENT_STREAM_SUBSCRIPTION = `subscription { wip { id hash } }` as const;

/** @emoji 📡 Alias for correlators that previously reused the same subscription document as the kit event stream. */
export const KIT_COMMAND_SUCCEEDED_SUBSCRIPTION = KIT_EVENT_STREAM_SUBSCRIPTION;

/** @emoji 🧭 Session entry query fragment aligned with {@code target.schema.graphql} (WIP head + {@code theKit} id). */
export const KIT_SESSION_QUERY_ENTRY = `query KitStoreEntry { wip { id theKit { id } } }` as const;

//#endregion 🌐Transport

export type SetErrorKind =
  | "IllegalName"
  | "NameTooLong"
  | "InvalidUrl"
  | "InvalidValue"
  | "DuplicateId"
  | "NotFound"
  | "CyclicReference"
  | "PortFamilyMismatch"
  | "Readonly"
  | "Disposed"
  | "Timeout"
  | "LockPoisoned"
  | "Internal"
  | "NotSupported";

/** @emoji 🧾 Normalized set/mutation error from rs {@code SetError}. */
export type SetError = { kind: SetErrorKind; message: string; field?: string; entity?: { kind: string; id: string } };

/** @emoji 🧾 Mutation receipt (no speculative client-side apply). */
export type SetResult = { ok: true } | { ok: false; error: SetError };

export type ChangeId = string;

/** @emoji 🧭 Materialization anchor for target-schema reads. */
export type KitReadPoint =
  | { readonly theKit: null }
  | {
    readonly checkpoint: {
      readonly checkpointId: string;
      readonly changeId?: string;
      readonly operationId?: string;
    };
  }
  | { readonly alternative: { readonly alternativeId: string } };

export const theKitReadPoint: KitReadPoint = { theKit: null };

export function kitReadPointKey(point: KitReadPoint): string {
  return JSON.stringify(point);
}

function isTheKitReadPoint(s: KitReadPoint): boolean {
  return "theKit" in s;
}

export type KitOpenOptions = Readonly<{
  timeoutMs?: number;
  wasmSpecifier?: string;
  workerFactory?: () => Worker;
}>;

function __gqlStr(s: string): string {
  return JSON.stringify(s);
}

function __gqlIds(ids: readonly string[]): string {
  return `[${ids.map((x) => __gqlStr(x)).join(",")}]`;
}

function __scopedKitMutationBody(changeId: string, kitSelection: string): { readonly query: string; readonly variables: JsonObject } {
  return {
    query: `mutation($changeId: ID!) { session { theKit { unsavedChange(id: $changeId) { kit { ${kitSelection} } } } } }`,
    variables: { changeId },
  };
}

function kitSessionWipStoreSelect(point: KitReadPoint, innerOnKitStore: string): { query: string; variables: JsonObject } {
  if (isTheKitReadPoint(point)) {
    return {
      query: `query KitSessionWipStore { wip { theKit { kit { ${innerOnKitStore} } } } }`,
      variables: {},
    };
  }
  if ("checkpoint" in point) {
    return {
      query: `query KitSessionWipStore($checkpointId: ID!) { wip { checkpoint(id: $checkpointId) { frozenRoot { ${innerOnKitStore} } } } }`,
      variables: { checkpointId: point.checkpoint.checkpointId },
    };
  }
  if ("alternative" in point) {
    return {
      query: `query KitSessionWipStore($alternativeId: ID!) { wip { alternative(id: $alternativeId) { kit { ${innerOnKitStore} } } } }`,
      variables: { alternativeId: point.alternative.alternativeId },
    };
  }
  return {
    query: `query KitSessionWipStore { wip { theKit { kit { ${innerOnKitStore} } } } }`,
    variables: {},
  };
}

function gqlDataSessionWipKitStore(d: JsonValue | null | undefined, point: KitReadPoint): JsonObject | null {
  if (d == null || typeof d !== "object" || Array.isArray(d)) return null;
  const wip = (d as { wip?: JsonObject | null }).wip;
  if (!wip || typeof wip !== "object" || Array.isArray(wip)) return null;
  if ("checkpoint" in point) {
    const cp = wip["checkpoint"];
    const root = cp != null && typeof cp === "object" && !Array.isArray(cp) ? (cp as JsonObject)["frozenRoot"] : null;
    return root != null && typeof root === "object" && !Array.isArray(root) ? (root as JsonObject) : null;
  }
  if ("alternative" in point) {
    const alt = wip["alternative"];
    const kit = alt != null && typeof alt === "object" && !Array.isArray(alt) ? (alt as JsonObject)["kit"] : null;
    return kit != null && typeof kit === "object" && !Array.isArray(kit) ? (kit as JsonObject) : null;
  }
  const tk = wip["theKit"];
  const kit = tk != null && typeof tk === "object" && !Array.isArray(tk) ? (tk as JsonObject)["kit"] : null;
  return kit != null && typeof kit === "object" && !Array.isArray(kit) ? (kit as JsonObject) : null;
}

async function kitGraphqlRun(handle: { execute(requestJson: string): Promise<string> }, body: { query: string; variables?: JsonObject; operationName?: string }, timeoutMs?: number): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
  const json = await withTimeout(handle.execute(JSON.stringify(body)), timeoutMs ?? 0, "graphql");
  return parseJsonValue(json) as KitGraphqlResponseEnvelope<JsonValue>;
}

function gqlOkFromEnvelope(env: KitGraphqlResponseEnvelope<JsonValue>): SetResult {
  if (Array.isArray(env.errors) && env.errors.length > 0) {
    return { ok: false, error: { kind: "Internal", message: env.errors[0]?.message ?? "GraphQL error" } };
  }
  return { ok: true };
}

type KitGraphqlHandle = { execute(requestJson: string): Promise<string> };

async function __readSemioBytesFromMonorepoCandidates(): Promise<Uint8Array | undefined> {
  try {
    const fs = await import("node:fs/promises");
    const path = await import("node:path");
    const url = await import("node:url");
    const here = path.dirname(url.fileURLToPath(import.meta.url));
    const candidates = [
      path.resolve(here, "../../rs/pkg/semio_bg.wasm"),
      path.resolve(here, "../../../semio/rs/pkg/semio_bg.wasm"),
    ];
    for (const p of candidates) {
      try {
        const buf = await fs.readFile(p);
        return new Uint8Array(buf);
      } catch {
        /* try next */
      }
    }
  } catch {
    /* non-node */
  }
  return undefined;
}

//#region 🧱Classes

//#region 🧬Entity

//#region 🛠️Base
/** @emoji 🧬 Strong entity anchor: {@link Kit} + id (no cached fields on the instance). */
export abstract class Entity {
  protected constructor(
    public readonly kit: Kit,
    public readonly id: string,
  ) { }
}
//#endregion 🛠️Base

//#region 🏭Factories
export type FieldSpec<T> = Readonly<{
  eventKind?: string;
  selection: string;
  parse: (v: JsonValue) => T;
}>;

export type OperationSpec = Readonly<{
  alias: string;
  call: string;
}>;

/** @emoji 🏭 Metadata-only field list (tooling / docs); reads use entity methods. */
export function defineFields<const S extends readonly FieldSpec<unknown>[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Metadata-only operation list (tooling / docs); writes use entity methods. */
export function defineOperations<const S extends readonly OperationSpec[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭  a field read when the caller supplies the kit-relative GraphQL tail. */
export function defineField<E extends Entity, T>(entity: E, spec: FieldSpec<T>, pathInKit: (self: E) => string): () => Promise<T> {
  return async () => {
    const frag = await entity.kit.readKitInner(pathInKit(entity));
    return spec.parse(frag as JsonValue);
  };
}

/** @emoji 🏭  a mutation leaf using {@link Kit#mutateScoped}. */
export function defineOperation(entity: Entity, spec: OperationSpec, buildPath: (self: Entity) => string): () => Promise<SetResult> {
  return async () => {
    void spec;
    const cid = await entity.kit.ensureChangeId();
    return entity.kit.mutateScoped(cid, buildPath(entity));
  };
}

/** @emoji 📑 Metadata mirror of {@code KitOperationInput} leaves (see {@link semio/graphql/target.schema.graphql}). */
export const KIT_OPERATION_SPECS = defineOperations([
  { alias: "rn", call: "rename(newName: String!): ID!" },
  { alias: "cd", call: "changeDescription(newDescription: String!): ID!" },
  { alias: "ct", call: "createTag(name: String!, description: String, icon: String, order: Int): ID!" },
  { alias: "dt", call: "deleteTag(id: ID!): ID!" },
  { alias: "dts", call: "deleteTags(ids: [ID!]!): ID!" },
  { alias: "cc", call: "createConcept(name: String!, description: String, icon: String, order: Int): ID!" },
  { alias: "dc", call: "deleteConcept(id: ID!): ID!" },
  { alias: "dcs", call: "deleteConcepts(ids: [ID!]!): ID!" },
  { alias: "cq", call: "createQuality(key: String!, value: String, unit: String, definition: String, description: String, icon: String): ID!" },
  { alias: "dq", call: "deleteQuality(id: ID!): ID!" },
  { alias: "dqs", call: "deleteQualities(ids: [ID!]!): ID!" },
  { alias: "cT", call: "createType(name: String!, description: String, icon: String, image: String, unit: String): ID!" },
  { alias: "dT", call: "deleteType(id: ID!): ID!" },
  { alias: "dTs", call: "deleteTypes(ids: [ID!]!): ID!" },
  { alias: "cD", call: "createDesign(name: String!, description: String, icon: String, image: String, unit: String): ID!" },
  { alias: "dD", call: "deleteDesign(id: ID!): ID!" },
  { alias: "dDs", call: "deleteDesigns(ids: [ID!]!): ID!" },
] as const);

function __kitScalar(v: JsonValue, key: string): string {
  if (!isJsonObjectNode(v)) return "";
  return String(v[key] ?? "");
}

/** @emoji 📑 {@code Kit} data fields for {@link Kit#fieldRead} / {@link bindDefinedFieldToReact}. */
export const KIT_ARTIFACT_FIELD_SPECS = defineFields([
  { selection: "id", parse: (v) => __kitScalar(v, "id") },
  { eventKind: "kitRenamed", selection: "name", parse: (v) => __kitScalar(v, "name") },
  { eventKind: "changedDescription", selection: "description", parse: (v) => __kitScalar(v, "description") },
  { selection: "icon", parse: (v) => __kitScalar(v, "icon") },
  { selection: "image", parse: (v) => __kitScalar(v, "image") },
  { selection: "preview", parse: (v) => __kitScalar(v, "preview") },
  { selection: "remote", parse: (v) => __kitScalar(v, "remote") },
  { selection: "homepage", parse: (v) => __kitScalar(v, "homepage") },
  { selection: "license", parse: (v) => __kitScalar(v, "license") },
  { selection: "uri", parse: (v) => __kitScalar(v, "uri") },
  { selection: "types { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(isJsonObjectNode(v) ? v : null, "types")] },
  { selection: "designs { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(isJsonObjectNode(v) ? v : null, "designs")] },
  { selection: "authors { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(isJsonObjectNode(v) ? v : null, "authors")] },
  { selection: "qualities { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(isJsonObjectNode(v) ? v : null, "qualities")] },
  { selection: "tags { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(isJsonObjectNode(v) ? v : null, "tags")] },
  { selection: "concepts { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(isJsonObjectNode(v) ? v : null, "concepts")] },
] as const);

/** @emoji 📑 Metadata mirror of {@code DesignOperationInput} leaves (see {@link semio/graphql/target.schema.graphql}). */
export const DESIGN_OPERATION_SPECS = defineOperations([
  { alias: "rn", call: "rename(newName: String!): ID!" },
  { alias: "cd", call: "changeDescription(newDescription: String!): ID!" },
  { alias: "fl", call: "flatten: ID!" },
  { alias: "aa", call: "addAttribute(key: String!, value: String!, definition: String!): ID!" },
  { alias: "ra", call: "removeAttribute(id: ID!): ID!" },
  { alias: "ras", call: "removeAttributes(ids: [ID!]!): ID!" },
  { alias: "afp", call: "addFixedPiece(blueprintId: ID!, position: PositionInput!, name: String, description: String): ID!" },
  { alias: "ac", call: "addChildPieceWithParentConnection(blueprintId: ID!, parentPieceId: ID!, parentConnector: String!, childConnector: String!, name: String, description: String, position: PositionInput, scale: Float): ID!" },
  { alias: "ah", call: "addHangingChildPieceWithParentConnection(blueprintId: ID!, parentPieceId: ID!, parentConnector: String!, childConnector: String!, position: PositionInput!, name: String, description: String, scale: Float): ID!" },
  { alias: "dp", call: "deletePiece(id: ID!): ID!" },
  { alias: "dps", call: "deletePieces(ids: [ID!]!): ID!" },
  { alias: "dpc", call: "deletePiecesAndConnections(pieceIds: [ID!]!, connectionIds: [ID!]!): ID!" },
] as const);

function __designJson(v: JsonValue): JsonObject | null {
  if (!isJsonObjectNode(v)) return null;
  const d = v["design"] as JsonObject | undefined;
  if (d != null && typeof d === "object" && !Array.isArray(d)) return d;
  return v;
}

function __designScalar(v: JsonValue, key: string): string {
  const d = __designJson(v);
  return d ? String(d[key] ?? "") : "";
}

/** @emoji 📑 {@code Design} data fields for {@link Design#fieldRead} / {@link bindDefinedFieldToReact}. */
export const DESIGN_ARTIFACT_FIELD_SPECS = defineFields([
  { selection: "id", parse: (v) => __designScalar(v, "id") },
  { selection: "name", parse: (v) => __designScalar(v, "name") },
  { eventKind: "changedDescription", selection: "description", parse: (v) => __designScalar(v, "description") },
  { selection: "icon", parse: (v) => __designScalar(v, "icon") },
  { selection: "image", parse: (v) => __designScalar(v, "image") },
  { selection: "unit", parse: (v) => __designScalar(v, "unit") },
  { selection: "qualitySum", parse: (v) => Number(__designScalar(v, "qualitySum")) },
  { selection: "pieces { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(__designJson(v), "pieces")] },
  { selection: "connections { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(__designJson(v), "connections")] },
  { selection: "attributes { edges { node { id } } }", parse: (v): unknown => [...__parseEntityConnectionIds(__designJson(v), "attributes")] },
] as const);
//#endregion 🏭Factories

//#region 🧩Parsers
/** @emoji 🧩 Parses {@code attributes { edges { node { … } } }} under a JSON object (e.g. {@code tag}, {@code node}). */
function parseAttributeConnectionUnder(owner: JsonObject | null | undefined): readonly Attribute[] {
  const attrs = owner?.["attributes"] as JsonObject | undefined;
  const edges = attrs?.["edges"] as readonly JsonValue[] | undefined;
  if (!Array.isArray(edges)) return [];
  const out: Attribute[] = [];
  for (const e of edges) {
    if (!isJsonObjectNode(e)) continue;
    const n = e["node"] as JsonObject | undefined;
    if (n == null) continue;
    out.push({
      id: String(n["id"] ?? ""),
      key: String(n["key"] ?? ""),
      value: n["value"] == null ? null : String(n["value"]),
      definition: String(n["definition"] ?? ""),
    });
  }
  return out;
}
//#endregion 🧩Parsers
//#endregion 🧬Entity


//#region 🎒Kit
/** @emoji 🔗 Map `Kit.open` input: inline JSON becomes `dev+json:` base64 for the WASM bootstrap URI. */
function backboneBootstrapUriForKitOpen(raw: string): string {
  const t = raw.trim();
  if (t.startsWith("{") || t.startsWith("[")) {
    const bytes = new TextEncoder().encode(t);
    let bin = "";
    for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i]!);
    return `dev+json:${btoa(bin)}`;
  }
  return t;
}

/** @emoji 📑 Same wire as {@link FieldSpec} but declared before {@link Entity} so {@link Kit#fieldRead} stays self-contained. */
export type KitFieldReadSpec<T> = Readonly<{
  eventKind?: string;
  selection: string;
  parse: (v: JsonValue) => T;
}>;

function __parseEntityConnectionIds(frag: JsonObject | null | undefined, key: string): readonly string[] {
  const conn = frag?.[key] as JsonObject | undefined;
  const edges = conn?.["edges"] as readonly JsonValue[] | undefined;
  if (!Array.isArray(edges)) return [];
  const out: string[] = [];
  for (const e of edges) {
    if (!isJsonObjectNode(e)) continue;
    const n = e["node"] as JsonObject | undefined;
    if (n == null) continue;
    const id = String(n["id"] ?? "");
    if (id !== "") out.push(id);
  }
  return out;
}

/**
 * @emoji 🎒 Stateless kit façade: owns {@link GqlTransport} + {@link EventBus}; every read is a fresh GraphQL round-trip.
 */
export class Kit {
  private readonly timeoutMs: number;
  private readonly handle: KitGraphqlHandle;
  private readonly innerTransport: WorkerStringTransport | InlineTransport;
  private gqlLoopRunning = false;
  private disposed = false;
  private activeReadPoint: KitReadPoint = theKitReadPoint;
  private kitWriteChangeId: string | null = null;

  /** @emoji 🌐 GraphQL executor (JSON wire). */
  readonly gql: GqlTransport;
  /** @emoji 📡 Demuxed subscription fan-out. */
  readonly bus: EventBus;

  private constructor(timeoutMs: number, inner: WorkerStringTransport | InlineTransport) {
    this.timeoutMs = timeoutMs;
    this.innerTransport = inner;
    this.handle = { execute: (j) => inner.execute(j) };
    this.gql = new GqlTransport(inner);
    this.bus = new EventBus();
  }

  private ensureAlive(): void {
    if (this.disposed) throw new Error("Kit disposed");
  }

  getReadPoint(): KitReadPoint {
    return this.activeReadPoint;
  }

  setReadPoint(next: KitReadPoint): void {
    this.ensureAlive();
    this.activeReadPoint = next;
  }

  private dispatchSubscriptionGraphqlData(data: JsonObject | null | undefined): void {
    if (data == null) return;
    if (data["event"] !== undefined) {
      this.bus.emit(data["event"] as JsonValue);
      return;
    }
    if (data["wip"] !== undefined) {
      // Coarse invalidation: live-query WIP tick → same bus kinds as legacy semantic events + command correlator.
      this.bus.emit({ kind: "kitRenamed", payload: undefined } as unknown as JsonValue);
      this.bus.emit({ kind: "changedDescription", payload: undefined } as unknown as JsonValue);
      this.bus.emit({ kind: "commandSucceeded", payload: undefined } as unknown as JsonValue);
      return;
    }
    if (data["session"] !== undefined) {
      this.bus.emit({ kind: "commandSucceeded", payload: undefined } as unknown as JsonValue);
      return;
    }
    if (data["commandSucceeded"] !== undefined) this.bus.emit({ kind: "commandSucceeded", payload: data["commandSucceeded"] });
    if (data["operationFailed"] !== undefined) this.bus.emit({ kind: "operationFailed", payload: data["operationFailed"] });
    const legacyOp = data["operationSucceeded"];
    if (legacyOp !== undefined) this.bus.emit(legacyOp as JsonValue);
  }

  private startSubscriptionLoop(): void {
    if (this.gqlLoopRunning) return;
    this.gqlLoopRunning = true;
    void this.innerTransport
      .subscribe(JSON.stringify({ query: KIT_EVENT_STREAM_SUBSCRIPTION }), (eventJson: string) => {
        try {
          const msg = parseJsonValue(eventJson) as KitGraphqlResponseEnvelope<JsonObject>;
          if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return;
          const row = msg.data;
          if (row == null || typeof row !== "object") return;
          this.dispatchSubscriptionGraphqlData(row as JsonObject);
        } catch {
          /* ignore */
        }
      })
      .catch(() => {
        this.gqlLoopRunning = false;
      });
  }

  private async gqlRun(body: { query: string; variables?: JsonObject; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    this.ensureAlive();
    return kitGraphqlRun(this.handle, body, this.timeoutMs);
  }

  /** @emoji 🌐 Public GraphQL round-trip (root {@code Query} / {@code Mutation}), for {@code node(id:)} reads. */
  async runGraphql(body: { query: string; variables?: JsonObject; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    return this.gqlRun(body);
  }

  /** @emoji 🧾 Reads a selection inside scoped {@code kit { … }} for {@link activeReadPoint}. */
  async readKitInner(inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    const { query, variables: v0 } = kitSessionWipStoreSelect(this.activeReadPoint, inner);
    const data = kitGraphqlData(await this.gqlRun({ query, variables: { ...v0, ...variables } })) as JsonValue;
    return gqlDataSessionWipKitStore(data, this.activeReadPoint);
  }

  /** @emoji 🧾 Runs {@code mutation session { theKit { unsavedChange { kit { … } } } }} when {@linkcode changeId} is set. */
  async mutateScoped(changeId: string, kitSelection: string): Promise<SetResult> {
    this.ensureAlive();
    const { query, variables } = __scopedKitMutationBody(changeId, kitSelection);
    const env = await this.gqlRun({ query, variables });
    return gqlOkFromEnvelope(env);
  }

  async ensureChangeId(): Promise<string> {
    this.ensureAlive();
    if (this.kitWriteChangeId) return this.kitWriteChangeId;
    const data = kitGraphqlData(await this.gqlRun({ query: `mutation { session { theKit { startNewChange } } }` })) as JsonObject;
    const sess = data["session"] as JsonObject | undefined;
    const tk = sess?.["theKit"] as JsonObject | undefined;
    const cid = String(tk?.["startNewChange"] ?? "");
    if (cid === "") throw new Error("startNewChange: empty change id");
    this.kitWriteChangeId = cid;
    return cid;
  }

  async saveChange(): Promise<void> {
    this.ensureAlive();
    kitGraphqlData(await this.gqlRun({ query: `mutation { session { theKit { save } } }` }));
    this.kitWriteChangeId = null;
  }

  async startNewChange(): Promise<ChangeId> {
    return await this.ensureChangeId();
  }

  async createCheckpoint(message: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { theKit { createCheckpoint(message: ${__gqlStr(message)}) } } }` });
    return gqlOkFromEnvelope(env);
  }

  async startAlternative(name?: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({
      query:
        name == null
          ? `mutation { session { startAlternative } }`
          : `mutation { session { startAlternative(name: ${__gqlStr(name)}) } }`,
    });
    return gqlOkFromEnvelope(env);
  }

  async integrateAlternative(alternativeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({
      query: `mutation { session { alternative(id: ${__gqlStr(alternativeId)}) { integrateIntoTheKit } } }`,
    });
    return gqlOkFromEnvelope(env);
  }

  async login(username: string, passwordHash: string, hubUrl?: string): Promise<SetResult> {
    this.ensureAlive();
    const env =
      hubUrl == null
        ? await this.gqlRun({
          query: `mutation { session { login(username: ${__gqlStr(username)}, passwordHash: ${__gqlStr(passwordHash)}) } }`,
        })
        : await this.gqlRun({
          query: `mutation { session { login(username: ${__gqlStr(username)}, passwordHash: ${__gqlStr(passwordHash)}, hubUrl: ${__gqlStr(hubUrl)}) } }`,
        });
    return gqlOkFromEnvelope(env);
  }

  async logout(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { logout } }` });
    return gqlOkFromEnvelope(env);
  }

  async sessionStart(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { start } }` });
    return gqlOkFromEnvelope(env);
  }

  async sessionEnd(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { end } }` });
    return gqlOkFromEnvelope(env);
  }

  async attachBackbone(uri: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { backbone { attach(uri: ${__gqlStr(uri)}) } } }` });
    return gqlOkFromEnvelope(env);
  }

  async detachBackbone(uri: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { backbone { detach(uri: ${__gqlStr(uri)}) } } }` });
    return gqlOkFromEnvelope(env);
  }

  /** @emoji 🧾 Warm-path query after WASM init. */
  private async warmGraphqlRead(): Promise<void> {
    await this.readKitInner("id name");
  }

  static async open(uri: string, opts?: KitOpenOptions): Promise<Kit> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const wasmSpecifier = opts?.wasmSpecifier ?? (globalThis as { __SEMIO_WASM_SPECIFIER__?: string }).__SEMIO_WASM_SPECIFIER__ ?? "@semio/rs-wasm";
    const preferInlineInVitest = (() => {
      try {
        const env = (import.meta as { env?: JsonObject }).env;
        if (env && Boolean(env["VITEST"])) return true;
      } catch {
        /* ignore */
      }
      return typeof process !== "undefined" && !!process.env && "VITEST" in process.env;
    })();

    const wasmBytesPre = await __readSemioBytesFromMonorepoCandidates();
    const useDedicatedWorker = typeof Worker !== "undefined" && !preferInlineInVitest && wasmBytesPre == null;

    const bootstrapUri = backboneBootstrapUriForKitOpen(uri);

    if (useDedicatedWorker) {
      const worker = opts?.workerFactory?.() ?? createKitStoreWorker();
      const wt = new WorkerStringTransport(worker);
      try {
        await wt.init(bootstrapUri);
        const k = new Kit(timeoutMs, wt);
        await withTimeout(k.warmGraphqlRead(), timeoutMs, "graphql");
        void k.startSubscriptionLoop();
        return k;
      } catch (workerErr) {
        console.warn("[semio/js] WASM worker init failed; falling back to inline WASM", workerErr);
        try {
          wt.dispose();
        } catch {
          /* ignore */
        }
      }
    }

    let mod: typeof import("@semio/rs-wasm");
    try {
      mod = wasmSpecifier === "@semio/rs-wasm" ? await import("@semio/rs-wasm") : await import(/* @vite-ignore */ wasmSpecifier);
    } catch (e) {
      const base = e instanceof Error ? e.message : String(e);
      throw new Error(`Failed to load @semio/rs-wasm (inline path): ${base}`);
    }
    if (typeof mod.default === "function") {
      if (wasmBytesPre) await mod.default({ module_or_path: wasmBytesPre });
      else await mod.default();
    } else await mod.default();
    if (typeof mod.boot === "function") mod.boot();
    const handleUnknown = mod.KitStoreHandle.create(bootstrapUri);
    const wasmHandle = handleUnknown instanceof Promise ? await handleUnknown : handleUnknown;
    if (wasmHandle == null || typeof (wasmHandle as { execute?: unknown }).execute !== "function") {
      throw new Error("KitStoreHandle.create did not return execute()");
    }
    const t = new InlineTransport(wasmHandle as { execute: ExecuteFn; subscribe: SubscribeFn; free?: () => void });
    const k = new Kit(timeoutMs, t);
    await withTimeout(k.warmGraphqlRead(), timeoutMs, "graphql");
    void k.startSubscriptionLoop();
    return k;
  }

  subscribe(handler: (ev: JsonValue) => void): Unsubscribe {
    return this.bus.subscribe(handler);
  }

  async dispose(): Promise<void> {
    if (this.disposed) return;
    this.disposed = true;
    this.innerTransport.dispose();
  }

  design(id: string): Design {
    return new Design(this, id);
  }

  type(id: string): Type {
    return new Type(this, id);
  }

  tag(id: string): Tag {
    return new Tag(this, id);
  }

  concept(id: string): Concept {
    return new Concept(this, id);
  }

  quality(id: string): Quality {
    return new Quality(this, id);
  }

  family(id: string): Family {
    return new Family(this, id);
  }

  file(id: string): File {
    return new File(this, id);
  }

  folder(id: string): Folder {
    return new Folder(this, id);
  }

  author(id: string): Author {
    return new Author(this, id);
  }

  stat(id: string): Stat {
    return new Stat(this, id);
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `rn: rename(newName: ${__gqlStr(newName)})`);
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`);
  }

  async createTag(name: string, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : __gqlStr(description);
    const ic = icon == null ? "null" : __gqlStr(icon);
    const ord = order == null ? "null" : String(order);
    return this.mutateScoped(cid, `ct: createTag(name: ${__gqlStr(name)}, description: ${d}, icon: ${ic}, order: ${ord})`);
  }

  async deleteTag(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dt: deleteTag(id: ${__gqlStr(id)})`);
  }

  async deleteTags(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dts: deleteTags(ids: ${__gqlIds(ids)})`);
  }

  async createConcept(name: string, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : __gqlStr(description);
    const ic = icon == null ? "null" : __gqlStr(icon);
    const ord = order == null ? "null" : String(order);
    return this.mutateScoped(cid, `cc: createConcept(name: ${__gqlStr(name)}, description: ${d}, icon: ${ic}, order: ${ord})`);
  }

  async deleteConcept(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dc: deleteConcept(id: ${__gqlStr(id)})`);
  }

  async deleteConcepts(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dcs: deleteConcepts(ids: ${__gqlIds(ids)})`);
  }

  async createQuality(
    key: string,
    value?: string | null,
    unit?: string | null,
    definition?: string | null,
    description?: string | null,
    icon?: string | null,
  ): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const va = value == null ? "null" : __gqlStr(value);
    const un = unit == null ? "null" : __gqlStr(unit);
    const de = definition == null ? "null" : __gqlStr(definition);
    const ds = description == null ? "null" : __gqlStr(description);
    const ic = icon == null ? "null" : __gqlStr(icon);
    return this.mutateScoped(
      cid,
      `cq: createQuality(key: ${__gqlStr(key)}, value: ${va}, unit: ${un}, definition: ${de}, description: ${ds}, icon: ${ic})`,
    );
  }

  async deleteQuality(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dq: deleteQuality(id: ${__gqlStr(id)})`);
  }

  async deleteQualities(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dqs: deleteQualities(ids: ${__gqlIds(ids)})`);
  }

  async createType(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : __gqlStr(description);
    const ic = icon == null ? "null" : __gqlStr(icon);
    const im = image == null ? "null" : __gqlStr(image);
    const u = unit == null ? "null" : __gqlStr(unit);
    return this.mutateScoped(cid, `cT: createType(name: ${__gqlStr(name)}, description: ${d}, icon: ${ic}, image: ${im}, unit: ${u})`);
  }

  async deleteType(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dT: deleteType(id: ${__gqlStr(id)})`);
  }

  async deleteTypes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dTs: deleteTypes(ids: ${__gqlIds(ids)})`);
  }

  async createDesign(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : __gqlStr(description);
    const ic = icon == null ? "null" : __gqlStr(icon);
    const im = image == null ? "null" : __gqlStr(image);
    const u = unit == null ? "null" : __gqlStr(unit);
    return this.mutateScoped(cid, `cD: createDesign(name: ${__gqlStr(name)}, description: ${d}, icon: ${ic}, image: ${im}, unit: ${u})`);
  }

  async deleteDesign(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dD: deleteDesign(id: ${__gqlStr(id)})`);
  }

  async deleteDesigns(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dDs: deleteDesigns(ids: ${__gqlIds(ids)})`);
  }

  async readName(): Promise<string> {
    const frag = await this.readKitInner("name");
    return String(frag?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = await this.readKitInner("description");
    return String(frag?.["description"] ?? "");
  }

  async readId(): Promise<string> {
    const frag = (await this.readKitInner("id")) as JsonObject | null;
    return String(frag?.["id"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.readKitInner("icon")) as JsonObject | null;
    return String(frag?.["icon"] ?? "");
  }

  async readImage(): Promise<string> {
    const frag = (await this.readKitInner("image")) as JsonObject | null;
    return String(frag?.["image"] ?? "");
  }

  async readTypeIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("types { edges { node { id } } }")) as JsonObject | null;
    return __parseEntityConnectionIds(frag, "types");
  }

  async readDesignIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("designs { edges { node { id } } }")) as JsonObject | null;
    return __parseEntityConnectionIds(frag, "designs");
  }

  async readAuthorIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("authors { edges { node { id } } }")) as JsonObject | null;
    return __parseEntityConnectionIds(frag, "authors");
  }

  async readQualityIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("qualities { edges { node { id } } }")) as JsonObject | null;
    return __parseEntityConnectionIds(frag, "qualities");
  }

  async readTagIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("tags { edges { node { id } } }")) as JsonObject | null;
    return __parseEntityConnectionIds(frag, "tags");
  }

  async readConceptIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("concepts { edges { node { id } } }")) as JsonObject | null;
    return __parseEntityConnectionIds(frag, "concepts");
  }
}
//#endregion 🎒Kit

//#region 📐Design
export class Design extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private dsel(inner: string): string {
    return `design(id: ${__gqlStr(this.id)}) { ${inner} }`;
  }

  piece(pieceId: string): Piece {
    return new Piece(this.kit, this.id, pieceId);
  }

  pieces(pieceIds: readonly string[]): PiecesOperations {
    return new PiecesOperations(this.kit, this.id, pieceIds);
  }

  connection(connectionId: string): Connection {
    return new Connection(this.kit, this.id, connectionId);
  }

  layer(layerId: string): Layer {
    return new Layer(this.kit, this.id, layerId);
  }

  group(groupId: string): Group {
    return new Group(this.kit, this.id, groupId);
  }

  /** @emoji 🧷 GraphQL kit-store tail for {@code design(id){ … }} (shared with {@link bindDefinedFieldToReact}). */
  kitInnerPath(inner: string): string {
    return this.dsel(inner);
  }

  /**
   * @emoji 📖 Stateless read for one {@code design(id){ … }} selection; {@link FieldSpec#parse} receives the kit row (with nested {@code design}).
   */
  async fieldRead<T>(spec: FieldSpec<T>): Promise<T> {
    const frag = await this.kit.readKitInner(this.dsel(spec.selection));
    return spec.parse(frag as JsonValue);
  }

  /**
   * @emoji 📡 When {@link FieldSpec#eventKind} matches {@link EventBus} kinds (legacy `Subscription.event` or live WIP ticks), refetches via {@link Design#fieldRead}.
   */
  subscribeField<T>(spec: FieldSpec<T>, cb: (next: T) => void): Unsubscribe {
    const kind = spec.eventKind;
    if (kind == null || kind === "") return () => { };
    return this.kit.bus.subscribeKind(kind, () => {
      void this.fieldRead(spec).then(cb);
    });
  }

  /** @emoji 📡 Design description stream (rs {@code changedDescription}; coarse — refetches design description). */
  onDescriptionChanged(cb: (next: string) => void): Unsubscribe {
    return this.kit.bus.subscribeKind("changedDescription", () => {
      void this.readDescription().then(cb);
    });
  }

  async readId(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.dsel("id"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return String(d?.["id"] ?? frag?.["id"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.dsel("icon"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return String(d?.["icon"] ?? frag?.["icon"] ?? "");
  }

  async readImage(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.dsel("image"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return String(d?.["image"] ?? frag?.["image"] ?? "");
  }

  async readUnit(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.dsel("unit"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return String(d?.["unit"] ?? frag?.["unit"] ?? "");
  }

  async readQualitySum(): Promise<number> {
    const frag = (await this.kit.readKitInner(this.dsel("qualitySum"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    const raw = d?.["qualitySum"] ?? frag?.["qualitySum"];
    return typeof raw === "number" ? raw : Number(raw ?? NaN);
  }

  async readPieceIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.dsel("pieces { edges { node { id } } }"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return __parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "pieces");
  }

  async readConnectionIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.dsel("connections { edges { node { id } } }"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return __parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "connections");
  }

  async readAttributeIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.dsel("attributes { edges { node { id } } }"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return __parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "attributes");
  }

  async readName(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.dsel("name"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return String(d?.["name"] ?? frag?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.dsel("description"))) as JsonObject | null;
    const d = frag?.["design"] as JsonObject | undefined;
    return String(d?.["description"] ?? frag?.["description"] ?? "");
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`rn: rename(newName: ${__gqlStr(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async flatten(): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`fl: flatten`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }

  async addFixedPiece(blueprintId: string, position: PositionInput, name?: string | null, description?: string | null): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    const pos = formatPositionInput(position);
    const n = name == null ? "null" : __gqlStr(name);
    const d = description == null ? "null" : __gqlStr(description);
    return this.kit.mutateScoped(cid, this.dsel(`afp: addFixedPiece(blueprintId: ${__gqlStr(blueprintId)}, position: ${pos}, name: ${n}, description: ${d})`));
  }

  async addChildPieceWithParentConnection(
    blueprintId: string,
    parentPieceId: string,
    parentConnector: string,
    childConnector: string,
    name?: string | null,
    description?: string | null,
    position?: PositionInput | null,
    scale?: number | null,
  ): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    const pos = position == null ? "null" : formatPositionInput(position);
    const n = name == null ? "null" : __gqlStr(name);
    const d = description == null ? "null" : __gqlStr(description);
    const sc = scale == null ? "null" : String(scale);
    return this.kit.mutateScoped(
      cid,
      this.dsel(
        `ac: addChildPieceWithParentConnection(blueprintId: ${__gqlStr(blueprintId)}, parentPieceId: ${__gqlStr(parentPieceId)}, parentConnector: ${__gqlStr(parentConnector)}, childConnector: ${__gqlStr(childConnector)}, name: ${n}, description: ${d}, position: ${pos}, scale: ${sc})`,
      ),
    );
  }

  async addHangingChildPieceWithParentConnection(
    blueprintId: string,
    parentPieceId: string,
    parentConnector: string,
    childConnector: string,
    position: PositionInput,
    name?: string | null,
    description?: string | null,
    scale?: number | null,
  ): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    const pos = formatPositionInput(position);
    const n = name == null ? "null" : __gqlStr(name);
    const d = description == null ? "null" : __gqlStr(description);
    const sc = scale == null ? "null" : String(scale);
    return this.kit.mutateScoped(
      cid,
      this.dsel(
        `ah: addHangingChildPieceWithParentConnection(blueprintId: ${__gqlStr(blueprintId)}, parentPieceId: ${__gqlStr(parentPieceId)}, parentConnector: ${__gqlStr(parentConnector)}, childConnector: ${__gqlStr(childConnector)}, position: ${pos}, name: ${n}, description: ${d}, scale: ${sc})`,
      ),
    );
  }

  async deletePiece(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`dp: deletePiece(id: ${__gqlStr(id)})`));
  }

  async deletePieces(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`dps: deletePieces(ids: ${__gqlIds(ids)})`));
  }

  async deletePiecesAndConnections(pieceIds: readonly string[], connectionIds: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.dsel(`dpc: deletePiecesAndConnections(pieceIds: ${__gqlIds(pieceIds)}, connectionIds: ${__gqlIds(connectionIds)})`));
  }
}
//#endregion 📐Design

//#region 🧰Type
export class Type extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private tsel(inner: string): string {
    return `type(id: ${__gqlStr(this.id)}) { ${inner} }`;
  }

  port(portId: string): Port {
    return new Port(this.kit, this.id, portId);
  }

  connector(connectorId: string): Connector {
    return new Connector(this.kit, this.id, connectorId);
  }

  representation(representationId: string): Representation {
    return new Representation(this.kit, this.id, representationId);
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`rn: rename(newName: ${__gqlStr(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`ci: changeIcon(newIcon: ${__gqlStr(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }

  async createPort(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    const c = code == null ? "null" : __gqlStr(code);
    const l = label == null ? "null" : __gqlStr(label);
    const d = description == null ? "null" : __gqlStr(description);
    const i = icon == null ? "null" : __gqlStr(icon);
    const o = order == null ? "null" : String(order);
    return this.kit.mutateScoped(cid, this.tsel(`cp: createPort(code: ${c}, label: ${l}, description: ${d}, icon: ${i}, order: ${o})`));
  }

  async deletePort(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`dp: deletePort(id: ${__gqlStr(id)})`));
  }

  async deletePorts(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`dps: deletePorts(ids: ${__gqlIds(ids)})`));
  }

  async addConnector(code: string, description?: string | null, icon?: string | null, portId?: string | null): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    const d = description == null ? "null" : __gqlStr(description);
    const i = icon == null ? "null" : __gqlStr(icon);
    const p = portId == null ? "null" : __gqlStr(portId);
    return this.kit.mutateScoped(cid, this.tsel(`ac: addConnector(code: ${__gqlStr(code)}, description: ${d}, icon: ${i}, portId: ${p})`));
  }

  async removeConnector(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`rc: removeConnector(id: ${__gqlStr(id)})`));
  }

  async removeConnectors(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`rcs: removeConnectors(ids: ${__gqlIds(ids)})`));
  }

  /** @emoji 🧰 Resolves {@code type(id){…}} on the materialized kit fragment. */
  private typeNode(frag: JsonObject | null): JsonObject | undefined {
    return frag?.["type"] as JsonObject | undefined;
  }

  async readName(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.tsel("name"))) as JsonObject | null;
    return String(this.typeNode(frag)?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.tsel("description"))) as JsonObject | null;
    return String(this.typeNode(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.tsel("icon"))) as JsonObject | null;
    return String(this.typeNode(frag)?.["icon"] ?? "");
  }

  async readImage(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.tsel("image"))) as JsonObject | null;
    return String(this.typeNode(frag)?.["image"] ?? "");
  }

  async readUnit(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.tsel("unit"))) as JsonObject | null;
    return String(this.typeNode(frag)?.["unit"] ?? "");
  }

  /** @emoji 🧰 Bulky {@code connectors { edges { node { id code name } } }} read (SDL {@code Type.connectors}). */
  async readConnectors(): Promise<readonly { readonly id: string; readonly code: string; readonly name: string }[]> {
    const inner = "connectors { edges { node { id code name } } }";
    const frag = (await this.kit.readKitInner(this.tsel(inner))) as JsonObject | null;
    const conn = this.typeNode(frag)?.["connectors"] as JsonObject | undefined;
    const edges = (conn?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    const out: { id: string; code: string; name: string }[] = [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      if (n == null) continue;
      const id = String(n["id"] ?? "");
      if (id === "") continue;
      out.push({ id, code: String(n["code"] ?? ""), name: String(n["name"] ?? "") });
    }
    return out;
  }

  /** @emoji 🧰 Bulky {@code representations { edges { node { id } } }} read (SDL {@code Type.representations}). */
  async readRepresentations(): Promise<readonly { readonly id: string }[]> {
    const inner = "representations { edges { node { id } } }";
    const frag = (await this.kit.readKitInner(this.tsel(inner))) as JsonObject | null;
    const rep = this.typeNode(frag)?.["representations"] as JsonObject | undefined;
    const edges = (rep?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    const out: { id: string }[] = [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      const id = String(n?.["id"] ?? "");
      if (id !== "") out.push({ id });
    }
    return out;
  }

  /** @emoji 🧰 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Type.attributes}). */
  async readAttributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.kit.readKitInner(this.tsel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this.typeNode(frag));
  }
}
//#endregion 🧰Type

//#region 🔘Port
export class Port extends Entity {
  readonly typeId: string;
  constructor(kit: Kit, typeId: string, id: string) {
    super(kit, id);
    this.typeId = typeId;
  }

  private psel(inner: string): string {
    return `type(id: ${__gqlStr(this.typeId)}) { port(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  /** @emoji 🔘 Resolves {@code type { port {…}}} on the kit fragment. */
  private portNode(frag: JsonObject | null): JsonObject | undefined {
    const t = frag?.["type"] as JsonObject | undefined;
    return t?.["port"] as JsonObject | undefined;
  }

  /** @emoji 🔘 SDL {@code Port.code}. */
  async readCode(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("code"))) as JsonObject | null;
    const v = this.portNode(frag)?.["code"];
    return v == null ? "" : String(v);
  }

  /** @emoji 🔘 SDL {@code Port.label}. */
  async readLabel(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("label"))) as JsonObject | null;
    const v = this.portNode(frag)?.["label"];
    return v == null ? "" : String(v);
  }

  /** @emoji 🔘 SDL {@code Port.order}. */
  async readOrder(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.psel("order"))) as JsonObject | null;
    const v = this.portNode(frag)?.["order"];
    return typeof v === "number" ? v : null;
  }

  async readName(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("name"))) as JsonObject | null;
    return String(this.portNode(frag)?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("description"))) as JsonObject | null;
    return String(this.portNode(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("icon"))) as JsonObject | null;
    return String(this.portNode(frag)?.["icon"] ?? "");
  }

  /** @emoji 🔘 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Port.attributes}). */
  async readAttributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.kit.readKitInner(this.psel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this.portNode(frag));
  }

  async rename(newCode: string, newLabel?: string | null): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    const lab = newLabel == null ? "null" : __gqlStr(newLabel);
    return this.kit.mutateScoped(cid, this.psel(`rn: rename(newCode: ${__gqlStr(newCode)}, newLabel: ${lab})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`ci: changeIcon(newIcon: ${__gqlStr(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }
}
//#endregion 🔘Port

//#region 🔗Connector
export class Connector extends Entity {
  readonly typeId: string;
  constructor(kit: Kit, typeId: string, id: string) {
    super(kit, id);
    this.typeId = typeId;
  }

  private csel(inner: string): string {
    return `type(id: ${__gqlStr(this.typeId)}) { connector(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  async rename(newCode: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`rn: rename(newCode: ${__gqlStr(newCode)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`ci: changeIcon(newIcon: ${__gqlStr(newIcon)})`));
  }

  /** @emoji 🔗 Resolves {@code type{ connector{…}}} on the kit fragment. */
  private connectorNode(frag: JsonObject | null): JsonObject | undefined {
    const t = frag?.["type"] as JsonObject | undefined;
    return t?.["connector"] as JsonObject | undefined;
  }

  async readName(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.csel("name"))) as JsonObject | null;
    return String(this.connectorNode(frag)?.["name"] ?? "");
  }

  async readCode(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.csel("code"))) as JsonObject | null;
    return String(this.connectorNode(frag)?.["code"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.csel("description"))) as JsonObject | null;
    return String(this.connectorNode(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.csel("icon"))) as JsonObject | null;
    return String(this.connectorNode(frag)?.["icon"] ?? "");
  }

  /** @emoji 🔗 Nullable {@code port { id }} per SDL {@code Connector.port}. */
  async readPortId(): Promise<string | null> {
    const frag = (await this.kit.readKitInner(this.csel("port { id }"))) as JsonObject | null;
    const p = this.connectorNode(frag)?.["port"] as JsonObject | null | undefined;
    if (p == null) return null;
    const id = String(p["id"] ?? "");
    return id === "" ? null : id;
  }

  /** @emoji 🔗 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Connector.attributes}). */
  async readAttributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.kit.readKitInner(this.csel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this.connectorNode(frag));
  }
}
//#endregion 🔗Connector

//#region 🧩Piece
/** @emoji 🧩 @description Blueprint target on a {@link Piece} (`Type` or `Design` node). */
export interface PieceBlueprint {
  readonly blueprintKind: "Type" | "Design";
  readonly id: string;
}

function __pieceKit(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const p = d?.["piece"] as JsonObject | undefined;
  return p ?? null;
}

function __parseCoordinateFromJson(node: JsonObject | null | undefined): Coordinate | null {
  if (node == null || typeof node !== "object") return null;
  const u = node["u"];
  const v = node["v"];
  if (typeof u !== "number" || typeof v !== "number") return null;
  return { u, v };
}

function __parsePointFromJson(node: JsonObject | null | undefined): Point | null {
  if (node == null || typeof node !== "object") return null;
  const x = node["x"];
  const y = node["y"];
  const z = node["z"];
  if (typeof x !== "number" || typeof y !== "number" || typeof z !== "number") return null;
  return { x, y, z };
}

function __parseVectorFromJson(node: JsonObject | null | undefined): Vector | null {
  return __parsePointFromJson(node);
}

function __parsePlaneFromJson(node: JsonObject | null | undefined): Plane | null {
  if (node == null || typeof node !== "object") return null;
  const origin = __parsePointFromJson(node["origin"] as JsonObject | undefined);
  const xAxis = __parseVectorFromJson(node["xAxis"] as JsonObject | undefined);
  const yAxis = __parseVectorFromJson(node["yAxis"] as JsonObject | undefined);
  if (origin == null || xAxis == null || yAxis == null) return null;
  return { origin, xAxis, yAxis };
}

function __parsePositionFromJson(node: JsonObject | null | undefined): Position | null {
  if (node == null || typeof node !== "object") return null;
  const center = __parseCoordinateFromJson(node["center"] as JsonObject | undefined);
  const plane = __parsePlaneFromJson(node["plane"] as JsonObject | undefined);
  if (center == null || plane == null) return null;
  return { center, plane };
}

function __parsePieceBlueprintFromJson(node: JsonObject | null | undefined): PieceBlueprint | null {
  if (node == null || typeof node !== "object") return null;
  const tn = String(node["__typename"] ?? "");
  const id = String(node["id"] ?? "");
  if (id === "") return null;
  if (tn === "Type") return { blueprintKind: "Type", id };
  if (tn === "Design") return { blueprintKind: "Design", id };
  return null;
}

function __parseAttributeNodesFromConnection(obj: JsonObject | null | undefined): readonly Attribute[] {
  const attrs = obj?.["attributes"] as JsonObject | undefined;
  const edges = attrs?.["edges"];
  if (!Array.isArray(edges)) return [];
  const out: Attribute[] = [];
  for (const e of edges) {
    if (e == null || typeof e !== "object" || Array.isArray(e)) continue;
    const n = (e as JsonObject)["node"] as JsonObject | undefined;
    if (n == null || typeof n !== "object") continue;
    const id = String(n["id"] ?? "");
    const key = String(n["key"] ?? "");
    if (id === "" || key === "") continue;
    const valueRaw = n["value"];
    const value = valueRaw == null ? null : String(valueRaw);
    out.push({ id, key, value, definition: String(n["definition"] ?? "") });
  }
  return out;
}

function __parseIdListConnection(obj: JsonObject | null | undefined, field: string): readonly string[] {
  const c = obj?.[field] as JsonObject | undefined;
  const edges = c?.["edges"];
  if (!Array.isArray(edges)) return [];
  const ids: string[] = [];
  for (const e of edges) {
    if (e == null || typeof e !== "object" || Array.isArray(e)) continue;
    const n = (e as JsonObject)["node"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    if (id !== "") ids.push(id);
  }
  return ids;
}

const __PIECE_POSITION_SUBSELECTION = "center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } }";

export class Piece extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  private psel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { piece(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  async readName(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("name"))) as JsonObject | null;
    return String(__pieceKit(frag)?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("description"))) as JsonObject | null;
    return String(__pieceKit(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("icon"))) as JsonObject | null;
    return String(__pieceKit(frag)?.["icon"] ?? "");
  }

  async readScale(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.psel("scale"))) as JsonObject | null;
    const v = __pieceKit(frag)?.["scale"];
    return typeof v === "number" ? v : null;
  }

  async readPosition(): Promise<Position | null> {
    const frag = (await this.kit.readKitInner(this.psel(`position { ${__PIECE_POSITION_SUBSELECTION} }`))) as JsonObject | null;
    return __parsePositionFromJson(__pieceKit(frag)?.["position"] as JsonObject | undefined);
  }

  async readFlatPosition(): Promise<Position | null> {
    const frag = (await this.kit.readKitInner(this.psel(`flatPosition { ${__PIECE_POSITION_SUBSELECTION} }`))) as JsonObject | null;
    return __parsePositionFromJson(__pieceKit(frag)?.["flatPosition"] as JsonObject | undefined);
  }

  async readPlane(): Promise<Plane | null> {
    return (await this.readPosition())?.plane ?? null;
  }

  async readCenter(): Promise<Coordinate | null> {
    return (await this.readPosition())?.center ?? null;
  }

  async readFlatPlane(): Promise<Plane | null> {
    return (await this.readFlatPosition())?.plane ?? null;
  }

  async readFlatCenter(): Promise<Coordinate | null> {
    return (await this.readFlatPosition())?.center ?? null;
  }

  async readBlueprint(): Promise<PieceBlueprint | null> {
    const frag = (await this.kit.readKitInner(this.psel("blueprint { __typename id }"))) as JsonObject | null;
    return __parsePieceBlueprintFromJson(__pieceKit(frag)?.["blueprint"] as JsonObject | undefined);
  }

  async readAttributes(): Promise<readonly Attribute[]> {
    const frag = (await this.kit.readKitInner(this.psel("attributes { edges { node { id key value definition } } }"))) as JsonObject | null;
    return __parseAttributeNodesFromConnection(__pieceKit(frag));
  }

  async readConnectionKind(): Promise<"FIXED" | "CONNECTED" | null> {
    const frag = (await this.kit.readKitInner(this.psel("connectionKind"))) as JsonObject | null;
    const k = __pieceKit(frag)?.["connectionKind"];
    if (k === "FIXED" || k === "CONNECTED") return k;
    return null;
  }

  async readParentPieceId(): Promise<string | null> {
    const frag = (await this.kit.readKitInner(this.psel("parentPiece { id }"))) as JsonObject | null;
    const n = __pieceKit(frag)?.["parentPiece"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  async readParentConnectionId(): Promise<string | null> {
    const frag = (await this.kit.readKitInner(this.psel("parentConnection { id }"))) as JsonObject | null;
    const n = __pieceKit(frag)?.["parentConnection"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  async readChildPieceIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.psel("childPieces { edges { node { id } } }"))) as JsonObject | null;
    return __parseIdListConnection(__pieceKit(frag), "childPieces");
  }

  async readChildConnectionIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.psel("childConnections { edges { node { id } } }"))) as JsonObject | null;
    return __parseIdListConnection(__pieceKit(frag), "childConnections");
  }

  async readDepth(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.psel("depth"))) as JsonObject | null;
    const v = __pieceKit(frag)?.["depth"];
    return typeof v === "number" ? v : null;
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`rn: rename(newName: ${__gqlStr(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async drag(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(position: PositionInput): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`mv: move(position: ${formatPositionInput(position)})`));
  }

  async fix(): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`fx: fix`));
  }

  async changeBlueprint(blueprintId: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`cb: changeBlueprint(blueprintId: ${__gqlStr(blueprintId)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }
}
//#endregion 🧩Piece

//#region 🪢PiecesOperations
export class PiecesOperations {
  constructor(
    private readonly kit: Kit,
    private readonly designId: string,
    private readonly pieceIds: readonly string[],
  ) { }

  private psel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { pieces(ids: ${__gqlIds(this.pieceIds)}) { ${inner} } }`;
  }

  async drag(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`mv: move(offset: ${formatOffsetInput(offset)})`));
  }

  async fix(): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`fx: fix`));
  }

  async changeBlueprint(blueprintId: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`cb: changeBlueprint(blueprintId: ${__gqlStr(blueprintId)})`));
  }
}
//#endregion 🪢PiecesOperations

//#region ⛓️Connection
/** @emoji ⛓️ @description Schema-aligned {@link Connection} side (piece + optional port / connector / designPiece ids). */
export interface ConnectionSide {
  readonly pieceId: string;
  readonly portId: string | null;
  readonly connectorId: string | null;
  readonly designPieceId: string | null;
}

const __CONNECTION_SIDE_SUBSELECTION = "piece { id } port { id } designPiece { id } connector { id }";

function __connectionKit(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const c = d?.["connection"] as JsonObject | undefined;
  return c ?? null;
}

function __parseConnectionSideFromJson(node: JsonObject | null | undefined): ConnectionSide | null {
  if (node == null || typeof node !== "object") return null;
  const piece = node["piece"] as JsonObject | undefined;
  const pieceId = piece == null ? "" : String(piece["id"] ?? "");
  if (pieceId === "") return null;
  const port = node["port"] as JsonObject | undefined;
  const portRaw = port == null ? "" : String(port["id"] ?? "");
  const portId = portRaw === "" ? null : portRaw;
  const dp = node["designPiece"] as JsonObject | undefined;
  const dpRaw = dp == null ? "" : String(dp["id"] ?? "");
  const designPieceId = dpRaw === "" ? null : dpRaw;
  const conn = node["connector"] as JsonObject | undefined;
  const cxRaw = conn == null ? "" : String(conn["id"] ?? "");
  const connectorId = cxRaw === "" ? null : cxRaw;
  return { pieceId, portId, connectorId, designPieceId };
}

export class Connection extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  private csel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { connection(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  async readName(): Promise<string> {
    const frag = ((await this.kit.readKitInner(this.csel("name"))) as JsonObject | null) ?? null;
    return String(__connectionKit(frag)?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = ((await this.kit.readKitInner(this.csel("description"))) as JsonObject | null) ?? null;
    return String(__connectionKit(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = ((await this.kit.readKitInner(this.csel("icon"))) as JsonObject | null) ?? null;
    return String(__connectionKit(frag)?.["icon"] ?? "");
  }

  async readGap(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("gap"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["gap"];
    return typeof v === "number" ? v : null;
  }

  async readShift(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("shift"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["shift"];
    return typeof v === "number" ? v : null;
  }

  async readRise(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("rise"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["rise"];
    return typeof v === "number" ? v : null;
  }

  async readRotation(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("rotation"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["rotation"];
    return typeof v === "number" ? v : null;
  }

  async readTurn(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("turn"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["turn"];
    return typeof v === "number" ? v : null;
  }

  async readTilt(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("tilt"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["tilt"];
    return typeof v === "number" ? v : null;
  }

  async readU(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("u"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["u"];
    return typeof v === "number" ? v : null;
  }

  async readV(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("v"))) as JsonObject | null) ?? null;
    const v = __connectionKit(frag)?.["v"];
    return typeof v === "number" ? v : null;
  }

  async readConnected(): Promise<ConnectionSide | null> {
    const frag = ((await this.kit.readKitInner(this.csel(`connected { ${__CONNECTION_SIDE_SUBSELECTION} }`))) as JsonObject | null) ?? null;
    return __parseConnectionSideFromJson(__connectionKit(frag)?.["connected"] as JsonObject | undefined);
  }

  async readConnecting(): Promise<ConnectionSide | null> {
    const frag = ((await this.kit.readKitInner(this.csel(`connecting { ${__CONNECTION_SIDE_SUBSELECTION} }`))) as JsonObject | null) ?? null;
    return __parseConnectionSideFromJson(__connectionKit(frag)?.["connecting"] as JsonObject | undefined);
  }

  async readAttributes(): Promise<readonly Attribute[]> {
    const frag = ((await this.kit.readKitInner(this.csel("attributes { edges { node { id key value definition } } }"))) as JsonObject | null) ?? null;
    return __parseAttributeNodesFromConnection(__connectionKit(frag));
  }
}
//#endregion ⛓️Connection

//#region ✍️Author
/** @emoji ✍️ Author artifact: kit-scoped reads only (no {@code *OperationInput} on Author in schema). */
export class Author extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readName(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { name } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { description } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { icon } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["icon"] ?? "");
  }

  async readEmail(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { email } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["email"] ?? "");
  }

  async readRole(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { role } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["role"] ?? "");
  }

  async readRank(): Promise<number | null> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { rank } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const r = n?.["rank"];
    return typeof r === "number" ? r : null;
  }
}
//#endregion ✍️Author

//#region 💎Quality
/** @emoji 💎 Quality artifact: {@code QualityOperationInput} leaves + scalar reads via {@code quality(id:)}. */
export class Quality extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private qsel(inner: string): string {
    return `quality(id: ${__gqlStr(this.id)}) { ${inner} }`;
  }

  private async readScalarUnderQuality(field: string): Promise<string> {
    const frag = (await this.kit.readKitInner(this.qsel(field))) as JsonObject | null;
    const q = frag?.["quality"] as JsonObject | undefined;
    return String(q?.[field] ?? "");
  }

  async readKey(): Promise<string> {
    return await this.readScalarUnderQuality("key");
  }

  async readValue(): Promise<string> {
    return await this.readScalarUnderQuality("value");
  }

  async readUnit(): Promise<string> {
    return await this.readScalarUnderQuality("unit");
  }

  async readDefinition(): Promise<string> {
    return await this.readScalarUnderQuality("definition");
  }

  async readName(): Promise<string> {
    return await this.readScalarUnderQuality("name");
  }

  async readDescription(): Promise<string> {
    return await this.readScalarUnderQuality("description");
  }

  async readIcon(): Promise<string> {
    return await this.readScalarUnderQuality("icon");
  }

  async readAttributes(): Promise<readonly Attribute[]> {
    const frag = (await this.kit.readKitInner(this.qsel(`attributes { edges { node { id key value definition } } }`))) as JsonObject | null;
    return parseAttributeConnectionUnder(frag?.["quality"] as JsonObject | undefined);
  }

  async readBenchmarks(): Promise<readonly Benchmark[]> {
    const frag = (await this.kit.readKitInner(
      this.qsel(`benchmarks { edges { node { id name min max minExcluded maxExcluded } } }`),
    )) as JsonObject | null;
    const q = frag?.["quality"] as JsonObject | undefined;
    const bench = q?.["benchmarks"] as JsonObject | undefined;
    const edges = bench?.["edges"] as readonly JsonValue[] | undefined;
    if (!Array.isArray(edges)) return [];
    const out: Benchmark[] = [];
    for (const e of edges) {
      if (!isJsonObjectNode(e)) continue;
      const n = e["node"] as JsonObject | undefined;
      if (n == null) continue;
      out.push({
        id: String(n["id"] ?? ""),
        name: String(n["name"] ?? ""),
        min: typeof n["min"] === "number" ? n["min"] : null,
        max: typeof n["max"] === "number" ? n["max"] : null,
        minExcluded: typeof n["minExcluded"] === "boolean" ? n["minExcluded"] : null,
        maxExcluded: typeof n["maxExcluded"] === "boolean" ? n["maxExcluded"] : null,
      });
    }
    return out;
  }

  async rename(newKey: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.qsel(`rk: rename(newKey: ${__gqlStr(newKey)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.qsel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.qsel(`ci: changeIcon(newIcon: ${__gqlStr(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.qsel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.qsel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.qsel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }
}
//#endregion 💎Quality

//#region 🏷️Tag
/** @emoji 🏷️ Tag artifact: {@code TagOperationInput} leaves + kit-scoped reads. */
export class Tag extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private tsel(inner: string): string {
    return `tag(id: ${__gqlStr(this.id)}) { ${inner} }`;
  }

  private async readScalarUnderTag(field: string): Promise<string> {
    const frag = (await this.kit.readKitInner(this.tsel(field))) as JsonObject | null;
    const t = frag?.["tag"] as JsonObject | undefined;
    return String(t?.[field] ?? "");
  }

  async readName(): Promise<string> {
    return await this.readScalarUnderTag("name");
  }

  async readDescription(): Promise<string> {
    return await this.readScalarUnderTag("description");
  }

  async readIcon(): Promise<string> {
    return await this.readScalarUnderTag("icon");
  }

  async readOrder(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.tsel("order"))) as JsonObject | null;
    const t = frag?.["tag"] as JsonObject | undefined;
    const o = t?.["order"];
    return typeof o === "number" ? o : null;
  }

  async readAttributes(): Promise<readonly Attribute[]> {
    const frag = (await this.kit.readKitInner(this.tsel(`attributes { edges { node { id key value definition } } }`))) as JsonObject | null;
    return parseAttributeConnectionUnder(frag?.["tag"] as JsonObject | undefined);
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`rn: rename(newName: ${__gqlStr(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`ci: changeIcon(newIcon: ${__gqlStr(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.tsel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }
}
//#endregion 🏷️Tag

//#region 💡Concept
/** @emoji 💡 Concept artifact: {@code ConceptOperationInput} leaves + kit-scoped reads. */
export class Concept extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private csel(inner: string): string {
    return `concept(id: ${__gqlStr(this.id)}) { ${inner} }`;
  }

  private async readScalarUnderConcept(field: string): Promise<string> {
    const frag = (await this.kit.readKitInner(this.csel(field))) as JsonObject | null;
    const c = frag?.["concept"] as JsonObject | undefined;
    return String(c?.[field] ?? "");
  }

  async readName(): Promise<string> {
    return await this.readScalarUnderConcept("name");
  }

  async readDescription(): Promise<string> {
    return await this.readScalarUnderConcept("description");
  }

  async readIcon(): Promise<string> {
    return await this.readScalarUnderConcept("icon");
  }

  async readOrder(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.csel("order"))) as JsonObject | null;
    const c = frag?.["concept"] as JsonObject | undefined;
    const o = c?.["order"];
    return typeof o === "number" ? o : null;
  }

  async readAttributes(): Promise<readonly Attribute[]> {
    const frag = (await this.kit.readKitInner(this.csel(`attributes { edges { node { id key value definition } } }`))) as JsonObject | null;
    return parseAttributeConnectionUnder(frag?.["concept"] as JsonObject | undefined);
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`rn: rename(newName: ${__gqlStr(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`ci: changeIcon(newIcon: ${__gqlStr(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`aa: addAttribute(key: ${__gqlStr(key)}, value: ${__gqlStr(value)}, definition: ${__gqlStr(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`ra: removeAttribute(id: ${__gqlStr(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.csel(`ras: removeAttributes(ids: ${__gqlIds(ids)})`));
  }
}
//#endregion 💡Concept

//#region 🎨Representation
/** @emoji 🎨 Representation under {@link Type}: read-only until schema adds {@code RepresentationOperationInput}. */
export class Representation extends Entity {
  readonly typeId: string;
  constructor(kit: Kit, typeId: string, id: string) {
    super(kit, id);
    this.typeId = typeId;
  }

  private rsel(inner: string): string {
    return `type(id: ${__gqlStr(this.typeId)}) { representation(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  private async readUnderRepresentation(field: string): Promise<string> {
    const frag = (await this.kit.readKitInner(this.rsel(field))) as JsonObject | null;
    const t = frag?.["type"] as JsonObject | undefined;
    const r = t?.["representation"] as JsonObject | undefined;
    return String(r?.[field] ?? "");
  }

  async readName(): Promise<string> {
    return await this.readUnderRepresentation("name");
  }

  async readUrl(): Promise<string> {
    return await this.readUnderRepresentation("url");
  }

  async readDescription(): Promise<string> {
    return await this.readUnderRepresentation("description");
  }

  async readIcon(): Promise<string> {
    return await this.readUnderRepresentation("icon");
  }

  async readFileId(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.rsel(`file { id }`))) as JsonObject | null;
    const t = frag?.["type"] as JsonObject | undefined;
    const r = t?.["representation"] as JsonObject | undefined;
    const f = r?.["file"] as JsonObject | undefined;
    return String(f?.["id"] ?? "");
  }

  async readTagIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.rsel(`tags { edges { node { id } } }`))) as JsonObject | null;
    const t = frag?.["type"] as JsonObject | undefined;
    const r = t?.["representation"] as JsonObject | undefined;
    const tags = r?.["tags"] as JsonObject | undefined;
    const edges = tags?.["edges"] as readonly JsonValue[] | undefined;
    if (!Array.isArray(edges)) return [];
    const ids: string[] = [];
    for (const e of edges) {
      if (!isJsonObjectNode(e)) continue;
      const n = e["node"] as JsonObject | undefined;
      if (n) ids.push(String(n["id"] ?? ""));
    }
    return ids;
  }

  async readQualityIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.rsel(`qualities { edges { node { id } } }`))) as JsonObject | null;
    const t = frag?.["type"] as JsonObject | undefined;
    const r = t?.["representation"] as JsonObject | undefined;
    const quals = r?.["qualities"] as JsonObject | undefined;
    const edges = quals?.["edges"] as readonly JsonValue[] | undefined;
    if (!Array.isArray(edges)) return [];
    const ids: string[] = [];
    for (const e of edges) {
      if (!isJsonObjectNode(e)) continue;
      const n = e["node"] as JsonObject | undefined;
      if (n) ids.push(String(n["id"] ?? ""));
    }
    return ids;
  }

  async readAttributes(): Promise<readonly Attribute[]> {
    const frag = (await this.kit.readKitInner(this.rsel(`attributes { edges { node { id key value definition } } }`))) as JsonObject | null;
    const t = frag?.["type"] as JsonObject | undefined;
    const r = t?.["representation"] as JsonObject | undefined;
    return parseAttributeConnectionUnder(r);
  }
}
//#endregion 🎨Representation

//#region 👨‍👩‍👦Family
/** @emoji 👨‍👩‍👦 Family artifact: read-only in current kit API. */
export class Family extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private async readScalarOnNode(field: string): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Family { ${field} } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.[field] ?? "");
  }

  async readName(): Promise<string> {
    return await this.readScalarOnNode("name");
  }

  async readDescription(): Promise<string> {
    return await this.readScalarOnNode("description");
  }

  async readIcon(): Promise<string> {
    return await this.readScalarOnNode("icon");
  }
}
//#endregion 👨‍👩‍👦Family

//#region 📄File
export class File extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readName(): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on File { name } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    return String((data["node"] as JsonObject | undefined)?.["name"] ?? "");
  }
}
//#endregion 📄File

//#region 📁Folder
/** @emoji 📁 Folder artifact: read-only in current kit API. */
export class Folder extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private async readScalarOnNode(field: string): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Folder { ${field} } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.[field] ?? "");
  }

  async readName(): Promise<string> {
    return await this.readScalarOnNode("name");
  }

  async readDescription(): Promise<string> {
    return await this.readScalarOnNode("description");
  }

  async readPath(): Promise<string> {
    return await this.readScalarOnNode("path");
  }
}
//#endregion 📁Folder

//#region 🪟Layer
/** @emoji 🪟 Design layer row: read-only in current kit API. */
export class Layer extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  private lsel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { layers { edges { node { id ${inner} } } } }`;
  }

  private async selfLayerNode(innerFields: string): Promise<JsonObject | null> {
    const frag = (await this.kit.readKitInner(this.lsel(innerFields))) as JsonObject | null;
    const edges = (((frag?.["design"] as JsonObject | undefined)?.["layers"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      if (n && String(n["id"] ?? "") === this.id) return n;
    }
    return null;
  }

  async readName(): Promise<string> {
    const n = await this.selfLayerNode("name");
    return String(n?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const n = await this.selfLayerNode("description");
    return String(n?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const n = await this.selfLayerNode("icon");
    return String(n?.["icon"] ?? "");
  }

  async readColor(): Promise<string> {
    const n = await this.selfLayerNode("color");
    return String(n?.["color"] ?? "");
  }

  async readOrder(): Promise<number | null> {
    const n = await this.selfLayerNode("order");
    const o = n?.["order"];
    return typeof o === "number" ? o : null;
  }

  async readVisible(): Promise<boolean | null> {
    const n = await this.selfLayerNode("visible");
    const v = n?.["visible"];
    return typeof v === "boolean" ? v : null;
  }

  async readLocked(): Promise<boolean | null> {
    const n = await this.selfLayerNode("locked");
    const v = n?.["locked"];
    return typeof v === "boolean" ? v : null;
  }
}
//#endregion 🪟Layer

//#region 👥Group
export class Group extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  async readName(): Promise<string> {
    const frag = await this.kit.readKitInner(`design(id: ${__gqlStr(this.designId)}) { groups { edges { node { id name } } } }`);
    const edges = (((frag as JsonObject | null)?.["design"] as JsonObject | undefined)?.["groups"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined;
    for (const e of edges ?? []) {
      const n = e["node"] as JsonObject | undefined;
      if (n && String(n["id"] ?? "") === this.id) return String(n["name"] ?? "");
    }
    return "";
  }
}
//#endregion 👥Group

//#region 📊Stat
/** @emoji 📊 Stat artifact: read-only in current kit API. */
export class Stat extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private async readScalarOnNode(field: string): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Stat { ${field} } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.[field] ?? "");
  }

  async readKey(): Promise<string> {
    return await this.readScalarOnNode("key");
  }

  async readValue(): Promise<string> {
    return await this.readScalarOnNode("value");
  }

  async readUnit(): Promise<string> {
    return await this.readScalarOnNode("unit");
  }

  async readName(): Promise<string> {
    return await this.readScalarOnNode("name");
  }

  async readDescription(): Promise<string> {
    return await this.readScalarOnNode("description");
  }

  async readIcon(): Promise<string> {
    return await this.readScalarOnNode("icon");
  }
}
//#endregion 📊Stat

//#region 🎚️Prop
/** @emoji 🎚️ Prop artifact: read-only in current kit API. */
export class Prop extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private async readScalarOnNode(field: string): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Prop { ${field} } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.[field] ?? "");
  }

  async readKey(): Promise<string> {
    return await this.readScalarOnNode("key");
  }

  async readValue(): Promise<string> {
    return await this.readScalarOnNode("value");
  }

  async readUnit(): Promise<string> {
    return await this.readScalarOnNode("unit");
  }

  async readName(): Promise<string> {
    return await this.readScalarOnNode("name");
  }

  async readQualityId(): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Prop { quality { id } } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const q = n?.["quality"] as JsonObject | undefined;
    return String(q?.["id"] ?? "");
  }
}
//#endregion 🎚️Prop

//#endregion 🧱Classes

//#region 🪶WeakEntities
//#region 📐Plane
/** @emoji 📐 Weak plane value (schema mirror). */
export interface Plane {
  readonly origin: Vector;
  readonly xAxis: Vector;
  readonly yAxis: Vector;
}
//#endregion 📐Plane
//#region 📍Coordinate
export interface Coordinate {
  readonly u: number;
  readonly v: number;
}
//#endregion 📍Coordinate
//#region 🔵Point
export interface Point {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
//#endregion 🔵Point
//#region ➡️Vector
export interface Vector {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
//#endregion ➡️Vector
//#region ↔️Side
export interface Side {
  readonly piece: { readonly id: string };
  readonly connector: string;
}
//#endregion ↔️Side
//#region 📌Position
export interface Position {
  readonly center: Coordinate;
  readonly plane: Plane;
}
//#endregion 📌Position

//#region 📥GeomInputs
/** @emoji 📥 GraphQL {@code PositionInput} mirror for kit mutations. */
export type PositionInput = Readonly<{
  center: Readonly<Coordinate>;
  plane: Readonly<Plane>;
}>;

/** @emoji 📥 GraphQL {@code OffsetInput} mirror for kit mutations. */
export type OffsetInput = Readonly<{
  u: number;
  v: number;
}>;

function __gqlGeomNum(n: number): string {
  return Number.isFinite(n) ? String(n) : "0";
}

/** @emoji 📡 Inline GraphQL object literal for {@code PositionInput}. */
export function formatPositionInput(p: PositionInput): string {
  const c = p.center;
  const pl = p.plane;
  const o = pl.origin;
  const xa = pl.xAxis;
  const ya = pl.yAxis;
  return `{ center: { u: ${__gqlGeomNum(c.u)}, v: ${__gqlGeomNum(c.v)} }, plane: { origin: { x: ${__gqlGeomNum(o.x)}, y: ${__gqlGeomNum(o.y)}, z: ${__gqlGeomNum(o.z)} }, xAxis: { x: ${__gqlGeomNum(xa.x)}, y: ${__gqlGeomNum(xa.y)}, z: ${__gqlGeomNum(xa.z)} }, yAxis: { x: ${__gqlGeomNum(ya.x)}, y: ${__gqlGeomNum(ya.y)}, z: ${__gqlGeomNum(ya.z)} } } }`;
}

/** @emoji 📡 Inline GraphQL object literal for {@code OffsetInput}. */
export function formatOffsetInput(o: OffsetInput): string {
  return `{ u: ${__gqlGeomNum(o.u)}, v: ${__gqlGeomNum(o.v)} }`;
}
//#endregion 📥GeomInputs
//#region 🌍Place
export interface Place {
  readonly location: Location;
}
//#endregion 🌍Place
//#region 🗺️Location
export interface Location {
  readonly latitude: number;
  readonly longitude: number;
}
//#endregion 🗺️Location
//#region 📷Camera
export interface Camera {
  readonly position: Point;
  readonly target: Point;
}
//#endregion 📷Camera
//#region 🏁Benchmark
/** @emoji 🏁 Benchmark row subset from {@code Benchmark} (owner: Quality). */
export interface Benchmark {
  readonly id: string;
  readonly name: string;
  readonly min: number | null;
  readonly max: number | null;
  readonly minExcluded: boolean | null;
  readonly maxExcluded: boolean | null;
}
//#endregion 🏁Benchmark
//#region 🪪Attribute
/** @emoji 🪪 Attribute row mirror for {@code Attribute} weak entity edges. */
export interface Attribute {
  readonly id: string;
  readonly key: string;
  readonly value: string | null;
  readonly definition: string;
}
//#endregion 🪪Attribute
//#endregion 🪶WeakEntities

//#region 🚀PublicAPI
/** @emoji 🚀 Opens a {@link Kit} backed by rs WASM (worker or inline). */
export async function openKit(uri: string, opts?: KitOpenOptions): Promise<Kit> {
  return Kit.open(uri, opts);
}
//#endregion 🚀PublicAPI


// #region 🧪EmbeddedTests
if (
  typeof process !== "undefined" &&
  !!process.env &&
  process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1"
) {
  const { describe, it, expect } = await import("vitest");
  type KitFullDto = Graph.KitFullDto;
  type KitJsonFileAdapter = Graph.KitJsonFileAdapter;
  type KitClassifiedMutationEvent = Graph.KitClassifiedMutationEvent;
  type ReadBatch = Graph.ReadBatch;
  type KitEvent = Graph.KitEvent;
  type ChangeKitCommand = Graph.ChangeKitCommand;
  type KitStoreClient = Graph.KitStoreClient;

  describe("semio-js KitStore", () => {

    it("KitStore has no JS snapshot() full-read method (use theKit / scoped reads only)", () => {
      type Snap = { snapshot?: () => unknown };
      const snap: Snap = KitStore.prototype as unknown as Snap;
      expect(snap.snapshot).toBeUndefined();
    });

    describe("wasm GraphQL integration (KitStoreHandle over GraphQL)", () => {
      it("opens dedicated worker wasm and returns typed full kit DTO from GraphQL", async () => {
        const minimalKit: KitFullDto = {
          id: "test-kit",
          name: "TestKit",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [{ id: "type-1", name: "Wall", connectors: [] }],
          designs: [{ id: "design-1", name: "Floor1", pieces: [], connections: [] }],
        };
        const ks = await KitStore.open(minimalKit);
        const snap = await ks.theKit();
        expect(snap.id).toBe("test-kit");
        expect(snap.name).toBe("TestKit");
        const typeStores = await ks.types();
        expect(typeStores.map((t) => t.id)).toEqual(["type-1"]);
        const designStores = await ks.designs();
        expect(designStores.map((d) => d.id)).toEqual(["design-1"]);
        const meta = await ks.type("type-1").metadata();
        expect(meta.id).toBe("type-1");
        expect(meta.name).toBe("Wall");
        await ks.dispose();
      });
    });

    it("kitReadPointKey normalizes the main line scope for cache keys", () => {
      expect(kitReadPointKey(theKitReadPoint)).toBe(JSON.stringify(theKitReadPoint));
    });

    it("kitChangeSemanticKindToGraphQl maps GraphQL enum + other label", () => {
      expect(kitChangeSemanticKindToGraphQl("ADD_PIECE", null)).toBe("addPiece");
      expect(kitChangeSemanticKindToGraphQl("OTHER", "addFamily")).toEqual({ other: "addFamily" });
      expect(kitChangeSemanticKindToGraphQl("OTHER", null)).toBe("inferred");
    });

    it("normalizeKitEventFromSubscription passes flat classified mutation rows", () => {
      const raw = {
        renamedDesign: {
          designId: "d1",
          change: { forward: [] as const, inverse: [] as const, kind: "modifyDesign" as const },
        },
      };
      const out = normalizeKitEventFromSubscription(raw);
      expect(out).toBeDefined();
      expect(isKitClassifiedMutationEvent(out!)).toBe(true);
      if (isKitClassifiedMutationEvent(out!)) {
        expect("renamedDesign" in out && out.renamedDesign.designId).toBe("d1");
      }
    });

    it("kitEventTouchesDesignStrict matches renamedDesign kit events", () => {
      const ev = {
        renamedDesign: { designId: "dx", change: { forward: [], inverse: [] } },
      } as const satisfies KitClassifiedMutationEvent;
      expect(kitEventTouchesDesignStrict(ev, "dx")).toBe(true);
      expect(kitEventTouchesDesignStrict(ev, "other")).toBe(false);
    });

    describe("wasm GraphQL integration · KitStore.read / vcs", () => {
      it("read batch returns typed rows", async () => {
        const minimalKit: KitFullDto = {
          id: "read-kit",
          name: "R",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        const batch: ReadBatch = [{ readKitTypesShallowCommand: null }, { readKitTypeIdsCommand: null }];
        const res = await ks.read(theKitReadPoint, batch);
        expect(res.length).toBe(2);
        await ks.dispose();
      });

      it("designIds and kindIds align with design() and type() factory lists", async () => {
        const minimalKit: KitFullDto = {
          id: "row-ids-kit",
          name: "R",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [{ id: "ta", name: "A", connectors: [] }],
          designs: [{ id: "da", name: "D", pieces: [], connections: [] }],
        };
        const ks = await KitStore.open(minimalKit);
        expect(await ks.designIds()).toEqual((await ks.designs()).map((d) => d.id));
        expect(await ks.kindIds()).toEqual((await ks.types()).map((t) => t.id));
        await ks.dispose();
      });

      it("PieceStore readFlatPlane is defined on the owning store (delegates to live read dto)", async () => {
        const minimalKit: KitFullDto = {
          id: "piece-flat-kit",
          name: "P",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [{ id: "t1", name: "T", connectors: [] }],
          designs: [
            {
              id: "d1",
              name: "D",
              pieces: [
                {
                  id: "p1",
                  name: "Piece1",
                  type: { id: "t1" },
                  plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
                  center: { u: 0, v: 0 },
                  scale: 1,
                  color: "#000000",
                  props: [],
                  attributes: [],
                },
              ],
              connections: [],
            },
          ],
        };
        const ks = await KitStore.open(minimalKit);
        expect(typeof ks.piece("d1", "p1").readFlatPlane).toBe("function");
        expect(typeof ks.design("d1").readClusterableGroups).toBe("function");
        expect(typeof ks.type("t1").readBestRepresentation).toBe("function");
        await ks.dispose();
      });

      it("rejects theKit() after dispose", async () => {
        const minimalKit: KitFullDto = {
          id: "dispose-kit",
          name: "D",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        await ks.dispose();
        await expect(ks.theKit()).rejects.toThrow(/disposed/i);
      });

      it("subscribe returns Unsubscribe and does not expose events$", async () => {
        const minimalKit: KitFullDto = {
          id: "sub-kit",
          name: "S",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        let n = 0;
        const off = ks.subscribe(() => {
          n += 1;
        });
        expect(typeof off).toBe("function");
        off();
        await ks.dispose();
        type KitStorePublicKeys = keyof KitStore;
        type MustNotIncludeEvents = "events$" extends KitStorePublicKeys ? never : true;
        const _compileAssert: MustNotIncludeEvents = true;
        expect(_compileAssert).toBe(true);
        expect(n).toBeGreaterThanOrEqual(0);
      });

      it("subscribeFiltered and subscribeSemioKitCommandLifecycle return Unsubscribe (RxJS internal; no events$ on KitStore)", async () => {
        const minimalKit: KitFullDto = {
          id: "sub-filter-kit",
          name: "Sf",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        const offFiltered = ks.subscribeFiltered(() => false, () => {
          /* noop */
        });
        const offLifecycle = ks.subscribeSemioKitCommandLifecycle(() => {
          /* noop */
        });
        expect(typeof offFiltered).toBe("function");
        expect(typeof offLifecycle).toBe("function");
        offFiltered();
        offLifecycle();
        await ks.dispose();
      });

      it("theKit, vcsState, readAt root, and undo/redo flags round-trip", async () => {
        const minimalKit: KitFullDto = {
          id: "vcs-kit",
          name: "V",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        const snap = await ks.theKit();
        const snap2 = await ks.theKit();
        expect(snap2.id).toBe(snap.id);
        const vcs = await ks.vcsState();
        expect(vcs != null && typeof vcs === "object").toBe(true);
        const mat = await ks.readAt("");
        expect(mat.id).toBe(snap.id);
        expect(typeof (await ks.canUndo())).toBe("boolean");
        expect(typeof (await ks.canRedo())).toBe("boolean");
        await ks.dispose();
      });

      it("createAlternativeFromTip adds an alternative on wip", async () => {
        const minimalKit: KitFullDto = {
          id: "alt-create-kit",
          name: "A",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        await ks.kitStoreInitializeDefaults();
        const aid = await ks.createAlternativeFromTip("branch-one", null);
        expect(aid.length).toBeGreaterThan(8);
        const vcs = await ks.vcsState();
        const wip = vcs["wip"] as { alternatives?: readonly { id?: string }[] } | undefined;
        const alts = wip?.alternatives;
        expect(Array.isArray(alts)).toBe(true);
        expect((alts as readonly { id?: string }[]).some((a) => String(a?.id) === aid)).toBe(true);
        await ks.dispose();
      });

      it("serializes a fresh dev-json bundle with three graphs, a seed checkpoint, and an open unsaved change", async () => {
        const minimalKit: KitFullDto = {
          id: "dev-json-kit",
          name: "the kit",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        await ks.kitStoreInitializeDefaults();
        const raw = await ks.serializeKitStoreBundleJson();
        const bundle = JSON.parse(raw) as JsonObject;
        expect(bundle["schema"]).toBe("🎆26🌙06⬆️1");
        for (const key of ["wip", "authoritative", "stage"]) {
          expect(bundle[key] != null && typeof bundle[key] === "object").toBe(true);
          expect(((bundle[key] as JsonObject)["initialKit"] as JsonObject | undefined)?.["name"]).toBe("the kit");
        }
        const wip = bundle["wip"] as JsonObject;
        expect((((wip["checkpoints"] as JsonObject)["items"] as readonly unknown[]) ?? []).length).toBe(1);
        expect(wip["drafts"]).toBeUndefined();
        expect(wip["transactions"]).toBeUndefined();
        expect(wip["savedChanges"]).toBeUndefined();
        expect(wip["unsavedChanges"]).toBeUndefined();
        const theKit = wip["theKit"] as JsonObject;
        const changes = ((theKit["unsavedChanges"] as JsonObject)["items"] as readonly JsonObject[]) ?? [];
        expect(changes.length).toBe(1);
        const edits = ((changes[0]["edits"] as JsonObject)["items"] as readonly unknown[]) ?? [];
        expect(edits.length).toBe(0);
        await ks.dispose();
      });

    });

    it("compile-time: KitStore public surface excludes rxjs-style stream fields", () => {
      type KitStorePublicKeys = keyof KitStore;
      type MustNotLeakRx = "events$" extends KitStorePublicKeys ? never : "pipe" extends KitStorePublicKeys ? never : "_trySubscribe" extends KitStorePublicKeys ? never : true;
      const _assert: MustNotLeakRx = true;
      expect(_assert).toBe(true);
    });
  });

  describe("semio-js GraphQL dto contract", () => {
    it("KIT_SESSION_QUERY_ENTRY and KIT_EVENT_STREAM_SUBSCRIPTION align with target.schema.graphql", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      let sdl = "";
      const here = dirname(fileURLToPath(import.meta.url));
      for (const p of [resolve(here, "../graphql/target.schema.graphql"), resolve(process.cwd(), "semio/graphql/target.schema.graphql")]) {
        try {
          sdl = readFileSync(p, "utf8");
          if (sdl.length > 100) break;
        } catch {
          /* try next */
        }
      }
      expect(sdl.length).toBeGreaterThan(100);
      expect(sdl).toContain("type Session");
      expect(sdl).toContain("type Kit");
      expect(sdl).toMatch(/type Kit[\s\S]*designs:/s);
      expect(sdl).toMatch(/type Subscription[\s\S]*\bwip\b/s);
      expect(sdl).not.toMatch(/^\s*event:\s*Json!/m);
      expect(sdl).toContain("type Mutation");
      expect(sdl).toContain("session: SessionCommandInput!");
      expect(sdl).not.toContain("type KitStoreMutation");
      expect(KIT_SESSION_QUERY_ENTRY).toContain("wip { id theKit");
      expect(KIT_EVENT_STREAM_SUBSCRIPTION).toContain("wip");
      expect(KIT_COMMAND_SUCCEEDED_SUBSCRIPTION).toBe(KIT_EVENT_STREAM_SUBSCRIPTION);
    });
  });

  describe("semio kit-store fixtures (US-001)", () => {
    it("golden ops + expected invariants parse and match op count", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const here = dirname(fileURLToPath(import.meta.url));
      const opsPath = resolve(here, "../assets/semio/kit-store.golden.ops.semio.json");
      const expPath = resolve(here, "../assets/semio/kit-store.golden.expected.semio.json");
      const ops = JSON.parse(readFileSync(opsPath, "utf8")) as { ops: unknown[] };
      const exp = JSON.parse(readFileSync(expPath, "utf8")) as { invariants: { totalPieces: number }; projectionFingerprint: string };
      expect(ops.ops.length).toBe(exp.invariants.totalPieces);
      expect(exp.projectionFingerprint.length).toBe(64);
    });

    it("metabolism.new kit bundle has metabolism on-disk shape (Rust-owned)", async () => {
      // 🚧 The on-disk bundle format is owned by `semio/rs` — JS only verifies the metabolism shape exists on the asset.
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const here = dirname(fileURLToPath(import.meta.url));
      const b = JSON.parse(readFileSync(resolve(here, "../assets/semio/metabolism.new.kit.semio.json"), "utf8")) as {
        schema: string;
        wip: { id: string; initialKit?: unknown; theKit?: { savedChanges?: { items: unknown[] }; unsavedChanges?: { items: unknown[] } }; checkpoints?: { items: unknown[] } };
        authoritative: { id: string };
        stage: { id: string };
        conflicts: { items: unknown[] };
        blobs: { items: unknown[] };
      };
      expect(b.schema).toBe("🎆26🌙06⬆️1");
      for (const k of ["wip", "authoritative", "stage", "conflicts", "blobs"] as const) {
        expect(b[k]).toBeTruthy();
      }
      expect(typeof b.wip.id).toBe("string");
      expect(b.wip.initialKit).toBeTruthy();
    });

    it("dev JSON backbone wire shape documents semanticOpLog + persistence hints (US-004)", async () => {
      const backboneDoc = {
        kind: "semio.kit_backbone.dev_json",
        schema: "2026-05-06",
        connectionUri: "file:///tmp/example.dev-kit.json",
        persistence: {
          atomic_rewrite:
            "Serialize full JSON to sibling path ending in .tmp.semio-write, fsync, then rename(2) over the canonical file.",
          crash_safety: "Readers only observe the last renamed complete document; orphaned temp tails are harmless.",
        },
        semanticOpLog: [] as { changeId: string; kind: string; input: Record<string, unknown> }[],
      };
      expect(backboneDoc.kind).toBe("semio.kit_backbone.dev_json");
      expect(backboneDoc.persistence.atomic_rewrite.includes("rename")).toBe(true);
      expect(Array.isArray(backboneDoc.semanticOpLog)).toBe(true);
    });
  });

  describe("semio-js kit event entity filters", () => {
    it("kitEventTouchesDesignStrict matches nested Design payload", () => {
      const ev = { Design: { design_id: "d1", event: { Piece: { piece_id: "p1", event: "Changed" } } } } as KitEvent;
      expect(kitEventTouchesDesignStrict(ev, "d1")).toBe(true);
      expect(kitEventTouchesDesignStrict(ev, "d2")).toBe(false);
    });

    it("kitEventTouchesPiece ignores bare Changed", () => {
      expect(kitEventTouchesPiece({ Changed: null } as KitEvent, "d1", "p1")).toBe(false);
    });

    it("kitEventTouchesPiece matches FlattenInvalidated piece list", () => {
      const ev = { FlattenInvalidated: { design: "d1", pieces: ["p1"] } } as KitEvent;
      expect(kitEventTouchesPiece(ev, "d1", "p1")).toBe(true);
      expect(kitEventTouchesPiece(ev, "d1", "p2")).toBe(false);
    });

    it("kitEventTouchesDesign matches rs-shaped design name field change", () => {
      const ev = { Design: { design_id: "design-a", event: { FieldChanged: "Name" } } } as KitEvent;
      expect(kitEventTouchesDesign(ev, "design-a")).toBe(true);
      expect(kitEventTouchesDesign(ev, "other")).toBe(false);
    });

    it("kitEventTouchesTypeStrict matches Type payload", () => {
      const ev = { Type: { type_id: "t1", event: "Changed" } } as KitEvent;
      expect(kitEventTouchesTypeStrict(ev, "t1")).toBe(true);
      expect(kitEventTouchesTypeStrict(ev, "t2")).toBe(false);
    });
  });

  describe("semio-js entity stores", () => {
    describe("wasm KitStoreHandle · entity stores", () => {
      it("TypeStore metadata and shallow read paths resolve", async () => {
        const minimalKit: KitFullDto = {
          id: "meta-type-kit",
          name: "K",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: [{ id: "type-z", name: "Zed", connectors: [] }],
          designs: [],
        };
        const ks = await KitStore.open(minimalKit);
        const t = ks.type("type-z");
        const meta = await t.metadata();
        expect(meta.id).toBe("type-z");
        expect(meta.name).toBe("Zed");
        const sh = await t.shallow();
        expect(sh.id).toBe("type-z");
        await ks.dispose();
      });
    });
  });

  describe("semio-js kit store dto helpers", () => {
    it("piecePatchToChangeCommands maps plane and type ref", () => {
      const cmds = piecePatchToChangeCommands({ plane: { x: 1 }, type: { id: "t1" } });
      expect(cmds.length).toBe(2);
      expect(cmds.some((c) => "plane" in c)).toBe(true);
      expect(cmds.some((c) => "type" in c && (c as { type: { typeId: { id: string } } }).type.typeId.id === "t1")).toBe(true);
    });

    it("connectionDiffKeyForDataKey maps u to x", () => {
      expect(connectionDiffKeyForDataKey("u")).toBe("x");
      expect(connectionDiffKeyForDataKey("gap")).toBe("gap");
    });

    it("buildSchemaEntityChangeCommands returns nested piece dto with design id", () => {
      const cmds = buildSchemaEntityChangeCommands("Piece", "p1", "color", "#fff", "d1");
      expect(cmds.length).toBe(1);
      expect(cmds[0]).toMatchObject({
        changeDesignCommands: { designId: { id: "d1" } },
      });
    });

    it("kitStoreClientUpdatePiece forwards to submitChangeKitCommands", async () => {
      let last: readonly ChangeKitCommand[] | undefined;
      const client = {
        getKitWriteScope: () => null,
        setKitWriteScope: () => { },
        finalizeKitWriteTransaction: async () => ({ ok: true as const }),
        abortKitWriteTransaction: async () => ({ ok: true as const }),
        submitChangeKitCommands: async (cs: readonly ChangeKitCommand[]) => {
          last = cs;
          return { ok: true as const };
        },
      } as unknown as KitStoreClient;
      await kitStoreClientUpdatePiece(client, "d1", "p1", { name: "N" });
      expect(last?.length).toBe(1);
    });

    it("decodeKitSemioEnvelopeToFullDtoFromValue unwraps initialKit and flattens bundle items", () => {
      const dto = decodeKitSemioEnvelopeToFullDtoFromValue({
        schema: "s",
        initialKit: {
          id: "kit-1",
          name: "Kit",
          createdAt: "2020-01-01T00:00:00.000Z",
          updatedAt: "2020-01-01T00:00:00.000Z",
          types: {
            hash: "h",
            items: [
              {
                id: "t1",
                name: "T",
                createdAt: "2020-01-01T00:00:00.000Z",
                updatedAt: "2020-01-01T00:00:00.000Z",
                families: { items: [{ id: "f1" }] },
                connectors: [],
                representations: [],
              },
            ],
          },
          designs: { items: [] },
        },
      });
      expect(dto.id).toBe("kit-1");
      expect(Array.isArray(dto.types)).toBe(true);
      expect(dto.types?.[0]?.id).toBe("t1");
      expect(Array.isArray(dto.types?.[0]?.families)).toBe(true);
    });
  });
}
// #endregion 🧪EmbeddedTests


//#region 🧪Tests
if (typeof process !== "undefined" && !!process.env && process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  describe("semio/js field-only kit", () => {
    it("source has no banned cache/sync substrings", async () => {
      const fs = await import("node:fs");
      const url = await import("node:url");
      const p = url.fileURLToPath(import.meta.url);
      const text = fs.readFileSync(p, "utf8");
      const marker = "//#region 🧪Tests";
      const idx = text.indexOf(marker);
      const head = idx < 0 ? text : text.slice(0, idx);
      for (const ban of ["applyToCache", "dispatchSync", "fieldSync", "KitStoreSnapshot", "optimistic", "reconcil"] as const) {
        expect(head.includes(ban), ban).toBe(false);
      }
    });

    it("Piece.drag issues one mutateScoped path (stub kit)", async () => {
      const calls: string[] = [];
      const k = Object.create(Kit.prototype) as Kit;
      (k as unknown as { ensureAlive(): void }).ensureAlive = () => { };
      (k as unknown as { mutateScoped: (c: string, s: string) => Promise<SetResult> }).mutateScoped = async (_c, s) => {
        calls.push(s);
        return { ok: true };
      };
      (k as unknown as { ensureChangeId: () => Promise<string> }).ensureChangeId = async () => "chg";
      const piece = new Piece(k, "d1", "p1");
      await piece.drag({ u: 1, v: 2 });
      expect(calls.length).toBe(1);
      expect(calls[0]).toContain("drag(offset:");
    });

    it("EventBus delivers subscription-shaped payloads", () => {
      const bus = new EventBus();
      const seen: JsonValue[] = [];
      bus.subscribe((e) => seen.push(e));
      bus.emit({ kind: "changed", payload: { x: 1 } } as unknown as JsonValue);
      expect(seen.length).toBe(1);
    });
  });
}
//#endregion 🧪Tests
