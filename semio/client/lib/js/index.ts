//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: stateless {@link Session} + GraphQL transport (WASM worker or inline); no client-side kit cache.
//#endregion 🧲Header

//#region 📥KitImports
//#endregion 📥KitImports

export type ID = string

//#region 🌐Transport
/** @emoji 🧵 Bundled worker — Vite resolves `@semio/rs-wasm`; Blob workers cannot import bare specifiers. */
export function createKitStoreWorker(): Worker {
  return new Worker(new URL("./kit-store.worker.ts", import.meta.url), { type: "module" });
}

/** @emoji 🧵 File-local GraphQL wire JSON (not part of the public @semio/js surface). */
type JsonValue = string | number | boolean | null | readonly JsonValue[] | JsonObject;
/** @emoji 🧵 File-local GraphQL wire JSON object node. */
type JsonObject = { readonly [k: string]: JsonValue };

type GraphqlEnvelope<TData> = Readonly<{
  data?: TData | null;
  errors?: readonly { readonly message?: string }[];
}>;

function parseJsonValue(text: string): JsonValue {
  return JSON.parse(text) as JsonValue;
}

function isJsonObjectNode(v: JsonValue | null | undefined): v is JsonObject {
  return v != null && typeof v === "object" && !Array.isArray(v);
}

function jsonObjectField(node: JsonObject | null | undefined, key: string): JsonObject | null {
  const value = node?.[key];
  return value != null && typeof value === "object" && !Array.isArray(value) ? (value as JsonObject) : null;
}

function unwrapGraphqlData<TData>(response: GraphqlEnvelope<TData>): TData {
  if (response == null || typeof response !== "object") throw new Error("graphql: response is not an object");
  if (Array.isArray(response.errors) && response.errors.length > 0) throw new Error(response.errors[0]?.message ?? "GraphQL error");
  const d = response.data;
  if (d != null && typeof d === "object") return d;
  throw new Error("graphql: no data in response");
}

type GraphqlWireKind = "query" | "mutation" | "subscription";

/** @emoji 🔍 SDL operation keyword after leading whitespace and full-line {@code #} comments. */
function graphqlWireOperationKind(document: string): GraphqlWireKind | null {
  let rest = document.trimStart();
  for (; ;) {
    if (rest.startsWith("#")) {
      const nl = rest.indexOf("\n");
      if (nl === -1) return null;
      rest = rest.slice(nl + 1).trimStart();
      continue;
    }
    break;
  }
  const m = rest.match(/^(query|mutation|subscription)\b/);
  if (m?.[1] === "query" || m?.[1] === "mutation" || m?.[1] === "subscription") return m[1] as GraphqlWireKind;
  return null;
}

/** @emoji 🛑 Enforces golden-schema split: {@code Query} vs {@code Mutation} vs {@code Subscription} roots only. */
function assertGraphqlWireKind(document: string, kind: GraphqlWireKind): void {
  const found = graphqlWireOperationKind(document);
  if (found !== kind) throw new Error(`graphql: expected ${kind}, got ${found ?? "unknown"}`);
}

/** @emoji 🧵 Canonical GraphQL-over-HTTP POST object: {@code query}, {@code variables}, {@code operationName} always present on the wire. */
type GraphqlWirePostBody = Readonly<{
  query: string;
  variables: JsonObject;
  operationName: string | null;
}>;

/** @emoji 🧵 Supplies omitted {@code variables} / {@code operationName} so JSON bodies always carry the full triple. */
function normalizeGraphqlWirePostBody(body: {
  readonly query: string;
  readonly variables?: JsonObject;
  readonly operationName?: string | null;
}): GraphqlWirePostBody {
  return {
    query: body.query,
    variables: body.variables ?? {},
    operationName: body.operationName === undefined ? null : body.operationName,
  };
}

/** @emoji 🧵 {@link JSON.stringify} of {@link normalizeGraphqlWirePostBody} for execute/subscribe transports. */
function graphqlWirePostBodyJson(body: {
  readonly query: string;
  readonly variables?: JsonObject;
  readonly operationName?: string | null;
}): string {
  return JSON.stringify(normalizeGraphqlWirePostBody(body));
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

function describeWorkerThreadError(ev: globalThis.Event): string {
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
      const onError = (ev: globalThis.Event) => {
        cleanup();
        reject(new Error(`worker init error: ${describeWorkerThreadError(ev)}`));
      };
      const cleanup = () => {
        clearTimeout(t);
        this.worker.removeEventListener("message", onMessage);
        this.worker.removeEventListener("error", onError as globalThis.EventListener);
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.addEventListener("error", onError as globalThis.EventListener);
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

//#region 🌐HttpStoreTransport
/** @emoji 🌐 GraphQL-over-HTTP to native `semio-store` (no WASM); subscriptions are no-ops until the sidecar exposes a stream. */
class HttpStringTransport {
  constructor(private readonly baseUrl: string) { }

  async execute(requestJson: string): Promise<string> {
    const r = await fetch(`${this.baseUrl}/graphql`, {
      method: "POST",
      headers: { "Content-Type": "application/json", Accept: "application/json" },
      body: requestJson,
    });
    const t = await r.text();
    if (!r.ok) throw new Error(`graphql http ${r.status}: ${t}`);
    return t;
  }

  async subscribe(_requestJson: string, _onEvent: (eventJson: string) => void): Promise<void> {
    return;
  }

  dispose(): void { }
}
//#endregion 🌐HttpStoreTransport

type KitStoreInnerTransport = WorkerStringTransport | InlineTransport | HttpStringTransport;

/** @emoji 🌐 Thin GraphQL JSON transport: request in, JSON string out; pairs with rs {@code KitStoreHandle}. */
export class GqlTransport {
  constructor(private readonly inner: KitStoreInnerTransport) { }

  private async executeWireJson(
    body: { readonly query: string; readonly variables?: JsonObject; readonly operationName?: string | null },
    timeoutMs: number,
  ): Promise<GraphqlEnvelope<JsonValue>> {
    const json = await withTimeout(this.inner.execute(graphqlWirePostBodyJson(body)), timeoutMs, "graphql");
    return parseJsonValue(json) as GraphqlEnvelope<JsonValue>;
  }

  /** @emoji 📖 POST wire aligned with {@code type Query} in {@code schema.golden.graphql}. */
  async executeQueryJson(
    body: { readonly query: string; readonly variables?: JsonObject; readonly operationName?: string | null },
    timeoutMs: number,
  ): Promise<GraphqlEnvelope<JsonValue>> {
    assertGraphqlWireKind(body.query, "query");
    return this.executeWireJson(body, timeoutMs);
  }

  /** @emoji ✍️ POST wire aligned with {@code type Mutation} in {@code schema.golden.graphql}. */
  async executeMutationJson(
    body: { readonly query: string; readonly variables?: JsonObject; readonly operationName?: string | null },
    timeoutMs: number,
  ): Promise<GraphqlEnvelope<JsonValue>> {
    assertGraphqlWireKind(body.query, "mutation");
    return this.executeWireJson(body, timeoutMs);
  }

  /** @emoji 📡 Subscribe wire aligned with {@code type Subscription} in {@code schema.golden.graphql}. */
  async subscribeJson(
    body: { readonly query: string; readonly variables?: JsonObject; readonly operationName?: string | null },
    onEvent: (env: GraphqlEnvelope<JsonValue>) => void,
  ): Promise<void> {
    assertGraphqlWireKind(body.query, "subscription");
    await this.inner.subscribe(graphqlWirePostBodyJson(body), (eventJson) => {
      try {
        onEvent(parseJsonValue(eventJson) as GraphqlEnvelope<JsonValue>);
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

/** @emoji 📡 Demultiplexes live subscription `data` roots into listener fan-out (no client cache). */
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

/** @emoji 📡 Live target {@code Subscription.operation} stream used to match command IDs emitted by mutations. */
export const KIT_EVENT_STREAM_SUBSCRIPTION = `subscription { operation { id __typename } }` as const;

/** @emoji 🧭 Store entry query fragment aligned with {@code schema.golden.graphql} (WIP head + {@code theKit} id). */
export const KIT_SESSION_QUERY_ENTRY = `query KitStoreEntry { session { stores { edges { node { wip { id theKit { id } } } } } } }` as const;

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

export type SessionOpenOptions = Readonly<{
  timeoutMs?: number;
  storeId?: string;
  wasmSpecifier?: string;
  workerFactory?: () => Worker;
}>;

/** @emoji 🌐 Options for {@link Session.openHttp} against `semio-store` (POST `/install` + POST `/graphql`). */
export type SessionHttpOpenOptions = Readonly<SessionOpenOptions & { readonly installCreateDto?: JsonObject }>;

/** @emoji 🧪 Canonical bootstrap URI for an empty in-memory RS kit store. */
export const SEMIO_IN_MEMORY_KIT_URI = "dev://empty" as const;

function gqlString(s: string): string {
  return JSON.stringify(s);
}

function gqlIdList(ids: readonly string[]): string {
  return `[${ids.map((x) => gqlString(x)).join(",")}]`;
}

function scopedKitMutationBody(storeId: string, changeId: string, kitSelection: string): { readonly query: string; readonly variables: JsonObject } {
  return {
    query: `mutation($storeId: ID!, $changeId: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { ${kitSelection} } } } } } }`,
    variables: { storeId, changeId },
  };
}

function sessionStoreSelectionDocument(innerOnStore: string): { query: string; variables: JsonObject } {
  return {
    query: `query Stores { session { stores { edges { cursor node { ${innerOnStore} } } } } }`,
    variables: {},
  };
}

function kitReadSelectionDocument(point: KitReadPoint, innerOnKitStore: string): { query: string; variables: JsonObject } {
  if (isTheKitReadPoint(point)) {
    return {
      query: `query KitSessionWipStore { session { stores { edges { cursor node { wip { theKit { kit { ${innerOnKitStore} } } } } } } } }`,
      variables: {},
    };
  }
  if ("checkpoint" in point) {
    return {
      query: `query KitSessionWipStore($checkpointId: ID!) { session { stores { edges { cursor node { wip { checkpoint(id: $checkpointId) { kit { ${innerOnKitStore} } } } } } } } }`,
      variables: { checkpointId: point.checkpoint.checkpointId },
    };
  }
  if ("alternative" in point) {
    return {
      query: `query KitSessionWipStore($alternativeId: ID!) { session { stores { edges { cursor node { wip { alternative(id: $alternativeId) { kit { ${innerOnKitStore} } } } } } } } }`,
      variables: { alternativeId: point.alternative.alternativeId },
    };
  }
  return {
    query: `query KitSessionWipStore { session { stores { edges { cursor node { wip { theKit { kit { ${innerOnKitStore} } } } } } } } }`,
    variables: {},
  };
}

function sessionStoreEdges(d: JsonValue | null | undefined): readonly JsonObject[] {
  if (d == null || typeof d !== "object" || Array.isArray(d)) return [];
  const session = jsonObjectField(d as JsonObject, "session");
  const stores = jsonObjectField(session, "stores");
  const edges = stores?.["edges"];
  return Array.isArray(edges) ? (edges.filter(isJsonObjectNode) as readonly JsonObject[]) : [];
}

function sessionStoreEdgeId(edge: JsonObject | null | undefined): string {
  return String(edge?.["cursor"] ?? "");
}

function sessionStoreNodeFromData(d: JsonValue | null | undefined, storeId?: string | null): JsonObject | null {
  const edges = sessionStoreEdges(d);
  const edge = storeId == null || storeId === "" ? edges[0] : edges.find((e) => sessionStoreEdgeId(e) === storeId);
  return jsonObjectField(edge, "node");
}

function kitReadSelectionFromData(d: JsonValue | null | undefined, point: KitReadPoint, storeId?: string | null): JsonObject | null {
  const store = sessionStoreNodeFromData(d, storeId);
  const wip = jsonObjectField(store, "wip");
  if (wip == null) return null;
  if ("checkpoint" in point) {
    return jsonObjectField(jsonObjectField(wip, "checkpoint"), "kit");
  }
  if ("alternative" in point) {
    return jsonObjectField(jsonObjectField(wip, "alternative"), "kit");
  }
  return jsonObjectField(jsonObjectField(wip, "theKit"), "kit");
}

async function executeGraphql(
  handle: { execute(requestJson: string): Promise<string> },
  body: { query: string; variables?: JsonObject; operationName?: string | null },
  timeoutMs?: number,
): Promise<GraphqlEnvelope<JsonValue>> {
  const json = await withTimeout(handle.execute(graphqlWirePostBodyJson(body)), timeoutMs ?? 0, "graphql");
  return parseJsonValue(json) as GraphqlEnvelope<JsonValue>;
}

function gqlOkFromEnvelope(env: GraphqlEnvelope<JsonValue>): SetResult {
  if (Array.isArray(env.errors) && env.errors.length > 0) {
    return { ok: false, error: { kind: "Internal", message: env.errors[0]?.message ?? "GraphQL error" } };
  }
  return { ok: true };
}

type GraphqlExecuteHandle = { execute(requestJson: string): Promise<string> };

async function readSemioWasmBytesFromMonorepoCandidates(): Promise<Uint8Array | undefined> {
  try {
    const fs = await import("node:fs/promises");
    const path = await import("node:path");
    const url = await import("node:url");
    const here = path.dirname(url.fileURLToPath(import.meta.url));
    const candidates = [
      path.resolve(here, "../rs/pkg/semio_bg.wasm"),
      path.resolve(here, "../../../../semio/client/lib/rs/pkg/semio_bg.wasm"),
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

function isBrowserWorkerRuntime(): boolean {
  return typeof Worker !== "undefined" && typeof window !== "undefined" && typeof document !== "undefined";
}

function shouldStartLiveSubscriptionLoop(): boolean {
  return isBrowserWorkerRuntime();
}

function defaultRsWasmSpecifier(): string {
  if (typeof window === "undefined" || typeof document === "undefined") {
    return new URL("../rs/pkg/semio.js", import.meta.url).href;
  }
  return "@semio/rs-wasm";
}

//#region 🧱Classes

//#region 🧬Entity

//#region 🛠️Base
/** @emoji 🧬 Strong entity anchor: {@link Session} + id (no cached fields on the instance). */
export abstract class Entity {
  public readonly session: Session;

  protected constructor(session: Session, public readonly id: string, public readonly storeId?: string) {
    this.session = session;
  }

  /** @emoji 🧾 Reads kit fields through the owning {@link Store} scope, never through an active-store fallback. */
  async readKitInner(inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    if (this.storeId == null || this.storeId === "") throw new Error(`${this.constructor.name} is not scoped to a Store`);
    return this.session.readKitInnerForStore(this.storeId, inner, variables);
  }

  /** @emoji 🎬 Starts or reuses the command change ID for the owning {@link Store} scope. */
  async ensureChangeId(): Promise<string> {
    if (this.storeId == null || this.storeId === "") throw new Error(`${this.constructor.name} is not scoped to a Store`);
    return this.session.ensureChangeId(this.storeId);
  }

  /** @emoji 🎬 Sends a kit mutation through the owning {@link Store} command scope. */
  async mutateScoped(changeId: string, kitSelection: string): Promise<SetResult> {
    if (this.storeId == null || this.storeId === "") throw new Error(`${this.constructor.name} is not scoped to a Store`);
    return this.session.mutateScoped(this.storeId, changeId, kitSelection);
  }

  protected entity<T extends Entity>(ctor: new (session: Session, id: string, storeId?: string) => T, id: string, storeId = this.storeId): T {
    return new ctor(this.session, id, storeId);
  }
}
//#endregion 🛠️Base

//#region 🪶WeakArtifacts
/** @emoji 🪪 Weak {@link Attribute} anchored on an owning {@link Entity} (no separate {@code node(id:)} identity). */
export class Attribute {
  constructor(
    public readonly owner: Entity,
    public readonly id: string,
    public readonly key: string,
    public readonly value: string | null,
    public readonly definition: string,
  ) { }

  get session(): Session {
    return this.owner.session;
  }
}

/** @emoji 🏁 Weak {@link Benchmark} under {@link Quality}. */
export class Benchmark {
  constructor(
    public readonly quality: Quality,
    public readonly id: string,
    public readonly name: string,
    public readonly min: number | null,
    public readonly max: number | null,
    public readonly minExcluded: boolean | null,
    public readonly maxExcluded: boolean | null,
  ) { }

  get session(): Session {
    return this.quality.session;
  }
}

/** @emoji 🧬 Union anchor for VCS scope / modification ends (narrow at callsites). */
export type EntityRef = Entity;
//#endregion 🪶WeakArtifacts

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

type KitPathEntity = Entity & {
  kitInnerPath(inner: string): string;
};

type BoundKitFieldSpec<T, E extends KitPathEntity = KitPathEntity> = FieldSpec<T> & Readonly<{
  parseEntity?: (entity: E, v: JsonValue) => T;
  /** @emoji 📡 Bus {@code kind}; defaults via {@link defaultFieldEventKind}. */
  eventKind?: string;
  /** @emoji 📡 List/connection fields: invalidate on {@code commandSucceeded} + {@code kitRenamed}. */
  coarseEvent?: boolean;
}>;

type BoundNodeFieldSpec<T> = Readonly<{
  selection: string;
  parse: (node: JsonObject | undefined) => T;
}>;

type BoundKitOperationSpec<E extends KitPathEntity> = Readonly<{
  buildInner: (entity: E, ...args: readonly unknown[]) => string;
}>;

/** @emoji 🏭 Metadata-only field list (tooling / docs); reads use entity methods. */
export function defineFields<const S extends readonly FieldSpec<unknown>[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Metadata-only operation list (tooling / docs); writes use entity methods. */
export function defineOperations<const S extends readonly OperationSpec[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Metadata-only bound field list used to install prototype methods from one schema-like roster. */
function defineBoundKitFields<const S extends readonly BoundKitFieldSpec<unknown>[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Metadata-only node field list used to install {@code node(id)} readers from one roster. */
function defineBoundNodeFields<const S extends readonly BoundNodeFieldSpec<unknown>[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Metadata-only bound operation list used to install mutation methods from one roster. */
function defineBoundKitOperations<const S extends readonly BoundKitOperationSpec<KitPathEntity>[]>(specs: S): S {
  return specs;
}

function schemaFieldName(selection: string): string {
  const name = selection.trim().match(/^[_A-Za-z][_0-9A-Za-z]*/)?.[0];
  if (name == null || name === "") throw new Error(`Invalid GraphQL field selection: ${selection}`);
  return name;
}

function schemaOperationName(inner: string): string {
  const name = inner.trim().match(/^(?:[_A-Za-z][_0-9A-Za-z]*:\s*)?([_A-Za-z][_0-9A-Za-z]*)/)?.[1];
  if (name == null || name === "") throw new Error(`Invalid GraphQL operation selection: ${inner}`);
  return name;
}

/** @emoji 📡 GraphQL {@code Operation} {@code __typename} → {@link EventBus} {@code kind} (react hooks rely on these strings). */
function operationTypenameToEventKind(typename: string): string {
  switch (typename) {
    case "RenamedKit":
      return "kitRenamed";
    case "ChangedDescription":
      return "changedDescription";
    case "CreatedFixedPiece":
      return "createdFixedPiece";
    case "FixedPiece":
      return "fixedPiece";
    case "DraggedPiece":
      return "draggedPiece";
    default:
      return typename.charAt(0).toLowerCase() + typename.slice(1);
  }
}

/** @emoji 📡 Installed {@code onFieldChanged} method name for a scalar/list field (e.g. {@code onNameChanged}). */
function fieldChangedEventMethodName(fieldName: string): string {
  return `on${fieldName.charAt(0).toUpperCase()}${fieldName.slice(1)}Changed`;
}

/** @emoji 📡 Default bus kind for a field read on an entity class. */
function defaultFieldEventKind(entityCtorName: string, fieldName: string): string {
  if (entityCtorName === "Kit" && fieldName === "name") return "kitRenamed";
  if (fieldName === "description") return "changedDescription";
  return "commandSucceeded";
}

/** @emoji 🏭  a field read when the caller supplies the kit-relative GraphQL tail. */
export function defineField<E extends Entity, T>(entity: E, spec: FieldSpec<T>, pathInKit: (self: E) => string): () => Promise<T> {
  return async () => {
    const frag = await entity.readKitInner(pathInKit(entity));
    return spec.parse(frag as JsonValue);
  };
}

/** @emoji 🏭  a mutation leaf using {@link Store#mutateScoped}. */
export function defineOperation(entity: Entity, spec: OperationSpec, buildPath: (self: Entity) => string): () => Promise<SetResult> {
  return async () => {
    void spec;
    const cid = await entity.ensureChangeId();
    return entity.mutateScoped(cid, buildPath(entity));
  };
}

/** @emoji 🏭 Installs kit-relative read methods on a prototype so classes stay declarative and schema-shaped. */
function installKitFieldMethods<E extends KitPathEntity>(
  ctor: abstract new (...args: never[]) => E,
  specs: readonly BoundKitFieldSpec<unknown, E>[],
): void {
  for (const spec of specs) {
    Object.defineProperty(ctor.prototype, schemaFieldName(spec.selection), {
      configurable: true,
      value: async function semioKitField(this: E): Promise<unknown> {
        const frag = await this.readKitInner(this.kitInnerPath(spec.selection));
        return spec.parseEntity != null ? spec.parseEntity(this, frag as JsonValue) : spec.parse(frag as JsonValue);
      },
      writable: true,
    });
  }
}

async function readNodeSelection<T>(
  entity: Entity,
  typename: string,
  selection: string,
  parse: (node: JsonObject | undefined) => T,
): Promise<T> {
  const data = unwrapGraphqlData(
    await executeSessionReadGraphql(entity.session, {
      query: `query($id: ID!) { node(id: $id) { ... on ${typename} { ${selection} } } }`,
      variables: { id: entity.id },
    }),
  ) as JsonObject;
  return parse(data["node"] as JsonObject | undefined);
}

/** @emoji 🏭 Installs {@code node(id)}-based read methods on a prototype from one typed roster. */
function installNodeFieldMethods<E extends Entity>(
  ctor: abstract new (...args: never[]) => E,
  typename: string,
  specs: readonly BoundNodeFieldSpec<unknown>[],
): void {
  for (const spec of specs) {
    Object.defineProperty(ctor.prototype, schemaFieldName(spec.selection), {
      configurable: true,
      value: function semioNodeField(this: E): Promise<unknown> {
        return readNodeSelection(this, typename, spec.selection, spec.parse);
      },
      writable: true,
    });
  }
}

/** @emoji 🏭 Installs kit-relative mutation methods on a prototype from one operation roster. */
function installKitOperationMethods<E extends KitPathEntity>(
  ctor: abstract new (...args: never[]) => E,
  specs: readonly BoundKitOperationSpec<E>[],
): void {
  for (const spec of specs) {
    Object.defineProperty(ctor.prototype, schemaOperationName(spec.buildInner({} as E)), {
      configurable: true,
      value: async function semioKitOperation(this: E, ...args: readonly unknown[]): Promise<SetResult> {
        const cid = await this.ensureChangeId();
        return this.mutateScoped(cid, this.kitInnerPath(spec.buildInner(this, ...args)));
      },
      writable: true,
    });
  }
}

/** @emoji 🏭 Installs {@code onFieldChanged} subscription methods — one per field spec. */
function installKitEventMethods<E extends KitPathEntity>(
  ctor: abstract new (...args: never[]) => E,
  specs: readonly BoundKitFieldSpec<unknown, E>[],
): void {
  const entityName = ctor.name;
  for (const spec of specs) {
    const fieldName = schemaFieldName(spec.selection);
    const readMethod = fieldName;
    const eventMethod = fieldChangedEventMethodName(fieldName);
    const eventKind = spec.eventKind ?? defaultFieldEventKind(entityName, fieldName);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioKitFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const run = (): void => {
          const read = (this as Record<string, () => Promise<unknown>>)[readMethod];
          if (typeof read !== "function") return;
          void read.call(this).then(cb);
        };
        if (spec.coarseEvent) return subscribeKitCoarseRefetch(this.session.bus, run);
        return this.session.bus.subscribeKind(eventKind, () => run());
      },
      writable: true,
    });
  }
}

/** @emoji 🏭 Installs kit field reads, mutation commands, and per-field change subscriptions from three rosters. */
function installEntityKitMethods<E extends KitPathEntity>(
  ctor: abstract new (...args: never[]) => E,
  fields: readonly BoundKitFieldSpec<unknown, E>[],
  operations: readonly BoundKitOperationSpec<E>[] = [],
): void {
  installKitFieldMethods(ctor, fields);
  if (operations.length > 0) installKitOperationMethods(ctor, operations);
  installKitEventMethods(ctor, fields);
}

/** @emoji 🏭 Installs {@code node(id)} field reads and per-field change subscriptions. */
function installEntityNodeMethods<E extends Entity>(
  ctor: abstract new (...args: never[]) => E,
  typename: string,
  fields: readonly BoundNodeFieldSpec<unknown>[],
): void {
  installNodeFieldMethods(ctor, typename, fields);
  for (const spec of fields) {
    const fieldName = schemaFieldName(spec.selection);
    const eventMethod = fieldChangedEventMethodName(fieldName);
    const eventKind = defaultFieldEventKind(ctor.name, fieldName);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioNodeFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const read = (this as Record<string, () => Promise<unknown>>)[fieldName];
        return this.session.bus.subscribeKind(eventKind, () => {
          if (typeof read === "function") void read.call(this).then(cb);
        });
      },
      writable: true,
    });
  }
}

//#endregion 🏭Factories

//#region 🧩Parsers
/** @emoji 🧩 Parses {@code attributes { edges { node { … } } }} under a JSON object (e.g. {@code tag}, {@code node}). */
function parseAttributeConnectionUnder(ownerEntity: Entity, owner: JsonObject | null | undefined): readonly Attribute[] {
  const attrs = owner?.["attributes"] as JsonObject | undefined;
  const edges = attrs?.["edges"] as readonly JsonValue[] | undefined;
  if (!Array.isArray(edges)) return [];
  const out: Attribute[] = [];
  for (const e of edges) {
    if (!isJsonObjectNode(e)) continue;
    const n = e["node"] as JsonObject | undefined;
    if (n == null) continue;
    out.push(
      new Attribute(
        ownerEntity,
        String(n["id"] ?? ""),
        String(n["key"] ?? ""),
        n["value"] == null ? null : String(n["value"]),
        String(n["definition"] ?? ""),
      ),
    );
  }
  return out;
}

//#region 📦KitBranch
/** @emoji 📦 String field from nested kit JSON (e.g. `{ design: { name } }` or flattened `{ name }`). */
function readKitBranchString(frag: JsonObject | null | undefined, branchKey: string, field: string): string {
  const branch = frag?.[branchKey] as JsonObject | undefined;
  const v = branch?.[field] ?? frag?.[field];
  return String(v ?? "");
}

/** @emoji 📦 Numeric field from nested kit JSON (e.g. {@code qualitySum}). */
function readKitBranchNumber(frag: JsonObject | null | undefined, branchKey: string, field: string): number {
  const branch = frag?.[branchKey] as JsonObject | undefined;
  const raw = branch?.[field] ?? frag?.[field];
  return typeof raw === "number" ? raw : Number(raw ?? NaN);
}

/** @emoji 📦 Nullable number on a nested branch (e.g. {@code Tag.order}). */
function readKitBranchNumberOrNull(frag: JsonObject | null | undefined, branchKey: string, field: string): number | null {
  const branch = frag?.[branchKey] as JsonObject | undefined;
  const raw = branch?.[field] ?? frag?.[field];
  return typeof raw === "number" ? raw : null;
}
//#endregion 📦KitBranch

//#region 🧭KitPath
/** @emoji 🧭 Descends string keys on kit JSON; returns undefined if any step is not an object. */
function readKitPathNode(frag: JsonObject | null | undefined, path: readonly string[]): JsonObject | undefined {
  let cur: JsonValue | undefined = frag;
  for (const p of path) {
    if (!isJsonObjectNode(cur)) return undefined;
    cur = cur[p];
  }
  return isJsonObjectNode(cur) ? cur : undefined;
}

/** @emoji 🧭 String scalar at {@code path} then {@code field} (e.g. {@code type → port → code}). */
function readKitPathString(frag: JsonObject | null | undefined, path: readonly string[], field: string): string {
  const n = readKitPathNode(frag, path);
  const v = n?.[field];
  return v == null ? "" : String(v);
}

/** @emoji 🧭 Nullable number at path end (e.g. {@code Port.order}). */
function readKitPathNumberOrNull(frag: JsonObject | null | undefined, path: readonly string[], field: string): number | null {
  const n = readKitPathNode(frag, path);
  const v = n?.[field];
  return typeof v === "number" ? v : null;
}
//#endregion 🧭KitPath

//#region 📡BusCoarse
/** @emoji 📡 Coarse invalidation pair used for kit-scoped list refetches ({@code commandSucceeded} + {@code kitRenamed}). */
function subscribeKitCoarseRefetch(
  bus: { subscribeKind(kind: string, fn: () => void): Unsubscribe },
  run: () => void,
): Unsubscribe {
  const a = bus.subscribeKind("commandSucceeded", run);
  const b = bus.subscribeKind("kitRenamed", run);
  return (): void => {
    a();
    b();
  };
}
//#endregion 📡BusCoarse
//#endregion 🧩Parsers
//#endregion 🧬Entity


//#region 🏪Store
/** @emoji 🔗 Map `Store.open` input: inline JSON becomes `dev+json:` base64 for the WASM bootstrap URI. */
function backboneBootstrapUriForStoreOpen(raw: string): string {
  const t = raw.trim();
  if (t.startsWith("{") || t.startsWith("[")) {
    return semioJsonBootstrapUri(t);
  }
  return t;
}

/** @emoji 🧪 Encodes inline kit JSON into the RS `dev+json:` bootstrap URI form. */
export function semioJsonBootstrapUri(raw: string): string {
  const bytes = new TextEncoder().encode(raw);
  let bin = "";
  for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i]!);
  return `dev+json:${btoa(bin)}`;
}

/** @emoji 📑 Same JSON shape as {@link FieldSpec} but declared before {@link Entity} so {@link Store} field reads stay self-contained. */
export type KitFieldReadSpec<T> = Readonly<{
  eventKind?: string;
  selection: string;
  parse: (v: JsonValue) => T;
}>;

function parseEntityConnectionIds(frag: JsonObject | null | undefined, key: string): readonly string[] {
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

/** @emoji 🧩 Parses {@code key: [{ id: … }]} non-relay {@code [StrongEntity!]} lists on a JSON object (e.g. {@code Checkpoint.changes}). */
function parseStrongEntityArrayIds(frag: JsonObject | null | undefined, key: string): readonly string[] {
  const arr = frag?.[key] as readonly JsonValue[] | undefined;
  if (!Array.isArray(arr)) return [];
  const out: string[] = [];
  for (const item of arr) {
    if (!isJsonObjectNode(item)) continue;
    const id = String(item["id"] ?? "");
    if (id !== "") out.push(id);
  }
  return out;
}

/**
 * @emoji 🧭 Target {@code Session}: owns GraphQL transport and only tracks command IDs for {@code Subscription.operation} correlation; {@link EventBus} emits {@code commandSucceeded} only (no duplicate event-kind payloads).
 */
export class Session {
  private readonly timeoutMs: number;
  private readonly handle: GraphqlExecuteHandle;
  private readonly innerTransport: KitStoreInnerTransport;
  private gqlLoopRunning = false;
  private disposed = false;
  private readonly commandIds = new Set<string>();

  /** @emoji 🌐 GraphQL executor (JSON in/out). */
  readonly gql: GqlTransport;
  /** @emoji 📡 Demuxed subscription fan-out. */
  readonly bus: EventBus;

  private constructor(timeoutMs: number, inner: KitStoreInnerTransport) {
    this.timeoutMs = timeoutMs;
    this.innerTransport = inner;
    this.handle = { execute: (j) => inner.execute(j) };
    this.gql = new GqlTransport(inner);
    this.bus = new EventBus();
  }

  private ensureAlive(): void {
    if (this.disposed) throw new Error("Session disposed");
  }

  private dispatchSubscriptionGraphqlData(data: JsonObject | null | undefined): void {
    if (data == null) return;
    const operation = jsonObjectField(data, "operation");
    const id = String(operation?.["id"] ?? "");
    const typename = String(operation?.["__typename"] ?? "");
    if (typename !== "") {
      this.bus.emit({ kind: operationTypenameToEventKind(typename), payload: operation } as unknown as JsonValue);
    }
    if (id === "" || !this.commandIds.has(id)) return;
    this.commandIds.delete(id);
    this.bus.emit({ kind: "commandSucceeded", payload: operation } as unknown as JsonValue);
  }

  private startSubscriptionLoop(): void {
    if (this.gqlLoopRunning) return;
    this.gqlLoopRunning = true;
    void this.gql
      .subscribeJson({ query: KIT_EVENT_STREAM_SUBSCRIPTION }, (msg) => {
        try {
          if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return;
          const subscriptionData = msg.data;
          if (subscriptionData == null || typeof subscriptionData !== "object") return;
          this.dispatchSubscriptionGraphqlData(subscriptionData as JsonObject);
        } catch {
          /* ignore */
        }
      })
      .catch(() => {
        this.gqlLoopRunning = false;
      });
  }

  private async readEnvelope(body: { query: string; variables?: JsonObject; operationName?: string | null }): Promise<GraphqlEnvelope<JsonValue>> {
    this.ensureAlive();
    assertGraphqlWireKind(body.query, "query");
    return executeGraphql(this.handle, body, this.timeoutMs);
  }

  private async mutateEnvelope(body: { query: string; variables?: JsonObject; operationName?: string | null }): Promise<GraphqlEnvelope<JsonValue>> {
    this.ensureAlive();
    assertGraphqlWireKind(body.query, "mutation");
    return executeGraphql(this.handle, body, this.timeoutMs);
  }

  /** @emoji 🧾 Applies a mutation envelope and registers emitted operation ids for {@code Subscription.operation}. */
  mutationReceipt(env: GraphqlEnvelope<JsonValue>): SetResult {
    this.ensureAlive();
    return this.trackCommandResult(env);
  }

  /** @emoji 🧾 Reads a selection inside a specific store's scoped {@code kit { … }} through GraphQL. */
  async readKitInnerForStore(storeId: string, inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    const { query, variables: v0 } = kitReadSelectionDocument(theKitReadPoint, inner);
    const data = unwrapGraphqlData(await this.readEnvelope({ query, variables: { ...v0, ...variables } })) as JsonValue;
    return kitReadSelectionFromData(data, theKitReadPoint, storeId);
  }

  async readKitInner(_inner: string, _variables: JsonObject = {}): Promise<JsonObject | null> {
    throw new Error("store id is required; use session.store(id).readKitInner(...)");
  }

  async readStoreInner(_inner: string, _variables: JsonObject = {}): Promise<JsonObject | null> {
    throw new Error("store id is required; use session.store(id).readStoreInner(...)");
  }

  /** @emoji 🧾 Reads a selection inside a specific target {@code Store} edge. */
  async readStoreInnerForId(storeId: string, inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    const { query, variables: v0 } = sessionStoreSelectionDocument(inner);
    const data = unwrapGraphqlData(await this.readEnvelope({ query, variables: { ...v0, ...variables } })) as JsonValue;
    return sessionStoreNodeFromData(data, storeId);
  }

  /** @emoji 🧾 Runs a store-scoped kit mutation through {@code SessionCommand.store(id:)}. */
  async mutateScoped(storeId: string, changeId?: string, kitSelection?: string): Promise<SetResult> {
    this.ensureAlive();
    if (changeId == null || kitSelection == null) throw new Error("store id is required for store-scoped mutation");
    const { query, variables } = scopedKitMutationBody(storeId, changeId, kitSelection);
    const env = await this.mutateEnvelope({ query, variables });
    return this.trackCommandResult(env);
  }

  store(storeId: string): Store {
    return new Store(this, storeId);
  }

  private async sessionStoreIds(): Promise<readonly string[]> {
    const data = unwrapGraphqlData(await this.readEnvelope({ query: `query { session { stores { edges { cursor } } } }` })) as JsonValue;
    return sessionStoreEdges(data).map(sessionStoreEdgeId).filter((id) => id !== "");
  }

  async stores(): Promise<readonly Store[]> {
    const ids = await this.sessionStoreIds();
    return Object.freeze(ids.map((id) => this.store(id)));
  }

  localProvider(): LocalProvider {
    return new LocalProvider(this);
  }

  remoteProvider(url: string): RemoteProvider {
    return new RemoteProvider(this, url);
  }

  private async remoteProviderUrls(): Promise<readonly string[]> {
    const data = unwrapGraphqlData(
      await this.readEnvelope({ query: `query { session { remoteProviders { edges { node { url } } } } }` }),
    ) as JsonObject;
    const session = jsonObjectField(data, "session");
    const edges = jsonObjectField(session, "remoteProviders")?.["edges"];
    if (!Array.isArray(edges)) return [];
    return edges.map((e) => String(((e as JsonObject)["node"] as JsonObject | undefined)?.["url"] ?? "")).filter((url) => url !== "");
  }

  async remoteProviders(): Promise<readonly RemoteProvider[]> {
    const urls = await this.remoteProviderUrls();
    return Object.freeze(urls.map((url) => this.remoteProvider(url)));
  }

  private trackCommandId(id: string): string {
    if (id !== "") this.commandIds.add(id);
    return id;
  }

  private trackCommandResult(env: GraphqlEnvelope<JsonValue>): SetResult {
    const visit = (value: JsonValue | undefined): string => {
      if (typeof value === "string" && value !== "") return value;
      if (Array.isArray(value)) {
        for (const item of value) {
          const id = visit(item);
          if (id !== "") return id;
        }
      } else if (isJsonObjectNode(value)) {
        for (const item of Object.values(value)) {
          const id = visit(item);
          if (id !== "") return id;
        }
      }
      return "";
    };
    this.trackCommandId(visit(env.data ?? undefined));
    return gqlOkFromEnvelope(env);
  }

  async ensureChangeId(storeId?: string): Promise<string> {
    this.ensureAlive();
    if (storeId == null || storeId === "") throw new Error("store id is required for store-scoped change");
    const data = unwrapGraphqlData(await this.mutateEnvelope({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange } } } }`, variables: { storeId } })) as JsonObject;
    const sess = data["session"] as JsonObject | undefined;
    const store = sess?.["store"] as JsonObject | undefined;
    const tk = store?.["theKit"] as JsonObject | undefined;
    const cid = String(tk?.["startNewChange"] ?? "");
    if (cid === "") throw new Error("startNewChange: empty change id");
    return this.trackCommandId(cid);
  }

  async saveChange(storeId?: string): Promise<void> {
    this.ensureAlive();
    if (storeId == null || storeId === "") throw new Error("store id is required for store-scoped save");
    const data = unwrapGraphqlData(await this.mutateEnvelope({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { save } } } }`, variables: { storeId } })) as JsonObject;
    const id = String(((data["session"] as JsonObject | undefined)?.["store"] as JsonObject | undefined)?.["theKit"] == null ? "" : (((data["session"] as JsonObject)["store"] as JsonObject)["theKit"] as JsonObject)["save"] ?? "");
    this.trackCommandId(id);
  }

  async startNewChange(storeId: string): Promise<ChangeId> {
    return await this.ensureChangeId(storeId);
  }

  async createCheckpoint(storeId: string, message: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { createCheckpoint(message: ${gqlString(message)}) } } } }`, variables: { storeId } });
    return this.trackCommandResult(env);
  }

  async startAlternative(storeId: string, name?: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query:
        name == null
          ? `mutation($storeId: ID!) { session { store(id: $storeId) { startAlternative } } }`
          : `mutation($storeId: ID!) { session { store(id: $storeId) { startAlternative(name: ${gqlString(name)}) } } }`,
      variables: { storeId },
    });
    return this.trackCommandResult(env);
  }

  async integrateAlternative(storeId: string, alternativeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { alternative(id: ${gqlString(alternativeId)}) { integrateIntoTheKit } } } }`,
      variables: { storeId },
    });
    return this.trackCommandResult(env);
  }

  async login(username: string, passwordHash: string, hubUrl?: string): Promise<SetResult> {
    this.ensureAlive();
    const url = hubUrl ?? "";
    const h = hubUrl == null ? "null" : gqlString(hubUrl);
    const env = await this.mutateEnvelope({
      query: `mutation { session { remoteProvider(url: ${gqlString(url)}) { login(username: ${gqlString(username)}, passwordHash: ${gqlString(passwordHash)}, hubUrl: ${h}) } } }`,
    });
    return this.trackCommandResult(env);
  }

  async logout(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation { session { remoteProvider(url: "") { logout } } }` });
    return this.trackCommandResult(env);
  }

  async sessionStart(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation { session { start } }` });
    return this.trackCommandResult(env);
  }

  async sessionEnd(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation { session { end } }` });
    return this.trackCommandResult(env);
  }

  async attachBackbone(storeId: string, provider: Provider, uri: string): Promise<SetResult> {
    return provider.ensureBackboneAttached(storeId, uri);
  }

  async detachBackbone(storeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { detach } } } }`, variables: { storeId } });
    return this.trackCommandResult(env);
  }

  /** @emoji 🛜 Runs target {@code BackboneCommand.sync} through the given store command scope. */
  async backboneSyncNow(storeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { sync } } } }`, variables: { storeId } });
    return this.trackCommandResult(env);
  }

  /** @emoji 🛜 Reads {@code BackboneStatus} via the command shell (typed snapshot, not raw JSON). */
  async backboneStatus(storeId: string): Promise<Readonly<{ attachedUri: string | null; kind: string }>> {
    this.ensureAlive();
    const data = unwrapGraphqlData(
      await this.readEnvelope({ query: `query { session { stores { edges { node { authoritative { id } conflicts { edges { node { id } } } } } } } }` }),
    ) as JsonObject;
    const st = sessionStoreNodeFromData(data, storeId);
    return {
      attachedUri: null,
      kind: st?.["authoritative"] == null ? "OFFLINE" : "ONLINE",
    };
  }

  /** @emoji 🧾 Warm-path query after WASM init. */
  private async warmGraphqlRead(): Promise<void> {
    const stores = await this.stores();
    if (stores.length > 0) await stores[0]!.readKitInner("id name");
  }

  static async open(uri: string, opts?: SessionOpenOptions): Promise<Session> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const wasmSpecifier = opts?.wasmSpecifier ?? (globalThis as { __SEMIO_WASM_SPECIFIER__?: string }).__SEMIO_WASM_SPECIFIER__ ?? defaultRsWasmSpecifier();
    const preferInlineInVitest = (() => {
      try {
        const env = (import.meta as { env?: JsonObject }).env;
        if (env && Boolean(env["VITEST"])) return true;
      } catch {
        /* ignore */
      }
      return typeof process !== "undefined" && !!process.env && "VITEST" in process.env;
    })();

    const wasmBytesPre = await readSemioWasmBytesFromMonorepoCandidates();
  const useDedicatedWorker = isBrowserWorkerRuntime() && !preferInlineInVitest && wasmBytesPre == null;

    const bootstrapUri = backboneBootstrapUriForStoreOpen(uri);
    if (useDedicatedWorker) {
      const worker = opts?.workerFactory?.() ?? createKitStoreWorker();
      const wt = new WorkerStringTransport(worker);
      try {
        await wt.init(bootstrapUri);
        const k = new Session(timeoutMs, wt);
        await withTimeout(k.warmGraphqlRead(), timeoutMs, "graphql");
        if (shouldStartLiveSubscriptionLoop()) void k.startSubscriptionLoop();
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
      throw new Error(`Failed to load @semio/rs-wasm (inline path): ${base}`, { cause: e });
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
    const k = new Session(timeoutMs, t);
    await withTimeout(k.warmGraphqlRead(), timeoutMs, "graphql");
    if (shouldStartLiveSubscriptionLoop()) void k.startSubscriptionLoop();
    return k;
  }

  /** @emoji 🧪 Opens the RS-backed empty in-memory kit store used for local bridge and UI smoke paths. */
  static async openInMemory(opts?: SessionOpenOptions): Promise<Session> {
    return Session.open(SEMIO_IN_MEMORY_KIT_URI, opts);
  }

  /** @emoji 🧪 Opens an RS-backed session from inline kit or bundle JSON via `dev+json:`. */
  static async openJson(raw: string, opts?: SessionOpenOptions): Promise<Session> {
    return Session.open(semioJsonBootstrapUri(raw), opts);
  }

  /** @emoji 🌐 Opens a {@link Session} against native `semio-store` at {@code baseUrl} (optional POST `/install` first). */
  static async openHttp(baseUrl: string, opts?: SessionHttpOpenOptions): Promise<Session> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const root = baseUrl.replace(/\/$/, "");
    if (opts?.installCreateDto != null) {
      const r = await fetch(`${root}/install`, {
        method: "POST",
        headers: { "Content-Type": "application/json", Accept: "application/json" },
        body: JSON.stringify({ create: { dto: opts.installCreateDto } }),
      });
      if (!r.ok) throw new Error(`semio-store install ${r.status}: ${await r.text()}`);
    }
    const inner = new HttpStringTransport(root);
    const k = new Session(timeoutMs, inner);
    await withTimeout(k.warmGraphqlRead(), timeoutMs, "graphql");
    if (shouldStartLiveSubscriptionLoop()) void k.startSubscriptionLoop();
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
}

//#endregion 🏪Store

//#region 📦Kit
/** @emoji 📦 Target-schema kit entity beneath {@link Version}; delegates transport work to {@link Store}. */
export class Kit extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `rn: rename(newName: ${gqlString(newName)})`);
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `cd: changeDescription(newDescription: ${gqlString(newDescription)})`);
  }

  async createTag(name: string, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : gqlString(description);
    const ic = icon == null ? "null" : gqlString(icon);
    const ord = order == null ? "null" : String(order);
    return this.mutateScoped(cid, `ct: createTag(name: ${gqlString(name)}, description: ${d}, icon: ${ic}, order: ${ord})`);
  }

  async deleteTag(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dt: deleteTag(id: ${gqlString(id)})`);
  }

  async deleteTags(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dts: deleteTags(ids: ${gqlIdList(ids)})`);
  }

  async createConcept(name: string, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : gqlString(description);
    const ic = icon == null ? "null" : gqlString(icon);
    const ord = order == null ? "null" : String(order);
    return this.mutateScoped(cid, `cc: createConcept(name: ${gqlString(name)}, description: ${d}, icon: ${ic}, order: ${ord})`);
  }

  async deleteConcept(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dc: deleteConcept(id: ${gqlString(id)})`);
  }

  async deleteConcepts(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dcs: deleteConcepts(ids: ${gqlIdList(ids)})`);
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
    const va = value == null ? "null" : gqlString(value);
    const un = unit == null ? "null" : gqlString(unit);
    const de = definition == null ? "null" : gqlString(definition);
    const ds = description == null ? "null" : gqlString(description);
    const ic = icon == null ? "null" : gqlString(icon);
    return this.mutateScoped(
      cid,
      `cq: createQuality(key: ${gqlString(key)}, value: ${va}, unit: ${un}, definition: ${de}, description: ${ds}, icon: ${ic})`,
    );
  }

  async deleteQuality(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dq: deleteQuality(id: ${gqlString(id)})`);
  }

  async deleteQualities(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dqs: deleteQualities(ids: ${gqlIdList(ids)})`);
  }

  async createType(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : gqlString(description);
    const ic = icon == null ? "null" : gqlString(icon);
    const im = image == null ? "null" : gqlString(image);
    const u = unit == null ? "null" : gqlString(unit);
    return this.mutateScoped(cid, `cT: createType(name: ${gqlString(name)}, description: ${d}, icon: ${ic}, image: ${im}, unit: ${u})`);
  }

  async deleteType(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dT: deleteType(id: ${gqlString(id)})`);
  }

  async deleteTypes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dTs: deleteTypes(ids: ${gqlIdList(ids)})`);
  }

  async createDesign(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : gqlString(description);
    const ic = icon == null ? "null" : gqlString(icon);
    const im = image == null ? "null" : gqlString(image);
    const u = unit == null ? "null" : gqlString(unit);
    return this.mutateScoped(cid, `cD: createDesign(name: ${gqlString(name)}, description: ${d}, icon: ${ic}, image: ${im}, unit: ${u})`);
  }

  async deleteDesign(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dD: deleteDesign(id: ${gqlString(id)})`);
  }

  async deleteDesigns(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, `dDs: deleteDesigns(ids: ${gqlIdList(ids)})`);
  }

  async name(): Promise<string> {
    const frag = await this.readKitInner("name");
    return String(frag?.["name"] ?? "");
  }

  async description(): Promise<string> {
    const frag = await this.readKitInner("description");
    return String(frag?.["description"] ?? "");
  }

  async icon(): Promise<string> {
    const frag = await this.readKitInner("icon");
    return String(frag?.["icon"] ?? "");
  }

  async image(): Promise<string> {
    const frag = await this.readKitInner("image");
    return String(frag?.["image"] ?? "");
  }

  async preview(): Promise<string> {
    const frag = await this.readKitInner("preview");
    return String(frag?.["preview"] ?? "");
  }

  async remote(): Promise<string> {
    const frag = await this.readKitInner("remote");
    return String(frag?.["remote"] ?? "");
  }

  async homepage(): Promise<string> {
    const frag = await this.readKitInner("homepage");
    return String(frag?.["homepage"] ?? "");
  }

  async license(): Promise<string> {
    const frag = await this.readKitInner("license");
    return String(frag?.["license"] ?? "");
  }

  async uri(): Promise<string> {
    const frag = await this.readKitInner("uri");
    return String(frag?.["uri"] ?? "");
  }

  async designs(): Promise<readonly Design[]> {
    const frag = await this.readKitInner("designs { edges { node { id } } }");
    return Object.freeze(parseEntityConnectionIds(frag, "designs").map((id) => this.entity(Design, id)));
  }

  async types(): Promise<readonly Type[]> {
    const frag = await this.readKitInner("types { edges { node { id } } }");
    return Object.freeze(parseEntityConnectionIds(frag, "types").map((id) => this.entity(Type, id)));
  }

  async authors(): Promise<readonly Author[]> {
    const frag = await this.readKitInner("authors { edges { node { id } } }");
    return Object.freeze(parseEntityConnectionIds(frag, "authors").map((id) => this.entity(Author, id)));
  }

  async qualities(): Promise<readonly Quality[]> {
    const frag = await this.readKitInner("qualities { edges { node { id } } }");
    return Object.freeze(parseEntityConnectionIds(frag, "qualities").map((id) => this.entity(Quality, id)));
  }

  async tags(): Promise<readonly Tag[]> {
    const frag = await this.readKitInner("tags { edges { node { id } } }");
    return Object.freeze(parseEntityConnectionIds(frag, "tags").map((id) => this.entity(Tag, id)));
  }

  async concepts(): Promise<readonly Concept[]> {
    const frag = await this.readKitInner("concepts { edges { node { id } } }");
    return Object.freeze(parseEntityConnectionIds(frag, "concepts").map((id) => this.entity(Concept, id)));
  }
}
//#endregion 📦Kit

function executeSessionReadGraphql(
  session: Session,
  body: Readonly<{ query: string; variables?: JsonObject; operationName?: string | null }>,
): Promise<GraphqlEnvelope<JsonValue>> {
  return (session as unknown as { readEnvelope(b: typeof body): Promise<GraphqlEnvelope<JsonValue>> }).readEnvelope(body);
}

function executeSessionWriteGraphql(
  session: Session,
  body: Readonly<{ query: string; variables?: JsonObject; operationName?: string | null }>,
): Promise<GraphqlEnvelope<JsonValue>> {
  return (session as unknown as { mutateEnvelope(b: typeof body): Promise<GraphqlEnvelope<JsonValue>> }).mutateEnvelope(body);
}

//#region 🧬VcsEntities
/** @emoji 🌐 WIP or authoritative {@code Graph} root from {@code Query}. */
export type GraphRootKind = "wip" | "authoritative";

/** @emoji 🏪 Target store selected from {@code Session.stores.edges.cursor}. */
export class Store extends Entity {
  constructor(session: Session, id: string) {
    super(session, id);
  }

  async readKitInner(inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    return this.session.readKitInnerForStore(this.id, inner, variables);
  }

  design(id: string): Design {
    return this.entity(Design, id, this.id);
  }

  type(id: string): Type {
    return this.entity(Type, id, this.id);
  }

  file(id: string): File {
    return this.entity(File, id, this.id);
  }

  tag(id: string): Tag {
    return this.entity(Tag, id, this.id);
  }

  concept(id: string): Concept {
    return this.entity(Concept, id, this.id);
  }

  quality(id: string): Quality {
    return this.entity(Quality, id, this.id);
  }

  author(id: string): Author {
    return this.entity(Author, id, this.id);
  }

  async mutateScoped(changeId: string, kitSelection: string): Promise<SetResult> {
    return this.session.mutateScoped(this.id, changeId, kitSelection);
  }

  async ensureChangeId(): Promise<string> {
    return this.session.ensureChangeId(this.id);
  }

  async saveChange(): Promise<void> {
    await this.session.saveChange(this.id);
  }

  async startNewChange(): Promise<ChangeId> {
    return await this.session.startNewChange(this.id);
  }

  async createCheckpoint(message: string): Promise<SetResult> {
    return this.session.createCheckpoint(this.id, message);
  }

  async startAlternative(name?: string): Promise<SetResult> {
    return this.session.startAlternative(this.id, name);
  }

  async integrateAlternative(alternativeId: string): Promise<SetResult> {
    return this.session.integrateAlternative(this.id, alternativeId);
  }

  wip(): Graph {
    return new Graph(this.session, "wip", this.id);
  }

  /** @emoji 🧭 Staging graph selection (UI tier); mirrors {@link Store#wip} until a distinct stage root exists in the schema. */
  stage(): Graph {
    return this.wip();
  }

  authoritative(): Graph {
    return new Graph(this.session, "authoritative", this.id);
  }

  async conflicts(): Promise<readonly Conflict[]> {
    const node = await this.session.readStoreInnerForId(this.id, "conflicts { edges { node { id } } }");
    return parseEntityConnectionIds(node, "conflicts").map((id) => new Conflict(this.session, id));
  }

  async attachBackbone(provider: Provider, uri: string): Promise<SetResult> {
    return provider.ensureBackboneAttached(this.id, uri);
  }

  async detachBackbone(): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { detach } } } }`,
      variables: { storeId: this.id },
    });
    return this.session.mutationReceipt(env);
  }

  async syncBackbone(): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { sync } } } }`,
      variables: { storeId: this.id },
    });
    return this.session.mutationReceipt(env);
  }
}

/** @emoji 🪢 Backbone exposed by a local or remote provider. */
export class Backbone extends Entity {
  constructor(session: Session, id: string, public readonly provider: Provider) {
    super(session, id);
  }
}

/** @emoji 🔌 Provider command facade for creating and attaching backbones to managed stores. */
export abstract class Provider extends Entity {
  protected abstract readonly commandSelection: string;

  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  protected abstract providerNode(inner: string): Promise<JsonObject | null>;

  async backboneUris(): Promise<readonly string[]> {
    const node = await this.providerNode("backbones { edges { node { uri } } }");
    const edges = jsonObjectField(node, "backbones")?.["edges"];
    if (!Array.isArray(edges)) return [];
    return edges.map((e) => String(((e as JsonObject)["node"] as JsonObject | undefined)?.["uri"] ?? "")).filter((uri) => uri !== "");
  }

  async createBackbone(uri: string): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, { query: `mutation { session { ${this.commandSelection} { createBackbone(uri: ${gqlString(uri)}) } } }` });
    return this.session.mutationReceipt(env);
  }

  async attachBackbone(storeId: string): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation($storeId: ID!) { session { ${this.commandSelection} { attachBackbone(store: $storeId) } } }`,
      variables: { storeId },
    });
    return this.session.mutationReceipt(env);
  }

  async ensureBackboneAttached(storeId: string, uri: string): Promise<SetResult> {
    if (!(await this.backboneUris()).includes(uri)) {
      const created = await this.createBackbone(uri);
      if (!created.ok) return created;
    }
    return this.attachBackbone(storeId);
  }
}

/** @emoji 💾 Local provider facade for file-backed stores and backbones. */
export class LocalProvider extends Provider {
  protected readonly commandSelection = "localProvider";

  constructor(session: Session) {
    super(session, "local");
  }

  protected async providerNode(inner: string): Promise<JsonObject | null> {
    const data = unwrapGraphqlData(await executeSessionReadGraphql(this.session, { query: `query { session { localProvider { ${inner} } } }` })) as JsonObject;
    return jsonObjectField(jsonObjectField(data, "session"), "localProvider");
  }
}

/** @emoji 🛜 Remote provider facade for websocket/hub-backed stores and backbones. */
export class RemoteProvider extends Provider {
  protected readonly commandSelection: string;

  constructor(session: Session, public readonly url: string) {
    super(session, url);
    this.commandSelection = `remoteProvider(url: ${gqlString(url)})`;
  }

  protected override async providerNode(inner: string): Promise<JsonObject | null> {
    const data = unwrapGraphqlData(
      await executeSessionReadGraphql(this.session, { query: `query { session { remoteProviders { edges { node { url ${inner} } } } } }` }),
    ) as JsonObject;
    const edges = jsonObjectField(jsonObjectField(data, "session"), "remoteProviders")?.["edges"];
    if (!Array.isArray(edges)) return null;
    for (const edge of edges) {
      const node = jsonObjectField(edge as JsonObject, "node");
      if (String(node?.["url"] ?? "") === this.url) return node;
    }
    return null;
  }

  async login(username: string, passwordHash: string, hubUrl?: string | null): Promise<SetResult> {
    const h = hubUrl == null ? "null" : gqlString(hubUrl);
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation { session { ${this.commandSelection} { login(username: ${gqlString(username)}, passwordHash: ${gqlString(passwordHash)}, hubUrl: ${h}) } } }`,
    });
    return this.session.mutationReceipt(env);
  }

  async logout(): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, { query: `mutation { session { ${this.commandSelection} { logout } } }` });
    return this.session.mutationReceipt(env);
  }
}

/** @emoji 🌐 VCS graph: {@code wip} / {@code authoritative} selections on {@link Store}. */
export class Graph extends Entity {
  constructor(session: Session, root: GraphRootKind, private readonly managedStoreId: string) {
    super(session, root);
  }

  private async readManagedStoreInner(inner: string): Promise<JsonObject | null> {
    return this.session.readStoreInnerForId(this.managedStoreId, inner);
  }

  get root(): GraphRootKind {
    return this.id as GraphRootKind;
  }

  /** @emoji 🏛 {@code graph { theKit }} handle. */
  theKit(): TheKit {
    return new TheKit(this.session, this.root, this.managedStoreId);
  }

  checkpoint(checkpointId: string): Checkpoint {
    return new Checkpoint(this.session, this.root, checkpointId, this.managedStoreId);
  }

  alternative(alternativeId: string): Alternative {
    return new Alternative(this.session, { parent: "graph", root: this.root, storeId: this.managedStoreId }, alternativeId);
  }

  private async readStoreGraphRootScalar(field: "id" | "hash"): Promise<string> {
    const node = jsonObjectField(await this.readManagedStoreInner(`${this.root} { ${field} }`), this.root);
    return node == null ? "" : String(node[field] ?? "");
  }

  async hash(): Promise<string> {
    return await this.readStoreGraphRootScalar("hash");
  }

  private async alternativeIds(): Promise<readonly string[]> {
    const node = jsonObjectField(await this.readManagedStoreInner(`${this.root} { alternatives { edges { node { id } } } }`), this.root);
    return parseEntityConnectionIds(node, "alternatives");
  }

  private async checkpointIds(): Promise<readonly string[]> {
    const node = jsonObjectField(await this.readManagedStoreInner(`${this.root} { checkpoints { edges { node { id } } } }`), this.root);
    return parseEntityConnectionIds(node, "checkpoints");
  }

  /** @emoji 📚 Id-list-stable {@link Alternative} handles under this graph root. */
  async alternatives(): Promise<readonly Alternative[]> {
    const ids = await this.alternativeIds();
    return Object.freeze(ids.map((id) => this.alternative(id)));
  }

  /** @emoji 📚 Id-list-stable {@link Checkpoint} handles under this graph root. */
  async checkpoints(): Promise<readonly Checkpoint[]> {
    const ids = await this.checkpointIds();
    return Object.freeze(ids.map((id) => this.checkpoint(id)));
  }

  /** @emoji 📡 Refetches {@link Graph#readAlternatives} on coarse kit ticks. */
  subscribeAlternatives(cb: (next: readonly Alternative[]) => void): Unsubscribe {
    const run = (): void => {
      void this.alternatives().then(cb);
    };
    return subscribeKitCoarseRefetch(this.session.bus, run);
  }

  /** @emoji 📡 Refetches {@link Graph#readCheckpoints} on coarse kit ticks. */
  subscribeCheckpoints(cb: (next: readonly Checkpoint[]) => void): Unsubscribe {
    const run = (): void => {
      void this.checkpoints().then(cb);
    };
    return subscribeKitCoarseRefetch(this.session.bus, run);
  }
}

/** @emoji 🧭 Parent scope for {@link Alternative} navigation. */
export type AlternativeParent = { readonly parent: "graph"; readonly root: GraphRootKind; readonly storeId: string };

/** @emoji 🔀 {@code Alternative} under {@link Graph} or {@link Session}. */
export class Alternative extends Entity {
  constructor(
    session: Session,
    private readonly ap: AlternativeParent,
    id: string,
  ) {
    super(session, id);
  }

  async name(): Promise<string> {
    const root = this.ap.parent === "graph" ? this.ap.root : "wip";
    const storeNode = await this.session.readStoreInnerForId(this.ap.storeId, `${root} { alternative(id: ${gqlString(this.id)}) { name } }`);
    const first = jsonObjectField(storeNode, root);
    const alt = first?.["alternative"] as JsonObject | undefined;
    return String(alt?.["name"] ?? "");
  }
}

/** @emoji 🏛 {@code TheKit} under {@code wip}/{@code authoritative}. */
export class TheKit extends Entity {
  constructor(session: Session, private readonly graphRoot: GraphRootKind, private readonly managedStoreId: string) {
    super(session, `theKit:${graphRoot}`);
  }

  /** @emoji 📦 Target {@code Version.kit} handle beneath this version node. */
  private kitRef(id = "kit"): Kit {
    return new Kit(this.session, id, this.managedStoreId);
  }

  private async kitId(): Promise<string> {
    const storeNode = await this.session.readStoreInnerForId(this.managedStoreId, `${this.graphRoot} { theKit { id } }`);
    const rootNode = jsonObjectField(storeNode, this.graphRoot);
    const tk = jsonObjectField(rootNode, "theKit");
    return String(tk?.["id"] ?? "");
  }

  /** @emoji 📦 Reads target {@code Version.kit} and returns the matching {@link Kit} handle. */
  async kit(): Promise<Kit> {
    return this.kitRef(await this.kitId());
  }
}

/** @emoji 🏁 {@code Checkpoint} under {@link Graph}. */
export class Checkpoint extends Entity {
  constructor(session: Session, private readonly graphRoot: GraphRootKind, checkpointId: string, private readonly managedStoreId: string) {
    super(session, checkpointId);
  }

  private async readManagedStoreInner(inner: string): Promise<JsonObject | null> {
    return this.session.readStoreInnerForId(this.managedStoreId, inner);
  }

  change(changeId: string): Change {
    return new Change(this.session, this.graphRoot, this.id, changeId, this.managedStoreId);
  }

  edit(editId: string): Edit {
    return new Edit(this.session, this.graphRoot, this.id, editId, this.managedStoreId);
  }

  async message(): Promise<string> {
    const rootNode = jsonObjectField(await this.readManagedStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { message } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return String(cp?.["message"] ?? "");
  }

  async timestamp(): Promise<string | null> {
    const rootNode = jsonObjectField(await this.readManagedStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { timestamp } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    const ts = cp?.["timestamp"];
    return ts == null ? null : String(ts);
  }

  async hash(): Promise<string> {
    const rootNode = jsonObjectField(await this.readManagedStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { hash } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return String(cp?.["hash"] ?? "");
  }

  private async changeIds(): Promise<readonly string[]> {
    const rootNode = jsonObjectField(await this.readManagedStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { changes { id } } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return parseStrongEntityArrayIds(cp ?? null, "changes");
  }

  private async editIds(): Promise<readonly string[]> {
    const rootNode = jsonObjectField(await this.readManagedStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { edits { edges { node { id } } } } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return parseEntityConnectionIds(cp ?? null, "edits");
  }

  /** @emoji 📚 Id-list-stable {@link Change} entities for this checkpoint (schema {@code changes: [Change!]!}). */
  async changes(): Promise<readonly Change[]> {
    const ids = await this.changeIds();
    return Object.freeze(ids.map((cid) => this.change(cid)));
  }

  /** @emoji 📚 Id-list-stable {@link Edit} handles for this checkpoint. */
  async edits(): Promise<readonly Edit[]> {
    const ids = await this.editIds();
    return Object.freeze(ids.map((eid) => this.edit(eid)));
  }

  /** @emoji 📡 Refetches {@link Checkpoint#readChanges} on coarse kit ticks. */
  subscribeChanges(cb: (next: readonly Change[]) => void): Unsubscribe {
    const run = (): void => {
      void this.changes().then(cb);
    };
    return subscribeKitCoarseRefetch(this.session.bus, run);
  }

  /** @emoji 📡 Refetches {@link Checkpoint#readEdits} on coarse kit ticks. */
  subscribeEdits(cb: (next: readonly Edit[]) => void): Unsubscribe {
    const run = (): void => {
      void this.edits().then(cb);
    };
    return subscribeKitCoarseRefetch(this.session.bus, run);
  }
}

/** @emoji 🔀 {@code Change} scoped to a {@link Checkpoint} (navigation shell; expand with field reads). */
export class Change extends Entity {
  constructor(
    session: Session,
    private readonly graphRoot: GraphRootKind,
    private readonly checkpointId: string,
    changeId: string,
    private readonly managedStoreId: string,
  ) {
    super(session, changeId);
  }

  private async readUnderChange(inner: string): Promise<JsonObject | null> {
    const storeNode = await this.session.readStoreInnerForId(this.managedStoreId, `${this.graphRoot} { checkpoint(id: ${gqlString(this.checkpointId)}) { change(id: ${gqlString(this.id)}) { ${inner} } } }`);
    const rootNode = jsonObjectField(storeNode, this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    const ch = cp?.["change"] as JsonObject | undefined;
    return ch ?? null;
  }

  async description(): Promise<string> {
    const node = await this.readUnderChange("description");
    return String(node?.["description"] ?? "");
  }

  async origin(): Promise<string> {
    const node = await this.readUnderChange("origin");
    return String(node?.["origin"] ?? "");
  }

  async saved(): Promise<boolean | null> {
    const node = await this.readUnderChange("saved");
    const v = node?.["saved"];
    if (v == null) return null;
    return Boolean(v);
  }

  async startedAt(): Promise<string> {
    const node = await this.readUnderChange("startedAt");
    const v = node?.["startedAt"];
    return v == null ? "" : String(v);
  }

  async savedAt(): Promise<string | null> {
    const node = await this.readUnderChange("savedAt");
    const v = node?.["savedAt"];
    return v == null ? null : String(v);
  }
}

/** @emoji ✏️ {@code Edit} scoped to a {@link Checkpoint} (navigation shell; expand with field reads). */
export class Edit extends Entity {
  constructor(
    session: Session,
    private readonly graphRoot: GraphRootKind,
    private readonly checkpointId: string,
    editId: string,
    private readonly managedStoreId: string,
  ) {
    super(session, editId);
  }

  private async readUnderEdit(inner: string): Promise<JsonObject | null> {
    const storeNode = await this.session.readStoreInnerForId(this.managedStoreId, `${this.graphRoot} { checkpoint(id: ${gqlString(this.checkpointId)}) { edit(id: ${gqlString(this.id)}) { ${inner} } } }`);
    const rootNode = jsonObjectField(storeNode, this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    const ed = cp?.["edit"] as JsonObject | undefined;
    return ed ?? null;
  }

  async description(): Promise<string> {
    const node = await this.readUnderEdit("description");
    return String(node?.["description"] ?? "");
  }

  async origin(): Promise<string> {
    const node = await this.readUnderEdit("origin");
    return String(node?.["origin"] ?? "");
  }

  async sequenceNumber(): Promise<number> {
    const node = await this.readUnderEdit("sequenceNumber");
    const v = node?.["sequenceNumber"];
    return typeof v === "number" ? v : Number(v ?? NaN);
  }

  async startedAt(): Promise<string> {
    const node = await this.readUnderEdit("startedAt");
    const v = node?.["startedAt"];
    return v == null ? "" : String(v);
  }
}

/** @emoji ⚔️ {@code Conflict} via {@code node(id:)}. */
export class Conflict extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  async reasons(): Promise<readonly string[]> {
    const data = unwrapGraphqlData(
      await executeSessionReadGraphql(this.session, { query: `query($id: ID!) { node(id: $id) { ... on Conflict { reasons } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const raw = n?.["reasons"] as readonly JsonValue[] | undefined;
    if (!Array.isArray(raw)) return [];
    return raw.map((x) => String(x));
  }

  private async authoritativeChangeId(): Promise<string> {
    const data = unwrapGraphqlData(
      await executeSessionReadGraphql(this.session, {
        query: `query($id: ID!) { node(id: $id) { ... on Conflict { authoritativeChange { id } } } }`,
        variables: { id: this.id },
      }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const ch = n?.["authoritativeChange"] as JsonObject | null | undefined;
    return ch ? String(ch["id"] ?? "") : "";
  }

  private async wipChangeId(): Promise<string> {
    const data = unwrapGraphqlData(
      await executeSessionReadGraphql(this.session, {
        query: `query($id: ID!) { node(id: $id) { ... on Conflict { wipChange { id } } } }`,
        variables: { id: this.id },
      }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const ch = n?.["wipChange"] as JsonObject | null | undefined;
    return ch ? String(ch["id"] ?? "") : "";
  }
}

/** @emoji 🧬 Abstract {@code Operation}; concrete operation subclasses follow the plan roster. */
export abstract class Operation extends Entity { }

//#region 🧮ChangeAlgebra
/** @emoji 🧮 Abstract diff leaf (kit algebra owned by rs; JS is navigation + reads). */
export abstract class Diff extends Entity { }

/** @emoji 🧮 Abstract modification triple (before, diff, after). */
export abstract class Modification extends Entity { }

/** @emoji 🧮 Wrapper for removed/added/modification aggregates on an entity diff. */
export class Modifications extends Entity { }

/** @emoji 📥 Abstract operation input payload (arguments mirror SDL input types). */
export abstract class Input extends Entity { }

/** @emoji 📜 Schema {@code Event}: domain ledger event with timestamp and involved entities. */
export abstract class Event extends Entity { }

//#region 🧬DiffVariants
/** @emoji 🧬 {@code KitDiff} navigation shell. */
export class KitDiff extends Diff { }
/** @emoji 🧬 {@code DesignDiff} navigation shell. */
export class DesignDiff extends Diff { }
/** @emoji 🧬 {@code TypeDiff} navigation shell. */
export class TypeDiff extends Diff { }
/** @emoji 🧬 {@code PieceDiff} navigation shell. */
export class PieceDiff extends Diff { }
/** @emoji 🧬 {@code ConnectionDiff} navigation shell. */
export class ConnectionDiff extends Diff { }
//#endregion 🧬DiffVariants

//#region 🧬ModificationVariants
export class KitModification extends Modification { }
export class DesignModification extends Modification { }
export class TypeModification extends Modification { }
export class PieceModification extends Modification { }
export class ConnectionModification extends Modification { }
//#endregion 🧬ModificationVariants

//#region 🧬ModificationsVariants
export class KitModifications extends Modifications { }
export class DesignModifications extends Modifications { }
//#endregion 🧬ModificationsVariants

//#region 🧬InputVariants
export class RenamedKitInput extends Input { }
export class CreatedTagInput extends Input { }
export class CreatedQualityInput extends Input { }
//#endregion 🧬InputVariants

//#region 🧬OperationVariants
export class RenamedKit extends Operation { }
export class ChangedDescription extends Operation { }
export class CreatedQuality extends Operation { }
export class CreatedQualities extends Operation { }
export class DeletedQuality extends Operation { }
export class CreatedTag extends Operation { }
export class DeletedPiece extends Operation { }
export class DeletedPieces extends Operation { }
export class DraggedPiece extends Operation { }
export class MovedPiece extends Operation { }
export class FixedPiece extends Operation { }
export class FlattenedDesign extends Operation { }
export class CreatedFixedPiece extends Operation { }
export class AddedChildPieceWithParentConnection extends Operation { }
export class AddedHangingChildPieceWithParentConnection extends Operation { }
//#endregion 🧬OperationVariants
//#endregion 🧮ChangeAlgebra

//#endregion 🧬VcsEntities

//#region 📐Design
export class Design extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  private child<T extends Entity>(ctor: new (session: Session, designId: string, id: string, storeId?: string) => T, id: string): T {
    return new ctor(this.session, this.id, id, this.storeId);
  }

  private dsel(inner: string): string {
    return `design(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  /** @emoji 🧷 Raw kit fragment for {@code design(id){ inner }} (shared scalar path for {@link readKitBranchString}). */
  private async designKitFrag(inner: string): Promise<JsonObject | null> {
    return (await this.readKitInner(this.dsel(inner))) as JsonObject | null;
  }

  piece(pieceId: string): Piece {
    return this.child(Piece, pieceId);
  }

  pieces(pieceIds: readonly string[]): PiecesOperation;
  async pieces(): Promise<readonly Piece[]>;
  pieces(pieceIds?: readonly string[]): PiecesOperation | Promise<readonly Piece[]> {
    if (pieceIds != null) return new PiecesOperation(this.session, this.id, pieceIds, this.storeId);
    return this.readPieces();
  }

  private async readPieces(): Promise<readonly Piece[]> {
    return Object.freeze((await this.pieceIds()).map((pid) => this.piece(pid)));
  }

  private piecesOperation(pieceIds: readonly string[]): PiecesOperation {
    return new PiecesOperation(this.session, this.id, pieceIds, this.storeId);
  }

  connection(connectionId: string): Connection {
    return this.child(Connection, connectionId);
  }

  layer(layerId: string): Layer {
    return this.child(Layer, layerId);
  }

  group(groupId: string): Group {
    return this.child(Group, groupId);
  }

  /** @emoji 🧷 GraphQL kit-store tail for {@code design(id){ … }} (shared with {@link bindDefinedFieldToReact}). */
  kitInnerPath(inner: string): string {
    return this.dsel(inner);
  }

  /**
   * @emoji 📖 Stateless read for one {@code design(id){ … }} selection; {@link FieldSpec#parse} receives the kit JSON (with nested {@code design}).
   */
  async fieldRead<T>(spec: FieldSpec<T>): Promise<T> {
    const frag = await this.readKitInner(this.dsel(spec.selection));
    return spec.parse(frag as JsonValue);
  }

  /**
   * @emoji 📡 When {@link FieldSpec#eventKind} matches {@link EventBus} kinds or live WIP ticks, refetches via {@link Design#fieldRead}.
   */
  subscribeField<T>(spec: FieldSpec<T>, cb: (next: T) => void): Unsubscribe {
    const kind = spec.eventKind;
    if (kind == null || kind === "") return () => { };
    return this.session.bus.subscribeKind(kind, () => {
      void this.fieldRead(spec).then(cb);
    });
  }

  /** @emoji 📡 Design description stream (rs {@code changedDescription}; coarse — refetches design description). */
  onDescriptionChanged(cb: (next: string) => void): Unsubscribe {
    return this.session.bus.subscribeKind("changedDescription", () => {
      void this.description().then(cb);
    });
  }

  async icon(): Promise<string> {
    return readKitBranchString(await this.designKitFrag("icon"), "design", "icon");
  }

  async image(): Promise<string> {
    return readKitBranchString(await this.designKitFrag("image"), "design", "image");
  }

  async unit(): Promise<string> {
    return readKitBranchString(await this.designKitFrag("unit"), "design", "unit");
  }

  async qualitySum(): Promise<number> {
    return readKitBranchNumber(await this.designKitFrag("qualitySum"), "design", "qualitySum");
  }

  private async pieceIds(): Promise<readonly string[]> {
    const frag = await this.designKitFrag("pieces { edges { node { id } } }");
    const d = frag?.["design"] as JsonObject | undefined;
    return parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "pieces");
  }

  /** @emoji 📚 Id-list-stable {@link Piece} handles (same order as the SDL {@code pieces} field). */
  /** @emoji 📡 Refetches {@link Design#readPieces} on coarse kit ticks (piece membership / graph writes). */
  subscribePieces(cb: (next: readonly Piece[]) => void): Unsubscribe {
    const run = (): void => {
      void this.pieces().then(cb);
    };
    return subscribeKitCoarseRefetch(this.session.bus, run);
  }

  private async connectionIds(): Promise<readonly string[]> {
    const frag = await this.designKitFrag("connections { edges { node { id } } }");
    const d = frag?.["design"] as JsonObject | undefined;
    return parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "connections");
  }

  /** @emoji 📚 Id-list-stable {@link Connection} handles (same order as the SDL {@code connections} field). */
  async connections(): Promise<readonly Connection[]> {
    const ids = await this.connectionIds();
    return Object.freeze(ids.map((cid) => this.connection(cid)));
  }

  /** @emoji 📡 Refetches {@link Design#readConnections} on coarse kit ticks. */
  subscribeConnections(cb: (next: readonly Connection[]) => void): Unsubscribe {
    const run = (): void => {
      void this.connections().then(cb);
    };
    return subscribeKitCoarseRefetch(this.session.bus, run);
  }

  private async attributeIds(): Promise<readonly string[]> {
    const frag = await this.designKitFrag("attributes { edges { node { id } } }");
    const d = frag?.["design"] as JsonObject | undefined;
    return parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "attributes");
  }

  async name(): Promise<string> {
    return readKitBranchString(await this.designKitFrag("name"), "design", "name");
  }

  async description(): Promise<string> {
    return readKitBranchString(await this.designKitFrag("description"), "design", "description");
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`rn: rename(newName: ${gqlString(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async flatten(): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`fl: flatten`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }

  async addFixedPiece(blueprintId: string, position: PositionInput, name?: string | null, description?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const pos = formatPositionInput(position);
    const n = name == null ? "null" : gqlString(name);
    const d = description == null ? "null" : gqlString(description);
    return this.mutateScoped(cid, this.dsel(`afp: addFixedPiece(blueprintId: ${gqlString(blueprintId)}, position: ${pos}, name: ${n}, description: ${d})`));
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
    const cid = await this.ensureChangeId();
    const pos = position == null ? "null" : formatPositionInput(position);
    const n = name == null ? "null" : gqlString(name);
    const d = description == null ? "null" : gqlString(description);
    const sc = scale == null ? "null" : String(scale);
    return this.session.mutateScoped(
      cid,
      this.dsel(
        `ac: addChildPieceWithParentConnection(blueprintId: ${gqlString(blueprintId)}, parentPieceId: ${gqlString(parentPieceId)}, parentConnector: ${gqlString(parentConnector)}, childConnector: ${gqlString(childConnector)}, name: ${n}, description: ${d}, position: ${pos}, scale: ${sc})`,
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
    const cid = await this.ensureChangeId();
    const pos = formatPositionInput(position);
    const n = name == null ? "null" : gqlString(name);
    const d = description == null ? "null" : gqlString(description);
    const sc = scale == null ? "null" : String(scale);
    return this.session.mutateScoped(
      cid,
      this.dsel(
        `ah: addHangingChildPieceWithParentConnection(blueprintId: ${gqlString(blueprintId)}, parentPieceId: ${gqlString(parentPieceId)}, parentConnector: ${gqlString(parentConnector)}, childConnector: ${gqlString(childConnector)}, position: ${pos}, name: ${n}, description: ${d}, scale: ${sc})`,
      ),
    );
  }

  async deletePiece(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`dp: deletePiece(id: ${gqlString(id)})`));
  }

  async deletePieces(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`dps: deletePieces(ids: ${gqlIdList(ids)})`));
  }

  async deletePiecesAndConnections(pieceIds: readonly string[], connectionIds: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.dsel(`dpc: deletePiecesAndConnections(pieceIds: ${gqlIdList(pieceIds)}, connectionIds: ${gqlIdList(connectionIds)})`));
  }
}
//#endregion 📐Design

//#region 🧰Type
export class Type extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  private child<T extends Entity>(ctor: new (session: Session, typeId: string, id: string, storeId?: string) => T, id: string): T {
    return new ctor(this.session, this.id, id, this.storeId);
  }

  private tsel(inner: string): string {
    return `type(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  /** @emoji 🧷 Raw kit fragment for {@code type(id){ inner }}. */
  private async typeKitFrag(inner: string): Promise<JsonObject | null> {
    return (await this.readKitInner(this.tsel(inner))) as JsonObject | null;
  }

  port(portId: string): Port {
    return this.child(Port, portId);
  }

  connector(connectorId: string): Connector {
    return this.child(Connector, connectorId);
  }

  representation(representationId: string): Representation {
    return this.child(Representation, representationId);
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`rn: rename(newName: ${gqlString(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }

  async createPort(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const c = code == null ? "null" : gqlString(code);
    const l = label == null ? "null" : gqlString(label);
    const d = description == null ? "null" : gqlString(description);
    const i = icon == null ? "null" : gqlString(icon);
    const o = order == null ? "null" : String(order);
    return this.mutateScoped(cid, this.tsel(`cp: createPort(code: ${c}, label: ${l}, description: ${d}, icon: ${i}, order: ${o})`));
  }

  async deletePort(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`dp: deletePort(id: ${gqlString(id)})`));
  }

  async deletePorts(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`dps: deletePorts(ids: ${gqlIdList(ids)})`));
  }

  async addConnector(code: string, description?: string | null, icon?: string | null, portId?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const d = description == null ? "null" : gqlString(description);
    const i = icon == null ? "null" : gqlString(icon);
    const p = portId == null ? "null" : gqlString(portId);
    return this.mutateScoped(cid, this.tsel(`ac: addConnector(code: ${gqlString(code)}, description: ${d}, icon: ${i}, portId: ${p})`));
  }

  async removeConnector(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`rc: removeConnector(id: ${gqlString(id)})`));
  }

  async removeConnectors(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.tsel(`rcs: removeConnectors(ids: ${gqlIdList(ids)})`));
  }

  /** @emoji 🧰 Resolves {@code type(id){…}} on the materialized kit fragment. */
  private typeNode(frag: JsonObject | null): JsonObject | undefined {
    return frag?.["type"] as JsonObject | undefined;
  }

  async name(): Promise<string> {
    return readKitBranchString(await this.typeKitFrag("name"), "type", "name");
  }

  async description(): Promise<string> {
    return readKitBranchString(await this.typeKitFrag("description"), "type", "description");
  }

  async icon(): Promise<string> {
    return readKitBranchString(await this.typeKitFrag("icon"), "type", "icon");
  }

  async image(): Promise<string> {
    return readKitBranchString(await this.typeKitFrag("image"), "type", "image");
  }

  async unit(): Promise<string> {
    return readKitBranchString(await this.typeKitFrag("unit"), "type", "unit");
  }

  /** @emoji 🧰 Stable {@link Port} handles for the SDL {@code ports} field. */
  async ports(): Promise<readonly Port[]> {
    const inner = "ports { edges { node { id } } }";
    const frag = await this.typeKitFrag(inner);
    const prt = this.typeNode(frag)?.["ports"] as JsonObject | undefined;
    const edges = (prt?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    const out: Port[] = [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      if (n == null) continue;
      const id = String(n["id"] ?? "");
      if (id === "") continue;
      out.push(this.port(id));
    }
    return Object.freeze(out);
  }

  /** @emoji 🧰 Stable {@link Connector} handles for the SDL {@code connectors} field. */
  async connectors(): Promise<readonly Connector[]> {
    const inner = "connectors { edges { node { id code name } } }";
    const frag = await this.typeKitFrag(inner);
    const conn = this.typeNode(frag)?.["connectors"] as JsonObject | undefined;
    const edges = (conn?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    const out: Connector[] = [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      if (n == null) continue;
      const id = String(n["id"] ?? "");
      if (id === "") continue;
      out.push(this.connector(id));
    }
    return Object.freeze(out);
  }

  /** @emoji 🧰 Stable {@link Representation} handles for the SDL {@code representations} field. */
  async representations(): Promise<readonly Representation[]> {
    const inner = "representations { edges { node { id } } }";
    const frag = await this.typeKitFrag(inner);
    const rep = this.typeNode(frag)?.["representations"] as JsonObject | undefined;
    const edges = (rep?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    const out: Representation[] = [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      const id = String(n?.["id"] ?? "");
      if (id !== "") out.push(this.representation(id));
    }
    return Object.freeze(out);
  }

  /** @emoji 🧰 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Type.attributes}). */
  async attributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = await this.typeKitFrag(inner);
    return parseAttributeConnectionUnder(this, this.typeNode(frag));
  }

  /** @emoji 🧰 Stable {@link Author} handles for the SDL {@code authors} field. */
  async authors(): Promise<readonly Author[]> {
    const inner = "authors { edges { node { id } } }";
    const frag = await this.typeKitFrag(inner);
    const authors = this.typeNode(frag)?.["authors"] as JsonObject | undefined;
    const edges = (authors?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    const out: Author[] = [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      const id = String(n?.["id"] ?? "");
      if (id !== "") out.push(new Author(this.session, id, this.storeId));
    }
    return Object.freeze(out);
  }
}
//#endregion 🧰Type

/** @emoji 🧷 Kit JSON path {@code type → port} for nested reads under {@link Port}. */
const KIT_PATH_TYPE_PORT: readonly string[] = ["type", "port"];
/** @emoji 🧷 Kit JSON path {@code type → connector}. */
const KIT_PATH_TYPE_CONNECTOR: readonly string[] = ["type", "connector"];
/** @emoji 🧷 Kit JSON path {@code type → representation}. */
const KIT_PATH_TYPE_REPRESENTATION: readonly string[] = ["type", "representation"];

//#region 🔘Port
export class Port extends Entity {
  readonly typeId: string;
  constructor(session: Session, typeId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.typeId = typeId;
  }

  private psel(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { port(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  /** @emoji 🔘 SDL {@code Port.code}. */
  async code(): Promise<string> {
    const frag = (await this.readKitInner(this.psel("code"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "code");
  }

  /** @emoji 🔘 SDL {@code Port.label}. */
  async label(): Promise<string> {
    const frag = (await this.readKitInner(this.psel("label"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "label");
  }

  /** @emoji 🔘 SDL {@code Port.order}. */
  async order(): Promise<number | null> {
    const frag = (await this.readKitInner(this.psel("order"))) as JsonObject | null;
    return readKitPathNumberOrNull(frag, KIT_PATH_TYPE_PORT, "order");
  }

  async name(): Promise<string> {
    const frag = (await this.readKitInner(this.psel("name"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "name");
  }

  async description(): Promise<string> {
    const frag = (await this.readKitInner(this.psel("description"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "description");
  }

  async icon(): Promise<string> {
    const frag = (await this.readKitInner(this.psel("icon"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "icon");
  }

  /** @emoji 🔘 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Port.attributes}). */
  async attributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.readKitInner(this.psel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this, readKitPathNode(frag, KIT_PATH_TYPE_PORT));
  }

  async rename(newCode: string, newLabel?: string | null): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    const lab = newLabel == null ? "null" : gqlString(newLabel);
    return this.mutateScoped(cid, this.psel(`rn: rename(newCode: ${gqlString(newCode)}, newLabel: ${lab})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }
}
//#endregion 🔘Port

//#region 🔗Connector
export class Connector extends Entity {
  readonly typeId: string;
  constructor(session: Session, typeId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.typeId = typeId;
  }

  private csel(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { connector(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  async rename(newCode: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.csel(`rn: rename(newCode: ${gqlString(newCode)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.csel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.csel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async name(): Promise<string> {
    const frag = (await this.readKitInner(this.csel("name"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "name");
  }

  async code(): Promise<string> {
    const frag = (await this.readKitInner(this.csel("code"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "code");
  }

  async description(): Promise<string> {
    const frag = (await this.readKitInner(this.csel("description"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "description");
  }

  async icon(): Promise<string> {
    const frag = (await this.readKitInner(this.csel("icon"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "icon");
  }

  /** @emoji 🔗 Nullable {@code port { id }} per SDL {@code Connector.port}. */
  private async portId(): Promise<string | null> {
    const frag = (await this.readKitInner(this.csel("port { id }"))) as JsonObject | null;
    const p = readKitPathNode(frag, KIT_PATH_TYPE_CONNECTOR)?.["port"] as JsonObject | null | undefined;
    if (p == null) return null;
    const id = String(p["id"] ?? "");
    return id === "" ? null : id;
  }

  /** @emoji 🔗 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Connector.attributes}). */
  async attributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.readKitInner(this.csel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this, readKitPathNode(frag, KIT_PATH_TYPE_CONNECTOR));
  }

  async port(): Promise<Port | null> {
    const id = await this.portId();
    return id == null ? null : new Type(this.session, this.typeId, this.storeId).port(id);
  }
}
//#endregion 🔗Connector

//#region 🧩Piece
/** @emoji 🧩 @description Blueprint target on a {@link Piece} (`Type` or `Design` node). */
export interface PieceBlueprint {
  readonly blueprintKind: "Type" | "Design";
  readonly id: string;
}

function pieceKit(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const p = d?.["piece"] as JsonObject | undefined;
  return p ?? null;
}

//#region 🪶WeakGeometry
/** @emoji 📌 Weak {@code position}/{@code flatPosition} anchored on {@link Piece} (stable child cache). */
export class Position {
  private readonly _center: Coordinate;
  private readonly _plane: Plane;
  constructor(
    public readonly piece: Piece,
    public readonly role: "position" | "flatPosition",
  ) {
    this._center = new Coordinate(this);
    this._plane = new Plane(this);
  }

  center(): Coordinate {
    return this._center;
  }

  plane(): Plane {
    return this._plane;
  }
}

/** @emoji 📍 Weak {@code Coordinate} under {@link Position}. */
export class Coordinate {
  constructor(public readonly parent: Position) { }

  async u(): Promise<number> {
    const frag = (await this.parent.piece.readKitInner(
      this.parent.piece.kitPieceSelection(`${this.parent.role} { center { u v } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.role] as JsonObject | undefined;
    const c = json?.["center"] as JsonObject | undefined;
    return typeof c?.["u"] === "number" ? c["u"] : 0;
  }

  async v(): Promise<number> {
    const frag = (await this.parent.piece.readKitInner(
      this.parent.piece.kitPieceSelection(`${this.parent.role} { center { u v } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.role] as JsonObject | undefined;
    const c = json?.["center"] as JsonObject | undefined;
    return typeof c?.["v"] === "number" ? c["v"] : 0;
  }
}

/** @emoji 📐 Weak {@code Plane} under {@link Position}. */
export class Plane {
  private readonly _origin: Point;
  private readonly _xAxis: Vector;
  private readonly _yAxis: Vector;
  constructor(public readonly parent: Position) {
    this._origin = new Point(this);
    this._xAxis = new Vector(this, "xAxis");
    this._yAxis = new Vector(this, "yAxis");
  }

  origin(): Point {
    return this._origin;
  }

  xAxis(): Vector {
    return this._xAxis;
  }

  yAxis(): Vector {
    return this._yAxis;
  }
}

/** @emoji 🔵 Weak 3D point leaf (origin) under {@link Plane}. */
export class Point {
  constructor(public readonly parent: Plane) { }

  async x(): Promise<number> {
    const frag = (await this.parent.parent.piece.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { origin { x y z } } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = json?.["plane"] as JsonObject | undefined;
    const o = pl?.["origin"] as JsonObject | undefined;
    return typeof o?.["x"] === "number" ? o["x"] : 0;
  }

  async y(): Promise<number> {
    const frag = (await this.parent.parent.piece.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { origin { x y z } } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = json?.["plane"] as JsonObject | undefined;
    const o = pl?.["origin"] as JsonObject | undefined;
    return typeof o?.["y"] === "number" ? o["y"] : 0;
  }

  async z(): Promise<number> {
    const frag = (await this.parent.parent.piece.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { origin { x y z } } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = json?.["plane"] as JsonObject | undefined;
    const o = pl?.["origin"] as JsonObject | undefined;
    return typeof o?.["z"] === "number" ? o["z"] : 0;
  }
}

/** @emoji ➡️ Weak axis vector leaf under {@link Plane}. */
export class Vector {
  constructor(
    public readonly parent: Plane,
    public readonly axisRole: "xAxis" | "yAxis",
  ) { }

  async x(): Promise<number> {
    const frag = (await this.parent.parent.piece.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { ${this.axisRole} { x y z } } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = json?.["plane"] as JsonObject | undefined;
    const ax = pl?.[this.axisRole] as JsonObject | undefined;
    return typeof ax?.["x"] === "number" ? ax["x"] : 0;
  }

  async y(): Promise<number> {
    const frag = (await this.parent.parent.piece.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { ${this.axisRole} { x y z } } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = json?.["plane"] as JsonObject | undefined;
    const ax = pl?.[this.axisRole] as JsonObject | undefined;
    return typeof ax?.["y"] === "number" ? ax["y"] : 0;
  }

  async z(): Promise<number> {
    const frag = (await this.parent.parent.piece.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { ${this.axisRole} { x y z } } }`),
    )) as JsonObject | null;
    const json = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = json?.["plane"] as JsonObject | undefined;
    const ax = pl?.[this.axisRole] as JsonObject | undefined;
    return typeof ax?.["z"] === "number" ? ax["z"] : 0;
  }
}

/** @emoji ↔️ Weak {@code OffsetInput} path shell (drag hints; expand when SDL exposes offset nodes). */
export class Offset {
  constructor(
    public readonly piece: Piece,
    public readonly role: "drag",
  ) { }
}

/** @emoji 🌍 Geographic weak shell (parent + role path for future SDL). */
export class Place {
  constructor(
    public readonly owner: Entity,
    public readonly role: string,
  ) { }
}

/** @emoji 🗺️ Weak lat/lon shell under {@link Place}. */
export class Location {
  constructor(public readonly parent: Place) { }
}

/** @emoji 📷 Weak camera shell (viewport hints). */
export class Camera {
  constructor(
    public readonly owner: Entity,
    public readonly role: string,
  ) { }
}

//#region 📥GeomInputs
/** @emoji 📥 GraphQL {@code PositionInput} mirror for kit mutations (matches SDL input, not {@link Position}). */
export type PositionInput = Readonly<{
  center: Readonly<{ u: number; v: number }>;
  plane: Readonly<{
    origin: Readonly<{ x: number; y: number; z: number }>;
    xAxis: Readonly<{ x: number; y: number; z: number }>;
    yAxis: Readonly<{ x: number; y: number; z: number }>;
  }>;
}>;

/** @emoji 📥 GraphQL {@code OffsetInput} mirror for kit mutations. */
export type OffsetInput = Readonly<{
  u: number;
  v: number;
}>;

function gqlFiniteNumber(n: number): string {
  return Number.isFinite(n) ? String(n) : "0";
}

/** @emoji 📡 Inline GraphQL object literal for {@code PositionInput}. */
export function formatPositionInput(p: PositionInput): string {
  const c = p.center;
  const pl = p.plane;
  const o = pl.origin;
  const xa = pl.xAxis;
  const ya = pl.yAxis;
  return `{ center: { u: ${gqlFiniteNumber(c.u)}, v: ${gqlFiniteNumber(c.v)} }, plane: { origin: { x: ${gqlFiniteNumber(o.x)}, y: ${gqlFiniteNumber(o.y)}, z: ${gqlFiniteNumber(o.z)} }, xAxis: { x: ${gqlFiniteNumber(xa.x)}, y: ${gqlFiniteNumber(xa.y)}, z: ${gqlFiniteNumber(xa.z)} }, yAxis: { x: ${gqlFiniteNumber(ya.x)}, y: ${gqlFiniteNumber(ya.y)}, z: ${gqlFiniteNumber(ya.z)} } } }`;
}

/** @emoji 📡 Inline GraphQL object literal for {@code OffsetInput}. */
export function formatOffsetInput(o: OffsetInput): string {
  return `{ u: ${gqlFiniteNumber(o.u)}, v: ${gqlFiniteNumber(o.v)} }`;
}
//#endregion 📥GeomInputs
//#endregion 🪶WeakGeometry

const PIECE_POSITION_SELECTION = "center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } }";

function parsePieceBlueprintFromJson(node: JsonObject | null | undefined): PieceBlueprint | null {
  if (node == null || typeof node !== "object") return null;
  const tn = String(node["__typename"] ?? "");
  const id = String(node["id"] ?? "");
  if (id === "") return null;
  if (tn === "Type") return { blueprintKind: "Type", id };
  if (tn === "Design") return { blueprintKind: "Design", id };
  return null;
}

function parseIdListConnection(obj: JsonObject | null | undefined, field: string): readonly string[] {
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

export class Piece extends Entity {
  readonly designId: string;
  private readonly positionByRole = new Map<"position" | "flatPosition", Position>();
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  /** @emoji 🧷 GraphQL path fragment under {@code design(id){ piece(id){ … }}} for kit reads (weak geometry + {@link Piece} fields). */
  kitPieceSelection(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { piece(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  /** @emoji 📌 Stable weak {@code position} handle for this piece. */
  position(): Position {
    let p = this.positionByRole.get("position");
    if (!p) {
      p = new Position(this, "position");
      this.positionByRole.set("position", p);
    }
    return p;
  }

  /** @emoji 📌 Stable weak {@code flatPosition} handle for this piece. */
  flatPosition(): Position {
    let p = this.positionByRole.get("flatPosition");
    if (!p) {
      p = new Position(this, "flatPosition");
      this.positionByRole.set("flatPosition", p);
    }
    return p;
  }

  async name(): Promise<string> {
    const frag = (await this.readKitInner(this.kitPieceSelection("name"))) as JsonObject | null;
    return String(pieceKit(frag)?.["name"] ?? "");
  }

  async description(): Promise<string> {
    const frag = (await this.readKitInner(this.kitPieceSelection("description"))) as JsonObject | null;
    return String(pieceKit(frag)?.["description"] ?? "");
  }

  async icon(): Promise<string> {
    const frag = (await this.readKitInner(this.kitPieceSelection("icon"))) as JsonObject | null;
    return String(pieceKit(frag)?.["icon"] ?? "");
  }

  async typeId(): Promise<string | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("type { id }"))) as JsonObject | null;
    const n = pieceKit(frag)?.["type"] as JsonObject | undefined;
    const tid = n == null ? "" : String(n["id"] ?? "");
    return tid === "" ? null : tid;
  }

  async scale(): Promise<number | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("scale"))) as JsonObject | null;
    const v = pieceKit(frag)?.["scale"];
    return typeof v === "number" ? v : null;
  }

  private async positionLoaded(): Promise<Position | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection(`position { ${PIECE_POSITION_SELECTION} }`))) as JsonObject | null;
    const raw = pieceKit(frag)?.["position"];
    if (raw == null || typeof raw !== "object") return null;
    return this.position();
  }

  private async flatPositionLoaded(): Promise<Position | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection(`flatPosition { ${PIECE_POSITION_SELECTION} }`))) as JsonObject | null;
    const raw = pieceKit(frag)?.["flatPosition"];
    if (raw == null || typeof raw !== "object") return null;
    return this.flatPosition();
  }

  private async planeLoaded(): Promise<Plane | null> {
    if ((await this.positionLoaded()) == null) return null;
    return this.position().plane();
  }

  private async centerLoaded(): Promise<Coordinate | null> {
    if ((await this.positionLoaded()) == null) return null;
    return this.position().center();
  }

  private async flatPlaneLoaded(): Promise<Plane | null> {
    if ((await this.flatPositionLoaded()) == null) return null;
    return this.flatPosition().plane();
  }

  private async flatCenterLoaded(): Promise<Coordinate | null> {
    if ((await this.flatPositionLoaded()) == null) return null;
    return this.flatPosition().center();
  }

  async blueprint(): Promise<PieceBlueprint | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("blueprint { __typename id }"))) as JsonObject | null;
    return parsePieceBlueprintFromJson(pieceKit(frag)?.["blueprint"] as JsonObject | undefined);
  }

  async attributes(): Promise<readonly Attribute[]> {
    const frag = (await this.readKitInner(this.kitPieceSelection("attributes { edges { node { id key value definition } } }"))) as JsonObject | null;
    return parseAttributeConnectionUnder(this, pieceKit(frag));
  }

  async connectionKind(): Promise<"FIXED" | "CONNECTED" | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("connectionKind"))) as JsonObject | null;
    const k = pieceKit(frag)?.["connectionKind"];
    if (k === "FIXED" || k === "CONNECTED") return k;
    return null;
  }

  private async parentPieceId(): Promise<string | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("parentPiece { id }"))) as JsonObject | null;
    const n = pieceKit(frag)?.["parentPiece"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  private async parentConnectionId(): Promise<string | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("parentConnection { id }"))) as JsonObject | null;
    const n = pieceKit(frag)?.["parentConnection"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  private async childPieceIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner(this.kitPieceSelection("childPieces { edges { node { id } } }"))) as JsonObject | null;
    return parseIdListConnection(pieceKit(frag), "childPieces");
  }

  private async childConnectionIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner(this.kitPieceSelection("childConnections { edges { node { id } } }"))) as JsonObject | null;
    return parseIdListConnection(pieceKit(frag), "childConnections");
  }

  async depth(): Promise<number | null> {
    const frag = (await this.readKitInner(this.kitPieceSelection("depth"))) as JsonObject | null;
    const v = pieceKit(frag)?.["depth"];
    return typeof v === "number" ? v : null;
  }

  async parentPiece(): Promise<Piece | null> {
    const id = await this.parentPieceId();
    return id == null ? null : new Design(this.session, this.designId, this.storeId).piece(id);
  }

  async parentConnection(): Promise<Connection | null> {
    const id = await this.parentConnectionId();
    return id == null ? null : new Design(this.session, this.designId, this.storeId).connection(id);
  }

  async childPieces(): Promise<readonly Piece[]> {
    return Object.freeze((await this.childPieceIds()).map((id) => new Design(this.session, this.designId, this.storeId).piece(id)));
  }

  async childConnections(): Promise<readonly Connection[]> {
    return Object.freeze((await this.childConnectionIds()).map((id) => new Design(this.session, this.designId, this.storeId).connection(id)));
  }

  /** @emoji 🧭 Ordered {@link Piece#path} piece node keys (root → … → self) from kit GraphQL. */
  async pathPieces(): Promise<readonly string[]> {
    const frag = (await this.readKitInner(this.kitPieceSelection("path { edges { node { id } } }"))) as JsonObject | null;
    return parseIdListConnection(pieceKit(frag), "path");
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`rn: rename(newName: ${gqlString(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async drag(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(position: PositionInput): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`mv: move(position: ${formatPositionInput(position)})`));
  }

  async fix(): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`fx: fix`));
  }

  async changeBlueprint(blueprintId: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`cb: changeBlueprint(blueprintId: ${gqlString(blueprintId)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.kitPieceSelection(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }
}
//#endregion 🧩Piece

//#region 🪢PiecesOperation
export class PiecesOperation {
  constructor(
    private readonly session: Session,
    private readonly designId: string,
    private readonly pieceIds: readonly string[],
    private readonly storeId?: string,
  ) { }

  private psel(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { pieces(ids: ${gqlIdList(this.pieceIds)}) { ${inner} } }`;
  }

  private async ensureChangeId(): Promise<string> {
    if (this.storeId == null || this.storeId === "") throw new Error("PiecesOperation is not scoped to a Store");
    return this.session.ensureChangeId(this.storeId);
  }

  private async mutateScoped(changeId: string, kitSelection: string): Promise<SetResult> {
    if (this.storeId == null || this.storeId === "") throw new Error("PiecesOperation is not scoped to a Store");
    return this.session.mutateScoped(this.storeId, changeId, kitSelection);
  }

  async drag(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`mv: move(offset: ${formatOffsetInput(offset)})`));
  }

  async fix(): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`fx: fix`));
  }

  async changeBlueprint(blueprintId: string): Promise<SetResult> {
    const cid = await this.ensureChangeId();
    return this.mutateScoped(cid, this.psel(`cb: changeBlueprint(blueprintId: ${gqlString(blueprintId)})`));
  }
}
//#endregion 🪢PiecesOperation

//#region ⛓️Connection
/** @emoji ⛓️ Schema-aligned {@link Connection} endpoint (piece + optional port / connector / designPiece ids). */
export class Side {
  constructor(
    public readonly session: Session,
    public readonly designId: string,
    public readonly connectionId: string,
    public readonly role: "parent" | "child",
    public readonly pieceId: string,
    public readonly portId: string | null,
    public readonly connectorId: string | null,
    public readonly designPieceId: string | null,
    public readonly storeId?: string,
  ) { }

  /** @emoji 🧩 Resolved {@link Piece} on this kit read point. */
  piece(): Piece {
    return new Design(this.session, this.designId, this.storeId).piece(this.pieceId);
  }
}

const CONNECTION_SIDE_SELECTION = "piece { id } port { id } designPiece { id } connector { id }";

function connectionKit(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const c = d?.["connection"] as JsonObject | undefined;
  return c ?? null;
}

function parseSideFromJson(
  session: Session,
  designId: string,
  connectionId: string,
  role: "parent" | "child",
  node: JsonObject | null | undefined,
  storeId?: string,
): Side | null {
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
  return new Side(session, designId, connectionId, role, pieceId, portId, connectorId, designPieceId, storeId);
}

export class Connection extends Entity {
  readonly designId: string;
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  private csel(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { connection(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  /** @emoji ⛓️ Resolved {@code design.connection} JSON for the given selection tail. */
  private async connectionUnderDesign(inner: string): Promise<JsonObject | null> {
    const frag = (await this.readKitInner(this.csel(inner))) as JsonObject | null;
    return connectionKit(frag);
  }

  private async readConnScalarString(field: string): Promise<string> {
    return String((await this.connectionUnderDesign(field))?.[field] ?? "");
  }

  private async readConnScalarNumberOrNull(field: string): Promise<number | null> {
    const v = (await this.connectionUnderDesign(field))?.[field];
    return typeof v === "number" ? v : null;
  }

  async name(): Promise<string> {
    return await this.readConnScalarString("name");
  }

  async description(): Promise<string> {
    return await this.readConnScalarString("description");
  }

  async icon(): Promise<string> {
    return await this.readConnScalarString("icon");
  }

  async gap(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("gap");
  }

  async shift(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("shift");
  }

  async rise(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("rise");
  }

  async rotation(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("rotation");
  }

  async turn(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("turn");
  }

  async tilt(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("tilt");
  }

  async u(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("u");
  }

  async v(): Promise<number | null> {
    return await this.readConnScalarNumberOrNull("v");
  }

  async parent(): Promise<Side | null> {
    const connJson = await this.connectionUnderDesign(`parent { ${CONNECTION_SIDE_SELECTION} }`);
    return parseSideFromJson(this.session, this.designId, this.id, "parent", connJson?.["parent"] as JsonObject | undefined, this.storeId);
  }

  async child(): Promise<Side | null> {
    const connJson = await this.connectionUnderDesign(`child { ${CONNECTION_SIDE_SELECTION} }`);
    return parseSideFromJson(this.session, this.designId, this.id, "child", connJson?.["child"] as JsonObject | undefined, this.storeId);
  }

  async attributes(): Promise<readonly Attribute[]> {
    return parseAttributeConnectionUnder(this, await this.connectionUnderDesign("attributes { edges { node { id key value definition } } }"));
  }
}
//#endregion ⛓️Connection

//#region ✍️Author
/** @emoji ✍️ Author artifact: kit-scoped reads only (no {@code *OperationInput} on Author in schema). */
export class Author extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare email: () => Promise<string>;
  declare role: () => Promise<string>;
  declare rank: () => Promise<number | null>;
}

const AUTHOR_FIELDS = defineBoundNodeFields([
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { selection: "icon", parse: (node) => String(node?.["icon"] ?? "") },
  { selection: "email", parse: (node) => String(node?.["email"] ?? "") },
  { selection: "role", parse: (node) => String(node?.["role"] ?? "") },
  { selection: "rank", parse: (node) => (typeof node?.["rank"] === "number" ? node["rank"] : null) },
] as const);
installNodeFieldMethods(Author, "Author", AUTHOR_FIELDS);
//#endregion ✍️Author

//#region 💎Quality
/** @emoji 💎 Quality artifact: {@code QualityOperationInput} leaves + scalar reads via {@code quality(id:)}. */
export class Quality extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  kitInnerPath(inner: string): string {
    return `quality(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  declare key: () => Promise<string>;
  declare value: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare definition: () => Promise<string>;
  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare rename: (newKey: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;

  async benchmarks(): Promise<readonly Benchmark[]> {
    const frag = (await this.readKitInner(
      this.kitInnerPath(`benchmarks { edges { node { id name min max minExcluded maxExcluded } } }`),
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
      out.push(
        new Benchmark(
          this,
          String(n["id"] ?? ""),
          String(n["name"] ?? ""),
          typeof n["min"] === "number" ? n["min"] : null,
          typeof n["max"] === "number" ? n["max"] : null,
          typeof n["minExcluded"] === "boolean" ? n["minExcluded"] : null,
          typeof n["maxExcluded"] === "boolean" ? n["maxExcluded"] : null,
        ),
      );
    }
    return out;
  }
}

const QUALITY_OPERATIONS = defineBoundKitOperations([
  { buildInner: (_entity, newKey) => `rk: rename(newKey: ${gqlString(String(newKey ?? ""))})` },
  { buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const);
installKitFieldMethods(Quality, defineBoundKitFields([
  { selection: "key", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "key") },
  { selection: "value", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "value") },
  { selection: "unit", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "unit") },
  { selection: "definition", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "definition") },
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "description") },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "icon") },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["quality"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Quality>[]);
installKitOperationMethods(Quality, QUALITY_OPERATIONS);
//#endregion 💎Quality

//#region 🏷️Tag
/** @emoji 🏷️ Tag artifact: {@code TagOperationInput} leaves + kit-scoped reads. */
export class Tag extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  kitInnerPath(inner: string): string {
    return `tag(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare order: () => Promise<number | null>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare rename: (newName: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;
}

installKitFieldMethods(Tag, defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "description") },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "icon") },
  { selection: "order", parse: (frag) => readKitBranchNumberOrNull(frag as JsonObject | null, "tag", "order") },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["tag"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Tag>[]);
installKitOperationMethods(Tag, defineBoundKitOperations([
  { buildInner: (_entity, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const));
//#endregion 🏷️Tag

//#region 💡Concept
/** @emoji 💡 Concept artifact: {@code ConceptOperationInput} leaves + kit-scoped reads. */
export class Concept extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  kitInnerPath(inner: string): string {
    return `concept(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare order: () => Promise<number | null>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare rename: (newName: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;
}

installKitFieldMethods(Concept, defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "description") },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "icon") },
  { selection: "order", parse: (frag) => readKitBranchNumberOrNull(frag as JsonObject | null, "concept", "order") },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["concept"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Concept>[]);
installKitOperationMethods(Concept, defineBoundKitOperations([
  { buildInner: (_entity, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const));
//#endregion 💡Concept

//#region 🎨Representation
/** @emoji 🎨 Representation under {@link Type}: read-only until schema adds {@code RepresentationOperationInput}. */
export class Representation extends Entity {
  readonly typeId: string;
  constructor(session: Session, typeId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.typeId = typeId;
  }

  kitInnerPath(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { representation(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  declare name: () => Promise<string>;
  declare url: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare file: () => Promise<File | null>;
  declare tags: () => Promise<readonly Tag[]>;
  declare qualities: () => Promise<readonly Quality[]>;
  declare attributes: () => Promise<readonly Attribute[]>;
}

installKitFieldMethods(Representation, defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "name") },
  { selection: "url", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "url") },
  { selection: "description", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "description") },
  { selection: "icon", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "icon") },
  {
    selection: "file { id }",
    parse: () => null,
    parseEntity: (entity, frag) => {
      const id = String((readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION)?.["file"] as JsonObject | undefined)?.["id"] ?? "");
      return id === "" ? null : new File(entity.session, id, entity.storeId);
    },
  },
  {
    selection: "tags { edges { node { id } } }",
    parse: () => [],
    parseEntity: (entity, frag) => Object.freeze(parseIdListConnection(readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION), "tags").map((id) => new Tag(entity.session, id, entity.storeId))),
  },
  {
    selection: "qualities { edges { node { id } } }",
    parse: () => [],
    parseEntity: (entity, frag) => Object.freeze(parseIdListConnection(readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION), "qualities").map((id) => new Quality(entity.session, id, entity.storeId))),
  },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION)),
  },
]) as readonly BoundKitFieldSpec<unknown, Representation>[]);
//#endregion 🎨Representation

//#region 👨‍👩‍👦Family
/** @emoji 👨‍👩‍👦 Family artifact: read-only in current kit API. */
export class Family extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
}

installNodeFieldMethods(Family, "Family", defineBoundNodeFields([
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { selection: "icon", parse: (node) => String(node?.["icon"] ?? "") },
] as const));
//#endregion 👨‍👩‍👦Family

//#region 📄File
export class File extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare name: () => Promise<string>;
}

installNodeFieldMethods(File, "File", defineBoundNodeFields([
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
] as const));
//#endregion 📄File

//#region 📁Folder
/** @emoji 📁 Folder artifact: read-only in current kit API. */
export class Folder extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare path: () => Promise<string>;
}

installNodeFieldMethods(Folder, "Folder", defineBoundNodeFields([
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { selection: "path", parse: (node) => String(node?.["path"] ?? "") },
] as const));
//#endregion 📁Folder

//#region 🪟Layer
/** @emoji 🪟 Design {@link Layer}: read-only in current kit API. */
export class Layer extends Entity {
  readonly designId: string;
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  private lsel(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { layers { edges { node { id ${inner} } } } }`;
  }

  private async selfLayerNode(innerFields: string): Promise<JsonObject | null> {
    const frag = (await this.readKitInner(this.lsel(innerFields))) as JsonObject | null;
    const edges = (((frag?.["design"] as JsonObject | undefined)?.["layers"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      if (n && String(n["id"] ?? "") === this.id) return n;
    }
    return null;
  }

  async name(): Promise<string> {
    const n = await this.selfLayerNode("name");
    return String(n?.["name"] ?? "");
  }

  async description(): Promise<string> {
    const n = await this.selfLayerNode("description");
    return String(n?.["description"] ?? "");
  }

  async icon(): Promise<string> {
    const n = await this.selfLayerNode("icon");
    return String(n?.["icon"] ?? "");
  }

  async color(): Promise<string> {
    const n = await this.selfLayerNode("color");
    return String(n?.["color"] ?? "");
  }

  async order(): Promise<number | null> {
    const n = await this.selfLayerNode("order");
    const o = n?.["order"];
    return typeof o === "number" ? o : null;
  }

  async visible(): Promise<boolean | null> {
    const n = await this.selfLayerNode("visible");
    const v = n?.["visible"];
    return typeof v === "boolean" ? v : null;
  }

  async locked(): Promise<boolean | null> {
    const n = await this.selfLayerNode("locked");
    const v = n?.["locked"];
    return typeof v === "boolean" ? v : null;
  }
}
//#endregion 🪟Layer

//#region 👥Group
export class Group extends Entity {
  readonly designId: string;
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  async name(): Promise<string> {
    const frag = await this.readKitInner(`design(id: ${gqlString(this.designId)}) { groups { edges { node { id name } } } }`);
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
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare key: () => Promise<string>;
  declare value: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
}

installNodeFieldMethods(Stat, "Stat", defineBoundNodeFields([
  { selection: "key", parse: (node) => String(node?.["key"] ?? "") },
  { selection: "value", parse: (node) => String(node?.["value"] ?? "") },
  { selection: "unit", parse: (node) => String(node?.["unit"] ?? "") },
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { selection: "icon", parse: (node) => String(node?.["icon"] ?? "") },
] as const));
//#endregion 📊Stat

//#region 🎚️Prop
/** @emoji 🎚️ Prop artifact: read-only in current kit API. */
export class Prop extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare key: () => Promise<string>;
  declare value: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare name: () => Promise<string>;
  declare quality: () => Promise<Quality | null>;
}

installNodeFieldMethods(Prop, "Prop", defineBoundNodeFields([
  { selection: "key", parse: (node) => String(node?.["key"] ?? "") },
  { selection: "value", parse: (node) => String(node?.["value"] ?? "") },
  { selection: "unit", parse: (node) => String(node?.["unit"] ?? "") },
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  {
    selection: "quality { id }",
    parse: (node) => {
      const id = String((node?.["quality"] as JsonObject | undefined)?.["id"] ?? "");
      return id === "" ? null : new Quality((undefined as never), id);
    },
  },
] as const));
//#endregion 🎚️Prop

//#endregion 🧱Classes

//#region 🚀PublicAPI
/** @emoji 🚀 Opens a {@link Session} backed by rs WASM (worker or inline). */
export async function openSession(uri: string, opts?: SessionOpenOptions): Promise<Session> {
  return Session.open(uri, opts);
}

/** @emoji 🚀 Opens a {@link Session} against native `semio-store` HTTP GraphQL. */
export async function openSessionHttp(baseUrl: string, opts?: SessionHttpOpenOptions): Promise<Session> {
  return Session.openHttp(baseUrl, opts);
}

/** @emoji 🧾 Resolves a wasm-backed kit store from a kit client when `internalKs` exists; otherwise null (HTTP line, stubs). */
export function kitStoreFromKitStoreClient(_client: unknown): null {
  return null;
}
//#endregion 🚀PublicAPI


//#region 🧪Tests
if (typeof process !== "undefined" && !!process.env && process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  const { describe, it, expect } = await import("vitest");
  const eventually = async <T>(read: () => Promise<T>, matches: (value: T) => boolean, timeoutMs = 5_000): Promise<T> => {
    const startedAt = Date.now();
    let lastValue = await read();
    while (!matches(lastValue)) {
      if (Date.now() - startedAt >= timeoutMs) throw new Error(`eventually: timed out after ${timeoutMs}ms`);
      await new Promise((resolve) => setTimeout(resolve, 100));
      lastValue = await read();
    }
    return lastValue;
  };
  describe("semio/js", () => {
    it("graphql wire kinds match golden operation roots", () => {
      expect(graphqlWireOperationKind("  #c\nquery X { session { __typename } }")).toBe("query");
      expect(graphqlWireOperationKind("mutation { session { start } }")).toBe("mutation");
      expect(graphqlWireOperationKind("subscription { operation { id } }")).toBe("subscription");
      expect(() => assertGraphqlWireKind("mutation { session { start } }", "query")).toThrow(/expected query/);
    });
    it("graphql wire post json always carries query variables operationName", () => {
      const raw = graphqlWirePostBodyJson({ query: "query Q { __typename }" });
      const o = JSON.parse(raw) as Record<string, unknown>;
      expect(Object.keys(o).sort()).toEqual(["operationName", "query", "variables"]);
      expect(o["query"]).toBe("query Q { __typename }");
      expect(o["variables"]).toEqual({});
      expect(o["operationName"]).toBe(null);
      expect(
        JSON.parse(graphqlWirePostBodyJson({ query: "mutation { __typename }", variables: { a: 1 }, operationName: "M" })) as Record<string, unknown>,
      ).toEqual({ query: "mutation { __typename }", variables: { a: 1 }, operationName: "M" });
    });
    it("runs the in-memory rs graphql js pipeline", async () => {
      const session = await Session.openInMemory({ timeoutMs: 120_000 });
      try {
        const envelope = await session.gql.executeQueryJson({ query: KIT_SESSION_QUERY_ENTRY }, 120_000);
        expect(envelope.errors ?? []).toHaveLength(0);

        const stores = await session.stores();
        expect(stores.length).toBeGreaterThan(0);

        const store = stores[0]!;
        const kit = await store.wip().theKit().kit();

        const createTag = await kit.createTag("alpha-tag");
        expect(createTag).toEqual({ ok: true });
        const tags = await eventually(() => kit.tags(), (value) => value.length === 1, 10_000);
        expect(tags).toHaveLength(1);
        expect(await tags[0]!.name()).toBe("alpha-tag");

        const createConcept = await kit.createConcept("beta-concept");
        expect(createConcept).toEqual({ ok: true });
        const concepts = await eventually(() => kit.concepts(), (value) => value.length === 1, 10_000);
        expect(concepts).toHaveLength(1);
        expect(await concepts[0]!.name()).toBe("beta-concept");

        const createQuality = await kit.createQuality("q1", "v1");
        expect(createQuality).toEqual({ ok: true });
        const qualities = await eventually(() => kit.qualities(), (value) => value.length === 1, 10_000);
        expect(qualities).toHaveLength(1);
        expect(await qualities[0]!.key()).toBe("q1");

        const snapshot = unwrapGraphqlData(
          await session.gql.executeQueryJson(
            {
              query: `query PipelineSnapshot { session { stores { edges { node { wip { theKit { kit { tags { edges { node { name } } } concepts { edges { node { name } } } qualities { edges { node { key value } } } } } } } } } } }`,
            },
            120_000,
          ),
        ) as JsonObject;
        const storeNode = sessionStoreNodeFromData(snapshot);
        const kitNode = kitReadSelectionFromData(snapshot, theKitReadPoint);
        expect(String(jsonObjectField(kitNode, "tags")?.["edges"] instanceof Array)).toBe("true");
        expect(String(jsonObjectField(kitNode, "concepts")?.["edges"] instanceof Array)).toBe("true");
        expect(String(jsonObjectField(kitNode, "qualities")?.["edges"] instanceof Array)).toBe("true");
        const liveKit = jsonObjectField(jsonObjectField(storeNode, "wip"), "theKit")?.["kit"] as JsonObject | undefined;
        expect((((jsonObjectField(liveKit, "tags")?.["edges"] as JsonValue[] | undefined) ?? []).length)).toBe(1);
        expect((((jsonObjectField(liveKit, "concepts")?.["edges"] as JsonValue[] | undefined) ?? []).length)).toBe(1);
        expect((((jsonObjectField(liveKit, "qualities")?.["edges"] as JsonValue[] | undefined) ?? []).length)).toBe(1);
      } finally {
        await session.dispose();
      }
    });
  })
}

//#endregion 🧪Tests

