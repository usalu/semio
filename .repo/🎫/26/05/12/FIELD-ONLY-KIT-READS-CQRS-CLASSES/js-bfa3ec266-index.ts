// #region ­ƒº▓Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later ÔÇö semio/js: stateless {@link Kit} + GraphQL transport (WASM worker or inline); no client-side kit cache.
// #endregion ­ƒº▓Header

//#region ­ƒîÉTransport
/** @emoji ­ƒºÁ Bundled worker ÔÇö Vite resolves `@semio/rs-wasm`; Blob workers cannot import bare specifiers. */
export function createKitStoreWorker(): Worker {
  return new Worker(new URL("./kit-store.worker.ts", import.meta.url), { type: "module" });
}

export type JsonValue = string | number | boolean | null | readonly JsonValue[] | JsonObject;
export type JsonObject = { readonly [k: string]: JsonValue };

type GraphQlVariables = JsonObject;

type KitGraphqlResponseEnvelope<TData> = Readonly<{
  data?: TData | null;
  errors?: readonly { readonly message?: string }[];
}>;

function parseJsonValue(text: string): JsonValue {
  return JSON.parse(text) as JsonValue;
}

function isJsonObjectNode(v: JsonValue | null | undefined): v is JsonObject {
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

type WasmExecuteFn = (requestJson: string) => Promise<string>;
type WasmSubscribeFn = (requestJson: string, onEvent: (eventJson: string) => void) => Promise<void>;

class InlineWasmTransport {
  constructor(
    private readonly handle: {
      execute: WasmExecuteFn;
      subscribe: WasmSubscribeFn;
      free?: () => void;
    },
  ) {}
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
    if (parts.length) return parts.join(" ÔÇö ");
  }
  return "worker script or module failed to load";
}

class WorkerStringTransport {
  private nextSerial = 0;
  constructor(private readonly worker: Worker) {}

  init(dto: KitBootstrapJson): Promise<void> {
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
      this.worker.postMessage(JSON.stringify({ op: "init", dto }));
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

/** @emoji ­ƒîÉ Thin GraphQL wire: JSON request in, JSON string out; pairs with rs {@code KitStoreHandle}. */
export class GqlTransport {
  constructor(private readonly inner: WorkerStringTransport | InlineWasmTransport) {}

  async executeJson(body: { readonly query: string; readonly variables?: GraphQlVariables; readonly operationName?: string }, timeoutMs: number): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    const json = await withTimeout(this.inner.execute(JSON.stringify(body)), timeoutMs, "graphql");
    return parseJsonValue(json) as KitGraphqlResponseEnvelope<JsonValue>;
  }

  async subscribeJson(body: { readonly query: string; readonly variables?: GraphQlVariables }, onEvent: (env: KitGraphqlResponseEnvelope<JsonValue>) => void): Promise<void> {
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

/** @emoji ­ƒôí Demultiplexes {@code Subscription.event} JSON into listener fan-out (no client cache). */
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

export const KIT_EVENT_STREAM_SUBSCRIPTION = `subscription { event }` as const;

//#endregion ­ƒîÉTransport

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

/** @emoji ­ƒº¥ Normalized set/mutation error from rs {@code SetError}. */
export type SetError = { kind: SetErrorKind; message: string; field?: string; entity?: { kind: string; id: string } };

/** @emoji ­ƒº¥ Mutation receipt (no speculative client-side apply). */
export type SetResult = { ok: true } | { ok: false; error: SetError };

export type ChangeId = string;

/** @emoji ­ƒº¡ Materialization anchor for target-schema reads. */
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

/** @emoji ­ƒº¥ JSON seed passed to rs to spawn the wip overlay (GraphQL authority). */
export type KitBootstrapJson = JsonObject;

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

function __scopedKitMutationBody(changeId: string, kitSelection: string): { readonly query: string; readonly variables: GraphQlVariables } {
  return {
    query: `mutation($changeId: ID!) { session { theKit { unsavedChange(id: $changeId) { kit { ${kitSelection} } } } } }`,
    variables: { changeId },
  };
}

function kitSessionWipStoreSelect(point: KitReadPoint, innerOnKitStore: string): { query: string; variables: GraphQlVariables } {
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

async function kitGraphqlRun(handle: { execute(requestJson: string): Promise<string> }, body: { query: string; variables?: GraphQlVariables; operationName?: string }, timeoutMs?: number): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
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

async function __readSemioWasmBytesFromMonorepoCandidates(): Promise<Uint8Array | undefined> {
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

//#region ­ƒº▒Classes
//#region ­ƒÄÆKit
/**
 * @emoji ­ƒÄÆ Stateless kit fa├ºade: owns {@link GqlTransport} + {@link EventBus}; every read is a fresh GraphQL round-trip.
 */
export class Kit {
  private readonly timeoutMs: number;
  private readonly handle: KitGraphqlHandle;
  private readonly innerTransport: WorkerStringTransport | InlineWasmTransport;
  private gqlLoopRunning = false;
  private disposed = false;
  private activeReadPoint: KitReadPoint = theKitReadPoint;
  private kitWriteChangeId: string | null = null;

  /** @emoji ­ƒîÉ GraphQL executor (JSON wire). */
  readonly gql: GqlTransport;
  /** @emoji ­ƒôí Demuxed subscription fan-out. */
  readonly bus: EventBus;

  private constructor(timeoutMs: number, inner: WorkerStringTransport | InlineWasmTransport) {
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

  private async gqlRun(body: { query: string; variables?: GraphQlVariables; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    this.ensureAlive();
    return kitGraphqlRun(this.handle, body, this.timeoutMs);
  }

  /** @emoji ­ƒîÉ Public GraphQL round-trip (root {@code Query} / {@code Mutation}), for {@code node(id:)} reads. */
  async runGraphql(body: { query: string; variables?: GraphQlVariables; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    return this.gqlRun(body);
  }

  /** @emoji ­ƒº¥ Reads a selection inside scoped {@code kit { ÔÇª }} for {@link activeReadPoint}. */
  async readKitInner(inner: string, variables: GraphQlVariables = {}): Promise<JsonObject | null> {
    const { query, variables: v0 } = kitSessionWipStoreSelect(this.activeReadPoint, inner);
    const data = kitGraphqlData(await this.gqlRun({ query, variables: { ...v0, ...variables } })) as JsonValue;
    return gqlDataSessionWipKitStore(data, this.activeReadPoint);
  }

  /** @emoji ­ƒº¥ Runs {@code mutation session { theKit { unsavedChange { kit { ÔÇª } } } }} when {@linkcode changeId} is set. */
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

  async hydrateKitStoreBundleJson(json: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { hydrateKitStoreBundleJson(json: ${__gqlStr(json)}) }` });
    return gqlOkFromEnvelope(env);
  }

  /** @emoji ­ƒº¥ Warm-path query after WASM init. */
  private async warmGraphqlRead(): Promise<void> {
    await this.readKitInner("id name");
  }

  static async open(seed: KitBootstrapJson, opts?: KitOpenOptions): Promise<Kit> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const wasmSpecifier = opts?.wasmSpecifier ?? (globalThis as { __SEMIO_WASM_SPECIFIER__?: string }).__SEMIO_WASM_SPECIFIER__ ?? "@semio/rs-wasm";
    const dto: KitBootstrapJson = JSON.parse(JSON.stringify(seed)) as KitBootstrapJson;
    const preferInlineWasmInVitest = (() => {
      try {
        const env = (import.meta as { env?: JsonObject }).env;
        if (env && Boolean(env["VITEST"])) return true;
      } catch {
        /* ignore */
      }
      return typeof process !== "undefined" && !!process.env && "VITEST" in process.env;
    })();

    const wasmBytesPre = await __readSemioWasmBytesFromMonorepoCandidates();
    const useDedicatedWorker = typeof Worker !== "undefined" && !preferInlineWasmInVitest && wasmBytesPre == null;

    if (useDedicatedWorker) {
      const worker = opts?.workerFactory?.() ?? createKitStoreWorker();
      const wt = new WorkerStringTransport(worker);
      try {
        await wt.init(dto);
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
    const handleUnknown = mod.KitStoreHandle.create(dto as object);
    const wasmHandle = handleUnknown instanceof Promise ? await handleUnknown : handleUnknown;
    if (wasmHandle == null || typeof (wasmHandle as { execute?: unknown }).execute !== "function") {
      throw new Error("KitStoreHandle.create did not return execute()");
    }
    const t = new InlineWasmTransport(wasmHandle as { execute: WasmExecuteFn; subscribe: WasmSubscribeFn; free?: () => void });
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

  file(id: string): FileEntity {
    return new FileEntity(this, id);
  }

  folder(id: string): FolderEntity {
    return new FolderEntity(this, id);
  }

  author(id: string): Author {
    return new Author(this, id);
  }

  stat(id: string): StatEntity {
    return new StatEntity(this, id);
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
}
//#endregion ­ƒÄÆKit
export type CoordinateInputWire = Readonly<{ u: number; v: number }>;
export type VectorInputWire = Readonly<{ x: number; y: number; z: number }>;
export type PlaneInputWire = Readonly<{ origin: VectorInputWire; xAxis: VectorInputWire; yAxis: VectorInputWire }>;
export type PositionInputWire = Readonly<{ center: CoordinateInputWire; plane: PlaneInputWire }>;
export type OffsetInputWire = Readonly<{ u: number; v: number }>;

function formatVectorInput(v: VectorInputWire): string {
  return `{ x: ${v.x}, y: ${v.y}, z: ${v.z} }`;
}

function formatCoordinateInput(c: CoordinateInputWire): string {
  return `{ u: ${c.u}, v: ${c.v} }`;
}

function formatPlaneInput(p: PlaneInputWire): string {
  return `{ origin: ${formatVectorInput(p.origin)}, xAxis: ${formatVectorInput(p.xAxis)}, yAxis: ${formatVectorInput(p.yAxis)} }`;
}

function formatPositionInput(p: PositionInputWire): string {
  return `{ center: ${formatCoordinateInput(p.center)}, plane: ${formatPlaneInput(p.plane)} }`;
}

function formatOffsetInput(o: OffsetInputWire): string {
  return `{ u: ${o.u}, v: ${o.v} }`;
}

//#region ­ƒº¼Entity
//#region ­ƒøá´©ÅBase
/** @emoji ­ƒº¼ Strong entity anchor: {@link Kit} + id (no cached fields on the instance). */
export abstract class Entity {
  protected constructor(
    public readonly kit: Kit,
    public readonly id: string,
  ) {}
}
//#endregion ­ƒøá´©ÅBase

//#region ­ƒÅ¡Factories
export type FieldSpec<T> = Readonly<{
  eventKind?: string;
  selection: string;
  parse: (v: JsonValue) => T;
}>;

export type OperationSpec = Readonly<{
  alias: string;
  call: string;
}>;

/** @emoji ­ƒÅ¡ Metadata-only field list (tooling / docs); reads use entity methods. */
export function defineFields<const S extends readonly FieldSpec<unknown>[]>(specs: S): S {
  return specs;
}

/** @emoji ­ƒÅ¡ Metadata-only operation list (tooling / docs); writes use entity methods. */
export function defineOperations<const S extends readonly OperationSpec[]>(specs: S): S {
  return specs;
}

/** @emoji ­ƒÅ¡ Wire a field read when the caller supplies the kit-relative GraphQL tail. */
export function defineField<E extends Entity, T>(entity: E, spec: FieldSpec<T>, pathInKit: (self: E) => string): () => Promise<T> {
  return async () => {
    const frag = await entity.kit.readKitInner(pathInKit(entity));
    return spec.parse(frag as JsonValue);
  };
}

/** @emoji ­ƒÅ¡ Wire a mutation leaf using {@link Kit#mutateScoped}. */
export function defineOperation(entity: Entity, spec: OperationSpec, buildPath: (self: Entity) => string): () => Promise<SetResult> {
  return async () => {
    void spec;
    const cid = await entity.kit.ensureChangeId();
    return entity.kit.mutateScoped(cid, buildPath(entity));
  };
}
//#endregion ­ƒÅ¡Factories
//#endregion ­ƒº¼Entity

//#region ­ƒôÉDesign
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

  layer(layerId: string): LayerEntity {
    return new LayerEntity(this.kit, this.id, layerId);
  }

  group(groupId: string): GroupEntity {
    return new GroupEntity(this.kit, this.id, groupId);
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

  async addFixedPiece(blueprintId: string, position: PositionInputWire, name?: string | null, description?: string | null): Promise<SetResult> {
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
    position?: PositionInputWire | null,
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
    position: PositionInputWire,
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
//#endregion ­ƒôÉDesign

//#region ­ƒº░Type
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
}
//#endregion ­ƒº░Type

//#region ­ƒöÿPort
export class Port extends Entity {
  readonly typeId: string;
  constructor(kit: Kit, typeId: string, id: string) {
    super(kit, id);
    this.typeId = typeId;
  }

  private psel(inner: string): string {
    return `type(id: ${__gqlStr(this.typeId)}) { port(id: ${__gqlStr(this.id)}) { ${inner} } }`;
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
//#endregion ­ƒöÿPort

//#region ­ƒöùConnector
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
}
//#endregion ­ƒöùConnector

//#region ­ƒº®Piece
export class Piece extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  private psel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { piece(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  async rename(newName: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`rn: rename(newName: ${__gqlStr(newName)})`));
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`cd: changeDescription(newDescription: ${__gqlStr(newDescription)})`));
  }

  async drag(offset: OffsetInputWire): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(position: PositionInputWire): Promise<SetResult> {
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
//#endregion ­ƒº®Piece

//#region ­ƒ¬óPiecesOperations
export class PiecesOperations {
  constructor(
    private readonly kit: Kit,
    private readonly designId: string,
    private readonly pieceIds: readonly string[],
  ) {}

  private psel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { pieces(ids: ${__gqlIds(this.pieceIds)}) { ${inner} } }`;
  }

  async drag(offset: OffsetInputWire): Promise<SetResult> {
    const cid = await this.kit.ensureChangeId();
    return this.kit.mutateScoped(cid, this.psel(`dg: drag(offset: ${formatOffsetInput(offset)})`));
  }

  async move(offset: OffsetInputWire): Promise<SetResult> {
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
//#endregion ­ƒ¬óPiecesOperations

//#region Ôøô´©ÅConnection
export class Connection extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  private csel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { connection(id: ${__gqlStr(this.id)}) { ${inner} } }`;
  }

  async readGap(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("gap"))) as JsonObject | null) ?? null;
    const d = frag?.["design"] as JsonObject | undefined;
    const c = d?.["connection"] as JsonObject | undefined;
    const v = c?.["gap"];
    return typeof v === "number" ? v : null;
  }

  async readShift(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("shift"))) as JsonObject | null) ?? null;
    const d = frag?.["design"] as JsonObject | undefined;
    const c = d?.["connection"] as JsonObject | undefined;
    const v = c?.["shift"];
    return typeof v === "number" ? v : null;
  }

  async readRise(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("rise"))) as JsonObject | null) ?? null;
    const d = frag?.["design"] as JsonObject | undefined;
    const c = d?.["connection"] as JsonObject | undefined;
    const v = c?.["rise"];
    return typeof v === "number" ? v : null;
  }

  async readRotation(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("rotation"))) as JsonObject | null) ?? null;
    const d = frag?.["design"] as JsonObject | undefined;
    const c = d?.["connection"] as JsonObject | undefined;
    const v = c?.["rotation"];
    return typeof v === "number" ? v : null;
  }

  async readTurn(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("turn"))) as JsonObject | null) ?? null;
    const d = frag?.["design"] as JsonObject | undefined;
    const c = d?.["connection"] as JsonObject | undefined;
    const v = c?.["turn"];
    return typeof v === "number" ? v : null;
  }

  async readTilt(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("tilt"))) as JsonObject | null) ?? null;
    const d = frag?.["design"] as JsonObject | undefined;
    const c = d?.["connection"] as JsonObject | undefined;
    const v = c?.["tilt"];
    return typeof v === "number" ? v : null;
  }
}
//#endregion Ôøô´©ÅConnection

//#region Ô£ì´©ÅAuthor
export class Author extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readName(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { name } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["name"] ?? "");
  }

  async readEmail(): Promise<string> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { email } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    return String(n?.["email"] ?? "");
  }

  async readRank(): Promise<number | null> {
    const data = kitGraphqlData(await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Author { rank } } }`, variables: { id: this.id } })) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const r = n?.["rank"];
    return typeof r === "number" ? r : null;
  }
}
//#endregion Ô£ì´©ÅAuthor

//#region ­ƒÆÄQuality
export class Quality extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private qsel(inner: string): string {
    return `quality(id: ${__gqlStr(this.id)}) { ${inner} }`;
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
//#endregion ­ƒÆÄQuality

//#region ­ƒÅÀ´©ÅTag
export class Tag extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private tsel(inner: string): string {
    return `tag(id: ${__gqlStr(this.id)}) { ${inner} }`;
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
//#endregion ­ƒÅÀ´©ÅTag

//#region ­ƒÆíConcept
export class Concept extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  private csel(inner: string): string {
    return `concept(id: ${__gqlStr(this.id)}) { ${inner} }`;
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
//#endregion ­ƒÆíConcept

//#region ­ƒÄ¿Representation
export class Representation extends Entity {
  readonly typeId: string;
  constructor(kit: Kit, typeId: string, id: string) {
    super(kit, id);
    this.typeId = typeId;
  }

  async readFileId(): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Representation { file { id } } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    const n = data["node"] as JsonObject | undefined;
    const f = n?.["file"] as JsonObject | undefined;
    return String(f?.["id"] ?? "");
  }
}
//#endregion ­ƒÄ¿Representation

//#region ­ƒæ¿ÔÇì­ƒæ®ÔÇì­ƒæªFamily
export class Family extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readName(): Promise<string> {
    const frag = await this.kit.readKitInner(`families { edges { node { id name } } }`);
    const edges = (((frag as JsonObject | null)?.["families"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined) ?? [];
    for (const e of edges) {
      const n = e["node"] as JsonObject | undefined;
      if (n && String(n["id"] ?? "") === this.id) return String(n["name"] ?? "");
    }
    return "";
  }
}
//#endregion ­ƒæ¿ÔÇì­ƒæ®ÔÇì­ƒæªFamily

//#region ­ƒôäFile
export class FileEntity extends Entity {
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
//#endregion ­ƒôäFile

//#region ­ƒôüFolder
export class FolderEntity extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readPath(): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Folder { path } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    return String((data["node"] as JsonObject | undefined)?.["path"] ?? "");
  }
}
//#endregion ­ƒôüFolder

//#region ­ƒ¬ƒLayer
export class LayerEntity extends Entity {
  readonly designId: string;
  constructor(kit: Kit, designId: string, id: string) {
    super(kit, id);
    this.designId = designId;
  }

  private lsel(inner: string): string {
    return `design(id: ${__gqlStr(this.designId)}) { layers { edges { node { id ${inner} } } } }`;
  }

  async readName(): Promise<string> {
    const frag = await this.kit.readKitInner(this.lsel("name"));
    const edges = (((frag as JsonObject | null)?.["design"] as JsonObject | undefined)?.["layers"] as JsonObject | undefined)?.["edges"] as readonly JsonObject[] | undefined;
    for (const e of edges ?? []) {
      const n = e["node"] as JsonObject | undefined;
      if (n && String(n["id"] ?? "") === this.id) return String(n["name"] ?? "");
    }
    return "";
  }
}
//#endregion ­ƒ¬ƒLayer

//#region ­ƒæÑGroup
export class GroupEntity extends Entity {
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
//#endregion ­ƒæÑGroup

//#region ­ƒôèStat
export class StatEntity extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readKey(): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Stat { key } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    return String((data["node"] as JsonObject | undefined)?.["key"] ?? "");
  }
}
//#endregion ­ƒôèStat

//#region ­ƒÄÜ´©ÅProp
export class PropEntity extends Entity {
  constructor(kit: Kit, id: string) {
    super(kit, id);
  }

  async readKey(): Promise<string> {
    const data = kitGraphqlData(
      await this.kit.runGraphql({ query: `query($id: ID!) { node(id: $id) { ... on Prop { key } } }`, variables: { id: this.id } }),
    ) as JsonObject;
    return String((data["node"] as JsonObject | undefined)?.["key"] ?? "");
  }
}
//#endregion ­ƒÄÜ´©ÅProp

//#endregion ­ƒº▒Classes

//#region ­ƒ¬ÂWeakEntities
//#region ­ƒôÉPlane
/** @emoji ­ƒôÉ Weak plane value (schema mirror). */
export interface PlaneWire {
  readonly origin: VectorWire;
  readonly xAxis: VectorWire;
  readonly yAxis: VectorWire;
}
//#endregion ­ƒôÉPlane
//#region ­ƒôìCoordinate
export interface CoordinateWire {
  readonly u: number;
  readonly v: number;
}
//#endregion ­ƒôìCoordinate
//#region ­ƒöÁPoint
export interface PointWire {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
//#endregion ­ƒöÁPoint
//#region Ô×í´©ÅVector
export interface VectorWire {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
//#endregion Ô×í´©ÅVector
//#region Ôåö´©ÅSide
export interface SideWire {
  readonly piece: { readonly id: string };
  readonly connector: string;
}
//#endregion Ôåö´©ÅSide
//#region ­ƒôîPosition
export interface PositionWire {
  readonly center: CoordinateWire;
  readonly plane: PlaneWire;
}
//#endregion ­ƒôîPosition
//#region ­ƒîìPlace
export interface PlaceWire {
  readonly location: LocationWire;
}
//#endregion ­ƒîìPlace
//#region ­ƒù║´©ÅLocation
export interface LocationWire {
  readonly latitude: number;
  readonly longitude: number;
}
//#endregion ­ƒù║´©ÅLocation
//#region ­ƒôÀCamera
export interface CameraWire {
  readonly position: PointWire;
  readonly target: PointWire;
}
//#endregion ­ƒôÀCamera
//#region ­ƒÅüBenchmark
export interface BenchmarkWire {
  readonly id: string;
  readonly name: string;
}
//#endregion ­ƒÅüBenchmark
//#region ­ƒ¬¬Attribute
export interface AttributeWire {
  readonly id: string;
  readonly key: string;
  readonly value: string | null;
}
//#endregion ­ƒ¬¬Attribute
//#endregion ­ƒ¬ÂWeakEntities

//#region ­ƒÜÇPublicAPI
/** @emoji ­ƒÜÇ Opens a {@link Kit} backed by rs WASM (worker or inline). */
export async function openKit(seed: KitBootstrapJson, opts?: KitOpenOptions): Promise<Kit> {
  return Kit.open(seed, opts);
}
//#endregion ­ƒÜÇPublicAPI

//#region ­ƒº¬Tests
if (typeof process !== "undefined" && !!process.env && process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  describe("semio/js field-only kit", () => {
    it("source has no banned cache/sync substrings", async () => {
      const fs = await import("node:fs");
      const url = await import("node:url");
      const p = url.fileURLToPath(import.meta.url);
      const text = fs.readFileSync(p, "utf8");
      const marker = "//#region ­ƒº¬Tests";
      const idx = text.indexOf(marker);
      const head = idx < 0 ? text : text.slice(0, idx);
      for (const ban of ["applyToCache", "dispatchSync", "fieldSync", "KitStoreSnapshot", "KitHostStore", "optimistic", "reconcil"] as const) {
        expect(head.includes(ban), ban).toBe(false);
      }
    });

    it("Piece.drag issues one mutateScoped path (stub kit)", async () => {
      const calls: string[] = [];
      const k = Object.create(Kit.prototype) as Kit;
      (k as unknown as { ensureAlive(): void }).ensureAlive = () => {};
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
//#endregion ­ƒº¬Tests
