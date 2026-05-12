// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: stateless {@link Kit} + GraphQL transport (WASM worker or inline); no client-side kit cache.
// #endregion 🧲Header

//#region 📥WasmKitImports
import { BehaviorSubject, Subject, filter } from "rxjs";
import { z } from "zod";
//#endregion 📥WasmKitImports

//#region 🌐Transport
/** @emoji 🧵 Bundled worker — Vite resolves `@semio/rs-wasm`; Blob workers cannot import bare specifiers. */
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
    if (parts.length) return parts.join(" — ");
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

/** @emoji 🌐 Thin GraphQL wire: JSON request in, JSON string out; pairs with rs {@code KitStoreHandle}. */
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

/** @emoji 📡 Demultiplexes {@code Subscription.event} JSON into listener fan-out (no client cache). */
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

/** @emoji 🧾 JSON seed passed to rs to spawn the wip overlay (GraphQL authority). */
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

//#region 🧱Classes
//#region 🎒Kit
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
  private readonly innerTransport: WorkerStringTransport | InlineWasmTransport;
  private gqlLoopRunning = false;
  private disposed = false;
  private activeReadPoint: KitReadPoint = theKitReadPoint;
  private kitWriteChangeId: string | null = null;

  /** @emoji 🌐 GraphQL executor (JSON wire). */
  readonly gql: GqlTransport;
  /** @emoji 📡 Demuxed subscription fan-out. */
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

  /** @emoji 🌐 Public GraphQL round-trip (root {@code Query} / {@code Mutation}), for {@code node(id:)} reads. */
  async runGraphql(body: { query: string; variables?: GraphQlVariables; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    return this.gqlRun(body);
  }

  /** @emoji 🧾 Reads a selection inside scoped {@code kit { … }} for {@link activeReadPoint}. */
  async readKitInner(inner: string, variables: GraphQlVariables = {}): Promise<JsonObject | null> {
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

  async hydrateKitStoreBundleJson(json: string): Promise<SetResult> {
    this.ensureAlive();
    const env = await this.gqlRun({ query: `mutation { hydrateKitStoreBundleJson(json: ${__gqlStr(json)}) }` });
    return gqlOkFromEnvelope(env);
  }

  /** @emoji 🧾 Warm-path query after WASM init. */
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

//#region 🧬Entity
//#region 🛠️Base
/** @emoji 🧬 Strong entity anchor: {@link Kit} + id (no cached fields on the instance). */
export abstract class Entity {
  protected constructor(
    public readonly kit: Kit,
    public readonly id: string,
  ) {}
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

/** @emoji 🏭 Wire a field read when the caller supplies the kit-relative GraphQL tail. */
export function defineField<E extends Entity, T>(entity: E, spec: FieldSpec<T>, pathInKit: (self: E) => string): () => Promise<T> {
  return async () => {
    const frag = await entity.kit.readKitInner(pathInKit(entity));
    return spec.parse(frag as JsonValue);
  };
}

/** @emoji 🏭 Wire a mutation leaf using {@link Kit#mutateScoped}. */
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

//#region 🧩WireParsers
/** @emoji 🧩 Parses {@code attributes { edges { node { … } } }} under a JSON object (e.g. {@code tag}, {@code node}). */
function parseAttributeConnectionUnder(owner: JsonObject | null | undefined): readonly AttributeWire[] {
  const attrs = owner?.["attributes"] as JsonObject | undefined;
  const edges = attrs?.["edges"] as readonly JsonValue[] | undefined;
  if (!Array.isArray(edges)) return [];
  const out: AttributeWire[] = [];
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
//#endregion 🧩WireParsers
//#endregion 🧬Entity

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

  layer(layerId: string): LayerEntity {
    return new LayerEntity(this.kit, this.id, layerId);
  }

  group(groupId: string): GroupEntity {
    return new GroupEntity(this.kit, this.id, groupId);
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
   * @emoji 📡 When {@link FieldSpec#eventKind} matches rs {@code Subscription.event} kinds, refetches via {@link Design#fieldRead}.
   */
  subscribeField<T>(spec: FieldSpec<T>, cb: (next: T) => void): Unsubscribe {
    const kind = spec.eventKind;
    if (kind == null || kind === "") return () => {};
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
  async readAttributes(): Promise<readonly AttributeWire[]> {
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
  async readAttributes(): Promise<readonly AttributeWire[]> {
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
  async readAttributes(): Promise<readonly AttributeWire[]> {
    const inner = "attributes { edges { node { id key value definition } } }";
    const frag = (await this.kit.readKitInner(this.csel(inner))) as JsonObject | null;
    return parseAttributeConnectionUnder(this.connectorNode(frag));
  }
}
//#endregion 🔗Connector

//#region 🧩Piece
/** @emoji 🧩 @description Blueprint target on a {@link Piece} (`Type` or `Design` node). */
export interface PieceBlueprintWire {
  readonly blueprintKind: "Type" | "Design";
  readonly id: string;
}

function __pieceKitRow(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const p = d?.["piece"] as JsonObject | undefined;
  return p ?? null;
}

function __parseCoordinateFromJson(node: JsonObject | null | undefined): CoordinateWire | null {
  if (node == null || typeof node !== "object") return null;
  const u = node["u"];
  const v = node["v"];
  if (typeof u !== "number" || typeof v !== "number") return null;
  return { u, v };
}

function __parsePointFromJson(node: JsonObject | null | undefined): PointWire | null {
  if (node == null || typeof node !== "object") return null;
  const x = node["x"];
  const y = node["y"];
  const z = node["z"];
  if (typeof x !== "number" || typeof y !== "number" || typeof z !== "number") return null;
  return { x, y, z };
}

function __parseVectorFromJson(node: JsonObject | null | undefined): VectorWire | null {
  return __parsePointFromJson(node);
}

function __parsePlaneFromJson(node: JsonObject | null | undefined): PlaneWire | null {
  if (node == null || typeof node !== "object") return null;
  const origin = __parsePointFromJson(node["origin"] as JsonObject | undefined);
  const xAxis = __parseVectorFromJson(node["xAxis"] as JsonObject | undefined);
  const yAxis = __parseVectorFromJson(node["yAxis"] as JsonObject | undefined);
  if (origin == null || xAxis == null || yAxis == null) return null;
  return { origin, xAxis, yAxis };
}

function __parsePositionFromJson(node: JsonObject | null | undefined): PositionWire | null {
  if (node == null || typeof node !== "object") return null;
  const center = __parseCoordinateFromJson(node["center"] as JsonObject | undefined);
  const plane = __parsePlaneFromJson(node["plane"] as JsonObject | undefined);
  if (center == null || plane == null) return null;
  return { center, plane };
}

function __parsePieceBlueprintFromJson(node: JsonObject | null | undefined): PieceBlueprintWire | null {
  if (node == null || typeof node !== "object") return null;
  const tn = String(node["__typename"] ?? "");
  const id = String(node["id"] ?? "");
  if (id === "") return null;
  if (tn === "Type") return { blueprintKind: "Type", id };
  if (tn === "Design") return { blueprintKind: "Design", id };
  return null;
}

function __parseAttributeNodesFromConnection(obj: JsonObject | null | undefined): readonly AttributeWire[] {
  const attrs = obj?.["attributes"] as JsonObject | undefined;
  const edges = attrs?.["edges"];
  if (!Array.isArray(edges)) return [];
  const out: AttributeWire[] = [];
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
    return String(__pieceKitRow(frag)?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("description"))) as JsonObject | null;
    return String(__pieceKitRow(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = (await this.kit.readKitInner(this.psel("icon"))) as JsonObject | null;
    return String(__pieceKitRow(frag)?.["icon"] ?? "");
  }

  async readScale(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.psel("scale"))) as JsonObject | null;
    const v = __pieceKitRow(frag)?.["scale"];
    return typeof v === "number" ? v : null;
  }

  async readPosition(): Promise<PositionWire | null> {
    const frag = (await this.kit.readKitInner(this.psel(`position { ${__PIECE_POSITION_SUBSELECTION} }`))) as JsonObject | null;
    return __parsePositionFromJson(__pieceKitRow(frag)?.["position"] as JsonObject | undefined);
  }

  async readFlatPosition(): Promise<PositionWire | null> {
    const frag = (await this.kit.readKitInner(this.psel(`flatPosition { ${__PIECE_POSITION_SUBSELECTION} }`))) as JsonObject | null;
    return __parsePositionFromJson(__pieceKitRow(frag)?.["flatPosition"] as JsonObject | undefined);
  }

  async readPlane(): Promise<PlaneWire | null> {
    return (await this.readPosition())?.plane ?? null;
  }

  async readCenter(): Promise<CoordinateWire | null> {
    return (await this.readPosition())?.center ?? null;
  }

  async readFlatPlane(): Promise<PlaneWire | null> {
    return (await this.readFlatPosition())?.plane ?? null;
  }

  async readFlatCenter(): Promise<CoordinateWire | null> {
    return (await this.readFlatPosition())?.center ?? null;
  }

  async readBlueprint(): Promise<PieceBlueprintWire | null> {
    const frag = (await this.kit.readKitInner(this.psel("blueprint { __typename id }"))) as JsonObject | null;
    return __parsePieceBlueprintFromJson(__pieceKitRow(frag)?.["blueprint"] as JsonObject | undefined);
  }

  async readAttributes(): Promise<readonly AttributeWire[]> {
    const frag = (await this.kit.readKitInner(this.psel("attributes { edges { node { id key value definition } } }"))) as JsonObject | null;
    return __parseAttributeNodesFromConnection(__pieceKitRow(frag));
  }

  async readConnectionKind(): Promise<"FIXED" | "CONNECTED" | null> {
    const frag = (await this.kit.readKitInner(this.psel("connectionKind"))) as JsonObject | null;
    const k = __pieceKitRow(frag)?.["connectionKind"];
    if (k === "FIXED" || k === "CONNECTED") return k;
    return null;
  }

  async readParentPieceId(): Promise<string | null> {
    const frag = (await this.kit.readKitInner(this.psel("parentPiece { id }"))) as JsonObject | null;
    const n = __pieceKitRow(frag)?.["parentPiece"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  async readParentConnectionId(): Promise<string | null> {
    const frag = (await this.kit.readKitInner(this.psel("parentConnection { id }"))) as JsonObject | null;
    const n = __pieceKitRow(frag)?.["parentConnection"] as JsonObject | undefined;
    const id = n == null ? "" : String(n["id"] ?? "");
    return id === "" ? null : id;
  }

  async readChildPieceIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.psel("childPieces { edges { node { id } } }"))) as JsonObject | null;
    return __parseIdListConnection(__pieceKitRow(frag), "childPieces");
  }

  async readChildConnectionIds(): Promise<readonly string[]> {
    const frag = (await this.kit.readKitInner(this.psel("childConnections { edges { node { id } } }"))) as JsonObject | null;
    return __parseIdListConnection(__pieceKitRow(frag), "childConnections");
  }

  async readDepth(): Promise<number | null> {
    const frag = (await this.kit.readKitInner(this.psel("depth"))) as JsonObject | null;
    const v = __pieceKitRow(frag)?.["depth"];
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
//#endregion 🧩Piece

//#region 🪢PiecesOperations
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
//#endregion 🪢PiecesOperations

//#region ⛓️Connection
/** @emoji ⛓️ @description Schema-aligned {@link Connection} side (piece + optional port / connector / designPiece ids). */
export interface ConnectionSideWire {
  readonly pieceId: string;
  readonly portId: string | null;
  readonly connectorId: string | null;
  readonly designPieceId: string | null;
}

const __CONNECTION_SIDE_SUBSELECTION = "piece { id } port { id } designPiece { id } connector { id }";

function __connectionKitRow(frag: JsonObject | null | undefined): JsonObject | null {
  const d = frag?.["design"] as JsonObject | undefined;
  const c = d?.["connection"] as JsonObject | undefined;
  return c ?? null;
}

function __parseConnectionSideFromJson(node: JsonObject | null | undefined): ConnectionSideWire | null {
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
    return String(__connectionKitRow(frag)?.["name"] ?? "");
  }

  async readDescription(): Promise<string> {
    const frag = ((await this.kit.readKitInner(this.csel("description"))) as JsonObject | null) ?? null;
    return String(__connectionKitRow(frag)?.["description"] ?? "");
  }

  async readIcon(): Promise<string> {
    const frag = ((await this.kit.readKitInner(this.csel("icon"))) as JsonObject | null) ?? null;
    return String(__connectionKitRow(frag)?.["icon"] ?? "");
  }

  async readGap(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("gap"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["gap"];
    return typeof v === "number" ? v : null;
  }

  async readShift(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("shift"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["shift"];
    return typeof v === "number" ? v : null;
  }

  async readRise(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("rise"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["rise"];
    return typeof v === "number" ? v : null;
  }

  async readRotation(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("rotation"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["rotation"];
    return typeof v === "number" ? v : null;
  }

  async readTurn(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("turn"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["turn"];
    return typeof v === "number" ? v : null;
  }

  async readTilt(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("tilt"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["tilt"];
    return typeof v === "number" ? v : null;
  }

  async readU(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("u"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["u"];
    return typeof v === "number" ? v : null;
  }

  async readV(): Promise<number | null> {
    const frag = ((await this.kit.readKitInner(this.csel("v"))) as JsonObject | null) ?? null;
    const v = __connectionKitRow(frag)?.["v"];
    return typeof v === "number" ? v : null;
  }

  async readConnected(): Promise<ConnectionSideWire | null> {
    const frag = ((await this.kit.readKitInner(this.csel(`connected { ${__CONNECTION_SIDE_SUBSELECTION} }`))) as JsonObject | null) ?? null;
    return __parseConnectionSideFromJson(__connectionKitRow(frag)?.["connected"] as JsonObject | undefined);
  }

  async readConnecting(): Promise<ConnectionSideWire | null> {
    const frag = ((await this.kit.readKitInner(this.csel(`connecting { ${__CONNECTION_SIDE_SUBSELECTION} }`))) as JsonObject | null) ?? null;
    return __parseConnectionSideFromJson(__connectionKitRow(frag)?.["connecting"] as JsonObject | undefined);
  }

  async readAttributes(): Promise<readonly AttributeWire[]> {
    const frag = ((await this.kit.readKitInner(this.csel("attributes { edges { node { id key value definition } } }"))) as JsonObject | null) ?? null;
    return __parseAttributeNodesFromConnection(__connectionKitRow(frag));
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

  async readAttributes(): Promise<readonly AttributeWire[]> {
    const frag = (await this.kit.readKitInner(this.qsel(`attributes { edges { node { id key value definition } } }`))) as JsonObject | null;
    return parseAttributeConnectionUnder(frag?.["quality"] as JsonObject | undefined);
  }

  async readBenchmarks(): Promise<readonly BenchmarkWire[]> {
    const frag = (await this.kit.readKitInner(
      this.qsel(`benchmarks { edges { node { id name min max minExcluded maxExcluded } } }`),
    )) as JsonObject | null;
    const q = frag?.["quality"] as JsonObject | undefined;
    const bench = q?.["benchmarks"] as JsonObject | undefined;
    const edges = bench?.["edges"] as readonly JsonValue[] | undefined;
    if (!Array.isArray(edges)) return [];
    const out: BenchmarkWire[] = [];
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

  async readAttributes(): Promise<readonly AttributeWire[]> {
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

  async readAttributes(): Promise<readonly AttributeWire[]> {
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

  async readAttributes(): Promise<readonly AttributeWire[]> {
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
//#endregion 📄File

//#region 📁Folder
/** @emoji 📁 Folder artifact: read-only in current kit API. */
export class FolderEntity extends Entity {
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
export class LayerEntity extends Entity {
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
//#endregion 👥Group

//#region 📊Stat
/** @emoji 📊 Stat artifact: read-only in current kit API. */
export class StatEntity extends Entity {
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
export class PropEntity extends Entity {
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
export interface PlaneWire {
  readonly origin: VectorWire;
  readonly xAxis: VectorWire;
  readonly yAxis: VectorWire;
}
//#endregion 📐Plane
//#region 📍Coordinate
export interface CoordinateWire {
  readonly u: number;
  readonly v: number;
}
//#endregion 📍Coordinate
//#region 🔵Point
export interface PointWire {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
//#endregion 🔵Point
//#region ➡️Vector
export interface VectorWire {
  readonly x: number;
  readonly y: number;
  readonly z: number;
}
//#endregion ➡️Vector
//#region ↔️Side
export interface SideWire {
  readonly piece: { readonly id: string };
  readonly connector: string;
}
//#endregion ↔️Side
//#region 📌Position
export interface PositionWire {
  readonly center: CoordinateWire;
  readonly plane: PlaneWire;
}
//#endregion 📌Position
//#region 🌍Place
export interface PlaceWire {
  readonly location: LocationWire;
}
//#endregion 🌍Place
//#region 🗺️Location
export interface LocationWire {
  readonly latitude: number;
  readonly longitude: number;
}
//#endregion 🗺️Location
//#region 📷Camera
export interface CameraWire {
  readonly position: PointWire;
  readonly target: PointWire;
}
//#endregion 📷Camera
//#region 🏁Benchmark
/** @emoji 🏁 Benchmark row subset from {@code Benchmark} (owner: Quality). */
export interface BenchmarkWire {
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
export interface AttributeWire {
  readonly id: string;
  readonly key: string;
  readonly value: string | null;
  readonly definition: string;
}
//#endregion 🪪Attribute
//#endregion 🪶WeakEntities

//#region 🚀PublicAPI
/** @emoji 🚀 Opens a {@link Kit} backed by rs WASM (worker or inline). */
export async function openKit(seed: KitBootstrapJson, opts?: KitOpenOptions): Promise<Kit> {
  return Kit.open(seed, opts);
}
//#endregion 🚀PublicAPI

//#region 🧷KitWasmHost
/** @emoji 🧷 Wasm kit graph + KitStore (nested under {@linkcode WasmGraph}). */
export namespace WasmGraph {
// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: `KitStore` + opaque dto batches to `semio/rs` WASM (read shapes are not re-exported).
// #endregion 🧲Header

// #region 📥Imports
// #endregion 📥Imports

// #region 🧵InlineWorker

/** @emoji 🧵 Bundled worker chunk — Vite resolves `@semio/rs-wasm`; Blob workers cannot import bare specifiers. */
function createKitStoreWorker(): Worker {
  return new Worker(new URL("./kit-store.worker.ts", import.meta.url), { type: "module" });
}

// #endregion 🧵InlineWorker
// #region 🔌JsonGraphQlDtoTypes

/** @emoji 🪪 Correlates kit command lifecycle events on the dto. */
export type KitCommandRequestId = string;

export type SetErrorKind = "IllegalName" | "NameTooLong" | "InvalidUrl" | "InvalidValue" | "DuplicateId" | "NotFound" | "CyclicReference" | "PortFamilyMismatch" | "Readonly" | "Disposed" | "Timeout" | "LockPoisoned" | "Internal" | "NotSupported";

/** @emoji 🧾 Normalized set/mutation error from Rust `SetError`. */
export type SetError = { kind: SetErrorKind; message: string; field?: string; entity?: { kind: string; id: string } };

export type SetResult = { ok: true; requestId?: KitCommandRequestId } | { ok: false; error: SetError; requestId?: KitCommandRequestId };

export type KitCommandLifecyclePhase = "accepted" | "succeeded" | "failed";

/**
 * @emoji 🧾 `KitChangeKind` on the dto (camelCase from `semio/rs`), plus any `other` label inside `other`.
 */
export type KitChangeKind =
  | "inferred"
  | "setKitMetadata"
  | "addType"
  | "removeType"
  | "modifyType"
  | "addDesign"
  | "removeDesign"
  | "modifyDesign"
  | "addPiece"
  | "removePiece"
  | "connect"
  | "disconnect"
  | "unifyCheckpoints"
  | "markRelease"
  | { readonly other: string }
  | (string & { readonly _semioExt?: 1 });

/** @emoji 🧾 GraphQL `KitChangeSemanticKind` enum (SCREAMING_SNAKE); pair with {@linkcode KitChangeKind} via {@linkcode kitChangeSemanticKindToGraphQl}. */
export type KitChangeSemanticKindGql =
  | "INFERRED"
  | "SET_KIT_METADATA"
  | "ADD_TYPE"
  | "REMOVE_TYPE"
  | "MODIFY_TYPE"
  | "ADD_DESIGN"
  | "REMOVE_DESIGN"
  | "MODIFY_DESIGN"
  | "ADD_PIECE"
  | "REMOVE_PIECE"
  | "CONNECT"
  | "DISCONNECT"
  | "UNIFY_CHECKPOINTS"
  | "MARK_RELEASE"
  | "OTHER";

/** @emoji 🧾 Maps batch `changeKind` + `changeKindOther` into {@linkcode KitChangeKind} (camelCase unit or `{ other }` for extension labels). */
export function kitChangeSemanticKindToGraphQl(gql: KitChangeSemanticKindGql | null | undefined, other: string | null | undefined): KitChangeKind {
  if (gql === "OTHER" || gql == null) {
    if (other != null && other.length > 0) return { other } as const;
    return "inferred";
  }
  const m: { readonly [K in Exclude<KitChangeSemanticKindGql, "OTHER">]: KitChangeKind } = {
    INFERRED: "inferred",
    SET_KIT_METADATA: "setKitMetadata",
    ADD_TYPE: "addType",
    REMOVE_TYPE: "removeType",
    MODIFY_TYPE: "modifyType",
    ADD_DESIGN: "addDesign",
    REMOVE_DESIGN: "removeDesign",
    MODIFY_DESIGN: "modifyDesign",
    ADD_PIECE: "addPiece",
    REMOVE_PIECE: "removePiece",
    CONNECT: "connect",
    DISCONNECT: "disconnect",
    UNIFY_CHECKPOINTS: "unifyCheckpoints",
    MARK_RELEASE: "markRelease",
  };
  return m[gql as Exclude<KitChangeSemanticKindGql, "OTHER">] ?? "inferred";
}

/** @emoji 🪢 Object branch for GraphQL / serde JSON trees (explicit string slots, no `Record` alias). */
export type KitJsonObjectDto = { readonly [slot: string]: KitJsonTreeDto };
/** @emoji 🪢 Recursive JSON tree from GraphQL / serde kit scalars. */
export type KitJsonTreeDto = string | number | boolean | null | readonly KitJsonTreeDto[] | KitJsonObjectDto;

/** @emoji 🧱 Parsed JSON tree (strict surface; no open index-signature object typing or untyped values). */
export type JsonValue = string | number | boolean | null | readonly JsonValue[] | JsonObject;
/** @emoji 🧱 JSON object node (string-keyed). */
export type JsonObject = { readonly [key: string]: JsonValue };

/** @emoji 🔒 Recursive readonly DTO view for Zod-inferred and GraphQL-shaped values. */
export type ReadonlyDto<T> = T extends ReadonlyArray<infer U> ? ReadonlyArray<ReadonlyDto<U>> : T extends object ? { readonly [K in keyof T]: ReadonlyDto<T[K]> } : T;

/** @emoji 🧱 Mutable JSON object for local construction. */
type JsonObjectMutable = { [key: string]: JsonValue };
/** @emoji 🧱 Mutable `variables` map for GraphQL / kit batch (alias of the JSON builder surface). */
type GraphQlObjectMutable = JsonObjectMutable;

/** @emoji 🧾 `JSON.parse` with a {@link JsonValue} root (GraphQL transport, config knobs). */
function parseJsonValue(text: string): JsonValue {
  return JSON.parse(text) as JsonValue;
}

/** @emoji 🧾 GraphQL HTTP/FFI response envelope before unwrapping `data`. */
type KitGraphqlResponseEnvelope<TData> = Readonly<{
  data?: TData | null;
  errors?: readonly { readonly message?: string }[];
}>;

/**
 * @emoji 🧾 Tags accepted on `KitStore` control-plane nested `Mutation.session` mappers and {@link WasmKitStoreClient} routing.
 * @public
 */
export const SEMIO_KIT_STORE_CONTROL_COMMAND_KINDS = {
  changeKitCommands: 1,
  changeKitWithInverse: 1,
  undo: 1,
  redo: 1,
  clusterPieces: 1,
  dragPieces: 1,
  movePieces: 1,
  fixPieces: 1,
  flattenDesign: 1,
  expandDesign: 1,
  deleteConnection: 1,
  changePieceType: 1,
  createHangingPieces: 1,
  createConnectedPiece: 1,
  createFixedPiece: 1,
  pasteDesignSelection: 1,
  listConflicts: 1,
  backboneStatus: 1,
  attachBackbone: 1,
  detachBackbone: 1,
  resolveConflict: 1,
  syncNow: 1,
} as const;
export type SemioKitStoreControlCommandName = keyof typeof SEMIO_KIT_STORE_CONTROL_COMMAND_KINDS;

export type KitCommandLifecycleEvent = {
  semioKitCommand: {
    requestId: KitCommandRequestId;
    commandKind: SemioKitStoreControlCommandName | (string & { readonly _semioStoreControlLabel?: 1 });
    phase: KitCommandLifecyclePhase;
    result?: KitJsonTreeDto;
    error?: SetError;
  };
};

/** @emoji 🧭 Backbone / conflict dto shapes (serde-tagged, matches `kit_backbone_dto` in `semio/rs`). */
export type BackboneConfig = { readonly Memory: null } | { readonly Dev: { readonly path: string } } | { readonly Local: { readonly folder: string } } | { readonly Remote: { readonly url: string; readonly sessionId: string } };
export type BackboneStatusDto = {
  readonly attached: boolean;
  readonly kind?: string | null;
  readonly backboneTip?: string | null;
  readonly pendingWipCheckpoints: number;
};
/** @emoji 🧾 GraphQL `ConflictResolutionBatchInput` (matches `semio/graphql/schema.graphql`). */
export type ConflictResolution = "DROP_WIP" | "FORCE_OVERWRITE_BACKBONE";
export type KitCheckpointDto = KitJsonObjectDto;
export type KitConflict = {
  id: string;
  wipCheckpoint: KitCheckpointDto;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
};

/** @emoji 🧾 One read command in a `KitStore.read` batch (matches `semio/rs` read kit dto, serde camelCase). */
export type ReadPieceCommand = { readonly readPieceFlatPlaneCommand: null } | { readonly readPieceFlatCenterCommand: null } | { readonly readPieceParentConnectionFullCommand: null };

export type ReadDesignCommand =
  | { readonly readDesignPiecesFullCommand: null }
  | { readonly readDesignConnectionsFullCommand: null }
  | { readonly readDesignPieceCommands: { readonly id: KitIdDto; readonly commands: ReadonlyArray<ReadPieceCommand> } }
  | { readonly readDesignClusterableGroupsCommand: { readonly selection: ReadonlyArray<KitIdDto> } }
  | { readonly readDesignIncludedDesignsCommand: null }
  | { readonly readDesignQualitySumCommand: { readonly qualityId: KitIdDto } }
  | { readonly readDesignReplaceableCatalogCommand: { readonly selection: ReadonlyArray<KitIdDto> } }
  | { readonly readDesignIncludedDesignIdsCommand: null };

export type ReadTypeCommand = { readonly readTypeBestRepresentationCommand: { readonly tagIds: ReadonlyArray<string> } };

export type ReadKitCommand =
  | { readonly readKitFullCommand: null }
  | { readonly readKitShallowCommand: null }
  | { readonly readKitMetadataCommand: null }
  | { readonly readKitTypeIdsCommand: null }
  | { readonly readKitDesignIdsCommand: null }
  | { readonly readKitTypesMetadataCommand: null }
  | { readonly readKitDesignsMetadataCommand: null }
  | { readonly readKitTypesShallowCommand: null }
  | { readonly readKitDesignsShallowCommand: null }
  | { readonly readKitAuthorsShallowCommand: null }
  | { readonly readKitDesignCommands: { readonly id: KitIdDto; readonly commands: ReadonlyArray<ReadDesignCommand> } }
  | { readonly readKitTypeCommands: { readonly id: KitIdDto; readonly commands: ReadonlyArray<ReadTypeCommand> } };

/**
 * @emoji 🧭 Materialization anchor for target-schema versions (`wip.theKit.kit`, checkpoints, or alternatives).
 */
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

/** @emoji 🧭 Main committed kit line (default read point). */
export const theKitReadPoint: KitReadPoint = { theKit: null };

/** @emoji 🧭 Maps {@link KitReadPoint} to GraphQL `KitReadPointInput` (flat camelCase fields). */
export function kitReadPointToGqlVariables(p: KitReadPoint): JsonObject {
  if ("theKit" in p && p.theKit === null) {
    return { theKit: true } as JsonObject;
  }
  if ("checkpoint" in p) {
    const c = p.checkpoint;
    const o: { [k: string]: JsonValue } = { checkpointId: c.checkpointId };
    if (c.changeId != null && c.changeId !== "") o["checkpointChangeId"] = c.changeId;
    if (c.operationId != null && c.operationId !== "") o["checkpointOperationId"] = c.operationId;
    return o as JsonObject;
  }
  if ("alternative" in p) {
    return { alternativeId: p.alternative.alternativeId } as JsonObject;
  }
  return {} as JsonObject;
}

/** @emoji 🧭 Read point used for scoped kit reads on a {@link KitStoreClient}. */
export function getKitClientReadPoint(client: { readonly kitReadPoint?: KitReadPoint }): KitReadPoint {
  return client.kitReadPoint ?? theKitReadPoint;
}

/** @emoji 🧪 Stable string for cache keys (JSON of the point branch). */
export function kitReadPointKey(point: KitReadPoint): string {
  return JSON.stringify(point);
}

function isTheKitReadPoint(s: KitReadPoint): boolean {
  return "theKit" in s;
}

// #region 🔖KitWriteScope
/** @emoji 🧭 Target-schema unsaved **Change** id (`VersionCommandInput.unsavedChange(id)`). */
export type ChangeId = string;

/** @emoji 🧭 Active unsaved change anchor for `Mutation.session { theKit { unsavedChange(id:) { kit { … } } } }`. */
export type KitWriteScope = { readonly changeId: ChangeId };

/**
 * @emoji 🧾 VCS helpers aligned with `SessionCommandInput` / `VersionCommandInput` (`semio/graphql/target.schema.graphql`).
 * @public
 */
export interface ChangeLifecycle {
  startNewChange(): Promise<ChangeId>;
  saveChange(changeId?: ChangeId): Promise<string>;
  createCheckpoint(message: string): Promise<string>;
  startAlternative(name?: string): Promise<string>;
  integrateAlternative(alternativeId: string): Promise<string>;
  login(username: string, passwordHash: string, hubUrl?: string): Promise<string>;
  logout(): Promise<string>;
}

function __normKitStoreBatchKind(k: JsonValue | undefined): string {
  const s = k === null || k === undefined ? "" : typeof k === "string" ? k : typeof k === "number" || typeof k === "boolean" || typeof k === "bigint" ? String(k) : "UNKNOWN";
  return s
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .replace(/[\s-]+/g, "_")
    .toUpperCase();
}

function __coerceAxisComponent(v: KitJsonTreeDto | undefined): number {
  if (v === undefined) return Number.NaN;
  if (typeof v === "number" && !Number.isNaN(v)) return v;
  if (typeof v === "string") return Number(v);
  return Number.NaN;
}

function __vec3(obj: KitJsonTreeDto | null | undefined): { x: number; y: number; z: number } | null {
  if (!isJsonObjectNode(obj)) return null;
  const x = __coerceAxisComponent(obj["x"]);
  const y = __coerceAxisComponent(obj["y"]);
  const z = __coerceAxisComponent(obj["z"]);
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) return null;
  return { x, y, z };
}

/** @emoji 🧾 Maps a loose plane DTO into GraphQL `PlaneInputBatch` (camelCase axes). */
function __kitPlaneToBatchInput(plane: KitJsonTreeDto | null | undefined): { origin: { x: number; y: number; z: number }; xAxis: { x: number; y: number; z: number }; yAxis: { x: number; y: number; z: number } } | null {
  if (!isJsonObjectNode(plane)) return null;
  const p = plane as KitJsonObjectDto;
  const origin = p["origin"] ?? p["Origin"];
  const xa = p["xAxis"] ?? p["x_axis"] ?? p["XAxis"];
  const ya = p["yAxis"] ?? p["y_axis"] ?? p["YAxis"];
  const o = __vec3(origin);
  const xAxis = __vec3(xa);
  const yAxis = __vec3(ya);
  if (!o || !xAxis || !yAxis) return null;
  return { origin: o, xAxis, yAxis };
}

// #endregion 🔖KitWriteScope

// #region 🔖ReadBatchAndKitRead
/** @emoji 🧾 One `design.flattenMap` row (`DesignFlattenMapEntryObject`). */
export type DesignFlattenMapEntryDto = Readonly<{
  readonly pieceId: string;
  readonly plane: PlaneDto;
  readonly center: PointDto;
}>;

/** @emoji 🧾 Per-piece hierarchy + flat pose row from `design.pieces` (`PieceStore` GraphQL fields). */
export type PiecePlacementRowDto = Readonly<{
  readonly pieceId: string;
  readonly plane: PlaneDto;
  readonly center: CoordinateDto;
  readonly fixedPieceId: string;
  readonly parentPieceId: string | null;
  readonly depth: number;
  readonly path: readonly string[];
}>;

/** @emoji 🧾 Connector color row from the live kit read model. */
export type KitColoredConnectorRowDto = Readonly<{
  readonly typeId: TypeIdDto;
  readonly connectorId: ConnectorIdDto;
  readonly color: string;
}>;

/** @emoji 🧾 One `design.includedDesigns` entry (`IncludedDesignObject`). */
export type IncludedDesignInfoDto = Readonly<{
  readonly id: string;
  readonly designId: string;
  readonly connectionKind: string;
  readonly center: PointDto | null;
  readonly plane: PlaneDto | null;
  readonly externalConnections?: readonly ConnectionDto[];
}>;

/** @emoji 🧾 `KitMetadataObject` root fields from GraphQL. */
export type KitMetadataDto = Readonly<{
  readonly id: string;
  readonly name: string;
  readonly description?: string | null;
  readonly icon?: string | null;
  readonly image?: string | null;
  readonly preview?: string | null;
  readonly remote?: string | null;
  readonly homepage?: string | null;
  readonly license?: string | null;
  readonly uri?: string | null;
  readonly created?: string | null;
  readonly updated?: string | null;
  readonly version?: string | null;
}>;
// #endregion 🔖ReadBatchAndKitRead

/** @emoji 🧾 Batch input for {@link KitStore.read} (per-command, same for all entries in a batch). */
export type ReadBatch = readonly ReadKitCommand[];

/** @emoji 🧾 One entry in a {@link ReadBatch} (alias for consumers that say “read dto item”). */
export type ReadBatchItem = ReadKitCommand;

export type ReadPieceCommandOutput =
  | { readonly readPieceFlatPlaneCommand: { readonly flatPlane: PlaneDto | null } }
  | { readonly readPieceFlatCenterCommand: { readonly flatCenter: CoordinateDto | null } }
  | { readonly readPieceParentConnectionFullCommand: { readonly connection: ConnectionDto | null } };

export type ReadTypeCommandOutput = {
  readonly readTypeBestRepresentationCommand: { readonly representation: RepresentationDto | null };
};

export type ReadDesignCommandOutput =
  | { readonly readDesignPiecesFullCommand: { readonly pieces: readonly PieceDto[] } }
  | { readonly readDesignConnectionsFullCommand: { readonly connections: readonly ConnectionDto[] } }
  | { readonly readDesignPieceCommands: { readonly results: readonly ReadPieceCommandOutput[] } }
  | { readonly readDesignClusterableGroupsCommand: { readonly groups: readonly (readonly KitIdDto[])[] } }
  | { readonly readDesignIncludedDesignsCommand: { readonly designs: readonly IncludedDesignInfoDto[] } }
  | { readonly readDesignQualitySumCommand: { readonly sum: number } }
  | { readonly readDesignReplaceableCatalogCommand: { readonly types: readonly KitIdDto[]; readonly designs: readonly KitIdDto[] } }
  | { readonly readDesignIncludedDesignIdsCommand: { readonly designIds: readonly string[] } };

/** @emoji 🧾 One command’s read output object (per-command payload shape from `semio/rs` GraphQL). */
export type ReadKitCommandOutput =
  | { readonly readKitFullCommand: { readonly full: KitFullDto } }
  | { readonly readKitShallowCommand: { readonly types: readonly TypeShallow[]; readonly designs: readonly DesignShallow[] } }
  | { readonly readKitTypeIdsCommand: { readonly typeIds: readonly KitIdDto[] } }
  | { readonly readKitDesignIdsCommand: { readonly designIds: readonly KitIdDto[] } }
  | { readonly readKitTypesMetadataCommand: { readonly types: readonly TypeMetadataDto[] } }
  | { readonly readKitDesignsMetadataCommand: { readonly designs: readonly DesignMetadataDto[] } }
  | { readonly readKitTypesShallowCommand: { readonly types: readonly TypeShallow[] } }
  | { readonly readKitDesignsShallowCommand: { readonly designs: readonly DesignShallow[] } }
  | { readonly readKitAuthorsShallowCommand: { readonly authors: readonly AuthorMetadataDto[] } }
  | { readonly readKitMetadataCommand: { readonly metadata: KitMetadataDto | null } }
  | { readonly readKitDesignCommands: { readonly results: readonly ReadDesignCommandOutput[] } }
  | { readonly readKitTypeCommands: { readonly results: readonly ReadTypeCommandOutput[] } };

/** @emoji 🧾 Batch output from {@link KitStore.read}. */
export type ReadBatchResult = readonly ReadKitCommandOutput[];

/**
 * @emoji 📣 GraphQL `KitEvent` scalar + synthetic invalidation rows used by {@link WasmKitStoreClient};
 * field-level rows remain {@link KitJsonObjectDto}; classified kit mutations are top-level keys (`renamedDesign`, `changedKit`, …).
 */
export type KitEvent = Readonly<{ readonly Changed: null } | { readonly ValidationInvalidated: null } | KitCommandLifecycleEvent | KitClassifiedMutationEvent | KitJsonObjectDto>;

/** @emoji 🧾 Optional filter for {@link KitStore.subscribeFiltered}. */
export type KitEventFilter = (event: KitEvent) => boolean;

/** @emoji 🧾 Unsubscribe handle returned by {@link KitStore.subscribe}. */
export type Unsubscribe = () => void;

export type KitCommandReceipt = { requestId: KitCommandRequestId; commandKind: string; accepted: boolean };

export type KitStoreOpenOptions = {
  wasmSpecifier?: string;
  timeoutMs?: number;
  /** Optional worker factory (tests); defaults to the inline module worker defined in this file. */
  workerFactory?: () => Worker;
};

// #region 🔖ChangeKitCommand
/** @emoji 🧾 `ChangePieceCommand` JSON (externally tagged, camelCase variant keys) for `kitStore.batch` live `changeKitCommands` (or `ChangeKitCommand` GraphQL scalars). */
export type ChangePieceCommand =
  | { readonly name: { readonly name?: string | null } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly plane: { readonly plane?: KitJsonTreeDto | null } }
  | { readonly center: { readonly center?: KitJsonTreeDto | null } }
  | { readonly scale: { readonly scale?: number | null } }
  | { readonly mirrorPlane: { readonly mirrorPlane?: KitJsonTreeDto | null } }
  | { readonly hidden: { readonly hidden?: boolean | null } }
  | { readonly locked: { readonly locked?: boolean | null } }
  | { readonly color: { readonly color?: string | null } }
  | { readonly type: { readonly typeId?: KitIdDto | null } }
  | { readonly addProp: { readonly prop: PropDto } }
  | { readonly removeProp: { readonly propId: KitIdDto } }
  | { readonly addAttribute: { readonly attribute: AttributeDto } }
  | { readonly removeAttribute: { readonly id: KitIdDto } };

/** @emoji 🧾 `ChangeConnectionCommand` JSON for nested design commands. */
export type ChangeConnectionCommand =
  | { readonly gap: { readonly value?: number | null } }
  | { readonly shift: { readonly value?: number | null } }
  | { readonly rise: { readonly value?: number | null } }
  | { readonly rotation: { readonly value?: number | null } }
  | { readonly turn: { readonly value?: number | null } }
  | { readonly tilt: { readonly value?: number | null } }
  | { readonly x: { readonly value?: number | null } }
  | { readonly y: { readonly value?: number | null } }
  | { readonly description: { readonly value?: string | null } }
  | { readonly addConnectionAttribute: { readonly attribute: AttributeDto } }
  | { readonly removeConnectionAttribute: { readonly id: KitIdDto } };

/** @emoji 🧾 Nested `ChangeDesignCommand` entries. */
export type ChangeDesignCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } }
  | { readonly image: { readonly image?: string | null } }
  | { readonly unit: { readonly unit?: string | null } }
  | { readonly addPiece: { readonly piece: PieceDto } }
  | { readonly removePiece: { readonly pieceId: KitIdDto } }
  | { readonly addConnection: { readonly connection: ConnectionDto } }
  | { readonly removeConnection: { readonly connectionId: KitIdDto } }
  | { readonly changePieceCommands: { readonly pieceId: KitIdDto; readonly commands: readonly ChangePieceCommand[] } }
  | { readonly changeConnectionCommands: { readonly connectionId: KitIdDto; readonly commands: readonly ChangeConnectionCommand[] } };

/** @emoji 🧾 Nested `ChangeTypeCommand` entries used by stores / React. */
export type ChangeTypeCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } }
  | { readonly image: { readonly image?: string | null } }
  | { readonly stock: { readonly stock?: number | null } }
  | { readonly typeVirtual: { readonly value?: boolean | null } }
  | { readonly unit: { readonly unit?: string | null } }
  | { readonly addRepresentation: { readonly representation: RepresentationDto } }
  | { readonly removeRepresentation: { readonly id: KitIdDto } }
  | { readonly addConnector: { readonly connector: ConnectorDto } }
  | { readonly removeConnector: { readonly connectorId: KitIdDto } }
  | { readonly addTypeProp: { readonly prop: PropDto } }
  | { readonly removeTypeProp: { readonly propId: KitIdDto } };

/** @emoji 🧾 `ChangeFamilyCommand` JSON. */
export type ChangeFamilyCommand = { readonly name: { readonly name: string } } | { readonly description: { readonly description?: string | null } } | { readonly icon: { readonly icon?: string | null } };

export type ChangeFileCommand =
  | { readonly url: { readonly url: string } }
  | { readonly mime: { readonly mime?: string | null } }
  | { readonly size: { readonly size?: number | null } }
  | { readonly hash: { readonly hash?: string | null } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly created: { readonly created?: string | null } }
  | { readonly updated: { readonly updated?: string | null } };

export type ChangeFolderCommand = { readonly path: { readonly path: string } } | { readonly description: { readonly description?: string | null } };

export type ChangeAuthorCommand = { readonly name: { readonly name: string } } | { readonly email: { readonly email: string } } | { readonly role: { readonly role?: string | null } } | { readonly rank: { readonly rank?: number | null } };

export type ChangeConceptCommand = { readonly name: { readonly name: string } } | { readonly description: { readonly description?: string | null } } | { readonly order: { readonly order?: number | null } };

export type ChangeTagCommand = { readonly name: { readonly name: string } } | { readonly order: { readonly order?: number | null } };

export type ChangeKitQualityCommand =
  | { readonly key: { readonly key: string } }
  | { readonly value: { readonly value?: string | null } }
  | { readonly unit: { readonly unit?: string | null } }
  | { readonly definition: { readonly definition?: string | null } }
  | { readonly description: { readonly description?: string | null } };

export type ChangePortCommand = { readonly name: { readonly name: string } } | { readonly description: { readonly description?: string | null } } | { readonly icon: { readonly icon?: string | null } };

/** @emoji 🧾 Top-level `ChangeKitCommand` JSON for `changeKitCommands` batch variables. */
export type ChangeKitCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } }
  | { readonly image: { readonly image?: string | null } }
  | { readonly preview: { readonly preview?: string | null } }
  | { readonly remote: { readonly remote?: string | null } }
  | { readonly homepage: { readonly homepage?: string | null } }
  | { readonly license: { readonly license?: string | null } }
  | { readonly uri: { readonly uri?: string | null } }
  | { readonly created: { readonly created?: string | null } }
  | { readonly updated: { readonly updated?: string | null } }
  | { readonly version: { readonly version?: string | null } }
  | { readonly addType: { readonly type: TypeDto } }
  | { readonly removeType: { readonly typeId: KitIdDto } }
  | { readonly addDesign: { readonly design: DesignDto } }
  | { readonly removeDesign: { readonly designId: KitIdDto } }
  | { readonly changeDesignCommands: { readonly designId: KitIdDto; readonly commands: readonly ChangeDesignCommand[] } }
  | { readonly changeTypeCommands: { readonly typeId: KitIdDto; readonly commands: readonly ChangeTypeCommand[] } }
  | { readonly changeFamilyCommands: { readonly familyId: KitIdDto; readonly commands: readonly ChangeFamilyCommand[] } }
  | { readonly changeFileCommands: { readonly fileId: KitIdDto; readonly commands: readonly ChangeFileCommand[] } }
  | { readonly changeFolderCommands: { readonly folderId: KitIdDto; readonly commands: readonly ChangeFolderCommand[] } }
  | { readonly changeAuthorCommands: { readonly authorId: KitIdDto; readonly commands: readonly ChangeAuthorCommand[] } }
  | { readonly changeConceptCommands: { readonly conceptId: KitIdDto; readonly commands: readonly ChangeConceptCommand[] } }
  | { readonly changeTagCommands: { readonly tagId: KitIdDto; readonly commands: readonly ChangeTagCommand[] } }
  | { readonly changeKitQualityCommands: { readonly qualityId: KitIdDto; readonly commands: readonly ChangeKitQualityCommand[] } }
  | { readonly changeKitPortCommands: { readonly portId: KitIdDto; readonly commands: readonly ChangePortCommand[] } }
  | { readonly addFamily: { readonly family: FamilyDto } }
  | { readonly removeFamily: { readonly familyId: KitIdDto } }
  | { readonly addFolder: { readonly folder: FolderDto } }
  | { readonly removeFolder: { readonly folderId: KitIdDto } }
  | { readonly addAuthor: { readonly author: AuthorDto } }
  | { readonly removeAuthor: { readonly authorId: KitIdDto } }
  | { readonly addConcept: { readonly concept: ConceptDto } }
  | { readonly removeConcept: { readonly conceptId: KitIdDto } }
  | { readonly addTag: { readonly tag: TagDto } }
  | { readonly removeTag: { readonly tagId: KitIdDto } }
  | { readonly addQuality: { readonly quality: QualityDto } }
  | { readonly removeQuality: { readonly qualityId: KitIdDto } }
  | { readonly addKitProp: { readonly prop: PropDto } }
  | { readonly removeKitProp: { readonly propId: KitIdDto } }
  | { readonly addKitAttribute: { readonly attribute: AttributeDto } }
  | { readonly removeKitAttribute: { readonly id: KitIdDto } }
  | { readonly addFile: { readonly file: FileDto } }
  | { readonly removeFile: { readonly fileId: KitIdDto } }
  | { readonly replaceKitFromFull: { readonly dto: KitFullDto } }
  | { readonly clusterPieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[]; readonly clusterName: string } }
  | { readonly dragPieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[]; readonly du: number; readonly dv: number } }
  | { readonly movePieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[]; readonly gap: number; readonly shift: number; readonly rise: number } }
  | { readonly fixPieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[] } }
  | { readonly flattenDesign: { readonly designId: KitIdDto } }
  | { readonly expandNestedDesign: { readonly parentDesignId: KitIdDto; readonly nestedDesignId: KitIdDto } }
  | { readonly deleteConnection: { readonly designId: KitIdDto; readonly connectionId: KitIdDto } }
  | { readonly changePieceKind: { readonly designId: KitIdDto; readonly pieceId: KitIdDto; readonly newTypeId: KitIdDto } }
  | {
      readonly addChildPieceWithParentConnection: {
        readonly designId: KitIdDto;
        readonly parentPiece: string;
        readonly parentPort: string;
        readonly childType: string;
        readonly childPort: string;
      };
    };

/**
 * @public GraphQL `variables` map for `kitStore.batch` (kit JSON object trees, camelCase; matches {@link GraphQlObjectMutable} construction in this module).
 */
export type GraphQlVariables = KitJsonObjectDto;

/** @emoji 🧾 True for object-shaped JSON / kit tree nodes (excludes arrays and `null`). */
function isJsonObjectNode(v: JsonValue | KitJsonTreeDto | null | undefined): v is GraphQlVariables | JsonObject | KitJsonObjectDto {
  return v != null && typeof v === "object" && !Array.isArray(v);
}

/** @emoji 🧾 GraphQL `ConflictBatchRecord` row (matches `semio/graphql/schema.graphql`). */
export type ConflictBatchRecord = Readonly<{
  id: string;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
}>;

/** @emoji 🧾 One row from GraphQL `KitStoreResult` (camelCase dto). */
export type KitStoreBatchResultRow = Readonly<{
  kind: string;
  ok?: boolean | null;
  sessionId?: string | null;
  changeId?: string | null;
  changeKind?: KitChangeSemanticKindGql | null;
  changeKindOther?: string | null;
  inverse?: readonly ChangeKitCommand[] | null;
  conflicts?: readonly ConflictBatchRecord[] | null;
  backbone?: { attached: boolean; kind?: string | null; tip?: string | null } | null;
}>;

/** @emoji 🧾 Forward + inverse command atoms on the subscription bus (`KitChange` from `semio/rs`). */
export type KitChange = Readonly<{
  readonly forward: readonly ChangeKitCommand[];
  readonly inverse: readonly ChangeKitCommand[];
  readonly kind?: KitChangeKind;
  readonly author?: string | null;
  readonly time?: string | null;
}>;

/** @emoji 🧾 Payload for {@link KitEvent} `renamedDesign` (camelCase from `semio/rs`). */
export type RenamedDesignKitEvent = Readonly<{ readonly designId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `renamedType`. */
export type RenamedTypeKitEvent = Readonly<{ readonly typeId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `draggedFlatCenterPiece`. */
export type DraggedFlatCenterPieceKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `movedPiecesFlatCenter`. */
export type MovedPiecesFlatCenterKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `clusteredPieces`. */
export type ClusteredPiecesKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `fixedPiecesFlatCenter`. */
export type FixedPiecesFlatCenterKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `flattenedDesign`. */
export type FlattenedDesignKitEvent = Readonly<{ readonly designId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `expandedNestedDesign`. */
export type ExpandedNestedDesignKitEvent = Readonly<{
  readonly parentDesignId: string;
  readonly nestedDesignId: string;
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `deletedConnection`. */
export type DeletedConnectionKitEvent = Readonly<{
  readonly designId: string;
  readonly connectionId: string;
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedPieceKind`. */
export type ChangedPieceKindKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceId: string;
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedDesignCommands`. */
export type ChangedDesignCommandsKitEvent = Readonly<{ readonly designId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedTypeCommands`. */
export type ChangedTypeCommandsKitEvent = Readonly<{ readonly typeId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedKit` (fallback classified change). */
export type ChangedKitEvent = Readonly<{ readonly change: KitChange }>;

/** @emoji 🧾 Classified kit mutation row: exactly one variant key plus payload (same dto as `semio/rs` `KitEvent`). */
export type KitClassifiedMutationEvent = Readonly<
  | { readonly renamedDesign: RenamedDesignKitEvent }
  | { readonly renamedType: RenamedTypeKitEvent }
  | { readonly draggedFlatCenterPiece: DraggedFlatCenterPieceKitEvent }
  | { readonly movedPiecesFlatCenter: MovedPiecesFlatCenterKitEvent }
  | { readonly clusteredPieces: ClusteredPiecesKitEvent }
  | { readonly fixedPiecesFlatCenter: FixedPiecesFlatCenterKitEvent }
  | { readonly flattenedDesign: FlattenedDesignKitEvent }
  | { readonly expandedNestedDesign: ExpandedNestedDesignKitEvent }
  | { readonly deletedConnection: DeletedConnectionKitEvent }
  | { readonly changedPieceKind: ChangedPieceKindKitEvent }
  | { readonly changedDesignCommands: ChangedDesignCommandsKitEvent }
  | { readonly changedTypeCommands: ChangedTypeCommandsKitEvent }
  | { readonly changedKit: ChangedKitEvent }
>;

const __KIT_CLASSIFIED_MUTATION_KEYS = [
  "renamedDesign",
  "renamedType",
  "draggedFlatCenterPiece",
  "movedPiecesFlatCenter",
  "clusteredPieces",
  "fixedPiecesFlatCenter",
  "flattenedDesign",
  "expandedNestedDesign",
  "deletedConnection",
  "changedPieceKind",
  "changedDesignCommands",
  "changedTypeCommands",
  "changedKit",
] as const;

/** @emoji 🧾 True when {@link KitEvent} is a {@link KitClassifiedMutationEvent}. */
export function isKitClassifiedMutationEvent(ev: KitEvent): ev is KitClassifiedMutationEvent {
  if (typeof ev !== "object" || ev === null) return false;
  for (const k of __KIT_CLASSIFIED_MUTATION_KEYS) {
    if (k in ev) return true;
  }
  return false;
}

/** @emoji 🧾 Wrap nested piece commands under one design id. */
export function kitChangeDesignPiece(designId: string, pieceId: string, commands: readonly ChangePieceCommand[]): ChangeKitCommand {
  return {
    changeDesignCommands: {
      designId: { id: designId },
      commands: [{ changePieceCommands: { pieceId: { id: pieceId }, commands: [...commands] } }],
    },
  };
}

/** @emoji 🧾 Wrap nested connection commands under one design id. */
export function kitChangeDesignConnection(designId: string, connectionId: string, commands: readonly ChangeConnectionCommand[]): ChangeKitCommand {
  return {
    changeDesignCommands: {
      designId: { id: designId },
      commands: [{ changeConnectionCommands: { connectionId: { id: connectionId }, commands: [...commands] } }],
    },
  };
}

const __kid = (x: string): { readonly id: string } => ({ id: x });

/** @emoji 🧾 Maps schema/UI data keys onto connection dto keys (`u`→`x`, `v`→`y`). */
export function connectionDiffKeyForDataKey(dataKey: string): string {
  if (dataKey === "u") return "x";
  if (dataKey === "v") return "y";
  return dataKey;
}

/** @emoji 🧾 UI/schema partial for piece field writes (maps to {@link ChangePieceCommand}). */
export type PieceFieldPatchInput = Readonly<{
  name?: string | null;
  description?: string | null;
  /** @emoji 📐 Optional explicit placement; expands to `plane` / `center` change commands. */
  pose?: Readonly<{ plane?: KitJsonTreeDto; center?: KitJsonTreeDto }>;
  plane?: KitJsonTreeDto;
  center?: KitJsonTreeDto;
  scale?: number | string | null;
  mirrorPlane?: KitJsonTreeDto;
  hidden?: boolean;
  isHidden?: boolean;
  locked?: boolean;
  isLocked?: boolean;
  color?: string | null;
  type?: string | KitJsonObjectDto | null;
}>;

/** @emoji 🧾 UI/schema partial for connection field writes. */
export type ConnectionFieldPatchInput = Readonly<{
  gap?: number | string | null;
  shift?: number | string | null;
  rise?: number | string | null;
  rotation?: number | string | null;
  turn?: number | string | null;
  tilt?: number | string | null;
  x?: number | string | null;
  y?: number | string | null;
  u?: number | string | null;
  v?: number | string | null;
  description?: string | null;
}>;

/** @emoji 🧾 Value bucket for `buildSchemaEntityChangeCommands` (schema hooks). */
export type SchemaEntityFieldValue = KitJsonTreeDto | string | number | boolean | null | object;

/** @emoji 🧾 Converts a piece field patch into nested `changePieceCommands` dto entries. */
export function piecePatchToChangeCommands(patch: PieceFieldPatchInput): ChangePieceCommand[] {
  const out: ChangePieceCommand[] = [];
  if ("name" in patch) out.push({ name: { name: patch.name == null ? null : String(patch.name) } });
  if ("description" in patch) out.push({ description: { description: patch.description == null ? null : String(patch.description) } });
  if ("pose" in patch && patch.pose) {
    const po = patch.pose;
    if ("plane" in po) out.push({ plane: { plane: po.plane } });
    if ("center" in po) out.push({ center: { center: po.center } });
  }
  if ("plane" in patch) out.push({ plane: { plane: patch.plane } });
  if ("center" in patch) out.push({ center: { center: patch.center } });
  if ("scale" in patch) out.push({ scale: { scale: typeof patch.scale === "number" ? patch.scale : Number(patch.scale) } });
  if ("mirrorPlane" in patch) out.push({ mirrorPlane: { mirrorPlane: patch.mirrorPlane } });
  if ("hidden" in patch) out.push({ hidden: { hidden: Boolean(patch.hidden) } });
  if ("isHidden" in patch) out.push({ hidden: { hidden: Boolean(patch.isHidden) } });
  if ("locked" in patch) out.push({ locked: { locked: Boolean(patch.locked) } });
  if ("isLocked" in patch) out.push({ locked: { locked: Boolean(patch.isLocked) } });
  if ("color" in patch) out.push({ color: { color: patch.color == null ? null : String(patch.color) } });
  if ("type" in patch) {
    const t = patch.type;
    const tid = t && typeof t === "object" && t !== null && "id" in t ? String((t as { id: string }).id) : String(t);
    out.push({ type: { typeId: { id: tid } } });
  }
  return out;
}

/** @emoji 🧾 Converts a connection field patch into nested `changeConnectionCommands` dto entries. */
export function connectionPatchToChangeCommands(patch: ConnectionFieldPatchInput): ChangeConnectionCommand[] {
  const out: ChangeConnectionCommand[] = [];
  const num = (v: string | number | null | undefined) => (typeof v === "number" && !Number.isNaN(v) ? v : Number(v));
  const opt = (v: string | number | null | undefined): number | null => (v == null ? null : num(v));
  if ("gap" in patch) out.push({ gap: { value: opt(patch.gap) } });
  if ("shift" in patch) out.push({ shift: { value: opt(patch.shift) } });
  if ("rise" in patch) out.push({ rise: { value: opt(patch.rise) } });
  if ("rotation" in patch) out.push({ rotation: { value: opt(patch.rotation) } });
  if ("turn" in patch) out.push({ turn: { value: opt(patch.turn) } });
  if ("tilt" in patch) out.push({ tilt: { value: opt(patch.tilt) } });
  if ("x" in patch) out.push({ x: { value: opt(patch.x) } });
  if ("y" in patch) out.push({ y: { value: opt(patch.y) } });
  if ("u" in patch) out.push({ x: { value: opt(patch.u) } });
  if ("v" in patch) out.push({ y: { value: opt(patch.v) } });
  if ("description" in patch) out.push({ description: { value: patch.description == null ? null : String(patch.description) } });
  return out;
}

/**
 * @emoji 🧾 Maps a schema entity + field to `changeKitCommands` for `submitChangeKitCommands` (React + kit store).
 * `designId` is required for Piece/Connection; otherwise pass `null`.
 */
export function buildSchemaEntityChangeCommands(kind: string, id: string, field: string, value: unknown, designId: string | null): readonly ChangeKitCommand[] {
  const valueCast = value as SchemaEntityFieldValue;
  void valueCast;
  switch (kind) {
    case "Kit": {
      if (field === "name") return [{ name: { name: String(value) } } as const];
      if (field === "description") return [{ description: { description: value == null ? null : String(value) } } as const];
      if (field === "icon") return [{ icon: { icon: (value as string) ?? null } } as const];
      if (field === "image") return [{ image: { image: (value as string) ?? null } } as const];
      if (field === "homepage") return [{ homepage: { homepage: (value as string) ?? null } } as const];
      if (field === "license") return [{ license: { license: (value as string) ?? null } } as const];
      if (field === "version" || field === "release") return [{ version: { version: (value as string) ?? null } } as const];
      if (field === "preview") return [{ preview: { preview: (value as string) ?? null } } as const];
      if (field === "remote") return [{ remote: { remote: (value as string) ?? null } } as const];
      if (field === "uri") return [{ uri: { uri: (value as string) ?? null } } as const];
      if (field === "created" || field === "createdAt") return [{ created: { created: (value as string) ?? null } } as const];
      if (field === "updated" || field === "updatedAt") return [{ updated: { updated: (value as string) ?? null } } as const];
      return [];
    }
    case "Type": {
      const inner = oneChangeTypeCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeTypeCommands: { typeId: __kid(id), commands: [inner] } } as const];
    }
    case "Design": {
      const inner = oneChangeDesignCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeDesignCommands: { designId: __kid(id), commands: [inner] } } as const];
    }
    case "Author": {
      const inner = oneChangeAuthorCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeAuthorCommands: { authorId: __kid(id), commands: [inner] } } as const];
    }
    case "Tag": {
      const inner = oneChangeTagCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeTagCommands: { tagId: __kid(id), commands: [inner] } } as const];
    }
    case "File": {
      const inner = oneChangeFileCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeFileCommands: { fileId: __kid(id), commands: [inner] } } as const];
    }
    case "Folder": {
      const inner = oneChangeFolderCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeFolderCommands: { folderId: __kid(id), commands: [inner] } } as const];
    }
    case "Quality": {
      const inner = oneChangeQualityCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeKitQualityCommands: { qualityId: __kid(id), commands: [inner] } } as const];
    }
    case "Port": {
      const inner = oneChangePortCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeKitPortCommands: { portId: __kid(id), commands: [inner] } } as const];
    }
    case "Concept": {
      const inner = oneChangeConceptCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeConceptCommands: { conceptId: __kid(id), commands: [inner] } } as const];
    }
    case "Family": {
      const inner = oneChangeFamilyCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeFamilyCommands: { familyId: __kid(id), commands: [inner] } } as const];
    }
    case "Piece": {
      if (!designId) return [];
      if (field === "name") return [kitChangeDesignPiece(designId, id, [{ name: { name: String(value) } }])];
      if (field === "description") return [kitChangeDesignPiece(designId, id, [{ description: { description: value == null ? null : String(value) } }])];
      if (field === "pose") {
        const po = value as { plane?: KitJsonTreeDto; center?: KitJsonTreeDto };
        const cmds: ChangePieceCommand[] = [];
        if (po && "plane" in po) cmds.push({ plane: { plane: po.plane } });
        if (po && "center" in po) cmds.push({ center: { center: po.center } });
        if (cmds.length === 0) return [];
        return [kitChangeDesignPiece(designId, id, cmds)];
      }
      if (field === "plane") return [kitChangeDesignPiece(designId, id, [{ plane: { plane: value as KitJsonTreeDto } }])];
      if (field === "center") return [kitChangeDesignPiece(designId, id, [{ center: { center: value as KitJsonTreeDto } }])];
      if (field === "scale") return [kitChangeDesignPiece(designId, id, [{ scale: { scale: Number(value) } }])];
      if (field === "mirrorPlane") return [kitChangeDesignPiece(designId, id, [{ mirrorPlane: { mirrorPlane: value as KitJsonTreeDto } }])];
      if (field === "isHidden" || field === "hidden") return [kitChangeDesignPiece(designId, id, [{ hidden: { hidden: Boolean(value) } }])];
      if (field === "isLocked" || field === "locked") return [kitChangeDesignPiece(designId, id, [{ locked: { locked: Boolean(value) } }])];
      if (field === "color") return [kitChangeDesignPiece(designId, id, [{ color: { color: value == null ? null : String(value) } }])];
      if (field === "type" || field === "typeId") {
        const t = value;
        const tid = t && typeof t === "object" && t !== null && "id" in t ? String((t as { id: string }).id) : String(t);
        return [kitChangeDesignPiece(designId, id, [{ type: { typeId: { id: tid } } }])];
      }
      return [];
    }
    case "ConnectionStore": {
      if (!designId) return [];
      const dk = connectionDiffKeyForDataKey(field);
      if (dk === "gap") return [kitChangeDesignConnection(designId, id, [{ gap: { value: Number(value) } }])];
      if (dk === "shift") return [kitChangeDesignConnection(designId, id, [{ shift: { value: Number(value) } }])];
      if (dk === "rise") return [kitChangeDesignConnection(designId, id, [{ rise: { value: Number(value) } }])];
      if (dk === "rotation") return [kitChangeDesignConnection(designId, id, [{ rotation: { value: Number(value) } }])];
      if (dk === "turn") return [kitChangeDesignConnection(designId, id, [{ turn: { value: Number(value) } }])];
      if (dk === "tilt") return [kitChangeDesignConnection(designId, id, [{ tilt: { value: Number(value) } }])];
      if (dk === "x") return [kitChangeDesignConnection(designId, id, [{ x: { value: Number(value) } }])];
      if (dk === "y") return [kitChangeDesignConnection(designId, id, [{ y: { value: Number(value) } }])];
      if (field === "description") return [kitChangeDesignConnection(designId, id, [{ description: { value: value == null ? null : String(value) } }])];
      return [];
    }
    default:
      return [];
  }
}

function oneChangeTypeCommandForField(field: string, value: SchemaEntityFieldValue): ChangeTypeCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  if (field === "image") return { image: { image: (value as string) ?? null } } as const;
  if (field === "stock") return { stock: { stock: (value as number) ?? null } } as const;
  if (field === "typeVirtual" || field === "virtual" || field === "isAbstract") return { typeVirtual: { value: (value as boolean) ?? null } } as const;
  if (field === "unit") return { unit: { unit: (value as string) ?? null } } as const;
  return null;
}
function oneChangeDesignCommandForField(field: string, value: SchemaEntityFieldValue): ChangeDesignCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  if (field === "image") return { image: { image: (value as string) ?? null } } as const;
  if (field === "unit") return { unit: { unit: (value as string) ?? null } } as const;
  return null;
}
function oneChangeAuthorCommandForField(field: string, value: SchemaEntityFieldValue): ChangeAuthorCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "email") return { email: { email: String(value ?? "") } } as const;
  if (field === "role") return { role: { role: (value as string) ?? null } } as const;
  if (field === "rank") return { rank: { rank: (value as number) ?? null } } as const;
  return null;
}
function oneChangeTagCommandForField(field: string, value: SchemaEntityFieldValue): ChangeTagCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "order" || field === "orderIndex") return { order: { order: (value as number) ?? null } } as const;
  return null;
}
function oneChangeFileCommandForField(field: string, value: SchemaEntityFieldValue): ChangeFileCommand | null {
  if (field === "url") return { url: { url: String(value ?? "") } } as const;
  if (field === "mime") return { mime: { mime: (value as string) ?? null } } as const;
  if (field === "size") return { size: { size: (value as number) ?? null } } as const;
  if (field === "hash") return { hash: { hash: (value as string) ?? null } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "created" || field === "createdAt") return { created: { created: (value as string) ?? null } } as const;
  if (field === "updated" || field === "updatedAt") return { updated: { updated: (value as string) ?? null } } as const;
  return null;
}
function oneChangeFolderCommandForField(field: string, value: SchemaEntityFieldValue): ChangeFolderCommand | null {
  if (field === "path") return { path: { path: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  return null;
}
function oneChangeQualityCommandForField(field: string, value: SchemaEntityFieldValue): ChangeKitQualityCommand | null {
  if (field === "key") return { key: { key: String(value ?? "") } } as const;
  if (field === "value") return { value: { value: (value as string) ?? null } } as const;
  if (field === "unit") return { unit: { unit: (value as string) ?? null } } as const;
  if (field === "definition") return { definition: { definition: (value as string) ?? null } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  return null;
}
function oneChangePortCommandForField(field: string, value: SchemaEntityFieldValue): ChangePortCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  return null;
}
function oneChangeConceptCommandForField(field: string, value: SchemaEntityFieldValue): ChangeConceptCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "order" || field === "orderIndex") return { order: { order: (value as number) ?? null } } as const;
  return null;
}
function oneChangeFamilyCommandForField(field: string, value: SchemaEntityFieldValue): ChangeFamilyCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  return null;
}

/** @emoji 🧾 Shorthand for `client.submitChangeKitCommands` (React / tests). */
export async function submitKitChangeCommands(client: KitStoreClient, commands: readonly ChangeKitCommand[]): Promise<SetResult> {
  return client.submitChangeKitCommands(commands);
}

/** @emoji 🧾 Locates the parent design id for a piece or connection via the current kit snapshot. */
export async function resolveDesignIdForPieceOrConnection(client: KitStoreClient, entityKind: string, entityId: string): Promise<string | null> {
  const snap = await client.fetchFullKit();
  if (entityKind === "Piece") return __findDesignIdForPieceInKitDto(snap, entityId);
  if (entityKind === "ConnectionStore") return __findDesignIdForConnectionInKitDto(snap, entityId);
  return null;
}

/** @emoji 🧾 Applies a piece field patch under one design (dto construction stays in JS). */
export async function kitStoreClientUpdatePiece(client: KitStoreClient, designId: string, pieceId: string, patch: unknown): Promise<SetResult> {
  const pcmds = piecePatchToChangeCommands(patch as PieceFieldPatchInput);
  if (!pcmds.length) return { ok: true };
  return client.submitChangeKitCommands([kitChangeDesignPiece(designId, pieceId, pcmds)]);
}

/** @emoji 🧾 Applies a connection field patch under one design (dto construction stays in JS). */
export async function kitStoreClientUpdateConnection(client: KitStoreClient, designId: string, connectionId: string, patch: unknown): Promise<SetResult> {
  const ccmds = connectionPatchToChangeCommands(patch as ConnectionFieldPatchInput);
  if (!ccmds.length) return { ok: true };
  return client.submitChangeKitCommands([kitChangeDesignConnection(designId, connectionId, ccmds)]);
}

function __findDesignIdForPieceInKitDto(kit: KitFullDto, pieceId: string): string | null {
  for (const d of kit.designs ?? []) {
    if (d.pieces?.some((p) => p.id === pieceId)) return d.id;
  }
  return null;
}
function __findDesignIdForConnectionInKitDto(kit: KitFullDto, connectionId: string): string | null {
  for (const d of kit.designs ?? []) {
    if (d.connections?.some((c) => c.id === connectionId)) return d.id;
  }
  return null;
}

/**
 * @emoji 🧾 Writes a single field on a top-level or nested entity via `changeKitCommands` (React / kit store).
 * `key` is the DTO / schema data key (e.g. `name`, `icon`).
 */
export async function writeKitStoreClientSchemaField(client: KitStoreClient, typeName: string, key: string, value: unknown, entityId: string): Promise<SetResult> {
  const valueCast = value as SchemaEntityFieldValue;
  const root = await client.fetchFullKit();
  let designId: string | null = null;
  if (typeName === "Piece") designId = __findDesignIdForPieceInKitDto(root, entityId);
  if (typeName === "ConnectionStore") designId = __findDesignIdForConnectionInKitDto(root, entityId);
  const cmds = buildSchemaEntityChangeCommands(typeName, entityId, key, valueCast, typeName === "Piece" || typeName === "ConnectionStore" ? designId : null);
  if (!cmds.length) return { ok: false, error: { kind: "NotSupported", message: `${typeName}.${key}` } };
  return client.submitChangeKitCommands(cmds);
}

// #endregion 🔖ChangeKitCommand

// #endregion 🔌JsonGraphQlDtoTypes

// #region 🔖ScopedKitMutations

/** @emoji 🧾 GraphQL string literal (escaped). */
function __gqlStr(s: string): string {
  return JSON.stringify(s);
}

/** @emoji 🧾 Inline `[ID!]!` list. */
function __gqlIds(ids: readonly string[]): string {
  return `[${ids.map((x) => __gqlStr(x)).join(",")}]`;
}

/** @emoji 🧾 Wraps kit selection under `unsavedChange` + `theKit` (target command tree). */
function __scopedKitMutationBody(changeId: string, kitSelection: string): { readonly query: string; readonly variables: GraphQlVariables } {
  return {
    query: `mutation($changeId: ID!) { session { theKit { unsavedChange(id: $changeId) { kit { ${kitSelection} } } } } }`,
    variables: { changeId },
  };
}

/** @emoji 🧾 One scoped mutation for a legacy {@link ChangeKitCommand}, or `null` when unsupported on `KitOperationInput`. */
function buildScopedChangeKitMutation(
  changeId: string,
  cmd: ChangeKitCommand,
): { readonly query: string; readonly variables: GraphQlVariables } | null {
  if ("name" in cmd && cmd.name != null && typeof cmd.name === "object" && "name" in cmd.name) {
    const n = String((cmd.name as { name?: string | null }).name ?? "");
    return __scopedKitMutationBody(changeId, `r: rename(newName: ${__gqlStr(n)})`);
  }
  if ("description" in cmd && cmd.description != null && typeof cmd.description === "object") {
    const d = String((cmd.description as { description?: string | null }).description ?? "");
    return __scopedKitMutationBody(changeId, `r: changeDescription(newDescription: ${__gqlStr(d)})`);
  }

  if ("addType" in cmd && cmd.addType != null && typeof cmd.addType === "object" && "type" in cmd.addType) {
    const t = (cmd.addType as { type: TypeDto }).type;
    return __scopedKitMutationBody(
      changeId,
      `r: createType(name: ${__gqlStr(t.name)}, description: ${__gqlStr(t.description ?? "")}, icon: ${__gqlStr(t.icon ?? "")}, image: ${__gqlStr(t.image ?? "")}, unit: ${__gqlStr(t.unit ?? "")})`,
    );
  }
  if ("removeType" in cmd && cmd.removeType != null) {
    const id = String((cmd.removeType as { typeId: KitIdDto }).typeId.id);
    return __scopedKitMutationBody(changeId, `r: deleteType(id: ${__gqlStr(id)})`);
  }

  if ("addDesign" in cmd && cmd.addDesign != null) {
    const d = (cmd.addDesign as { design: DesignDto }).design;
    return __scopedKitMutationBody(
      changeId,
      `r: createDesign(name: ${__gqlStr(d.name)}, description: ${__gqlStr(d.description ?? "")}, icon: ${__gqlStr(d.icon ?? "")}, image: ${__gqlStr(d.image ?? "")}, unit: ${__gqlStr(d.unit ?? "")})`,
    );
  }
  if ("removeDesign" in cmd && cmd.removeDesign != null) {
    const id = String((cmd.removeDesign as { designId: KitIdDto }).designId.id);
    return __scopedKitMutationBody(changeId, `r: deleteDesign(id: ${__gqlStr(id)})`);
  }

  if ("dragPieces" in cmd && cmd.dragPieces != null) {
    const x = cmd.dragPieces as { designId: KitIdDto; pieceIds: readonly string[]; du: number; dv: number };
    return __scopedKitMutationBody(
      changeId,
      `r: design(id: ${__gqlStr(x.designId.id)}) { pieces(ids: ${__gqlIds(x.pieceIds)}) { d: drag(offset: { u: ${x.du}, v: ${x.dv} }) } }`,
    );
  }
  if ("movePieces" in cmd && cmd.movePieces != null) {
    const x = cmd.movePieces as { designId: KitIdDto; pieceIds: readonly string[]; gap: number; shift: number; rise: number };
    return __scopedKitMutationBody(
      changeId,
      `r: design(id: ${__gqlStr(x.designId.id)}) { pieces(ids: ${__gqlIds(x.pieceIds)}) { m: move(offset: { u: ${x.gap}, v: ${x.shift} }) } }`,
    );
  }
  if ("fixPieces" in cmd && cmd.fixPieces != null) {
    const x = cmd.fixPieces as { designId: KitIdDto; pieceIds: readonly string[] };
    return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(x.designId.id)}) { pieces(ids: ${__gqlIds(x.pieceIds)}) { f: fix } }`);
  }
  if ("flattenDesign" in cmd && cmd.flattenDesign != null) {
    const id = String((cmd.flattenDesign as { designId: KitIdDto }).designId.id);
    return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(id)}) { fl: flatten }`);
  }

  if ("changeTypeCommands" in cmd && cmd.changeTypeCommands != null) {
    const block = cmd.changeTypeCommands;
    const tid = String(block.typeId.id);
    const c0 = block.commands[0];
    if (!c0) return null;
    if ("name" in c0 && c0.name != null) {
      return __scopedKitMutationBody(changeId, `r: type(id: ${__gqlStr(tid)}) { n: rename(newName: ${__gqlStr(String((c0.name as { name: string }).name))}) }`);
    }
    if ("description" in c0 && c0.description != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: type(id: ${__gqlStr(tid)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0.description as { description?: string | null }).description ?? ""))}) }`,
      );
    }
    if ("icon" in c0 && c0.icon != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: type(id: ${__gqlStr(tid)}) { i: changeIcon(newIcon: ${__gqlStr(String((c0.icon as { icon?: string | null }).icon ?? ""))}) }`,
      );
    }
    if ("image" in c0 && c0.image != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: type(id: ${__gqlStr(tid)}) { i: changeIcon(newIcon: ${__gqlStr(String((c0.image as { image?: string | null }).image ?? ""))}) }`,
      );
    }
    if ("unit" in c0 && c0.unit != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: type(id: ${__gqlStr(tid)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0.unit as { unit?: string | null }).unit ?? ""))}) }`,
      );
    }
    if ("stock" in c0 && c0.stock != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: type(id: ${__gqlStr(tid)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0.stock as { stock?: number | null }).stock ?? ""))}) }`,
      );
    }
    if ("typeVirtual" in c0 && c0.typeVirtual != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: type(id: ${__gqlStr(tid)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0.typeVirtual as { value?: boolean | null }).value ?? false))}) }`,
      );
    }
    return null;
  }

  if ("changeDesignCommands" in cmd && cmd.changeDesignCommands != null) {
    const block = cmd.changeDesignCommands;
    const did = String(block.designId.id);
    const d0 = block.commands[0];
    if (!d0) return null;
    if ("name" in d0 && d0.name != null) {
      return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(did)}) { n: rename(newName: ${__gqlStr(String((d0.name as { name: string }).name))}) }`);
    }
    if ("description" in d0 && d0.description != null) {
      return __scopedKitMutationBody(
        changeId,
        `r: design(id: ${__gqlStr(did)}) { d: changeDescription(newDescription: ${__gqlStr(String((d0.description as { description?: string | null }).description ?? ""))}) }`,
      );
    }
    if ("addPiece" in d0 && d0.addPiece != null) {
      const piece = (d0.addPiece as { piece: PieceDto }).piece;
      const bp =
        typeof piece.type === "object" && piece.type != null && "id" in piece.type ? String((piece.type as { id: string }).id) : String(piece.type);
      const pl = __kitPlaneToBatchInput(piece.plane);
      const c = piece.center;
      if (!pl || !isJsonObjectNode(c)) return null;
      const u = __coerceAxisComponent(c["u"]);
      const v = __coerceAxisComponent(c["v"]);
      if (!Number.isFinite(u) || !Number.isFinite(v)) return null;
      const posInl = `{ center: { u: ${u}, v: ${v} }, plane: { origin: { x: ${pl.origin.x}, y: ${pl.origin.y}, z: ${pl.origin.z} }, xAxis: { x: ${pl.xAxis.x}, y: ${pl.xAxis.y}, z: ${pl.xAxis.z} }, yAxis: { x: ${pl.yAxis.x}, y: ${pl.yAxis.y}, z: ${pl.yAxis.z} } } }`;
      return __scopedKitMutationBody(
        changeId,
        `r: design(id: ${__gqlStr(did)}) { ap: addFixedPiece(blueprintId: ${__gqlStr(bp)}, position: ${posInl}, name: ${piece.name != null ? __gqlStr(String(piece.name)) : "null"}, description: ${piece.description != null ? __gqlStr(String(piece.description)) : "null"}) }`,
      );
    }
    if ("removePiece" in d0 && d0.removePiece != null) {
      const pid = String((d0.removePiece as { pieceId: KitIdDto }).pieceId.id);
      return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(did)}) { dp: deletePiece(id: ${__gqlStr(pid)}) }`);
    }
    if ("removeConnection" in d0 && d0.removeConnection != null) {
      const cid = String((d0.removeConnection as { connectionId: KitIdDto }).connectionId.id);
      return __scopedKitMutationBody(
        changeId,
        `r: design(id: ${__gqlStr(did)}) { dc: deletePiecesAndConnections(pieceIds: [], connectionIds: [${__gqlStr(cid)}]) }`,
      );
    }
    if ("changePieceCommands" in d0 && d0.changePieceCommands != null) {
      const pid = String(d0.changePieceCommands.pieceId.id);
      const p0 = d0.changePieceCommands.commands[0];
      if (!p0) return null;
      if ("name" in p0 && p0.name != null) {
        return __scopedKitMutationBody(
          changeId,
          `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { n: rename(newName: ${__gqlStr(String((p0.name as { name?: string | null }).name ?? ""))}) } }`,
        );
      }
      if ("description" in p0 && p0.description != null) {
        return __scopedKitMutationBody(
          changeId,
          `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { d: changeDescription(newDescription: ${__gqlStr(String((p0.description as { description?: string | null }).description ?? ""))}) } }`,
        );
      }
      if ("plane" in p0 && p0.plane != null) {
        const pl = __kitPlaneToBatchInput((p0.plane as { plane?: KitJsonTreeDto }).plane);
        if (!pl) return null;
        const posInl = `{ center: { u: 0, v: 0 }, plane: { origin: { x: ${pl.origin.x}, y: ${pl.origin.y}, z: ${pl.origin.z} }, xAxis: { x: ${pl.xAxis.x}, y: ${pl.xAxis.y}, z: ${pl.xAxis.z} }, yAxis: { x: ${pl.yAxis.x}, y: ${pl.yAxis.y}, z: ${pl.yAxis.z} } } }`;
        return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { mv: move(position: ${posInl}) } }`);
      }
      if ("center" in p0 && p0.center != null) {
        const ctr = (p0.center as { center?: KitJsonTreeDto }).center;
        if (!isJsonObjectNode(ctr)) return null;
        const u = __coerceAxisComponent(ctr["u"]);
        const v = __coerceAxisComponent(ctr["v"]);
        if (!Number.isFinite(u) || !Number.isFinite(v)) return null;
        const posInl = `{ center: { u: ${u}, v: ${v} }, plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } } }`;
        return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { mv: move(position: ${posInl}) } }`);
      }
      if ("scale" in p0 && p0.scale != null) {
        return __scopedKitMutationBody(
          changeId,
          `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { d: changeDescription(newDescription: ${__gqlStr(String((p0.scale as { scale?: number | null }).scale ?? ""))}) } }`,
        );
      }
      if ("color" in p0 && p0.color != null) {
        return __scopedKitMutationBody(
          changeId,
          `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { d: changeDescription(newDescription: ${__gqlStr(String((p0.color as { color?: string | null }).color ?? ""))}) } }`,
        );
      }
      if ("hidden" in p0 && p0.hidden != null) {
        return __scopedKitMutationBody(
          changeId,
          `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { d: changeDescription(newDescription: ${__gqlStr(String((p0.hidden as { hidden?: boolean | null }).hidden ?? false))}) } }`,
        );
      }
      if ("locked" in p0 && p0.locked != null) {
        return __scopedKitMutationBody(
          changeId,
          `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { d: changeDescription(newDescription: ${__gqlStr(String((p0.locked as { locked?: boolean | null }).locked ?? false))}) } }`,
        );
      }
      if ("type" in p0 && p0.type != null) {
        const t = (p0.type as { typeId?: KitIdDto }).typeId?.id;
        if (!t) return null;
        return __scopedKitMutationBody(changeId, `r: design(id: ${__gqlStr(did)}) { piece(id: ${__gqlStr(pid)}) { cb: changeBlueprint(blueprintId: ${__gqlStr(String(t))}) } }`);
      }
    }
    return null;
  }

  if ("changeTagCommands" in cmd && cmd.changeTagCommands != null) {
    const b = cmd.changeTagCommands;
    const id = String(b.tagId.id);
    const c0 = b.commands[0];
    if (!c0) return null;
    if ("name" in c0) return __scopedKitMutationBody(changeId, `r: tag(id: ${__gqlStr(id)}) { n: rename(newName: ${__gqlStr(String((c0 as { name: { name: string } }).name.name))}) }`);
    if ("order" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: tag(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { order: { order?: number | null } }).order.order ?? ""))}) }`,
      );
    return null;
  }
  if ("changeConceptCommands" in cmd && cmd.changeConceptCommands != null) {
    const b = cmd.changeConceptCommands;
    const id = String(b.conceptId.id);
    const c0 = b.commands[0];
    if (!c0) return null;
    if ("name" in c0) return __scopedKitMutationBody(changeId, `r: concept(id: ${__gqlStr(id)}) { n: rename(newName: ${__gqlStr(String((c0 as { name: { name: string } }).name.name))}) }`);
    if ("description" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: concept(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { description: { description?: string | null } }).description.description ?? ""))}) }`,
      );
    if ("order" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: concept(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { order: { order?: number | null } }).order.order ?? ""))}) }`,
      );
    return null;
  }
  if ("changeKitQualityCommands" in cmd && cmd.changeKitQualityCommands != null) {
    const b = cmd.changeKitQualityCommands;
    const id = String(b.qualityId.id);
    const c0 = b.commands[0];
    if (!c0) return null;
    if ("key" in c0) return __scopedKitMutationBody(changeId, `r: quality(id: ${__gqlStr(id)}) { k: rename(newKey: ${__gqlStr(String((c0 as { key: { key: string } }).key.key))}) }`);
    if ("value" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: quality(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { value: { value?: string | null } }).value.value ?? ""))}) }`,
      );
    if ("unit" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: quality(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { unit: { unit?: string | null } }).unit.unit ?? ""))}) }`,
      );
    if ("definition" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: quality(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { definition: { definition?: string | null } }).definition.definition ?? ""))}) }`,
      );
    if ("description" in c0)
      return __scopedKitMutationBody(
        changeId,
        `r: quality(id: ${__gqlStr(id)}) { d: changeDescription(newDescription: ${__gqlStr(String((c0 as { description: { description?: string | null } }).description.description ?? ""))}) }`,
      );
    return null;
  }

  if ("addChildPieceWithParentConnection" in cmd && cmd.addChildPieceWithParentConnection != null) {
    const x = cmd.addChildPieceWithParentConnection;
    return __scopedKitMutationBody(
      changeId,
      `r: design(id: ${__gqlStr(x.designId.id)}) { ac: addChildPieceWithParentConnection(blueprintId: ${__gqlStr(x.childType)}, parentPieceId: ${__gqlStr(x.parentPiece)}, parentConnector: ${__gqlStr(x.parentPort)}, childConnector: ${__gqlStr(x.childPort)}) }`,
    );
  }

  if ("changePieceKind" in cmd && cmd.changePieceKind != null) {
    const x = cmd.changePieceKind as { designId: KitIdDto; pieceId: KitIdDto; newTypeId: KitIdDto };
    return __scopedKitMutationBody(
      changeId,
      `r: design(id: ${__gqlStr(x.designId.id)}) { piece(id: ${__gqlStr(x.pieceId.id)}) { cb: changeBlueprint(blueprintId: ${__gqlStr(x.newTypeId.id)}) } }`,
    );
  }

  return null;
}

// #endregion 🔖ScopedKitMutations

// #region 🧰GraphqlUtil

function normalizeRustSetError(raw: JsonValue): SetError {
  if (raw == null || typeof raw !== "object" || Array.isArray(raw)) return { kind: "Internal", message: "invalid error payload" };
  if (!isJsonObjectNode(raw)) return { kind: "Internal", message: "invalid error payload" };
  const o = raw as JsonObject;
  const kind = typeof o["kind"] === "string" ? (o["kind"] as SetErrorKind) : "Internal";
  const message = typeof o["message"] === "string" ? o["message"] : JSON.stringify(raw);
  return { kind, message };
}

function normalizeWasmThrownKitError(err: { toString(): string }): SetError {
  const message = String(err)
    .replace(/^Error:\s*/, "")
    .trim();
  const lower = message.toLowerCase();
  if (lower.includes("illegal name") || lower.includes("cannot be empty")) return { kind: "IllegalName", message };
  if (lower.includes("name too long") || (lower.includes("exceeds") && lower.includes("char"))) return { kind: "NameTooLong", message };
  return { kind: "Internal", message };
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

function kitGraphqlData<TData>(response: KitGraphqlResponseEnvelope<TData>): TData {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response;
  if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
  const d = r.data;
  if (d != null && typeof d === "object") return d;
  throw new Error("kitGraphql: no data in response");
}

/** @emoji 🧾 GraphQL selection on a target-schema version's nested `kit` (`wip.theKit.kit`, checkpoint root, or alternative kit). */
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

/** @emoji 🧾 Extract the scoped `Kit` JSON payload for a {@link KitReadPoint}. */
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

/** @emoji 🧾 Flattens Relay `*Connection` lists on `Query.wip` for VCS helpers (`canUndo`, tests). */
function __normalizeWipVcsRelayFields(wip: JsonObject | null | undefined): JsonObject {
  if (wip == null || typeof wip !== "object" || Array.isArray(wip)) return {} as JsonObject;
  const out = __mutableJsonObjectCopy(wip);
  for (const k of ["alternatives", "checkpoints", "releases"] as const) {
    const v = out[k];
    if (v != null) out[k] = kitGraphqlJsonToReadonlyArray(v as JsonValue) as unknown as JsonValue;
  }
  return out as JsonObject;
}

function kitGraphqlJsonToReadonlyArray(v: JsonValue | KitJsonTreeDto | null | undefined): readonly KitJsonTreeDto[] {
  if (Array.isArray(v)) return v as readonly KitJsonTreeDto[];
  if (v == null) return [];
  if (typeof v === "string") {
    try {
      const p: JsonValue = parseJsonValue(v);
      return kitGraphqlJsonToReadonlyArray(p);
    } catch {
      return [];
    }
  }
  if (typeof v === "object" && !Array.isArray(v)) {
    const jo = v as JsonObject;
    const items = jo["items"];
    if (Array.isArray(items)) {
      return items as readonly KitJsonTreeDto[];
    }
    const edges = jo["edges"];
    if (Array.isArray(edges)) {
      const out: KitJsonTreeDto[] = [];
      for (const e of edges) {
        if (e != null && typeof e === "object" && !Array.isArray(e)) {
          const n = (e as JsonObject)["node"];
          if (n != null && typeof n === "object" && !Array.isArray(n)) out.push(n as KitJsonTreeDto);
        }
      }
      return out;
    }
  }
  return [];
}

function __mutableJsonObjectCopy(row: JsonObject | KitJsonObjectDto): { [k: string]: JsonValue } {
  const o: { [k: string]: JsonValue } = {};
  for (const [k, v] of Object.entries(row)) {
    o[k] = v as JsonValue;
  }
  return o;
}

/** @emoji 🧾 Maps GraphQL `TypeMetadataObject` / `DesignMetadataObject` field names to {@link TypeSchema} / {@link DesignSchema} dto keys (`createdAt`, `virtual`, …). */
function __normalizeTypeOrDesignMetadataRow(row: JsonObject | KitJsonObjectDto): JsonObject {
  const out = __mutableJsonObjectCopy(row);
  if (out["createdAt"] === undefined && typeof out["created"] === "string") out["createdAt"] = out["created"] as string;
  delete out["created"];
  if (out["updatedAt"] === undefined && typeof out["updated"] === "string") out["updatedAt"] = out["updated"] as string;
  delete out["updated"];
  if (out["updatedAt"] === undefined && typeof out["lastChangedAt"] === "string") out["updatedAt"] = out["lastChangedAt"] as string;
  delete out["lastChangedAt"];
  if (out["virtual"] === undefined && typeof out["typeVirtual"] === "boolean") out["virtual"] = out["typeVirtual"] as boolean;
  delete out["typeVirtual"];
  if (out["parent"] === undefined && out["kit"] != null && typeof out["kit"] === "object" && !Array.isArray(out["kit"])) {
    const k = out["kit"] as JsonObject;
    if (typeof k["id"] === "string") out["parent"] = { id: k["id"] as string };
  }
  delete out["kit"];
  return out;
}

//#region 🔌KitGraphqlReadSelections
const KIT_GQL_TYPE_SHALLOW_FIELDS =
  "id hash name description icon image unit created updated connectors { edges { node { id } } }";
const KIT_GQL_DESIGN_SHALLOW_FIELDS =
  "id hash name description icon image unit created updated pieces { edges { node { id } } } connections { edges { node { id } } }";
const KIT_GQL_TYPE_METADATA_FIELDS = "id hash name description icon image unit created updated";
const KIT_GQL_DESIGN_METADATA_FIELDS =
  "id hash name description icon image unit created updated pieces { edges { node { id } } } connections { edges { node { id } } }";
/** @emoji 🧾 Relay `Kit.types { edges { node { … } } }` fragment body (fields live on {@link Type} nodes). */
function kitGqlKitTypesRelay(innerOnTypeNode: string): string {
  return `types { edges { node { ${innerOnTypeNode} } } }`;
}
/** @emoji 🧾 Relay `Kit.designs { edges { node { … } } }` fragment body (fields live on {@link Design} nodes). */
function kitGqlKitDesignsRelay(innerOnDesignNode: string): string {
  return `designs { edges { node { ${innerOnDesignNode} } } }`;
}
/** @emoji 🧾 Relay `Kit.authors { edges { node { … } } }` fragment body. */
function kitGqlKitAuthorsRelay(innerOnAuthorNode: string): string {
  return `authors { edges { node { ${innerOnAuthorNode} } } }`;
}
//#endregion 🔌KitGraphqlReadSelections

function kitGraphqlKitShallowRoot(row: JsonObject): JsonObject {
  const s = row["shallow"];
  if (s != null && typeof s === "object" && !Array.isArray(s)) return s as JsonObject;
  return {} as JsonObject;
}

function kitGraphqlShallowPacketsFromArray(arr: JsonValue | undefined): readonly JsonObject[] {
  if (!Array.isArray(arr)) return [];
  const out: JsonObject[] = [];
  for (const el of arr) {
    if (el != null && typeof el === "object" && !Array.isArray(el)) {
      const o = el as JsonObject;
      const sh = o["shallow"];
      if (sh != null && typeof sh === "object" && !Array.isArray(sh)) out.push(sh as JsonObject);
      else out.push(o);
    }
  }
  return out;
}

function kitGraphqlExtractNestedMetadata(arr: JsonValue | undefined): readonly JsonObject[] {
  if (!Array.isArray(arr)) return [];
  const out: JsonObject[] = [];
  for (const el of arr) {
    if (el != null && typeof el === "object" && !Array.isArray(el)) {
      const m = (el as JsonObject)["metadata"];
      if (m != null && typeof m === "object" && !Array.isArray(m)) out.push(m as JsonObject);
    }
  }
  return out;
}

/** @emoji 🧾 GraphQL JSON uses `null` for absent scalars; Zod `.optional()` expects omission — drop top-level `null` entries before parse. */
function __stripTopLevelJsonNulls(row: JsonObject | KitJsonObjectDto): JsonObject {
  const out = __mutableJsonObjectCopy(row);
  for (const k of Object.keys(out)) {
    if (out[k] === null) delete out[k];
  }
  return out;
}

/** @emoji 🧾 JSON, `KitJsonTreeDto`, or typed bus {@link KitEvent} in subscription and lifecycle helpers. */

/** @emoji 🧾 Narrows subscription payloads to semio kit command lifecycle rows. */
export function isKitCommandLifecycleEvent(event: unknown): event is KitCommandLifecycleEvent {
  if (event == null || typeof event !== "object" || Array.isArray(event)) return false;
  const c = (event as KitJsonObjectDto)["semioKitCommand"];
  if (c == null || typeof c !== "object" || Array.isArray(c)) return false;
  const v = c as JsonObject;
  return typeof v["requestId"] === "string" && typeof v["commandKind"] === "string" && typeof v["phase"] === "string";
}

function __normalizeTopLevelKitEventJson(raw: unknown): KitJsonTreeDto | null {
  if (raw === "Changed") return { Changed: null } as KitJsonTreeDto;
  if (raw === "ValidationInvalidated") return { ValidationInvalidated: null } as KitJsonTreeDto;
  if (raw == null) return null;
  return raw as KitJsonTreeDto;
}

export function normalizeKitEventFromSubscription(raw: unknown): KitEvent | undefined {
  let routed = raw;
  if (routed != null && typeof routed === "object" && !Array.isArray(routed)) {
    const env = routed as JsonObject;
    if (typeof env["kind"] === "string") {
      const k = env["kind"] as string;
      const pl = env["payload"];
      if (k === "operationSucceeded" || k === "changed" || k === "kitMutation") {
        routed = pl !== undefined ? pl : routed;
      }
    }
  }
  const raw0 = __normalizeTopLevelKitEventJson(routed);
  if (raw0 == null) return undefined;
  if (typeof raw0 === "string") return undefined;
  if (typeof raw0 !== "object" || Array.isArray(raw0)) return undefined;
  const top = raw0 as JsonObject;
  const lifecycleWrapper: KitJsonTreeDto = top["semioKitCommand"] !== undefined ? raw0 : top["SemioKitCommand"] !== undefined ? { semioKitCommand: top["SemioKitCommand"] as KitJsonTreeDto } : raw0;
  const w = isJsonObjectNode(lifecycleWrapper) ? lifecycleWrapper : null;
  const command = w?.["semioKitCommand"];
  if (isKitCommandLifecycleEvent({ semioKitCommand: command } as KitEvent)) {
    if (command == null || typeof command !== "object" || Array.isArray(command)) return undefined;
    const value = command as JsonObject;
    const requestIdRaw = value["requestId"];
    if (typeof requestIdRaw !== "string" || typeof value["commandKind"] !== "string" || typeof value["phase"] !== "string") return undefined;
    const errRaw = value["error"];
    const error = errRaw != null && typeof errRaw === "object" && !Array.isArray(errRaw) ? normalizeRustSetError(errRaw as JsonValue) : undefined;
    return {
      semioKitCommand: {
        requestId: requestIdRaw,
        commandKind: value["commandKind"] as string,
        phase: value["phase"] as KitCommandLifecyclePhase,
        result: (value["result"] as KitJsonTreeDto | undefined) ?? undefined,
        error,
      },
    };
  }
  return raw0 as KitEvent;
}

type KitGraphqlHandle = { execute(requestJson: string): Promise<string> };

async function kitGraphqlRun(handle: KitGraphqlHandle, body: { query: string; variables?: GraphQlVariables; operationName?: string }, timeoutMs?: number): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
  const json = await withTimeout(handle.execute(JSON.stringify(body)), timeoutMs ?? 0, "graphql");
  return parseJsonValue(json) as KitGraphqlResponseEnvelope<JsonValue>;
}

/**
 * @emoji 🌐 Typed GraphQL execute: same transport as {@link kitGraphqlRun} with a parametric `data` root for call sites.
 * @typeParam TData GraphQL `data` object shape (e.g. {@link KitGraphqlDataKitScopedFullDto} for {@link KIT_SCOPED_FULL_DTO_QUERY}).
 */
export async function kitGraphqlRunTyped<TData extends JsonValue>(
  handle: { execute(requestJson: string): Promise<string> },
  body: { query: string; variables?: GraphQlVariables; operationName?: string },
  timeoutMs?: number,
): Promise<KitGraphqlResponseEnvelope<TData>> {
  return (await kitGraphqlRun(handle, body, timeoutMs)) as KitGraphqlResponseEnvelope<TData>;
}

// #endregion 🧰GraphqlUtil

// #region 🪜Transport

type WasmExecuteFn = (requestJson: string) => Promise<string>;
type WasmSubscribeFn = (requestJson: string, onEvent: (eventJson: string) => void) => Promise<void>;

/** @internal Used only when `globalThis.Worker` is missing (e.g. Node vitest); browser builds always use {@link WorkerStringTransport}. */
class InlineWasmTransport {
  constructor(
    private readonly handle: {
      execute: WasmExecuteFn;
      subscribe: WasmSubscribeFn;
      free?: () => void;
    },
  ) {}
  /** Returns the **complete JSON** GraphQL response document (one `{ "data": ..., "errors": ... }`). */
  async execute(requestJson: string): Promise<string> {
    return await this.handle.execute(requestJson);
  }
  /** Streams subscription events as **complete JSON** documents (one full GraphQL response per event). */
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

/** @emoji 🧵 `Worker` `error` events are often `ErrorEvent` with empty `message` when a module script fails to load (e.g. Vite down → net::ERR_CONNECTION_REFUSED). */
function describeWorkerThreadError(ev: Event): string {
  if (ev instanceof ErrorEvent) {
    const parts: string[] = [];
    if (ev.message) parts.push(ev.message);
    if (ev.error instanceof Error) parts.push(ev.error.message);
    else if (ev.error) parts.push(String(ev.error));
    if (ev.filename) parts.push(`at ${ev.filename}:${ev.lineno}:${ev.colno}`);
    if (parts.length) return parts.join(" — ");
  }
  return "worker script or module failed to load (if the Vite dev server stopped, run `npm run dev` in the sketchpad package and hard-refresh)";
}

class WorkerStringTransport {
  private worker: Worker;
  private nextSerial = 0;

  constructor(worker: Worker) {
    this.worker = worker;
  }

  /** @emoji 🧵 Resolves on `ready`; rejects fast on worker-thread `error` (e.g. `@semio/rs-wasm` not resolvable from Blob worker) instead of waiting the full timeout. */
  init(dto: KitFullDto): Promise<void> {
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

  /** Returns the **complete JSON** GraphQL response document (one `{ "data": ..., "errors": ... }`). */
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
        if (m.op === "result" && typeof m.json === "string") {
          result = m.json;
        }
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

  /** Streams subscription events as **complete JSON** documents (one full GraphQL response per event). */
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

// #endregion 🪜Transport

// #region 📦KitStore

/** @internal Resolves `semio_bg.wasm` for Node / Vitest when `import.meta.url` is not adjacent to `semio/rs/pkg` (e.g. bundled `semio/react` tests). */
async function __readSemioWasmBytesFromMonorepoCandidates(): Promise<Uint8Array | undefined> {
  try {
    const fs = await import("node:fs/promises");
    const path = await import("node:path");
    const envPath = typeof process !== "undefined" && process.env ? (process.env["SEMIO_WASM_BG_PATH"] ?? process.env["SEMIO_RS_WASM_PATH"]) : undefined;
    const candidates: string[] = [];
    if (typeof envPath === "string" && envPath.trim().length) candidates.push(envPath.trim());
    try {
      const { fileURLToPath } = await import("node:url");
      candidates.push(fileURLToPath(new URL("../rs/pkg/semio_bg.wasm", import.meta.url)));
    } catch {
      /* Vitest may bundle this module with a synthetic `import.meta.url` that is not beside `semio/rs/pkg`. */
    }
    if (typeof process !== "undefined" && typeof process.cwd === "function") {
      let dir = process.cwd();
      for (let i = 0; i < 16; i++) {
        candidates.push(path.join(dir, "semio", "rs", "pkg", "semio_bg.wasm"));
        candidates.push(path.join(dir, "rs", "pkg", "semio_bg.wasm"));
        const parent = path.dirname(dir);
        if (parent === dir) break;
        dir = parent;
      }
    }
    for (const wasmPath of candidates) {
      try {
        return await fs.readFile(wasmPath);
      } catch {
        /* try next candidate */
      }
    }
  } catch {
    /* fs / path unavailable */
  }
  return undefined;
}

/** @emoji 🔗 Smoke query: `Query.wip` + `Graph.theKit` (`semio/graphql/target.schema.graphql`). */
export const KIT_SESSION_QUERY_ENTRY = `query { wip { id theKit { id } } }` as const;

/** @emoji 🔗 Multiplexed kit stream: `Subscription.event: Json!` (`semio/graphql/target.schema.graphql`). */
export const KIT_EVENT_STREAM_SUBSCRIPTION = `subscription { event }` as const;

/** @emoji 🧾 Envelope kinds inside {@link KIT_EVENT_STREAM_SUBSCRIPTION} (Worker D contract). */
export type KitEventEnvelope = Readonly<{ readonly kind: string; readonly payload?: unknown }>;

/** @emoji 🔗 @deprecated Prefer {@link KIT_EVENT_STREAM_SUBSCRIPTION}; correlator listens via `event` envelope. */
export const KIT_COMMAND_SUCCEEDED_SUBSCRIPTION = KIT_EVENT_STREAM_SUBSCRIPTION;

/** @emoji 🔗 @deprecated Prefer {@link KIT_EVENT_STREAM_SUBSCRIPTION}. */
export const KIT_OPERATION_FAILED_SUBSCRIPTION = KIT_EVENT_STREAM_SUBSCRIPTION;

/**
 * @emoji 🔗 Main-line full kit DTO query.
 */
export const KIT_SCOPED_FULL_DTO_QUERY = `query { wip { theKit { kit { fullSnapshot } } } }` as const;

/** @emoji 🧾 Typed `data` shape for {@link KIT_SCOPED_FULL_DTO_QUERY} on {@link theKitReadPoint}. */
export type KitGraphqlDataKitScopedFullDto = Readonly<{
  readonly wip: Readonly<{ readonly theKit: Readonly<{ readonly kit: Readonly<{ readonly fullSnapshot: KitJsonTreeDto }> | null }> | null }> | null;
}>;

//#region 🧱StorePrimitives

/** @emoji 📛 Write lane status for {@link StoreCommand} (stable object identities for triads). */
export type WriteStatus =
  | { kind: "readonly"; pending: 0; lastError?: SetError }
  | { kind: "idle"; pending: 0; lastError?: SetError }
  | { kind: "pending"; pending: number; lastError?: SetError }
  | { kind: "error"; pending: 0; lastError: SetError };

/** @emoji 🧾 Frozen idle — stable identity for `useSyncExternalStore` snapshots. */
export const WRITE_STATUS_IDLE: WriteStatus = Object.freeze({ kind: "idle", pending: 0 });
/** @emoji 🧾 Frozen readonly — stable identity when the lane is read-only. */
export const WRITE_STATUS_READONLY: WriteStatus = Object.freeze({ kind: "readonly", pending: 0 });
/** @emoji 🪪 Frozen pending for rs-backed mutations — stable identity while a command is in flight. */
export const WRITE_STATUS_PENDING: WriteStatus = Object.freeze({ kind: "pending", pending: 1 });

/** @emoji 🪪 Same semantic {@link WriteStatus} → reuse prior reference (avoids React #520 from fresh object identity). */
export function writeStatusEquivalent(a: WriteStatus, b: WriteStatus): boolean {
  if (a === b) return true;
  if (a.kind !== b.kind) return false;
  if (a.kind === "pending" && b.kind === "pending") {
    return a.pending === b.pending && a.lastError === b.lastError;
  }
  if (a.kind === "error" && b.kind === "error") {
    return a.lastError === b.lastError;
  }
  return true;
}

/** @emoji 🧾 Pushes live values into {@link StoreField}; return cleanup unsubscriber. */
export type StoreFieldSource<T> = (push: (next: T) => void) => Unsubscribe;

/** @emoji 📥 Read-only typed mirror; values enter only through the optional `source` `push` callback. */
export class StoreField<T> {
  private readonly value$: BehaviorSubject<T>;
  private unsubSource: Unsubscribe | null = null;
  constructor(initial: T, source?: StoreFieldSource<T>) {
    this.value$ = new BehaviorSubject(initial);
    if (source) this.unsubSource = source((next) => this.value$.next(next));
  }
  subscribe = (h: () => void): Unsubscribe => {
    const s = this.value$.subscribe({ next: () => h() });
    return () => {
      s.unsubscribe();
    };
  };
  getSnapshot = (): T => this.value$.getValue();
  dispose(): void {
    if (this.unsubSource) {
      try {
        this.unsubSource();
      } catch {
        /* ignore */
      }
      this.unsubSource = null;
    }
    try {
      this.value$.complete();
    } catch {
      /* ignore */
    }
  }
}

/** @emoji 📝 Async side-effect carrier with render-ready {@link WriteStatus} (domain-agnostic). */
export class StoreCommand<TArgs> {
  readonly status: StoreField<WriteStatus>;
  private pushStatus!: (next: WriteStatus) => void;
  private lastError: SetError | null = null;
  constructor(private readonly exec: (args: TArgs) => Promise<SetResult>) {
    this.status = new StoreField<WriteStatus>(WRITE_STATUS_IDLE, (push) => {
      this.pushStatus = push;
      return () => {};
    });
  }
  readonly run = async (args: TArgs): Promise<SetResult> => {
    this.pushStatus(WRITE_STATUS_PENDING);
    const r = await this.exec(args);
    if (r.ok) {
      this.lastError = null;
      this.pushStatus(WRITE_STATUS_IDLE);
    } else {
      const cached = this.lastError;
      const lastError = cached && cached.kind === r.error.kind && cached.message === r.error.message ? cached : r.error;
      this.lastError = lastError;
      const cur = this.status.getSnapshot();
      const next: WriteStatus = { kind: "error", pending: 0, lastError };
      if (!writeStatusEquivalent(cur, next)) this.pushStatus(next);
    }
    return r;
  };
  dispose(): void {
    this.status.dispose();
  }
}

/** @emoji 🚦 Generic request-id ↔ Promise-resolver correlator for mutations that complete on the rs output stream. */
export class RequestCorrelator {
  private readonly resolvers = new Map<string, (r: SetResult) => void>();
  private readonly pending = new Map<string, SetResult>();
  constructor(private readonly timeoutMs: number) {}
  await(requestId: string): Promise<SetResult> {
    const buffered = this.pending.get(requestId);
    if (buffered) {
      this.pending.delete(requestId);
      return Promise.resolve(buffered);
    }
    return new Promise<SetResult>((resolve) => {
      const t = setTimeout(() => {
        if (!this.resolvers.has(requestId)) return;
        this.resolvers.delete(requestId);
        resolve({
          ok: false,
          error: { kind: "Timeout", message: `request ${requestId}: timed out waiting for rs operation stream` },
        });
      }, this.timeoutMs);
      this.resolvers.set(requestId, (r) => {
        clearTimeout(t);
        resolve(r);
      });
    });
  }
  resolve(requestId: string, r: SetResult): void {
    const fn = this.resolvers.get(requestId);
    if (fn) {
      fn(r);
      this.resolvers.delete(requestId);
      return;
    }
    this.pending.set(requestId, r);
    setTimeout(() => {
      const cur = this.pending.get(requestId);
      if (cur === r) this.pending.delete(requestId);
    }, 10_000);
  }
  disposeAll(reason = "KitStore disposed"): void {
    for (const [rid, fn] of this.resolvers.entries()) {
      fn({ ok: false, error: { kind: "Disposed", message: `${reason} (${rid})` } });
    }
    this.resolvers.clear();
    this.pending.clear();
  }
}

//#endregion 🧱StorePrimitives

/** @emoji 🧾 Uniform `renameKit` args; scope is unused (session `theKit` path). */
export type RenameKitCommandArgs = { readonly scope: Record<string, never>; readonly input: { readonly name: string } };

/**
 * @emoji 🌐 Single kit control plane: GraphQL strings over one dedicated `Worker` running `semio/rs` WASM (`KitStoreHandle`).
 */
export class KitStore {
  private readonly timeoutMs: number;
  private transport!: WorkerStringTransport | InlineWasmTransport;
  private readonly fanout = new Subject<KitEvent>();
  private readonly invalidations = new Subject<void>();
  private gqlLoopRunning = false;
  private disposed = false;
  /** @emoji 🧭 Active unsaved {@link Change} id for `VersionCommandInput.unsavedChange(id)`. */
  private kitWriteChangeId: string | null = null;

  /** @emoji 🧭 Active {@link KitReadPoint} for scoped kit reads (see {@link WasmKitStoreClient.setKitReadPoint}). */
  private activeReadPoint: KitReadPoint = theKitReadPoint;

  /** @emoji 🧾 Cached {@link StoreField}s for {@link kitField} (disposed with store). */
  private readonly kitFields = new Map<string, StoreField<unknown>>();

  private readonly correlator: RequestCorrelator;
  /** @emoji 🪪 Scoped `session { theKit { unsavedChange { kit { rename } } } }` + version-level save. */
  readonly renameKit: StoreCommand<RenameKitCommandArgs>;
  private subscriptionLoopStarted = false;

  private constructor(timeoutMs: number, transport: WorkerStringTransport | InlineWasmTransport) {
    this.timeoutMs = timeoutMs;
    this.transport = transport;
    this.correlator = new RequestCorrelator(timeoutMs);
    this.renameKit = new StoreCommand<RenameKitCommandArgs>(async ({ input }) => {
      try {
        const changeId = await this.ensureKitWriteChangeId();
        const body = __scopedKitMutationBody(changeId, `r: rename(newName: ${__gqlStr(input.name)})`);
        kitGraphqlData(await this.gqlRun(body));
        await this.apiTheKitSave();
        try {
          this.invalidations.next();
        } catch {
          /* ignore */
        }
        return { ok: true };
      } catch (e) {
        return { ok: false, error: { kind: "Internal", message: String(e) } };
      }
    });
  }

  /** @emoji 🪪 Live kit name via {@link kitField}. */
  get kitName(): StoreField<string> {
    return this.kitField<string>("kit:name", {
      innerOnKit: "name",
      parse: (frag) => String(frag["name"] ?? ""),
      initial: "",
    });
  }

  /**
   * @emoji 🧾 Live {@link StoreField} over `Query.wip.theKit(at:)` + `innerOnKit` GraphQL selection (cache keyed by {@link cacheKey}).
   * Invalidates with {@link invalidations}; reads {@link activeReadPoint} each fetch.
   */
  kitField<T>(
    cacheKey: string,
    spec: {
      /** Declarations after `$at`, e.g. `$typeId: Id!` (omit when selection uses only `$at`). */
      extraVariableDecl?: string;
      extraVariables?: GraphQlVariables;
      innerOnKit: string;
      parse: (kitFragment: JsonObject) => T;
      initial: T;
    },
  ): StoreField<T> {
    this.ensureAlive();
    const hit = this.kitFields.get(cacheKey);
    if (hit) return hit as StoreField<T>;
    const sf = new StoreField<T>(spec.initial, (push) => {
      const refetch = async () => {
        try {
          const point = this.activeReadPoint;
          const extraDecl = spec.extraVariableDecl?.trim();
          let query: string;
          let variables: GraphQlVariables;
          if (isTheKitReadPoint(point)) {
            const varHeader = extraDecl ? (`${extraDecl}` as const) : "";
            const headerPart = varHeader ? `(${varHeader})` : "";
            query = `query SemioMatKit${headerPart} { wip { theKit { kit { ${spec.innerOnKit} } } } }`;
            variables = { ...(spec.extraVariables ?? {}) } as GraphQlVariables;
          } else {
            const varHeader = extraDecl ? (`${extraDecl}` as const) : "";
            const headerPart = varHeader ? `(${varHeader})` : "";
            const selected = kitSessionWipStoreSelect(point, spec.innerOnKit);
            query = extraDecl ? selected.query.replace("query KitSessionWipStore", `query SemioMatKit${headerPart}`) : selected.query.replace("query KitSessionWipStore", "query SemioMatKit");
            variables = { ...selected.variables, ...(spec.extraVariables ?? {}) } as GraphQlVariables;
          }
          const data = kitGraphqlData(await this.gqlRun({ query, variables })) as JsonValue;
          const frag = gqlDataSessionWipKitStore(data, point) ?? ({} as JsonObject);
          push(spec.parse(frag));
        } catch {
          /* keep last */
        }
      };
      void refetch();
      const sub = this.invalidations.subscribe({ next: () => void refetch() });
      return () => sub.unsubscribe();
    });
    this.kitFields.set(cacheKey, sf as StoreField<unknown>);
    return sf;
  }

  static async open(initialKit: KitFullDto, opts?: KitStoreOpenOptions): Promise<KitStore> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const wasmSpecifier = opts?.wasmSpecifier ?? (globalThis as { __SEMIO_WASM_SPECIFIER__?: string }).__SEMIO_WASM_SPECIFIER__ ?? "@semio/rs-wasm";
    const dto = JSON.parse(JSON.stringify(initialKit)) as KitFullDto;
    /** Vitest may expose `Worker` (e.g. jsdom); blob worker still `fetch`es `.wasm` — prefer inline init when Vitest is active. */
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
        const ks = new KitStore(timeoutMs, wt);
        await withTimeout(ks.warmGraphqlRead(), timeoutMs, "graphql");
        void ks.startSubscriptionLoop();
        return ks;
      } catch (workerErr) {
        /** @emoji 🧵 Blob worker can't resolve `@semio/rs-wasm` bare specifier → fall back to inline WASM on the main thread (still real rust authority). */
        console.warn("[semio/js] dedicated WASM worker init failed; falling back to inline main-thread WASM", workerErr);
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
      const net = /Failed to fetch|fetch|ERR_CONNECTION_REFUSED|LOAD_FAILED|network/i.test(base);
      const hint = net
        ? " The Vite dev server may have stopped (restore with `npm run dev` in semio/sketchpad, then hard-refresh)."
        : "";
      throw new Error(`Failed to load @semio/rs-wasm (inline path): ${base}.${hint}`);
    }
    if (typeof mod.default === "function") {
      if (wasmBytesPre) await mod.default({ module_or_path: wasmBytesPre });
      else await mod.default();
    } else await mod.default();
    if (typeof mod.boot === "function") mod.boot();
    const handleUnknown = mod.KitStoreHandle.create(dto as object);
    const handle = handleUnknown instanceof Promise ? await handleUnknown : handleUnknown;
    if (handle == null || typeof (handle as KitGraphqlHandle).execute !== "function") {
      throw new Error("KitStoreHandle.create did not return an object with execute()");
    }
    const t = new InlineWasmTransport(handle as { execute: WasmExecuteFn; subscribe: WasmSubscribeFn; free?: () => void });
    const ks = new KitStore(timeoutMs, t);
    await withTimeout(ks.warmGraphqlRead(), timeoutMs, "graphql");
    void ks.startSubscriptionLoop();
    return ks;
  }

  private graphqlHandle(): KitGraphqlHandle {
    return { execute: (requestJson: string) => this.transport.execute(requestJson) };
  }

  /** @emoji 🧭 Pin materialization reads to `at`; bumps {@link invalidations}. */
  setReadPoint(next: KitReadPoint): void {
    this.ensureAlive();
    this.activeReadPoint = next;
    try {
      this.invalidations.next();
    } catch {
      /* ignore */
    }
  }

  private ensureAlive(): void {
    if (this.disposed) throw new Error("KitStore disposed");
  }

  private async gqlRun(body: { query: string; variables?: GraphQlVariables; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    this.ensureAlive();
    return kitGraphqlRun(this.graphqlHandle(), body, this.timeoutMs);
  }

  /** @emoji 🧾 `Query.wip.theKit` materialized {@link KitStore} selection for read helpers. */
  private async gqlKitReadOnlyScope(scope: KitReadPoint, innerOnKitStore: string): Promise<JsonObject> {
    const { query, variables } = kitSessionWipStoreSelect(scope, innerOnKitStore);
    const data = kitGraphqlData(await this.gqlRun({ query, variables })) as JsonValue;
    return gqlDataSessionWipKitStore(data, scope) ?? ({} as JsonObject);
  }

  /** @emoji 🧾 Session/wip {@link KitStore} JSON fragment for a read scope (used by live reads such as connector colors). */
  async kitStoreGraphqlFieldsForReadPoint(scope: KitReadPoint, innerOnKitStore: string): Promise<JsonObject> {
    return this.gqlKitReadOnlyScope(scope, innerOnKitStore);
  }

  /** @emoji 🧾 Runs a session/wip/store query merging `extra` variables (e.g. `id` for `design(id: $id)`). */
  private async gqlRunSessionWipStore(scope: KitReadPoint, innerOnKitStore: string, extra: GraphQlVariables = {} as GraphQlVariables): Promise<JsonValue> {
    const { query, variables } = kitSessionWipStoreSelect(scope, innerOnKitStore);
    return kitGraphqlData(await this.gqlRun({ query, variables: { ...variables, ...extra } })) as JsonValue;
  }

  /** @emoji 📣 Subscribe to kit GraphQL subscription events (RxJS-free public surface). */
  subscribe(handler: (event: KitEvent) => void): Unsubscribe {
    const sub = this.fanout.subscribe({ next: handler });
    return () => {
      sub.unsubscribe();
    };
  }

  /** @emoji 📣 Subscribe only when {@link KitEventFilter} returns true. */
  subscribeFiltered(filterFn: KitEventFilter, handler: (event: KitEvent) => void): Unsubscribe {
    const sub = this.fanout.pipe(filter(filterFn)).subscribe({ next: handler });
    return () => {
      sub.unsubscribe();
    };
  }

  /** @emoji 📣 Fires only after coalescing dto `Changed` / synthetic `{ Changed: null }` rows. */
  subscribeRootInvalidation(handler: () => void): Unsubscribe {
    return this.subscribeFiltered(
      (ev) => typeof ev === "object" && ev !== null && "Changed" in ev && (ev as { Changed?: null }).Changed === null,
      () => handler(),
    );
  }

  /** @emoji 📣 Kit command lifecycle scalar rows (`semioKitCommand` / `SemioKitCommand`). */
  subscribeSemioKitCommandLifecycle(handler: (row: KitCommandLifecycleEvent["semioKitCommand"]) => void): Unsubscribe {
    return this.subscribeFiltered(
      (ev) => isKitCommandLifecycleEvent(ev),
      (ev) => handler((ev as KitCommandLifecycleEvent).semioKitCommand),
    );
  }

  //#region 🪪KitNameAndRename

  /** @emoji 🧾 GraphQL-backed read; pushes `parse(data)` initially and on each {@link invalidations} tick. */
  private query<T>(body: string, parse: (data: JsonValue) => T, initial: T): StoreField<T> {
    return new StoreField<T>(initial, (push) => {
      const refetch = async () => {
        try {
          const data = kitGraphqlData(await this.gqlRun({ query: `query { ${body} }` })) as JsonValue;
          push(parse(data));
        } catch {
          /* keep last value; read errors never leak to UI — only writes carry status */
        }
      };
      void refetch();
      const sub = this.invalidations.subscribe({ next: () => void refetch() });
      return () => sub.unsubscribe();
    });
  }

  private async apiTheKitSave(): Promise<void> {
    kitGraphqlData(await this.gqlRun({ query: `mutation { session { theKit { save } } }` }));
    this.kitWriteChangeId = null;
  }

  /** @emoji 🧾 Ensures {@link kitWriteChangeId} via `VersionCommandInput.startNewChange`. */
  private async ensureKitWriteChangeId(): Promise<string> {
    if (this.kitWriteChangeId) return this.kitWriteChangeId;
    const data = kitGraphqlData(await this.gqlRun({ query: `mutation { session { theKit { startNewChange } } }` })) as JsonObject;
    const sess = data["session"] as JsonObject | undefined;
    const tk = sess?.["theKit"] as JsonObject | undefined;
    const cid = String(tk?.["startNewChange"] ?? "");
    if (cid === "") throw new Error("startNewChange: empty change id");
    this.kitWriteChangeId = cid;
    return cid;
  }

  /** @emoji 🪪 Cacheless Promise read of kit name via GraphQL into rs (honours {@link activeReadPoint}). */
  async readKitName(): Promise<string> {
    this.ensureAlive();
    const point = this.activeReadPoint;
    const { query, variables } = kitSessionWipStoreSelect(point, "name");
    const data = kitGraphqlData(await this.gqlRun({ query, variables })) as JsonValue;
    const frag = gqlDataSessionWipKitStore(data, point);
    return String(frag?.["name"] ?? "");
  }

  private mapOperationFailedToSetError(kind: string, message: string): SetError {
    const k = kind.trim();
    if (k === "Invalid") return { kind: "InvalidValue", message };
    if (k === "NotFound") return { kind: "NotFound", message };
    return { kind: "Internal", message };
  }

  private dispatchCommandSucceededPayload(payload: unknown): void {
    if (payload == null || typeof payload !== "object" || Array.isArray(payload)) return;
    const rid = String((payload as JsonObject)["requestId"] ?? "");
    if (rid !== "") this.correlator.resolve(rid, { ok: true });
  }

  private dispatchCommandSucceeded(data: { readonly commandSucceeded?: { readonly requestId?: string } | null } | null): void {
    this.dispatchCommandSucceededPayload(data?.commandSucceeded ?? null);
  }

  private dispatchOperationFailed(data: {
    readonly operationFailed?: { readonly kind?: string; readonly message?: string; readonly requestId?: string | null } | null;
  }): void {
    const failed = data.operationFailed;
    if (failed == null || typeof failed !== "object") return;
    const ridRaw = failed.requestId;
    const rid = ridRaw != null && String(ridRaw).length > 0 ? String(ridRaw) : "";
    if (rid === "") return;
    const err = this.mapOperationFailedToSetError(String(failed.kind ?? "Internal"), String(failed.message ?? "operationFailed"));
    this.correlator.resolve(rid, { ok: false, error: err });
  }

  private dispatchSubscriptionGraphqlData(data: JsonObject | null | undefined): void {
    if (data == null) return;
    if (data["event"] !== undefined) {
      this.dispatchMultiplexedSubscriptionEvent(data["event"] as JsonValue);
      return;
    }
    if (data["commandSucceeded"] !== undefined) {
      this.dispatchCommandSucceededPayload(data["commandSucceeded"]);
    }
    if (data["operationFailed"] !== undefined) {
      this.dispatchOperationFailed({ operationFailed: data["operationFailed"] as { kind?: string; message?: string; requestId?: string | null } });
    }
    const legacyOp = data["operationSucceeded"];
    if (legacyOp !== undefined) {
      this.dispatchMultiplexedSubscriptionEvent(legacyOp as JsonValue);
    }
  }

  /** @emoji 🧾 `Subscription.event` JSON envelope (`kind` + `payload`) or legacy stream rows. */
  private dispatchMultiplexedSubscriptionEvent(ev: JsonValue): void {
    if (ev != null && typeof ev === "object" && !Array.isArray(ev) && typeof (ev as JsonObject)["kind"] === "string") {
      const o = ev as JsonObject;
      const k = o["kind"] as string;
      const pl = o["payload"];
      if (k === "commandSucceeded") {
        this.dispatchCommandSucceededPayload(pl);
        return;
      }
      if (k === "operationFailed") {
        this.dispatchOperationFailed({ operationFailed: pl as { kind?: string; message?: string; requestId?: string | null } });
        return;
      }
      if (k === "operationSucceeded" || k === "changed" || k === "kitMutation") {
        const inner = pl !== undefined ? pl : ev;
        const n = normalizeKitEventFromSubscription(inner);
        if (n) this.fanout.next(n);
        else this.fanout.next(inner as KitEvent);
        try {
          this.invalidations.next();
        } catch {
          /* ignore */
        }
        return;
      }
    }
    const n = normalizeKitEventFromSubscription(ev);
    if (n) this.fanout.next(n);
    else this.fanout.next(ev as KitEvent);
    try {
      this.invalidations.next();
    } catch {
      /* ignore */
    }
  }

  //#endregion 🪪KitNameAndRename

  private startSubscriptionLoop(): void {
    if (this.gqlLoopRunning) return;
    this.gqlLoopRunning = true;
    void this.transport
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

  async dispose(): Promise<void> {
    if (this.disposed) return;
    this.disposed = true;
    this.correlator.disposeAll();
    for (const f of this.kitFields.values()) {
      try {
        f.dispose();
      } catch {
        /* ignore */
      }
    }
    this.kitFields.clear();
    this.renameKit.dispose();
    try {
      this.invalidations.complete();
    } catch {
      /* ignore */
    }
    this.fanout.complete();
    this.transport.dispose();
  }

  /** @internal One `Query.wip.theKit { kit { fullSnapshot } }` read to verify GraphQL after WASM init (no local kit cache). */
  private async warmGraphqlRead(): Promise<void> {
    await this.readKitSnapshotForReadPoint(theKitReadPoint);
  }

  /** @emoji 🧾 Full DTO for a {@link KitReadPoint} via target-schema version `kit.fullSnapshot`; sole full-kit read path (no WASM `snapshot` fallback). */
  async readKitSnapshotForReadPoint(scope: KitReadPoint): Promise<KitFullDto> {
    this.ensureAlive();
    const { query, variables } = kitSessionWipStoreSelect(scope, "fullSnapshot");
    const data = kitGraphqlData(
      await kitGraphqlRunTyped<JsonValue>(this.graphqlHandle(), { query, variables }, this.timeoutMs),
    ) as JsonValue;
    const raw = gqlDataSessionWipKitStore(data, scope)?.["fullSnapshot"] as JsonValue | undefined;
    const j: JsonValue = typeof raw === "string" ? parseJsonValue(raw) : raw == null ? null : (raw as JsonValue);
    if (j == null || typeof j !== "object" || Array.isArray(j)) {
      throw new Error("kitGraphql: fullSnapshot missing or not an object for the given read scope");
    }
    return semioCoerceKitFullDtoFromJson(j as KitJsonTreeDto);
  }

  /** @emoji 🧾 Main-line live kit DTO from `Query.wip.theKit.kit` (`fullSnapshot` / materialized JSON). */
  async theKit(): Promise<KitFullDto> {
    return this.readKitSnapshotForReadPoint(theKitReadPoint);
  }

  async readAt(checkpointId: string): Promise<KitFullDto> {
    const idArg = checkpointId.trim();
    if (idArg === "") return this.readKitSnapshotForReadPoint(theKitReadPoint);
    return this.readKitSnapshotForReadPoint({ checkpoint: { checkpointId: idArg } });
  }

  async vcsState(): Promise<JsonObject> {
    const data = kitGraphqlData(
      await this.gqlRun({
        query: `query { wip { id theKit { id kit { id } } checkpoints { edges { node { id message } } } alternatives { edges { node { id name } } } } }`,
      }),
    ) as JsonValue;
    const wipRaw = (data as { wip?: JsonObject | null }).wip;
    const wip = __normalizeWipVcsRelayFields(wipRaw);
    return { wip } as JsonObject;
  }

  /** @emoji 🧾 Undo stack is owned by `semio/rs` edit log; JS does not read a draft cursor on the target schema. */
  async canUndo(): Promise<boolean> {
    void (await this.vcsState());
    return false;
  }

  /** @emoji 🧾 Redo stack is owned by `semio/rs` edit log; JS does not read a draft cursor on the target schema. */
  async canRedo(): Promise<boolean> {
    void (await this.vcsState());
    return false;
  }

  /** @emoji 🧾 Maps control-plane canvas `commandKind` + `variables` to a `changeKitCommands` batch (`session.theKit.unsavedChange`). */
  private scopedTransactionInnerFromControl(commandKind: string, variables: JsonObject): JsonObject | null {
    const v = variables;
    const kid = (s: string): KitIdDto => ({ id: s });
    const one = (cmd: ChangeKitCommand): JsonObject => ({ changeKitCommands: { commands: [cmd] } } as JsonObject);
    switch (commandKind) {
      case "changeKitCommands":
        return { changeKitCommands: { commands: (v["commands"] as readonly ChangeKitCommand[] | undefined) ?? [] } } as GraphQlVariables;
      case "clusterPieces":
        return null;
      case "dragPieces":
        return one({
          dragPieces: {
            designId: kid(String(v.designId)),
            pieceIds: (v.pieceIds as readonly string[]) ?? [],
            du: Number(v.du),
            dv: Number(v.dv),
          },
        });
      case "movePieces":
        return one({
          movePieces: {
            designId: kid(String(v.designId)),
            pieceIds: (v.pieceIds as readonly string[]) ?? [],
            gap: Number(v.gap),
            shift: Number(v.shift),
            rise: Number(v.rise),
          },
        });
      case "fixPieces":
        return one({
          fixPieces: { designId: kid(String(v.designId)), pieceIds: (v.pieceIds as readonly string[]) ?? [] },
        });
      case "flattenDesign":
        return one({ flattenDesign: { designId: kid(String(v.designId)) } });
      case "expandDesign":
        return null;
      case "deleteConnection":
        return one({
          changeDesignCommands: {
            designId: kid(String(v.designId)),
            commands: [{ removeConnection: { connectionId: kid(String(v.connectionId)) } }],
          },
        });
      case "changePieceType":
        return one({
          changePieceKind: {
            designId: kid(String(v.designId)),
            pieceId: kid(String(v.pieceId)),
            newTypeId: kid(String(v.newTypeId)),
          },
        });
      case "createHangingPieces":
        return null;
      case "createConnectedPiece":
        return one({
          addChildPieceWithParentConnection: {
            designId: kid(String(v.designId)),
            parentPiece: String(v.parentPiece),
            parentPort: String(v.parentPort),
            childType: String(v.childType),
            childPort: String(v.childPort),
          },
        });
      case "createFixedPiece": {
        const pl = __kitPlaneToBatchInput(v.plane);
        if (!pl) return null;
        const pid = id();
        const piece = {
          id: pid,
          name: "",
          type: kid(String(v.typeId)),
          plane: v.plane as PlaneDto,
          center: { u: 0, v: 0 },
          scale: 1,
          color: "#000000",
          props: [],
          attributes: [],
        } as PieceDto;
        return one({
          changeDesignCommands: {
            designId: kid(String(v.designId)),
            commands: [{ addPiece: { piece } }],
          },
        });
      }
      default:
        return null;
    }
  }

  /** @emoji 🌱 Ensures an unsaved change exists for bundle hosts (`startNewChange`); target schema has no `kitStoreInitializeDefaults` root field. */
  async kitStoreInitializeDefaults(): Promise<string> {
    this.ensureAlive();
    return this.ensureKitWriteChangeId();
  }

  /** @emoji 🌱 `Mutation.session { startAlternative }`; `sourceAlternativeId` is not represented on the target command tree yet. */
  async createAlternativeFromTip(name: string, sourceAlternativeId: string | null): Promise<string> {
    void sourceAlternativeId;
    this.ensureAlive();
    const data = kitGraphqlData(
      await this.gqlRun({
        query: `mutation($n: String!) { session { startAlternative(name: $n) } }`,
        variables: { n: name },
      }),
    ) as JsonObject;
    const altId = String((data["session"] as JsonObject | undefined)?.["startAlternative"] ?? "");
    if (altId === "") throw new Error("createAlternativeFromTip: empty id");
    try {
      this.fanout.next({ Changed: null } as KitEvent);
    } catch {
      /* ignore */
    }
    return altId;
  }

  /** @emoji 📸 Serialize the RS-owned kit store bundle (`schema`, three graphs, checkpoints, changes, edits). */
  async serializeKitStoreBundleJson(): Promise<string> {
    this.ensureAlive();
    const data = kitGraphqlData(await this.gqlRun({ query: `query { kitStoreBundleJson }` })) as JsonObject;
    const json = String(data["kitStoreBundleJson"] ?? "");
    if (json.trim() === "") throw new Error("kitStoreBundleJson: empty response");
    return json;
  }

  /** @emoji 🩻 Hydrate the RS graph from a metabolism-shaped bundle read by the host adapter. */
  async hydrateKitStoreBundleJson(json: string): Promise<void> {
    this.ensureAlive();
    if (json.trim() === "") return;
    kitGraphqlData(await this.gqlRun({ query: `mutation($json: String!) { hydrateKitStoreBundleJson(json: $json) }`, variables: { json } }));
    try {
      this.invalidations.next();
    } catch {
      /* ignore */
    }
  }

  /** @emoji 🟢 Starts a fresh unsaved change (`startNewChange`); sketchpad may call this on input focus. */
  async openKitWriteTransaction(): Promise<{ ok: true; changeId: string } | { ok: false; error: SetError }> {
    this.ensureAlive();
    try {
      const data = kitGraphqlData(await this.gqlRun({ query: `mutation { session { theKit { startNewChange } } }` })) as JsonObject;
      const sess = data["session"] as JsonObject | undefined;
      const tk = sess?.["theKit"] as JsonObject | undefined;
      const cid = String(tk?.["startNewChange"] ?? "");
      if (cid === "") return { ok: false, error: { kind: "Internal", message: "openKitWriteTransaction: empty change id" } };
      this.kitWriteChangeId = cid;
      return { ok: true, changeId: cid };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  /** @emoji 🧾 Runs each {@link ChangeKitCommand} as `session { theKit { unsavedChange(id:) { kit { … } } } } }` (see {@link buildScopedChangeKitMutation}). */
  private async runScopedTransactionBatch(innerTxCommands: readonly JsonObject[]): Promise<readonly JsonObject[]> {
    const changeId = await this.ensureKitWriteChangeId();
    const results: JsonObject[] = [];
    for (const inner of innerTxCommands) {
      if ("changeKitCommands" in inner && inner.changeKitCommands != null && typeof inner.changeKitCommands === "object") {
        const cmds = (inner.changeKitCommands as { commands?: unknown }).commands;
        const list = (cmds as readonly ChangeKitCommand[] | undefined) ?? [];
        for (const cmd of list) {
          const built = buildScopedChangeKitMutation(changeId, cmd);
          if (built == null) {
            throw new Error(`runScopedTransactionBatch: unsupported ChangeKitCommand ${JSON.stringify(Object.keys(cmd as object))}`);
          }
          kitGraphqlData(await this.gqlRun(built));
        }
        results.push({ ok: true, kind: "CHANGE_KIT" } as JsonObject);
        continue;
      }
      throw new Error(`runScopedTransactionBatch: unsupported ${Object.keys(inner).join(",")}`);
    }
    return results;
  }

  /** @emoji 🧭 Exposes the active unsaved change id (or {@link KitStore.setKitWriteScope}). */
  getKitWriteScope(): KitWriteScope | null {
    const c = this.kitWriteChangeId;
    return c ? { changeId: c } : null;
  }

  /** @emoji 🧭 Pins the unsaved change id (`null` clears the local handle only). */
  setKitWriteScope(scope: KitWriteScope | null): void {
    this.kitWriteChangeId = scope?.changeId ?? null;
  }

  /** @emoji ✅ Persists the version lane (`session { theKit { save } }`). */
  async finalizeKitWriteTransaction(): Promise<SetResult> {
    if (!this.kitWriteChangeId) {
      return { ok: false, error: { kind: "Internal", message: "finalizeKitWriteTransaction: no active change" } };
    }
    try {
      await this.apiTheKitSave();
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  /** @emoji ⛔ Drops the local unsaved-change handle without calling save (server-side discard is not on the target command tree). */
  async abortKitWriteTransaction(): Promise<SetResult> {
    if (!this.kitWriteChangeId) {
      return { ok: false, error: { kind: "Internal", message: "abortKitWriteTransaction: no active change" } };
    }
    this.kitWriteChangeId = null;
    return { ok: true };
  }

  /** @emoji 🧾 Top-level `session.backbone` mutations returning {@link SetResult}. */
  private async runBackboneBatchSetResult(kind: string, variables: Readonly<JsonObject>): Promise<SetResult> {
    this.ensureAlive();
    try {
      if (kind === "attachBackbone") {
        const c = (variables.config as BackboneConfig | null | undefined) ?? { Memory: null };
        const cfg: JsonObject =
          "Memory" in c
            ? { memory: { confirm: true } }
            : "Dev" in c
              ? { dev: { path: c.Dev.path } }
              : "Local" in c
                ? { local: { folder: c.Local.folder } }
                : { remote: { url: c.Remote.url, sessionId: c.Remote.sessionId } };
        kitGraphqlData(await this.gqlRun({ query: `mutation($c: BackboneConfigInput!) { session { backbone { attach(config: $c) { attached } } } } }`, variables: { c: cfg } }));
        return { ok: true };
      }
      if (kind === "detachBackbone") {
        kitGraphqlData(await this.gqlRun({ query: `mutation { session { backbone { detach } } }` }));
        return { ok: true };
      }
      if (kind === "syncNow") {
        kitGraphqlData(await this.gqlRun({ query: `mutation { session { backbone { syncNow } } }` }));
        return { ok: true };
      }
      if (kind === "resolveConflict") {
        kitGraphqlData(
          await this.gqlRun({
            query: `mutation($id: String!, $strategy: ConflictResolutionInput!) { session { backbone { resolveConflict(id: $id, strategy: $strategy) } } } }`,
            variables: { id: String(variables.id), strategy: variables.strategy },
          }),
        );
        return { ok: true };
      }
      return { ok: false, error: { kind: "Internal", message: `backbone batch: unsupported ${kind}` } };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  /** @emoji 🧾 Backbone reads: conflicts list or attach status via GraphQL. */
  private async runBackboneBatchTypedRead<T>(kind: "listConflicts" | "backboneStatus"): Promise<T> {
    this.ensureAlive();
    if (kind === "listConflicts") {
      const data = kitGraphqlData(
        await this.gqlRun({ query: `query { session { conflicts { id reason createdAt wipCheckpoint { id } backboneTip { id } } } }` }),
      ) as { session?: { conflicts?: readonly ConflictBatchRecord[] } };
      return ((data.session?.conflicts as readonly ConflictBatchRecord[] | undefined) ?? []) as T;
    }
    const data = kitGraphqlData(
      await this.gqlRun({ query: `mutation { session { backbone { status { attached kind kindOther tip } } } }` }),
    ) as { session?: { backbone?: { status?: Partial<BackboneStatusDto> & { tip?: string | null } } } };
    const br = data.session?.backbone?.status ?? {};
    return {
      attached: br.attached ?? false,
      kind: br.kind ?? null,
      backboneTip: br.tip ?? null,
      pendingWipCheckpoints: 0,
    } as T;
  }

  /** @emoji 🧾 Undo/redo are not on the target `SessionCommandInput` tree; callers should use rs edit-log APIs when exposed. */
  private async runVcsUndoRedo(kind: "undo" | "redo"): Promise<SetResult> {
    void kind;
    this.ensureAlive();
    return {
      ok: false,
      error: { kind: "NotSupported", message: "undo/redo: not on target SessionCommandInput (draft/transaction stack removed)" },
    };
  }

  /** @emoji 🧾 Canvas ops mapped into `changeKitCommands` + {@link runScopedTransactionBatch}. */
  private async runScopedCanvasControl(kind: string, variables: JsonObject): Promise<SetResult> {
    this.ensureAlive();
    const inner = this.scopedTransactionInnerFromControl(kind, variables);
    if (inner == null) return { ok: false, error: { kind: "NotSupported", message: `no batch mapping for ${kind}` } };
    try {
      const rows = await this.runScopedTransactionBatch([inner]);
      for (const row of rows) {
        if (row.ok === false) return { ok: false, error: { kind: "Internal", message: `batch ${String(row.kind)}` } };
      }
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  async changeKitWithInverse(commands: unknown): Promise<{ kind: KitChangeKind; inverse: readonly ChangeKitCommand[] }> {
    this.ensureAlive();
    const list = Array.isArray(commands) ? ([...commands] as ChangeKitCommand[]) : [];
    await this.runScopedTransactionBatch([{ changeKitCommands: { commands: list } } as JsonObject]);
    return { kind: "inferred", inverse: [] };
  }

  async clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult> {
    return this.runScopedCanvasControl("clusterPieces", { designId, pieceIds: [...pieceIds], clusterName });
  }

  async dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.runScopedCanvasControl("dragPieces", { designId, pieceIds: [...pieceIds], du, dv });
  }

  async movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.runScopedCanvasControl("movePieces", { designId, pieceIds: [...pieceIds], gap, shift, rise });
  }

  async fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult> {
    return this.runScopedCanvasControl("fixPieces", { designId, pieceIds: [...pieceIds] });
  }

  async flattenDesign(designId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("flattenDesign", { designId });
  }

  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("expandDesign", { parentDesignId, nestedDesignId });
  }

  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("deleteConnection", { designId, connectionId });
  }

  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("changePieceType", { designId, pieceId, newTypeId });
  }

  async pasteDesignSelection(_designId: string, _selection: KitJsonTreeDto, _plane: PlaneDto | null): Promise<SetResult> {
    return {
      ok: false,
      error: { kind: "NotSupported", message: "pasteDesignSelection: not yet implemented in Rust store" },
    };
  }

  async createHangingPieces(designId: string, typeIds: readonly string[], plane: PlaneDto): Promise<SetResult> {
    return this.runScopedCanvasControl("createHangingPieces", { designId, typeIds: [...typeIds], plane });
  }

  async createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    return this.runScopedCanvasControl("createConnectedPiece", { designId, parentPiece, parentPort, childType, childPort });
  }

  async createFixedPiece(designId: string, typeId: string, plane: PlaneDto): Promise<SetResult> {
    return this.runScopedCanvasControl("createFixedPiece", { designId, typeId, plane });
  }

  async undo(): Promise<SetResult> {
    return this.runVcsUndoRedo("undo");
  }

  async redo(): Promise<SetResult> {
    return this.runVcsUndoRedo("redo");
  }

  async attachBackbone(cfg: BackboneConfig): Promise<SetResult> {
    return this.runBackboneBatchSetResult("attachBackbone", { config: cfg });
  }

  async detachBackbone(): Promise<SetResult> {
    return this.runBackboneBatchSetResult("detachBackbone", {});
  }

  async backboneStatus(): Promise<BackboneStatusDto> {
    return this.runBackboneBatchTypedRead<BackboneStatusDto>("backboneStatus");
  }

  async listConflicts(): Promise<KitConflict[]> {
    const raw = await this.runBackboneBatchTypedRead<readonly ConflictBatchRecord[]>("listConflicts");
    if (Array.isArray(raw)) return raw as KitConflict[];
    return [];
  }

  async resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> {
    return this.runBackboneBatchSetResult("resolveConflict", { id, strategy });
  }

  async syncNow(): Promise<SetResult> {
    return this.runBackboneBatchSetResult("syncNow", {});
  }

  /**
   * @emoji 🧭 Read-only flatten map rows for one design (`semio/rs` `flatten_map`), for algorithm / MCP tooling.
   */
  async readDesignFlattenMap(scope: KitReadPoint, designId: string): Promise<readonly DesignFlattenMapEntryDto[]> {
    const data = await this.gqlRunSessionWipStore(scope, `design(id: $id) { flattenMap }`, { id: designId });
    const raw = (gqlDataSessionWipKitStore(data, scope)?.["design"] as JsonObject | undefined)?.["flattenMap"];
    const FlatRow = z.object({ pieceId: z.string(), plane: PlaneSchema, center: PointSchema });
    if (Array.isArray(raw)) {
      const out: DesignFlattenMapEntryDto[] = [];
      for (const row of raw) {
        const pr = FlatRow.safeParse(row);
        if (pr.success) out.push(pr.data);
      }
      return out;
    }
    if (typeof raw === "string") {
      try {
        const p = parseJsonValue(raw) as JsonValue;
        if (!Array.isArray(p)) return [];
        const out: DesignFlattenMapEntryDto[] = [];
        for (const row of p) {
          const pr = FlatRow.safeParse(row);
          if (pr.success) out.push(pr.data);
        }
        return out;
      } catch {
        return [];
      }
    }
    return [];
  }

  async read(scope: KitReadPoint, batch: ReadBatch): Promise<ReadBatchResult> {
    this.ensureAlive();
    const out: ReadKitCommandOutput[] = [];
    for (const c of batch) out.push(await this.mapReadCommand(scope, c));
    return out;
  }

  /** @emoji 🧾 Apply typed `ChangeKitCommand` batch inside the active unsaved version change (`kitStore.batch`). */
  async submitChangeKitCommands(commands: readonly ChangeKitCommand[]): Promise<SetResult> {
    try {
      const rows = await this.runScopedTransactionBatch([{ changeKitCommands: { commands: [...commands] } } as JsonObject]);
      for (const row of rows) {
        if (row.ok === false) return { ok: false, error: { kind: "Internal", message: `batch ${String(row.kind)}` } };
      }
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  private async mapReadCommand(scope: KitReadPoint, c: ReadKitCommand): Promise<ReadKitCommandOutput> {
    if ("readKitFullCommand" in c && c.readKitFullCommand === null) {
      const d = await this.readKitSnapshotForReadPoint(scope);
      return { readKitFullCommand: { full: d } };
    }
    if ("readKitShallowCommand" in c && c.readKitShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, `${kitGqlKitTypesRelay(KIT_GQL_TYPE_SHALLOW_FIELDS)} ${kitGqlKitDesignsRelay(KIT_GQL_DESIGN_SHALLOW_FIELDS)}`);
      const typesRows = kitGraphqlJsonToReadonlyArray((row.types as KitJsonTreeDto | undefined) ?? []);
      const designRows = kitGraphqlJsonToReadonlyArray((row.designs as KitJsonTreeDto | undefined) ?? []);
      return {
        readKitShallowCommand: {
          types: semioParseTypeShallowArrayJson(typesRows as KitJsonTreeDto[]),
          designs: semioParseDesignShallowArrayJson(designRows as KitJsonTreeDto[]),
        },
      };
    }
    if ("readKitTypeIdsCommand" in c && c.readKitTypeIdsCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitTypesRelay("id"));
      return { readKitTypeIdsCommand: { typeIds: semioParseKitIdDtoArray(row.types as KitJsonTreeDto | string) } };
    }
    if ("readKitDesignIdsCommand" in c && c.readKitDesignIdsCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitDesignsRelay("id"));
      return { readKitDesignIdsCommand: { designIds: semioParseKitIdDtoArray(row.designs as KitJsonTreeDto | string) } };
    }
    if ("readKitTypesMetadataCommand" in c && c.readKitTypesMetadataCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitTypesRelay(KIT_GQL_TYPE_METADATA_FIELDS));
      const metas = kitGraphqlJsonToReadonlyArray((row.types as KitJsonTreeDto | undefined) ?? []);
      const parsed = metas.map((raw) =>
        typeof raw === "object" && raw != null ? TypeMetadataDtoSchema.parse(__coerceTypeMetadataGqlRow(raw as JsonObject)) : null,
      );
      return {
        readKitTypesMetadataCommand: {
          types: parsed.filter((x) => x !== null) as readonly TypeMetadataDto[],
        },
      };
    }
    if ("readKitDesignsMetadataCommand" in c && c.readKitDesignsMetadataCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitDesignsRelay(KIT_GQL_DESIGN_METADATA_FIELDS));
      const metas = kitGraphqlJsonToReadonlyArray((row.designs as KitJsonTreeDto | undefined) ?? []);
      const parsed = metas.map((raw) =>
        typeof raw === "object" && raw != null ? DesignMetadataDtoSchema.parse(__stripTopLevelJsonNulls(raw as JsonObject) as KitJsonTreeDto) : null,
      );
      return {
        readKitDesignsMetadataCommand: {
          designs: parsed.filter((x) => x !== null) as readonly DesignMetadataDto[],
        },
      };
    }
    if ("readKitTypesShallowCommand" in c && c.readKitTypesShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitTypesRelay(KIT_GQL_TYPE_SHALLOW_FIELDS));
      const rows = kitGraphqlJsonToReadonlyArray((row.types as KitJsonTreeDto | undefined) ?? []);
      return {
        readKitTypesShallowCommand: {
          types: semioParseTypeShallowArrayJson(rows as KitJsonTreeDto[]),
        },
      };
    }
    if ("readKitDesignsShallowCommand" in c && c.readKitDesignsShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitDesignsRelay(KIT_GQL_DESIGN_SHALLOW_FIELDS));
      const rows = kitGraphqlJsonToReadonlyArray((row.designs as KitJsonTreeDto | undefined) ?? []);
      return {
        readKitDesignsShallowCommand: {
          designs: semioParseDesignShallowArrayJson(rows as KitJsonTreeDto[]),
        },
      };
    }
    if ("readKitAuthorsShallowCommand" in c && c.readKitAuthorsShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, kitGqlKitAuthorsRelay("id name email role rank"));
      const authors = kitGraphqlJsonToReadonlyArray((row.authors as KitJsonTreeDto | undefined) ?? []);
      return {
        readKitAuthorsShallowCommand: {
          authors: semioParseAuthorMetadataArrayJson(authors as KitJsonTreeDto[]),
        },
      };
    }
    if ("readKitMetadataCommand" in c && c.readKitMetadataCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "id name description icon image preview remote homepage license uri created updated version hash");
      return { readKitMetadataCommand: { metadata: semioParseKitMetadataJson(row as KitJsonTreeDto) } };
    }
    if ("readKitDesignCommands" in c && c.readKitDesignCommands) {
      const { id, commands } = c.readKitDesignCommands;
      const results: ReadDesignCommandOutput[] = [];
      for (const sub of commands) results.push(await this.mapDesignRead(scope, id.id, sub));
      return { readKitDesignCommands: { results } };
    }
    if ("readKitTypeCommands" in c && c.readKitTypeCommands) {
      const { id, commands } = c.readKitTypeCommands;
      const results: ReadTypeCommandOutput[] = [];
      for (const sub of commands) results.push(await this.mapTypeRead(scope, id.id, sub));
      return { readKitTypeCommands: { results } };
    }
    throw new Error(`read: unsupported ${Object.keys(c).join(",")}`);
  }

  private async mapDesignRead(scope: KitReadPoint, designId: string, cmd: ReadDesignCommand): Promise<ReadDesignCommandOutput> {
    if ("readDesignPiecesFullCommand" in cmd && cmd.readDesignPiecesFullCommand === null) {
      const kit = await this.readKitSnapshotForReadPoint(scope);
      const des = (kit.designs ?? []).find((x) => x.id === designId);
      return { readDesignPiecesFullCommand: { pieces: des?.pieces ?? [] } };
    }
    if ("readDesignConnectionsFullCommand" in cmd && cmd.readDesignConnectionsFullCommand === null) {
      const kit = await this.readKitSnapshotForReadPoint(scope);
      const des = (kit.designs ?? []).find((x) => x.id === designId);
      return { readDesignConnectionsFullCommand: { connections: des?.connections ?? [] } };
    }
    if ("readDesignPieceCommands" in cmd && cmd.readDesignPieceCommands) {
      const { id, commands } = cmd.readDesignPieceCommands;
      const results: ReadPieceCommandOutput[] = [];
      for (const pc of commands) results.push(await this.mapPieceRead(scope, designId, id.id, pc));
      return { readDesignPieceCommands: { results } };
    }
    if ("readDesignClusterableGroupsCommand" in cmd && cmd.readDesignClusterableGroupsCommand) {
      // Grouping is not exposed on `Design` in the integrator SDL; kit graph remains RS truth when implemented.
      return { readDesignClusterableGroupsCommand: { groups: [] } };
    }
    if ("readDesignIncludedDesignsCommand" in cmd && cmd.readDesignIncludedDesignsCommand === null) {
      return { readDesignIncludedDesignsCommand: { designs: [] } };
    }
    if ("readDesignQualitySumCommand" in cmd && cmd.readDesignQualitySumCommand) {
      const qid = cmd.readDesignQualitySumCommand.qualityId.id;
      const d = await this.gqlRunSessionWipStore(scope, `design(id: $id) { qualitySum(qualityId: $qid) }`, { id: designId, qid });
      const q = (gqlDataSessionWipKitStore(d, scope)?.["design"] as JsonObject | undefined)?.["qualitySum"];
      const sum = typeof q === "number" && !Number.isNaN(q) ? q : Number(q ?? 0);
      return { readDesignQualitySumCommand: { sum } };
    }
    if ("readDesignReplaceableCatalogCommand" in cmd && cmd.readDesignReplaceableCatalogCommand) {
      return { readDesignReplaceableCatalogCommand: { types: [], designs: [] } };
    }
    if ("readDesignIncludedDesignIdsCommand" in cmd && cmd.readDesignIncludedDesignIdsCommand === null) {
      return { readDesignIncludedDesignIdsCommand: { designIds: [] } };
    }
    throw new Error(`readDesign: ${Object.keys(cmd).join(",")}`);
  }

  private async mapPieceRead(scope: KitReadPoint, designId: string, pieceId: string, cmd: ReadPieceCommand): Promise<ReadPieceCommandOutput> {
    const pieceGql = `piece(id: $p) { flatPosition { plane { origin { x y z } xAxis { x y z } yAxis { x y z } } center { u v } } parentConnection { id gap shift rise rotation turn tilt u v description } }`;
    if ("readPieceFlatPlaneCommand" in cmd && cmd.readPieceFlatPlaneCommand === null) {
      const d = await this.gqlRunSessionWipStore(scope, `design(id: $d) { ${pieceGql} }`, { d: designId, p: pieceId });
      const piece = (gqlDataSessionWipKitStore(d, scope)?.["design"] as JsonObject | undefined)?.["piece"] as JsonObject | undefined;
      const fp = piece?.["flatPosition"] as JsonObject | undefined;
      return {
        readPieceFlatPlaneCommand: { flatPlane: semioParsePlaneNullableJson(fp?.["plane"] as KitJsonTreeDto) },
      };
    }
    if ("readPieceFlatCenterCommand" in cmd && cmd.readPieceFlatCenterCommand === null) {
      const d = await this.gqlRunSessionWipStore(scope, `design(id: $d) { ${pieceGql} }`, { d: designId, p: pieceId });
      const piece = (gqlDataSessionWipKitStore(d, scope)?.["design"] as JsonObject | undefined)?.["piece"] as JsonObject | undefined;
      const fp = piece?.["flatPosition"] as JsonObject | undefined;
      return {
        readPieceFlatCenterCommand: {
          flatCenter: semioParseCoordinateNullableJson(fp?.["center"] as KitJsonTreeDto),
        },
      };
    }
    if ("readPieceParentConnectionFullCommand" in cmd && cmd.readPieceParentConnectionFullCommand === null) {
      const d = await this.gqlRunSessionWipStore(scope, `design(id: $d) { ${pieceGql} }`, { d: designId, p: pieceId });
      const piece = (gqlDataSessionWipKitStore(d, scope)?.["design"] as JsonObject | undefined)?.["piece"] as JsonObject | undefined;
      return {
        readPieceParentConnectionFullCommand: {
          connection: semioParseConnectionNullableJson(piece?.["parentConnection"] as KitJsonTreeDto),
        },
      };
    }
    throw new Error(`readPiece: ${Object.keys(cmd).join(",")}`);
  }

  private async mapTypeRead(scope: KitReadPoint, typeId: string, cmd: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
    if ("readTypeBestRepresentationCommand" in cmd && cmd.readTypeBestRepresentationCommand) {
      const tags = cmd.readTypeBestRepresentationCommand.tagIds;
      const d = await this.gqlRunSessionWipStore(scope, `type(id: $id) { bestRepresentation(tagIds: $tags) }`, { id: typeId, tags: [...tags] });
      return {
        readTypeBestRepresentationCommand: {
          representation: semioParseRepresentationNullableJson((gqlDataSessionWipKitStore(d, scope)?.["type"] as JsonObject | undefined)?.["bestRepresentation"] as KitJsonTreeDto),
        },
      };
    }
    throw new Error(`readType: ${Object.keys(cmd).join(",")}`);
  }

  async getPiecesMetadata(scope: KitReadPoint, designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    const d = await this.gqlRunSessionWipStore(
      scope,
      `design(id: $id) { pieces { id depth flatPosition { plane { origin { x y z } xAxis { x y z } yAxis { x y z } } center { u v } } path { id } parentPiece { id } } }`,
      { id: designId },
    );
    const rawPieces = (gqlDataSessionWipKitStore(d, scope)?.["design"] as JsonObject | undefined)?.["pieces"];
    const rows = Array.isArray(rawPieces) ? rawPieces : [];
    const normalized = rows.map((p: JsonObject) => {
      const fp = p["flatPosition"] as JsonObject | undefined;
      return { ...p, flatPlane: fp?.["plane"], flatCenter: fp?.["center"] };
    });
    return semioParsePiecePlacementMapJson(normalized);
  }

  async getPieces(scope: KitReadPoint, designId: string): Promise<readonly PieceDto[]> {
    const out = await this.read(scope, [{ readKitDesignCommands: { id: { id: designId }, commands: [{ readDesignPiecesFullCommand: null }] } }]);
    const row = out[0];
    if (row && "readKitDesignCommands" in row) {
      const sub = row.readKitDesignCommands.results[0];
      if (sub && "readDesignPiecesFullCommand" in sub) return sub.readDesignPiecesFullCommand.pieces;
    }
    return [];
  }

  async getConnections(scope: KitReadPoint, designId: string): Promise<readonly ConnectionDto[]> {
    const out = await this.read(scope, [{ readKitDesignCommands: { id: { id: designId }, commands: [{ readDesignConnectionsFullCommand: null }] } }]);
    const row = out[0];
    if (row && "readKitDesignCommands" in row) {
      const sub = row.readKitDesignCommands.results[0];
      if (sub && "readDesignConnectionsFullCommand" in sub) return sub.readDesignConnectionsFullCommand.connections;
    }
    return [];
  }

  async getDesigns(scope: KitReadPoint): Promise<readonly DesignShallow[]> {
    const out = await this.read(scope, [{ readKitDesignsShallowCommand: null }]);
    const row = out[0];
    if (row && "readKitDesignsShallowCommand" in row) return row.readKitDesignsShallowCommand.designs;
    return [];
  }

  async getTypes(scope: KitReadPoint): Promise<readonly TypeShallow[]> {
    const out = await this.read(scope, [{ readKitTypesShallowCommand: null }]);
    const row = out[0];
    if (row && "readKitTypesShallowCommand" in row) return row.readKitTypesShallowCommand.types;
    return [];
  }

  async getAuthors(scope: KitReadPoint): Promise<readonly AuthorMetadataDto[]> {
    const out = await this.read(scope, [{ readKitAuthorsShallowCommand: null }]);
    const row = out[0];
    if (row && "readKitAuthorsShallowCommand" in row) return row.readKitAuthorsShallowCommand.authors;
    return [];
  }

  async getKitMetadata(scope: KitReadPoint): Promise<KitMetadataDto | null> {
    const out = await this.read(scope, [{ readKitMetadataCommand: null }]);
    const row = out[0];
    if (row && "readKitMetadataCommand" in row) return row.readKitMetadataCommand.metadata;
    return null;
  }

  // #region KitStoreEntityFactories
  /** @emoji 🧭 Sync handle for kit-scoped design reads and mutations (no I/O). */
  design(id: string, readPoint: KitReadPoint = theKitReadPoint): DesignStore {
    return new DesignStore(this, id, readPoint);
  }
  /** @emoji 🧭 Sync handle for kit-scoped kind reads and mutations (no I/O). */
  type(id: string, readPoint: KitReadPoint = theKitReadPoint): TypeStore {
    return new TypeStore(this, id, readPoint);
  }
  /** @emoji 🧭 Sync handle for a piece within a design (no I/O). */
  piece(designId: string, id: string, readPoint: KitReadPoint = theKitReadPoint): PieceStore {
    return new PieceStore(this, designId, id, readPoint);
  }
  /** @emoji 🧭 Sync handle for a connection within a design (no I/O). */
  connection(designId: string, id: string, readPoint: KitReadPoint = theKitReadPoint): ConnectionStore {
    return new ConnectionStore(this, designId, id, readPoint);
  }
  family(id: string, readPoint: KitReadPoint = theKitReadPoint): FamilyStore {
    return new FamilyStore(this, id, readPoint);
  }
  file(id: string, readPoint: KitReadPoint = theKitReadPoint): FileStore {
    return new FileStore(this, id, readPoint);
  }
  folder(id: string, readPoint: KitReadPoint = theKitReadPoint): FolderStore {
    return new FolderStore(this, id, readPoint);
  }

  /** @emoji 🧭 All design ids in the live kit as {@link DesignStore} handles. */
  async designs(scope: KitReadPoint = theKitReadPoint): Promise<readonly DesignStore[]> {
    const out = await this.read(scope, [{ readKitDesignIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray((out[0] as { readKitDesignIdsCommand?: { designIds?: JsonValue } }).readKitDesignIdsCommand?.designIds);
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => this.design(toId(row), scope)).filter((s) => s.id !== "");
  }

  /** @emoji 🧭 All kind ids in the live kit as {@link TypeStore} handles. */
  async types(scope: KitReadPoint = theKitReadPoint): Promise<readonly TypeStore[]> {
    const out = await this.read(scope, [{ readKitTypeIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray((out[0] as { readKitTypeIdsCommand?: { typeIds?: JsonValue } }).readKitTypeIdsCommand?.typeIds);
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => this.type(toId(row), scope)).filter((s) => s.id !== "");
  }

  /** @emoji 🧾 Design row id strings from `readKitDesignIdsCommand` (no {@link DesignStore} allocation). */
  async designRowIds(scope: KitReadPoint = theKitReadPoint): Promise<readonly string[]> {
    const out = await this.read(scope, [{ readKitDesignIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray((out[0] as { readKitDesignIdsCommand?: { designIds?: JsonValue } }).readKitDesignIdsCommand?.designIds);
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => toId(row)).filter((s) => s !== "");
  }

  /** @emoji 🧾 Kind row id strings from `readKitTypeIdsCommand` (no {@link TypeStore} allocation). */
  async kindRowIds(scope: KitReadPoint = theKitReadPoint): Promise<readonly string[]> {
    const out = await this.read(scope, [{ readKitTypeIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray((out[0] as { readKitTypeIdsCommand?: { typeIds?: JsonValue } }).readKitTypeIdsCommand?.typeIds);
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => toId(row)).filter((s) => s !== "");
  }

  /** @emoji 🧾 Per-kind metadata rows (`readKitTypesMetadataCommand`). */
  async kindMetadataRows(scope: KitReadPoint = theKitReadPoint): Promise<readonly unknown[]> {
    const out = await this.read(scope, [{ readKitTypesMetadataCommand: null }]);
    return kitGraphqlJsonToReadonlyArray((out[0] as { readKitTypesMetadataCommand?: { types?: JsonValue } }).readKitTypesMetadataCommand?.types);
  }

  /** @emoji 🧾 Per-design metadata rows (`readKitDesignsMetadataCommand`). */
  async designMetadataRows(scope: KitReadPoint = theKitReadPoint): Promise<readonly unknown[]> {
    const out = await this.read(scope, [{ readKitDesignsMetadataCommand: null }]);
    return kitGraphqlJsonToReadonlyArray((out[0] as { readKitDesignsMetadataCommand?: { designs?: JsonValue } }).readKitDesignsMetadataCommand?.designs);
  }
  // #endregion KitStoreEntityFactories
}

// #endregion 📦KitStore

// #region 🧰OpenKit

/**
 * @emoji 🧰 Convenience alias for {@link KitStore.open}.
 */
export async function openKit(initialKit: KitFullDto, opts?: KitStoreOpenOptions): Promise<KitStore> {
  return KitStore.open(initialKit, opts);
}

// #endregion 🧰OpenKit

// #region KitEventEntityFilter
/** @emoji 🧾 Whether a {@link KitChange} references a design id in forward or inverse commands. */
function kitChangeTouchesDesignId(change: KitChange, designId: string): boolean {
  for (const cmd of [...change.forward, ...change.inverse]) {
    if (jsonSubtreeHasIdKey(cmd, "designId", designId)) return true;
    if (jsonSubtreeHasIdKey(cmd, "design_id", designId)) return true;
    if (jsonSubtreeHasIdKey(cmd, "parentDesignId", designId)) return true;
    if (jsonSubtreeHasIdKey(cmd, "nestedDesignId", designId)) return true;
  }
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given design id. */
function kitClassifiedMutationTouchesDesign(ev: KitClassifiedMutationEvent, designId: string): boolean {
  if ("renamedDesign" in ev && ev.renamedDesign.designId === designId) return true;
  if ("draggedFlatCenterPiece" in ev && ev.draggedFlatCenterPiece.designId === designId) return true;
  if ("movedPiecesFlatCenter" in ev && ev.movedPiecesFlatCenter.designId === designId) return true;
  if ("clusteredPieces" in ev && ev.clusteredPieces.designId === designId) return true;
  if ("fixedPiecesFlatCenter" in ev && ev.fixedPiecesFlatCenter.designId === designId) return true;
  if ("flattenedDesign" in ev && ev.flattenedDesign.designId === designId) return true;
  if ("expandedNestedDesign" in ev && (ev.expandedNestedDesign.parentDesignId === designId || ev.expandedNestedDesign.nestedDesignId === designId)) return true;
  if ("deletedConnection" in ev && ev.deletedConnection.designId === designId) return true;
  if ("changedPieceKind" in ev && ev.changedPieceKind.designId === designId) return true;
  if ("changedDesignCommands" in ev && ev.changedDesignCommands.designId === designId) return true;
  if ("changedKit" in ev && kitChangeTouchesDesignId(ev.changedKit.change, designId)) return true;
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given kind id. */
function kitClassifiedMutationTouchesType(ev: KitClassifiedMutationEvent, typeId: string): boolean {
  if ("renamedType" in ev && ev.renamedType.typeId === typeId) return true;
  if ("changedTypeCommands" in ev && ev.changedTypeCommands.typeId === typeId) return true;
  if ("changedPieceKind" in ev && jsonSubtreeHasIdKey(ev.changedPieceKind.change, "newTypeId", typeId)) return true;
  if ("changedKit" in ev) {
    const ch = ev.changedKit.change;
    for (const cmd of [...ch.forward, ...ch.inverse]) {
      if (jsonSubtreeHasIdKey(cmd, "typeId", typeId)) return true;
      if (jsonSubtreeHasIdKey(cmd, "type_id", typeId)) return true;
    }
  }
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given piece in a design. */
function kitClassifiedMutationTouchesPiece(ev: KitClassifiedMutationEvent, designId: string, pieceId: string): boolean {
  if ("changedPieceKind" in ev && ev.changedPieceKind.designId === designId && ev.changedPieceKind.pieceId === pieceId) return true;
  if ("draggedFlatCenterPiece" in ev && ev.draggedFlatCenterPiece.designId === designId && ev.draggedFlatCenterPiece.pieceIds.includes(pieceId)) return true;
  if ("movedPiecesFlatCenter" in ev && ev.movedPiecesFlatCenter.designId === designId && ev.movedPiecesFlatCenter.pieceIds.includes(pieceId)) return true;
  if ("clusteredPieces" in ev && ev.clusteredPieces.designId === designId && ev.clusteredPieces.pieceIds.includes(pieceId)) return true;
  if ("fixedPiecesFlatCenter" in ev && ev.fixedPiecesFlatCenter.designId === designId && ev.fixedPiecesFlatCenter.pieceIds.includes(pieceId)) return true;
  if ("changedDesignCommands" in ev && ev.changedDesignCommands.designId === designId) {
    return jsonSubtreeHasIdKey(ev.changedDesignCommands.change, "pieceId", pieceId);
  }
  if ("changedKit" in ev && kitChangeTouchesDesignId(ev.changedKit.change, designId)) {
    return jsonSubtreeHasIdKey(ev.changedKit.change, "pieceId", pieceId);
  }
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given connection in a design. */
function kitClassifiedMutationTouchesConnection(ev: KitClassifiedMutationEvent, designId: string, connectionId: string): boolean {
  if ("deletedConnection" in ev && ev.deletedConnection.designId === designId && ev.deletedConnection.connectionId === connectionId) return true;
  if (kitClassifiedMutationTouchesDesign(ev, designId) && "changedKit" in ev) {
    return jsonSubtreeHasIdKey(ev.changedKit.change, "connectionId", connectionId);
  }
  return false;
}

/** @emoji 🧪 True when JSON-like subtree (including kit command atoms) contains a string field `key` equal to `id`. */
function jsonSubtreeHasIdKey(raw: unknown, key: string, id: string): boolean {
  if (raw == null) return false;
  if (typeof raw === "string") return false;
  if (typeof raw === "number" || typeof raw === "boolean") return false;
  if (Array.isArray(raw)) {
    for (const x of raw) if (jsonSubtreeHasIdKey(x, key, id)) return true;
    return false;
  }
  if (typeof raw === "object") {
    const o = raw as { readonly [k: string]: unknown };
    const v = o[key];
    if (typeof v === "string" && v === id) return true;
    for (const k of Object.keys(o)) if (jsonSubtreeHasIdKey(o[k], key, id)) return true;
  }
  return false;
}

/** @emoji 🧭 Design-scoped kit events (excludes bare `Changed` and `FlattenInvalidated`, which are handled separately per subscriber). */
export function kitEventTouchesDesignStrict(ev: KitEvent, designId: string): boolean {
  if (designId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesDesign(ev, designId)) return true;
  const d = (ev as { Design?: { design_id?: string; event?: JsonValue } }).Design;
  if (d && typeof d.design_id === "string" && d.design_id === designId) return true;
  if (jsonSubtreeHasIdKey(ev, "design_id", designId)) return true;
  const ca = (ev as { ChildAdded?: { parent?: { id?: string }; child?: { id?: string } } }).ChildAdded;
  if (ca && ca.parent?.id === designId) return true;
  const cr = (ev as { ChildRemoved?: { parent?: { id?: string }; child?: { id?: string } } }).ChildRemoved;
  if (cr && cr.parent?.id === designId) return true;
  return false;
}

/** @emoji 🧭 Whether a subscription {@link KitEvent} likely concerns the given design (includes kit-wide invalidations). */
export function kitEventTouchesDesign(ev: KitEvent, designId: string): boolean {
  if (designId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  if ("ValidationInvalidated" in ev && (ev as { ValidationInvalidated?: null }).ValidationInvalidated === null) return true;
  const fi = (ev as { FlattenInvalidated?: { design?: string; pieces?: readonly string[] } }).FlattenInvalidated;
  if (fi && typeof fi.design === "string" && fi.design === designId) return true;
  return kitEventTouchesDesignStrict(ev, designId);
}

/** @emoji 🧭 Type-scoped events (no bare `Changed`). */
export function kitEventTouchesTypeStrict(ev: KitEvent, typeId: string): boolean {
  if (typeId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesType(ev, typeId)) return true;
  const t = (ev as { Type?: { type_id?: string } }).Type;
  if (t && typeof t.type_id === "string" && t.type_id === typeId) return true;
  if (jsonSubtreeHasIdKey(ev, "type_id", typeId)) return true;
  const ca = (ev as { ChildAdded?: { parent?: { id?: string; kind?: string }; child?: { id?: string } } }).ChildAdded;
  if (ca?.parent?.kind === "Type" && ca.parent.id === typeId) return true;
  const cr = (ev as { ChildRemoved?: { parent?: { id?: string; kind?: string }; child?: { id?: string } } }).ChildRemoved;
  if (cr?.parent?.kind === "Type" && cr.parent.id === typeId) return true;
  return false;
}

/** @emoji 🧭 Whether a subscription {@link KitEvent} likely concerns the given kind id. */
export function kitEventTouchesType(ev: KitEvent, typeId: string): boolean {
  if (typeId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  if ("ValidationInvalidated" in ev && (ev as { ValidationInvalidated?: null }).ValidationInvalidated === null) return true;
  return kitEventTouchesTypeStrict(ev, typeId);
}

/** @emoji 🧭 Piece-scoped events (design-scoped strict + piece id + flatten rows). */
export function kitEventTouchesPiece(ev: KitEvent, designId: string, pieceId: string): boolean {
  if (pieceId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesPiece(ev, designId, pieceId)) return true;
  if (kitEventTouchesDesignStrict(ev, designId)) return true;
  const p = (ev as { Piece?: { piece_id?: string } }).Piece;
  if (p && typeof p.piece_id === "string" && p.piece_id === pieceId) return true;
  if (jsonSubtreeHasIdKey(ev, "piece_id", pieceId)) return true;
  const fi = (ev as { FlattenInvalidated?: { design?: string; pieces?: string[] } }).FlattenInvalidated;
  if (fi && fi.design === designId) {
    const rows = fi.pieces;
    if (!Array.isArray(rows) || rows.length === 0) return true;
    return rows.includes(pieceId);
  }
  return false;
}

/** @emoji 🧭 Connection-scoped events (design-scoped strict + connection id). */
export function kitEventTouchesConnection(ev: KitEvent, designId: string, connectionId: string): boolean {
  if (connectionId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesConnection(ev, designId, connectionId)) return true;
  if (kitEventTouchesDesignStrict(ev, designId)) return true;
  const c = (ev as { Connection?: { connection_id?: string } }).Connection;
  if (c && typeof c.connection_id === "string" && c.connection_id === connectionId) return true;
  if (jsonSubtreeHasIdKey(ev, "connection_id", connectionId)) return true;
  return false;
}

/** @emoji 🧭 Family / file / folder entity filters (ChildAdded paths + id fields). */
export function kitEventTouchesFamily(ev: KitEvent, familyId: string): boolean {
  if (familyId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  const f = (ev as { Family?: { family_id?: string } }).Family;
  if (f && typeof f.family_id === "string" && f.family_id === familyId) return true;
  if (jsonSubtreeHasIdKey(ev, "family_id", familyId)) return true;
  return false;
}

export function kitEventTouchesFile(ev: KitEvent, fileId: string): boolean {
  if (fileId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  const f = (ev as { File?: { file_id?: string } }).File;
  if (f && typeof f.file_id === "string" && f.file_id === fileId) return true;
  if (jsonSubtreeHasIdKey(ev, "file_id", fileId)) return true;
  return false;
}

export function kitEventTouchesFolder(ev: KitEvent, folderId: string): boolean {
  if (folderId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  const f = (ev as { Folder?: { folder_id?: string } }).Folder;
  if (f && typeof f.folder_id === "string" && f.folder_id === folderId) return true;
  if (jsonSubtreeHasIdKey(ev, "folder_id", folderId)) return true;
  return false;
}
// #endregion KitEventEntityFilter

// #region 🧩KitWasmBridgeMerged
// #region 🔌KitStoreClientTypes

export type KitStoreExecuteResult = { ok: true; result: JsonValue } | { ok: false; error: SetError };

/** @emoji 🧾 Sketchpad string-command context/result (opaque JSON). */
export type KitCommandContext = JsonObject;
export type KitCommandResult = JsonObject;

/** @emoji 🧾 Typed kit mutation envelope for React facades (`kitStore.batch` version change `changeKitCommands`). */
type KitTypedChangeKitCommandsBatch = { readonly kind: "changeKitCommands"; readonly commands: readonly ChangeKitCommand[] };

/** @emoji 🧾 Typed `changeKitCommands` batch facade for React (opaque to string command routers). */
export type SemioKitCommandFacade = { runMutation(cmd: KitTypedChangeKitCommandsBatch): Promise<SetResult> };

export type KitStoreReadSnap = { readonly version: number; readonly data: unknown; readonly pending: number };

export type KitDesignReadKind = "metadata" | "pieces" | "connections";
export type KitShallowListKind = "designs" | "types" | "authors";
export type KitViewCatalogKey = "typeIds" | "typesMetadata" | "designIds" | "designsMetadata";

/** @emoji 🧾 Minimal async bridge for pulling authoritative kit JSON into a {@link KitHostStore}. */
export type SemioKitBridge = { fetchFullKit(): Promise<KitFullDto> };

/** @emoji 🧾 Browser / test kit RPC surface used by React hooks (wraps {@link KitStore}). */
export type KitStoreClient = SemioKitBridge & {
  /** @emoji 🧾 Scoped read point (see {@link WasmKitStoreClient#kitReadPoint} / {@link getKitClientReadPoint}). */
  readonly kitReadPoint: KitReadPoint;
  getKitWriteScope(): KitWriteScope | null;
  setKitWriteScope(scope: KitWriteScope | null): void;
  finalizeKitWriteTransaction(): Promise<SetResult>;
  abortKitWriteTransaction(): Promise<SetResult>;
  clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult>;
  dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult>;
  movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult>;
  fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult>;
  flattenDesign(designId: string): Promise<SetResult>;
  expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult>;
  deleteConnection(designId: string, connectionId: string): Promise<SetResult>;
  changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult>;
  pasteDesignSelection(designId: string, selection: KitJsonTreeDto, plane: PlaneDto | null): Promise<SetResult>;
  createHangingPieces(designId: string, typeIds: readonly string[], plane: PlaneDto): Promise<SetResult>;
  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult>;
  createFixedPiece(designId: string, typeId: string, plane: PlaneDto): Promise<SetResult>;
  submitChangeKitCommands(commands: readonly ChangeKitCommand[]): Promise<SetResult>;
  undo(): Promise<SetResult>;
  redo(): Promise<SetResult>;
  canUndo(): Promise<boolean>;
  canRedo(): Promise<boolean>;
  getPiecesMetadata(designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>>;
  getPieces(designId: string): Promise<readonly PieceDto[]>;
  getConnections(designId: string): Promise<readonly ConnectionDto[]>;
  getDesigns(): Promise<readonly DesignShallow[]>;
  getTypes(): Promise<readonly TypeShallow[]>;
  getAuthors(): Promise<readonly AuthorMetadataDto[]>;
  getKitMetadata(): Promise<KitMetadataDto | null>;
  backboneStatus(): Promise<BackboneStatusDto>;
  attachBackbone(cfg: BackboneConfig): Promise<SetResult>;
  detachBackbone(): Promise<SetResult>;
  listConflicts(): Promise<KitConflict[]>;
  resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult>;
  syncNow(): Promise<SetResult>;
  /** @emoji 🪪 Live kit name field + async rename (see {@link KitStore.kitName} / {@link KitStore.renameKit}). */
  readonly kitName: StoreField<string>;
  readonly renameKit: StoreCommand<RenameKitCommandArgs>;
  readKitName(): Promise<string>;
  /** @emoji 🧾 Live scoped kit field selection (see {@link KitStore.kitField}). */
  kitField<T>(
    cacheKey: string,
    spec: {
      extraVariableDecl?: string;
      extraVariables?: GraphQlVariables;
      innerOnKit: string;
      parse: (kitFragment: JsonObject) => T;
      initial: T;
    },
  ): StoreField<T>;
  /** @emoji 🌱 Fork a named alternative from the current checkpoint tip (`null` = kit main line). */
  createAlternativeFromTip(name: string, sourceAlternativeId: string | null): Promise<string>;
  kitGraphql(): LiveKitRoot;
  subscribe(cb: (ev: KitEvent) => void): () => void;
  readPieceFlatPlane(designId: string, pieceId: string): Promise<PlaneDto | null>;
  readPieceFlatCenter(designId: string, pieceId: string): Promise<CoordinateDto | null>;
  readPieceParentConnectionFull(designId: string, pieceId: string): Promise<ConnectionDto | null>;
  readDesignIncludedDesigns(designId: string): Promise<readonly IncludedDesignInfoDto[]>;
  readDesignClusterableGroups(designId: string, selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]>;
  readDesignQualitySum(designId: string, qualityId: string): Promise<number>;
  readTypeBestRepresentation(typeId: string, tagIds: readonly string[]): Promise<RepresentationDto | null>;
  readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]>;
  readDesignReplaceableCatalogTypes(designId: string, selection: readonly string[]): Promise<readonly string[]>;
  readDesignReplaceableCatalogDesigns(designId: string, selection: readonly string[]): Promise<readonly string[]>;
  readDesignIncludedDesignIds(designId: string): Promise<readonly string[]>;
  /** @emoji 🧭 Switch scoped kit read root (matches {@link WasmKitStoreClient.setKitReadPoint}). */
  setKitReadPoint(scope: KitReadPoint): void;
  dispose(): void;
};

// #endregion 🔌KitStoreClientTypes

// #region 🧰ReadHelpers

function firstDesignPieceResult(out: readonly unknown[], cmdKey: string): unknown {
  const row = out[0] as { readKitDesignCommands?: { results?: readonly unknown[] } };
  const r0 = row.readKitDesignCommands?.results?.[0] as JsonObject | undefined;
  if (!r0) return undefined;
  const inner = r0.readDesignPieceCommands as { results?: readonly unknown[] } | undefined;
  const p0 = inner?.results?.[0] as JsonObject | undefined;
  if (!p0) return undefined;
  const block = p0[cmdKey] as JsonObject | undefined;
  return block;
}

function firstDesignResult(out: readonly unknown[], cmdKey: string): unknown {
  const row = out[0] as { readKitDesignCommands?: { results?: readonly unknown[] } };
  const r0 = row.readKitDesignCommands?.results?.[0] as JsonObject | undefined;
  if (!r0) return undefined;
  return r0[cmdKey];
}

// #endregion 🧰ReadHelpers

// #region 📦LiveKitRoot

/** @emoji 🧭 Graph-shaped reads routed through {@link KitStore.read} (no legacy JS kit graph). */
export class LiveKitRoot {
  constructor(
    private readonly ks: KitStore,
    private readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  piece(designId: string, pieceId: string): LivePiece {
    return new LivePiece(this.ks, this.readPoint, designId, pieceId);
  }

  design(designId: string): LiveDesign {
    return new LiveDesign(this.ks, this.readPoint, designId);
  }

  type(typeId: string): LiveType {
    return new LiveType(this.ks, this.readPoint, typeId);
  }

  async readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]> {
    const store = await this.ks.kitStoreGraphqlFieldsForReadPoint(this.readPoint, kitGqlKitTypesRelay("id connectors { edges { node { id } } }"));
    const types = kitGraphqlJsonToReadonlyArray(store["types"] as JsonValue);
    const out: KitColoredConnectorRowDto[] = [];
    for (const t of types) {
      if (!t || typeof t !== "object" || Array.isArray(t)) continue;
      const to = t as JsonObject;
      const tid = String(to["id"] ?? "");
      const conns = kitGraphqlJsonToReadonlyArray(to["connectors"] as JsonValue);
      for (const c of conns) {
        if (!c || typeof c !== "object" || Array.isArray(c)) continue;
        const co = c as JsonObject;
        const cid = String(co["id"] ?? "");
        if (tid && cid) out.push({ typeId: { id: tid }, connectorId: { id: cid }, color: "" });
      }
    }
    return out;
  }
}

class LivePiece {
  constructor(
    private readonly ks: KitStore,
    private readonly readPoint: KitReadPoint,
    private readonly designId: string,
    private readonly pieceId: string,
  ) {}

  private run(cmd: ReadPieceCommand): Promise<ReadBatchResult> {
    const batch: ReadBatch = [
      {
        readKitDesignCommands: {
          id: { id: this.designId },
          commands: [{ readDesignPieceCommands: { id: { id: this.pieceId }, commands: [cmd] } }],
        },
      },
    ];
    return this.ks.read(this.readPoint, batch);
  }

  readFlatPlane(): Promise<PlaneDto | null> {
    return this.run({ readPieceFlatPlaneCommand: null }).then((out) => {
      const blk = firstDesignPieceResult(out, "readPieceFlatPlaneCommand") as { flatPlane?: PlaneDto | null } | undefined;
      return blk?.flatPlane ?? null;
    });
  }

  readFlatCenter(): Promise<CoordinateDto | null> {
    return this.run({ readPieceFlatCenterCommand: null }).then((out) => {
      const blk = firstDesignPieceResult(out, "readPieceFlatCenterCommand") as { flatCenter?: CoordinateDto | null } | undefined;
      return blk?.flatCenter ?? null;
    });
  }

  readParentConnectionFull(): Promise<ConnectionDto | null> {
    return this.run({ readPieceParentConnectionFullCommand: null }).then((out) => {
      const blk = firstDesignPieceResult(out, "readPieceParentConnectionFullCommand") as { connection?: ConnectionDto | null } | undefined;
      return blk?.connection ?? null;
    });
  }
}

class LiveDesign {
  constructor(
    private readonly ks: KitStore,
    private readonly readPoint: KitReadPoint,
    private readonly designId: string,
  ) {}

  private run(cmd: ReadDesignCommand): Promise<ReadBatchResult> {
    return this.ks.read(this.readPoint, [{ readKitDesignCommands: { id: { id: this.designId }, commands: [cmd] } }]);
  }

  readIncludedDesigns(): Promise<readonly IncludedDesignInfoDto[]> {
    return this.run({ readDesignIncludedDesignsCommand: null }).then((out) => {
      const blk = firstDesignResult(out, "readDesignIncludedDesignsCommand") as { designs?: readonly IncludedDesignInfoDto[] } | undefined;
      return blk?.designs ?? [];
    });
  }

  readClusterableGroups(selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    const cmd: ReadDesignCommand = {
      readDesignClusterableGroupsCommand: { selection: selection.map((id) => ({ id })) },
    };
    return this.run(cmd).then((out) => {
      const blk = firstDesignResult(out, "readDesignClusterableGroupsCommand") as { groups?: readonly (readonly KitIdDto[])[] } | undefined;
      return blk?.groups ?? [];
    });
  }

  readQualitySum(qualityId: string): Promise<number> {
    const cmd: ReadDesignCommand = { readDesignQualitySumCommand: { qualityId: { id: qualityId } } };
    return this.run(cmd).then((out) => {
      const s = (firstDesignResult(out, "readDesignQualitySumCommand") as { sum?: number } | undefined)?.sum;
      return typeof s === "number" && !Number.isNaN(s) ? s : 0;
    });
  }

  readReplaceableCatalog(selection: readonly string[]): Promise<{ types: string[]; designs: string[] }> {
    const cmd: ReadDesignCommand = {
      readDesignReplaceableCatalogCommand: { selection: selection.map((id) => ({ id })) },
    };
    return this.run(cmd).then((out) => {
      const blk = firstDesignResult(out, "readDesignReplaceableCatalogCommand") as { types?: readonly unknown[]; designs?: readonly unknown[] } | undefined;
      const toIds = (xs: readonly unknown[] | undefined) => (xs ?? []).map((x) => (typeof x === "string" ? x : (x as { id?: string })?.id)).filter((x): x is string => typeof x === "string");
      return { types: toIds(blk?.types), designs: toIds(blk?.designs) };
    });
  }

  readIncludedDesignIds(): Promise<string[]> {
    return this.run({ readDesignIncludedDesignIdsCommand: null }).then((out) => {
      const ids = (firstDesignResult(out, "readDesignIncludedDesignIdsCommand") as { designIds?: readonly unknown[] } | undefined)?.designIds;
      return (ids ?? []).map((x) => (typeof x === "string" ? x : (x as { id?: string }).id)).filter((x): x is string => typeof x === "string");
    });
  }
}

class LiveType {
  constructor(
    private readonly ks: KitStore,
    private readonly readPoint: KitReadPoint,
    private readonly typeId: string,
  ) {}

  readBestRepresentation(tagIds: readonly string[]): Promise<RepresentationDto | null> {
    return this.ks
      .read(this.readPoint, [
        {
          readKitTypeCommands: {
            id: { id: this.typeId },
            commands: [{ readTypeBestRepresentationCommand: { tagIds: [...tagIds] } }],
          },
        },
      ])
      .then((out) => {
        const row = out[0];
        if (row && "readKitTypeCommands" in row) {
          const r0 = row.readKitTypeCommands.results[0];
          if (r0 && "readTypeBestRepresentationCommand" in r0) return r0.readTypeBestRepresentationCommand.representation;
        }
        return null;
      });
  }
}

// #endregion 📦LiveKitRoot

// #region 🪜LiveReadHub
/** @emoji 🧾 Live-read snapshot hub: {@link getSemioKitLiveReadStore} in 🪜SemioKitLiveReadHub (after {@link WasmKitStoreClient} / {@link kitStoreFromKitStoreClient}). */
// #endregion 🪜LiveReadHub

// #region 🧰EventFilters

/** @emoji 🧭 Any kit graph mutation may flip undo/redo eligibility. */
export function kitEventAffectsCanUndoRedo(ev: KitEvent): boolean {
  void ev;
  return true;
}

/** @emoji 🧭 Live piece reads invalidate when the piece or its design changes. */
export function kitEventAffectsPieceLiveRead(ev: KitEvent, designId?: string, pieceId?: string): boolean {
  if (!designId || !pieceId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesPiece(ev as KitEvent, designId, pieceId);
}

/** @emoji 🧭 Replaceable catalog reads are design-scoped. */
export function kitEventAffectsReplaceableCatalogRead(ev: KitEvent, designId?: string, _selection?: ReadonlySet<string>): boolean {
  void _selection;
  if (!designId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesDesign(ev as KitEvent, designId);
}

/** @emoji 🧭 Design quality sum reads follow design-scoped invalidation. */
export function kitEventAffectsDesignQualitySumRead(ev: KitEvent, designId?: string, _qualityId?: string): boolean {
  void _qualityId;
  if (!designId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesDesign(ev as KitEvent, designId);
}

/** @emoji 🧭 Type-scoped reads follow kind-scoped invalidation. */
export function kitEventAffectsTypeScopedRead(ev: KitEvent, typeId?: string): boolean {
  if (!typeId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesType(ev as KitEvent, typeId);
}

/** @emoji 🧭 Colored connector rows follow kit-wide GraphQL reads; invalidate on any kit event. */
export function kitEventAffectsKitColoredConnectorsRead(ev: KitEvent): boolean {
  void ev;
  return true;
}

// #endregion 🧰EventFilters

// #region 📦WasmKitStoreClient

export class WasmKitStoreClient implements KitStoreClient {
  private readonly listeners = new Set<(ev: KitEvent) => void>();
  private readonly offKit: () => void;
  /** @emoji 🧭 Active read scope for {@link getPieces} and {@link fetchFullKit}. */
  kitReadPoint: KitReadPoint = theKitReadPoint;

  constructor(
    private readonly ks: KitStore,
    readPoint: KitReadPoint = theKitReadPoint,
  ) {
    this.kitReadPoint = readPoint;
    this.offKit = this.ks.subscribe((ev: KitEvent) => {
      for (const l of this.listeners) l(ev);
    });
  }

  setKitReadPoint(next: KitReadPoint): void {
    this.kitReadPoint = next;
    this.ks.setReadPoint(next);
    const ev = { Changed: null } as KitEvent;
    for (const l of this.listeners) l(ev);
  }

  /** @internal For read-store adapters. */
  internalKs(): KitStore {
    return this.ks;
  }

  /** @emoji 🧾 Authoritative full kit from `semio/rs` via GraphQL (no local DTO cache). */
  fetchFullKit(): Promise<KitFullDto> {
    return this.ks.readKitSnapshotForReadPoint(this.kitReadPoint);
  }

  subscribe(cb: (ev: KitEvent) => void): () => void {
    this.listeners.add(cb);
    return () => {
      this.listeners.delete(cb);
    };
  }

  dispose(): void {
    this.offKit();
    this.listeners.clear();
    void this.ks.dispose();
  }

  kitGraphql(): LiveKitRoot {
    return new LiveKitRoot(this.ks, this.kitReadPoint);
  }

  getKitWriteScope(): KitWriteScope | null {
    return this.ks.getKitWriteScope();
  }

  setKitWriteScope(scope: KitWriteScope | null): void {
    this.ks.setKitWriteScope(scope);
  }

  finalizeKitWriteTransaction(): Promise<SetResult> {
    return this.ks.finalizeKitWriteTransaction();
  }

  abortKitWriteTransaction(): Promise<SetResult> {
    return this.ks.abortKitWriteTransaction();
  }

  readPieceFlatPlane(designId: string, pieceId: string): Promise<PlaneDto | null> {
    return this.ks.piece(designId, pieceId, this.kitReadPoint).readFlatPlane();
  }

  readPieceFlatCenter(designId: string, pieceId: string): Promise<CoordinateDto | null> {
    return this.ks.piece(designId, pieceId, this.kitReadPoint).readFlatCenter();
  }

  readPieceParentConnectionFull(designId: string, pieceId: string): Promise<ConnectionDto | null> {
    return this.ks.piece(designId, pieceId, this.kitReadPoint).readParentConnectionFull();
  }

  readDesignIncludedDesigns(designId: string): Promise<readonly IncludedDesignInfoDto[]> {
    return this.ks.design(designId, this.kitReadPoint).readIncludedDesigns();
  }

  readDesignClusterableGroups(designId: string, selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    return this.ks.design(designId, this.kitReadPoint).readClusterableGroups(selection);
  }

  readDesignQualitySum(designId: string, qualityId: string): Promise<number> {
    return this.ks.design(designId, this.kitReadPoint).readQualitySum(qualityId);
  }

  readTypeBestRepresentation(typeId: string, tagIds: readonly string[]): Promise<RepresentationDto | null> {
    return this.ks.type(typeId, this.kitReadPoint).readBestRepresentation(tagIds);
  }

  readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]> {
    return new LiveKitRoot(this.ks, this.kitReadPoint).readColoredConnectors();
  }

  readDesignReplaceableCatalogTypes(designId: string, selection: readonly string[]): Promise<readonly string[]> {
    return this.ks.design(designId, this.kitReadPoint).readReplaceableCatalogTypes(selection);
  }

  readDesignReplaceableCatalogDesigns(designId: string, selection: readonly string[]): Promise<readonly string[]> {
    return this.ks.design(designId, this.kitReadPoint).readReplaceableCatalogDesigns(selection);
  }

  readDesignIncludedDesignIds(designId: string): Promise<readonly string[]> {
    return this.ks.design(designId, this.kitReadPoint).readIncludedDesignIds();
  }

  submitChangeKitCommands(commands: readonly ChangeKitCommand[]): Promise<SetResult> {
    return this.ks.submitChangeKitCommands(commands);
  }

  clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult> {
    return this.ks.clusterPieces(designId, pieceIds, clusterName);
  }

  dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.ks.dragPieces(designId, pieceIds, du, dv);
  }

  movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.ks.movePieces(designId, pieceIds, gap, shift, rise);
  }

  fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult> {
    return this.ks.fixPieces(designId, pieceIds);
  }

  flattenDesign(designId: string): Promise<SetResult> {
    return this.ks.flattenDesign(designId);
  }

  expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.ks.expandDesign(parentDesignId, nestedDesignId);
  }

  deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.ks.deleteConnection(designId, connectionId);
  }

  changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.ks.changePieceType(designId, pieceId, newTypeId);
  }

  pasteDesignSelection(designId: string, selection: KitJsonTreeDto, plane: PlaneDto | null): Promise<SetResult> {
    return this.ks.pasteDesignSelection(designId, selection, plane);
  }

  createHangingPieces(designId: string, typeIds: readonly string[], plane: PlaneDto): Promise<SetResult> {
    return this.ks.createHangingPieces(designId, typeIds, plane);
  }

  createConnectedPiece(designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    return this.ks.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort);
  }

  createFixedPiece(designId: string, typeId: string, plane: PlaneDto): Promise<SetResult> {
    return this.ks.createFixedPiece(designId, typeId, plane);
  }

  undo(): Promise<SetResult> {
    return this.ks.undo();
  }

  redo(): Promise<SetResult> {
    return this.ks.redo();
  }

  canUndo(): Promise<boolean> {
    return this.ks.canUndo();
  }

  canRedo(): Promise<boolean> {
    return this.ks.canRedo();
  }

  getPiecesMetadata(designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    return this.ks.getPiecesMetadata(this.kitReadPoint, designId);
  }

  getPieces(designId: string): Promise<readonly PieceDto[]> {
    return this.ks.getPieces(this.kitReadPoint, designId);
  }

  getConnections(designId: string): Promise<readonly ConnectionDto[]> {
    return this.ks.getConnections(this.kitReadPoint, designId);
  }

  getDesigns(): Promise<readonly DesignShallow[]> {
    return this.ks.getDesigns(this.kitReadPoint);
  }

  getTypes(): Promise<readonly TypeShallow[]> {
    return this.ks.getTypes(this.kitReadPoint);
  }

  getAuthors(): Promise<readonly AuthorMetadataDto[]> {
    return this.ks.getAuthors(this.kitReadPoint);
  }

  getKitMetadata(): Promise<KitMetadataDto | null> {
    return this.ks.getKitMetadata(this.kitReadPoint);
  }

  backboneStatus(): Promise<BackboneStatusDto> {
    return this.ks.backboneStatus();
  }

  attachBackbone(cfg: BackboneConfig): Promise<SetResult> {
    return this.ks.attachBackbone(cfg);
  }

  detachBackbone(): Promise<SetResult> {
    return this.ks.detachBackbone();
  }

  listConflicts(): Promise<KitConflict[]> {
    return this.ks.listConflicts();
  }

  resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> {
    return this.ks.resolveConflict(id, strategy);
  }

  syncNow(): Promise<SetResult> {
    return this.ks.syncNow();
  }

  kitField<T>(
    cacheKey: string,
    spec: {
      extraVariableDecl?: string;
      extraVariables?: GraphQlVariables;
      innerOnKit: string;
      parse: (kitFragment: JsonObject) => T;
      initial: T;
    },
  ): StoreField<T> {
    return this.ks.kitField(cacheKey, spec);
  }

  get kitName(): StoreField<string> {
    return this.ks.kitName;
  }

  get renameKit(): StoreCommand<RenameKitCommandArgs> {
    return this.ks.renameKit;
  }

  readKitName(): Promise<string> {
    return this.ks.readKitName();
  }

  createAlternativeFromTip(name: string, sourceAlternativeId: string | null): Promise<string> {
    return this.ks.createAlternativeFromTip(name, sourceAlternativeId);
  }
}

/** @emoji 🧾 Resolves the live {@link KitStore} behind a {@link KitStoreClient}, or `null` for non-WASM bridges. */
export function kitStoreFromKitStoreClient(client: KitStoreClient): KitStore | null {
  if (client instanceof WasmKitStoreClient) return client.internalKs();
  const probe = client as { internalKs?: () => KitStore };
  return probe.internalKs?.() ?? null;
}

// #region 🪜SemioKitLiveReadHub

/** 🧾 Default {@link SemioKitLiveReadStore#getSnapshot} when a key has not polled yet (stable identity for consumers). */
const SEMIO_KIT_LIVE_READ_EMPTY: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: Object.freeze([]) as readonly unknown[],
  pending: 0,
}) as KitStoreReadSnap;

const SEMIO_KIT_DESIGN_READ_EMPTY_META: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: Object.freeze({}) as unknown,
  pending: 0,
}) as KitStoreReadSnap;

/**
 * @emoji 🧾 Hub for async GraphQL-backed kit reads: {@link KitStoreClient} subscription fan-out, per-key
 * {@link KitStoreReadSnap}, and invalidation predicates — {@link useSyncExternalStore} wires in `@semio/react`.
 */
export class SemioKitLiveReadStore {
  private readonly snap = new Map<string, KitStoreReadSnap>();
  private readonly regs: Array<{
    key: string;
    fetch: () => Promise<unknown>;
    affects: (ev: KitEvent) => boolean;
    onChange: () => void;
  }> = [];
  private off: (() => void) | undefined;

  constructor(private readonly client: KitStoreClient) {
    this.off = client.subscribe((ev: KitEvent) => {
      for (const r of this.regs) {
        if (r.affects(ev)) void this.poll(r);
      }
    });
  }

  subscribe(key: string, fetch: () => Promise<unknown>, affects: (ev: KitEvent) => boolean, onChange: () => void): () => void {
    const r = { key, fetch, affects, onChange };
    this.regs.push(r);
    void this.poll(r);
    return () => {
      const i = this.regs.indexOf(r);
      if (i >= 0) this.regs.splice(i, 1);
    };
  }

  getSnapshot(key: string): KitStoreReadSnap {
    return this.snap.get(key) ?? SEMIO_KIT_LIVE_READ_EMPTY;
  }

  private async poll(r: { key: string; fetch: () => Promise<unknown>; onChange: () => void }): Promise<void> {
    const cur = this.snap.get(r.key) ?? SEMIO_KIT_LIVE_READ_EMPTY;
    this.snap.set(r.key, { version: cur.version, data: cur.data, pending: cur.pending + 1 });
    r.onChange();
    try {
      const data = await r.fetch();
      this.snap.set(r.key, { version: cur.version + 1, data, pending: 0 });
      r.onChange();
    } catch {
      this.snap.set(r.key, { version: cur.version, data: cur.data, pending: 0 });
      r.onChange();
    }
  }

  dispose(): void {
    this.off?.();
    this.off = undefined;
    this.regs.length = 0;
    this.snap.clear();
  }
}

const liveReadHubs = new WeakMap<KitStoreClient, SemioKitLiveReadStore>();

export function getSemioKitLiveReadStore(c: KitStoreClient): SemioKitLiveReadStore {
  let h = liveReadHubs.get(c);
  if (!h) {
    h = new SemioKitLiveReadStore(c);
    liveReadHubs.set(c, h);
  }
  return h;
}

function viewStoreKey(k: KitViewCatalogKey): string {
  return `view:${k}`;
}

async function fetchViewCatalogSnapshot(client: KitStoreClient, key: KitViewCatalogKey): Promise<unknown> {
  const ks = kitStoreFromKitStoreClient(client);
  const scope = getKitClientReadPoint(client);
  if (!ks) {
    if (key === "typeIds" || key === "designIds") return [];
    return [];
  }
  if (key === "typeIds") return ks.kindRowIds(scope);
  if (key === "designIds") return ks.designRowIds(scope);
  if (key === "typesMetadata") {
    const rows = await ks.kindMetadataRows(scope);
    return (rows as readonly { id?: string; name?: string }[]).map((t) => ({ id: String(t.id ?? ""), name: String(t.name ?? "") }));
  }
  const rows = await ks.designMetadataRows(scope);
  return (rows as readonly { id?: string; name?: string }[]).map((d) => ({ id: String(d.id ?? ""), name: String(d.name ?? "") }));
}

/** @emoji 🧾 View-catalog reads via async {@link KitStore}. */
export class SemioKitViewStore {
  private readonly hub: SemioKitLiveReadStore;
  constructor(private readonly client: KitStoreClient) {
    this.hub = getSemioKitLiveReadStore(client);
  }

  subscribe(_key: KitViewCatalogKey, onChange: () => void): () => void {
    const keys: KitViewCatalogKey[] = ["typeIds", "designIds", "typesMetadata", "designsMetadata"];
    const unsubs = keys.map((k) =>
      this.hub.subscribe(
        viewStoreKey(k),
        () => fetchViewCatalogSnapshot(this.client, k),
        () => true,
        onChange,
      ),
    );
    return () => {
      for (const u of unsubs) u();
    };
  }

  getSnapshot(key: KitViewCatalogKey): unknown {
    return this.hub.getSnapshot(viewStoreKey(key)).data;
  }
}

const viewStores = new WeakMap<KitStoreClient, SemioKitViewStore>();

export function getSemioKitViewStore(c: KitStoreClient): SemioKitViewStore {
  let v = viewStores.get(c);
  if (!v) {
    v = new SemioKitViewStore(c);
    viewStores.set(c, v);
  }
  return v;
}

function designReadKey(designId: string, field: KitDesignReadKind): string {
  return `design:${designId}:${field}`;
}

/** @emoji 🧾 Per-design list/metadata reads on {@link KitStoreClient}. */
export class SemioKitDesignReadStore {
  private readonly hub: SemioKitLiveReadStore;

  constructor(private readonly client: KitStoreClient) {
    this.hub = getSemioKitLiveReadStore(client);
  }

  subscribe(designId: string, field: KitDesignReadKind, onChange: () => void): () => void {
    return this.hub.subscribe(
      designReadKey(designId, field),
      async () => {
        const list = await this.client.getPieces(designId);
        const conns = await this.client.getConnections(designId);
        if (field === "metadata") {
          const meta: { [k: string]: JsonValue } = {};
          for (const p of list) {
            if (p && typeof p === "object" && p !== null && "id" in p) meta[String((p as { id: string }).id)] = p as unknown as JsonValue;
          }
          return meta;
        }
        if (field === "pieces") return [...list];
        return [...conns];
      },
      (ev) => kitEventTouchesDesign(ev as KitEvent, designId),
      onChange,
    );
  }

  getSnapshot(designId: string, field: KitDesignReadKind): KitStoreReadSnap {
    const s = this.hub.getSnapshot(designReadKey(designId, field));
    if (field === "metadata" && s.version === 0 && s.pending === 0 && Array.isArray(s.data) && s.data.length === 0) {
      return SEMIO_KIT_DESIGN_READ_EMPTY_META;
    }
    return s;
  }
}

const designStores = new WeakMap<KitStoreClient, SemioKitDesignReadStore>();

export function getSemioKitDesignReadStore(c: KitStoreClient): SemioKitDesignReadStore {
  let d = designStores.get(c);
  if (!d) {
    d = new SemioKitDesignReadStore(c);
    designStores.set(c, d);
  }
  return d;
}

function shallowListKey(kind: KitShallowListKind): string {
  return `shallow:${kind}`;
}

/** @emoji 🧾 Shallow entity lists (designs / kinds / authors). */
export class SemioKitShallowListReadStore {
  private readonly hub: SemioKitLiveReadStore;
  constructor(private readonly client: KitStoreClient) {
    this.hub = getSemioKitLiveReadStore(client);
  }

  subscribe(kind: KitShallowListKind, onChange: () => void): () => void {
    return this.hub.subscribe(
      shallowListKey(kind),
      async () => {
        if (kind === "designs") return [...(await this.client.getDesigns())];
        if (kind === "types") return [...(await this.client.getTypes())];
        return [...(await this.client.getAuthors())];
      },
      () => true,
      onChange,
    );
  }

  getSnapshot(kind: KitShallowListKind): KitStoreReadSnap {
    const s = this.hub.getSnapshot(shallowListKey(kind));
    return s;
  }
}

const shallowStores = new WeakMap<KitStoreClient, SemioKitShallowListReadStore>();

export function getSemioKitShallowListReadStore(c: KitStoreClient): SemioKitShallowListReadStore {
  let s = shallowStores.get(c);
  if (!s) {
    s = new SemioKitShallowListReadStore(c);
    shallowStores.set(c, s);
  }
  return s;
}

// #endregion 🪜SemioKitLiveReadHub

export async function createKitStoreClient(opts: { initialKit: KitFullDto; readPoint?: KitReadPoint }): Promise<KitStoreClient> {
  const ks = await KitStore.open(opts.initialKit);
  const c = new WasmKitStoreClient(ks, opts.readPoint);
  await c.fetchFullKit();
  return c;
}

const facades = new WeakMap<KitStoreClient, SemioKitCommandFacade>();

export function acquireSemioKitCommandFacade(client: KitStoreClient): SemioKitCommandFacade {
  let f = facades.get(client);
  if (!f) {
    f = {
      runMutation: async (cmd: KitTypedChangeKitCommandsBatch): Promise<SetResult> => {
        if (cmd.kind !== "changeKitCommands") return { ok: false, error: { kind: "NotSupported", message: "command" } };
        return client.submitChangeKitCommands(cmd.commands);
      },
    };
    facades.set(client, f);
  }
  return f;
}

export function releaseSemioKitCommandFacade(client: KitStoreClient): void {
  facades.delete(client);
}

// #region 🔖CommandBuilder

/** @emoji 🧭 Fluent `Mutation.session` navigator matching the target command tree; uses {@link KitWriteScope.changeId} / {@link KitStore.openKitWriteTransaction}. */
export class CommandBuilder {
  constructor(private readonly client: KitStoreClient) {}

  /** @emoji 🧭 Root `Mutation.session` scope. */
  session(): SessionCommandNav {
    return new SessionCommandNav(this.client);
  }
}

/** @emoji 🧭 `SessionCommandInput` façade (subset wired to rs today). */
export class SessionCommandNav {
  constructor(private readonly client: KitStoreClient) {}

  async start(): Promise<SetResult> {
    void this.client;
    return { ok: false, error: { kind: "NotSupported", message: "session.start is not wired in this bundle yet" } };
  }

  async end(): Promise<SetResult> {
    void this.client;
    return { ok: false, error: { kind: "NotSupported", message: "session.end is not wired in this bundle yet" } };
  }

  async login(username: string, passwordHash: string, hubUrl?: string): Promise<SetResult> {
    void username;
    void passwordHash;
    void hubUrl;
    return { ok: false, error: { kind: "NotSupported", message: "session.login is not wired in this bundle yet" } };
  }

  async logout(): Promise<SetResult> {
    void this.client;
    return { ok: false, error: { kind: "NotSupported", message: "session.logout is not wired in this bundle yet" } };
  }

  theKit(): VersionCommandNav {
    return new VersionCommandNav(this.client);
  }

  alternative(id: string): AlternativeCommandNav {
    return new AlternativeCommandNav(this.client, id);
  }

  async startAlternative(name?: string): Promise<SetResult> {
    try {
      const altId = await this.client.createAlternativeFromTip(name ?? "", null);
      return { ok: true, requestId: altId };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }
}

/** @emoji 🧭 `VersionCommandInput` façade (`Session.theKit` version lane). */
export class VersionCommandNav {
  constructor(private readonly client: KitStoreClient) {}

  async startNewChange(): Promise<SetResult> {
    const ks = kitStoreFromKitStoreClient(this.client);
    if (!ks) return { ok: false, error: { kind: "NotSupported", message: "CommandBuilder requires WasmKitStoreClient / KitStore" } };
    const opened = await ks.openKitWriteTransaction();
    if (!opened.ok) return opened;
    return { ok: true, requestId: opened.changeId };
  }

  unsavedChange(changeId: ChangeId): UnsavedChangeCommandNav {
    void changeId;
    return new UnsavedChangeCommandNav(this.client);
  }

  async save(): Promise<SetResult> {
    return this.client.finalizeKitWriteTransaction();
  }

  async createCheckpoint(_message: string): Promise<SetResult> {
    void _message;
    return { ok: false, error: { kind: "NotSupported", message: "createCheckpoint awaits target-schema Checkpoint wiring" } };
  }
}

/** @emoji 🧭 `UnsavedChangeCommandInput` façade (nested kit ops land in Worker E). */
export class UnsavedChangeCommandNav {
  constructor(private readonly client: KitStoreClient) {}

  kit(): KitOperationNav {
    return new KitOperationNav(this.client);
  }

  async save(): Promise<SetResult> {
    return this.client.finalizeKitWriteTransaction();
  }
}

/** @emoji 🧭 `KitOperationInput` façade — delegates to existing {@link KitStoreClient} RPCs where they exist. */
export class KitOperationNav {
  constructor(private readonly client: KitStoreClient) {}

  async rename(newName: string): Promise<SetResult> {
    return this.client.renameKit.run({ scope: {} as Record<string, never>, input: { name: newName } });
  }

  async changeDescription(newDescription: string): Promise<SetResult> {
    const cmds = buildSchemaEntityChangeCommands("Kit", String((await this.client.fetchFullKit()).id ?? ""), "description", newDescription, null);
    return submitKitChangeCommands(this.client, cmds);
  }
}

/** @emoji 🧭 `AlternativeCommandInput` façade. */
export class AlternativeCommandNav {
  constructor(
    private readonly client: KitStoreClient,
    private readonly alternativeId: string,
  ) {}

  async version(): Promise<SetResult> {
    void this.alternativeId;
    return { ok: false, error: { kind: "NotSupported", message: "alternative.version awaits rs wiring" } };
  }

  async integrateIntoTheKit(): Promise<SetResult> {
    void this.alternativeId;
    return { ok: false, error: { kind: "NotSupported", message: "integrateIntoTheKit awaits rs wiring" } };
  }
}

// #endregion 🔖CommandBuilder

// #endregion 📦WasmKitStoreClient

// #region 🧩KitEntitiesMerged
// #region Constants
// Global constants MUST define shared numeric parameters.

/** Standard icon width in pixels.
 **/
export const ICON_WIDTH = 50;
/**
 * Numeric tolerance for floating-point comparisons.
 **/
export const TOLERANCE = 1e-5;

// #endregion Constants

// #region Utilities
// Removed: toArray, SeededRandom, Generator, round, jaccard, deepEqual, arraysEqual — domain logic moved to semio/rs (Requirements 1.3, 3.5)

/**
 * Zod schema for DiffStatus validation.
 **/
export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

/**
 * Enumeration of DiffStatus values.
 **/
export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}

/**
 * Type alias for Id.
 **/
export type Id = string;

// #endregion Utilities

// #region Entity IDs
// Entity identifier types and comparison functions MUST be defined here.

export type AttributeIdDto = Readonly<{ readonly id: Id }>;
export type LocationIdDto = Readonly<{ readonly id: Id }>;
export type AuthorIdDto = Readonly<{ readonly id: Id }>;
export type FileIdDto = Readonly<{ readonly id: Id }>;
export type FolderIdDto = Readonly<{ readonly id: Id }>;
export type BenchmarkIdDto = Readonly<{ readonly id: Id }>;
export type QualityIdDto = Readonly<{ readonly id: Id }>;
export type PortIdDto = Readonly<{ readonly id: Id }>;
export type PropIdDto = Readonly<{ readonly id: Id }>;
export type RepresentationIdDto = Readonly<{ readonly id: Id }>;
export type ConnectorIdDto = Readonly<{ readonly id: Id }>;
export type TypeIdDto = Readonly<{ readonly id: Id }>;
export type LayerIdDto = Readonly<{ readonly id: Id }>;
export type PieceIdDto = Readonly<{ readonly id: Id }>;
export type GroupIdDto = Readonly<{ readonly id: Id }>;
export type ConnectionIdDto = Readonly<{ readonly id: Id }>;
export type StatIdDto = Readonly<{ readonly id: Id }>;
export type DesignIdDto = Readonly<{ readonly id: Id }>;
export type KitIdDto = Readonly<{ readonly id: Id }>;
export type TagIdDto = Readonly<{ readonly id: Id }>;
export type ConceptIdDto = Readonly<{ readonly id: Id }>;
export type FamilyIdDto = Readonly<{ readonly id: Id }>;

export const AttributeIdSchema = z.object({ id: z.string() });
export const LocationIdSchema = z.object({ id: z.string() });
export const AuthorIdSchema = z.object({ id: z.string() });
export const FileIdSchema = z.object({ id: z.string(), hash: z.string().optional() });
export const FolderIdSchema = z.object({ id: z.string() });
export const BenchmarkIdSchema = z.object({ id: z.string() });
export const QualityIdSchema = z.object({ id: z.string() });
export const PortIdSchema = z.object({ id: z.string() });
export const PropIdSchema = z.object({ id: z.string() });
export const RepresentationIdSchema = z.object({ id: z.string() });
export const ConnectorIdSchema = z.object({ id: z.string() });
export const TypeIdSchema = z.object({ id: z.string() });
export const LayerIdSchema = z.object({ id: z.string() });
export const PieceIdSchema = z.object({ id: z.string() });
export const GroupIdSchema = z.object({ id: z.string() });
export const ConnectionIdSchema = z.object({ id: z.string() });
export const StatIdSchema = z.object({ id: z.string() });
export const DesignIdSchema = z.object({ id: z.string() });
export const KitIdSchema = z.object({ id: z.string() });
export const TagIdSchema = z.object({ id: z.string() });
export const ConceptIdSchema = z.object({ id: z.string() });
export const FamilyIdSchema = z.object({ id: z.string() });

// Removed: All free create*Id, areSame*Id, get*Id functions — use Entity.createId/areSameId static methods (Requirement 3.2)

// #endregion Entity IDs

// #region Weak Entities

// #region Coordinate
export const CoordinateSchema = z.object({ u: z.number(), v: z.number() });
export type CoordinateDto = ReadonlyDto<z.infer<typeof CoordinateSchema>>;
export class Coordinate implements CoordinateDto {
  u!: number;
  v!: number;
  constructor(dto: CoordinateDto) {
    Object.assign(this, CoordinateSchema.parse(dto));
  }
  static from(dto: CoordinateDto): Coordinate {
    return new Coordinate(dto);
  }
  toDto(): CoordinateDto {
    return CoordinateSchema.parse(this as CoordinateDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Coordinate {
    return new Coordinate(CoordinateSchema.parse(JSON.parse(json)));
  }
}
export const CoordinateDiffSchema = CoordinateSchema.partial();
export type CoordinateDiff = ReadonlyDto<z.infer<typeof CoordinateDiffSchema>>;
// #endregion Coordinate

// #region Vec
export const VecSchema = z.object({ u: z.number(), v: z.number() });
export type VecDto = ReadonlyDto<z.infer<typeof VecSchema>>;
export class Vec implements VecDto {
  u!: number;
  v!: number;
  constructor(dto: VecDto) {
    Object.assign(this, VecSchema.parse(dto));
  }
  static from(dto: VecDto): Vec {
    return new Vec(dto);
  }
  static fromDto(dto: VecDto): Vec {
    return new Vec(dto);
  }
  toDto(): VecDto {
    return VecSchema.parse(this as VecDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Vec {
    return new Vec(VecSchema.parse(JSON.parse(json)));
  }
}
export const VecDiffSchema = VecSchema.partial();
export type VecDiff = ReadonlyDto<z.infer<typeof VecDiffSchema>>;
// #endregion Vec

// #region Point
export const PointSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type PointDto = ReadonlyDto<z.infer<typeof PointSchema>>;
export class Point implements PointDto {
  x!: number;
  y!: number;
  z!: number;
  constructor(dto: PointDto) {
    Object.assign(this, PointSchema.parse(dto));
  }
  static from(dto: PointDto): Point {
    return new Point(dto);
  }
  static fromDto(dto: PointDto): Point {
    return new Point(dto);
  }
  toDto(): PointDto {
    return PointSchema.parse(this as PointDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Point {
    return new Point(PointSchema.parse(JSON.parse(json)));
  }
}
export const PointDiffSchema = PointSchema.partial();
export type PointDiff = ReadonlyDto<z.infer<typeof PointDiffSchema>>;
// #endregion Point

// #region Vector
export const VectorSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type VectorDto = ReadonlyDto<z.infer<typeof VectorSchema>>;
export class Vector implements VectorDto {
  x!: number;
  y!: number;
  z!: number;
  constructor(dto: VectorDto) {
    Object.assign(this, VectorSchema.parse(dto));
  }
  static from(dto: VectorDto): Vector {
    return new Vector(dto);
  }
  static fromDto(dto: VectorDto): Vector {
    return new Vector(dto);
  }
  toDto(): VectorDto {
    return VectorSchema.parse(this as VectorDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Vector {
    return new Vector(VectorSchema.parse(JSON.parse(json)));
  }
}
export const VectorDiffSchema = VectorSchema.partial();
export type VectorDiff = ReadonlyDto<z.infer<typeof VectorDiffSchema>>;
// #endregion Vector

// #region Plane
export const PlaneSchema = z.object({ origin: PointSchema, xAxis: VectorSchema, yAxis: VectorSchema });
export type PlaneDto = ReadonlyDto<z.infer<typeof PlaneSchema>>;
export class Plane implements PlaneDto {
  origin!: Point;
  xAxis!: Vector;
  yAxis!: Vector;
  constructor(dto: PlaneDto) {
    const p = PlaneSchema.parse(dto);
    this.origin = new Point(p.origin);
    this.xAxis = new Vector(p.xAxis);
    this.yAxis = new Vector(p.yAxis);
  }
  static from(dto: PlaneDto): Plane {
    return new Plane(dto);
  }
  static fromDto(dto: PlaneDto): Plane {
    return new Plane(dto);
  }
  toDto(): PlaneDto {
    return PlaneSchema.parse(this as PlaneDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Plane {
    return new Plane(PlaneSchema.parse(JSON.parse(json)));
  }
  // Removed: averageWith, average, rounded — geometry computation moved to semio/rs (Requirement 1.14)
}
export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true }).extend({ origin: PointDiffSchema, xAxis: VectorDiffSchema, yAxis: VectorDiffSchema }).partial();
export type PlaneDiff = ReadonlyDto<z.infer<typeof PlaneDiffSchema>>;
// #endregion Plane

// #region Camera
export const CameraSchema = z.object({ position: PointSchema, forward: VectorSchema, up: VectorSchema });
export type CameraDto = ReadonlyDto<z.infer<typeof CameraSchema>>;
export class Camera implements CameraDto {
  position!: Point;
  forward!: Vector;
  up!: Vector;
  constructor(dto: CameraDto) {
    const p = CameraSchema.parse(dto);
    this.position = new Point(p.position);
    this.forward = new Vector(p.forward);
    this.up = new Vector(p.up);
  }
  static from(dto: CameraDto): Camera {
    return new Camera(dto);
  }
  static fromDto(dto: CameraDto): Camera {
    return new Camera(dto);
  }
  toDto(): CameraDto {
    return CameraSchema.parse(this as CameraDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Camera {
    return new Camera(CameraSchema.parse(JSON.parse(json)));
  }
}
export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true }).extend({ position: PointDiffSchema, forward: VectorDiffSchema, up: VectorDiffSchema }).partial();
export type CameraDiff = ReadonlyDto<z.infer<typeof CameraDiffSchema>>;
// #endregion Camera

// #endregion Weak Entities

// #region Attribute
const DateProperty = () => z.string().optional();
export const AttributeSchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), definition: z.string().optional() });
export type AttributeDto = ReadonlyDto<z.infer<typeof AttributeSchema>>;
export class Attribute implements AttributeDto {
  id!: string;
  key!: string;
  value?: string;
  definition?: string;
  constructor(dto: AttributeDto) {
    Object.assign(this, AttributeSchema.parse(dto));
  }
  static from(dto: AttributeDto): Attribute {
    return new Attribute(dto);
  }
  static fromDto(dto: AttributeDto): Attribute {
    return new Attribute(dto);
  }
  static createId(id: string): AttributeIdDto {
    return { id };
  }
  static areSameId(a: AttributeIdDto, b: AttributeIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): AttributeDto {
    return AttributeSchema.parse(this as AttributeDto);
  }
  toJson(): string {
    return JSON.stringify(this.toDto());
  }
  static fromJson(json: string): Attribute {
    return new Attribute(AttributeSchema.parse(JSON.parse(json)));
  }
}
export const AttributeMetadataDtoSchema = AttributeSchema;
export type AttributeMetadataDto = ReadonlyDto<z.infer<typeof AttributeMetadataDtoSchema>>;
export const AttributeShallowSchema = AttributeSchema;
export type AttributeShallow = ReadonlyDto<z.infer<typeof AttributeShallowSchema>>;
export const AttributeDiffSchema = AttributeSchema.partial();
export type AttributeDiff = ReadonlyDto<z.infer<typeof AttributeDiffSchema>>;
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type AttributesDiff = ReadonlyDto<z.infer<typeof AttributesDiffSchema>>;
// #endregion Attribute

// #region Location
export const LocationSchema = z.object({ id: z.string(), longitude: z.number().optional(), latitude: z.number().optional(), altitude: z.number().optional(), attributes: z.array(AttributeSchema).optional() });
export type LocationDto = ReadonlyDto<z.infer<typeof LocationSchema>>;
export class Location implements LocationDto {
  id!: string;
  longitude?: number;
  latitude?: number;
  altitude?: number;
  attributes?: Attribute[];
  constructor(dto: LocationDto) {
    const p = LocationSchema.parse(dto);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: LocationDto): Location {
    return new Location(dto);
  }
  static fromDto(dto: LocationDto): Location {
    return new Location(dto);
  }
  static createId(id: string): LocationIdDto {
    return { id };
  }
  static areSameId(a: LocationIdDto, b: LocationIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): LocationDto {
    return LocationSchema.parse(this as LocationDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Location {
    return new Location(LocationSchema.parse(JSON.parse(json)));
  }
}
export const LocationMetadataDtoSchema = LocationSchema;
export type LocationMetadataDto = ReadonlyDto<z.infer<typeof LocationMetadataDtoSchema>>;
export const LocationShallowSchema = LocationSchema;
export type LocationShallow = ReadonlyDto<z.infer<typeof LocationShallowSchema>>;
export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LocationDiff = ReadonlyDto<z.infer<typeof LocationDiffSchema>>;
// #endregion Location

// #region Author
export const AuthorSchema = z.object({ id: z.string(), name: z.string(), email: z.string().optional(), role: z.string().optional(), rank: z.number().optional() });
export type AuthorDto = ReadonlyDto<z.infer<typeof AuthorSchema>>;
export class Author implements AuthorDto {
  id!: string;
  name!: string;
  email?: string;
  role?: string;
  rank?: number;
  constructor(dto: AuthorDto) {
    Object.assign(this, AuthorSchema.parse(dto));
  }
  static from(dto: AuthorDto): Author {
    return new Author(dto);
  }
  static fromDto(dto: AuthorDto): Author {
    return new Author(dto);
  }
  static createId(id: string): AuthorIdDto {
    return { id };
  }
  static areSameId(a: AuthorIdDto, b: AuthorIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): AuthorDto {
    return AuthorSchema.parse(this as AuthorDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Author {
    return new Author(AuthorSchema.parse(JSON.parse(json)));
  }
}
export const AuthorMetadataDtoSchema = AuthorSchema;
export type AuthorMetadataDto = ReadonlyDto<z.infer<typeof AuthorMetadataDtoSchema>>;
export const AuthorShallowSchema = AuthorSchema;
export type AuthorShallow = ReadonlyDto<z.infer<typeof AuthorShallowSchema>>;
export const AuthorDiffSchema = AuthorSchema.partial();
export type AuthorDiff = ReadonlyDto<z.infer<typeof AuthorDiffSchema>>;
export const AuthorsDiffSchema = z.object({ removed: z.array(AuthorIdSchema).optional(), updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type AuthorsDiff = ReadonlyDto<z.infer<typeof AuthorsDiffSchema>>;
// #endregion Author

// #region File
export const FileSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  folder: FolderIdSchema.optional(),
  url: z.string().optional(),
  remote: z.string().optional(),
  mime: z.string().optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  /** Content-addressed Blake3 hex referencing [`blobs`] row (`hash`), when payload is stored outside the projection JSON. */
  blobHash: z.string().optional(),
  description: z.string().optional(),
  blob: z.union([z.string(), z.custom<Blob>((v) => typeof Blob !== "undefined" && v instanceof Blob)]).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type FileDto = ReadonlyDto<z.infer<typeof FileSchema>>;
export class File implements FileDto {
  id!: string;
  name?: string;
  folder?: { id: string };
  url?: string;
  remote?: string;
  mime?: string;
  size?: number;
  hash?: string;
  blobHash?: string;
  description?: string;
  blob?: string | Blob;
  createdAt?: string;
  updatedAt?: string;
  constructor(dto: FileDto) {
    Object.assign(this, FileSchema.parse(dto));
  }
  static from(dto: FileDto): File {
    return new File(dto);
  }
  static fromDto(dto: FileDto): File {
    return new File(dto);
  }
  static createId(id: string): FileIdDto {
    return { id };
  }
  static areSameId(a: FileIdDto, b: FileIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): FileDto {
    return FileSchema.parse(this as FileDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): File {
    return new File(FileSchema.parse(JSON.parse(json)));
  }
}
export const FileMetadataDtoSchema = FileSchema;
export type FileMetadataDto = ReadonlyDto<z.infer<typeof FileMetadataDtoSchema>>;
export const FileShallowSchema = FileSchema;
export type FileShallow = ReadonlyDto<z.infer<typeof FileShallowSchema>>;
export const FileDiffSchema = FileSchema.partial();
export type FileDiff = ReadonlyDto<z.infer<typeof FileDiffSchema>>;
export const FilesDiffSchema = z.object({ removed: z.array(FileIdSchema).optional(), updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FilesDiff = ReadonlyDto<z.infer<typeof FilesDiffSchema>>;
// #endregion File

// #region Folder
export const FolderSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  parent: z.object({ id: z.string() }).optional(),
  path: z.string().optional(),
  description: z.string().optional(),
});
export type FolderDto = ReadonlyDto<z.infer<typeof FolderSchema>>;
export class Folder implements FolderDto {
  id!: string;
  name?: string;
  parent?: { id: string };
  path?: string;
  description?: string;
  constructor(dto: FolderDto) {
    Object.assign(this, FolderSchema.parse(dto));
  }
  static from(dto: FolderDto): Folder {
    return new Folder(dto);
  }
  static fromDto(dto: FolderDto): Folder {
    return new Folder(dto);
  }
  static createId(id: string): FolderIdDto {
    return { id };
  }
  static areSameId(a: FolderIdDto, b: FolderIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): FolderDto {
    return FolderSchema.parse(this as FolderDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Folder {
    return new Folder(FolderSchema.parse(JSON.parse(json)));
  }
}
export const FolderMetadataDtoSchema = FolderSchema;
export type FolderMetadataDto = ReadonlyDto<z.infer<typeof FolderMetadataDtoSchema>>;
export const FolderShallowSchema = FolderSchema;
export type FolderShallow = ReadonlyDto<z.infer<typeof FolderShallowSchema>>;
export const FolderDiffSchema = FolderSchema.partial();
export type FolderDiff = ReadonlyDto<z.infer<typeof FolderDiffSchema>>;
export const FoldersDiffSchema = z.object({ removed: z.array(FolderIdSchema).optional(), updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FoldersDiff = ReadonlyDto<z.infer<typeof FoldersDiffSchema>>;
// #endregion Folder

// #region Benchmark
export const BenchmarkSchema = z.object({ id: z.string(), name: z.string(), min: z.number().optional(), max: z.number().optional(), minExcluded: z.boolean().optional(), maxExcluded: z.boolean().optional() });
export type BenchmarkDto = ReadonlyDto<z.infer<typeof BenchmarkSchema>>;
export class Benchmark implements BenchmarkDto {
  id!: string;
  name!: string;
  min?: number;
  max?: number;
  minExcluded?: boolean;
  maxExcluded?: boolean;
  constructor(dto: BenchmarkDto) {
    Object.assign(this, BenchmarkSchema.parse(dto));
  }
  static from(dto: BenchmarkDto): Benchmark {
    return new Benchmark(dto);
  }
  static fromDto(dto: BenchmarkDto): Benchmark {
    return new Benchmark(dto);
  }
  static createId(id: string): BenchmarkIdDto {
    return { id };
  }
  static areSameId(a: BenchmarkIdDto, b: BenchmarkIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): BenchmarkDto {
    return BenchmarkSchema.parse(this as BenchmarkDto);
  }
  toJson(): string {
    return JSON.stringify(this.toDto());
  }
  static fromJson(json: string): Benchmark {
    return new Benchmark(BenchmarkSchema.parse(JSON.parse(json)));
  }
}
export const BenchmarkMetadataDtoSchema = BenchmarkSchema;
export type BenchmarkMetadataDto = ReadonlyDto<z.infer<typeof BenchmarkMetadataDtoSchema>>;
export const BenchmarkShallowSchema = BenchmarkSchema;
export type BenchmarkShallow = ReadonlyDto<z.infer<typeof BenchmarkShallowSchema>>;
export const BenchmarkDiffSchema = BenchmarkSchema.partial();
export type BenchmarkDiff = ReadonlyDto<z.infer<typeof BenchmarkDiffSchema>>;
export const BenchmarksDiffSchema = z.object({ removed: z.array(BenchmarkIdSchema).optional(), updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type BenchmarksDiff = ReadonlyDto<z.infer<typeof BenchmarksDiffSchema>>;
// #endregion Benchmark

// #region Quality
export const QualitySchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  key: z.string(),
  folder: z.string().optional(),
  value: z.string().optional(),
  unit: z.string().optional(),
  definition: z.string().optional(),
  description: z.string().optional(),
  benchmarks: z.array(BenchmarkSchema).optional(),
});
export type QualityDto = ReadonlyDto<z.infer<typeof QualitySchema>>;
export class Quality implements QualityDto {
  id!: string;
  name?: string;
  key!: string;
  folder?: string;
  value?: string;
  unit?: string;
  definition?: string;
  description?: string;
  benchmarks?: Benchmark[];
  constructor(dto: QualityDto) {
    const p = QualitySchema.parse(dto);
    Object.assign(this, p);
    this.benchmarks = p.benchmarks?.map((b) => new Benchmark(b));
  }
  static from(dto: QualityDto): Quality {
    return new Quality(dto);
  }
  static fromDto(dto: QualityDto): Quality {
    return new Quality(dto);
  }
  static createId(id: string): QualityIdDto {
    return { id };
  }
  static areSameId(a: QualityIdDto, b: QualityIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): QualityDto {
    return QualitySchema.parse(this as QualityDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Quality {
    return new Quality(QualitySchema.parse(JSON.parse(json)));
  }
}
export const QualityMetadataDtoSchema = QualitySchema.omit({ benchmarks: true });
export type QualityMetadataDto = ReadonlyDto<z.infer<typeof QualityMetadataDtoSchema>>;
export const QualityShallowSchema = QualitySchema;
export type QualityShallow = ReadonlyDto<z.infer<typeof QualityShallowSchema>>;
export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true }).extend({ benchmarks: BenchmarksDiffSchema.optional() });
export type QualityDiff = ReadonlyDto<z.infer<typeof QualityDiffSchema>>;
export const QualitiesDiffSchema = z.object({ removed: z.array(QualityIdSchema).optional(), updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type QualitiesDiff = ReadonlyDto<z.infer<typeof QualitiesDiffSchema>>;
// #endregion Quality

// #region Port
export const PortSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  compatibleFamilies: z.array(FamilyIdSchema).optional(),
  mandatory: z.boolean().optional(),
  t: z.number().optional(),
  point: PointSchema.optional(),
  direction: VectorSchema.optional(),
  compatiblePorts: z.array(PortIdSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
  maxChildren: z.number().optional(),
});
export type PortDto = ReadonlyDto<z.infer<typeof PortSchema>>;
export class Port implements PortDto {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  compatibleFamilies?: FamilyIdDto[];
  mandatory?: boolean;
  t?: number;
  point?: Point;
  direction?: Vector;
  compatiblePorts?: PortIdDto[];
  qualities?: Quality[];
  attributes?: Attribute[];
  maxChildren?: number;
  constructor(dto: PortDto) {
    const p = PortSchema.parse(dto);
    Object.assign(this, p);
    this.point = p.point ? new Point(p.point) : undefined;
    this.direction = p.direction ? new Vector(p.direction) : undefined;
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: PortDto): Port {
    return new Port(dto);
  }
  static fromDto(dto: PortDto): Port {
    return new Port(dto);
  }
  static createId(id: string): PortIdDto {
    return { id };
  }
  static areSameId(a: PortIdDto, b: PortIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): PortDto {
    return PortSchema.parse(this as PortDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Port {
    return new Port(PortSchema.parse(JSON.parse(json)));
  }
}
export const PortMetadataDtoSchema = PortSchema.omit({ qualities: true, attributes: true });
export type PortMetadataDto = ReadonlyDto<z.infer<typeof PortMetadataDtoSchema>>;
export const PortShallowSchema = PortSchema;
export type PortShallow = ReadonlyDto<z.infer<typeof PortShallowSchema>>;
export const PortDiffSchema = PortSchema.partial().omit({ qualities: true, attributes: true }).extend({ qualities: QualitiesDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PortDiff = ReadonlyDto<z.infer<typeof PortDiffSchema>>;
export const PortsDiffSchema = z.object({ removed: z.array(PortIdSchema).optional(), updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PortsDiff = ReadonlyDto<z.infer<typeof PortsDiffSchema>>;
// #endregion Port

// #region Family
export const FamilySchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), icon: z.string().optional(), ports: z.array(PortSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type FamilyDto = ReadonlyDto<z.infer<typeof FamilySchema>>;
export class Family implements FamilyDto {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  ports?: Port[];
  attributes?: Attribute[];
  constructor(dto: FamilyDto) {
    const p = FamilySchema.parse(dto);
    Object.assign(this, p);
    this.ports = p.ports?.map((x) => new Port(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: FamilyDto): Family {
    return new Family(dto);
  }
  static fromDto(dto: FamilyDto): Family {
    return new Family(dto);
  }
  static createId(id: string): FamilyIdDto {
    return { id };
  }
  static areSameId(a: FamilyIdDto, b: FamilyIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): FamilyDto {
    return FamilySchema.parse(this as FamilyDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Family {
    return new Family(FamilySchema.parse(JSON.parse(json)));
  }
}
export const FamilyMetadataDtoSchema = FamilySchema.omit({ ports: true, attributes: true });
export type FamilyMetadataDto = ReadonlyDto<z.infer<typeof FamilyMetadataDtoSchema>>;
export const FamilyShallowSchema = FamilySchema;
export type FamilyShallow = ReadonlyDto<z.infer<typeof FamilyShallowSchema>>;
export const FamilyDiffSchema = FamilySchema.partial().omit({ ports: true, attributes: true }).extend({ ports: PortsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type FamilyDiff = ReadonlyDto<z.infer<typeof FamilyDiffSchema>>;
export const FamiliesDiffSchema = z.object({ removed: z.array(FamilyIdSchema).optional(), updated: z.array(z.object({ family: FamilyIdSchema, diff: FamilyDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FamiliesDiff = ReadonlyDto<z.infer<typeof FamiliesDiffSchema>>;
// #endregion Family

// #region Prop
export const PropSchema = z.object({
  id: z.coerce.string(),
  key: z.coerce.string(),
  value: z.string().optional(),
  unit: z.string().optional(),
  quality: QualityIdSchema.optional(),
});
export type PropDto = ReadonlyDto<z.infer<typeof PropSchema>>;
export class Prop implements PropDto {
  id!: string;
  key!: string;
  value?: string;
  unit?: string;
  quality?: QualityIdDto;
  constructor(dto: PropDto) {
    Object.assign(this, PropSchema.parse(dto));
  }
  static from(dto: PropDto): Prop {
    return new Prop(dto);
  }
  static fromDto(dto: PropDto): Prop {
    return new Prop(dto);
  }
  static createId(id: string): PropIdDto {
    return { id };
  }
  static areSameId(a: PropIdDto, b: PropIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): PropDto {
    return PropSchema.parse(this as PropDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Prop {
    return new Prop(PropSchema.parse(JSON.parse(json)));
  }
}
export const PropMetadataDtoSchema = PropSchema;
export type PropMetadataDto = ReadonlyDto<z.infer<typeof PropMetadataDtoSchema>>;
export const PropShallowSchema = PropSchema;
export type PropShallow = ReadonlyDto<z.infer<typeof PropShallowSchema>>;
export const PropDiffSchema = PropSchema.partial();
export type PropDiff = ReadonlyDto<z.infer<typeof PropDiffSchema>>;
export const PropsDiffSchema = z.object({ removed: z.array(PropIdSchema).optional(), updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PropsDiff = ReadonlyDto<z.infer<typeof PropsDiffSchema>>;
// #endregion Prop

// #region Tag
export const TagSchema = z.object({ id: z.string(), name: z.string(), order: z.number().optional() });
export type TagDto = ReadonlyDto<z.infer<typeof TagSchema>>;
export class Tag implements TagDto {
  id!: string;
  name!: string;
  order?: number;
  constructor(dto: TagDto) {
    Object.assign(this, TagSchema.parse(dto));
  }
  static from(dto: TagDto): Tag {
    return new Tag(dto);
  }
  static fromDto(dto: TagDto): Tag {
    return new Tag(dto);
  }
  static createId(id: string): TagIdDto {
    return { id };
  }
  static areSameId(a: TagIdDto, b: TagIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): TagDto {
    return TagSchema.parse(this as TagDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Tag {
    return new Tag(TagSchema.parse(JSON.parse(json)));
  }
}
export const TagMetadataDtoSchema = TagSchema;
export type TagMetadataDto = ReadonlyDto<z.infer<typeof TagMetadataDtoSchema>>;
export const TagShallowSchema = TagSchema;
export type TagShallow = ReadonlyDto<z.infer<typeof TagShallowSchema>>;
export const TagDiffSchema = TagSchema.partial();
export type TagDiff = ReadonlyDto<z.infer<typeof TagDiffSchema>>;
export const TagsDiffSchema = z.object({ removed: z.array(TagIdSchema).optional(), updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TagsDiff = ReadonlyDto<z.infer<typeof TagsDiffSchema>>;
// #endregion Tag

// #region Concept
export const ConceptSchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), order: z.number().optional() });
export type ConceptDto = ReadonlyDto<z.infer<typeof ConceptSchema>>;
export class Concept implements ConceptDto {
  id!: string;
  name!: string;
  description?: string;
  order?: number;
  constructor(dto: ConceptDto) {
    Object.assign(this, ConceptSchema.parse(dto));
  }
  static from(dto: ConceptDto): Concept {
    return new Concept(dto);
  }
  static fromDto(dto: ConceptDto): Concept {
    return new Concept(dto);
  }
  static createId(id: string): ConceptIdDto {
    return { id };
  }
  static areSameId(a: ConceptIdDto, b: ConceptIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): ConceptDto {
    return ConceptSchema.parse(this as ConceptDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Concept {
    return new Concept(ConceptSchema.parse(JSON.parse(json)));
  }
}
export const ConceptMetadataDtoSchema = ConceptSchema;
export type ConceptMetadataDto = ReadonlyDto<z.infer<typeof ConceptMetadataDtoSchema>>;
export const ConceptShallowSchema = ConceptSchema;
export type ConceptShallow = ReadonlyDto<z.infer<typeof ConceptShallowSchema>>;
export const ConceptDiffSchema = ConceptSchema.partial();
export type ConceptDiff = ReadonlyDto<z.infer<typeof ConceptDiffSchema>>;
export const ConceptsDiffSchema = z.object({ removed: z.array(ConceptIdSchema).optional(), updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConceptsDiff = ReadonlyDto<z.infer<typeof ConceptsDiffSchema>>;
// #endregion Concept

// #region Representation
export const RepresentationSchema = z.object({ id: z.string(), name: z.string().optional(), tags: z.array(TagIdSchema).optional(), file: FileIdSchema, description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type RepresentationDto = ReadonlyDto<z.infer<typeof RepresentationSchema>>;
export class Representation implements RepresentationDto {
  id!: string;
  name?: string;
  tags?: TagIdDto[];
  file!: FileIdDto;
  description?: string;
  attributes?: Attribute[];
  constructor(dto: RepresentationDto) {
    const p = RepresentationSchema.parse(dto);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: RepresentationDto): Representation {
    return new Representation(dto);
  }
  static fromDto(dto: RepresentationDto): Representation {
    return new Representation(dto);
  }
  static createId(id: string): RepresentationIdDto {
    return { id };
  }
  static areSameId(a: RepresentationIdDto, b: RepresentationIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): RepresentationDto {
    return RepresentationSchema.parse(this as RepresentationDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Representation {
    return new Representation(RepresentationSchema.parse(JSON.parse(json)));
  }
}
export const RepresentationMetadataDtoSchema = RepresentationSchema.omit({ tags: true, attributes: true });
export type RepresentationMetadataDto = ReadonlyDto<z.infer<typeof RepresentationMetadataDtoSchema>>;
export const RepresentationShallowSchema = RepresentationSchema;
export type RepresentationShallow = ReadonlyDto<z.infer<typeof RepresentationShallowSchema>>;
export const RepresentationDiffSchema = RepresentationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type RepresentationDiff = ReadonlyDto<z.infer<typeof RepresentationDiffSchema>>;
export const RepresentationsDiffSchema = z.object({
  removed: z.array(RepresentationIdSchema).optional(),
  updated: z.array(z.object({ representation: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type RepresentationsDiff = ReadonlyDto<z.infer<typeof RepresentationsDiffSchema>>;
// Removed: selectBestRepresentation, filterRepresentationsByTagIds, getAvailableTagIdsForRepresentations, getAllTagIdsFromRepresentations, findRepresentation, areSameRepresentation, SUPPORTED_3D_EXTENSIONS, isSupportedRepresentationExtension, validateRepresentationFile, RepresentationFileValidation — representation selection logic moved to semio/rs (Requirement 1.3)
// #endregion Representation

// #region Connector
export const ConnectorSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  t: z.number(),
  point: PointSchema,
  direction: VectorSchema,
  description: z.string().optional(),
  port: PortIdSchema.optional(),
  mandatory: z.boolean().optional(),
  maxChildren: z.number().int().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type ConnectorDto = ReadonlyDto<z.infer<typeof ConnectorSchema>>;
export class Connector implements ConnectorDto {
  id!: string;
  name?: string;
  t!: number;
  point!: Point;
  direction!: Vector;
  description?: string;
  port?: PortIdDto;
  mandatory?: boolean;
  maxChildren?: number;
  props?: Prop[];
  attributes?: Attribute[];
  constructor(dto: ConnectorDto) {
    const p = ConnectorSchema.parse(dto);
    Object.assign(this, p);
    this.point = new Point(p.point);
    this.direction = new Vector(p.direction);
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: ConnectorDto): Connector {
    return new Connector(dto);
  }
  static fromDto(dto: ConnectorDto): Connector {
    return new Connector(dto);
  }
  static createId(id: string): ConnectorIdDto {
    return { id };
  }
  static areSameId(a: ConnectorIdDto, b: ConnectorIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): ConnectorDto {
    return ConnectorSchema.parse(this as ConnectorDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Connector {
    return new Connector(ConnectorSchema.parse(JSON.parse(json)));
  }
}
export const ConnectorMetadataDtoSchema = ConnectorSchema.omit({ props: true, attributes: true });
export type ConnectorMetadataDto = ReadonlyDto<z.infer<typeof ConnectorMetadataDtoSchema>>;
export const ConnectorShallowSchema = ConnectorSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type ConnectorShallow = ReadonlyDto<z.infer<typeof ConnectorShallowSchema>>;
export const ConnectorDiffSchema = ConnectorSchema.partial()
  .omit({ point: true, direction: true, props: true, attributes: true })
  .extend({ point: PointDiffSchema.optional(), direction: VectorDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), maxChildren: z.number().int().nullable().optional() });
export type ConnectorDiff = ReadonlyDto<z.infer<typeof ConnectorDiffSchema>>;
export const ConnectorsDiffSchema = z.object({ removed: z.array(ConnectorIdSchema).optional(), updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectorsDiff = ReadonlyDto<z.infer<typeof ConnectorsDiffSchema>>;
// Removed: areConnectorsCompatible, unifyConnectorPortsAndCompatiblePortsForTypes, findConnector, findConnectorInType — connector compatibility moved to semio/rs (Requirement 1.5)
// #endregion Connector

// #region Type
export type EntityLifecycle = "active" | "deleted";
export const TypeSchema = z.object({
  id: z.string(),
  name: z.string(),
  parent: z.object({ id: z.string() }).optional(),
  families: z.array(FamilyIdSchema).optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  representations: z.array(RepresentationSchema).optional(),
  connectors: z.array(ConnectorSchema).optional(),
  props: z.array(PropSchema).optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  lifecycle: z.enum(["active", "deleted"]).optional(),
  deletedByUserId: z.string().optional(),
  deletedByDisplayName: z.string().optional(),
  deletedAt: z.string().optional(),
  deletedInChangeId: z.string().optional(),
});
export type TypeDto = ReadonlyDto<z.infer<typeof TypeSchema>>;
export class Type {
  id!: string;
  name!: string;
  parent?: { id: string };
  families?: FamilyIdDto[];
  isAbstract?: boolean;
  folder?: string;
  representations?: Representation[];
  connectors?: Connector[];
  props?: Prop[];
  stock?: number;
  virtual?: boolean;
  unit?: string;
  createdAt?: string;
  updatedAt?: string;
  location?: LocationIdDto;
  authors?: AuthorIdDto[];
  concepts?: ConceptIdDto[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  lifecycle?: EntityLifecycle;
  deletedByUserId?: string;
  deletedByDisplayName?: string;
  deletedAt?: string;
  deletedInChangeId?: string;
  constructor(dto: TypeDto) {
    const p = TypeSchema.parse(dto);
    Object.assign(this, p);
    this.representations = p.representations?.map((m) => new Representation(m));
    this.connectors = p.connectors?.map((c) => new Connector(c));
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromDto(dto: TypeDto): Type {
    return new Type(dto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Type {
    return Type.fromDto(TypeSchema.parse(JSON.parse(json)));
  }
  toDto(): TypeDto {
    return TypeSchema.parse({ ...(this as TypeDto) });
  }
  static createId(id: string): TypeIdDto {
    return { id };
  }
  static areSameId(a: TypeIdDto, b: TypeIdDto): boolean {
    return a.id === b.id;
  }
  /** @emoji 🖼️ Picks a representation for scene rendering (`@semio/ui`); first match until WASM metadata is dtod. */
  static pickBestRepresentation(representations: readonly Representation[], _tagIds: readonly string[]): Representation | undefined {
    void _tagIds;
    return representations[0];
  }
}
export const TypeMetadataDtoSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
export type TypeMetadataDto = ReadonlyDto<z.infer<typeof TypeMetadataDtoSchema>>;
export const TypeShallowSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true }).extend({
  representations: z.array(RepresentationMetadataDtoSchema).optional(),
  connectors: z.array(ConnectorMetadataDtoSchema).optional(),
  props: z.array(PropMetadataDtoSchema).optional(),
  attributes: z.array(AttributeMetadataDtoSchema).optional(),
});
export type TypeShallow = ReadonlyDto<z.infer<typeof TypeShallowSchema>>;
export const TypeDiffSchema = TypeSchema.partial()
  .omit({ representations: true, connectors: true, props: true, attributes: true })
  .extend({
    representations: RepresentationsDiffSchema.optional(),
    connectors: ConnectorsDiffSchema.optional(),
    props: PropsDiffSchema.optional(),
    attributes: AttributesDiffSchema.optional(),
    description: z.string().nullable().optional(),
    icon: z.string().nullable().optional(),
    image: z.string().nullable().optional(),
    location: LocationIdSchema.nullable().optional(),
    folder: z.string().nullable().optional(),
    concepts: z.array(ConceptIdSchema).nullable().optional(),
    authors: z.array(AuthorIdSchema).nullable().optional(),
    families: z.array(FamilyIdSchema).nullable().optional(),
    lifecycle: z.enum(["active", "deleted"]).optional(),
    deletedByUserId: z.string().nullable().optional(),
    deletedByDisplayName: z.string().nullable().optional(),
    deletedAt: z.string().nullable().optional(),
    deletedInChangeId: z.string().nullable().optional(),
  });
export type TypeDiff = ReadonlyDto<z.infer<typeof TypeDiffSchema>>;
export const TypesDiffSchema = z.object({ removed: z.array(TypeIdSchema).optional(), updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TypesDiff = ReadonlyDto<z.infer<typeof TypesDiffSchema>>;
// #endregion Type

// #region Layer
export const LayerSchema = z.object({
  id: z.string(),
  path: z.string(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type LayerDto = ReadonlyDto<z.infer<typeof LayerSchema>>;
export class Layer implements LayerDto {
  id!: string;
  path!: string;
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
  description?: string;
  attributes?: Attribute[];
  constructor(dto: LayerDto) {
    const p = LayerSchema.parse(dto);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: LayerDto): Layer {
    return new Layer(dto);
  }
  static fromDto(dto: LayerDto): Layer {
    return new Layer(dto);
  }
  static createId(id: string): LayerIdDto {
    return { id };
  }
  static areSameId(a: LayerIdDto, b: LayerIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): LayerDto {
    return LayerSchema.parse(this as LayerDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Layer {
    return new Layer(LayerSchema.parse(JSON.parse(json)));
  }
}
export const LayerMetadataDtoSchema = LayerSchema.omit({ attributes: true });
export type LayerMetadataDto = ReadonlyDto<z.infer<typeof LayerMetadataDtoSchema>>;
export const LayerShallowSchema = LayerSchema;
export type LayerShallow = ReadonlyDto<z.infer<typeof LayerShallowSchema>>;
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LayerDiff = ReadonlyDto<z.infer<typeof LayerDiffSchema>>;
export const LayersDiffSchema = z.object({ removed: z.array(LayerIdSchema).optional(), updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type LayersDiff = ReadonlyDto<z.infer<typeof LayersDiffSchema>>;
// #endregion Layer

// #region Piece
export const PieceSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  type: TypeIdSchema.optional(),
  design: DesignIdSchema.optional(),
  plane: PlaneSchema.optional(),
  center: CoordinateSchema.optional(),
  scale: z.number().optional(),
  mirrorPlane: PlaneSchema.optional(),
  isHidden: z.boolean().optional(),
  isLocked: z.boolean().optional(),
  color: z.string().optional(),
  description: z.string().optional(),
  props: z.array(PropSchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type PieceDto = ReadonlyDto<z.infer<typeof PieceSchema>>;
export class Piece {
  id!: string;
  name?: string;
  type?: TypeIdDto;
  design?: DesignIdDto;
  plane?: Plane;
  center?: Coordinate;
  scale?: number;
  mirrorPlane?: Plane;
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
  description?: string;
  props?: Prop[];
  attributes?: Attribute[];
  constructor(dto: PieceDto) {
    const p = PieceSchema.parse(dto);
    Object.assign(this, p);
    this.plane = p.plane ? new Plane(p.plane) : undefined;
    this.center = p.center ? new Coordinate(p.center) : undefined;
    this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined;
    this.props = p.props?.map((x) => new Prop(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromDto(dto: PieceDto): Piece {
    return new Piece(dto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Piece {
    return new Piece(PieceSchema.parse(JSON.parse(json)));
  }
  toDto(): PieceDto {
    return PieceSchema.parse(this as PieceDto);
  }
  static createId(id: string): PieceIdDto {
    return { id };
  }
  static areSameId(a: PieceIdDto, b: PieceIdDto): boolean {
    return a.id === b.id;
  }

  /** @emoji 🧭 Whether this piece dtos a nested design id (schema hooks). */
  dtoDesignAsPieceId(): boolean {
    return Boolean(this.design?.id);
  }

  /** @emoji 🧭 Wired type id for schema hooks. */
  dtoTypeId(): { id: string } | undefined {
    return this.type ? { id: this.type.id } : undefined;
  }

  /** @emoji 🧭 Flat plane DTO for UI (structural truth in `semio/rs` reads). */
  flatPlane(): unknown {
    return this.plane ? this.plane.toDto() : undefined;
  }

  /** @emoji 🧭 Flat center UV for UI. */
  flatCenter(): unknown {
    return this.center ? this.center.toDto() : undefined;
  }

  /** @emoji 🧭 Alternative types for replaceable UI (populated from reads in full hosts). */
  alternativeTypes(): readonly Type[] {
    return [];
  }
}
export const PieceMetadataDtoSchema = PieceSchema.omit({ props: true, attributes: true });
export type PieceMetadataDto = ReadonlyDto<z.infer<typeof PieceMetadataDtoSchema>>;
export const PieceShallowSchema = PieceSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type PieceShallow = ReadonlyDto<z.infer<typeof PieceShallowSchema>>;
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({ plane: PlaneDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PieceDiff = ReadonlyDto<z.infer<typeof PieceDiffSchema>>;
export const PiecesDiffSchema = z.object({ removed: z.array(PieceIdSchema).optional(), updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PiecesDiff = ReadonlyDto<z.infer<typeof PiecesDiffSchema>>;
// Removed: isFixedPiece, findPiece, findPieceConnections, findConnectorForPieceInConnection, getPieceRepresentationFileIds, getPieceRepresentationUrls, resolvePieceTypeForFlatten — domain logic moved to semio/rs
// #endregion Piece

// #region Group
export const GroupSchema = z.object({ id: z.string(), pieces: z.array(PieceIdSchema), color: z.string().optional(), name: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type GroupDto = ReadonlyDto<z.infer<typeof GroupSchema>>;
export class Group implements GroupDto {
  id!: string;
  pieces!: PieceIdDto[];
  color?: string;
  name?: string;
  description?: string;
  attributes?: Attribute[];
  constructor(dto: GroupDto) {
    const p = GroupSchema.parse(dto);
    Object.assign(this, p);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static from(dto: GroupDto): Group {
    return new Group(dto);
  }
  static fromDto(dto: GroupDto): Group {
    return new Group(dto);
  }
  static createId(id: string): GroupIdDto {
    return { id };
  }
  static areSameId(a: GroupIdDto, b: GroupIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): GroupDto {
    return GroupSchema.parse(this as GroupDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Group {
    return new Group(GroupSchema.parse(JSON.parse(json)));
  }
}
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type GroupDiff = ReadonlyDto<z.infer<typeof GroupDiffSchema>>;
export const GroupsDiffSchema = z.object({ removed: z.array(GroupIdSchema).optional(), updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type GroupsDiff = ReadonlyDto<z.infer<typeof GroupsDiffSchema>>;
export const GroupMetadataDtoSchema = GroupSchema.omit({ pieces: true, attributes: true });
export type GroupMetadataDto = ReadonlyDto<z.infer<typeof GroupMetadataDtoSchema>>;
export const GroupShallowSchema = GroupSchema;
export type GroupShallow = ReadonlyDto<z.infer<typeof GroupShallowSchema>>;
// #endregion Group

// #region Side
export const SideSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SideDto = ReadonlyDto<z.infer<typeof SideSchema>>;
export class Side {
  #pieceId!: string;
  #designPieceId?: string;
  #connectorId?: string;
  constructor(dto: SideDto) {
    const p = SideSchema.parse(dto);
    this.#pieceId = p.piece.id;
    this.#designPieceId = p.designPiece?.id;
    this.#connectorId = p.connector?.id;
  }
  get piece(): PieceIdDto {
    return { id: this.#pieceId };
  }
  get designPiece(): PieceIdDto | undefined {
    if (!this.#designPieceId) return undefined;
    return { id: this.#designPieceId };
  }
  get connector(): ConnectorIdDto | undefined {
    return this.#connectorId !== undefined ? { id: this.#connectorId } : undefined;
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Side {
    return new Side(SideSchema.parse(JSON.parse(json)));
  }
  static from(dto: SideDto): Side {
    return new Side(dto);
  }
  static fromDto(dto: SideDto): Side {
    return new Side(dto);
  }
  toDto(): SideDto {
    return SideSchema.parse({ piece: { id: this.#pieceId }, designPiece: this.#designPieceId ? { id: this.#designPieceId } : undefined, connector: this.#connectorId ? { id: this.#connectorId } : undefined });
  }
}
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = ReadonlyDto<z.infer<typeof SideDiffSchema>>;
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SideIdDto = ReadonlyDto<z.infer<typeof SideIdSchema>>;
export class SideId implements SideIdDto {
  piece!: PieceIdDto;
  designPiece?: PieceIdDto;
  connector?: ConnectorIdDto;
  constructor(dto: SideIdDto) {
    Object.assign(this, SideIdSchema.parse(dto));
  }
  static from(dto: SideIdDto): SideId {
    return new SideId(dto);
  }
  toDto(): SideIdDto {
    return SideIdSchema.parse(this as SideIdDto);
  }
}
export const SidesDiffSchema = z.object({ removed: z.array(SideIdSchema).optional(), updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type SidesDiff = ReadonlyDto<z.infer<typeof SidesDiffSchema>>;
// #endregion Side

// #region Connection
export const ConnectionSchema = z.object({
  id: z.string(),
  connected: SideSchema,
  connecting: SideSchema,
  gap: z.number().optional(),
  shift: z.number().optional(),
  rise: z.number().optional(),
  rotation: z.number().optional(),
  turn: z.number().optional(),
  tilt: z.number().optional(),
  u: z.number().optional(),
  v: z.number().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
export type ConnectionDto = ReadonlyDto<z.infer<typeof ConnectionSchema>>;
export class Connection implements ConnectionDto {
  id!: string;
  connected!: Side;
  connecting!: Side;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  u?: number;
  v?: number;
  description?: string;
  attributes?: Attribute[];
  constructor(dto: ConnectionDto) {
    const p = ConnectionSchema.parse(dto);
    Object.assign(this, p);
    this.connected = new Side(p.connected);
    this.connecting = new Side(p.connecting);
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Connection {
    return new Connection(ConnectionSchema.parse(JSON.parse(json)));
  }
  static from(dto: ConnectionDto): Connection {
    return new Connection(dto);
  }
  static fromDto(dto: ConnectionDto): Connection {
    return new Connection(dto);
  }
  static createId(id: string): ConnectionIdDto {
    return { id };
  }
  static areSameId(a: ConnectionIdDto, b: ConnectionIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): ConnectionDto {
    return ConnectionSchema.parse({
      id: this.id,
      connected: this.connected.toDto(),
      connecting: this.connecting.toDto(),
      gap: this.gap,
      shift: this.shift,
      rise: this.rise,
      rotation: this.rotation,
      turn: this.turn,
      tilt: this.tilt,
      u: this.u,
      v: this.v,
      description: this.description,
      attributes: this.attributes?.map((a) => a.toDto()),
    } as ConnectionDto);
  }
}
export const ConnectionDiffSchema = ConnectionSchema.partial()
  .omit({ id: true, connected: true, connecting: true, attributes: true })
  .extend({ connected: SideDiffSchema.optional(), connecting: SideDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type ConnectionDiff = ReadonlyDto<z.infer<typeof ConnectionDiffSchema>>;
export const ConnectionsDiffSchema = z.object({ removed: z.array(ConnectionIdSchema).optional(), updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectionsDiff = ReadonlyDto<z.infer<typeof ConnectionsDiffSchema>>;
export const ConnectionMetadataDtoSchema = ConnectionSchema.omit({ attributes: true });
export type ConnectionMetadataDto = ReadonlyDto<z.infer<typeof ConnectionMetadataDtoSchema>>;
export const ConnectionShallowSchema = ConnectionSchema;
export type ConnectionShallow = ReadonlyDto<z.infer<typeof ConnectionShallowSchema>>;
// #endregion Connection

// #region Stat
export const StatSchema = z.object({ id: z.string(), quality: QualityIdSchema, unit: z.string().optional(), min: z.number().optional(), minExcluded: z.boolean().optional(), max: z.number().optional(), maxExcluded: z.boolean().optional() });
export type StatDto = ReadonlyDto<z.infer<typeof StatSchema>>;
export class Stat implements StatDto {
  id!: string;
  quality!: QualityIdDto;
  unit?: string;
  min?: number;
  minExcluded?: boolean;
  max?: number;
  maxExcluded?: boolean;
  constructor(dto: StatDto) {
    Object.assign(this, StatSchema.parse(dto));
  }
  static from(dto: StatDto): Stat {
    return new Stat(dto);
  }
  static fromDto(dto: StatDto): Stat {
    return new Stat(dto);
  }
  static createId(id: string): StatIdDto {
    return { id };
  }
  static areSameId(a: StatIdDto, b: StatIdDto): boolean {
    return a.id === b.id;
  }
  toDto(): StatDto {
    return StatSchema.parse(this as StatDto);
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Stat {
    return new Stat(StatSchema.parse(JSON.parse(json)));
  }
}
export const StatDiffSchema = StatSchema.partial();
export type StatDiff = ReadonlyDto<z.infer<typeof StatDiffSchema>>;
export const StatsDiffSchema = z.object({ removed: z.array(StatIdSchema).optional(), updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type StatsDiff = ReadonlyDto<z.infer<typeof StatsDiffSchema>>;
export const StatMetadataDtoSchema = StatSchema;
export type StatMetadataDto = ReadonlyDto<z.infer<typeof StatMetadataDtoSchema>>;
export const StatShallowSchema = StatSchema;
export type StatShallow = ReadonlyDto<z.infer<typeof StatShallowSchema>>;
// #endregion Stat

// #region Design
export const DesignSchema = z.object({
  id: z.string(),
  name: z.string(),
  parent: z.object({ id: z.string() }).optional(),
  families: z.array(FamilyIdSchema).optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  pieces: z.array(PieceSchema).optional(),
  connections: z.array(ConnectionSchema).optional(),
  stats: z.array(StatSchema).optional(),
  props: z.array(PropSchema).optional(),
  layers: z.array(LayerSchema).optional(),
  activeLayer: LayerIdSchema.optional(),
  groups: z.array(GroupSchema).optional(),
  canScale: z.boolean().optional(),
  canMirror: z.boolean().optional(),
  unit: z.string().optional(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type DesignDto = ReadonlyDto<z.infer<typeof DesignSchema>>;

export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true })
  .partial()
  .extend({
    pieces: PiecesDiffSchema.optional(),
    connections: ConnectionsDiffSchema.optional(),
    stats: StatsDiffSchema.optional(),
    props: PropsDiffSchema.optional(),
    layers: LayersDiffSchema.optional(),
    groups: GroupsDiffSchema.optional(),
    authors: AuthorsDiffSchema.optional(),
    attributes: AttributesDiffSchema.optional(),
  });
export type DesignDiff = ReadonlyDto<z.infer<typeof DesignDiffSchema>>;
export const DesignsDiffSchema = z.object({ removed: z.array(DesignIdSchema).optional(), updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type DesignsDiff = ReadonlyDto<z.infer<typeof DesignsDiffSchema>>;

/** @emoji ⚠️ Algorithm adapter / native REST error row. */
export type AlgorithmError = { readonly code: string; readonly message: string };
export type DesignDiffOperationResult = { readonly ok: true; readonly diff: DesignDiff } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };
export type OperationResult<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };

/** @emoji 🔧 Gap/shift/rise knobs for structural move previews (algorithms UI). */
export type MoveVector = { readonly gap: number; readonly shift: number; readonly rise: number };

/** @emoji 📌 Paste anchoring modes for copy/paste algorithm stories. */
export type PasteDesignAnchoringKind = "original" | "middle" | "centroid" | "bottomLeft" | "bottomRight" | "topLeft" | "topRight";

/** @emoji 🧠 Optional per-piece flatten cache row (TS algorithm path; opaque to callers). */
export type FlatMerkleCacheEntry = Readonly<JsonObject>;

export class Design {
  id!: string;
  name!: string;
  parent?: { id: string };
  families?: FamilyIdDto[];
  isAbstract?: boolean;
  folder?: string;
  pieces?: Piece[];
  _connections?: Connection[];
  stats?: Stat[];
  props?: Prop[];
  layers?: Layer[];
  activeLayer?: LayerIdDto;
  groups?: Group[];
  canScale?: boolean;
  canMirror?: boolean;
  unit?: string;
  location?: LocationIdDto;
  authors?: AuthorIdDto[];
  concepts?: ConceptIdDto[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;
  get connections(): Connection[] | undefined {
    return this._connections;
  }
  constructor(dtoIn: DesignDto | Design) {
    const dto = dtoIn instanceof Design ? dtoIn.toDto() : dtoIn;
    const p = DesignSchema.parse(dto);
    const { connections: _wcon, pieces: _wp, ...rest } = p;
    Object.assign(this, rest);
    this.pieces = p.pieces?.map((x) => new Piece(x));
    this._connections = p.connections?.map((x) => new Connection(x));
    this.stats = p.stats?.map((x) => new Stat(x));
    this.props = p.props?.map((x) => new Prop(x));
    this.layers = p.layers?.map((x) => new Layer(x));
    this.groups = p.groups?.map((x) => new Group(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromDto(dto: DesignDto): Design {
    return new Design(dto);
  }
  toDto(): DesignDto {
    return DesignSchema.parse({
      ...(this as DesignDto),
      pieces: this.pieces?.map((x) => x.toDto()),
      connections: this._connections?.map((x) => x.toDto()),
      stats: this.stats?.map((x) => x.toDto()),
      props: this.props?.map((x) => x.toDto()),
      layers: this.layers?.map((x) => x.toDto()),
      groups: this.groups?.map((x) => x.toDto()),
      attributes: this.attributes?.map((x) => x.toDto()),
    });
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Design {
    return new Design(DesignSchema.parse(JSON.parse(json)));
  }
  static createId(id: string): DesignIdDto {
    return { id };
  }
  static areSameId(a: DesignIdDto, b: DesignIdDto): boolean {
    return a.id === b.id;
  }

  /** @emoji 🧭 Included / sibling designs for nested-design UI (DTO navigation). */
  getDesignFamily(): Design[] {
    return [];
  }

  /** @emoji 🧾 Legacy alias for diagram consumers (`@semio/ui`). */
  getConnections(): Connection[] {
    return [...(this._connections ?? [])];
  }

  /** @emoji 🧾 Non-mutating diff overlay for MCP / diagram previews. */
  static previewWithDiff(design: Design, diff: DesignDiff): Design {
    const dto = design instanceof Design ? design.toDto() : DesignSchema.parse(design as DesignDto);
    const n = new Design(dto);
    n.applyDiff(diff);
    return n;
  }

  /** @emoji 🧩 Merges a structural {@link DesignDiff} into this design (pieces + connections). */
  applyDiff(diff: DesignDiff): void {
    if (diff.pieces?.removed?.length) {
      const rm = new Set(diff.pieces.removed.map((x) => x.id));
      this.pieces = (this.pieces ?? []).filter((p) => !rm.has(p.id));
    }
    if (diff.pieces?.updated?.length) {
      for (const u of diff.pieces.updated) {
        const p = (this.pieces ?? []).find((x) => x.id === u.piece.id);
        if (!p) continue;
        const d = u.diff;
        if (d.name !== undefined) p.name = d.name;
        if (d.scale !== undefined) p.scale = d.scale;
        if (d.center) {
          const c = p.center ? p.center.toDto() : { u: 0, v: 0 };
          p.center = new Coordinate({ ...c, ...d.center });
        }
        if (d.plane && p.plane) {
          const pl = p.plane.toDto();
          const o = d.plane.origin ? { ...pl.origin, ...d.plane.origin } : pl.origin;
          const xa = d.plane.xAxis ? { ...pl.xAxis, ...d.plane.xAxis } : pl.xAxis;
          const ya = d.plane.yAxis ? { ...pl.yAxis, ...d.plane.yAxis } : pl.yAxis;
          p.plane = new Plane({ origin: o, xAxis: xa, yAxis: ya });
        }
      }
    }
    if (diff.pieces?.added?.length) {
      this.pieces = [...(this.pieces ?? []), ...diff.pieces.added.map((x) => new Piece(PieceSchema.parse(x as PieceDto)))];
    }
    if (diff.connections?.removed?.length) {
      const rm = new Set(diff.connections.removed.map((x) => x.id));
      this._connections = (this._connections ?? []).filter((c) => !rm.has(c.id));
    }
    if (diff.connections?.updated?.length) {
      for (const u of diff.connections.updated) {
        const c = (this._connections ?? []).find((x) => x.id === u.connection.id);
        if (!c) continue;
        Object.assign(c, u.diff);
      }
    }
    if (diff.connections?.added?.length) {
      this._connections = [...(this._connections ?? []), ...diff.connections.added.map((x) => new Connection(ConnectionSchema.parse(x as z.infer<typeof ConnectionSchema>)))];
    }
  }

  /** @emoji 🧾 Selection drag in flat UV space (piece centers only; algorithm preview). */
  dragBySelection(piecesDesign: Design, offset: CoordinateDto): DesignDiff {
    const du = offset.u ?? 0;
    const dv = offset.v ?? 0;
    const sel = new Set((piecesDesign.pieces ?? []).map((p) => p.id));
    const updated = (this.pieces ?? [])
      .filter((p) => sel.has(p.id))
      .map((p) => {
        const c = p.center?.toDto() ?? { u: 0, v: 0 };
        return { piece: { id: p.id }, diff: { center: { u: c.u + du, v: c.v + dv } } };
      });
    return { pieces: { updated } };
  }

  /** @emoji 🗑️ Diff removing the given pieces and connections (preview-only; kit graph unchanged). */
  deletePiecesAndConnectionsDiff(pieceIds: readonly string[], connectionIds: readonly string[]): DesignDiffOperationResult {
    return {
      ok: true,
      diff: {
        pieces: { removed: pieceIds.map((id) => ({ id })) },
        connections: { removed: connectionIds.map((id) => ({ id })) },
      },
    };
  }
}

export type DesignOperationResult = { readonly ok: true; readonly design: Design; readonly diff: { forward: DesignDiff; reverse: DesignDiff } } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };

/** @emoji 🧾 Coerces native REST flatten payloads into {@link DesignOperationResult}. */
export function normalizeDesignFlattenResult(raw: unknown): DesignOperationResult {
  return raw as DesignOperationResult;
}
/** @emoji 🧾 Coerces native REST diff payloads into {@link DesignDiffOperationResult}. */
export function normalizeDesignDiffResult(raw: unknown): DesignDiffOperationResult {
  return raw as DesignDiffOperationResult;
}
/** @emoji 🧾 Coerces native REST copy payloads into {@link OperationResult}<{@link Design}>. */
export function normalizeDesignCopyResult(raw: unknown): OperationResult<Design> {
  return raw as OperationResult<Design>;
}

export const DesignMetadataDtoSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true, authors: true, concepts: true });
export type DesignMetadataDto = ReadonlyDto<z.infer<typeof DesignMetadataDtoSchema>>;
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true }).extend({
  pieces: z.array(PieceMetadataDtoSchema).optional(),
  connections: z.array(ConnectionMetadataDtoSchema).optional(),
  stats: z.array(StatMetadataDtoSchema).optional(),
  props: z.array(PropMetadataDtoSchema).optional(),
  layers: z.array(LayerMetadataDtoSchema).optional(),
  groups: z.array(GroupMetadataDtoSchema).optional(),
  attributes: z.array(AttributeMetadataDtoSchema).optional(),
});
export type DesignShallow = ReadonlyDto<z.infer<typeof DesignShallowSchema>>;
// Removed: addPieceToDesignDiff, setPieceInDesignDiff, removePieceFromDesignDiff, addPiecesToDesignDiff, setPiecesInDesignDiff, removePiecesFromDesignDiff, addConnectionToDesignDiff, setConnectionInDesignDiff, removeConnectionFromDesignDiff, addConnectionsToDesignDiff, setConnectionsInDesignDiff, removeConnectionsFromDesignDiff, mergeDesigns, orientDesign, duplicateDesignDiffForIsolation — design-diff builder functions moved to semio/rs (Requirement 3.7)
// #endregion Design

// #region 🧾KitStoreClientChildCommands

/** @emoji 🧾 Child DTO kinds for kit-root add/remove commands (port rows use type-scoped flows, not top-level add). */
export type KitChildEntityKind = "Family" | "Author" | "Concept" | "Tag" | "Quality" | "File" | "Folder" | "Type" | "Design" | "Port";

/** @emoji 🧾 Parses `dto` with the matching zod schema and runs `add*` kit dtos. */
export async function kitStoreClientAddChildByKind(client: KitStoreClient, childKind: string, dto: unknown): Promise<SetResult> {
  let cmds: readonly ChangeKitCommand[];
  try {
    switch (childKind) {
      case "Family":
        cmds = [{ addFamily: { family: FamilySchema.parse(dto) } }];
        break;
      case "Author":
        cmds = [{ addAuthor: { author: AuthorSchema.parse(dto) } }];
        break;
      case "Concept":
        cmds = [{ addConcept: { concept: ConceptSchema.parse(dto) } }];
        break;
      case "Tag":
        cmds = [{ addTag: { tag: TagSchema.parse(dto) } }];
        break;
      case "Quality":
        cmds = [{ addQuality: { quality: QualitySchema.parse(dto) } }];
        break;
      case "File":
        cmds = [{ addFile: { file: FileSchema.parse(dto) } }];
        break;
      case "Folder":
        cmds = [{ addFolder: { folder: FolderSchema.parse(dto) } }];
        break;
      case "Type":
        cmds = [{ addType: { type: TypeSchema.parse(dto) } }];
        break;
      case "Design":
        cmds = [{ addDesign: { design: DesignSchema.parse(dto) } }];
        break;
      case "Port":
        return { ok: false, error: { kind: "NotSupported", message: "add Port: use type-scoped kit commands" } };
      default:
        return { ok: false, error: { kind: "NotSupported", message: `add to kit: ${childKind}` } };
    }
  } catch (err) {
    return { ok: false, error: { kind: "InvalidValue", message: String(err) } };
  }
  return client.submitChangeKitCommands(cmds);
}

/** @emoji 🧾 Emits matching `remove*` kit dto for a kit-root child id. */
export async function kitStoreClientRemoveChildByKind(client: KitStoreClient, childKind: string, childId: string): Promise<SetResult> {
  const idw = { id: childId };
  let cmds: readonly ChangeKitCommand[];
  switch (childKind) {
    case "Family":
      cmds = [{ removeFamily: { familyId: idw } }];
      break;
    case "Author":
      cmds = [{ removeAuthor: { authorId: idw } }];
      break;
    case "Concept":
      cmds = [{ removeConcept: { conceptId: idw } }];
      break;
    case "Tag":
      cmds = [{ removeTag: { tagId: idw } }];
      break;
    case "Quality":
      cmds = [{ removeQuality: { qualityId: idw } }];
      break;
    case "File":
      cmds = [{ removeFile: { fileId: idw } }];
      break;
    case "Folder":
      cmds = [{ removeFolder: { folderId: idw } }];
      break;
    case "Type":
      cmds = [{ removeType: { typeId: idw } }];
      break;
    case "Design":
      cmds = [{ removeDesign: { designId: idw } }];
      break;
    case "Port":
      return { ok: false, error: { kind: "NotSupported", message: "remove Port: use type-scoped kit commands" } };
    default:
      return { ok: false, error: { kind: "NotSupported", message: `remove from kit: ${childKind}` } };
  }
  return client.submitChangeKitCommands(cmds);
}

/** @emoji 🧾 Adds a piece under a design (`addPiece` dto). */
export async function kitStoreClientAddPiece(client: KitStoreClient, designId: string, piece: unknown): Promise<SetResult> {
  return client.submitChangeKitCommands([{ changeDesignCommands: { designId: { id: designId }, commands: [{ addPiece: { piece: PieceSchema.parse(piece) } }] } }]);
}

/** @emoji 🧾 Removes a piece from a design (`removePiece` dto). */
export async function kitStoreClientRemovePiece(client: KitStoreClient, designId: string, pieceId: string): Promise<SetResult> {
  return client.submitChangeKitCommands([{ changeDesignCommands: { designId: { id: designId }, commands: [{ removePiece: { pieceId: { id: pieceId } } }] } }]);
}

/** @emoji 🧾 Adds a connection under a design (`addConnection` dto). */
export async function kitStoreClientAddConnection(client: KitStoreClient, designId: string, connection: unknown): Promise<SetResult> {
  return client.submitChangeKitCommands([
    {
      changeDesignCommands: {
        designId: { id: designId },
        commands: [{ addConnection: { connection: ConnectionSchema.parse(connection) } }],
      },
    },
  ]);
}

// #endregion 🧾KitStoreClientChildCommands

// #region Kit
export const KitKindSchema = z.enum(["dev", "local", "archive", "remote", "transport"]);
export type KitKind = ReadonlyDto<z.infer<typeof KitKindSchema>>;
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;

export const KitFullDtoSchema = z.object({
  id: z.string(),
  name: z.string(),
  version: z.string().optional(),
  types: z.array(TypeSchema).optional(),
  designs: z.array(DesignSchema).optional(),
  tags: z.array(TagSchema).optional(),
  concepts: z.array(ConceptSchema).optional(),
  families: z.array(FamilySchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  files: z.array(FileSchema).optional(),
  folders: z.array(FolderSchema).optional(),
  authors: z.array(AuthorSchema).optional(),
  remote: z.string().optional(),
  homepage: z.string().optional(),
  license: z.string().optional(),
  preview: z.string().optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type KitFullDto = ReadonlyDto<z.infer<typeof KitFullDtoSchema>>;

function semioCoerceKitFullDtoFromJson(v: KitJsonTreeDto | KitFullDto): KitFullDto {
  return KitFullDtoSchema.parse(v);
}

function semioParseTypeShallowArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly TypeShallow[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) => {
    if (row == null || typeof row !== "object" || Array.isArray(row)) return row;
    const r0 = __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject)) as JsonObject;
    const r = __mutableJsonObjectCopy(r0);
    for (const k of ["connectors", "representations", "props", "attributes"] as const) {
      const inner = r[k];
      if (inner != null && typeof inner === "object") {
        r[k] = kitGraphqlJsonToReadonlyArray(inner as JsonValue) as unknown as JsonValue;
      }
    }
    return r as KitJsonTreeDto;
  });
  const r = z.array(TypeShallowSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseDesignShallowArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly DesignShallow[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) => {
    if (row == null || typeof row !== "object" || Array.isArray(row)) return row;
    const r0 = __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject)) as JsonObject;
    const r = __mutableJsonObjectCopy(r0);
    for (const k of ["pieces", "connections", "layers", "groups", "stats", "props", "attributes"] as const) {
      const inner = r[k];
      if (inner != null && typeof inner === "object") {
        r[k] = kitGraphqlJsonToReadonlyArray(inner as JsonValue) as unknown as JsonValue;
      }
    }
    return r as KitJsonTreeDto;
  });
  const r = z.array(DesignShallowSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseKitIdDtoArray(v: KitJsonTreeDto | string | undefined | null): readonly KitIdDto[] {
  const xs = kitGraphqlJsonToReadonlyArray(v);
  const out: KitIdDto[] = [];
  for (const x of xs) {
    if (x != null && typeof x === "object" && !Array.isArray(x) && "id" in x && typeof (x as { id: KitJsonTreeDto }).id === "string") out.push({ id: (x as { id: string }).id });
    else if (typeof x === "string") out.push({ id: x });
  }
  return out;
}

function semioParseTypeMetadataArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly TypeMetadataDto[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) => (row && typeof row === "object" && !Array.isArray(row) ? __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject)) : row));
  const r = z.array(TypeMetadataDtoSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseDesignMetadataArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly DesignMetadataDto[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) => (row && typeof row === "object" && !Array.isArray(row) ? __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject)) : row));
  const r = z.array(DesignMetadataDtoSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseAuthorMetadataArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly AuthorMetadataDto[] {
  const r = z.array(AuthorMetadataDtoSchema).safeParse(kitGraphqlJsonToReadonlyArray(v));
  return r.success ? r.data : [];
}

function semioParseKitMetadataJson(v: KitJsonTreeDto | undefined | null): KitMetadataDto | null {
  if (v == null || typeof v !== "object" || Array.isArray(v)) return null;
  return v as KitMetadataDto;
}

function semioParseColoredConnectorRowsJson(v: KitJsonTreeDto | readonly KitJsonTreeDto[] | undefined | null): readonly KitColoredConnectorRowDto[] {
  if (Array.isArray(v)) {
    const out: KitColoredConnectorRowDto[] = [];
    for (const row of v) {
      if (row && typeof row === "object" && !Array.isArray(row) && "color" in row) {
        const r = row as { typeId?: { id?: string }; connectorId?: { id?: string }; color?: string };
        if (typeof r.color === "string" && r.typeId && r.connectorId) {
          const tid = typeof r.typeId.id === "string" ? r.typeId.id : "";
          const cid = typeof r.connectorId.id === "string" ? r.connectorId.id : "";
          if (tid && cid) out.push({ typeId: { id: tid }, connectorId: { id: cid }, color: r.color });
        }
      }
    }
    return out;
  }
  return [];
}

function semioParsePieceDtoArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly PieceDto[] {
  const r = z.array(PieceSchema).safeParse(kitGraphqlJsonToReadonlyArray(v));
  return r.success ? r.data : [];
}

function semioParseConnectionDtoArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly ConnectionDto[] {
  const r = z.array(ConnectionSchema).safeParse(kitGraphqlJsonToReadonlyArray(v));
  return r.success ? r.data : [];
}

const includedDesignInfoJsonZod = z.object({
  id: z.string(),
  designId: z.string(),
  connectionKind: z.string(),
  center: PointSchema.nullable().optional(),
  plane: PlaneSchema.nullable().optional(),
  externalConnections: z.array(ConnectionSchema).optional(),
});

/** Zod for one `PieceStore` row returned under `designByDtoId.pieces` (hierarchy + flat pose for {@link PiecePlacementRowDto}). */
const pieceStoreHierarchyPieceGqlZod = z.object({
  id: z.string(),
  depth: z.number(),
  path: z.array(z.object({ id: z.string() })),
  parentPiece: z.object({ id: z.string() }).nullable().optional(),
  flatPlane: PlaneSchema,
  flatCenter: CoordinateSchema,
});

function semioParseDesignIncludedDesignArrayJson(v: KitJsonTreeDto | readonly KitJsonTreeDto[] | undefined | null): readonly IncludedDesignInfoDto[] {
  const r = z.array(includedDesignInfoJsonZod).safeParse(Array.isArray(v) ? v : kitGraphqlJsonToReadonlyArray(v));
  return r.success ? (r.data as readonly IncludedDesignInfoDto[]) : [];
}

function semioParsePiecePlacementMapJson(pieces: readonly unknown[] | undefined | null): ReadonlyMap<string, PiecePlacementRowDto> {
  const m = new Map<string, PiecePlacementRowDto>();
  if (!Array.isArray(pieces)) return m;
  for (const r of pieces) {
    const parsed = pieceStoreHierarchyPieceGqlZod.safeParse(r);
    if (!parsed.success) continue;
    const row = parsed.data;
    const pathIds = row.path.map((p) => p.id);
    const fixedPieceId = pathIds[0] ?? row.id;
    m.set(row.id, {
      pieceId: row.id,
      plane: row.flatPlane,
      center: row.flatCenter,
      fixedPieceId,
      parentPieceId: row.parentPiece?.id ?? null,
      depth: row.depth,
      path: pathIds,
    });
  }
  return m;
}

function semioParsePlaneNullableJson(v: KitJsonTreeDto | undefined | null): PlaneDto | null {
  const p = PlaneSchema.safeParse(v);
  return p.success ? p.data : null;
}

function semioParseCoordinateNullableJson(v: KitJsonTreeDto | undefined | null): CoordinateDto | null {
  const p = CoordinateSchema.safeParse(v);
  return p.success ? p.data : null;
}

function semioParseConnectionNullableJson(v: KitJsonTreeDto | undefined | null): ConnectionDto | null {
  const p = ConnectionSchema.safeParse(v);
  return p.success ? p.data : null;
}

function semioParseRepresentationNullableJson(v: KitJsonTreeDto | undefined | null): RepresentationDto | null {
  const p = RepresentationSchema.safeParse(v);
  return p.success ? p.data : null;
}

/** @emoji 🧾 Fills missing `folders[].path` from legacy `name` + `parent` before {@link FolderSchema} parse. */
export function normalizeKitFullDtoFolderPaths(dto: KitFullDto): KitFullDto {
  const foldersUnknown = (dto as { folders?: unknown }).folders;
  if (!Array.isArray(foldersUnknown) || foldersUnknown.length === 0) return dto;
  const list = foldersUnknown as Array<JsonObject>;
  const byId = new Map<string, JsonObject>();
  for (const row of list) {
    if (row && typeof row.id === "string") byId.set(row.id, row);
  }
  const resolvePath = (f: JsonObject, visiting: Set<string>): string => {
    const fid = typeof f.id === "string" ? f.id : "";
    const existing = f.path;
    if (typeof existing === "string" && existing.length > 0) return existing;
    if (fid && visiting.has(fid)) return String(f.name ?? fid);
    if (fid) visiting.add(fid);
    const seg = String((f.name as string | undefined) ?? (fid || "folder"));
    const parent = f.parent as { id?: string } | undefined;
    const pid = parent?.id != null ? String(parent.id) : "";
    if (pid && byId.has(pid)) {
      const base = resolvePath(byId.get(pid)!, visiting);
      if (fid) visiting.delete(fid);
      return base ? `${base}/${seg}` : seg;
    }
    if (fid) visiting.delete(fid);
    return seg;
  };
  const nextFolders = list.map((row) => ({ ...row, path: resolvePath(row, new Set()) }));
  return { ...(dto as object), folders: nextFolders } as unknown as KitFullDto;
}

export class Kit {
  /** @emoji 📌 Anchoring kinds exposed to copy/paste algorithm UI. */
  static readonly pasteDesignAnchoringKinds: readonly PasteDesignAnchoringKind[] = ["original", "middle", "centroid", "bottomLeft", "bottomRight", "topLeft", "topRight"];

  /** @emoji 🧭 Normalizes dto/DTO kit records to a {@link Kit} entity (replaces legacy `Kit.ensure`). */
  static ensure(kit: Kit | KitFullDto): Kit {
    return kit instanceof Kit ? kit : Kit.fromDto(kit as KitFullDto);
  }

  /** @emoji 📋 Copy selection (TS path stub — use REST language or extend with KitStore batch). */
  copyDesignOp(_design: Design, _pieceIds: readonly string[], _connectionIds: readonly string[]): OperationResult<Design> {
    void _design;
    void _pieceIds;
    void _connectionIds;
    return { ok: false, errors: [{ code: "native.copy.ts", message: "nativeCopyDesign(ts): not dtod to WASM batch yet; switch language or implement batch copy." }] };
  }

  /** @emoji 📋 Paste selection (TS path stub). */
  pasteDesignOp(_source: Design, _target: Design, _anchoring: string, _coordinate: CoordinateDto | undefined): DesignDiff {
    void _source;
    void _target;
    void _anchoring;
    void _coordinate;
    return {};
  }

  id!: string;
  name!: string;
  version?: string;
  types?: Type[];
  designs?: Design[];
  tags?: Tag[];
  concepts?: Concept[];
  families?: Family[];
  qualities?: Quality[];
  files?: File[];
  folders?: Folder[];
  authors?: Author[];
  remote?: string;
  homepage?: string;
  license?: string;
  preview?: string;
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;
  constructor(data: KitFullDto) {
    const p = KitFullDtoSchema.parse(normalizeKitFullDtoFolderPaths(data));
    Object.assign(this, p);
    this.types = p.types?.map((t) => new Type(t));
    this.designs = p.designs?.map((d) => new Design(d));
    this.tags = p.tags?.map((t) => new Tag(t));
    this.concepts = p.concepts?.map((c) => new Concept(c));
    this.families = p.families?.map((f) => new Family(f));
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.files = p.files?.map((f) => new File(f));
    this.folders = p.folders?.map((f) => new Folder(f));
    this.authors = p.authors?.map((a) => new Author(a));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromDto(data: KitFullDto): Kit {
    return new Kit(data);
  }
  toDto(): KitFullDto {
    return KitFullDtoSchema.parse({
      ...(this as KitFullDto),
      types: this.types?.map((t) => t.toDto()),
      designs: this.designs?.map((d) => d.toDto()),
      tags: this.tags?.map((t) => t.toDto()),
      concepts: this.concepts?.map((c) => c.toDto()),
      families: this.families?.map((f) => f.toDto()),
      qualities: this.qualities?.map((q) => q.toDto()),
      files: this.files?.map((f) => f.toDto()),
      folders: this.folders?.map((f) => f.toDto()),
      authors: this.authors?.map((a) => a.toDto()),
      attributes: this.attributes?.map((a) => a.toDto()),
    });
  }
  serialize(): string {
    return JSON.stringify(this.toDto());
  }
  static deserialize(json: string): Kit {
    return Kit.fromDto(KitFullDtoSchema.parse(JSON.parse(json)));
  }
  toJSON(): KitFullDto {
    return this.toDto();
  }
  static createId(id: string): KitIdDto {
    return { id };
  }
  static areSameId(a: KitIdDto, b: KitIdDto): boolean {
    return a.id === b.id;
  }

  /** @emoji 🧭 Resolve a design by id (DTO graph navigation for React schema hooks). */
  findDesign(id: string): Design | undefined {
    return this.designs?.find((d) => d.id === id);
  }

  /** @emoji 🧭 Resolve a type by id. */
  findType(id: string): Type | undefined {
    return this.types?.find((t) => t.id === id);
  }

  /** @emoji 🧭 Flatten / parent metadata map (DTO host; WASM bridge may supply richer maps). */
  piecesMetadataFor(_designId: string): { ok: true; diff: Map<string, { parentPieceId?: string }> } | { ok: false; diff?: undefined } {
    void _designId;
    return { ok: true, diff: new Map() };
  }

  /** @emoji 🧭 Parent piece for `pieceId` via connection graph (connecting → connected). */
  findParentPieceInDesign(designId: string, pieceId: string): Piece | undefined {
    const d = this.findDesign(designId);
    if (!d?._connections || !d.pieces) return undefined;
    for (const c of d._connections) {
      const connectingId = c.connecting?.piece?.id;
      if (connectingId !== pieceId) continue;
      const parentId = c.connected?.piece?.id;
      if (!parentId) return undefined;
      return d.pieces.find((p) => p.id === parentId);
    }
    return undefined;
  }

  /** @emoji 🧭 Parent connection whose connecting side matches `pieceId`. */
  findParentConnectionForPieceInDesign(designId: string, pieceId: string): Connection | undefined {
    const d = this.findDesign(designId);
    if (!d?._connections) return undefined;
    for (const c of d._connections) {
      if (c.connecting?.piece?.id === pieceId) return c;
    }
    return undefined;
  }

  /** @emoji 🧭 Child pieces: connections where connected side is `parentPieceId` and connecting side is another piece. */
  findChildrenPiecesInDesign(designId: string, parentPieceId: string): Piece[] {
    const d = this.findDesign(designId);
    if (!d?._connections || !d.pieces) return [];
    const out: Piece[] = [];
    for (const c of d._connections) {
      if (c.connected?.piece?.id !== parentPieceId) continue;
      const childId = c.connecting?.piece?.id;
      if (!childId) continue;
      const p = d.pieces.find((x) => x.id === childId);
      if (p) out.push(p);
    }
    return out;
  }

  /**
   * @emoji 🧭 Sync flatten preview for MCP / `@semio/ui` (identity plane fallback until async WASM is threaded here).
   */
  flattenDesignCachedOp(designId: string, _prev?: { [pieceId: string]: FlatMerkleCacheEntry }): { result: DesignOperationResult; cache: { [pieceId: string]: FlatMerkleCacheEntry } } {
    void _prev;
    const design = this.designs?.find((d) => d.id === designId);
    if (!design) {
      return {
        result: {
          ok: false,
          errors: [{ code: "mcp-flatten.design-not-found", message: `design ${designId} missing on kit` }],
        },
        cache: {},
      };
    }
    const defaultPlane = { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
    const conns = design.connections ?? [];
    const forward: DesignDiff = {
      pieces: {
        updated: (design.pieces ?? []).map((p) => ({
          piece: { id: p.id },
          diff: {
            plane: (p.plane?.toDto() as unknown) ?? defaultPlane,
            center: p.center?.toDto() ?? { u: 0, v: 0 },
          },
        })),
      },
      connections: conns.length ? { removed: conns.map((c) => ({ id: c.id })) } : undefined,
    };
    return { result: { ok: true, design, diff: { forward, reverse: {} } }, cache: {} };
  }
}
export type KitLike = Kit | KitFullDto;

// #region KitHostStores
/** @emoji 🧭 Client-side v7/UUID id for empty kit records when not using WASM. */
export function id(): string {
  if (typeof globalThis !== "undefined" && globalThis.crypto && typeof (globalThis.crypto as Crypto).randomUUID === "function") return (globalThis.crypto as Crypto).randomUUID()!;
  return `k-${Date.now()}-${((Math.random() * 0x1_0000_0000) | 0).toString(16)}`;
}

/** @emoji 🧭 DTO/entity to `Kit` (react / kit registry). */
export function asKitInstance(input: KitLike): Kit {
  return input instanceof Kit ? input : Kit.fromDto(input as KitFullDto);
}

/**
 * @emoji 🧾 Pulls the authoritative DTO from `kitClient` into a host {@link KitHostStore} (no React; call after GQL events).
 */
/** @emoji 🧾 Minimal bridge surface used when applying WASM snapshots onto a host store.
 *  When the host store is bundle-persisting (`JsonFileKitStore` / `FolderKitStore`) we also drive the
 *  metabolism-shaped JSON file end-to-end: on first call we hydrate rs from the file's bytes (if any) and
 *  bootstrap the seed checkpoint + unsaved change; after every change we ask rs to serialize the bundle and atomically
 *  write it back through the host adapter. The file therefore always mirrors `wip.initialKit` + version changes / edits
 *  state and looks like `semio/assets/semio/metabolism.new.kit.semio.json`. */
const KIT_BUNDLE_BOOTSTRAPPED = new WeakSet<KitHostStore>();
export async function applyKitClientSnapshotToLocalStore(kitClient: KitStoreClient, store: KitHostStore): Promise<void> {
  const ks = kitStoreFromKitStoreClient(kitClient);
  // 🌱 First-time wiring for bundle-persisting hosts: hydrate rs from the file (if any), then ensure rs has a seed checkpoint + default unsaved change.
  if (isKitBundlePersistingStore(store) && !KIT_BUNDLE_BOOTSTRAPPED.has(store)) {
    KIT_BUNDLE_BOOTSTRAPPED.add(store);
    if (ks) {
      const initial = store.initialBundleJson;
      if (initial.trim() !== "") {
        try {
          await ks.hydrateKitStoreBundleJson(initial);
        } catch {
          /* keep going — even if hydration fails we still bootstrap defaults so the file becomes a well-formed bundle */
        }
      }
      try {
        await ks.kitStoreInitializeDefaults();
      } catch {
        /* ignore */
      }
    }
  }

  try {
    const incoming = await kitClient.fetchFullKit();
    const curJson = store.getSnapshot().kit.toJSON();
    const changed = JSON.stringify(incoming) !== JSON.stringify(curJson);
    if (changed) store.replace(asKitInstance(incoming));
    if (isKitBundlePersistingStore(store) && ks) {
      const json = await ks.serializeKitStoreBundleJson();
      if (json.trim() !== "") {
        await store.persistBundle(json);
      }
    }
  } catch {
    /* ignore */
  }
}

/** @emoji 🧭 Local/sync facet on every kit store snapshot (WASM or file-backed; hooks read `sync.readonly` etc). */
export type KitSyncSnapshot = { status: string; dirty: boolean; readonly: boolean; lastSyncedAt: string | null; error: unknown | null };
export const DEFAULT_KIT_SYNC: Readonly<KitSyncSnapshot> = Object.freeze({ status: "idle", dirty: false, readonly: false, lastSyncedAt: null, error: null });
export type KitStoreSnapshot = { kit: Kit; sync: KitSyncSnapshot };
export type KitHostStore = { getSnapshot(): KitStoreSnapshot; subscribe(onChange: () => void): () => void; replace(kit: Kit): void };
/** @emoji 🧾 Alias for hosts that still import `KitHostStoreSnapshot` from `@semio/js`. */
export type KitHostStoreSnapshot = KitStoreSnapshot;
/** @emoji 🧾 Plain DTO aliases for React/schema bridges (same as `*Dto` types). */
export type DesignPlain = DesignDto;
export type TypePlain = TypeDto;
export type PiecePlain = PieceDto;
export type ConnectionPlain = ConnectionDto;
export type PlanePlain = PlaneDto;
export type CoordinatePlain = CoordinateDto;

export class InMemoryKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal Used by `inferPersistenceFromInit` in @semio/react. */
  readonly name = "InMemoryKitStore";
  constructor(seed: KitLike) {
    this._kit = seed instanceof Kit ? seed : Kit.fromDto(seed as KitFullDto);
  }
  getSnapshot(): KitStoreSnapshot {
    return { kit: this._kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) {
    this.listeners.add(onChange);
    return () => {
      this.listeners.delete(onChange);
    };
  }
  replace(kit: Kit) {
    this._kit = kit;
    for (const l of this.listeners) {
      try {
        l();
      } catch {
        /* ignore */
      }
    }
  }
}

// NOTE 🚧 The on-disk kit-store bundle format (see `semio/assets/semio/metabolism.new.kit.semio.json`)
// is owned exclusively by `semio/rs`. JS host stores MUST NOT parse, validate, generate, or
// reshape that file format. The JS file/folder host stores below are transient bridges that
// will be replaced once Rust drives the dev-json backbone end-to-end through a host adapter.
// Until then they keep a flat in-memory DTO snapshot for React without speaking the bundle.

export type KitJsonFileAdapter = { read: () => Promise<string>; write: (json: string) => Promise<void> };
/** @emoji 🧾 Folder persistence adapter (Electron passes two path segments for `createDirectory`). */
export type KitFolderAdapter = {
  readKit: () => Promise<Uint8Array | undefined>;
  writeKit: (bytes: Uint8Array) => void | Promise<void>;
  readFile: (path: string) => Promise<Blob | undefined>;
  writeFile: (path: string, blob: Blob) => Promise<void>;
  deleteFile: (path: string) => Promise<void>;
  createDirectory: ((path: string) => Promise<void>) | ((folderPath: string, directoryPath: string) => Promise<void>);
  moveEntry: (from: string, to: string) => Promise<void>;
  listFiles: () => Promise<string[]>;
  watch?: (callback: () => void) => () => void;
};

/** @emoji 🧭 Marker interface for host stores that want the kit client to push the rs-produced bundle bytes
 *  back to disk. Implementations decide *how* to write (file vs. folder vs. webview postMessage). The bytes
 *  themselves are produced exclusively by `semio/rs` (`KitStore.serializeKitStoreBundleJson`); JS treats
 *  them as opaque. */
export interface KitBundlePersisting {
  /** @emoji 📥 Bytes the host read off the backing file at mount time (empty string if the file is new / empty). */
  readonly initialBundleJson: string;
  /** @emoji 📤 Atomically write the rs-produced bundle JSON to the backing file/folder. */
  persistBundle(json: string): Promise<void>;
}

export class JsonFileKitStore implements KitHostStore, KitBundlePersisting {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal */
  readonly name = "JsonFileKitStore";
  /** @emoji 📥 Snapshot of the file at mount time (rs uses it for `kitStoreBundleHydrate`). */
  readonly initialBundleJson: string;
  private constructor(
    private readonly adapter: KitJsonFileAdapter,
    seed: Kit,
    initialBundleJson: string,
  ) {
    this._kit = seed;
    this.initialBundleJson = initialBundleJson;
  }
  static async create(adapter: KitJsonFileAdapter) {
    // 🚧 Bundle format is Rust-owned. We read the file once to capture the bytes for rs hydration,
    // but never parse them on the JS side. Rust will project the authoritative state back through
    // `applyKitClientSnapshotToLocalStore` and persist updates via `persistBundle`.
    let initialBundleJson = "";
    try {
      initialBundleJson = await adapter.read();
    } catch {
      initialBundleJson = "";
    }
    let seed = asKitInstance({ id: id(), name: "the kit", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() });
    if (initialBundleJson.trim() !== "") {
      try {
        const parsed: unknown = JSON.parse(initialBundleJson);
        const flat = KitFullDtoSchema.safeParse(parsed);
        if (flat.success) {
          seed = Kit.fromDto(flat.data);
        }
      } catch {
        /* keep Untitled — opaque rs bundle or invalid JSON */
      }
    }
    return new JsonFileKitStore(adapter, seed, initialBundleJson);
  }
  getSnapshot(): KitStoreSnapshot {
    return { kit: this._kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) {
    this.listeners.add(onChange);
    return () => {
      this.listeners.delete(onChange);
    };
  }
  replace(kit: Kit) {
    // 🚧 No JS-side persistence: bundle writing happens through `persistBundle(rs-json)` driven by the kit client subscription.
    this._kit = kit;
    for (const l of this.listeners) l();
  }
  async persistBundle(json: string): Promise<void> {
    if (json.trim() === "") return;
    await this.adapter.write(json);
  }
}

export class FolderKitStore implements KitHostStore, KitBundlePersisting {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal */
  readonly name = "FolderKitStore";
  readonly initialBundleJson: string;
  private constructor(
    private readonly adapter: KitFolderAdapter,
    seed: Kit,
    initialBundleJson: string,
  ) {
    this._kit = seed;
    this.initialBundleJson = initialBundleJson;
  }
  static async create(adapter: KitFolderAdapter, initial?: KitFullDto) {
    // 🚧 Bundle format is Rust-owned. Folder host reads the canonical kit bytes once to feed rs hydration;
    // updates are persisted through `persistBundle` via `adapter.writeKit` after every kit-client change.
    let initialBundleJson = "";
    try {
      const bytes = await adapter.readKit();
      initialBundleJson = bytes ? new TextDecoder().decode(bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes)) : "";
    } catch {
      initialBundleJson = "";
    }
    return new FolderKitStore(adapter, asKitInstance(initial ?? { id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }), initialBundleJson);
  }
  getSnapshot(): KitStoreSnapshot {
    return { kit: this._kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) {
    this.listeners.add(onChange);
    return () => {
      this.listeners.delete(onChange);
    };
  }
  replace(kit: Kit) {
    this._kit = kit;
    for (const l of this.listeners) l();
  }
  async persistBundle(json: string): Promise<void> {
    if (json.trim() === "") return;
    const bytes = new TextEncoder().encode(json);
    await this.adapter.writeKit(bytes);
  }
}

/** @emoji 🧭 True when the host store wants the kit client to drive bundle persistence (read at mount, write after each change). */
export function isKitBundlePersistingStore(store: KitHostStore): store is KitHostStore & KitBundlePersisting {
  return typeof (store as Partial<KitBundlePersisting>).persistBundle === "function" && typeof (store as Partial<KitBundlePersisting>).initialBundleJson === "string";
}

export async function createJsonFileKitStore(adapter: KitJsonFileAdapter) {
  return await JsonFileKitStore.create(adapter);
}
export async function createFolderKitStore(adapter: KitFolderAdapter, initial?: KitFullDto) {
  return await FolderKitStore.create(adapter, initial);
}

export type SessionKitStoreConfig = { serverUrl: string; sessionId?: string; kitName?: string; personId?: string; clientId?: string; authToken?: string; readOnly?: boolean };
/** @emoji 🧭 Placeholder session store: in-memory until hub sync is host-dtod. */
export async function createSessionKitStore(config: SessionKitStoreConfig) {
  const t = new Date().toISOString();
  const store = new InMemoryKitStore(asKitInstance({ id: id(), name: config.kitName ?? "Remote", createdAt: t, updatedAt: t, remote: config.serverUrl }));
  (store as InMemoryKitStore & { __semioSessionConfig?: SessionKitStoreConfig }).__semioSessionConfig = config;
  return store;
}
// #endregion KitHostStores

// #region KitFileHelpers
// @emoji 🧾 Transport-side kit file URLs, object URLs, and flattened kit ports (no domain diffs; mirrors kit JSON shape).

/**
 * @emoji 🧾 Upload/download surface used by `getKitFileProvider` / sketchpad `FileProvider` (aligned names, not re-exporting sketchpad).
 */
export type KitFileProvider = {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
};

/**
 * @emoji 🧾 Factory resolved once per opened kit; sketchpad sets this on `KitFileState`.
 */
export type KitFileProviderFactory = (kitId: string) => Promise<KitFileProvider>;

/**
 * @emoji 🧾 Per-`KitHostStore` blob/object URL and provider resolution cache (host-only; not serialized in kit).
 */
export type KitFileState = {
  objectUrls: Map<string, string>;
  providerUrls: Map<string, string>;
  blobs: Map<string, Blob>;
  pendingBlobDownloads: Map<string, Promise<string | null>>;
  providerFactory?: KitFileProviderFactory;
  /** @internal Last provider returned from {@link getKitFileProvider} for sync hooks. */
  _lastSyncProvider?: KitFileProvider;
  /** @internal */
  _cachedProviderByKitId?: Map<string, KitFileProvider>;
};

const kitFileStateByStore = new WeakMap<KitHostStore, KitFileState>();

function newKitFileState(): KitFileState {
  return { objectUrls: new Map(), providerUrls: new Map(), blobs: new Map(), pendingBlobDownloads: new Map() };
}

/** @emoji 🧾 Lazily created host cache keyed by the live `KitHostStore` (same identity as open kit). */
export function getOrCreateKitFileState(kitStore: KitHostStore): KitFileState {
  let st = kitFileStateByStore.get(kitStore);
  if (!st) {
    st = newKitFileState();
    kitFileStateByStore.set(kitStore, st);
  }
  return st;
}

const defaultKitFileProviderFactory: KitFileProviderFactory = async (kitId: string) => {
  const storage = new Map<string, Blob>();
  const key = (k: string, f: string, p: string) => `${k}/${f}/${p}`;
  return {
    upload: async (k, f, p, blob) => {
      storage.set(key(k, f, p), blob);
      return `memory://${key(k, f, p)}`;
    },
    download: async (k, f, p) => {
      const b = storage.get(key(k, f, p));
      if (!b) throw new Error(`missing ${key(k, f, p)}`);
      return b;
    },
    delete: async (k, f, p) => {
      storage.delete(key(k, f, p));
    },
    getUrl: (k, f, p) => `memory://${key(k, f, p)}`,
  };
};

/** @emoji 🧾 Async resolve + cache; warms {@link getExistingKitFileProvider} after first await. */
export async function getKitFileProvider(kitStore: KitHostStore, kitId: string): Promise<KitFileProvider> {
  const st = getOrCreateKitFileState(kitStore);
  st._cachedProviderByKitId = st._cachedProviderByKitId ?? new Map();
  const hit = st._cachedProviderByKitId.get(kitId);
  if (hit) {
    st._lastSyncProvider = hit;
    return hit;
  }
  const factory = st.providerFactory ?? defaultKitFileProviderFactory;
  const p = await factory(kitId);
  st._cachedProviderByKitId.set(kitId, p);
  st._lastSyncProvider = p;
  return p;
}

/** @emoji 🧾 Synchronous best-effort provider (after at least one {@link getKitFileProvider} call for this store). */
export function getExistingKitFileProvider(kitStore: KitHostStore): KitFileProvider | undefined {
  return getOrCreateKitFileState(kitStore)._lastSyncProvider;
}

/** @emoji 🧾 Relative path segment for sidecar / provider I/O (matches sketchpad memory layout `kitId/fileId/path`). */
export function getKitFileStoragePath(kit: Kit, file: { id: string }): string {
  void kit;
  return `files/${file.id}`;
}

export function isBrowserReadableFileUrl(u: string): boolean {
  return u.startsWith("blob:") || u.startsWith("data:") || u.startsWith("http://") || u.startsWith("https://");
}

/** @emoji 🧾 Prefer in-memory object URL, then embedded data/file URL fields. */
export function getReadableKitFileUrl(fileState: KitFileState, file: { id: string; url?: string; remote?: string }): string | null {
  const o = fileState.objectUrls.get(file.id);
  if (o) return o;
  const p = fileState.providerUrls.get(file.id);
  if (p && isBrowserReadableFileUrl(p)) return p;
  if (file.url && isBrowserReadableFileUrl(file.url)) return file.url;
  if (file.remote && isBrowserReadableFileUrl(file.remote)) return file.remote;
  return null;
}

/**
 * @emoji 🧾 Merged file-id → best readable URL for UI maps (`useKitStoredFileUrls`).
 */
export function getStoredKitFileUrls(kitStore: KitHostStore): Map<string, string> {
  const kit = kitStore.getSnapshot().kit;
  const st = getOrCreateKitFileState(kitStore);
  const out = new Map<string, string>();
  for (const f of kit.files ?? []) {
    const u = getReadableKitFileUrl(st, f);
    if (u) out.set(f.id, u);
  }
  for (const [k, v] of st.objectUrls) if (!out.has(k)) out.set(k, v);
  for (const [k, v] of st.providerUrls) if (!out.has(k) && isBrowserReadableFileUrl(v)) out.set(k, v);
  return out;
}

/** @emoji 🧾 Registers a `blob:` URL in {@link KitFileState.objectUrls} (revokes prior for same `fileId`). */
export function createKitFileObjectUrl(kitStore: KitHostStore, fileId: string, blob: Blob): string {
  const st = getOrCreateKitFileState(kitStore);
  const prev = st.objectUrls.get(fileId);
  if (prev) {
    try {
      URL.revokeObjectURL(prev);
    } catch {
      /* ignore */
    }
  }
  const url = URL.createObjectURL(blob);
  st.objectUrls.set(fileId, url);
  return url;
}

export async function fetchReadableKitFileBlob(u: string): Promise<Blob | null> {
  try {
    const r = await fetch(u);
    if (!r.ok) return null;
    return await r.blob();
  } catch {
    return null;
  }
}

/**
 * @emoji 🧾 All ports defined on families (read-only helper for schema/UI).
 */
export function getKitPorts(kit: Kit): Port[] {
  const out: Port[] = [];
  for (const fam of kit.families ?? []) for (const p of fam.ports ?? []) out.push(p);
  return out;
}
// #endregion KitFileHelpers

// #region KitStoreBinaryFacet
export type KitBinaryStore = KitHostStore & {
  readFile?: (path: string) => Promise<Blob | null>;
  writeFile?: (path: string, blob: Blob) => Promise<void>;
  deleteFile?: (path: string) => Promise<void>;
  createDirectory?: (path: string) => Promise<void>;
  moveEntry?: (from: string, to: string) => Promise<void>;
};
// #endregion KitStoreBinaryFacet

export const KitDiffSchema = z.object({ types: TypesDiffSchema.optional(), designs: DesignsDiffSchema.optional() }).passthrough();
export type KitDiff = ReadonlyDto<z.infer<typeof KitDiffSchema>>;
// #endregion Kit

// #region KitImportHelpers
/** @emoji 🧾 Recursively turns `{ items: [...] }` / Relay `{ edges: [{ node }] }` containers into plain JSON arrays for Zod DTO parsing. */
function semioDenormalizeBundleValue(v: unknown): unknown {
  if (v == null || typeof v !== "object") return v;
  if (Array.isArray(v)) return v.map(semioDenormalizeBundleValue);
  const o = v as JsonObject;
  if (Array.isArray(o.items)) {
    return (o.items as unknown[]).map(semioDenormalizeBundleValue);
  }
  if (Array.isArray(o.edges)) {
    const out: unknown[] = [];
    for (const e of o.edges) {
      if (e != null && typeof e === "object" && !Array.isArray(e) && "node" in e) {
        out.push(semioDenormalizeBundleValue((e as JsonObject).node));
      }
    }
    return out;
  }
  const out: { [key: string]: JsonValue } = {};
  for (const [k, val] of Object.entries(o)) {
    out[k] = semioDenormalizeBundleValue(val) as JsonValue;
  }
  return out;
}

/** @emoji 🧾 Lifts `*.kit.semio.json` envelope (`initialKit` / `wip.initialKit`) and flattens bundle `items` lists to {@link KitFullDto}. */
export function decodeKitSemioEnvelopeToFullDtoFromValue(v: unknown): KitFullDto {
  let inner: unknown = v;
  if (inner && typeof inner === "object" && !Array.isArray(inner)) {
    const top = inner as JsonObject;
    if (top.initialKit != null && typeof top.initialKit === "object" && !Array.isArray(top.initialKit)) {
      inner = top.initialKit;
    } else if (top.wip != null && typeof top.wip === "object" && !Array.isArray(top.wip)) {
      const wr = (top.wip as JsonObject).initialKit;
      if (wr != null && typeof wr === "object" && !Array.isArray(wr)) inner = wr;
    }
  }
  const flat = semioDenormalizeBundleValue(inner);
  return KitFullDtoSchema.parse(flat as KitJsonTreeDto);
}

/** @emoji 🧾 Decode UTF-8 kit JSON bytes (flat or `*.kit.semio.json` envelope) to {@link KitFullDto}. */
export function decodeKitSemioEnvelopeBytesToFullDto(buf: ArrayBuffer | Uint8Array): KitFullDto {
  const u8 = buf instanceof Uint8Array ? buf : new Uint8Array(buf);
  const text = new TextDecoder().decode(u8);
  return decodeKitSemioEnvelopeToFullDtoFromValue(JSON.parse(text));
}

/** @emoji 🧾 Decode kit bytes as a flat `KitFullDto` (accepts plain kit JSON or semio bundle envelope with `initialKit` / relay `items`). */
export function importKitToDto(buf: ArrayBuffer | Uint8Array): KitFullDto {
  return decodeKitSemioEnvelopeBytesToFullDto(buf);
}
// #endregion KitImportHelpers

// #region EntityKitStores
/** @emoji 🧭 Arbitrary kit entity handle: patch fields and subscribe to rs {@link KitEvent} stream. */
export class KitEntityStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly entityKind: string,
    public readonly id: string,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  async patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    void field;
    void value;
    return Promise.resolve({
      ok: false,
      error: { kind: "NotSupported", message: "use typed KitStore.submitChangeKitCommands or entity store methods" },
    });
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) {
        this._version += 1;
        handler(ev);
        return;
      }
      const hi = (ev as { HashInvalidated?: { entity?: { kind?: string; id?: string } } }).HashInvalidated;
      if (hi?.entity?.id === this.id && hi.entity.kind === this.entityKind) {
        this._version += 1;
        handler(ev);
        return;
      }
      if (jsonSubtreeHasIdKey(ev, `${this.entityKind.charAt(0).toLowerCase() + this.entityKind.slice(1)}_id`, this.id)) {
        this._version += 1;
        handler(ev);
      }
    });
  }
}

/** @emoji 🧭 Per-design kit handle: GraphQL reads and semantic design mutations on {@link KitStore}. */
export class DesignStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesDesign(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async metadata(): Promise<DesignMetadataDto> {
    const out = await this.root.read(this.readPoint, [{ readKitDesignsMetadataCommand: null }]);
    const designs = kitGraphqlJsonToReadonlyArray((out[0] as { readKitDesignsMetadataCommand?: { designs?: JsonValue } }).readKitDesignsMetadataCommand?.designs);
    const row = designs.find((d: unknown) => d && typeof d === "object" && String((d as { id?: string }).id) === this.id);
    if (!row) throw new Error(`design metadata not found: ${this.id}`);
    return DesignMetadataDtoSchema.parse(row);
  }

  async shallow(): Promise<DesignShallow> {
    const out = await this.root.read(this.readPoint, [{ readKitDesignsShallowCommand: null }]);
    const designs = kitGraphqlJsonToReadonlyArray((out[0] as { readKitDesignsShallowCommand?: { designs?: JsonValue } }).readKitDesignsShallowCommand?.designs);
    const row = designs.find((d: unknown) => d && typeof d === "object" && String((d as { id?: string }).id) === this.id);
    if (!row) throw new Error(`design shallow not found: ${this.id}`);
    return DesignShallowSchema.parse(row);
  }

  /** @emoji 🧾 Full design DTO from a kit snapshot (rs materialized truth). */
  async full(): Promise<DesignDto> {
    const kit = (await this.root.readKitSnapshotForReadPoint(this.readPoint)) as KitFullDto;
    const raw = (kit.designs ?? []).find((d) => d.id === this.id);
    if (!raw) throw new Error(`design not found: ${this.id}`);
    return DesignSchema.parse(raw);
  }

  async pieces(): Promise<readonly PieceStore[]> {
    const rows = await this.root.getPieces(this.readPoint, this.id);
    return rows.map((p) => this.root.piece(this.id, String(p.id), this.readPoint));
  }

  piece(pieceId: string): PieceStore {
    return this.root.piece(this.id, pieceId, this.readPoint);
  }

  async connections(): Promise<readonly ConnectionStore[]> {
    const rows = await this.root.getConnections(this.readPoint, this.id);
    return rows.map((c) => this.root.connection(this.id, String(c.id), this.readPoint));
  }

  connection(connectionId: string): ConnectionStore {
    return this.root.connection(this.id, connectionId, this.readPoint);
  }

  /** @emoji 🧾 Live design graph reads routed like {@link LiveDesign}. */
  private liveDesign(): LiveDesign {
    return new LiveDesign(this.root, this.readPoint, this.id);
  }

  readIncludedDesigns(): Promise<readonly IncludedDesignInfoDto[]> {
    return this.liveDesign().readIncludedDesigns();
  }

  readClusterableGroups(selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    return this.liveDesign().readClusterableGroups(selection);
  }

  readQualitySum(qualityId: string): Promise<number> {
    return this.liveDesign().readQualitySum(qualityId);
  }

  readReplaceableCatalogTypes(selection: readonly string[]): Promise<readonly string[]> {
    return this.liveDesign()
      .readReplaceableCatalog(selection)
      .then((v) => v.types);
  }

  readReplaceableCatalogDesigns(selection: readonly string[]): Promise<readonly string[]> {
    return this.liveDesign()
      .readReplaceableCatalog(selection)
      .then((v) => v.designs);
  }

  readIncludedDesignIds(): Promise<readonly string[]> {
    return this.liveDesign()
      .readIncludedDesignIds()
      .then((v) => (Array.isArray(v) ? v : []));
  }

  /** @emoji 🧾 Per-piece hierarchy + flat pose metadata (`getPiecesMetadata` / `PieceStore` GraphQL). */
  readPiecesPlacementMetadataMap(): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    return this.root.getPiecesMetadata(this.readPoint, this.id);
  }

  /** @emoji 🧾 Full piece DTO rows for this design (`getPieces`). */
  readPiecesFullRows(): Promise<readonly PieceDto[]> {
    return this.root.getPieces(this.readPoint, this.id);
  }

  /** @emoji 🧾 Full connection DTO rows for this design (`getConnections`). */
  readConnectionsFullRows(): Promise<readonly ConnectionDto[]> {
    return this.root.getConnections(this.readPoint, this.id);
  }

  setName(name: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([{ changeDesignCommands: { designId: { id: this.id }, commands: [{ name: { name } }] } }]);
  }

  cluster(pieceIds: readonly string[], name: string): Promise<SetResult> {
    return this.root.clusterPieces(this.id, pieceIds, name);
  }

  drag(pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.root.dragPieces(this.id, pieceIds, du, dv);
  }

  move(pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.root.movePieces(this.id, pieceIds, gap, shift, rise);
  }

  fix(pieceIds: readonly string[]): Promise<SetResult> {
    return this.root.fixPieces(this.id, pieceIds);
  }

  flatten(): Promise<SetResult> {
    return this.root.flattenDesign(this.id);
  }

  expand(nestedDesignId: string): Promise<SetResult> {
    return this.root.expandDesign(this.id, nestedDesignId);
  }

  paste(selection: KitJsonTreeDto, plane?: PlaneDto | null): Promise<SetResult> {
    return this.root.pasteDesignSelection(this.id, selection, plane ?? null);
  }

  createHangingPieces(typeIds: readonly string[], plane: PlaneDto): Promise<SetResult> {
    return this.root.createHangingPieces(this.id, typeIds, plane);
  }

  createConnectedPiece(parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    return this.root.createConnectedPiece(this.id, parentPiece, parentPort, childType, childPort);
  }

  createFixedPiece(typeId: string, plane: PlaneDto): Promise<SetResult> {
    return this.root.createFixedPiece(this.id, typeId, plane);
  }

  addPiece(dto: PieceDto): Promise<SetResult> {
    const piece = PieceSchema.parse(dto);
    return this.root.submitChangeKitCommands([{ changeDesignCommands: { designId: { id: this.id }, commands: [{ addPiece: { piece } }] } }]);
  }

  removePiece(pieceId: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([{ changeDesignCommands: { designId: { id: this.id }, commands: [{ removePiece: { pieceId: { id: pieceId } } }] } }]);
  }
}

/** @emoji 🧾 GraphQL `TypeMetadataObject` uses JSON `null` for absent fields; strip those before Zod DTO parse. */
function __coerceTypeMetadataGqlRow(row: JsonObject): JsonObject {
  const out = { ...row };
  for (const k of Object.keys(out)) {
    if (out[k] === null) delete out[k];
  }
  return out;
}

/** @emoji 🧭 Per-kind kit handle (semio domain kind, not TS typeof). */
export class TypeStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesType(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async metadata(): Promise<TypeMetadataDto> {
    const out = await this.root.read(this.readPoint, [{ readKitTypesMetadataCommand: null }]);
    const types = kitGraphqlJsonToReadonlyArray((out[0] as { readKitTypesMetadataCommand?: { types?: JsonValue } }).readKitTypesMetadataCommand?.types);
    const row = types.find((t: unknown) => t && typeof t === "object" && String((t as { id?: string }).id) === this.id);
    if (!row) throw new Error(`kind metadata not found: ${this.id}`);
    return TypeMetadataDtoSchema.parse(__coerceTypeMetadataGqlRow(row as JsonObject));
  }

  async shallow(): Promise<TypeShallow> {
    const out = await this.root.read(this.readPoint, [{ readKitTypesShallowCommand: null }]);
    const types = kitGraphqlJsonToReadonlyArray((out[0] as { readKitTypesShallowCommand?: { types?: JsonValue } }).readKitTypesShallowCommand?.types);
    const row = types.find((t: unknown) => t && typeof t === "object" && String((t as { id?: string }).id) === this.id);
    if (!row) throw new Error(`kind shallow not found: ${this.id}`);
    return TypeShallowSchema.parse(row);
  }

  async full(): Promise<TypeDto> {
    const kit = (await this.root.readKitSnapshotForReadPoint(this.readPoint)) as KitFullDto;
    const raw = (kit.types ?? []).find((t) => t.id === this.id);
    if (!raw) throw new Error(`kind not found: ${this.id}`);
    return TypeSchema.parse(raw);
  }

  /** @emoji 🧾 Best representation for tag ids (`readTypeBestRepresentationCommand`). */
  readBestRepresentation(tagIds: readonly string[]): Promise<RepresentationDto | null> {
    return new LiveType(this.root, this.readPoint, this.id).readBestRepresentation(tagIds);
  }

  setName(name: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ name: { name } }] } }]);
  }

  addRepresentation(dto: unknown): Promise<SetResult> {
    const representation = RepresentationSchema.parse(dto);
    return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ addRepresentation: { representation } }] } }]);
  }

  addConnector(dto: unknown): Promise<SetResult> {
    const connector = ConnectorSchema.parse(dto);
    return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ addConnector: { connector } }] } }]);
  }

  addProp(dto: unknown): Promise<SetResult> {
    const prop = PropSchema.parse(dto);
    return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ addTypeProp: { prop } }] } }]);
  }

  removeChild(childKind: string, childId: string): Promise<SetResult> {
    if (childKind === "RepresentationStore") {
      return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ removeRepresentation: { id: { id: childId } } }] } }]);
    }
    if (childKind === "ConnectorStore") {
      return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ removeConnector: { connectorId: { id: childId } } }] } }]);
    }
    if (childKind === "Prop") {
      return this.root.submitChangeKitCommands([{ changeTypeCommands: { typeId: { id: this.id }, commands: [{ removeTypeProp: { propId: { id: childId } } }] } }]);
    }
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `removeChild: ${childKind}` } });
  }
}

/** @emoji 🧭 Piece scoped to one design id plus piece id. */
export class PieceStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly designId: string,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesPiece(ev, this.designId, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  /** @emoji 🧾 Flattened placement plane in world space (`readPieceFlatPlaneCommand`). */
  readFlatPlane(): Promise<PlaneDto | null> {
    return new LivePiece(this.root, this.readPoint, this.designId, this.id).readFlatPlane();
  }

  /** @emoji 🧾 Flattened placement center (`readPieceFlatCenterCommand`). */
  readFlatCenter(): Promise<CoordinateDto | null> {
    return new LivePiece(this.root, this.readPoint, this.designId, this.id).readFlatCenter();
  }

  /** @emoji 🧾 Parent connection row when connected (`readPieceParentConnectionFullCommand`). */
  readParentConnectionFull(): Promise<ConnectionDto | null> {
    return new LivePiece(this.root, this.readPoint, this.designId, this.id).readParentConnectionFull();
  }

  async full(): Promise<PieceDto> {
    const pieces = await this.root.getPieces(this.readPoint, this.designId);
    const row = pieces.find((p) => String(p.id) === this.id);
    if (!row) throw new Error(`piece not found: ${this.id}`);
    return PieceSchema.parse(row);
  }

  setPlane(plane: PlaneDto): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ plane: { plane: plane as KitJsonTreeDto } }])]);
  }

  setCenter(center: CoordinateDto): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ center: { center: center as KitJsonTreeDto } }])]);
  }

  setScale(scale: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ scale: { scale } }])]);
  }

  setColor(color: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ color: { color } }])]);
  }

  hide(isHidden: boolean): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ hidden: { hidden: isHidden } }])]);
  }

  lock(isLocked: boolean): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ locked: { locked: isLocked } }])]);
  }

  addProp(dto: unknown): Promise<SetResult> {
    const prop = PropSchema.parse(dto);
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ addProp: { prop } }])]);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    if (field === "name") return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ name: { name: String(value) } }])]);
    if (field === "description") return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ description: { description: value as string | null } }])]);
    if (field === "type" || field === "typeId")
      return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ type: { typeId: value && typeof value === "object" && "id" in (value as object) ? (value as { id: string }) : { id: String(value) } } }])]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `piece field: ${field}` } });
  }
}

/** @emoji 🧭 Connection scoped to one design id plus connection id. */
export class ConnectionStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly designId: string,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesConnection(ev, this.designId, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<ConnectionDto> {
    const connections = await this.root.getConnections(this.readPoint, this.designId);
    const row = connections.find((c: unknown) => c && typeof c === "object" && String((c as { id?: string }).id) === this.id);
    if (!row) throw new Error(`connection not found: ${this.id}`);
    return ConnectionSchema.parse(row);
  }

  setGap(gap: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ gap: { value: gap } }])]);
  }

  setShift(shift: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ shift: { value: shift } }])]);
  }

  setRotation(rotation: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ rotation: { value: rotation } }])]);
  }

  setTilt(tilt: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ tilt: { value: tilt } }])]);
  }

  setTurn(turn: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ turn: { value: turn } }])]);
  }

  delete(): Promise<SetResult> {
    return this.root.deleteConnection(this.designId, this.id);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    if (field === "rise") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ rise: { value: Number(value) } }])]);
    if (field === "description") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ description: { value: value as string | null } }])]);
    if (field === "u" || field === "x") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ x: { value: Number(value) } }])]);
    if (field === "v" || field === "y") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ y: { value: Number(value) } }])]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `connection field: ${field}` } });
  }
}

/** @emoji 🧭 Kit family row (ports live under families in rs). */
export class FamilyStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesFamily(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<FamilyDto> {
    const kit = (await this.root.readKitSnapshotForReadPoint(this.readPoint)) as KitFullDto;
    const raw = (kit.families ?? []).find((f) => f.id === this.id);
    if (!raw) throw new Error(`family not found: ${this.id}`);
    return FamilySchema.parse(raw);
  }

  setName(name: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([{ changeFamilyCommands: { familyId: { id: this.id }, commands: [{ name: { name } }] } }]);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    if (field === "description") return this.root.submitChangeKitCommands([{ changeFamilyCommands: { familyId: { id: this.id }, commands: [{ description: { description: value as string | null } }] } }]);
    if (field === "icon") return this.root.submitChangeKitCommands([{ changeFamilyCommands: { familyId: { id: this.id }, commands: [{ icon: { icon: value as string | null } }] } }]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `family field: ${field}` } });
  }
}

/** @emoji 🧭 Kit file blob row. */
export class FileStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesFile(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<FileDto> {
    const kit = (await this.root.readKitSnapshotForReadPoint(this.readPoint)) as KitFullDto;
    const raw = (kit.files ?? []).find((f) => f.id === this.id);
    if (!raw) throw new Error(`file not found: ${this.id}`);
    return FileSchema.parse(raw);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    const fid = { id: this.id };
    if (field === "url") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ url: { url: String(value) } }] } }]);
    if (field === "mime") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ mime: { mime: value as string | null } }] } }]);
    if (field === "size") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ size: { size: value as number | null } }] } }]);
    if (field === "hash") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ hash: { hash: value as string | null } }] } }]);
    if (field === "description") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ description: { description: value as string | null } }] } }]);
    if (field === "created" || field === "createdAt") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ created: { created: value as string | null } }] } }]);
    if (field === "updated" || field === "updatedAt") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ updated: { updated: value as string | null } }] } }]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `file field: ${field}` } });
  }
}

/** @emoji 🧭 Kit folder row. */
export class FolderStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readPoint: KitReadPoint = theKitReadPoint,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesFolder(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<FolderDto> {
    const kit = (await this.root.readKitSnapshotForReadPoint(this.readPoint)) as KitFullDto;
    const raw = (kit.folders ?? []).find((f) => f.id === this.id);
    if (!raw) throw new Error(`folder not found: ${this.id}`);
    return FolderSchema.parse(raw);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    const folderId = { id: this.id };
    if (field === "path") return this.root.submitChangeKitCommands([{ changeFolderCommands: { folderId, commands: [{ path: { path: String(value) } }] } }]);
    if (field === "description") return this.root.submitChangeKitCommands([{ changeFolderCommands: { folderId, commands: [{ description: { description: value as string | null } }] } }]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `folder field: ${field}` } });
  }
}
// #endregion EntityKitStores
// #endregion 🧩KitEntitiesMerged

// #endregion 🧩KitWasmBridgeMerged


}

//#endregion 🧷KitWasmHost

// #region 🧪EmbeddedTests
if (
  typeof process !== "undefined" &&
  !!process.env &&
  process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1"
) {
  const { describe, it, expect } = await import("vitest");
  type KitFullDto = WasmGraph.KitFullDto;
  type KitJsonFileAdapter = WasmGraph.KitJsonFileAdapter;
  type KitClassifiedMutationEvent = WasmGraph.KitClassifiedMutationEvent;
  type ReadBatch = WasmGraph.ReadBatch;
  type KitEvent = WasmGraph.KitEvent;
  type ChangeKitCommand = WasmGraph.ChangeKitCommand;
  type KitStoreClient = WasmGraph.KitStoreClient;

  describe("semio-js KitStore", () => {

    it("KIT_SCOPED_FULL_DTO_QUERY matches wip.theKit { kit { fullSnapshot } }", () => {
      expect(KIT_SCOPED_FULL_DTO_QUERY).toContain("wip { theKit { kit { fullSnapshot");
    });

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

    it("JsonFileKitStore.create seeds in-memory kit from flat KitFullDto JSON (dev JSON adapters)", async () => {
      const t = "2020-01-01T00:00:00.000Z";
      const dto: KitFullDto = {
        id: "json-seed-kit",
        name: "Json Seed",
        createdAt: t,
        updatedAt: t,
        qualities: [{ id: "q1", key: "k1", folder: "fa" }],
      };
      const adapter: KitJsonFileAdapter = {
        read: async () => JSON.stringify(dto),
        write: async () => {},
      };
      const store = await JsonFileKitStore.create(adapter);
      expect(store.getSnapshot().kit.id).toBe("json-seed-kit");
      expect(store.getSnapshot().kit.qualities?.map((q) => q.id)).toEqual(["q1"]);
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

    it("designRowIds and kindRowIds align with design() and type() factory lists", async () => {
      const minimalKit: KitFullDto = {
        id: "row-ids-kit",
        name: "R",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [{ id: "ta", name: "A", connectors: [] }],
        designs: [{ id: "da", name: "D", pieces: [], connections: [] }],
      };
      const ks = await KitStore.open(minimalKit);
      expect(await ks.designRowIds()).toEqual((await ks.designs()).map((d) => d.id));
      expect(await ks.kindRowIds()).toEqual((await ks.types()).map((t) => t.id));
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

    it("persists the initial RS bundle into an empty JsonFileKitStore", async () => {
      let fileJson = "";
      const store = await createJsonFileKitStore({
        read: async () => fileJson,
        write: async (nextJson: string) => {
          fileJson = nextJson;
        },
      });
      const client = await createKitStoreClient({ initialKit: store.getSnapshot().kit.toJSON() });
      await applyKitClientSnapshotToLocalStore(client, store);
      const bundle = JSON.parse(fileJson) as JsonObject;
      expect(bundle["schema"]).toBe("🎆26🌙06⬆️1");
      expect(((bundle["wip"] as JsonObject)["initialKit"] as JsonObject)["name"]).toBe("the kit");
      expect((((bundle["wip"] as JsonObject)["checkpoints"] as JsonObject)["items"] as readonly unknown[]).length).toBe(1);
      expect((bundle["wip"] as JsonObject)["drafts"]).toBeUndefined();
      expect((bundle["wip"] as JsonObject)["savedChanges"]).toBeUndefined();
      expect((bundle["wip"] as JsonObject)["unsavedChanges"]).toBeUndefined();
      const theKit = (bundle["wip"] as JsonObject)["theKit"] as JsonObject;
      const changes = (((theKit["unsavedChanges"] as JsonObject)["items"] as readonly JsonObject[]) ?? []);
      expect(changes).toHaveLength(1);
      expect(((changes[0]["edits"] as JsonObject)["items"] as readonly unknown[])).toHaveLength(0);
      client.dispose();
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
      expect(sdl).toMatch(/type Subscription[\s\S]*\bevent\b/s);
      expect(sdl).toContain("type Mutation");
      expect(sdl).toContain("session: SessionCommandInput!");
      expect(sdl).not.toContain("type KitStoreMutation");
      expect(KIT_SESSION_QUERY_ENTRY).toContain("wip { id theKit");
      expect(KIT_EVENT_STREAM_SUBSCRIPTION).toContain("event");
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
        setKitWriteScope: () => {},
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


//#region 🧷WasmGraphFlatReexports
export import ALL_KIT_KINDS = WasmGraph.ALL_KIT_KINDS;
export import AlternativeCommandNav = WasmGraph.AlternativeCommandNav;
export import Attribute = WasmGraph.Attribute;
export import AttributeDiffSchema = WasmGraph.AttributeDiffSchema;
export import AttributeIdSchema = WasmGraph.AttributeIdSchema;
export import AttributeMetadataDtoSchema = WasmGraph.AttributeMetadataDtoSchema;
export import AttributeSchema = WasmGraph.AttributeSchema;
export import AttributeShallowSchema = WasmGraph.AttributeShallowSchema;
export import AttributesDiffSchema = WasmGraph.AttributesDiffSchema;
export import AuthorGraphDto = WasmGraph.Author;
export import AuthorDiffSchema = WasmGraph.AuthorDiffSchema;
export import AuthorIdSchema = WasmGraph.AuthorIdSchema;
export import AuthorMetadataDtoSchema = WasmGraph.AuthorMetadataDtoSchema;
export import AuthorSchema = WasmGraph.AuthorSchema;
export import AuthorShallowSchema = WasmGraph.AuthorShallowSchema;
export import AuthorsDiffSchema = WasmGraph.AuthorsDiffSchema;
export import Benchmark = WasmGraph.Benchmark;
export import BenchmarkDiffSchema = WasmGraph.BenchmarkDiffSchema;
export import BenchmarkIdSchema = WasmGraph.BenchmarkIdSchema;
export import BenchmarkMetadataDtoSchema = WasmGraph.BenchmarkMetadataDtoSchema;
export import BenchmarkSchema = WasmGraph.BenchmarkSchema;
export import BenchmarkShallowSchema = WasmGraph.BenchmarkShallowSchema;
export import BenchmarksDiffSchema = WasmGraph.BenchmarksDiffSchema;
export import Camera = WasmGraph.Camera;
export import CameraDiffSchema = WasmGraph.CameraDiffSchema;
export import CameraSchema = WasmGraph.CameraSchema;
export import CommandBuilder = WasmGraph.CommandBuilder;
export import ConceptGraphDto = WasmGraph.Concept;
export import ConceptDiffSchema = WasmGraph.ConceptDiffSchema;
export import ConceptIdSchema = WasmGraph.ConceptIdSchema;
export import ConceptMetadataDtoSchema = WasmGraph.ConceptMetadataDtoSchema;
export import ConceptSchema = WasmGraph.ConceptSchema;
export import ConceptShallowSchema = WasmGraph.ConceptShallowSchema;
export import ConceptsDiffSchema = WasmGraph.ConceptsDiffSchema;
export import ConnectionGraphDto = WasmGraph.Connection;
export import ConnectionDiffSchema = WasmGraph.ConnectionDiffSchema;
export import ConnectionIdSchema = WasmGraph.ConnectionIdSchema;
export import ConnectionMetadataDtoSchema = WasmGraph.ConnectionMetadataDtoSchema;
export import ConnectionSchema = WasmGraph.ConnectionSchema;
export import ConnectionShallowSchema = WasmGraph.ConnectionShallowSchema;
export import ConnectionStore = WasmGraph.ConnectionStore;
export import ConnectionsDiffSchema = WasmGraph.ConnectionsDiffSchema;
export import ConnectorGraphDto = WasmGraph.Connector;
export import ConnectorDiffSchema = WasmGraph.ConnectorDiffSchema;
export import ConnectorIdSchema = WasmGraph.ConnectorIdSchema;
export import ConnectorMetadataDtoSchema = WasmGraph.ConnectorMetadataDtoSchema;
export import ConnectorSchema = WasmGraph.ConnectorSchema;
export import ConnectorShallowSchema = WasmGraph.ConnectorShallowSchema;
export import ConnectorsDiffSchema = WasmGraph.ConnectorsDiffSchema;
export import Coordinate = WasmGraph.Coordinate;
export import CoordinateDiffSchema = WasmGraph.CoordinateDiffSchema;
export import CoordinateSchema = WasmGraph.CoordinateSchema;
export import DEFAULT_KIT_SYNC = WasmGraph.DEFAULT_KIT_SYNC;
export import DesignGraphDto = WasmGraph.Design;
export import DesignDiffSchema = WasmGraph.DesignDiffSchema;
export import DesignIdSchema = WasmGraph.DesignIdSchema;
export import DesignMetadataDtoSchema = WasmGraph.DesignMetadataDtoSchema;
export import DesignSchema = WasmGraph.DesignSchema;
export import DesignShallowSchema = WasmGraph.DesignShallowSchema;
export import DesignStore = WasmGraph.DesignStore;
export import DesignsDiffSchema = WasmGraph.DesignsDiffSchema;
export import DiffStatusSchema = WasmGraph.DiffStatusSchema;
export import FamiliesDiffSchema = WasmGraph.FamiliesDiffSchema;
export import FamilyGraphDto = WasmGraph.Family;
export import FamilyDiffSchema = WasmGraph.FamilyDiffSchema;
export import FamilyIdSchema = WasmGraph.FamilyIdSchema;
export import FamilyMetadataDtoSchema = WasmGraph.FamilyMetadataDtoSchema;
export import FamilySchema = WasmGraph.FamilySchema;
export import FamilyShallowSchema = WasmGraph.FamilyShallowSchema;
export import FamilyStore = WasmGraph.FamilyStore;
export import File = WasmGraph.File;
export import FileDiffSchema = WasmGraph.FileDiffSchema;
export import FileIdSchema = WasmGraph.FileIdSchema;
export import FileMetadataDtoSchema = WasmGraph.FileMetadataDtoSchema;
export import FileSchema = WasmGraph.FileSchema;
export import FileShallowSchema = WasmGraph.FileShallowSchema;
export import FileStore = WasmGraph.FileStore;
export import FilesDiffSchema = WasmGraph.FilesDiffSchema;
export import Folder = WasmGraph.Folder;
export import FolderDiffSchema = WasmGraph.FolderDiffSchema;
export import FolderIdSchema = WasmGraph.FolderIdSchema;
export import FolderKitStore = WasmGraph.FolderKitStore;
export import FolderMetadataDtoSchema = WasmGraph.FolderMetadataDtoSchema;
export import FolderSchema = WasmGraph.FolderSchema;
export import FolderShallowSchema = WasmGraph.FolderShallowSchema;
export import FolderStore = WasmGraph.FolderStore;
export import FoldersDiffSchema = WasmGraph.FoldersDiffSchema;
export import Group = WasmGraph.Group;
export import GroupDiffSchema = WasmGraph.GroupDiffSchema;
export import GroupIdSchema = WasmGraph.GroupIdSchema;
export import GroupMetadataDtoSchema = WasmGraph.GroupMetadataDtoSchema;
export import GroupSchema = WasmGraph.GroupSchema;
export import GroupShallowSchema = WasmGraph.GroupShallowSchema;
export import GroupsDiffSchema = WasmGraph.GroupsDiffSchema;
export import ICON_WIDTH = WasmGraph.ICON_WIDTH;
export import InMemoryKitStore = WasmGraph.InMemoryKitStore;
export import JsonFileKitStore = WasmGraph.JsonFileKitStore;
export import KIT_COMMAND_SUCCEEDED_SUBSCRIPTION = WasmGraph.KIT_COMMAND_SUCCEEDED_SUBSCRIPTION;
export import KIT_OPERATION_FAILED_SUBSCRIPTION = WasmGraph.KIT_OPERATION_FAILED_SUBSCRIPTION;
export import KIT_SCOPED_FULL_DTO_QUERY = WasmGraph.KIT_SCOPED_FULL_DTO_QUERY;
export import KIT_SESSION_QUERY_ENTRY = WasmGraph.KIT_SESSION_QUERY_ENTRY;
export import KitGraphDto = WasmGraph.Kit;
export import KitDiffSchema = WasmGraph.KitDiffSchema;
export import KitEntityStore = WasmGraph.KitEntityStore;
export import KitFullDtoSchema = WasmGraph.KitFullDtoSchema;
export import KitIdSchema = WasmGraph.KitIdSchema;
export import KitKindSchema = WasmGraph.KitKindSchema;
export import KitOperationNav = WasmGraph.KitOperationNav;
export import KitStore = WasmGraph.KitStore;
export import Layer = WasmGraph.Layer;
export import LayerDiffSchema = WasmGraph.LayerDiffSchema;
export import LayerIdSchema = WasmGraph.LayerIdSchema;
export import LayerMetadataDtoSchema = WasmGraph.LayerMetadataDtoSchema;
export import LayerSchema = WasmGraph.LayerSchema;
export import LayerShallowSchema = WasmGraph.LayerShallowSchema;
export import LayersDiffSchema = WasmGraph.LayersDiffSchema;
export import LiveKitRoot = WasmGraph.LiveKitRoot;
export import Location = WasmGraph.Location;
export import LocationDiffSchema = WasmGraph.LocationDiffSchema;
export import LocationIdSchema = WasmGraph.LocationIdSchema;
export import LocationMetadataDtoSchema = WasmGraph.LocationMetadataDtoSchema;
export import LocationSchema = WasmGraph.LocationSchema;
export import LocationShallowSchema = WasmGraph.LocationShallowSchema;
export import PieceGraphDto = WasmGraph.Piece;
export import PieceDiffSchema = WasmGraph.PieceDiffSchema;
export import PieceIdSchema = WasmGraph.PieceIdSchema;
export import PieceMetadataDtoSchema = WasmGraph.PieceMetadataDtoSchema;
export import PieceSchema = WasmGraph.PieceSchema;
export import PieceShallowSchema = WasmGraph.PieceShallowSchema;
export import PieceStore = WasmGraph.PieceStore;
export import PiecesDiffSchema = WasmGraph.PiecesDiffSchema;
export import Plane = WasmGraph.Plane;
export import PlaneDiffSchema = WasmGraph.PlaneDiffSchema;
export import PlaneSchema = WasmGraph.PlaneSchema;
export import Point = WasmGraph.Point;
export import PointDiffSchema = WasmGraph.PointDiffSchema;
export import PointSchema = WasmGraph.PointSchema;
export import PortGraphDto = WasmGraph.Port;
export import PortDiffSchema = WasmGraph.PortDiffSchema;
export import PortIdSchema = WasmGraph.PortIdSchema;
export import PortMetadataDtoSchema = WasmGraph.PortMetadataDtoSchema;
export import PortSchema = WasmGraph.PortSchema;
export import PortShallowSchema = WasmGraph.PortShallowSchema;
export import PortsDiffSchema = WasmGraph.PortsDiffSchema;
export import Prop = WasmGraph.Prop;
export import PropDiffSchema = WasmGraph.PropDiffSchema;
export import PropIdSchema = WasmGraph.PropIdSchema;
export import PropMetadataDtoSchema = WasmGraph.PropMetadataDtoSchema;
export import PropSchema = WasmGraph.PropSchema;
export import PropShallowSchema = WasmGraph.PropShallowSchema;
export import PropsDiffSchema = WasmGraph.PropsDiffSchema;
export import QualitiesDiffSchema = WasmGraph.QualitiesDiffSchema;
export import QualityGraphDto = WasmGraph.Quality;
export import QualityDiffSchema = WasmGraph.QualityDiffSchema;
export import QualityIdSchema = WasmGraph.QualityIdSchema;
export import QualityMetadataDtoSchema = WasmGraph.QualityMetadataDtoSchema;
export import QualitySchema = WasmGraph.QualitySchema;
export import QualityShallowSchema = WasmGraph.QualityShallowSchema;
export import RepresentationGraphDto = WasmGraph.Representation;
export import RepresentationDiffSchema = WasmGraph.RepresentationDiffSchema;
export import RepresentationIdSchema = WasmGraph.RepresentationIdSchema;
export import RepresentationMetadataDtoSchema = WasmGraph.RepresentationMetadataDtoSchema;
export import RepresentationSchema = WasmGraph.RepresentationSchema;
export import RepresentationShallowSchema = WasmGraph.RepresentationShallowSchema;
export import RepresentationsDiffSchema = WasmGraph.RepresentationsDiffSchema;
export import RequestCorrelator = WasmGraph.RequestCorrelator;
export import SEMIO_KIT_STORE_CONTROL_COMMAND_KINDS = WasmGraph.SEMIO_KIT_STORE_CONTROL_COMMAND_KINDS;
export import SemioKitDesignReadStore = WasmGraph.SemioKitDesignReadStore;
export import SemioKitLiveReadStore = WasmGraph.SemioKitLiveReadStore;
export import SemioKitShallowListReadStore = WasmGraph.SemioKitShallowListReadStore;
export import SemioKitViewStore = WasmGraph.SemioKitViewStore;
export import SessionCommandNav = WasmGraph.SessionCommandNav;
export import Side = WasmGraph.Side;
export import SideDiffSchema = WasmGraph.SideDiffSchema;
export import SideId = WasmGraph.SideId;
export import SideIdSchema = WasmGraph.SideIdSchema;
export import SideSchema = WasmGraph.SideSchema;
export import SidesDiffSchema = WasmGraph.SidesDiffSchema;
export import Stat = WasmGraph.Stat;
export import StatDiffSchema = WasmGraph.StatDiffSchema;
export import StatIdSchema = WasmGraph.StatIdSchema;
export import StatMetadataDtoSchema = WasmGraph.StatMetadataDtoSchema;
export import StatSchema = WasmGraph.StatSchema;
export import StatShallowSchema = WasmGraph.StatShallowSchema;
export import StatsDiffSchema = WasmGraph.StatsDiffSchema;
export import StoreCommand = WasmGraph.StoreCommand;
export import StoreField = WasmGraph.StoreField;
export import TOLERANCE = WasmGraph.TOLERANCE;
export import TagGraphDto = WasmGraph.Tag;
export import TagDiffSchema = WasmGraph.TagDiffSchema;
export import TagIdSchema = WasmGraph.TagIdSchema;
export import TagMetadataDtoSchema = WasmGraph.TagMetadataDtoSchema;
export import TagSchema = WasmGraph.TagSchema;
export import TagShallowSchema = WasmGraph.TagShallowSchema;
export import TagsDiffSchema = WasmGraph.TagsDiffSchema;
export import TypeGraphDto = WasmGraph.Type;
export import TypeDiffSchema = WasmGraph.TypeDiffSchema;
export import TypeIdSchema = WasmGraph.TypeIdSchema;
export import TypeMetadataDtoSchema = WasmGraph.TypeMetadataDtoSchema;
export import TypeSchema = WasmGraph.TypeSchema;
export import TypeShallowSchema = WasmGraph.TypeShallowSchema;
export import TypeStore = WasmGraph.TypeStore;
export import TypesDiffSchema = WasmGraph.TypesDiffSchema;
export import UnsavedChangeCommandNav = WasmGraph.UnsavedChangeCommandNav;
export import Vec = WasmGraph.Vec;
export import VecDiffSchema = WasmGraph.VecDiffSchema;
export import VecSchema = WasmGraph.VecSchema;
export import Vector = WasmGraph.Vector;
export import VectorDiffSchema = WasmGraph.VectorDiffSchema;
export import VectorSchema = WasmGraph.VectorSchema;
export import VersionCommandNav = WasmGraph.VersionCommandNav;
export import WRITE_STATUS_IDLE = WasmGraph.WRITE_STATUS_IDLE;
export import WRITE_STATUS_PENDING = WasmGraph.WRITE_STATUS_PENDING;
export import WRITE_STATUS_READONLY = WasmGraph.WRITE_STATUS_READONLY;
export import WasmKitStoreClient = WasmGraph.WasmKitStoreClient;
export import acquireSemioKitCommandFacade = WasmGraph.acquireSemioKitCommandFacade;
export import applyKitClientSnapshotToLocalStore = WasmGraph.applyKitClientSnapshotToLocalStore;
export import asKitInstance = WasmGraph.asKitInstance;
export import buildSchemaEntityChangeCommands = WasmGraph.buildSchemaEntityChangeCommands;
export import connectionDiffKeyForDataKey = WasmGraph.connectionDiffKeyForDataKey;
export import connectionPatchToChangeCommands = WasmGraph.connectionPatchToChangeCommands;
export import createFolderKitStore = WasmGraph.createFolderKitStore;
export import createJsonFileKitStore = WasmGraph.createJsonFileKitStore;
export import createKitFileObjectUrl = WasmGraph.createKitFileObjectUrl;
export import createKitStoreClient = WasmGraph.createKitStoreClient;
export import createSessionKitStore = WasmGraph.createSessionKitStore;
export import decodeKitSemioEnvelopeBytesToFullDto = WasmGraph.decodeKitSemioEnvelopeBytesToFullDto;
export import decodeKitSemioEnvelopeToFullDtoFromValue = WasmGraph.decodeKitSemioEnvelopeToFullDtoFromValue;
export import fetchReadableKitFileBlob = WasmGraph.fetchReadableKitFileBlob;
export import getExistingKitFileProvider = WasmGraph.getExistingKitFileProvider;
export import getKitClientReadPoint = WasmGraph.getKitClientReadPoint;
export import getKitFileProvider = WasmGraph.getKitFileProvider;
export import getKitFileStoragePath = WasmGraph.getKitFileStoragePath;
export import getKitPorts = WasmGraph.getKitPorts;
export import getOrCreateKitFileState = WasmGraph.getOrCreateKitFileState;
export import getReadableKitFileUrl = WasmGraph.getReadableKitFileUrl;
export import getSemioKitDesignReadStore = WasmGraph.getSemioKitDesignReadStore;
export import getSemioKitLiveReadStore = WasmGraph.getSemioKitLiveReadStore;
export import getSemioKitShallowListReadStore = WasmGraph.getSemioKitShallowListReadStore;
export import getSemioKitViewStore = WasmGraph.getSemioKitViewStore;
export import getStoredKitFileUrls = WasmGraph.getStoredKitFileUrls;
export import id = WasmGraph.id;
export import importKitToDto = WasmGraph.importKitToDto;
export import isBrowserReadableFileUrl = WasmGraph.isBrowserReadableFileUrl;
export import isKitBundlePersistingStore = WasmGraph.isKitBundlePersistingStore;
export import isKitClassifiedMutationEvent = WasmGraph.isKitClassifiedMutationEvent;
export import isKitCommandLifecycleEvent = WasmGraph.isKitCommandLifecycleEvent;
export import kitChangeDesignConnection = WasmGraph.kitChangeDesignConnection;
export import kitChangeDesignPiece = WasmGraph.kitChangeDesignPiece;
export import kitChangeSemanticKindToGraphQl = WasmGraph.kitChangeSemanticKindToGraphQl;
export import kitEventAffectsCanUndoRedo = WasmGraph.kitEventAffectsCanUndoRedo;
export import kitEventAffectsDesignQualitySumRead = WasmGraph.kitEventAffectsDesignQualitySumRead;
export import kitEventAffectsKitColoredConnectorsRead = WasmGraph.kitEventAffectsKitColoredConnectorsRead;
export import kitEventAffectsPieceLiveRead = WasmGraph.kitEventAffectsPieceLiveRead;
export import kitEventAffectsReplaceableCatalogRead = WasmGraph.kitEventAffectsReplaceableCatalogRead;
export import kitEventAffectsTypeScopedRead = WasmGraph.kitEventAffectsTypeScopedRead;
export import kitEventTouchesConnection = WasmGraph.kitEventTouchesConnection;
export import kitEventTouchesDesign = WasmGraph.kitEventTouchesDesign;
export import kitEventTouchesDesignStrict = WasmGraph.kitEventTouchesDesignStrict;
export import kitEventTouchesFamily = WasmGraph.kitEventTouchesFamily;
export import kitEventTouchesFile = WasmGraph.kitEventTouchesFile;
export import kitEventTouchesFolder = WasmGraph.kitEventTouchesFolder;
export import kitEventTouchesPiece = WasmGraph.kitEventTouchesPiece;
export import kitEventTouchesType = WasmGraph.kitEventTouchesType;
export import kitEventTouchesTypeStrict = WasmGraph.kitEventTouchesTypeStrict;
export import kitGraphqlRunTyped = WasmGraph.kitGraphqlRunTyped;
export import kitReadPointToGqlVariables = WasmGraph.kitReadPointToGqlVariables;
export import kitStoreClientAddChildByKind = WasmGraph.kitStoreClientAddChildByKind;
export import kitStoreClientAddConnection = WasmGraph.kitStoreClientAddConnection;
export import kitStoreClientAddPiece = WasmGraph.kitStoreClientAddPiece;
export import kitStoreClientRemoveChildByKind = WasmGraph.kitStoreClientRemoveChildByKind;
export import kitStoreClientRemovePiece = WasmGraph.kitStoreClientRemovePiece;
export import kitStoreClientUpdateConnection = WasmGraph.kitStoreClientUpdateConnection;
export import kitStoreClientUpdatePiece = WasmGraph.kitStoreClientUpdatePiece;
export import kitStoreFromKitStoreClient = WasmGraph.kitStoreFromKitStoreClient;
export import normalizeDesignCopyResult = WasmGraph.normalizeDesignCopyResult;
export import normalizeDesignDiffResult = WasmGraph.normalizeDesignDiffResult;
export import normalizeDesignFlattenResult = WasmGraph.normalizeDesignFlattenResult;
export import normalizeKitEventFromSubscription = WasmGraph.normalizeKitEventFromSubscription;
export import normalizeKitFullDtoFolderPaths = WasmGraph.normalizeKitFullDtoFolderPaths;
export import openKitStore = WasmGraph.openKit;
export import piecePatchToChangeCommands = WasmGraph.piecePatchToChangeCommands;
export import releaseSemioKitCommandFacade = WasmGraph.releaseSemioKitCommandFacade;
export import resolveDesignIdForPieceOrConnection = WasmGraph.resolveDesignIdForPieceOrConnection;
export import submitKitChangeCommands = WasmGraph.submitKitChangeCommands;
export import writeKitStoreClientSchemaField = WasmGraph.writeKitStoreClientSchemaField;
export import writeStatusEquivalent = WasmGraph.writeStatusEquivalent;
//#endregion 🧷WasmGraphFlatReexports



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
//#endregion 🧪Tests
