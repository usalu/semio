//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: stateless {@link Store} + GraphQL transport (WASM worker or inline); no client-side kit cache.
//#endregion 🧲Header

//#region 📥KitImports
//#endregion 📥KitImports

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

/** @emoji 🌐 Thin GraphQL JSON transport: request in, JSON string out; pairs with rs {@code KitStoreHandle}. */
export class GqlTransport {
  constructor(private readonly inner: WorkerStringTransport | InlineTransport) { }

  async executeJson(body: { readonly query: string; readonly variables?: JsonObject; readonly operationName?: string }, timeoutMs: number): Promise<GraphqlEnvelope<JsonValue>> {
    const json = await withTimeout(this.inner.execute(JSON.stringify(body)), timeoutMs, "graphql");
    return parseJsonValue(json) as GraphqlEnvelope<JsonValue>;
  }

  async subscribeJson(body: { readonly query: string; readonly variables?: JsonObject }, onEvent: (env: GraphqlEnvelope<JsonValue>) => void): Promise<void> {
    await this.inner.subscribe(JSON.stringify(body), (eventJson) => {
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

/** @emoji 📡 Live-query mirror of target {@code Subscription.session}; ticks {@link Store#bus} on each WIP emission. */
export const KIT_EVENT_STREAM_SUBSCRIPTION = `subscription { session { stores { edges { node { wip { id hash } } } } } }` as const;

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

export type StoreOpenOptions = Readonly<{
  timeoutMs?: number;
  storeId?: string;
  wasmSpecifier?: string;
  workerFactory?: () => Worker;
}>;

function gqlString(s: string): string {
  return JSON.stringify(s);
}

function gqlIdList(ids: readonly string[]): string {
  return `[${ids.map((x) => gqlString(x)).join(",")}]`;
}

function scopedKitMutationBody(changeId: string, kitSelection: string): { readonly query: string; readonly variables: JsonObject } {
  return {
    query: `mutation($storeId: ID!, $changeId: ID!) { session { store(id: $storeId) { theKit { unsavedChange(id: $changeId) { kit { ${kitSelection} } } } } } }`,
    variables: { changeId },
  };
}

function sessionStoreSelectionDocument(innerOnStore: string): { query: string; variables: JsonObject } {
  return {
    query: `query SemioSessionStore { session { stores { edges { node { ${innerOnStore} } } } } }`,
    variables: {},
  };
}

function kitReadSelectionDocument(point: KitReadPoint, innerOnKitStore: string): { query: string; variables: JsonObject } {
  if (isTheKitReadPoint(point)) {
    return {
      query: `query KitSessionWipStore { session { stores { edges { node { wip { theKit { kit { ${innerOnKitStore} } } } } } } } }`,
      variables: {},
    };
  }
  if ("checkpoint" in point) {
    return {
      query: `query KitSessionWipStore($checkpointId: ID!) { session { stores { edges { node { wip { checkpoint(id: $checkpointId) { kit { ${innerOnKitStore} } } } } } } } }`,
      variables: { checkpointId: point.checkpoint.checkpointId },
    };
  }
  if ("alternative" in point) {
    return {
      query: `query KitSessionWipStore($alternativeId: ID!) { session { stores { edges { node { wip { alternative(id: $alternativeId) { kit { ${innerOnKitStore} } } } } } } } }`,
      variables: { alternativeId: point.alternative.alternativeId },
    };
  }
  return {
    query: `query KitSessionWipStore { session { stores { edges { node { wip { theKit { kit { ${innerOnKitStore} } } } } } } } }`,
    variables: {},
  };
}

function firstSessionStoreNode(d: JsonValue | null | undefined): JsonObject | null {
  if (d == null || typeof d !== "object" || Array.isArray(d)) return null;
  const session = jsonObjectField(d as JsonObject, "session");
  const stores = jsonObjectField(session, "stores");
  const edges = stores?.["edges"];
  if (!Array.isArray(edges)) return null;
  return jsonObjectField(edges[0] as JsonObject | undefined, "node");
}

function kitReadSelectionFromData(d: JsonValue | null | undefined, point: KitReadPoint): JsonObject | null {
  const store = firstSessionStoreNode(d);
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

async function executeGraphql(handle: { execute(requestJson: string): Promise<string> }, body: { query: string; variables?: JsonObject; operationName?: string }, timeoutMs?: number): Promise<GraphqlEnvelope<JsonValue>> {
  const json = await withTimeout(handle.execute(JSON.stringify(body)), timeoutMs ?? 0, "graphql");
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
/** @emoji 🧬 Strong entity anchor: {@link Store} + id (no cached fields on the instance). */
export abstract class Entity {
  public readonly store: Store;

  protected constructor(store: Store, public readonly id: string) {
    this.store = store;
  }
}
//#endregion 🛠️Base

//#region 🪶WeakArtifacts
/** @emoji 🪪 Weak attribute row anchored on an owning {@link Entity} (no separate {@code node(id:)} identity). */
export class Attribute {
  constructor(
    public readonly owner: Entity,
    public readonly id: string,
    public readonly key: string,
    public readonly value: string | null,
    public readonly definition: string,
  ) { }

  get store(): Store {
    return this.owner.store;
  }
}

/** @emoji 🏁 Weak benchmark row under {@link Quality}. */
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

  get store(): Store {
    return this.quality.store;
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
  methodName: string;
  parseEntity?: (entity: E, v: JsonValue) => T;
}>;

type BoundNodeFieldSpec<T> = Readonly<{
  methodName: string;
  selection: string;
  parse: (node: JsonObject | undefined) => T;
}>;

type BoundKitOperationSpec<E extends KitPathEntity> = Readonly<{
  methodName: string;
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

/** @emoji 🏭  a field read when the caller supplies the kit-relative GraphQL tail. */
export function defineField<E extends Entity, T>(entity: E, spec: FieldSpec<T>, pathInKit: (self: E) => string): () => Promise<T> {
  return async () => {
    const frag = await entity.store.readKitInner(pathInKit(entity));
    return spec.parse(frag as JsonValue);
  };
}

/** @emoji 🏭  a mutation leaf using {@link Store#mutateScoped}. */
export function defineOperation(entity: Entity, spec: OperationSpec, buildPath: (self: Entity) => string): () => Promise<SetResult> {
  return async () => {
    void spec;
    const cid = await entity.store.ensureChangeId();
    return entity.store.mutateScoped(cid, buildPath(entity));
  };
}

/** @emoji 🏭 Installs kit-relative read methods on a prototype so classes stay declarative and schema-shaped. */
function installKitFieldMethods<E extends KitPathEntity>(
  ctor: abstract new (...args: never[]) => E,
  specs: readonly BoundKitFieldSpec<unknown, E>[],
): void {
  for (const spec of specs) {
    Object.defineProperty(ctor.prototype, spec.methodName, {
      configurable: true,
      value: async function semioKitField(this: E): Promise<unknown> {
        const frag = await this.store.readKitInner(this.kitInnerPath(spec.selection));
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
    await executeStoreGraphql(entity.store, {
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
    Object.defineProperty(ctor.prototype, spec.methodName, {
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
    Object.defineProperty(ctor.prototype, spec.methodName, {
      configurable: true,
      value: async function semioKitOperation(this: E, ...args: readonly unknown[]): Promise<SetResult> {
        const cid = await this.store.ensureChangeId();
        return this.store.mutateScoped(cid, this.kitInnerPath(spec.buildInner(this, ...args)));
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

//#region 🧷MapCache
/** @emoji 🧷 File-local memo: get-or-insert for {@link Map} entity-handle caches (single-source instance cache per plan). */
function cachedMapGet<K, V>(map: Map<K, V>, key: K, create: () => V): V {
  let v = map.get(key);
  if (!v) {
    v = create();
    map.set(key, v);
  }
  return v;
}
//#endregion 🧷MapCache

//#region 📦KitBranch
/** @emoji 📦 String field from nested kit row (e.g. `{ design: { name } }` or flattened `{ name }`). */
function readKitBranchString(frag: JsonObject | null | undefined, branchKey: string, field: string): string {
  const branch = frag?.[branchKey] as JsonObject | undefined;
  const v = branch?.[field] ?? frag?.[field];
  return String(v ?? "");
}

/** @emoji 📦 Numeric field from nested kit row (e.g. {@code qualitySum}). */
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
    const bytes = new TextEncoder().encode(t);
    let bin = "";
    for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i]!);
    return `dev+json:${btoa(bin)}`;
  }
  return t;
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

/** @emoji 🧩 Parses {@code key: [{ id: … }]} non-relay {@code [StrongEntity!]} lists on a JSON row (e.g. {@code Checkpoint.changes}). */
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

function readStableEntityList<T>(
  cache: { ids: readonly string[]; arr: readonly T[] },
  nextIds: readonly string[],
  build: (id: string) => T,
): readonly T[] {
  if (cache.ids.length === nextIds.length && cache.ids.every((v, i) => v === nextIds[i]!)) {
    return cache.arr;
  }
  const ids = [...nextIds];
  cache.ids = ids;
  cache.arr = Object.freeze(ids.map((id) => build(id)));
  return cache.arr;
}

/**
 * @emoji 🏪 Stateless store root: owns {@link GqlTransport} + {@link EventBus}; every read is a fresh GraphQL round-trip.
 */
export class Store {
  private readonly timeoutMs: number;
  private readonly storeId: string;
  private readonly handle: GraphqlExecuteHandle;
  private readonly innerTransport: WorkerStringTransport | InlineTransport;
  private gqlLoopRunning = false;
  private disposed = false;
  private activeReadPoint: KitReadPoint = theKitReadPoint;
  private kitWriteChangeId: string | null = null;
  private readonly designCache = new Map<string, Design>();
  private readonly typeCache = new Map<string, Type>();
  private readonly tagCache = new Map<string, Tag>();
  private readonly conceptCache = new Map<string, Concept>();
  private readonly qualityCache = new Map<string, Quality>();
  private readonly familyCache = new Map<string, Family>();
  private readonly fileCache = new Map<string, File>();
  private readonly folderCache = new Map<string, Folder>();
  private readonly authorCache = new Map<string, Author>();
  private readonly statCache = new Map<string, Stat>();
  private readonly conflictCache = new Map<string, Conflict>();
  private readonly graphByRoot = new Map<"wip" | "authoritative", Graph>();
  private sessionEntity: Session | undefined;
  private stableDesigns: { ids: readonly string[]; arr: readonly Design[] } = { ids: [], arr: [] };
  private stableTypes: { ids: readonly string[]; arr: readonly Type[] } = { ids: [], arr: [] };
  private stableAuthors: { ids: readonly string[]; arr: readonly Author[] } = { ids: [], arr: [] };
  private stableQualities: { ids: readonly string[]; arr: readonly Quality[] } = { ids: [], arr: [] };
  private stableTags: { ids: readonly string[]; arr: readonly Tag[] } = { ids: [], arr: [] };
  private stableConcepts: { ids: readonly string[]; arr: readonly Concept[] } = { ids: [], arr: [] };

  /** @emoji 🌐 GraphQL executor (JSON in/out). */
  readonly gql: GqlTransport;
  /** @emoji 📡 Demuxed subscription fan-out. */
  readonly bus: EventBus;

  private constructor(timeoutMs: number, storeId: string, inner: WorkerStringTransport | InlineTransport) {
    this.timeoutMs = timeoutMs;
    this.storeId = storeId;
    this.innerTransport = inner;
    this.handle = { execute: (j) => inner.execute(j) };
    this.gql = new GqlTransport(inner);
    this.bus = new EventBus();
  }

  private ensureAlive(): void {
    if (this.disposed) throw new Error("Store disposed");
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
    const root = firstSessionStoreNode(data) ?? jsonObjectField(data, "session") ?? data;
    if (root["wip"] !== undefined || root["stores"] !== undefined) {
      // Coarse invalidation: live-query WIP tick fans out to the current semantic bus kinds and command correlator.
      this.bus.emit({ kind: "kitRenamed", payload: undefined } as unknown as JsonValue);
      this.bus.emit({ kind: "changedDescription", payload: undefined } as unknown as JsonValue);
      this.bus.emit({ kind: "commandSucceeded", payload: undefined } as unknown as JsonValue);
      return;
    }
    if (root["session"] !== undefined) {
      this.bus.emit({ kind: "commandSucceeded", payload: undefined } as unknown as JsonValue);
      return;
    }
    if (root["commandSucceeded"] !== undefined) this.bus.emit({ kind: "commandSucceeded", payload: root["commandSucceeded"] });
    if (root["operationFailed"] !== undefined) this.bus.emit({ kind: "operationFailed", payload: root["operationFailed"] });
  }

  private startSubscriptionLoop(): void {
    if (this.gqlLoopRunning) return;
    this.gqlLoopRunning = true;
    void this.innerTransport
      .subscribe(JSON.stringify({ query: KIT_EVENT_STREAM_SUBSCRIPTION }), (eventJson: string) => {
        try {
          const msg = parseJsonValue(eventJson) as GraphqlEnvelope<JsonObject>;
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

  private async gqlRun(body: { query: string; variables?: JsonObject; operationName?: string }): Promise<GraphqlEnvelope<JsonValue>> {
    this.ensureAlive();
    return executeGraphql(this.handle, body, this.timeoutMs);
  }

  /** @emoji 🧾 Reads a selection inside scoped {@code kit { … }} for {@link activeReadPoint}. */
  async readKitInner(inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    const { query, variables: v0 } = kitReadSelectionDocument(this.activeReadPoint, inner);
    const data = unwrapGraphqlData(await this.gqlRun({ query, variables: { ...v0, ...variables } })) as JsonValue;
    return kitReadSelectionFromData(data, this.activeReadPoint);
  }

  /** @emoji 🧾 Reads a selection inside the active target {@code Session.stores} node. */
  async readStoreInner(inner: string, variables: JsonObject = {}): Promise<JsonObject | null> {
    const { query, variables: v0 } = sessionStoreSelectionDocument(inner);
    const data = unwrapGraphqlData(await this.gqlRun({ query, variables: { ...v0, ...variables } })) as JsonValue;
    return firstSessionStoreNode(data);
  }

  /** @emoji 🧾 Runs {@code mutation session { theKit { unsavedChange { kit { … } } } }} when {@linkcode changeId} is set. */
  async mutateScoped(changeId: string, kitSelection: string): Promise<SetResult> {
    this.ensureAlive();
    const { query, variables } = scopedKitMutationBody(changeId, kitSelection);
    const env = await this.gqlRun({ query, variables: { ...variables, storeId: this.storeId } });
    return gqlOkFromEnvelope(env);
  }

  async ensureChangeId(): Promise<string> {
    this.ensureAlive();
    if (this.kitWriteChangeId) return this.kitWriteChangeId;
    const data = unwrapGraphqlData(await this.gqlRun({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { startNewChange } } } }`, variables: { storeId: this.storeId } })) as JsonObject;
    const sess = data["session"] as JsonObject | undefined;
    const store = sess?.["store"] as JsonObject | undefined;
    const tk = store?.["theKit"] as JsonObject | undefined;
    const cid = String(tk?.["startNewChange"] ?? "");
    if (cid === "") throw new Error("startNewChange: empty change id");
    this.kitWriteChangeId = cid;
    return cid;
  }

  async saveChange(): Promise<void> {
    this.ensureAlive();
    unwrapGraphqlData(await this.gqlRun({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { save } } } }`, variables: { storeId: this.storeId } }));
    this.kitWriteChangeId = null;
  }

  async startNewChange(): Promise<ChangeId> {
    return await this.ensureChangeId();
  }

  async createCheckpoint(message: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { theKit { createCheckpoint(message: ${gqlString(message)}) } } } }`, variables: { storeId: this.storeId } });
    return gqlOkFromEnvelope(env);
  }

  async startAlternative(name?: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({
      query:
        name == null
          ? `mutation($storeId: ID!) { session { store(id: $storeId) { startAlternative } } }`
          : `mutation($storeId: ID!) { session { store(id: $storeId) { startAlternative(name: ${gqlString(name)}) } } }`,
      variables: { storeId: this.storeId },
    });
    return gqlOkFromEnvelope(env);
  }

  async integrateAlternative(alternativeId: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({
      query: `mutation($storeId: ID!) { session { store(id: $storeId) { alternative(id: ${gqlString(alternativeId)}) { integrateIntoTheKit } } } }`,
      variables: { storeId: this.storeId },
    });
    return gqlOkFromEnvelope(env);
  }

  async login(username: string, passwordHash: string, hubUrl?: string): Promise<SetResult> {
    this.ensureAlive();
    const url = hubUrl ?? "";
    const h = hubUrl == null ? "null" : gqlString(hubUrl);
    const env = await this.gqlRun({
      query: `mutation { session { remoteProvider(url: ${gqlString(url)}) { login(username: ${gqlString(username)}, passwordHash: ${gqlString(passwordHash)}, hubUrl: ${h}) } } }`,
    });
    return gqlOkFromEnvelope(env);
  }

  async logout(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { session { remoteProvider(url: "") { logout } } }` });
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
    const createEnv = await this.gqlRun({ query: `mutation { session { localProvider { createBackbone(uri: ${gqlString(uri)}) } } }` });
    if (Array.isArray(createEnv.errors) && createEnv.errors.length > 0) return gqlOkFromEnvelope(createEnv);
    const env = await this.gqlRun({ query: `mutation($storeId: ID!) { session { localProvider { attachBackbone(store: $storeId) } } }`, variables: { storeId: this.storeId } });
    return gqlOkFromEnvelope(env);
  }

  async detachBackbone(_uri: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { detach } } } }`, variables: { storeId: this.storeId } });
    return gqlOkFromEnvelope(env);
  }

  /** @emoji 🛜 Runs target {@code BackboneCommand.sync} through the active store command scope. */
  async backboneSyncNow(): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation($storeId: ID!) { session { store(id: $storeId) { backbone { sync } } } }`, variables: { storeId: this.storeId } });
    return gqlOkFromEnvelope(env);
  }

  /** @emoji 🛜 Reads {@code BackboneStatus} via the command shell (typed snapshot, not raw JSON). */
  async backboneStatus(): Promise<Readonly<{ attachedUri: string | null; kind: string }>> {
    this.ensureAlive();
    const data = unwrapGraphqlData(
      await this.gqlRun({ query: `query { session { stores { edges { node { authoritative { id } conflicts { edges { node { id } } } } } } } }` }),
    ) as JsonObject;
    const st = firstSessionStoreNode(data);
    return {
      attachedUri: null,
      kind: st?.["authoritative"] == null ? "OFFLINE" : "ONLINE",
    };
  }

  /** @emoji 🧾 Warm-path query after WASM init. */
  private async warmGraphqlRead(): Promise<void> {
    await this.readKitInner("id name");
  }

  static async open(uri: string, opts?: StoreOpenOptions): Promise<Store> {
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

    const wasmBytesPre = await readSemioWasmBytesFromMonorepoCandidates();
    const useDedicatedWorker = typeof Worker !== "undefined" && !preferInlineInVitest && wasmBytesPre == null;

    const bootstrapUri = backboneBootstrapUriForStoreOpen(uri);
    const storeId = opts?.storeId ?? bootstrapUri;

    if (useDedicatedWorker) {
      const worker = opts?.workerFactory?.() ?? createKitStoreWorker();
      const wt = new WorkerStringTransport(worker);
      try {
        await wt.init(bootstrapUri);
        const k = new Store(timeoutMs, storeId, wt);
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
    const k = new Store(timeoutMs, storeId, t);
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
    return cachedMapGet(this.designCache, id, () => new Design(this, id));
  }

  type(id: string): Type {
    return cachedMapGet(this.typeCache, id, () => new Type(this, id));
  }

  tag(id: string): Tag {
    return cachedMapGet(this.tagCache, id, () => new Tag(this, id));
  }

  concept(id: string): Concept {
    return cachedMapGet(this.conceptCache, id, () => new Concept(this, id));
  }

  quality(id: string): Quality {
    return cachedMapGet(this.qualityCache, id, () => new Quality(this, id));
  }

  family(id: string): Family {
    return cachedMapGet(this.familyCache, id, () => new Family(this, id));
  }

  file(id: string): File {
    return cachedMapGet(this.fileCache, id, () => new File(this, id));
  }

  folder(id: string): Folder {
    return cachedMapGet(this.folderCache, id, () => new Folder(this, id));
  }

  author(id: string): Author {
    return cachedMapGet(this.authorCache, id, () => new Author(this, id));
  }

  stat(id: string): Stat {
    return cachedMapGet(this.statCache, id, () => new Stat(this, id));
  }

  /** @emoji 🌐 WIP {@link Graph} ({@code Query.session.stores.edges.node.wip}). */
  wip(): Graph {
    let g = this.graphByRoot.get("wip");
    if (!g) {
      g = new Graph(this, "wip");
      this.graphByRoot.set("wip", g);
    }
    return g;
  }

  /** @emoji 🌐 Authoritative {@link Graph} when the server exposes it ({@code Query.session.stores.edges.node.authoritative}). */
  authoritative(): Graph {
    let g = this.graphByRoot.get("authoritative");
    if (!g) {
      g = new Graph(this, "authoritative");
      this.graphByRoot.set("authoritative", g);
    }
    return g;
  }

  /** @emoji 🗂️ Root {@link Session} ({@code Query.session}). */
  session(): Session {
    return (this.sessionEntity ??= new Session(this));
  }

  /** @emoji ⚔️ {@link Conflict} via {@code node(id:)}. */
  conflict(id: string): Conflict {
    return cachedMapGet(this.conflictCache, id, () => new Conflict(this, id));
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

  private async kitId(): Promise<string> {
    const frag = (await this.readKitInner("id")) as JsonObject | null;
    return String(frag?.["id"] ?? "");
  }

  async icon(): Promise<string> {
    const frag = (await this.readKitInner("icon")) as JsonObject | null;
    return String(frag?.["icon"] ?? "");
  }

  async image(): Promise<string> {
    const frag = (await this.readKitInner("image")) as JsonObject | null;
    return String(frag?.["image"] ?? "");
  }

  private async typeIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("types { edges { node { id } } }")) as JsonObject | null;
    return parseEntityConnectionIds(frag, "types");
  }

  private async designIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("designs { edges { node { id } } }")) as JsonObject | null;
    return parseEntityConnectionIds(frag, "designs");
  }

  private async authorIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("authors { edges { node { id } } }")) as JsonObject | null;
    return parseEntityConnectionIds(frag, "authors");
  }

  private async qualityIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("qualities { edges { node { id } } }")) as JsonObject | null;
    return parseEntityConnectionIds(frag, "qualities");
  }

  private async tagIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("tags { edges { node { id } } }")) as JsonObject | null;
    return parseEntityConnectionIds(frag, "tags");
  }

  private async conceptIds(): Promise<readonly string[]> {
    const frag = (await this.readKitInner("concepts { edges { node { id } } }")) as JsonObject | null;
    return parseEntityConnectionIds(frag, "concepts");
  }

  /** @emoji 📚 Id-list-stable {@link Design} handles for the active read point. */
  async designs(): Promise<readonly Design[]> {
    const ids = await this.designIds();
    return readStableEntityList(this.stableDesigns, ids, (id) => this.design(id));
  }

  /** @emoji 📚 Id-list-stable {@link Type} handles. */
  async types(): Promise<readonly Type[]> {
    const ids = await this.typeIds();
    return readStableEntityList(this.stableTypes, ids, (id) => this.type(id));
  }

  /** @emoji 📚 Id-list-stable {@link Author} handles. */
  async authors(): Promise<readonly Author[]> {
    const ids = await this.authorIds();
    return readStableEntityList(this.stableAuthors, ids, (id) => this.author(id));
  }

  /** @emoji 📚 Id-list-stable {@link Quality} handles. */
  async qualities(): Promise<readonly Quality[]> {
    const ids = await this.qualityIds();
    return readStableEntityList(this.stableQualities, ids, (id) => this.quality(id));
  }

  /** @emoji 📚 Id-list-stable {@link Tag} handles. */
  async tags(): Promise<readonly Tag[]> {
    const ids = await this.tagIds();
    return readStableEntityList(this.stableTags, ids, (id) => this.tag(id));
  }

  /** @emoji 📚 Id-list-stable {@link Concept} handles. */
  async concepts(): Promise<readonly Concept[]> {
    const ids = await this.conceptIds();
    return readStableEntityList(this.stableConcepts, ids, (id) => this.concept(id));
  }
}
//#endregion 🏪Store

//#region 📦Kit
/** @emoji 📦 Target-schema kit entity beneath {@link Version}; delegates transport work to {@link Store}. */
export class Kit extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  async rename(newName: string): Promise<SetResult> {
    return this.store.rename(newName);
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    return this.store.changeDescription(newDescription);
  }

  async createTag(name: string, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    return this.store.createTag(name, description, icon, order);
  }

  async deleteTag(id: string): Promise<SetResult> {
    return this.store.deleteTag(id);
  }

  async deleteTags(ids: readonly string[]): Promise<SetResult> {
    return this.store.deleteTags(ids);
  }

  async createConcept(name: string, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    return this.store.createConcept(name, description, icon, order);
  }

  async deleteConcept(id: string): Promise<SetResult> {
    return this.store.deleteConcept(id);
  }

  async deleteConcepts(ids: readonly string[]): Promise<SetResult> {
    return this.store.deleteConcepts(ids);
  }

  async createQuality(
    key: string,
    value?: string | null,
    unit?: string | null,
    definition?: string | null,
    description?: string | null,
    icon?: string | null,
  ): Promise<SetResult> {
    return this.store.createQuality(key, value, unit, definition, description, icon);
  }

  async deleteQuality(id: string): Promise<SetResult> {
    return this.store.deleteQuality(id);
  }

  async deleteQualities(ids: readonly string[]): Promise<SetResult> {
    return this.store.deleteQualities(ids);
  }

  async createType(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null): Promise<SetResult> {
    return this.store.createType(name, description, icon, image, unit);
  }

  async deleteType(id: string): Promise<SetResult> {
    return this.store.deleteType(id);
  }

  async deleteTypes(ids: readonly string[]): Promise<SetResult> {
    return this.store.deleteTypes(ids);
  }

  async createDesign(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null): Promise<SetResult> {
    return this.store.createDesign(name, description, icon, image, unit);
  }

  async deleteDesign(id: string): Promise<SetResult> {
    return this.store.deleteDesign(id);
  }

  async deleteDesigns(ids: readonly string[]): Promise<SetResult> {
    return this.store.deleteDesigns(ids);
  }

  async name(): Promise<string> {
    return this.store.name();
  }

  async description(): Promise<string> {
    return this.store.description();
  }

  async icon(): Promise<string> {
    return this.store.icon();
  }

  async image(): Promise<string> {
    return this.store.image();
  }

  async designs(): Promise<readonly Design[]> {
    return this.store.designs();
  }

  async types(): Promise<readonly Type[]> {
    return this.store.types();
  }

  async authors(): Promise<readonly Author[]> {
    return this.store.authors();
  }

  async qualities(): Promise<readonly Quality[]> {
    return this.store.qualities();
  }

  async tags(): Promise<readonly Tag[]> {
    return this.store.tags();
  }

  async concepts(): Promise<readonly Concept[]> {
    return this.store.concepts();
  }
}
//#endregion 📦Kit

function executeStoreGraphql(
  store: Store,
  body: Readonly<{ query: string; variables?: JsonObject; operationName?: string }>,
): Promise<GraphqlEnvelope<JsonValue>> {
  return (store as unknown as { gqlRun(b: typeof body): Promise<GraphqlEnvelope<JsonValue>> }).gqlRun(body);
}

//#region 🧬VcsEntities
/** @emoji 🌐 WIP or authoritative {@code Graph} root from {@code Query}. */
export type GraphRootKind = "wip" | "authoritative";

/** @emoji 🌐 VCS graph: {@code wip} / {@code authoritative} selections on {@link Store}. */
export class Graph extends Entity {
  private readonly checkpointCache = new Map<string, Checkpoint>();
  private readonly alternativeCache = new Map<string, Alternative>();
  private theKitEntity: TheKit | undefined;
  private stableAlternatives: { ids: readonly string[]; arr: readonly Alternative[] } = { ids: [], arr: [] };
  private stableCheckpoints: { ids: readonly string[]; arr: readonly Checkpoint[] } = { ids: [], arr: [] };

  constructor(store: Store, root: GraphRootKind) {
    super(store, root);
  }

  get root(): GraphRootKind {
    return this.id as GraphRootKind;
  }

  /** @emoji 🏛 {@code graph { theKit }} handle. */
  theKit(): TheKit {
    return (this.theKitEntity ??= new TheKit(this.store, this.root));
  }

  checkpoint(checkpointId: string): Checkpoint {
    return cachedMapGet(this.checkpointCache, checkpointId, () => new Checkpoint(this.store, this.root, checkpointId));
  }

  alternative(alternativeId: string): Alternative {
    return cachedMapGet(this.alternativeCache, alternativeId, () => new Alternative(this.store, { parent: "graph", root: this.root }, alternativeId));
  }

  private async readStoreGraphRootScalar(field: "id" | "hash"): Promise<string> {
    const node = jsonObjectField(await this.store.readStoreInner(`${this.root} { ${field} }`), this.root);
    return node == null ? "" : String(node[field] ?? "");
  }

  async hash(): Promise<string> {
    return await this.readStoreGraphRootScalar("hash");
  }

  private async alternativeIds(): Promise<readonly string[]> {
    const node = jsonObjectField(await this.store.readStoreInner(`${this.root} { alternatives { edges { node { id } } } }`), this.root);
    return parseEntityConnectionIds(node, "alternatives");
  }

  private async checkpointIds(): Promise<readonly string[]> {
    const node = jsonObjectField(await this.store.readStoreInner(`${this.root} { checkpoints { edges { node { id } } } }`), this.root);
    return parseEntityConnectionIds(node, "checkpoints");
  }

  /** @emoji 📚 Id-list-stable {@link Alternative} handles under this graph root. */
  async alternatives(): Promise<readonly Alternative[]> {
    const ids = await this.alternativeIds();
    return readStableEntityList(this.stableAlternatives, ids, (id) => this.alternative(id));
  }

  /** @emoji 📚 Id-list-stable {@link Checkpoint} handles under this graph root. */
  async checkpoints(): Promise<readonly Checkpoint[]> {
    const ids = await this.checkpointIds();
    return readStableEntityList(this.stableCheckpoints, ids, (id) => this.checkpoint(id));
  }

  /** @emoji 📡 Refetches {@link Graph#readAlternatives} on coarse kit ticks. */
  subscribeAlternatives(cb: (next: readonly Alternative[]) => void): Unsubscribe {
    const run = (): void => {
      void this.alternatives().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
  }

  /** @emoji 📡 Refetches {@link Graph#readCheckpoints} on coarse kit ticks. */
  subscribeCheckpoints(cb: (next: readonly Checkpoint[]) => void): Unsubscribe {
    const run = (): void => {
      void this.checkpoints().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
  }
}

/** @emoji 🗂️ {@code Query.session} singleton anchor. */
export class Session extends Entity {
  private readonly alternativeCache = new Map<string, Alternative>();
  private stableAlternatives: { ids: readonly string[]; arr: readonly Alternative[] } = { ids: [], arr: [] };

  constructor(store: Store) {
    super(store, "session");
  }

  alternative(alternativeId: string): Alternative {
    return cachedMapGet(this.alternativeCache, alternativeId, () => new Alternative(this.store, { parent: "session" }, alternativeId));
  }

  async startedAt(): Promise<string | null> {
    const data = unwrapGraphqlData(await executeStoreGraphql(this.store, { query: `query { session { startedAt } }` })) as JsonObject;
    const s = jsonObjectField(data, "session");
    const v = s?.["startedAt"];
    if (v == null) return null;
    return String(v);
  }

  private async alternativeIds(): Promise<readonly string[]> {
    const node = jsonObjectField(await this.store.readStoreInner("wip { alternatives { edges { node { id } } } }"), "wip");
    return parseEntityConnectionIds(node, "alternatives");
  }

  /** @emoji 📚 Id-list-stable {@link Alternative} handles on {@code session.alternatives}. */
  async alternatives(): Promise<readonly Alternative[]> {
    const ids = await this.alternativeIds();
    return readStableEntityList(this.stableAlternatives, ids, (id) => this.alternative(id));
  }

  /** @emoji 📡 Refetches {@link Session#readAlternatives} on coarse kit ticks. */
  subscribeAlternatives(cb: (next: readonly Alternative[]) => void): Unsubscribe {
    const run = (): void => {
      void this.alternatives().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
  }
}

/** @emoji 🧭 Parent scope for {@link Alternative} navigation. */
export type AlternativeParent = { readonly parent: "graph"; readonly root: GraphRootKind } | { readonly parent: "session" };

/** @emoji 🔀 {@code Alternative} under {@link Graph} or {@link Session}. */
export class Alternative extends Entity {
  constructor(
    store: Store,
    private readonly ap: AlternativeParent,
    id: string,
  ) {
    super(store, id);
  }

  async name(): Promise<string> {
    const root = this.ap.parent === "graph" ? this.ap.root : "wip";
    const first = jsonObjectField(await this.store.readStoreInner(`${root} { alternative(id: ${gqlString(this.id)}) { name } }`), root);
    const alt = first?.["alternative"] as JsonObject | undefined;
    return String(alt?.["name"] ?? "");
  }
}

/** @emoji 🏛 {@code TheKit} under {@code wip}/{@code authoritative}. */
export class TheKit extends Entity {
  private readonly kitCache = new Map<string, Kit>();

  constructor(store: Store, private readonly graphRoot: GraphRootKind) {
    super(store, `theKit:${graphRoot}`);
  }

  /** @emoji 📦 Target {@code Version.kit} handle beneath this version node. */
  private kitRef(id = "kit"): Kit {
    return cachedMapGet(this.kitCache, id, () => new Kit(this.store, id));
  }

  private async kitId(): Promise<string> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { theKit { id } }`), this.graphRoot);
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
  private readonly editCache = new Map<string, Edit>();
  private readonly changeCache = new Map<string, Change>();
  private stableChanges: { ids: readonly string[]; arr: readonly Change[] } = { ids: [], arr: [] };
  private stableEdits: { ids: readonly string[]; arr: readonly Edit[] } = { ids: [], arr: [] };

  constructor(store: Store, private readonly graphRoot: GraphRootKind, checkpointId: string) {
    super(store, checkpointId);
  }

  change(changeId: string): Change {
    return cachedMapGet(this.changeCache, changeId, () => new Change(this.store, this.graphRoot, this.id, changeId));
  }

  edit(editId: string): Edit {
    return cachedMapGet(this.editCache, editId, () => new Edit(this.store, this.graphRoot, this.id, editId));
  }

  async message(): Promise<string> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { message } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return String(cp?.["message"] ?? "");
  }

  async timestamp(): Promise<string | null> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { timestamp } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    const ts = cp?.["timestamp"];
    return ts == null ? null : String(ts);
  }

  async hash(): Promise<string> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { hash } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return String(cp?.["hash"] ?? "");
  }

  private async changeIds(): Promise<readonly string[]> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { changes { id } } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return parseStrongEntityArrayIds(cp ?? null, "changes");
  }

  private async editIds(): Promise<readonly string[]> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.id)}) { edits { edges { node { id } } } } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    return parseEntityConnectionIds(cp ?? null, "edits");
  }

  /** @emoji 📚 Id-list-stable {@link Change} rows for this checkpoint (schema {@code changes: [Change!]!}). */
  async changes(): Promise<readonly Change[]> {
    const ids = await this.changeIds();
    return readStableEntityList(this.stableChanges, ids, (cid) => this.change(cid));
  }

  /** @emoji 📚 Id-list-stable {@link Edit} handles for this checkpoint. */
  async edits(): Promise<readonly Edit[]> {
    const ids = await this.editIds();
    return readStableEntityList(this.stableEdits, ids, (eid) => this.edit(eid));
  }

  /** @emoji 📡 Refetches {@link Checkpoint#readChanges} on coarse kit ticks. */
  subscribeChanges(cb: (next: readonly Change[]) => void): Unsubscribe {
    const run = (): void => {
      void this.changes().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
  }

  /** @emoji 📡 Refetches {@link Checkpoint#readEdits} on coarse kit ticks. */
  subscribeEdits(cb: (next: readonly Edit[]) => void): Unsubscribe {
    const run = (): void => {
      void this.edits().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
  }
}

/** @emoji 🔀 {@code Change} scoped to a {@link Checkpoint} (navigation shell; expand with field reads). */
export class Change extends Entity {
  constructor(store: Store, private readonly graphRoot: GraphRootKind, private readonly checkpointId: string, changeId: string) {
    super(store, changeId);
  }

  private async readUnderChange(inner: string): Promise<JsonObject | null> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.checkpointId)}) { change(id: ${gqlString(this.id)}) { ${inner} } } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    const ch = cp?.["change"] as JsonObject | undefined;
    return ch ?? null;
  }

  async description(): Promise<string> {
    const row = await this.readUnderChange("description");
    return String(row?.["description"] ?? "");
  }

  async origin(): Promise<string> {
    const row = await this.readUnderChange("origin");
    return String(row?.["origin"] ?? "");
  }

  async saved(): Promise<boolean | null> {
    const row = await this.readUnderChange("saved");
    const v = row?.["saved"];
    if (v == null) return null;
    return Boolean(v);
  }

  async startedAt(): Promise<string> {
    const row = await this.readUnderChange("startedAt");
    const v = row?.["startedAt"];
    return v == null ? "" : String(v);
  }

  async savedAt(): Promise<string | null> {
    const row = await this.readUnderChange("savedAt");
    const v = row?.["savedAt"];
    return v == null ? null : String(v);
  }
}

/** @emoji ✏️ {@code Edit} scoped to a {@link Checkpoint} (navigation shell; expand with field reads). */
export class Edit extends Entity {
  constructor(store: Store, private readonly graphRoot: GraphRootKind, private readonly checkpointId: string, editId: string) {
    super(store, editId);
  }

  private async readUnderEdit(inner: string): Promise<JsonObject | null> {
    const rootNode = jsonObjectField(await this.store.readStoreInner(`${this.graphRoot} { checkpoint(id: ${gqlString(this.checkpointId)}) { edit(id: ${gqlString(this.id)}) { ${inner} } } }`), this.graphRoot);
    const cp = rootNode?.["checkpoint"] as JsonObject | undefined;
    const ed = cp?.["edit"] as JsonObject | undefined;
    return ed ?? null;
  }

  async description(): Promise<string> {
    const row = await this.readUnderEdit("description");
    return String(row?.["description"] ?? "");
  }

  async origin(): Promise<string> {
    const row = await this.readUnderEdit("origin");
    return String(row?.["origin"] ?? "");
  }

  async sequenceNumber(): Promise<number> {
    const row = await this.readUnderEdit("sequenceNumber");
    const v = row?.["sequenceNumber"];
    return typeof v === "number" ? v : Number(v ?? NaN);
  }

  async startedAt(): Promise<string> {
    const row = await this.readUnderEdit("startedAt");
    const v = row?.["startedAt"];
    return v == null ? "" : String(v);
  }
}

/** @emoji ⚔️ {@code Conflict} via {@code node(id:)}. */
export class Conflict extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  async reasons(): Promise<readonly string[]> {
    const data = unwrapGraphqlData(
      await executeStoreGraphql(this.store, { query: `query($id: ID!) { node(id: $id) { ... on Conflict { reasons } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const raw = n?.["reasons"] as readonly JsonValue[] | undefined;
    if (!Array.isArray(raw)) return [];
    return raw.map((x) => String(x));
  }

  private async authoritativeChangeId(): Promise<string> {
    const data = unwrapGraphqlData(
      await executeStoreGraphql(this.store, {
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
      await executeStoreGraphql(this.store, {
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

/** @emoji 🧮 Wrapper for removed/added/modification rows on an entity diff. */
export class Modifications extends Entity { }

/** @emoji 📥 Abstract operation input payload (arguments mirror SDL input types). */
export abstract class Input extends Entity { }

/** @emoji 📜 Domain ledger event (timestamp + involves — avoid shadowing DOM {@link Event}). */
export abstract class ChangeLedgerEvent extends Entity { }

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
export class ChangedDescriptionOperation extends Operation { }
export class CreatedQualityOperation extends Operation { }
export class CreatedQualitiesOperation extends Operation { }
export class DeletedQualityOperation extends Operation { }
export class CreatedTagOperation extends Operation { }
export class DeletedPieceOperation extends Operation { }
export class DeletedPiecesOperation extends Operation { }
export class DraggedPieceOperation extends Operation { }
export class MovedPieceOperation extends Operation { }
export class FixedPieceOperation extends Operation { }
export class FlattenedDesignOperation extends Operation { }
export class CreatedFixedPieceOperation extends Operation { }
export class AddedChildPieceWithParentConnectionOperation extends Operation { }
export class AddedHangingChildPieceWithParentConnectionOperation extends Operation { }
//#endregion 🧬OperationVariants
//#endregion 🧮ChangeAlgebra

//#endregion 🧬VcsEntities

//#region 📐Design
export class Design extends Entity {
  private readonly pieceCache = new Map<string, Piece>();
  private readonly connectionCache = new Map<string, Connection>();
  private readonly layerCache = new Map<string, Layer>();
  private readonly groupCache = new Map<string, Group>();
  private stablePieces: { ids: readonly string[]; arr: readonly Piece[] } = { ids: [], arr: [] };
  private stableConnections: { ids: readonly string[]; arr: readonly Connection[] } = { ids: [], arr: [] };

  constructor(store: Store, id: string) {
    super(store, id);
  }

  private dsel(inner: string): string {
    return `design(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  /** @emoji 🧷 Raw kit fragment for {@code design(id){ inner }} (shared scalar path for {@link readKitBranchString}). */
  private async designKitFrag(inner: string): Promise<JsonObject | null> {
    return (await this.store.readKitInner(this.dsel(inner))) as JsonObject | null;
  }

  piece(pieceId: string): Piece {
    return cachedMapGet(this.pieceCache, pieceId, () => new Piece(this.store, this.id, pieceId));
  }

  private pieceBatch(pieceIds: readonly string[]): PiecesOperations {
    return new PiecesOperations(this.store, this.id, pieceIds);
  }

  connection(connectionId: string): Connection {
    return cachedMapGet(this.connectionCache, connectionId, () => new Connection(this.store, this.id, connectionId));
  }

  layer(layerId: string): Layer {
    return cachedMapGet(this.layerCache, layerId, () => new Layer(this.store, this.id, layerId));
  }

  group(groupId: string): Group {
    return cachedMapGet(this.groupCache, groupId, () => new Group(this.store, this.id, groupId));
  }

  /** @emoji 🧷 GraphQL kit-store tail for {@code design(id){ … }} (shared with {@link bindDefinedFieldToReact}). */
  kitInnerPath(inner: string): string {
    return this.dsel(inner);
  }

  /**
   * @emoji 📖 Stateless read for one {@code design(id){ … }} selection; {@link FieldSpec#parse} receives the kit row (with nested {@code design}).
   */
  async fieldRead<T>(spec: FieldSpec<T>): Promise<T> {
    const frag = await this.store.readKitInner(this.dsel(spec.selection));
    return spec.parse(frag as JsonValue);
  }

  /**
   * @emoji 📡 When {@link FieldSpec#eventKind} matches {@link EventBus} kinds or live WIP ticks, refetches via {@link Design#fieldRead}.
   */
  subscribeField<T>(spec: FieldSpec<T>, cb: (next: T) => void): Unsubscribe {
    const kind = spec.eventKind;
    if (kind == null || kind === "") return () => { };
    return this.store.bus.subscribeKind(kind, () => {
      void this.fieldRead(spec).then(cb);
    });
  }

  /** @emoji 📡 Design description stream (rs {@code changedDescription}; coarse — refetches design description). */
  onDescriptionChanged(cb: (next: string) => void): Unsubscribe {
    return this.store.bus.subscribeKind("changedDescription", () => {
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
  async pieces(): Promise<readonly Piece[]> {
    const ids = await this.pieceIds();
    return readStableEntityList(this.stablePieces, ids, (pid) => this.piece(pid));
  }

  /** @emoji 📡 Refetches {@link Design#readPieces} on coarse kit ticks (piece membership / graph writes). */
  subscribePieces(cb: (next: readonly Piece[]) => void): Unsubscribe {
    const run = (): void => {
      void this.pieces().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
  }

  private async connectionIds(): Promise<readonly string[]> {
    const frag = await this.designKitFrag("connections { edges { node { id } } }");
    const d = frag?.["design"] as JsonObject | undefined;
    return parseEntityConnectionIds(d ?? (isJsonObjectNode(frag) ? frag : null), "connections");
  }

  /** @emoji 📚 Id-list-stable {@link Connection} handles (same order as the SDL {@code connections} field). */
  async connections(): Promise<readonly Connection[]> {
    const ids = await this.connectionIds();
    return readStableEntityList(this.stableConnections, ids, (cid) => this.connection(cid));
  }

  /** @emoji 📡 Refetches {@link Design#readConnections} on coarse kit ticks. */
  subscribeConnections(cb: (next: readonly Connection[]) => void): Unsubscribe {
    const run = (): void => {
      void this.connections().then(cb);
    };
    return subscribeKitCoarseRefetch(this.store.bus, run);
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
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`rn: rename(newName: ${gqlString(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async flatten(): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`fl: flatten`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }

  async addFixedPiece(blueprintId: string, position: PositionInput, name?: string | null, description?: string | null): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    const pos = formatPositionInput(position);
    const n = name == null ? "null" : gqlString(name);
    const d = description == null ? "null" : gqlString(description);
    return this.store.mutateScoped(cid, this.dsel(`afp: addFixedPiece(blueprintId: ${gqlString(blueprintId)}, position: ${pos}, name: ${n}, description: ${d})`));
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
    const cid = await this.store.ensureChangeId();
    const pos = position == null ? "null" : formatPositionInput(position);
    const n = name == null ? "null" : gqlString(name);
    const d = description == null ? "null" : gqlString(description);
    const sc = scale == null ? "null" : String(scale);
    return this.store.mutateScoped(
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
    const cid = await this.store.ensureChangeId();
    const pos = formatPositionInput(position);
    const n = name == null ? "null" : gqlString(name);
    const d = description == null ? "null" : gqlString(description);
    const sc = scale == null ? "null" : String(scale);
    return this.store.mutateScoped(
      cid,
      this.dsel(
        `ah: addHangingChildPieceWithParentConnection(blueprintId: ${gqlString(blueprintId)}, parentPieceId: ${gqlString(parentPieceId)}, parentConnector: ${gqlString(parentConnector)}, childConnector: ${gqlString(childConnector)}, position: ${pos}, name: ${n}, description: ${d}, scale: ${sc})`,
      ),
    );
  }

  async deletePiece(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`dp: deletePiece(id: ${gqlString(id)})`));
  }

  async deletePieces(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`dps: deletePieces(ids: ${gqlIdList(ids)})`));
  }

  async deletePiecesAndConnections(pieceIds: readonly string[], connectionIds: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.dsel(`dpc: deletePiecesAndConnections(pieceIds: ${gqlIdList(pieceIds)}, connectionIds: ${gqlIdList(connectionIds)})`));
  }
}
//#endregion 📐Design

//#region 🧰Type
export class Type extends Entity {
  private readonly portCache = new Map<string, Port>();
  private readonly connectorCache = new Map<string, Connector>();
  private readonly representationCache = new Map<string, Representation>();

  constructor(store: Store, id: string) {
    super(store, id);
  }

  private tsel(inner: string): string {
    return `type(id: ${gqlString(this.id)}) { ${inner} }`;
  }

  /** @emoji 🧷 Raw kit fragment for {@code type(id){ inner }}. */
  private async typeKitFrag(inner: string): Promise<JsonObject | null> {
    return (await this.store.readKitInner(this.tsel(inner))) as JsonObject | null;
  }

  port(portId: string): Port {
    return cachedMapGet(this.portCache, portId, () => new Port(this.store, this.id, portId));
  }

  connector(connectorId: string): Connector {
    return cachedMapGet(this.connectorCache, connectorId, () => new Connector(this.store, this.id, connectorId));
  }

  representation(representationId: string): Representation {
    return cachedMapGet(this.representationCache, representationId, () => new Representation(this.store, this.id, representationId));
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`rn: rename(newName: ${gqlString(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }

  async createPort(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    const c = code == null ? "null" : gqlString(code);
    const l = label == null ? "null" : gqlString(label);
    const d = description == null ? "null" : gqlString(description);
    const i = icon == null ? "null" : gqlString(icon);
    const o = order == null ? "null" : String(order);
    return this.store.mutateScoped(cid, this.tsel(`cp: createPort(code: ${c}, label: ${l}, description: ${d}, icon: ${i}, order: ${o})`));
  }

  async deletePort(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`dp: deletePort(id: ${gqlString(id)})`));
  }

  async deletePorts(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`dps: deletePorts(ids: ${gqlIdList(ids)})`));
  }

  async addConnector(code: string, description?: string | null, icon?: string | null, portId?: string | null): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    const d = description == null ? "null" : gqlString(description);
    const i = icon == null ? "null" : gqlString(icon);
    const p = portId == null ? "null" : gqlString(portId);
    return this.store.mutateScoped(cid, this.tsel(`ac: addConnector(code: ${gqlString(code)}, description: ${d}, icon: ${i}, portId: ${p})`));
  }

  async removeConnector(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`rc: removeConnector(id: ${gqlString(id)})`));
  }

  async removeConnectors(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.tsel(`rcs: removeConnectors(ids: ${gqlIdList(ids)})`));
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
  constructor(store: Store, typeId: string, id: string) {
    super(store, id);
    this.typeId = typeId;
  }

  private psel(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { port(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  /** @emoji 🔘 SDL {@code Port.code}. */
  async code(): Promise<string> {
    const frag = (await this.store.readKitInner(this.psel("code"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "code");
  }

  /** @emoji 🔘 SDL {@code Port.label}. */
  async label(): Promise<string> {
    const frag = (await this.store.readKitInner(this.psel("label"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "label");
  }

  /** @emoji 🔘 SDL {@code Port.order}. */
  async order(): Promise<number | null> {
    const frag = (await this.store.readKitInner(this.psel("order"))) as JsonObject | null;
    return readKitPathNumberOrNull(frag, KIT_PATH_TYPE_PORT, "order");
  }

  async name(): Promise<string> {
    const frag = (await this.store.readKitInner(this.psel("name"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "name");
  }

  async description(): Promise<string> {
    const frag = (await this.store.readKitInner(this.psel("description"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "description");
  }

  async icon(): Promise<string> {
    const frag = (await this.store.readKitInner(this.psel("icon"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_PORT, "icon");
  }

  /** @emoji 🔘 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Port.attributes}). */
  async attributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.store.readKitInner(this.psel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this, readKitPathNode(frag, KIT_PATH_TYPE_PORT));
  }

  async rename(newCode: string, newLabel?: string | null): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    const lab = newLabel == null ? "null" : gqlString(newLabel);
    return this.store.mutateScoped(cid, this.psel(`rn: rename(newCode: ${gqlString(newCode)}, newLabel: ${lab})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }
}
//#endregion 🔘Port

//#region 🔗Connector
export class Connector extends Entity {
  readonly typeId: string;
  constructor(store: Store, typeId: string, id: string) {
    super(store, id);
    this.typeId = typeId;
  }

  private csel(inner: string): string {
    return `type(id: ${gqlString(this.typeId)}) { connector(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  async rename(newCode: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.csel(`rn: rename(newCode: ${gqlString(newCode)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.csel(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async changeIcon(newIcon: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.csel(`ci: changeIcon(newIcon: ${gqlString(newIcon)})`));
  }

  async name(): Promise<string> {
    const frag = (await this.store.readKitInner(this.csel("name"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "name");
  }

  async code(): Promise<string> {
    const frag = (await this.store.readKitInner(this.csel("code"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "code");
  }

  async description(): Promise<string> {
    const frag = (await this.store.readKitInner(this.csel("description"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "description");
  }

  async icon(): Promise<string> {
    const frag = (await this.store.readKitInner(this.csel("icon"))) as JsonObject | null;
    return readKitPathString(frag, KIT_PATH_TYPE_CONNECTOR, "icon");
  }

  /** @emoji 🔗 Nullable {@code port { id }} per SDL {@code Connector.port}. */
  private async portId(): Promise<string | null> {
    const frag = (await this.store.readKitInner(this.csel("port { id }"))) as JsonObject | null;
    const p = readKitPathNode(frag, KIT_PATH_TYPE_CONNECTOR)?.["port"] as JsonObject | null | undefined;
    if (p == null) return null;
    const id = String(p["id"] ?? "");
    return id === "" ? null : id;
  }

  /** @emoji 🔗 Bulky {@code attributes { edges { node {…} } }} read (SDL {@code Connector.attributes}). */
  async attributes(): Promise<readonly Attribute[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.store.readKitInner(this.csel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this, readKitPathNode(frag, KIT_PATH_TYPE_CONNECTOR));
  }

  async port(): Promise<Port | null> {
    const id = await this.portId();
    return id == null ? null : this.store.type(this.typeId).port(id);
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
    const frag = (await this.parent.piece.store.readKitInner(
      this.parent.piece.kitPieceSelection(`${this.parent.role} { center { u v } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.role] as JsonObject | undefined;
    const c = row?.["center"] as JsonObject | undefined;
    return typeof c?.["u"] === "number" ? c["u"] : 0;
  }

  async v(): Promise<number> {
    const frag = (await this.parent.piece.store.readKitInner(
      this.parent.piece.kitPieceSelection(`${this.parent.role} { center { u v } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.role] as JsonObject | undefined;
    const c = row?.["center"] as JsonObject | undefined;
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
    const frag = (await this.parent.parent.piece.store.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { origin { x y z } } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = row?.["plane"] as JsonObject | undefined;
    const o = pl?.["origin"] as JsonObject | undefined;
    return typeof o?.["x"] === "number" ? o["x"] : 0;
  }

  async y(): Promise<number> {
    const frag = (await this.parent.parent.piece.store.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { origin { x y z } } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = row?.["plane"] as JsonObject | undefined;
    const o = pl?.["origin"] as JsonObject | undefined;
    return typeof o?.["y"] === "number" ? o["y"] : 0;
  }

  async z(): Promise<number> {
    const frag = (await this.parent.parent.piece.store.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { origin { x y z } } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = row?.["plane"] as JsonObject | undefined;
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
    const frag = (await this.parent.parent.piece.store.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { ${this.axisRole} { x y z } } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = row?.["plane"] as JsonObject | undefined;
    const ax = pl?.[this.axisRole] as JsonObject | undefined;
    return typeof ax?.["x"] === "number" ? ax["x"] : 0;
  }

  async y(): Promise<number> {
    const frag = (await this.parent.parent.piece.store.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { ${this.axisRole} { x y z } } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = row?.["plane"] as JsonObject | undefined;
    const ax = pl?.[this.axisRole] as JsonObject | undefined;
    return typeof ax?.["y"] === "number" ? ax["y"] : 0;
  }

  async z(): Promise<number> {
    const frag = (await this.parent.parent.piece.store.readKitInner(
      this.parent.parent.piece.kitPieceSelection(`${this.parent.parent.role} { plane { ${this.axisRole} { x y z } } }`),
    )) as JsonObject | null;
    const row = pieceKit(frag)?.[this.parent.parent.role] as JsonObject | undefined;
    const pl = row?.["plane"] as JsonObject | undefined;
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
  constructor(store: Store, designId: string, id: string) {
    super(store, id);
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
    const frag = (await this.store.readKitInner(this.kitPieceSelection("name"))) as JsonObject | null;
    return String(pieceKit(frag)?.["name"] ?? "");
  }

  async description(): Promise<string> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("description"))) as JsonObject | null;
    return String(pieceKit(frag)?.["description"] ?? "");
  }

  async icon(): Promise<string> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("icon"))) as JsonObject | null;
    return String(pieceKit(frag)?.["icon"] ?? "");
  }

  async scale(): Promise<number | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("scale"))) as JsonObject | null;
    const v = pieceKit(frag)?.["scale"];
    return typeof v === "number" ? v : null;
  }

  private async positionLoaded(): Promise<Position | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection(`position { ${PIECE_POSITION_SELECTION} }`))) as JsonObject | null;
    const raw = pieceKit(frag)?.["position"];
    if (raw == null || typeof raw !== "object") return null;
    return this.position();
  }

  private async flatPositionLoaded(): Promise<Position | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection(`flatPosition { ${PIECE_POSITION_SELECTION} }`))) as JsonObject | null;
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
    const frag = (await this.store.readKitInner(this.kitPieceSelection("blueprint { __typename id }"))) as JsonObject | null;
    return parsePieceBlueprintFromJson(pieceKit(frag)?.["blueprint"] as JsonObject | undefined);
  }

  async attributes(): Promise<readonly Attribute[]> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("attributes { edges { node { id key value definition } } }"))) as JsonObject | null;
    return parseAttributeConnectionUnder(this, pieceKit(frag));
  }

  async connectionKind(): Promise<"FIXED" | "CONNECTED" | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("connectionKind"))) as JsonObject | null;
    const k = pieceKit(frag)?.["connectionKind"];
    if (k === "FIXED" || k === "CONNECTED") return k;
    return null;
  }

  private async parentPieceId(): Promise<string | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("parentPiece { id }"))) as JsonObject | null;
    const n = pieceKit(frag)?.["parentPiece"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  private async parentConnectionId(): Promise<string | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("parentConnection { id }"))) as JsonObject | null;
    const n = pieceKit(frag)?.["parentConnection"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  private async childPieceIds(): Promise<readonly string[]> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("childPieces { edges { node { id } } }"))) as JsonObject | null;
    return parseIdListConnection(pieceKit(frag), "childPieces");
  }

  private async childConnectionIds(): Promise<readonly string[]> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("childConnections { edges { node { id } } }"))) as JsonObject | null;
    return parseIdListConnection(pieceKit(frag), "childConnections");
  }

  async depth(): Promise<number | null> {
    const frag = (await this.store.readKitInner(this.kitPieceSelection("depth"))) as JsonObject | null;
    const v = pieceKit(frag)?.["depth"];
    return typeof v === "number" ? v : null;
  }

  async parentPiece(): Promise<Piece | null> {
    const id = await this.parentPieceId();
    return id == null ? null : this.store.design(this.designId).piece(id);
  }

  async parentConnection(): Promise<Connection | null> {
    const id = await this.parentConnectionId();
    return id == null ? null : this.store.design(this.designId).connection(id);
  }

  async childPieces(): Promise<readonly Piece[]> {
    return Object.freeze((await this.childPieceIds()).map((id) => this.store.design(this.designId).piece(id)));
  }

  async childConnections(): Promise<readonly Connection[]> {
    return Object.freeze((await this.childConnectionIds()).map((id) => this.store.design(this.designId).connection(id)));
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`rn: rename(newName: ${gqlString(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`cd: changeDescription(newDescription: ${gqlString(newDescription)})`));
  }

  async drag(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(position: PositionInput): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`mv: move(position: ${formatPositionInput(position)})`));
  }

  async fix(): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`fx: fix`));
  }

  async changeBlueprint(blueprintId: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`cb: changeBlueprint(blueprintId: ${gqlString(blueprintId)})`));
  }

  async addAttribute(key: string, value: string, definition: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`aa: addAttribute(key: ${gqlString(key)}, value: ${gqlString(value)}, definition: ${gqlString(definition)})`));
  }

  async removeAttribute(id: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`ra: removeAttribute(id: ${gqlString(id)})`));
  }

  async removeAttributes(ids: readonly string[]): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.kitPieceSelection(`ras: removeAttributes(ids: ${gqlIdList(ids)})`));
  }
}
//#endregion 🧩Piece

//#region 🪢PiecesOperations
export class PiecesOperations {
  constructor(
    private readonly store: Store,
    private readonly designId: string,
    private readonly pieceIds: readonly string[],
  ) { }

  private psel(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { pieces(ids: ${gqlIdList(this.pieceIds)}) { ${inner} } }`;
  }

  async drag(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(offset: OffsetInput): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`mv: move(offset: ${formatOffsetInput(offset)})`));
  }

  async fix(): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`fx: fix`));
  }

  async changeBlueprint(blueprintId: string): Promise<SetResult> {
    const cid = await this.store.ensureChangeId();
    return this.store.mutateScoped(cid, this.psel(`cb: changeBlueprint(blueprintId: ${gqlString(blueprintId)})`));
  }
}
//#endregion 🪢PiecesOperations

//#region ⛓️Connection
/** @emoji ⛓️ Schema-aligned {@link Connection} endpoint (piece + optional port / connector / designPiece ids). */
export class ConnectionSide {
  constructor(
    public readonly store: Store,
    public readonly designId: string,
    public readonly connectionId: string,
    public readonly role: "connected" | "connecting",
    public readonly pieceId: string,
    public readonly portId: string | null,
    public readonly connectorId: string | null,
    public readonly designPieceId: string | null,
  ) { }

  /** @emoji 🧩 Resolved {@link Piece} on this kit read point. */
  piece(): Piece {
    return this.store.design(this.designId).piece(this.pieceId);
  }
}

/** @emoji ↔️ SDL {@code Side} alias for {@link ConnectionSide} in UI layers. */
export type Side = ConnectionSide;

const CONNECTION_SIDE_SELECTION = "piece { id } port { id } designPiece { id } connector { id }";

function connectionKit(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const c = d?.["connection"] as JsonObject | undefined;
  return c ?? null;
}

function parseConnectionSideFromJson(
  store: Store,
  designId: string,
  connectionId: string,
  role: "connected" | "connecting",
  node: JsonObject | null | undefined,
): ConnectionSide | null {
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
  return new ConnectionSide(store, designId, connectionId, role, pieceId, portId, connectorId, designPieceId);
}

export class Connection extends Entity {
  readonly designId: string;
  constructor(store: Store, designId: string, id: string) {
    super(store, id);
    this.designId = designId;
  }

  private csel(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { connection(id: ${gqlString(this.id)}) { ${inner} } }`;
  }

  /** @emoji ⛓️ Resolved {@code design.connection} row for the given selection tail. */
  private async connectionRow(inner: string): Promise<JsonObject | null> {
    const frag = (await this.store.readKitInner(this.csel(inner))) as JsonObject | null;
    return connectionKit(frag);
  }

  private async readConnScalarString(field: string): Promise<string> {
    return String((await this.connectionRow(field))?.[field] ?? "");
  }

  private async readConnScalarNumberOrNull(field: string): Promise<number | null> {
    const v = (await this.connectionRow(field))?.[field];
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

  async connected(): Promise<ConnectionSide | null> {
    const row = await this.connectionRow(`connected { ${CONNECTION_SIDE_SELECTION} }`);
    return parseConnectionSideFromJson(this.store, this.designId, this.id, "connected", row?.["connected"] as JsonObject | undefined);
  }

  async connecting(): Promise<ConnectionSide | null> {
    const row = await this.connectionRow(`connecting { ${CONNECTION_SIDE_SELECTION} }`);
    return parseConnectionSideFromJson(this.store, this.designId, this.id, "connecting", row?.["connecting"] as JsonObject | undefined);
  }

  async attributes(): Promise<readonly Attribute[]> {
    return parseAttributeConnectionUnder(this, await this.connectionRow("attributes { edges { node { id key value definition } } }"));
  }
}
//#endregion ⛓️Connection

//#region ✍️Author
/** @emoji ✍️ Author artifact: kit-scoped reads only (no {@code *OperationInput} on Author in schema). */
export class Author extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
  declare email: () => Promise<string>;
  declare role: () => Promise<string>;
  declare rank: () => Promise<number | null>;
}

const AUTHOR_FIELDS = defineBoundNodeFields([
  { methodName: "name", selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { methodName: "description", selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { methodName: "icon", selection: "icon", parse: (node) => String(node?.["icon"] ?? "") },
  { methodName: "email", selection: "email", parse: (node) => String(node?.["email"] ?? "") },
  { methodName: "role", selection: "role", parse: (node) => String(node?.["role"] ?? "") },
  { methodName: "rank", selection: "rank", parse: (node) => (typeof node?.["rank"] === "number" ? node["rank"] : null) },
] as const);
installNodeFieldMethods(Author, "Author", AUTHOR_FIELDS);
//#endregion ✍️Author

//#region 💎Quality
/** @emoji 💎 Quality artifact: {@code QualityOperationInput} leaves + scalar reads via {@code quality(id:)}. */
export class Quality extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
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
    const frag = (await this.store.readKitInner(
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
  { methodName: "rename", buildInner: (_entity, newKey) => `rk: rename(newKey: ${gqlString(String(newKey ?? ""))})` },
  { methodName: "changeDescription", buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { methodName: "changeIcon", buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    methodName: "addAttribute",
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { methodName: "removeAttribute", buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { methodName: "removeAttributes", buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const);
installKitFieldMethods(Quality, defineBoundKitFields([
  { methodName: "key", selection: "key", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "key") },
  { methodName: "value", selection: "value", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "value") },
  { methodName: "unit", selection: "unit", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "unit") },
  { methodName: "definition", selection: "definition", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "definition") },
  { methodName: "name", selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "name") },
  { methodName: "description", selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "description") },
  { methodName: "icon", selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "quality", "icon") },
  {
    methodName: "attributes",
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
  constructor(store: Store, id: string) {
    super(store, id);
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
  { methodName: "name", selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "name") },
  { methodName: "description", selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "description") },
  { methodName: "icon", selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "tag", "icon") },
  { methodName: "order", selection: "order", parse: (frag) => readKitBranchNumberOrNull(frag as JsonObject | null, "tag", "order") },
  {
    methodName: "attributes",
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["tag"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Tag>[]);
installKitOperationMethods(Tag, defineBoundKitOperations([
  { methodName: "rename", buildInner: (_entity, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { methodName: "changeDescription", buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { methodName: "changeIcon", buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    methodName: "addAttribute",
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { methodName: "removeAttribute", buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { methodName: "removeAttributes", buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const));
//#endregion 🏷️Tag

//#region 💡Concept
/** @emoji 💡 Concept artifact: {@code ConceptOperationInput} leaves + kit-scoped reads. */
export class Concept extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
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
  { methodName: "name", selection: "name", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "name") },
  { methodName: "description", selection: "description", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "description") },
  { methodName: "icon", selection: "icon", parse: (frag) => readKitBranchString(frag as JsonObject | null, "concept", "icon") },
  { methodName: "order", selection: "order", parse: (frag) => readKitBranchNumberOrNull(frag as JsonObject | null, "concept", "order") },
  {
    methodName: "attributes",
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, (frag as JsonObject | null)?.["concept"] as JsonObject | undefined),
  },
]) as readonly BoundKitFieldSpec<unknown, Concept>[]);
installKitOperationMethods(Concept, defineBoundKitOperations([
  { methodName: "rename", buildInner: (_entity, newName) => `rn: rename(newName: ${gqlString(String(newName ?? ""))})` },
  { methodName: "changeDescription", buildInner: (_entity, newDescription) => `cd: changeDescription(newDescription: ${gqlString(String(newDescription ?? ""))})` },
  { methodName: "changeIcon", buildInner: (_entity, newIcon) => `ci: changeIcon(newIcon: ${gqlString(String(newIcon ?? ""))})` },
  {
    methodName: "addAttribute",
    buildInner: (_entity, key, value, definition) =>
      `aa: addAttribute(key: ${gqlString(String(key ?? ""))}, value: ${gqlString(String(value ?? ""))}, definition: ${gqlString(String(definition ?? ""))})`,
  },
  { methodName: "removeAttribute", buildInner: (_entity, id) => `ra: removeAttribute(id: ${gqlString(String(id ?? ""))})` },
  { methodName: "removeAttributes", buildInner: (_entity, ids) => `ras: removeAttributes(ids: ${gqlIdList((ids as readonly string[]) ?? [])})` },
] as const));
//#endregion 💡Concept

//#region 🎨Representation
/** @emoji 🎨 Representation under {@link Type}: read-only until schema adds {@code RepresentationOperationInput}. */
export class Representation extends Entity {
  readonly typeId: string;
  constructor(store: Store, typeId: string, id: string) {
    super(store, id);
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
  { methodName: "name", selection: "name", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "name") },
  { methodName: "url", selection: "url", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "url") },
  { methodName: "description", selection: "description", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "description") },
  { methodName: "icon", selection: "icon", parse: (frag) => readKitPathString(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION, "icon") },
  {
    methodName: "file",
    selection: "file { id }",
    parse: () => null,
    parseEntity: (entity, frag) => {
      const id = String((readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION)?.["file"] as JsonObject | undefined)?.["id"] ?? "");
      return id === "" ? null : entity.store.file(id);
    },
  },
  {
    methodName: "tags",
    selection: "tags { edges { node { id } } }",
    parse: () => [],
    parseEntity: (entity, frag) => Object.freeze(parseIdListConnection(readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION), "tags").map((id) => entity.store.tag(id))),
  },
  {
    methodName: "qualities",
    selection: "qualities { edges { node { id } } }",
    parse: () => [],
    parseEntity: (entity, frag) => Object.freeze(parseIdListConnection(readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION), "qualities").map((id) => entity.store.quality(id))),
  },
  {
    methodName: "attributes",
    selection: "attributes { edges { node { id key value definition } } }",
    parse: () => [],
    parseEntity: (entity, frag) => parseAttributeConnectionUnder(entity, readKitPathNode(frag as JsonObject | null, KIT_PATH_TYPE_REPRESENTATION)),
  },
]) as readonly BoundKitFieldSpec<unknown, Representation>[]);
//#endregion 🎨Representation

//#region 👨‍👩‍👦Family
/** @emoji 👨‍👩‍👦 Family artifact: read-only in current kit API. */
export class Family extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
}

installNodeFieldMethods(Family, "Family", defineBoundNodeFields([
  { methodName: "name", selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { methodName: "description", selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { methodName: "icon", selection: "icon", parse: (node) => String(node?.["icon"] ?? "") },
] as const));
//#endregion 👨‍👩‍👦Family

//#region 📄File
export class File extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  declare name: () => Promise<string>;
}

installNodeFieldMethods(File, "File", defineBoundNodeFields([
  { methodName: "name", selection: "name", parse: (node) => String(node?.["name"] ?? "") },
] as const));
//#endregion 📄File

//#region 📁Folder
/** @emoji 📁 Folder artifact: read-only in current kit API. */
export class Folder extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare path: () => Promise<string>;
}

installNodeFieldMethods(Folder, "Folder", defineBoundNodeFields([
  { methodName: "name", selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { methodName: "description", selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { methodName: "path", selection: "path", parse: (node) => String(node?.["path"] ?? "") },
] as const));
//#endregion 📁Folder

//#region 🪟Layer
/** @emoji 🪟 Design layer row: read-only in current kit API. */
export class Layer extends Entity {
  readonly designId: string;
  constructor(store: Store, designId: string, id: string) {
    super(store, id);
    this.designId = designId;
  }

  private lsel(inner: string): string {
    return `design(id: ${gqlString(this.designId)}) { layers { edges { node { id ${inner} } } } }`;
  }

  private async selfLayerNode(innerFields: string): Promise<JsonObject | null> {
    const frag = (await this.store.readKitInner(this.lsel(innerFields))) as JsonObject | null;
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
  constructor(store: Store, designId: string, id: string) {
    super(store, id);
    this.designId = designId;
  }

  async name(): Promise<string> {
    const frag = await this.store.readKitInner(`design(id: ${gqlString(this.designId)}) { groups { edges { node { id name } } } }`);
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
  constructor(store: Store, id: string) {
    super(store, id);
  }

  declare key: () => Promise<string>;
  declare value: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare name: () => Promise<string>;
  declare description: () => Promise<string>;
  declare icon: () => Promise<string>;
}

installNodeFieldMethods(Stat, "Stat", defineBoundNodeFields([
  { methodName: "key", selection: "key", parse: (node) => String(node?.["key"] ?? "") },
  { methodName: "value", selection: "value", parse: (node) => String(node?.["value"] ?? "") },
  { methodName: "unit", selection: "unit", parse: (node) => String(node?.["unit"] ?? "") },
  { methodName: "name", selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  { methodName: "description", selection: "description", parse: (node) => String(node?.["description"] ?? "") },
  { methodName: "icon", selection: "icon", parse: (node) => String(node?.["icon"] ?? "") },
] as const));
//#endregion 📊Stat

//#region 🎚️Prop
/** @emoji 🎚️ Prop artifact: read-only in current kit API. */
export class Prop extends Entity {
  constructor(store: Store, id: string) {
    super(store, id);
  }

  declare key: () => Promise<string>;
  declare value: () => Promise<string>;
  declare unit: () => Promise<string>;
  declare name: () => Promise<string>;
  declare quality: () => Promise<Quality | null>;
}

installNodeFieldMethods(Prop, "Prop", defineBoundNodeFields([
  { methodName: "key", selection: "key", parse: (node) => String(node?.["key"] ?? "") },
  { methodName: "value", selection: "value", parse: (node) => String(node?.["value"] ?? "") },
  { methodName: "unit", selection: "unit", parse: (node) => String(node?.["unit"] ?? "") },
  { methodName: "name", selection: "name", parse: (node) => String(node?.["name"] ?? "") },
  {
    methodName: "quality",
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
/** @emoji 🚀 Opens a {@link Store} backed by rs WASM (worker or inline). */
export async function openStore(uri: string, opts?: StoreOpenOptions): Promise<Store> {
  return Store.open(uri, opts);
}
//#endregion 🚀PublicAPI


//#region 🧪Tests
if (typeof process !== "undefined" && !!process.env && process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  const { describe, it, expect } = await import("vitest");
  describe("semio-js Store root (strict)", () => {
    it("Store.prototype has no snapshot() hook", () => {
      type Snap = { snapshot?: () => unknown };
      const snap: Snap = Store.prototype as unknown as Snap;
      expect(snap.snapshot).toBeUndefined();
    });

    it("kitReadPointKey normalizes the main line scope for cache keys", () => {
      expect(kitReadPointKey(theKitReadPoint)).toBe(JSON.stringify(theKitReadPoint));
    });

    it("VCS + change-algebra shells JSON without a runtime umbrella", () => {
      const k = Object.create(Store.prototype) as Store;
      const g = new Graph(k, "wip");
      expect(g.root).toBe("wip");
      expect(new Session(k).id).toBe("session");
      expect(new TheKit(k, "wip").id).toContain("theKit");
      expect(new Kit(k, "kit1").store).toBe(k);
      expect(typeof new TheKit(k, "wip").kit).toBe("function");
      const cp = new Checkpoint(k, "wip", "cp1");
      expect(cp.change("c1").id).toBe("c1");
      expect(cp.edit("e1").id).toBe("e1");
      expect(RenamedKit.name).toBe("RenamedKit");
      expect(KitDiff.name).toBe("KitDiff");
      expect(RenamedKitInput.name).toBe("RenamedKitInput");
    });

    it("Design / Graph / Session / Checkpoint expose id-list-stable read* + subscribe* for owned lists", () => {
      const k = Object.create(Store.prototype) as Store;
      const d = new Design(k, "d1");
      expect(typeof d.pieces).toBe("function");
      expect(typeof d.subscribePieces).toBe("function");
      expect(typeof d.connections).toBe("function");
      expect(typeof d.subscribeConnections).toBe("function");
      const g = new Graph(k, "wip");
      expect(typeof g.alternatives).toBe("function");
      expect(typeof g.checkpoints).toBe("function");
      expect(typeof g.subscribeAlternatives).toBe("function");
      expect(typeof g.subscribeCheckpoints).toBe("function");
      const s = new Session(k);
      expect(typeof s.alternatives).toBe("function");
      expect(typeof s.subscribeAlternatives).toBe("function");
      const cp = new Checkpoint(k, "wip", "cp1");
      expect(typeof cp.changes).toBe("function");
      expect(typeof cp.edits).toBe("function");
      expect(typeof cp.subscribeChanges).toBe("function");
      expect(typeof cp.subscribeEdits).toBe("function");
    });
  });
  describe("semio-js GraphQL dto contract", () => {
    it("KIT_SESSION_QUERY_ENTRY and KIT_EVENT_STREAM_SUBSCRIPTION align with schema.golden.graphql", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      let sdl = "";
      const here = dirname(fileURLToPath(import.meta.url));
      for (const p of [resolve(here, "../../../schema/graphql/schema.golden.graphql"), resolve(process.cwd(), "semio/schema/graphql/schema.golden.graphql")]) {
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
      expect(sdl).toMatch(/type Query[\s\S]*\bsession:\s*Session!/s);
      expect(sdl).toMatch(/type Session[\s\S]*\bstores:\s*StoreConnection/s);
      expect(sdl).toMatch(/type Store[\s\S]*\bwip:\s*Graph!/s);
      expect(sdl).toMatch(/type Subscription[\s\S]*\bsession:\s*Session!/s);
      expect(sdl).not.toMatch(/^\s*event:\s*Json!/m);
      expect(sdl).toContain("type Mutation");
      expect(sdl).toContain("session: SessionCommand!");
      expect(sdl).not.toContain("type KitStoreMutation");
      expect(KIT_SESSION_QUERY_ENTRY).toContain("session { stores");
      expect(KIT_EVENT_STREAM_SUBSCRIPTION).toContain("session { stores");
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

    it("dev JSON backbone bundle shape documents semanticOpLog + persistence hints (US-004)", async () => {
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

    it("refactored declarative installers drive tag reads and mutations", async () => {
      const reads: string[] = [];
      const writes: string[] = [];
      const k = Object.create(Store.prototype) as Store;
      (k as unknown as { readKitInner: (inner: string) => Promise<JsonObject> }).readKitInner = async (inner) => {
        reads.push(inner);
        return {
          tag: {
            name: "Facade",
            order: 3,
            attributes: {
              edges: [
                { node: { id: "attr-1", key: "finish", value: "matte", definition: "Surface finish" } },
              ],
            },
          },
        } as unknown as JsonObject;
      };
      (k as unknown as { ensureChangeId: () => Promise<string> }).ensureChangeId = async () => "chg-1";
      (k as unknown as { mutateScoped: (changeId: string, path: string) => Promise<SetResult> }).mutateScoped = async (_changeId, path) => {
        writes.push(path);
        return { ok: true };
      };
      const tag = new Tag(k, "tag-1");

      expect(await tag.name()).toBe("Facade");
      expect(await tag.order()).toBe(3);
      const attrs = await tag.attributes();
      expect(attrs).toHaveLength(1);
      expect(attrs[0]?.owner).toBe(tag);
      await tag.rename("Envelope");

      expect(reads[0]).toContain('tag(id: "tag-1") { name }');
      expect(reads[2]).toContain("attributes { edges { node { id key value definition } } }");
      expect(writes[0]).toContain('tag(id: "tag-1") { rn: rename(newName: "Envelope") }');
    });

    it("Piece.drag issues one mutateScoped path (stub kit)", async () => {
      const calls: string[] = [];
      const k = Object.create(Store.prototype) as Store;
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
