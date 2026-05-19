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
    query: `mutation($storeId: ID!, $changeId: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { ${withResponseSelection(kitSelection)} } } } } } }`,
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

/** @emoji 📬 Selection for golden {@code Response} on command mutation leaves. */
const GQL_RESPONSE_SELECTION =
  "ok errors { kind message requestId } result { ... on IdResult { value } }";

/** @emoji 📬 Parses a {@code Response} object from mutation data. */
function parseResponsePayload(node: JsonValue | undefined): SetResult {
  if (!isJsonObjectNode(node)) return { ok: true };
  if (node["ok"] === false) {
    const err = jsonObjectField(node, "errors");
    return {
      ok: false,
      error: {
        kind: (String(err?.["kind"] ?? "Internal") as SetErrorKind),
        message: String(err?.["message"] ?? "command failed"),
      },
    };
  }
  return { ok: true };
}

/** @emoji 🆔 Reads {@code IdResult.value} from a {@code Response} payload. */
function responseResultId(node: JsonValue | undefined): string {
  if (!isJsonObjectNode(node)) return "";
  const result = jsonObjectField(node, "result");
  const value = result?.["value"];
  return value == null ? "" : String(value);
}

/** @emoji 📬 Appends {@link GQL_RESPONSE_SELECTION} to the innermost kit command field. */
function withResponseSelection(kitSelection: string): string {
  const trimmed = kitSelection.trim();
  const brace = trimmed.indexOf("{");
  if (brace === -1) return `${trimmed} { ${GQL_RESPONSE_SELECTION} }`;
  const head = trimmed.slice(0, brace).trimEnd();
  const inner = trimmed.slice(brace + 1, trimmed.lastIndexOf("}")).trim();
  return `${head} { ${withResponseSelection(inner)} }`;
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
type KitPathEntity = Entity & {
  kitInnerPath(inner: string): string;
};

type BoundKitFieldSpec<T, E extends KitPathEntity = KitPathEntity> = Readonly<{
  selection: string;
  parse: (v: JsonValue) => T;
  parseEntity?: (entity: E, v: JsonValue) => T;
  /** @emoji 📖 Prototype method name when it differs from the GraphQL field (e.g. {@code typeId} for {@code type { id }}). */
  method?: string;
  /** @emoji 📡 Bus {@code kind}; defaults via {@link defaultFieldEventKind}. */
  eventKind?: string;
  /** @emoji 📡 List/connection fields: invalidate on {@code commandSucceeded} + {@code kitRenamed}. */
  coarseEvent?: boolean;
}>;

type BoundNodeFieldSpec<T> = Readonly<{
  selection: string;
  parse: (node: JsonObject | undefined) => T;
  /** @emoji 📖 Prototype method name when it differs from the GraphQL selection root. */
  method?: string;
}>;

type BoundKitOperationSpec<E extends KitPathEntity> = Readonly<{
  /** @emoji 🎬 Prototype method name (GraphQL operation leaf). */
  method: string;
  buildInner: (entity: E, ...args: readonly unknown[]) => string;
}>;

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

/** @emoji 🏭 Installs kit-relative read methods on a prototype so classes stay declarative and schema-shaped. */
function installKitFieldMethods<E extends KitPathEntity>(
  ctor: abstract new (...args: never[]) => E,
  specs: readonly BoundKitFieldSpec<unknown, E>[],
): void {
  for (const spec of specs) {
    const method = spec.method ?? schemaFieldName(spec.selection);
    Object.defineProperty(ctor.prototype, method, {
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
    const method = spec.method ?? schemaFieldName(spec.selection);
    Object.defineProperty(ctor.prototype, method, {
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
    Object.defineProperty(ctor.prototype, spec.method, {
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
    const fieldName = spec.method ?? schemaFieldName(spec.selection);
    const readMethod = fieldName;
    const eventMethod = fieldChangedEventMethodName(fieldName);
    const eventKind = spec.eventKind ?? defaultFieldEventKind(entityName, fieldName);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioKitFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const run = (): void => {
          const read = (this as unknown as Record<string, () => Promise<unknown>>)[readMethod];
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
    const fieldName = spec.method ?? schemaFieldName(spec.selection);
    const eventMethod = fieldChangedEventMethodName(fieldName);
    const eventKind = defaultFieldEventKind(ctor.name, fieldName);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioNodeFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const read = (this as unknown as Record<string, () => Promise<unknown>>)[fieldName];
        return this.session.bus.subscribeKind(eventKind, () => {
          if (typeof read === "function") void read.call(this).then(cb);
        });
      },
      writable: true,
    });
  }
}

type StoreBranchEntity = Entity & Readonly<{
  storeBranchPath(selection: string): string;
  parseStoreBranch(frag: JsonObject | null): JsonObject | null;
  readStoreBranch(selection: string): Promise<JsonObject | null>;
}>;

type BoundStoreBranchFieldSpec<T, E extends StoreBranchEntity> = Readonly<{
  selection: string;
  parse: (branch: JsonObject | null) => T;
  parseEntity?: (entity: E, branch: JsonObject | null) => T;
  method?: string;
  eventKind?: string;
  coarseEvent?: boolean;
}>;

function defineBoundStoreBranchFields<const S extends readonly BoundStoreBranchFieldSpec<unknown, StoreBranchEntity>[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Installs store-scoped field reads and {@code onFieldChanged} on a nested branch (VCS graph roots, checkpoints, …). */
function installStoreBranchFieldMethods<E extends StoreBranchEntity>(
  ctor: abstract new (...args: never[]) => E,
  specs: readonly BoundStoreBranchFieldSpec<unknown, E>[],
): void {
  const entityName = ctor.name;
  for (const spec of specs) {
    const method = spec.method ?? schemaFieldName(spec.selection);
    Object.defineProperty(ctor.prototype, method, {
      configurable: true,
      value: async function semioStoreBranchField(this: E): Promise<unknown> {
        const frag = await this.readStoreBranch(spec.selection);
        const branch = this.parseStoreBranch(frag);
        return spec.parseEntity != null ? spec.parseEntity(this, branch) : spec.parse(branch);
      },
      writable: true,
    });
    const eventMethod = fieldChangedEventMethodName(method);
    const eventKind = spec.eventKind ?? defaultFieldEventKind(entityName, method);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioStoreBranchFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const run = (): void => {
          const read = (this as unknown as Record<string, () => Promise<unknown>>)[method];
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

/** @emoji 🏭 Installs store-scoped field reads, events, and optional branch helpers on one entity class. */
function installEntityStoreBranchMethods<E extends StoreBranchEntity>(
  ctor: abstract new (...args: never[]) => E,
  fields: readonly BoundStoreBranchFieldSpec<unknown, E>[],
): void {
  installStoreBranchFieldMethods(ctor, fields);
}

type ScopedNodeEntity = Entity & Readonly<{
  scopedNodePath(selection: string): string;
  readScopedNode(selection: string): Promise<JsonObject | null>;
}>;

type BoundScopedNodeFieldSpec<T> = Readonly<{
  selection: string;
  parse: (node: JsonObject | null) => T;
  eventKind?: string;
  coarseEvent?: boolean;
}>;

function defineBoundScopedNodeFields<const S extends readonly BoundScopedNodeFieldSpec<unknown>[]>(specs: S): S {
  return specs;
}

/** @emoji 🏭 Installs field reads on a store-nested node already resolved by {@link ScopedNodeEntity#readScopedNode}. */
function installEntityScopedNodeMethods<E extends ScopedNodeEntity>(
  ctor: abstract new (...args: never[]) => E,
  fields: readonly BoundScopedNodeFieldSpec<unknown>[],
): void {
  const entityName = ctor.name;
  for (const spec of fields) {
    const method = schemaFieldName(spec.selection);
    Object.defineProperty(ctor.prototype, method, {
      configurable: true,
      value: async function semioScopedNodeField(this: E): Promise<unknown> {
        const node = await this.readScopedNode(spec.selection);
        return spec.parse(node);
      },
      writable: true,
    });
    const eventMethod = fieldChangedEventMethodName(method);
    const eventKind = spec.eventKind ?? defaultFieldEventKind(entityName, method);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioScopedNodeFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const run = (): void => {
          const read = (this as unknown as Record<string, () => Promise<unknown>>)[method];
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

type BoundWeakFieldSpec<T> = Readonly<{
  method: string;
  selection: string;
  parse: (role: JsonObject | null) => T;
  coarseEvent?: boolean;
}>;

/** @emoji 🏭 Installs async scalar reads on weak kit-nested artifacts ({@link Coordinate}, {@link Point}, …). */
function installWeakKitFieldMethods<E extends object>(
  ctor: abstract new (...args: never[]) => E,
  readRole: (self: E, selection: string) => Promise<JsonObject | null>,
  parseRole: (self: E, frag: JsonObject | null) => JsonObject | null,
  specs: readonly BoundWeakFieldSpec<unknown>[],
  bus: (self: E) => EventBus,
): void {
  for (const spec of specs) {
    Object.defineProperty(ctor.prototype, spec.method, {
      configurable: true,
      value: async function semioWeakKitField(this: E): Promise<unknown> {
        const frag = await readRole(this, spec.selection);
        return spec.parse(parseRole(this, frag));
      },
      writable: true,
    });
    const eventMethod = fieldChangedEventMethodName(spec.method);
    Object.defineProperty(ctor.prototype, eventMethod, {
      configurable: true,
      value: function semioWeakKitFieldEvent(this: E, cb: (next: unknown) => void): Unsubscribe {
        const run = (): void => {
          const read = (this as unknown as Record<string, () => Promise<unknown>>)[spec.method];
          if (typeof read !== "function") return;
          void read.call(this).then(cb);
        };
        const b = bus(this);
        if (spec.coarseEvent) return subscribeKitCoarseRefetch(b, run);
        return b.subscribeKind("commandSucceeded", () => run());
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

/** @emoji 🏁 Parses {@code benchmarks { edges { node { … } } }} under a {@link Quality} kit branch. */
function parseBenchmarkConnectionUnder(quality: Quality, owner: JsonObject | null | undefined): readonly Benchmark[] {
  const bench = owner?.["benchmarks"] as JsonObject | undefined;
  const edges = bench?.["edges"] as readonly JsonValue[] | undefined;
  if (!Array.isArray(edges)) return [];
  const out: Benchmark[] = [];
  for (const e of edges) {
    if (!isJsonObjectNode(e)) continue;
    const n = e["node"] as JsonObject | undefined;
    if (n == null) continue;
    out.push(
      new Benchmark(
        quality,
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
  const kinds = ["commandSucceeded", "kitRenamed", "draggedPiece", "fixedPiece"] as const;
  const subs = kinds.map((kind) => bus.subscribeKind(kind, run));
  return (): void => {
    for (const off of subs) off();
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
    let failed: SetResult | null = null;
    const visit = (value: JsonValue | undefined): void => {
      if (isJsonObjectNode(value) && "ok" in value) {
        const r = parseResponsePayload(value);
        if (!r.ok) failed = r;
        const id = responseResultId(value);
        if (id !== "") this.trackCommandId(id);
        return;
      }
      if (Array.isArray(value)) {
        for (const item of value) visit(item);
      } else if (isJsonObjectNode(value)) {
        for (const item of Object.values(value)) visit(item);
      }
    };
    visit(env.data ?? undefined);
    if (failed != null) return failed;
    return gqlOkFromEnvelope(env);
  }

  async ensureChangeId(storeId?: string): Promise<string> {
    this.ensureAlive();
    if (storeId == null || storeId === "") throw new Error("store id is required for store-scoped change");
    const data = unwrapGraphqlData(
      await this.mutateEnvelope({
        query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange { ${GQL_RESPONSE_SELECTION} } } } } }`,
        variables: { storeId },
      }),
    ) as JsonObject;
    const start = ((data["session"] as JsonObject | undefined)?.["store"] as JsonObject | undefined)?.["theKit"] as JsonObject | undefined;
    const payload = start?.["startNewChange"];
    const r = parseResponsePayload(payload);
    if (!r.ok) throw new Error(r.error.message);
    const cid = responseResultId(payload);
    if (cid === "") throw new Error("startNewChange: empty change id");
    return this.trackCommandId(cid);
  }

  async saveChange(storeId?: string): Promise<void> {
    this.ensureAlive();
    if (storeId == null || storeId === "") throw new Error("store id is required for store-scoped save");
    const env = await this.mutateEnvelope({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { save { ${GQL_RESPONSE_SELECTION} } } } } }`,
      variables: { storeId },
    });
    this.trackCommandResult(env);
  }

  async startNewChange(storeId: string): Promise<ChangeId> {
    return await this.ensureChangeId(storeId);
  }

  async createCheckpoint(storeId: string, message: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { createCheckpoint(message: ${gqlString(message)}) { ${GQL_RESPONSE_SELECTION} } } } } }`,
      variables: { storeId },
    });
    return this.trackCommandResult(env);
  }

  async startAlternative(storeId: string, name?: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query:
        name == null
          ? `mutation($storeId: ID!) { session { store(id: $storeId) { startAlternative { ${GQL_RESPONSE_SELECTION} } } } }`
          : `mutation($storeId: ID!) { session { store(id: $storeId) { startAlternative(name: ${gqlString(name)}) { ${GQL_RESPONSE_SELECTION} } } } }`,
      variables: { storeId },
    });
    return this.trackCommandResult(env);
  }

  async integrateAlternative(storeId: string, alternativeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { alternative(id: ${gqlString(alternativeId)}) { integrateIntoTheKit { ${GQL_RESPONSE_SELECTION} } } } } }`,
      variables: { storeId },
    });
    return this.trackCommandResult(env);
  }

  async login(username: string, passwordHash: string, hubUrl?: string): Promise<SetResult> {
    this.ensureAlive();
    const url = hubUrl ?? "";
    const h = hubUrl == null ? "null" : gqlString(hubUrl);
    const env = await this.mutateEnvelope({
      query: `mutation { session { remoteProvider(url: ${gqlString(url)}) { login(username: ${gqlString(username)}, passwordHash: ${gqlString(passwordHash)}, hubUrl: ${h}) { ${GQL_RESPONSE_SELECTION} } } } }`,
    });
    return this.trackCommandResult(env);
  }

  async logout(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation { session { remoteProvider(url: "") { logout { ${GQL_RESPONSE_SELECTION} } } } }` });
    return this.trackCommandResult(env);
  }

  async sessionStart(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation { session { start { ${GQL_RESPONSE_SELECTION} } } }` });
    return this.trackCommandResult(env);
  }

  async sessionEnd(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({ query: `mutation { session { end { ${GQL_RESPONSE_SELECTION} } } }` });
    return this.trackCommandResult(env);
  }

  async attachBackbone(storeId: string, provider: Provider, uri: string): Promise<SetResult> {
    return provider.ensureBackboneAttached(storeId, uri);
  }

  async detachBackbone(storeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { detach { ${GQL_RESPONSE_SELECTION} } } } } }`,
      variables: { storeId },
    });
    return this.trackCommandResult(env);
  }

  /** @emoji 🛜 Runs target {@code BackboneCommand.sync} through the given store command scope. */
  async backboneSyncNow(storeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.mutateEnvelope({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { sync { ${GQL_RESPONSE_SELECTION} } } } } }`,
      variables: { storeId },
    });
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
/** @emoji 📦 Target-schema kit entity beneath {@link Version}; one read + one change event per field, one method per command. */
export class Kit extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  kitInnerPath(inner: string): string {
    return inner;
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare image: () => Promise<string>;
  declare preview: () => Promise<string>;
  declare remote: () => Promise<string>;
  declare homepage: () => Promise<string>;
  declare license: () => Promise<string>;
  declare uri: () => Promise<string>;
  declare designs: () => Promise<readonly Design[]>;
  declare types: () => Promise<readonly Type[]>;
  declare authors: () => Promise<readonly Author[]>;
  declare qualities: () => Promise<readonly Quality[]>;
  declare tags: () => Promise<readonly Tag[]>;
  declare concepts: () => Promise<readonly Concept[]>;
  declare onNameChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onDescriptionChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onIconChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onImageChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onPreviewChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onRemoteChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onHomepageChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onLicenseChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onUriChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onDesignsChanged: (cb: (next: readonly Design[]) => void) => Unsubscribe;
  declare onTypesChanged: (cb: (next: readonly Type[]) => void) => Unsubscribe;
  declare onAuthorsChanged: (cb: (next: readonly Author[]) => void) => Unsubscribe;
  declare onQualitiesChanged: (cb: (next: readonly Quality[]) => void) => Unsubscribe;
  declare onTagsChanged: (cb: (next: readonly Tag[]) => void) => Unsubscribe;
  declare onConceptsChanged: (cb: (next: readonly Concept[]) => void) => Unsubscribe;
  declare rename: (newName: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare createTag: (name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>;
  declare deleteTag: (id: string) => Promise<SetResult>;
  declare deleteTags: (ids: readonly string[]) => Promise<SetResult>;
  declare createConcept: (name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>;
  declare deleteConcept: (id: string) => Promise<SetResult>;
  declare deleteConcepts: (ids: readonly string[]) => Promise<SetResult>;
  declare createQuality: (key: string, value?: string | null, unit?: string | null, definition?: string | null, description?: string | null, icon?: string | null) => Promise<SetResult>;
  declare deleteQuality: (id: string) => Promise<SetResult>;
  declare deleteQualities: (ids: readonly string[]) => Promise<SetResult>;
  declare createType: (name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>;
  declare deleteType: (id: string) => Promise<SetResult>;
  declare deleteTypes: (ids: readonly string[]) => Promise<SetResult>;
  declare createDesign: (name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>;
  declare deleteDesign: (id: string) => Promise<SetResult>;
  declare deleteDesigns: (ids: readonly string[]) => Promise<SetResult>;
}

const KIT_FIELDS = defineBoundKitFields([
  { selection: "name", parse: (frag) => String((frag as JsonObject | null)?.["name"] ?? ""), eventKind: "kitRenamed" },
  { selection: "description", parse: (frag) => String((frag as JsonObject | null)?.["description"] ?? ""), eventKind: "changedDescription" },
  { selection: "icon", parse: (frag) => String((frag as JsonObject | null)?.["icon"] ?? "") },
  { selection: "image", parse: (frag) => String((frag as JsonObject | null)?.["image"] ?? "") },
  { selection: "preview", parse: (frag) => String((frag as JsonObject | null)?.["preview"] ?? "") },
  { selection: "remote", parse: (frag) => String((frag as JsonObject | null)?.["remote"] ?? "") },
  { selection: "homepage", parse: (frag) => String((frag as JsonObject | null)?.["homepage"] ?? "") },
  { selection: "license", parse: (frag) => String((frag as JsonObject | null)?.["license"] ?? "") },
  { selection: "uri", parse: (frag) => String((frag as JsonObject | null)?.["uri"] ?? "") },
  {
    selection: "designs { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseEntityConnectionIds(frag as JsonObject | null, "designs").map((id) => new Design(entity.session, id, entity.storeId))),
  },
  {
    selection: "types { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseEntityConnectionIds(frag as JsonObject | null, "types").map((id) => new Type(entity.session, id, entity.storeId))),
  },
  {
    selection: "authors { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseEntityConnectionIds(frag as JsonObject | null, "authors").map((id) => new Author(entity.session, id, entity.storeId))),
  },
  {
    selection: "qualities { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseEntityConnectionIds(frag as JsonObject | null, "qualities").map((id) => new Quality(entity.session, id, entity.storeId))),
  },
  {
    selection: "tags { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseEntityConnectionIds(frag as JsonObject | null, "tags").map((id) => new Tag(entity.session, id, entity.storeId))),
  },
  {
    selection: "concepts { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseEntityConnectionIds(frag as JsonObject | null, "concepts").map((id) => new Concept(entity.session, id, entity.storeId))),
  },
] as const);

const KIT_OPERATIONS = defineBoundKitOperations([
  { method: "rename", buildInner: (_e, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { method: "changeDescription", buildInner: (_e, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  {
    method: "createTag",
    buildInner: (_e, name, description, icon, order) =>
      `ct: createTag(name: ${gqlString(String(name ?? ""))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))}, order: ${order == null ? "null" : String(order)})`,
  },
  { method: "deleteTag", buildInner: (_e, id) => `dt: deleteTag(id: ${gqlString(String(id ?? ""))})` },
  { method: "deleteTags", buildInner: (_e, ids) => `dts: deleteTags(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "createConcept",
    buildInner: (_e, name, description, icon, order) =>
      `cc: createConcept(name: ${gqlString(String(name ?? ""))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))}, order: ${order == null ? "null" : String(order)})`,
  },
  { method: "deleteConcept", buildInner: (_e, id) => `dc: deleteConcept(id: ${gqlString(String(id ?? ""))})` },
  { method: "deleteConcepts", buildInner: (_e, ids) => `dcs: deleteConcepts(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "createQuality",
    buildInner: (_e, key, value, unit, definition, description, icon) =>
      `cq: createQuality(key: ${gqlString(String(key ?? ""))}, value: ${value == null ? "null" : gqlString(String(value))}, unit: ${unit == null ? "null" : gqlString(String(unit))}, definition: ${definition == null ? "null" : gqlString(String(definition))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))})`,
  },
  { method: "deleteQuality", buildInner: (_e, id) => `dq: deleteQuality(id: ${gqlString(String(id ?? ""))})` },
  { method: "deleteQualities", buildInner: (_e, ids) => `dqs: deleteQualities(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "createType",
    buildInner: (_e, name, description, icon, image, unit) =>
      `cT: createType(name: ${gqlString(String(name ?? ""))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))}, image: ${image == null ? "null" : gqlString(String(image))}, unit: ${unit == null ? "null" : gqlString(String(unit))})`,
  },
  { method: "deleteType", buildInner: (_e, id) => `dT: deleteType(id: ${gqlString(String(id ?? ""))})` },
  { method: "deleteTypes", buildInner: (_e, ids) => `dTs: deleteTypes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "createDesign",
    buildInner: (_e, name, description, icon, image, unit) =>
      `cD: createDesign(name: ${gqlString(String(name ?? ""))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))}, image: ${image == null ? "null" : gqlString(String(image))}, unit: ${unit == null ? "null" : gqlString(String(unit))})`,
  },
  { method: "deleteDesign", buildInner: (_e, id) => `dD: deleteDesign(id: ${gqlString(String(id ?? ""))})` },
  { method: "deleteDesigns", buildInner: (_e, ids) => `dDs: deleteDesigns(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const);

installEntityKitMethods(Kit, KIT_FIELDS as readonly BoundKitFieldSpec<unknown, Kit>[], KIT_OPERATIONS);
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
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { detach { ${GQL_RESPONSE_SELECTION} } } } } }`,
      variables: { storeId: this.id },
    });
    return this.session.mutationReceipt(env);
  }

  async syncBackbone(): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { sync { ${GQL_RESPONSE_SELECTION} } } } } }`,
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
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation { session { ${this.commandSelection} { createBackbone(uri: ${gqlString(uri)}) { ${GQL_RESPONSE_SELECTION} } } } }`,
    });
    return this.session.mutationReceipt(env);
  }

  async attachBackbone(storeId: string): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation($storeId: ID!) { session { ${this.commandSelection} { attachBackbone(store: $storeId) { ${GQL_RESPONSE_SELECTION} } } } }`,
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
      query: `mutation { session { ${this.commandSelection} { login(username: ${gqlString(username)}, passwordHash: ${gqlString(passwordHash)}, hubUrl: ${h}) { ${GQL_RESPONSE_SELECTION} } } } }`,
    });
    return this.session.mutationReceipt(env);
  }

  async logout(): Promise<SetResult> {
    const env = await executeSessionWriteGraphql(this.session, {
      query: `mutation { session { ${this.commandSelection} { logout { ${GQL_RESPONSE_SELECTION} } } } }`,
    });
    return this.session.mutationReceipt(env);
  }
}

/** @emoji 🌐 VCS graph: {@code wip} / {@code authoritative} selections on {@link Store}. */
export class Graph extends Entity {
  constructor(session: Session, root: GraphRootKind, private readonly managedStoreId: string) {
    super(session, root);
  }

  get root(): GraphRootKind {
    return this.id as GraphRootKind;
  }

  storeBranchPath(selection: string): string {
    return `${this.root} { ${selection} }`;
  }

  parseStoreBranch(frag: JsonObject | null): JsonObject | null {
    return jsonObjectField(frag, this.root);
  }

  async readStoreBranch(selection: string): Promise<JsonObject | null> {
    return await this.session.readStoreInnerForId(this.managedStoreId, this.storeBranchPath(selection));
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

  declare hash: () => Promise<string>;
  declare alternatives: () => Promise<readonly Alternative[]>;
  declare checkpoints: () => Promise<readonly Checkpoint[]>;
}

installEntityStoreBranchMethods(
  Graph,
  defineBoundStoreBranchFields([
    { selection: "hash", parse: (branch) => String(branch?.["hash"] ?? "") },
    {
      selection: "alternatives { edges { node { id } } }",
      parse: () => [],
      coarseEvent: true,
      parseEntity: (entity, branch) =>
        Object.freeze(parseEntityConnectionIds(branch, "alternatives").map((id) => (entity as Graph).alternative(id))),
    },
    {
      selection: "checkpoints { edges { node { id } } }",
      parse: () => [],
      coarseEvent: true,
      parseEntity: (entity, branch) =>
        Object.freeze(parseEntityConnectionIds(branch, "checkpoints").map((id) => (entity as Graph).checkpoint(id))),
    },
  ] as const) as readonly BoundStoreBranchFieldSpec<unknown, Graph>[],
);

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

  private graphRoot(): GraphRootKind {
    return this.ap.parent === "graph" ? this.ap.root : "wip";
  }

  storeBranchPath(selection: string): string {
    return `${this.graphRoot()} { alternative(id: ${gqlString(this.id)}) { ${selection} } }`;
  }

  parseStoreBranch(frag: JsonObject | null): JsonObject | null {
    return jsonObjectField(jsonObjectField(frag, this.graphRoot()), "alternative");
  }

  async readStoreBranch(selection: string): Promise<JsonObject | null> {
    return await this.session.readStoreInnerForId(this.ap.storeId, this.storeBranchPath(selection));
  }

  declare name: () => Promise<string>;
  declare unsavedChangeCount: () => Promise<number>;
}

installEntityStoreBranchMethods(
  Alternative,
  defineBoundStoreBranchFields([
    { selection: "name", parse: (branch) => String(branch?.["name"] ?? "") },
    {
      selection: "unsavedChanges { edges { node { id } } }",
      method: "unsavedChangeCount",
      parse: () => 0,
      coarseEvent: true,
      parseEntity: (_entity, branch) => parseEntityConnectionIds(branch, "unsavedChanges").length,
    },
  ] as const) as readonly BoundStoreBranchFieldSpec<unknown, Alternative>[],
);

/** @emoji 🏛 {@code TheKit} under {@code wip}/{@code authoritative}. */
export class TheKit extends Entity {
  constructor(session: Session, readonly graphRoot: GraphRootKind, private readonly managedStoreId: string) {
    super(session, `theKit:${graphRoot}`);
  }

  /** @emoji 📦 Target {@code Version.kit} handle beneath this version node. */
  kitRef(id = "kit"): Kit {
    return new Kit(this.session, id, this.managedStoreId);
  }

  storeBranchPath(selection: string): string {
    return `${this.graphRoot} { theKit { ${selection} } }`;
  }

  parseStoreBranch(frag: JsonObject | null): JsonObject | null {
    return jsonObjectField(jsonObjectField(frag, this.graphRoot), "theKit");
  }

  async readStoreBranch(selection: string): Promise<JsonObject | null> {
    return await this.session.readStoreInnerForId(this.managedStoreId, this.storeBranchPath(selection));
  }

  declare kit: () => Promise<Kit>;
}

installEntityStoreBranchMethods(
  TheKit,
  defineBoundStoreBranchFields([
    {
      selection: "id",
      method: "kit",
      parse: () => null as unknown as Kit,
      parseEntity: (entity, branch) => (entity as TheKit).kitRef(String(branch?.["id"] ?? "kit")),
      coarseEvent: true,
    },
  ] as const) as readonly BoundStoreBranchFieldSpec<unknown, TheKit>[],
);

/** @emoji 🏁 {@code Checkpoint} under {@link Graph}. */
export class Checkpoint extends Entity {
  readonly graphRoot: GraphRootKind;
  constructor(session: Session, graphRoot: GraphRootKind, checkpointId: string, private readonly managedStoreId: string) {
    super(session, checkpointId);
    this.graphRoot = graphRoot;
  }

  storeBranchPath(selection: string): string {
    return `${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { ${selection} } }`;
  }

  parseStoreBranch(frag: JsonObject | null): JsonObject | null {
    return jsonObjectField(jsonObjectField(frag, this.graphRoot), "checkpoint");
  }

  async readStoreBranch(selection: string): Promise<JsonObject | null> {
    return await this.session.readStoreInnerForId(this.managedStoreId, this.storeBranchPath(selection));
  }

  change(changeId: string): Change {
    return new Change(this.session, this.graphRoot, this.id, changeId, this.managedStoreId);
  }

  edit(editId: string): Edit {
    return new Edit(this.session, this.graphRoot, this.id, editId, this.managedStoreId);
  }

  declare message: () => Promise<string>;
  declare timestamp: () => Promise<string | null>;
  declare hash: () => Promise<string>;
  declare changes: () => Promise<readonly Change[]>;
  declare edits: () => Promise<readonly Edit[]>;
}

installEntityStoreBranchMethods(
  Checkpoint,
  defineBoundStoreBranchFields([
    { selection: "message", parse: (branch) => String(branch?.["message"] ?? "") },
    {
      selection: "timestamp",
      parse: (branch) => {
        const ts = branch?.["timestamp"];
        return ts == null ? null : String(ts);
      },
    },
    { selection: "hash", parse: (branch) => String(branch?.["hash"] ?? "") },
    {
      selection: "changes { id }",
      parse: () => [],
      coarseEvent: true,
      parseEntity: (entity, branch) =>
        Object.freeze(parseStrongEntityArrayIds(branch, "changes").map((cid) => (entity as Checkpoint).change(cid))),
    },
    {
      selection: "edits { edges { node { id } } }",
      parse: () => [],
      coarseEvent: true,
      parseEntity: (entity, branch) =>
        Object.freeze(parseEntityConnectionIds(branch, "edits").map((eid) => (entity as Checkpoint).edit(eid))),
    },
  ] as const) as readonly BoundStoreBranchFieldSpec<unknown, Checkpoint>[],
);

/** @emoji 🔀 {@code Change} scoped to a {@link Checkpoint}. */
export class Change extends Entity {
  readonly graphRoot: GraphRootKind;
  readonly checkpointId: string;
  constructor(
    session: Session,
    graphRoot: GraphRootKind,
    checkpointId: string,
    changeId: string,
    private readonly managedStoreId: string,
  ) {
    super(session, changeId);
    this.graphRoot = graphRoot;
    this.checkpointId = checkpointId;
  }

  scopedNodePath(selection: string): string {
    return `${this.graphRoot} { checkpoint(id: ${gqlString(this.checkpointId)}) { change(id: ${gqlString(this.id)}) { ${selection} } } } }`;
  }

  async readScopedNode(selection: string): Promise<JsonObject | null> {
    const storeNode = await this.session.readStoreInnerForId(this.managedStoreId, this.scopedNodePath(selection));
    const cp = jsonObjectField(jsonObjectField(storeNode, this.graphRoot), "checkpoint");
    return (cp?.["change"] as JsonObject | undefined) ?? null;
  }

  declare description: () => Promise<string>;
  declare origin: () => Promise<string>;
  declare saved: () => Promise<boolean | null>;
  declare startedAt: () => Promise<string>;
  declare savedAt: () => Promise<string | null>;
}

installEntityScopedNodeMethods(
  Change,
  defineBoundScopedNodeFields([
    { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
    { selection: "origin", parse: (node) => String(node?.["origin"] ?? "") },
    {
      selection: "saved",
      parse: (node) => {
        const v = node?.["saved"];
        if (v == null) return null;
        return Boolean(v);
      },
    },
    {
      selection: "startedAt",
      parse: (node) => {
        const v = node?.["startedAt"];
        return v == null ? "" : String(v);
      },
    },
    {
      selection: "savedAt",
      parse: (node) => {
        const v = node?.["savedAt"];
        return v == null ? null : String(v);
      },
    },
  ] as const),
);

/** @emoji ✏️ {@code Edit} scoped to a {@link Checkpoint}. */
export class Edit extends Entity {
  readonly graphRoot: GraphRootKind;
  readonly checkpointId: string;
  constructor(
    session: Session,
    graphRoot: GraphRootKind,
    checkpointId: string,
    editId: string,
    private readonly managedStoreId: string,
  ) {
    super(session, editId);
    this.graphRoot = graphRoot;
    this.checkpointId = checkpointId;
  }

  scopedNodePath(selection: string): string {
    return `${this.graphRoot} { checkpoint(id: ${gqlString(this.checkpointId)}) { edit(id: ${gqlString(this.id)}) { ${selection} } } } }`;
  }

  async readScopedNode(selection: string): Promise<JsonObject | null> {
    const storeNode = await this.session.readStoreInnerForId(this.managedStoreId, this.scopedNodePath(selection));
    const cp = jsonObjectField(jsonObjectField(storeNode, this.graphRoot), "checkpoint");
    return (cp?.["edit"] as JsonObject | undefined) ?? null;
  }

  declare description: () => Promise<string>;
  declare origin: () => Promise<string>;
  declare sequenceNumber: () => Promise<number>;
  declare startedAt: () => Promise<string>;
}

installEntityScopedNodeMethods(
  Edit,
  defineBoundScopedNodeFields([
    { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
    { selection: "origin", parse: (node) => String(node?.["origin"] ?? "") },
    {
      selection: "sequenceNumber",
      parse: (node) => {
        const v = node?.["sequenceNumber"];
        return typeof v === "number" ? v : Number(v ?? NaN);
      },
    },
    {
      selection: "startedAt",
      parse: (node) => {
        const v = node?.["startedAt"];
        return v == null ? "" : String(v);
      },
    },
  ] as const),
);

/** @emoji ⚔️ {@code Conflict} via {@code node(id:)}. */
export class Conflict extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  declare reasons: () => Promise<readonly string[]>;
  declare authoritativeChangeId: () => Promise<string>;
  declare wipChangeId: () => Promise<string>;
}

installEntityNodeMethods(
  Conflict,
  "Conflict",
  defineBoundNodeFields([
    {
      selection: "reasons",
      parse: (node) => {
        const raw = node?.["reasons"] as readonly JsonValue[] | undefined;
        if (!Array.isArray(raw)) return [];
        return raw.map((x) => String(x));
      },
    },
    {
      selection: "authoritativeChange { id }",
      method: "authoritativeChangeId",
      parse: (node) => String((node?.["authoritativeChange"] as JsonObject | undefined)?.["id"] ?? ""),
    },
    {
      selection: "wipChange { id }",
      method: "wipChangeId",
      parse: (node) => String((node?.["wipChange"] as JsonObject | undefined)?.["id"] ?? ""),
    },
  ] as const) as readonly BoundNodeFieldSpec<unknown>[],
);

//#endregion 🧬VcsEntities

//#region 📐Design
/** @emoji 📐 Design artifact: declarative field reads, commands, and per-field change subscriptions. */
export class Design extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  private child<T extends Entity>(ctor: new (session: Session, designId: string, id: string, storeId?: string) => T, id: string): T {
    return new ctor(this.session, this.id, id, this.storeId);
  }

  kitInnerPath(inner: string): string {
    return `design(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  piece(pieceId: string): Piece {
    return this.child(Piece, pieceId);
  }

  declare pieces: {
    (): Promise<readonly Piece[]>;
    (pieceIds: readonly string[]): PiecesOperation;
  };

  connection(connectionId: string): Connection {
    return this.child(Connection, connectionId);
  }

  layer(layerId: string): Layer {
    return this.child(Layer, layerId);
  }

  group(groupId: string): Group {
    return this.child(Group, groupId);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare image: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare qualitySum: () => Promise<number>;
  declare connections: () => Promise<readonly Connection[]>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare onNameChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onDescriptionChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onIconChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onImageChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onUnitChanged: (cb: (next: string) => void) => Unsubscribe;
  declare onQualitySumChanged: (cb: (next: number) => void) => Unsubscribe;
  declare onPiecesChanged: (cb: (next: readonly Piece[]) => void) => Unsubscribe;
  declare onConnectionsChanged: (cb: (next: readonly Connection[]) => void) => Unsubscribe;
  declare onAttributesChanged: (cb: (next: readonly Attribute[]) => void) => Unsubscribe;
  declare rename: (newName: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
  declare flatten: () => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;
  declare addFixedPiece: (blueprintId: string, position: PositionInput, name?: string | null, description?: string | null) => Promise<SetResult>;
  declare addChildPieceWithParentConnection: (
    blueprintId: string,
    parentPieceId: string,
    parentConnector: string,
    childConnector: string,
    name?: string | null,
    description?: string | null,
    position?: PositionInput | null,
    scale?: number | null,
  ) => Promise<SetResult>;
  declare addHangingChildPieceWithParentConnection: (
    blueprintId: string,
    parentPieceId: string,
    parentConnector: string,
    childConnector: string,
    position: PositionInput,
    name?: string | null,
    description?: string | null,
    scale?: number | null,
  ) => Promise<SetResult>;
  declare deletePiece: (id: string) => Promise<SetResult>;
  declare deletePieces: (ids: readonly string[]) => Promise<SetResult>;
  declare deletePiecesAndConnections: (pieceIds: readonly string[], connectionIds: readonly string[]) => Promise<SetResult>;
}

function parseDesignBranchConnection(frag: JsonObject | null, key: string): readonly string[] {
  const d = frag?.["design"] as JsonObject | undefined;
  return parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), key);
}

/** @emoji 🧭 Descends {@code type → …} connection ids on kit JSON. */
function parseTypeBranchConnection(frag: JsonObject | null, key: string): readonly string[] {
  const t = frag?.["type"] as JsonObject | undefined;
  return parseEntityConnectionIds(t ?? (isJsonObjectNode(frag) ? frag : null), key);
}

/** @emoji 🧭 Scalar on a nested {@code design.layers|groups} edge node matched by id. */
function readDesignListNodeField(frag: JsonObject | null, listKey: "layers" | "groups", nodeId: string, field: string): string {
  const edges = (((frag?.["design"] as JsonObject | undefined)?.[listKey] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined) ?? [];
  for (const e of edges) {
    const n = e["node"] as JsonObject | undefined;
    if (n && String(n["id"] ?? "") === nodeId) return String(n[field] ?? "");
  }
  return "";
}

/** @emoji 🧭 Nullable number on a nested {@code design.layers} edge node. */
function readDesignListNodeNumberOrNull(frag: JsonObject | null, nodeId: string, field: string): number | null {
  const edges = (((frag?.["design"] as JsonObject | undefined)?.["layers"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined) ?? [];
  for (const e of edges) {
    const n = e["node"] as JsonObject | undefined;
    if (n && String(n["id"] ?? "") === nodeId) {
      const v = n[field];
      return typeof v === "number" ? v : null;
    }
  }
  return null;
}

/** @emoji 🧭 Nullable boolean on a nested {@code design.layers} edge node. */
function readDesignListNodeBooleanOrNull(frag: JsonObject | null, nodeId: string, field: string): boolean | null {
  const edges = (((frag?.["design"] as JsonObject | undefined)?.["layers"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined) ?? [];
  for (const e of edges) {
    const n = e["node"] as JsonObject | undefined;
    if (n && String(n["id"] ?? "") === nodeId) {
      const v = n[field];
      return typeof v === "boolean" ? v : null;
    }
  }
  return null;
}

/** @emoji 🧩 Scalar under {@code design.piece} on kit JSON. */
function readPieceBranchString(frag: JsonObject | null, field: string): string {
  return String(pieceKit(frag)?.[field] ?? "");
}

/** @emoji 🧩 Nullable scalar under {@code design.piece}. */
function readPieceBranchNumberOrNull(frag: JsonObject | null, field: string): number | null {
  const v = pieceKit(frag)?.[field];
  return typeof v === "number" ? v : null;
}

/** @emoji ⛓️ Scalar under {@code design.connection}. */
function readConnectionBranchString(frag: JsonObject | null, field: string): string {
  return String(connectionKit(frag)?.[field] ?? "");
}

/** @emoji ⛓️ Nullable scalar under {@code design.connection}. */
function readConnectionBranchNumberOrNull(frag: JsonObject | null, field: string): number | null {
  const v = connectionKit(frag)?.[field];
  return typeof v === "number" ? v : null;
}

const DESIGN_FIELDS = defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "design", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "design", "description"), eventKind: "changedDescription" },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "design", "icon") },
  { selection: "image", parse: (frag) => readKitBranchString(frag as JsonObject | null, "design", "image") },
  { selection: "unit", parse: (frag) => readKitBranchString(frag as JsonObject | null, "design", "unit") },
  { selection: "qualitySum", parse: (frag) => readKitBranchNumber(frag as JsonObject | null, "design", "qualitySum") },
  {
    selection: "pieces { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) =>
      Object.freeze(parseDesignBranchConnection(frag as JsonObject | null, "pieces").map((pid) => (entity as Design).piece(pid))),
  },
  {
    selection: "connections { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) =>
      Object.freeze(parseDesignBranchConnection(frag as JsonObject | null, "connections").map((cid) => (entity as Design).connection(cid))),
  },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["design"] as JsonObject | undefined),
  },
] as const);

const DESIGN_OPERATIONS = defineBoundKitOperations([
  { method: "rename", buildInner: (_e, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { method: "changeDescription", buildInner: (_e, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { method: "changeIcon", buildInner: (_e, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  { method: "flatten", buildInner: () => `fl: flatten` },
  {
    method: "addAttribute",
    buildInner: (_e, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { method: "removeAttribute", buildInner: (_e, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeAttributes", buildInner: (_e, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "addFixedPiece",
    buildInner: (_e, blueprintId, position, name, description) =>
      `afp: addFixedPiece(blueprintId: ${gqlString(String(blueprintId ?? ""))}, position: ${formatPositionInput(position as PositionInput)}, name: ${name == null ? "null" : gqlString(String(name))}, description: ${description == null ? "null" : gqlString(String(description))})`,
  },
  {
    method: "addChildPieceWithParentConnection",
    buildInner: (_e, blueprintId, parentPieceId, parentConnector, childConnector, name, description, position, scale) =>
      `ac: addChildPieceWithParentConnection(blueprintId: ${gqlString(String(blueprintId ?? ""))}, parentPieceId: ${gqlString(String(parentPieceId ?? ""))}, parentConnector: ${gqlString(String(parentConnector ?? ""))}, childConnector: ${gqlString(String(childConnector ?? ""))}, name: ${name == null ? "null" : gqlString(String(name))}, description: ${description == null ? "null" : gqlString(String(description))}, position: ${position == null ? "null" : formatPositionInput(position as PositionInput)}, scale: ${scale == null ? "null" : String(scale)})`,
  },
  {
    method: "addHangingChildPieceWithParentConnection",
    buildInner: (_e, blueprintId, parentPieceId, parentConnector, childConnector, position, name, description, scale) =>
      `ah: addHangingChildPieceWithParentConnection(blueprintId: ${gqlString(String(blueprintId ?? ""))}, parentPieceId: ${gqlString(String(parentPieceId ?? ""))}, parentConnector: ${gqlString(String(parentConnector ?? ""))}, childConnector: ${gqlString(String(childConnector ?? ""))}, position: ${formatPositionInput(position as PositionInput)}, name: ${name == null ? "null" : gqlString(String(name))}, description: ${description == null ? "null" : gqlString(String(description))}, scale: ${scale == null ? "null" : String(scale)})`,
  },
  { method: "deletePiece", buildInner: (_e, id) => `dp: deletePiece(id: ${gqlString(String(id ?? ""))})` },
  { method: "deletePieces", buildInner: (_e, ids) => `dps: deletePieces(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "deletePiecesAndConnections",
    buildInner: (_e, pieceIds, connectionIds) =>
      `dpc: deletePiecesAndConnections(pieceIds: ${gqlIdList((pieceIds as readonly string[]) ?? [])}, connectionIds: ${gqlIdList((connectionIds as readonly string[]) ?? [])})`,
  },
] as const);

installEntityKitMethods(Design, DESIGN_FIELDS as readonly BoundKitFieldSpec<unknown, Design>[], DESIGN_OPERATIONS);

{
  const readPieces = Design.prototype.pieces as unknown as (this: Design) => Promise<readonly Piece[]>;
  Object.defineProperty(Design.prototype, "pieces", {
    configurable: true,
    writable: true,
    value: function designPieces(this: Design, pieceIds?: readonly string[]): PiecesOperation | Promise<readonly Piece[]> {
      if (pieceIds != null) return new PiecesOperation(this.session, this.id, pieceIds, this.storeId);
      return readPieces.call(this);
    },
  });
}
//#endregion 📐Design

//#region 🧰Type
/** @emoji 🧰 Type artifact: declarative field reads, commands, and per-field change subscriptions. */
export class Type extends Entity {
  constructor(session: Session, id: string, storeId?: string) {
    super(session, id, storeId);
  }

  private child<T extends Entity>(ctor: new (session: Session, typeId: string, id: string, storeId?: string) => T, id: string): T {
    return new ctor(this.session, this.id, id, this.storeId);
  }

  kitInnerPath(inner: string): string {
    return `type(id: ${gqlString(this.id)}) { ${inner} }`;
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

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare image: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare ports: () => Promise<readonly Port[]>;
  declare connectors: () => Promise<readonly Connector[]>;
  declare representations: () => Promise<readonly Representation[]>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare authors: () => Promise<readonly Author[]>;
  declare rename: (newName: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;
  declare createPort: (code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>;
  declare deletePort: (id: string) => Promise<SetResult>;
  declare deletePorts: (ids: readonly string[]) => Promise<SetResult>;
  declare addConnector: (code: string, description?: string | null, icon?: string | null, portId?: string | null) => Promise<SetResult>;
  declare removeConnector: (id: string) => Promise<SetResult>;
  declare removeConnectors: (ids: readonly string[]) => Promise<SetResult>;
}

const TYPE_FIELDS = defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "type", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "type", "description"), eventKind: "changedDescription" },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "type", "icon") },
  { selection: "image", parse: (frag) => readKitBranchString(frag as JsonObject | null, "type", "image") },
  { selection: "unit", parse: (frag) => readKitBranchString(frag as JsonObject | null, "type", "unit") },
  {
    selection: "ports { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseTypeBranchConnection(frag as JsonObject | null, "ports").map((id) => (entity as Type).port(id))),
  },
  {
    selection: "connectors { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseTypeBranchConnection(frag as JsonObject | null, "connectors").map((id) => (entity as Type).connector(id))),
  },
  {
    selection: "representations { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseTypeBranchConnection(frag as JsonObject | null, "representations").map((id) => (entity as Type).representation(id))),
  },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["type"] as JsonObject | undefined),
  },
  {
    selection: "authors { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => Object.freeze(parseTypeBranchConnection(frag as JsonObject | null, "authors").map((id) => new Author(entity.session, id, entity.storeId))),
  },
] as const);

const TYPE_OPERATIONS = defineBoundKitOperations([
  { method: "rename", buildInner: (_e, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { method: "changeDescription", buildInner: (_e, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { method: "changeIcon", buildInner: (_e, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    method: "addAttribute",
    buildInner: (_e, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { method: "removeAttribute", buildInner: (_e, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeAttributes", buildInner: (_e, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "createPort",
    buildInner: (_e, code, label, description, icon, order) =>
      `cp: createPort(code: ${code == null ? "null" : gqlString(String(code))}, label: ${label == null ? "null" : gqlString(String(label))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))}, order: ${order == null ? "null" : String(order)})`,
  },
  { method: "deletePort", buildInner: (_e, id) => `dp: deletePort(id: ${gqlString(String(id ?? ""))})` },
  { method: "deletePorts", buildInner: (_e, ids) => `dps: deletePorts(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  {
    method: "addConnector",
    buildInner: (_e, code, description, icon, portId) =>
      `ac: addConnector(code: ${gqlString(String(code ?? ""))}, description: ${description == null ? "null" : gqlString(String(description))}, icon: ${icon == null ? "null" : gqlString(String(icon))}, portId: ${portId == null ? "null" : gqlString(String(portId))})`,
  },
  { method: "removeConnector", buildInner: (_e, id) => `rc: removeConnector(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeConnectors", buildInner: (_e, ids) => `rcs: removeConnectors(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const);

installEntityKitMethods(Type, TYPE_FIELDS as readonly BoundKitFieldSpec<unknown, Type>[], TYPE_OPERATIONS);
//#endregion 🧰Type

/** @emoji 🧷 Kit JSON path {@code type → port} for nested reads under {@link Port}. */
const KIT_PATH_TYPE_PORT: readonly string[] = ["type", "port"];
/** @emoji 🧷 Kit JSON path {@code type → connector}. */
const KIT_PATH_TYPE_CONNECTOR: readonly string[] = ["type", "connector"];
/** @emoji 🧷 Kit JSON path {@code type → representation}. */
const KIT_PATH_TYPE_REPRESENTATION: readonly string[] = ["type", "representation"];

//#region 🔘Port
/** @emoji 🔘 Port under {@link Type}: declarative field reads, commands, and change subscriptions. */
export class Port extends Entity {
  readonly typeId: string;
  constructor(session: Session, typeId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.typeId = typeId;
  }

  kitInnerPath(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { port(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  declare code: () => Promise<string>;
  declare label: () => Promise<string>;
  declare order: () => Promise<number | null>;
  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare rename: (newCode: string, newLabel?: string | null) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;
}

installEntityKitMethods(
  Port,
  defineBoundKitFields([
    { selection: "code", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_PORT, "code") },
    { selection: "label", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_PORT, "label") },
    { selection: "order", parse: (frag) => readKitPathNumberOrNull(frag as JsonObject | null, KIT_PATH_TYPE_PORT, "order") },
    { selection: "name", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_PORT, "name") },
    { selection: "description", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_PORT, "description"), eventKind: "changedDescription" },
    { selection: "icon", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_PORT, "icon") },
    {
      selection: "attributes { edges { node { id key value definition } } }",
      parse: () => [],
      coarseEvent: true,
      parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_PORT)),
    },
  ] as const) as readonly BoundKitFieldSpec<unknown, Port>[],
  defineBoundKitOperations([
    { method: "rename", buildInner: (_e, newCode, newLabel) => `rn: rename(newCode: ${gqlString(String(newCode ?? ""))}, newLabel: ${newLabel == null ? "null" : gqlString(String(newLabel))})` },
    { method: "changeDescription", buildInner: (_e, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
    { method: "changeIcon", buildInner: (_e, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
    {
      method: "addAttribute",
      buildInner: (_e, key, value, definition) =>
        `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
    },
    { method: "removeAttribute", buildInner: (_e, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
    { method: "removeAttributes", buildInner: (_e, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
  ] as const),
);
//#endregion 🔘Port

//#region 🔗Connector
/** @emoji 🔗 Connector under {@link Type}: declarative field reads, commands, and change subscriptions. */
export class Connector extends Entity {
  readonly typeId: string;
  constructor(session: Session, typeId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.typeId = typeId;
  }

  kitInnerPath(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { connector(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  declare name: () => Promise<string>;
  declare code: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare port: () => Promise<Port | null>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare rename: (newCode: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare changeIcon: (newIcon: string) => Promise<SetResult>;
}

installEntityKitMethods(
  Connector,
  defineBoundKitFields([
    { selection: "name", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_CONNECTOR, "name") },
    { selection: "code", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_CONNECTOR, "code") },
    { selection: "description", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_CONNECTOR, "description"), eventKind: "changedDescription" },
    { selection: "icon", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_CONNECTOR, "icon") },
    {
      selection: "port { id }",
      parse: () => null,
      parseEntity: (entity, frag) => {
        const id = String((readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_CONNECTOR)?.["port"] as JsonObject | undefined)?.["id"] ?? "");
        return id === "" ? null : new Type(entity.session, (entity as Connector).typeId, entity.storeId).port(id);
      },
    },
    {
      selection: "attributes { edges { node { id key value definition } } }",
      parse: () => [],
      coarseEvent: true,
      parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_CONNECTOR)),
    },
  ] as const) as readonly BoundKitFieldSpec<unknown, Connector>[],
  defineBoundKitOperations([
    { method: "rename", buildInner: (_e, newCode) => `rn: rename(newCode: ${gqlString(String(newCode ?? ""))})` },
    { method: "changeDescription", buildInner: (_e, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
    { method: "changeIcon", buildInner: (_e, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  ] as const),
);
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

  declare u: () => Promise<number>;
  declare v: () => Promise<number>;
}

function parsePieceRoleCenter(frag: JsonObject | null, role: string): JsonObject | null {
  const json = pieceKit(frag)?.[role] as JsonObject | undefined;
  return (json?.["center"] as JsonObject | undefined) ?? null;
}

installWeakKitFieldMethods(
  Coordinate,
  (self, selection) =>
    self.parent.piece.readKitInner(self.parent.piece.kitInnerPath(`${self.parent.role} { center { ${selection} } }`)) as Promise<JsonObject | null>,
  (self, frag) => parsePieceRoleCenter(frag, self.parent.role),
  [
    { method: "u", selection: "u", parse: (c) => (typeof c?.["u"] === "number" ? c["u"] : 0), coarseEvent: true },
    { method: "v", selection: "v", parse: (c) => (typeof c?.["v"] === "number" ? c["v"] : 0), coarseEvent: true },
  ],
  (self) => self.parent.piece.session.bus,
);

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

  declare x: () => Promise<number>;
  declare y: () => Promise<number>;
  declare z: () => Promise<number>;
}

function parsePieceRolePlaneOrigin(frag: JsonObject | null, role: string): JsonObject | null {
  const json = pieceKit(frag)?.[role] as JsonObject | undefined;
  const pl = json?.["plane"] as JsonObject | undefined;
  return (pl?.["origin"] as JsonObject | undefined) ?? null;
}

installWeakKitFieldMethods(
  Point,
  (self, selection) =>
    self.parent.parent.piece.readKitInner(
      self.parent.parent.piece.kitInnerPath(`${self.parent.parent.role} { plane { origin { ${selection} } } }`),
    ) as Promise<JsonObject | null>,
  (self, frag) => parsePieceRolePlaneOrigin(frag, self.parent.parent.role),
  [
    { method: "x", selection: "x", parse: (o) => (typeof o?.["x"] === "number" ? o["x"] : 0), coarseEvent: true },
    { method: "y", selection: "y", parse: (o) => (typeof o?.["y"] === "number" ? o["y"] : 0), coarseEvent: true },
    { method: "z", selection: "z", parse: (o) => (typeof o?.["z"] === "number" ? o["z"] : 0), coarseEvent: true },
  ],
  (self) => self.parent.parent.piece.session.bus,
);

/** @emoji ➡️ Weak axis vector leaf under {@link Plane}. */
export class Vector {
  constructor(
    public readonly parent: Plane,
    public readonly axisRole: "xAxis" | "yAxis",
  ) { }

  declare x: () => Promise<number>;
  declare y: () => Promise<number>;
  declare z: () => Promise<number>;
}

function parsePieceRolePlaneAxis(frag: JsonObject | null, role: string, axisRole: "xAxis" | "yAxis"): JsonObject | null {
  const json = pieceKit(frag)?.[role] as JsonObject | undefined;
  const pl = json?.["plane"] as JsonObject | undefined;
  return (pl?.[axisRole] as JsonObject | undefined) ?? null;
}

installWeakKitFieldMethods(
  Vector,
  (self, selection) =>
    self.parent.parent.piece.readKitInner(
      self.parent.parent.piece.kitInnerPath(`${self.parent.parent.role} { plane { ${self.axisRole} { ${selection} } } }`),
    ) as Promise<JsonObject | null>,
  (self, frag) => parsePieceRolePlaneAxis(frag, self.parent.parent.role, self.axisRole),
  [
    { method: "x", selection: "x", parse: (ax) => (typeof ax?.["x"] === "number" ? ax["x"] : 0), coarseEvent: true },
    { method: "y", selection: "y", parse: (ax) => (typeof ax?.["y"] === "number" ? ax["y"] : 0), coarseEvent: true },
    { method: "z", selection: "z", parse: (ax) => (typeof ax?.["z"] === "number" ? ax["z"] : 0), coarseEvent: true },
  ],
  (self) => self.parent.parent.piece.session.bus,
);

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

/** @emoji 🧩 Piece under {@link Design}: declarative reads/commands/events; weak {@link Position} handles stay synchronous. */
export class Piece extends Entity {
  readonly designId: string;
  private readonly positionByRole = new Map<"position" | "flatPosition", Position>();
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  kitInnerPath(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { piece(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  position(): Position {
    let p = this.positionByRole.get("position");
    if (!p) {
      p = new Position(this, "position");
      this.positionByRole.set("position", p);
    }
    return p;
  }

  flatPosition(): Position {
    let p = this.positionByRole.get("flatPosition");
    if (!p) {
      p = new Position(this, "flatPosition");
      this.positionByRole.set("flatPosition", p);
    }
    return p;
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare typeId: () => Promise<string | null>;
  declare scale: () => Promise<number | null>;
  declare blueprint: () => Promise<PieceBlueprint | null>;
  declare attributes: () => Promise<readonly Attribute[]>;
  declare connectionKind: () => Promise<"FIXED" | "CONNECTED" | null>;
  declare depth: () => Promise<number | null>;
  declare parentPiece: () => Promise<Piece | null>;
  declare parentConnection: () => Promise<Connection | null>;
  declare childPieces: () => Promise<readonly Piece[]>;
  declare childConnections: () => Promise<readonly Connection[]>;
  declare pathPieces: () => Promise<readonly string[]>;
  declare rename: (newName: string) => Promise<SetResult>;
  declare changeDescription: (newDescription: string) => Promise<SetResult>;
  declare drag: (offset: OffsetInput) => Promise<SetResult>;
  declare move: (position: PositionInput) => Promise<SetResult>;
  declare fix: () => Promise<SetResult>;
  declare changeBlueprint: (blueprintId: string) => Promise<SetResult>;
  declare addAttribute: (key: string, value: string, definition: string) => Promise<SetResult>;
  declare removeAttribute: (id: string) => Promise<SetResult>;
  declare removeAttributes: (ids: readonly string[]) => Promise<SetResult>;
}

const PIECE_FIELDS = defineBoundKitFields([
  { selection: "name", parse: (frag) => readPieceBranchString(frag as JsonObject | null, "name") },
  { selection: "description", parse: (frag) => readPieceBranchString(frag as JsonObject | null, "description"), eventKind: "changedDescription" },
  { selection: "icon", parse: (frag) => readPieceBranchString(frag as JsonObject | null, "icon") },
  {
    method: "typeId",
    selection: "type { id }",
    parse: () => null,
    parseEntity: (_entity, frag) => {
      const id = String((pieceKit(frag as JsonObject | null)?.["type"] as JsonObject | undefined)?.["id"] ?? "");
      return id === "" ? null : id;
    },
  },
  { selection: "scale", parse: (frag) => readPieceBranchNumberOrNull(frag as JsonObject | null, "scale") },
  {
    selection: "blueprint { __typename id }",
    parse: () => null,
    parseEntity: (_entity, frag) => parsePieceBlueprintFromJson(pieceKit(frag as JsonObject | null)?.["blueprint"] as JsonObject | undefined),
  },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, pieceKit(frag as JsonObject | null)),
  },
  {
    selection: "connectionKind",
    parse: (frag) => {
      const k = pieceKit(frag as JsonObject | null)?.["connectionKind"];
      return k === "FIXED" || k === "CONNECTED" ? k : null;
    },
    eventKind: "draggedPiece",
  },
  { selection: "depth", parse: (frag) => readPieceBranchNumberOrNull(frag as JsonObject | null, "depth") },
  {
    selection: "parentPiece { id }",
    parse: () => null,
    parseEntity: (entity, frag) => {
      const id = String((pieceKit(frag as JsonObject | null)?.["parentPiece"] as JsonObject | undefined)?.["id"] ?? "");
      return id === "" ? null : new Design(entity.session, (entity as Piece).designId, entity.storeId).piece(id);
    },
    coarseEvent: true,
  },
  {
    selection: "parentConnection { id }",
    parse: () => null,
    parseEntity: (entity, frag) => {
      const id = String((pieceKit(frag as JsonObject | null)?.["parentConnection"] as JsonObject | undefined)?.["id"] ?? "");
      return id === "" ? null : new Design(entity.session, (entity as Piece).designId, entity.storeId).connection(id);
    },
    coarseEvent: true,
  },
  {
    selection: "childPieces { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) =>
      Object.freeze(parseIdListConnection(pieceKit(frag as JsonObject | null), "childPieces").map((id) => new Design(entity.session, (entity as Piece).designId, entity.storeId).piece(id))),
  },
  {
    selection: "childConnections { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) =>
      Object.freeze(parseIdListConnection(pieceKit(frag as JsonObject | null), "childConnections").map((id) => new Design(entity.session, (entity as Piece).designId, entity.storeId).connection(id))),
  },
  {
    selection: "path { edges { node { id } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (_entity, frag) => parseIdListConnection(pieceKit(frag as JsonObject | null), "path"),
  },
] as const);

const PIECE_OPERATIONS = defineBoundKitOperations([
  { method: "rename", buildInner: (_e, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { method: "changeDescription", buildInner: (_e, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { method: "drag", buildInner: (_e, offset) => `dg: drag(offset: ${formatOffsetInput(offset as OffsetInput)})` },
  { method: "move", buildInner: (_e, position) => `mv: move(position: ${formatPositionInput(position as PositionInput)})` },
  { method: "fix", buildInner: () => `fx: fix` },
  { method: "changeBlueprint", buildInner: (_e, blueprintId) => `cb: changeBlueprint(blueprintId: ${gqlString(String(blueprintId ?? ""))})` },
  {
    method: "addAttribute",
    buildInner: (_e, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { method: "removeAttribute", buildInner: (_e, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeAttributes", buildInner: (_e, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const);

installEntityKitMethods(Piece, PIECE_FIELDS as readonly BoundKitFieldSpec<unknown, Piece>[], PIECE_OPERATIONS);
//#endregion 🧩Piece

//#region 🪢PiecesOperation
type PiecesOperationCommandSpec = Readonly<{
  method: string;
  buildInner: (self: PiecesOperation, ...args: readonly unknown[]) => string;
}>;

/** @emoji 🪢 Installs one batch-command method per spec on {@link PiecesOperation}. */
function installPiecesOperationMethods(specs: readonly PiecesOperationCommandSpec[]): void {
  for (const spec of specs) {
    Object.defineProperty(PiecesOperation.prototype, spec.method, {
      configurable: true,
      writable: true,
      value: async function piecesOpCommand(this: PiecesOperation, ...args: readonly unknown[]): Promise<SetResult> {
        if (this.storeId == null || this.storeId === "") throw new Error("PiecesOperation is not scoped to a Store");
        const cid = await this.session.ensureChangeId(this.storeId);
        return this.session.mutateScoped(this.storeId, cid, spec.buildInner(this, ...args));
      },
    });
  }
}

/** @emoji 🪢 Batch piece commands under one design: one command method each (no cached fields). */
export class PiecesOperation {
  constructor(
    readonly session: Session,
    readonly designId: string,
    readonly pieceIds: readonly string[],
    readonly storeId?: string,
  ) { }

  kitInnerPath(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { pieces(ids: ${gqlIdList(this.pieceIds)}) { ${inner} } }`;
  }

  declare drag: (offset: OffsetInput) => Promise<SetResult>;
  declare move: (offset: OffsetInput) => Promise<SetResult>;
  declare fix: () => Promise<SetResult>;
  declare changeBlueprint: (blueprintId: string) => Promise<SetResult>;
}

installPiecesOperationMethods([
  { method: "drag", buildInner: (self, offset) => self.kitInnerPath(`dg: drag(offset: ${formatOffsetInput(offset as OffsetInput)})`) },
  { method: "move", buildInner: (self, offset) => self.kitInnerPath(`mv: move(offset: ${formatOffsetInput(offset as OffsetInput)})`) },
  { method: "fix", buildInner: (self) => self.kitInnerPath(`fx: fix`) },
  { method: "changeBlueprint", buildInner: (self, blueprintId) => self.kitInnerPath(`cb: changeBlueprint(blueprintId: ${gqlString(String(blueprintId ?? ""))})`) },
]);
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

/** @emoji ⛓️ Connection under {@link Design}: declarative field reads and change subscriptions (read-only commands in schema). */
export class Connection extends Entity {
  readonly designId: string;
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  kitInnerPath(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { connection(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare gap: () => Promise<number | null>;
  declare shift: () => Promise<number | null>;
  declare rise: () => Promise<number | null>;
  declare rotation: () => Promise<number | null>;
  declare turn: () => Promise<number | null>;
  declare tilt: () => Promise<number | null>;
  declare u: () => Promise<number | null>;
  declare v: () => Promise<number | null>;
  declare parent: () => Promise<Side | null>;
  declare child: () => Promise<Side | null>;
  declare attributes: () => Promise<readonly Attribute[]>;
}

const CONNECTION_FIELDS = defineBoundKitFields([
  { selection: "name", parse: (frag) => readConnectionBranchString(frag as JsonObject | null, "name") },
  { selection: "description", parse: (frag) => readConnectionBranchString(frag as JsonObject | null, "description"), eventKind: "changedDescription" },
  { selection: "icon", parse: (frag) => readConnectionBranchString(frag as JsonObject | null, "icon") },
  { selection: "gap", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "gap") },
  { selection: "shift", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "shift") },
  { selection: "rise", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "rise") },
  { selection: "rotation", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "rotation") },
  { selection: "turn", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "turn") },
  { selection: "tilt", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "tilt") },
  { method: "u", selection: "u", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "u") },
  { method: "v", selection: "v", parse: (frag) => readConnectionBranchNumberOrNull(frag as JsonObject | null, "v") },
  {
    selection: `parent { ${CONNECTION_SIDE_SELECTION} }`,
    parse: () => null,
    parseEntity: (entity, frag) =>
      parseSideFromJson(entity.session, (entity as Connection).designId, entity.id, "parent", connectionKit(frag as JsonObject | null)?.["parent"] as JsonObject | undefined, entity.storeId),
    coarseEvent: true,
  },
  {
    selection: `child { ${CONNECTION_SIDE_SELECTION} }`,
    parse: () => null,
    parseEntity: (entity, frag) =>
      parseSideFromJson(entity.session, (entity as Connection).designId, entity.id, "child", connectionKit(frag as JsonObject | null)?.["child"] as JsonObject | undefined, entity.storeId),
    coarseEvent: true,
  },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, connectionKit(frag as JsonObject | null)),
  },
] as const);

installEntityKitMethods(Connection, CONNECTION_FIELDS as readonly BoundKitFieldSpec<unknown, Connection>[]);
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
installEntityNodeMethods(Author, "Author", AUTHOR_FIELDS);
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
  declare benchmarks: () => Promise<readonly Benchmark[]>;
}

const QUALITY_OPERATIONS = defineBoundKitOperations([
  { method: "rename", buildInner: (_entity, newKey) => `rk: rename(newKey: ${gqlString(String(newKey ?? ""))})` },
  { method: "changeDescription", buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { method: "changeIcon", buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    method: "addAttribute",
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { method: "removeAttribute", buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeAttributes", buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const);
installEntityKitMethods(Quality, defineBoundKitFields([
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
    coarseEvent: true,
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["quality"] as JsonObject | undefined),
  },
  {
    selection: "benchmarks { edges { node { id name min max minExcluded maxExcluded } } }",
    parse: () => [],
    coarseEvent: true,
    parseEntity: (entity, frag) => parseBenchmarkConnectionUnder(entity as Quality, (frag as JsonObject | null)?.["quality"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Quality>[], QUALITY_OPERATIONS);
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

installEntityKitMethods(Tag, defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "description") },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "icon") },
  { selection: "order", parse: (frag) => readKitBranchNumberOrNull(frag as JsonObject | null, "tag", "order") },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["tag"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Tag>[], defineBoundKitOperations([
  { method: "rename", buildInner: (_entity, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { method: "changeDescription", buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { method: "changeIcon", buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    method: "addAttribute",
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { method: "removeAttribute", buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeAttributes", buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
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

installEntityKitMethods(Concept, defineBoundKitFields([
  { selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "name") },
  { selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "description") },
  { selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "icon") },
  { selection: "order", parse: (frag) => readKitBranchNumberOrNull(frag as JsonObject | null, "concept", "order") },
  {
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["concept"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Concept>[], defineBoundKitOperations([
  { method: "rename", buildInner: (_entity, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { method: "changeDescription", buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { method: "changeIcon", buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    method: "addAttribute",
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { method: "removeAttribute", buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { method: "removeAttributes", buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
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

installEntityKitMethods(Representation, defineBoundKitFields([
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

installEntityNodeMethods(Family, "Family", defineBoundNodeFields([
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

installEntityNodeMethods(File, "File", defineBoundNodeFields([
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

installEntityNodeMethods(Folder, "Folder", defineBoundNodeFields([
  { selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { selection: "path", parse: (node) => String(node?.["path"] ?? "") },
] as const));
//#endregion 📁Folder

//#region 🪟Layer
/** @emoji 🪟 Design {@link Layer}: declarative field reads and change subscriptions (read-only in current kit API). */
export class Layer extends Entity {
  readonly designId: string;
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  kitInnerPath(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { layers { edges { node { id ${inner} } } } }`;
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare color: () => Promise<string>;
  declare order: () => Promise<number | null>;
  declare visible: () => Promise<boolean | null>;
  declare locked: () => Promise<boolean | null>;
}

installEntityKitMethods(
  Layer,
  defineBoundKitFields([
    { selection: "name", parse: (frag) => "", parseEntity: (entity, frag) => readDesignListNodeField(frag as JsonObject | null, "layers", entity.id, "name") },
    { selection: "description", parse: (frag) => "", parseEntity: (entity, frag) => readDesignListNodeField(frag as JsonObject | null, "layers", entity.id, "description"), eventKind: "changedDescription" },
    { selection: "icon", parse: (frag) => "", parseEntity: (entity, frag) => readDesignListNodeField(frag as JsonObject | null, "layers", entity.id, "icon") },
    { selection: "color", parse: (frag) => "", parseEntity: (entity, frag) => readDesignListNodeField(frag as JsonObject | null, "layers", entity.id, "color") },
    { selection: "order", parse: (frag) => null, parseEntity: (entity, frag) => readDesignListNodeNumberOrNull(frag as JsonObject | null, entity.id, "order") },
    { selection: "visible", parse: (frag) => null, parseEntity: (entity, frag) => readDesignListNodeBooleanOrNull(frag as JsonObject | null, entity.id, "visible") },
    { selection: "locked", parse: (frag) => null, parseEntity: (entity, frag) => readDesignListNodeBooleanOrNull(frag as JsonObject | null, entity.id, "locked") },
  ] as const) as readonly BoundKitFieldSpec<unknown, Layer>[],
);
//#endregion 🪟Layer

//#region 👥Group
/** @emoji 👥 Design {@link Group}: declarative field reads and change subscriptions (read-only in current kit API). */
export class Group extends Entity {
  readonly designId: string;
  constructor(session: Session, designId: string, id: string, storeId?: string) {
    super(session, id, storeId);
    this.designId = designId;
  }

  kitInnerPath(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { groups { edges { node { id ${inner} } } } }`;
  }

  declare name: () => Promise<string>;
}

installEntityKitMethods(
  Group,
  defineBoundKitFields([
    { selection: "name", parse: (frag) => "", parseEntity: (entity, frag) => readDesignListNodeField(frag as JsonObject | null, "groups", entity.id, "name") },
  ] as const) as readonly BoundKitFieldSpec<unknown, Group>[],
);
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

installEntityNodeMethods(Stat, "Stat", defineBoundNodeFields([
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

installEntityNodeMethods(Prop, "Prop", defineBoundNodeFields([
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
    it("parseResponsePayload reads ok errors and IdResult value", () => {
      expect(parseResponsePayload({ ok: true, result: { value: "abc" } })).toEqual({ ok: true });
      expect(responseResultId({ ok: true, result: { value: "abc" } })).toBe("abc");
      const failed = parseResponsePayload({ ok: false, errors: { kind: "Invalid", message: "nope" } });
      expect(failed.ok).toBe(false);
      if (!failed.ok) expect(failed.error.message).toBe("nope");
    });
    it("withResponseSelection wraps leaf kit commands", () => {
      expect(withResponseSelection("rename(newName: \"x\")")).toContain("ok");
      expect(withResponseSelection('design(id: "d") { addFixedPiece(blueprintId: "b", position: { center: { u: 0, v: 0 } }) }')).toContain(
        "addFixedPiece",
      );
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

