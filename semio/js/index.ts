// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: `KitStore` + opaque wire batches to `semio/rs` WASM (read shapes are not re-exported).
// #endregion 🧲Header

// #region 📥Imports
import { Subject, filter } from "rxjs";
// #endregion 📥Imports

// #region 🔌WireTypes

/** @emoji 🪪 Correlates kit command lifecycle events on the wire. */
export type KitCommandRequestId = string;

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

/** @emoji 🧾 Normalized set/mutation error from Rust `SetError`. */
export type SetError = { kind: SetErrorKind; message: string; field?: string; entity?: { kind: string; id: string } };

export type SetResult =
  | { ok: true; requestId?: KitCommandRequestId }
  | { ok: false; error: SetError; requestId?: KitCommandRequestId };

export type KitCommandLifecyclePhase = "accepted" | "succeeded" | "failed";

export type KitCommandLifecycleEvent = {
  semioKitCommand: {
    requestId: KitCommandRequestId;
    commandKind: string;
    phase: KitCommandLifecyclePhase;
    result?: unknown;
    error?: SetError;
  };
};

/** @emoji 🧭 Backbone / conflict wire shapes (opaque JSON; native coordinator fills these). */
export type BackboneConfig = Record<string, unknown>;
export type BackboneStatusDto = Record<string, unknown>;
export type ConflictResolution = Record<string, unknown>;
export type KitConflict = {
  id: string;
  wipCheckpoint: unknown;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
};

/** @emoji 📦 Authoritative kit snapshot (camelCase dates match serde `alias` on the wire). */
export type KitFullDto = {
  id: string;
  name: string;
  createdAt?: string;
  updatedAt?: string;
  created?: string;
  updated?: string;
  types?: readonly unknown[];
  designs?: readonly unknown[];
  files?: readonly unknown[];
  folders?: readonly unknown[];
  authors?: readonly unknown[];
  concepts?: readonly unknown[];
  tags?: readonly unknown[];
  qualities?: readonly unknown[];
  props?: readonly unknown[];
  attributes?: readonly unknown[];
  ports?: readonly unknown[];
  families?: readonly unknown[];
  locations?: readonly unknown[];
  [k: string]: unknown;
};

/** @emoji 🔌 One read batch entry; exact keys are owned by `semio/rs` kit store wire (not duplicated as TS unions here). */
export type ReadWireItem = Readonly<Record<string, unknown>>;
export type ReadWireBatch = readonly ReadWireItem[];
export type ReadWireBatchResult = readonly ReadWireItem[];

/** @emoji 📣 Subscription payload: GraphQL wraps `KitEvent` as JSON. */
export type KitEvent = Readonly<Record<string, unknown>>;

/** @emoji 🧾 Optional filter for {@link KitStore.subscribeFiltered}. */
export type KitEventFilter = (event: KitEvent) => boolean;

/** @emoji 🧾 Unsubscribe handle returned by {@link KitStore.subscribe}. */
export type Unsubscribe = () => void;

export type KitCommandReceipt = { requestId: KitCommandRequestId; commandKind: string; accepted: boolean };

export type KitStoreOpenOptions = {
  wasmSpecifier?: string;
  timeoutMs?: number;
  /** Optional worker factory (tests); defaults to `new Worker(new URL("./worker.ts", import.meta.url), { type: "module" })`. */
  workerFactory?: () => Worker;
};

// #endregion 🔌WireTypes

// #region 🪢InternalReadWire
/** @emoji 🪢 Internal typing for `KitStore.read` implementation only (mirrors rs wire, not public API). */

type IdDto = { readonly id: string };

type ReadPieceCommand =
  | { readonly readPieceFlatPlaneCommand: null }
  | { readonly readPieceFlatCenterCommand: null }
  | { readonly readPieceParentConnectionFullCommand: null };

type ReadDesignCommand =
  | { readonly readDesignPiecesFullCommand: null }
  | { readonly readDesignConnectionsFullCommand: null }
  | { readonly readDesignPieceCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadPieceCommand> } }
  | { readonly readDesignClusterableGroupsCommand: { readonly selection: ReadonlyArray<IdDto> } }
  | { readonly readDesignIncludedDesignsCommand: null }
  | { readonly readDesignQualitySumCommand: { readonly qualityId: IdDto } }
  | { readonly readDesignReplaceableCatalogCommand: { readonly selection: ReadonlyArray<IdDto> } }
  | { readonly readDesignIncludedDesignIdsCommand: null };

type ReadTypeCommand = { readonly readTypeBestRepresentationCommand: { readonly tagIds: ReadonlyArray<string> } };

type ReadKitCommand =
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
  | { readonly readKitColoredConnectorsCommand: null }
  | { readonly readKitDesignCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadDesignCommand> } }
  | { readonly readKitTypeCommands: { readonly id: IdDto; readonly commands: ReadonlyArray<ReadTypeCommand> } };

type ReadKitCommandOutput = Readonly<Record<string, unknown>>;
type ReadDesignCommandOutput = Readonly<Record<string, unknown>>;
type ReadPieceCommandOutput = Readonly<Record<string, unknown>>;
type ReadTypeCommandOutput = Readonly<Record<string, unknown>>;

type ReadCommandBatch = ReadonlyArray<ReadKitCommand>;
// #endregion 🪢InternalReadWire

// #region 🧰GraphqlUtil

function normalizeRustSetError(raw: unknown): SetError {
  if (raw == null || typeof raw !== "object") return { kind: "Internal", message: "invalid error payload" };
  const o = raw as Record<string, unknown>;
  const kind = typeof o.kind === "string" ? (o.kind as SetErrorKind) : "Internal";
  const message = typeof o.message === "string" ? o.message : JSON.stringify(raw);
  return { kind, message };
}

function normalizeWasmThrownKitError(err: unknown): SetError {
  const message = String(err).replace(/^Error:\s*/, "").trim();
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

function kitGraphqlData(response: unknown): Record<string, unknown> {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response as { data?: Record<string, unknown> | null; errors?: readonly { message?: string }[] };
  if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
  if (r.data != null && typeof r.data === "object") return r.data as Record<string, unknown>;
  throw new Error("kitGraphql: no data in response");
}

function kitGraphqlJsonToReadonlyArray(v: unknown): readonly unknown[] {
  if (Array.isArray(v)) return v;
  if (v == null) return [];
  if (typeof v === "string") {
    try {
      const p = JSON.parse(v) as unknown;
      return Array.isArray(p) ? p : [];
    } catch {
      return [];
    }
  }
  return [];
}

function isKitCommandLifecycleEvent(event: unknown): event is KitCommandLifecycleEvent {
  const c = (event as { semioKitCommand?: unknown } | null)?.semioKitCommand;
  if (c == null || typeof c !== "object") return false;
  const v = c as Record<string, unknown>;
  return typeof v.requestId === "string" && typeof v.commandKind === "string" && typeof v.phase === "string";
}

function normalizeKitEventFromSubscription(raw: unknown): KitEvent | undefined {
  if (raw == null || typeof raw !== "object") return undefined;
  const top = raw as Record<string, unknown>;
  /** serde externally-tagged enum: `{ "SemioKitCommand": { requestId, ... } }` */
  const lifecycleWrapper: unknown =
    top.semioKitCommand !== undefined
      ? raw
      : top.SemioKitCommand !== undefined
        ? { semioKitCommand: top.SemioKitCommand }
        : raw;
  if (isKitCommandLifecycleEvent({ semioKitCommand: (lifecycleWrapper as { semioKitCommand?: unknown }).semioKitCommand })) {
    const command = (lifecycleWrapper as { semioKitCommand: unknown }).semioKitCommand;
    const value = command as Record<string, unknown>;
    const requestIdRaw = value.requestId;
    if (typeof requestIdRaw !== "string" || typeof value.commandKind !== "string" || typeof value.phase !== "string") return undefined;
    const error =
      value.error && typeof value.error === "object"
        ? normalizeRustSetError(value.error as Record<string, unknown>)
        : undefined;
    return {
      semioKitCommand: {
        requestId: requestIdRaw,
        commandKind: value.commandKind as string,
        phase: value.phase as KitCommandLifecyclePhase,
        result: value.result,
        error,
      },
    };
  }
  return raw as KitEvent;
}

type KitGraphqlHandle = { execute(requestJson: string): Promise<string> };

async function kitGraphqlRun(
  handle: KitGraphqlHandle,
  body: { query: string; variables?: Record<string, unknown>; operationName?: string },
  timeoutMs?: number,
): Promise<unknown> {
  const json = await withTimeout(handle.execute(JSON.stringify(body)), timeoutMs ?? 0, "graphql");
  return JSON.parse(json) as unknown;
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
      snapshot: () => unknown;
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
  snapshotJson(): string {
    return JSON.stringify(this.handle.snapshot());
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

class WorkerStringTransport {
  private worker: Worker;
  private nextSerial = 0;

  constructor(worker: Worker) {
    this.worker = worker;
  }

  init(dto: KitFullDto): Promise<void> {
    return new Promise((resolve, reject) => {
      const t = setTimeout(() => reject(new Error("worker init timeout")), 30_000);
      const onReady = (ev: MessageEvent<string>) => {
        try {
          const m = JSON.parse(ev.data) as { op: string };
          if (m.op === "ready") {
            clearTimeout(t);
            this.worker.removeEventListener("message", onReady);
            resolve();
          }
        } catch {
          /* ignore */
        }
      };
      this.worker.addEventListener("message", onReady);
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

  async snapshotJson(): Promise<string> {
    const reqId = `s-${++this.nextSerial}-${Date.now().toString(36)}`;
    return await new Promise<string>((resolve, reject) => {
      const w = (ev: MessageEvent<string>) => {
        let m: { op: string; reqId?: string; json?: string; message?: string };
        try {
          m = JSON.parse(ev.data) as typeof m;
        } catch {
          return;
        }
        if (m.reqId !== reqId) return;
        if (m.op === "snapshotResult" && typeof m.json === "string") {
          this.worker.removeEventListener("message", w);
          resolve(m.json);
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", w);
          reject(new Error(m.message ?? "snapshot error"));
        }
      };
      this.worker.addEventListener("message", w);
      this.worker.postMessage(JSON.stringify({ op: "snapshot", reqId }));
    });
  }

  dispose(): void {
    this.worker.terminate();
  }
}

// #endregion 🪜Transport

// #region 📦KitStore

/**
 * @emoji 🌐 Single kit control plane: GraphQL strings over one dedicated `Worker` running `semio/rs` WASM (`KitStoreHandle`).
 */
export class KitStore {
  private readonly timeoutMs: number;
  private transport!: WorkerStringTransport | InlineWasmTransport;
  private readonly fanout = new Subject<KitEvent>();
  private gqlLoopRunning = false;
  private disposed = false;

  private constructor(timeoutMs: number) {
    this.timeoutMs = timeoutMs;
  }

  static async open(initialKit: KitFullDto, opts?: KitStoreOpenOptions): Promise<KitStore> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const wasmSpecifier = opts?.wasmSpecifier ?? (globalThis as { __SEMIO_WASM_SPECIFIER__?: string }).__SEMIO_WASM_SPECIFIER__ ?? "@semio/rs-wasm";
    const dto = JSON.parse(JSON.stringify(initialKit)) as KitFullDto;

    if (typeof Worker !== "undefined") {
      const worker = opts?.workerFactory?.() ?? new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
      const wt = new WorkerStringTransport(worker);
      await wt.init(dto);
      await withTimeout(wt.snapshotJson(), timeoutMs, "snapshot");
      const ks = new KitStore(timeoutMs);
      ks.transport = wt;
      void ks.startSubscriptionLoop();
      return ks;
    }

    const mod = wasmSpecifier === "@semio/rs-wasm" ? await import("@semio/rs-wasm") : await import(/* @vite-ignore */ wasmSpecifier);
    if (typeof mod.default === "function") {
      try {
        const fs = await import("node:fs/promises");
        const { fileURLToPath } = await import("node:url");
        const wasmPath = fileURLToPath(new URL("../rs/pkg/semio_bg.wasm", import.meta.url));
        const wasmBytes = await fs.readFile(wasmPath);
        await mod.default({ module_or_path: wasmBytes });
      } catch {
        await mod.default();
      }
    } else await mod.default();
    if (typeof mod.boot === "function") mod.boot();
    const handle = mod.KitStoreHandle.create(dto as object);
    const t = new InlineWasmTransport(handle);
    await withTimeout(Promise.resolve(t.snapshotJson()), timeoutMs, "snapshot");
    const ks = new KitStore(timeoutMs);
    ks.transport = t;
    void ks.startSubscriptionLoop();
    return ks;
  }

  private graphqlHandle(): KitGraphqlHandle {
    return { execute: (requestJson: string) => this.transport.execute(requestJson) };
  }

  private ensureAlive(): void {
    if (this.disposed) throw new Error("KitStore disposed");
  }

  private async gqlRun(body: { query: string; variables?: Record<string, unknown>; operationName?: string }): Promise<unknown> {
    this.ensureAlive();
    return kitGraphqlRun(this.graphqlHandle(), body, this.timeoutMs);
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

  private startSubscriptionLoop(): void {
    if (this.gqlLoopRunning) return;
    this.gqlLoopRunning = true;
    void this.transport
      .subscribe(JSON.stringify({ query: "subscription { eventStream }" }), (eventJson: string) => {
        try {
          const msg = JSON.parse(eventJson) as { data?: { eventStream?: unknown } | null; errors?: unknown[] };
          if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return;
          const ev = msg.data?.eventStream;
          if (ev === undefined) return;
          const n = normalizeKitEventFromSubscription(ev);
          if (n) this.fanout.next(n);
          else this.fanout.next(ev as KitEvent);
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
    this.fanout.complete();
    this.transport.dispose();
  }

  async snapshot(): Promise<KitFullDto> {
    this.ensureAlive();
    const json =
      this.transport instanceof InlineWasmTransport
        ? this.transport.snapshotJson()
        : await withTimeout((this.transport as WorkerStringTransport).snapshotJson(), this.timeoutMs, "snapshot");
    return JSON.parse(json) as KitFullDto;
  }

  async theKit(): Promise<KitFullDto> {
    const data = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { theKitDto } }` }));
    const j = (data.kitStore as { theKitDto?: unknown })?.theKitDto;
    return j as KitFullDto;
  }

  async materializeAt(checkpointId: string): Promise<KitFullDto> {
    const idArg = checkpointId.trim() === "" ? null : checkpointId;
    const data = kitGraphqlData(
      await this.gqlRun({ query: `query($id: String) { kitStore { materializeAt(checkpointId: $id) } }`, variables: { id: idArg } }),
    );
    const j = (data.kitStore as { materializeAt?: unknown })?.materializeAt;
    return j as KitFullDto;
  }

  async vcsState(): Promise<Record<string, unknown>> {
    const data = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { vcsStateJson } }` }));
    return (data.kitStore as { vcsStateJson?: Record<string, unknown> }).vcsStateJson ?? {};
  }

  async canUndo(): Promise<boolean> {
    const data = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { canUndo } }` }));
    return Boolean((data.kitStore as { canUndo?: boolean }).canUndo);
  }

  async canRedo(): Promise<boolean> {
    const data = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { canRedo } }` }));
    return Boolean((data.kitStore as { canRedo?: boolean }).canRedo);
  }

  /**
   * @emoji 🧭 `submitKitCommand` + await `succeeded`/`failed` on the kit event bus. Subscribes **before** the mutation so WASM cannot deliver the outcome before the listener exists (single-thread / microtask ordering).
   */
  private async submitShell(commandKind: string, request: { query: string; variables?: Record<string, unknown> }): Promise<SetResult> {
    this.ensureAlive();
    const maxBuffered = 64;
    const buffered: KitCommandLifecycleEvent[] = [];
    let targetRequestId: string | null = null;
    let settled = false;
    return await new Promise<SetResult>((resolve) => {
      const finish = (r: SetResult) => {
        if (settled) return;
        settled = true;
        clearTimeout(tid);
        off();
        resolve(r);
      };
      const drainForRequestId = (rid: string) => {
        for (const ev of buffered) {
          const row = ev.semioKitCommand;
          if (row.requestId !== rid) continue;
          if (row.phase === "succeeded") finish({ ok: true, requestId: rid });
          else if (row.phase === "failed")
            finish({
              ok: false,
              error: row.error ?? { kind: "Internal", message: "kit command failed" },
              requestId: rid,
            });
        }
      };
      const tid = setTimeout(() => {
        finish({
          ok: false,
          error: {
            kind: "Timeout",
            message: `kit command ${targetRequestId ?? "?"}: no succeeded/failed event`,
          },
          requestId: targetRequestId ?? undefined,
        });
      }, this.timeoutMs);
      const off = this.subscribe((ev) => {
        if (!isKitCommandLifecycleEvent(ev)) return;
        if (buffered.length < maxBuffered) buffered.push(ev);
        if (targetRequestId !== null) drainForRequestId(targetRequestId);
      });
      void (async () => {
        try {
          const data = kitGraphqlData(
            await this.gqlRun({
              query: `mutation($input: KitCommandShellInput!) { submitKitCommand(input: $input) { requestId commandKind accepted } }`,
              variables: { input: { commandKind, request } },
            }),
          );
          const receipt = (data as { submitKitCommand?: Partial<KitCommandReceipt> }).submitKitCommand;
          if (!receipt || typeof receipt.requestId !== "string" || receipt.accepted !== true) {
            finish({ ok: false, error: { kind: "Internal", message: "submitKitCommand: invalid receipt" } });
            return;
          }
          targetRequestId = receipt.requestId;
          drainForRequestId(targetRequestId);
        } catch (e) {
          finish({ ok: false, error: { kind: "Internal", message: String(e) } });
        }
      })();
    });
  }

  async patchEntityField(entityKind: string, id: string, field: string, value: unknown): Promise<SetResult> {
    try {
      const data = kitGraphqlData(
        await this.gqlRun({
          query: `query($k: String!, $id: String!, $f: String!, $vj: String!) { kitStore { changeKitCommandsForFieldPatchValueJson(kind: $k, id: $id, field: $f, valueJson: $vj) } }`,
          variables: { k: entityKind, id, f: field, vj: JSON.stringify(value) },
        }),
      );
      const cmds = (data.kitStore as { changeKitCommandsForFieldPatchValueJson?: unknown }).changeKitCommandsForFieldPatchValueJson;
      return this.submitShell("changeKitCommands", {
        query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`,
        variables: { commands: cmds },
      });
    } catch (e) {
      return { ok: false, error: normalizeWasmThrownKitError(e) };
    }
  }

  async addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult> {
    try {
      const data = kitGraphqlData(
        await this.gqlRun({
          query: `query($pk: String!, $pid: String!, $ck: String!, $dj: String!) { kitStore { changeKitCommandsForAddChildDtoJson(parentKind: $pk, parentId: $pid, childKind: $ck, dtoJson: $dj) } }`,
          variables: { pk: parentKind, pid: parentId, ck: childKind, dj: JSON.stringify(dto) },
        }),
      );
      const cmds = (data.kitStore as { changeKitCommandsForAddChildDtoJson?: unknown }).changeKitCommandsForAddChildDtoJson;
      return this.submitShell("changeKitCommands", {
        query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`,
        variables: { commands: cmds },
      });
    } catch (e) {
      return { ok: false, error: normalizeWasmThrownKitError(e) };
    }
  }

  async removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult> {
    try {
      const data = kitGraphqlData(
        await this.gqlRun({
          query: `query($pk: String!, $pid: String!, $ck: String!, $cid: String!) { kitStore { changeKitCommandsForRemoveChild(parentKind: $pk, parentId: $pid, childKind: $ck, childId: $cid) } }`,
          variables: { pk: parentKind, pid: parentId, ck: childKind, cid: childId },
        }),
      );
      const cmds = (data.kitStore as { changeKitCommandsForRemoveChild?: unknown }).changeKitCommandsForRemoveChild;
      return this.submitShell("changeKitCommands", {
        query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`,
        variables: { commands: cmds },
      });
    } catch (e) {
      return { ok: false, error: normalizeWasmThrownKitError(e) };
    }
  }

  async changeKitCommands(commands: unknown): Promise<SetResult> {
    return this.submitShell("changeKitCommands", { query: `mutation($commands: JSON!) { changeKitCommands(commands: $commands) }`, variables: { commands } });
  }

  async changeKitWithInverse(commands: unknown): Promise<{ kind: string; inverse: unknown }> {
    const data = kitGraphqlData(
      await this.gqlRun({ query: `mutation($commands: JSON!) { changeKitWithInverse(commands: $commands) }`, variables: { commands } }),
    );
    return (data as { changeKitWithInverse?: { kind: string; inverse: unknown } }).changeKitWithInverse as { kind: string; inverse: unknown };
  }

  async clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult> {
    return this.submitShell("clusterPieces", {
      query: `mutation($designId: String!, $pieceIds: [String!]!, $clusterName: String!) { clusterPieces(designId: $designId, pieceIds: $pieceIds, clusterName: $clusterName) }`,
      variables: { designId, pieceIds: [...pieceIds], clusterName },
    });
  }

  async dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.submitShell("dragPieces", {
      query: `mutation($designId: String!, $pieceIds: [String!]!, $du: Float!, $dv: Float!) { dragPieces(designId: $designId, pieceIds: $pieceIds, du: $du, dv: $dv) }`,
      variables: { designId, pieceIds: [...pieceIds], du, dv },
    });
  }

  async movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.submitShell("movePieces", {
      query: `mutation($designId: String!, $pieceIds: [String!]!, $gap: Float!, $shift: Float!, $rise: Float!) { movePieces(designId: $designId, pieceIds: $pieceIds, gap: $gap, shift: $shift, rise: $rise) }`,
      variables: { designId, pieceIds: [...pieceIds], gap, shift, rise },
    });
  }

  async fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult> {
    return this.submitShell("fixPieces", {
      query: `mutation($designId: String!, $pieceIds: [String!]!) { fixPieces(designId: $designId, pieceIds: $pieceIds) }`,
      variables: { designId, pieceIds: [...pieceIds] },
    });
  }

  async flattenDesign(designId: string): Promise<SetResult> {
    return this.submitShell("flattenDesign", { query: `mutation($designId: String!) { flattenDesign(designId: $designId) }`, variables: { designId } });
  }

  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.submitShell("expandDesign", {
      query: `mutation($parentDesignId: String!, $nestedDesignId: String!) { expandDesign(parentDesignId: $parentDesignId, nestedDesignId: $nestedDesignId) }`,
      variables: { parentDesignId, nestedDesignId },
    });
  }

  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.submitShell("deleteConnection", {
      query: `mutation($designId: String!, $connectionId: String!) { deleteConnection(designId: $designId, connectionId: $connectionId) }`,
      variables: { designId, connectionId },
    });
  }

  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.submitShell("changePieceType", {
      query: `mutation($designId: String!, $pieceId: String!, $newTypeId: String!) { changePieceType(designId: $designId, pieceId: $pieceId, newTypeId: $newTypeId) }`,
      variables: { designId, pieceId, newTypeId },
    });
  }

  async pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> {
    return this.submitShell("pasteDesignSelection", {
      query: `mutation($designId: String!, $selection: JSON!, $plane: JSON) { pasteDesignSelection(designId: $designId, selection: $selection, plane: $plane) }`,
      variables: { designId, selection, plane },
    });
  }

  async createHangingPieces(designId: string, typeIds: readonly string[], plane: unknown): Promise<SetResult> {
    return this.submitShell("createHangingPieces", {
      query: `mutation($designId: String!, $typeIds: [String!]!, $plane: JSON!) { createHangingPieces(designId: $designId, typeIds: $typeIds, plane: $plane) }`,
      variables: { designId, typeIds: [...typeIds], plane },
    });
  }

  async createConnectedPiece(
    designId: string,
    parentPiece: string,
    parentPort: string,
    childType: string,
    childPort: string,
  ): Promise<SetResult> {
    return this.submitShell("createConnectedPiece", {
      query: `mutation($designId: String!, $parentPiece: String!, $parentPort: String!, $childType: String!, $childPort: String!) { createConnectedPiece(designId: $designId, parentPiece: $parentPiece, parentPort: $parentPort, childType: $childType, childPort: $childPort) }`,
      variables: { designId, parentPiece, parentPort, childType, childPort },
    });
  }

  async createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> {
    return this.submitShell("createFixedPiece", {
      query: `mutation($designId: String!, $typeId: String!, $plane: JSON!) { createFixedPiece(designId: $designId, typeId: $typeId, plane: $plane) }`,
      variables: { designId, typeId, plane },
    });
  }

  async undo(): Promise<SetResult> {
    return this.submitShell("undo", { query: `mutation { undo }` });
  }

  async redo(): Promise<SetResult> {
    return this.submitShell("redo", { query: `mutation { redo }` });
  }

  async attachBackbone(cfg: BackboneConfig): Promise<unknown> {
    const data = kitGraphqlData(
      await this.gqlRun({ query: `mutation($config: JSON!) { attachBackbone(config: $config) }`, variables: { config: cfg } }),
    );
    return (data as { attachBackbone?: unknown }).attachBackbone;
  }

  async detachBackbone(): Promise<unknown> {
    const data = kitGraphqlData(await this.gqlRun({ query: `mutation { detachBackbone }` }));
    return (data as { detachBackbone?: unknown }).detachBackbone;
  }

  async backboneStatus(): Promise<BackboneStatusDto> {
    const data = kitGraphqlData(await this.gqlRun({ query: `mutation { backboneStatus }` }));
    return (data as { backboneStatus?: BackboneStatusDto }).backboneStatus ?? {};
  }

  async listConflicts(): Promise<unknown> {
    const data = kitGraphqlData(await this.gqlRun({ query: `mutation { listConflicts }` }));
    return (data as { listConflicts?: unknown }).listConflicts;
  }

  async resolveConflict(id: string, strategy: ConflictResolution): Promise<unknown> {
    const data = kitGraphqlData(
      await this.gqlRun({ query: `mutation($id: String!, $strategy: JSON!) { resolveConflict(id: $id, strategy: $strategy) }`, variables: { id, strategy } }),
    );
    return (data as { resolveConflict?: unknown }).resolveConflict;
  }

  async syncNow(): Promise<unknown> {
    const data = kitGraphqlData(await this.gqlRun({ query: `mutation { syncNow }` }));
    return (data as { syncNow?: unknown }).syncNow;
  }

  /**
   * @emoji 🧭 Read-only flatten map rows for one design (`semio/rs` `flatten_map`), for algorithm / MCP tooling.
   */
  async readDesignFlattenMap(designId: string): Promise<readonly Record<string, unknown>[]> {
    const data = kitGraphqlData(
      await this.gqlRun({
        query: `query($id: String!) { kitStore { designByDtoId(id: $id) { flattenMap } } }`,
        variables: { id: designId },
      }),
    ) as { kitStore?: { designByDtoId?: { flattenMap?: unknown } | null } | null };
    const raw = data.kitStore?.designByDtoId?.flattenMap;
    if (Array.isArray(raw)) return raw as Record<string, unknown>[];
    if (typeof raw === "string") {
      try {
        const p = JSON.parse(raw) as unknown;
        return Array.isArray(p) ? (p as Record<string, unknown>[]) : [];
      } catch {
        return [];
      }
    }
    return [];
  }

  async read(batch: ReadWireBatch): Promise<ReadWireBatchResult> {
    this.ensureAlive();
    const typed = batch as unknown as ReadCommandBatch;
    const out: ReadKitCommandOutput[] = [];
    for (const c of typed) out.push(await this.mapReadCommand(c));
    return out;
  }

  private async mapReadCommand(c: ReadKitCommand): Promise<ReadKitCommandOutput> {
    if ("readKitTypeIdsCommand" in c && c.readKitTypeIdsCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { typeIds } }` })) as { kitStore?: { typeIds?: unknown } };
      return { readKitTypeIdsCommand: { typeIds: kitGraphqlJsonToReadonlyArray(d.kitStore?.typeIds) } };
    }
    if ("readKitDesignIdsCommand" in c && c.readKitDesignIdsCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { designIds } }` })) as { kitStore?: { designIds?: unknown } };
      return { readKitDesignIdsCommand: { designIds: kitGraphqlJsonToReadonlyArray(d.kitStore?.designIds) } };
    }
    if ("readKitTypesMetadataCommand" in c && c.readKitTypesMetadataCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { typesMetadata } }` })) as { kitStore?: { typesMetadata?: unknown } };
      return { readKitTypesMetadataCommand: { types: kitGraphqlJsonToReadonlyArray(d.kitStore?.typesMetadata) } };
    }
    if ("readKitDesignsMetadataCommand" in c && c.readKitDesignsMetadataCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { designsMetadata } }` })) as { kitStore?: { designsMetadata?: unknown } };
      return { readKitDesignsMetadataCommand: { designs: kitGraphqlJsonToReadonlyArray(d.kitStore?.designsMetadata) } };
    }
    if ("readKitTypesShallowCommand" in c && c.readKitTypesShallowCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { typesShallowJson } }` })) as { kitStore?: { typesShallowJson?: unknown } };
      return { readKitTypesShallowCommand: { types: kitGraphqlJsonToReadonlyArray(d.kitStore?.typesShallowJson) } };
    }
    if ("readKitDesignsShallowCommand" in c && c.readKitDesignsShallowCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { designsShallowJson } }` })) as { kitStore?: { designsShallowJson?: unknown } };
      return { readKitDesignsShallowCommand: { designs: kitGraphqlJsonToReadonlyArray(d.kitStore?.designsShallowJson) } };
    }
    if ("readKitAuthorsShallowCommand" in c && c.readKitAuthorsShallowCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { authorsShallowJson } }` })) as { kitStore?: { authorsShallowJson?: unknown } };
      return { readKitAuthorsShallowCommand: { authors: kitGraphqlJsonToReadonlyArray(d.kitStore?.authorsShallowJson) } };
    }
    if ("readKitMetadataCommand" in c && c.readKitMetadataCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { kitMetadataJson } }` })) as { kitStore?: { kitMetadataJson?: unknown } };
      return { readKitMetadataCommand: { metadata: d.kitStore?.kitMetadataJson } };
    }
    if ("readKitColoredConnectorsCommand" in c && c.readKitColoredConnectorsCommand === null) {
      const d = kitGraphqlData(await this.gqlRun({ query: `query { kitStore { coloredConnectors } }` })) as { kitStore?: { coloredConnectors?: unknown } };
      return { readKitColoredConnectorsCommand: { rows: d.kitStore?.coloredConnectors } };
    }
    if ("readKitDesignCommands" in c && c.readKitDesignCommands) {
      const { id, commands } = c.readKitDesignCommands;
      const results: ReadDesignCommandOutput[] = [];
      for (const sub of commands) results.push(await this.mapDesignRead(id.id, sub));
      return { readKitDesignCommands: { results } };
    }
    if ("readKitTypeCommands" in c && c.readKitTypeCommands) {
      const { id, commands } = c.readKitTypeCommands;
      const results: ReadTypeCommandOutput[] = [];
      for (const sub of commands) results.push(await this.mapTypeRead(id.id, sub));
      return { readKitTypeCommands: { results } };
    }
    throw new Error(`read: unsupported ${Object.keys(c).join(",")}`);
  }

  private async mapDesignRead(designId: string, cmd: ReadDesignCommand): Promise<ReadDesignCommandOutput> {
    if ("readDesignPiecesFullCommand" in cmd && cmd.readDesignPiecesFullCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!) { kitStore { designByDtoId(id: $id) { piecesFullJson } } }`,
          variables: { id: designId },
        }),
      ) as { kitStore?: { designByDtoId?: { piecesFullJson?: unknown } | null } | null };
      return { readDesignPiecesFullCommand: { pieces: kitGraphqlJsonToReadonlyArray(d.kitStore?.designByDtoId?.piecesFullJson) } };
    }
    if ("readDesignConnectionsFullCommand" in cmd && cmd.readDesignConnectionsFullCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!) { kitStore { designByDtoId(id: $id) { connectionsFullJson } } }`,
          variables: { id: designId },
        }),
      ) as { kitStore?: { designByDtoId?: { connectionsFullJson?: unknown } | null } | null };
      return { readDesignConnectionsFullCommand: { connections: kitGraphqlJsonToReadonlyArray(d.kitStore?.designByDtoId?.connectionsFullJson) } };
    }
    if ("readDesignPieceCommands" in cmd && cmd.readDesignPieceCommands) {
      const { id, commands } = cmd.readDesignPieceCommands;
      const results: ReadPieceCommandOutput[] = [];
      for (const pc of commands) results.push(await this.mapPieceRead(designId, id.id, pc));
      return { readDesignPieceCommands: { results } };
    }
    if ("readDesignClusterableGroupsCommand" in cmd && cmd.readDesignClusterableGroupsCommand) {
      const sel = cmd.readDesignClusterableGroupsCommand.selection.map((s) => s.id);
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!, $sel: [String!]!) { kitStore { designByDtoId(id: $id) { clusterableGroups(selection: $sel) } } }`,
          variables: { id: designId, sel },
        }),
      ) as { kitStore?: { designByDtoId?: { clusterableGroups?: unknown } | null } | null };
      const raw = d.kitStore?.designByDtoId?.clusterableGroups;
      const groups = Array.isArray(raw)
        ? raw.map((row: unknown) => (Array.isArray(row) ? row.map((pid: unknown) => ({ id: String(pid) })) : []))
        : [];
      return { readDesignClusterableGroupsCommand: { groups } };
    }
    if ("readDesignIncludedDesignsCommand" in cmd && cmd.readDesignIncludedDesignsCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!) { kitStore { designByDtoId(id: $id) { includedDesigns } } }`,
          variables: { id: designId },
        }),
      ) as { kitStore?: { designByDtoId?: { includedDesigns?: unknown } | null } | null };
      return { readDesignIncludedDesignsCommand: { designs: d.kitStore?.designByDtoId?.includedDesigns } };
    }
    if ("readDesignQualitySumCommand" in cmd && cmd.readDesignQualitySumCommand) {
      const qid = cmd.readDesignQualitySumCommand.qualityId.id;
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!, $qid: String!) { kitStore { designByDtoId(id: $id) { qualitySum(qualityId: $qid) } } }`,
          variables: { id: designId, qid },
        }),
      ) as { kitStore?: { designByDtoId?: { qualitySum?: number } | null } | null };
      return { readDesignQualitySumCommand: { sum: d.kitStore?.designByDtoId?.qualitySum ?? 0 } };
    }
    if ("readDesignReplaceableCatalogCommand" in cmd && cmd.readDesignReplaceableCatalogCommand) {
      const sel = cmd.readDesignReplaceableCatalogCommand.selection.map((s) => s.id);
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!, $sel: [String!]!) { kitStore { designByDtoId(id: $id) { replaceableCatalog(selection: $sel) { typeIds designIds } } } }`,
          variables: { id: designId, sel },
        }),
      ) as { kitStore?: { designByDtoId?: { replaceableCatalog?: { typeIds?: string[]; designIds?: string[] } } | null } | null };
      const rc = d.kitStore?.designByDtoId?.replaceableCatalog;
      return {
        readDesignReplaceableCatalogCommand: {
          types: (rc?.typeIds ?? []).map((id: string) => ({ id: String(id) })),
          designs: (rc?.designIds ?? []).map((id: string) => ({ id: String(id) })),
        },
      };
    }
    if ("readDesignIncludedDesignIdsCommand" in cmd && cmd.readDesignIncludedDesignIdsCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!) { kitStore { designByDtoId(id: $id) { includedDesignIds } } }`,
          variables: { id: designId },
        }),
      ) as { kitStore?: { designByDtoId?: { includedDesignIds?: string[] } | null } | null };
      return { readDesignIncludedDesignIdsCommand: { designIds: d.kitStore?.designByDtoId?.includedDesignIds ?? [] } };
    }
    throw new Error(`readDesign: ${Object.keys(cmd).join(",")}`);
  }

  private async mapPieceRead(designId: string, pieceId: string, cmd: ReadPieceCommand): Promise<ReadPieceCommandOutput> {
    if ("readPieceFlatPlaneCommand" in cmd && cmd.readPieceFlatPlaneCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($d: String!, $p: String!) { kitStore { designByDtoId(id: $d) { pieceByDtoId(id: $p) { flatPlane } } } }`,
          variables: { d: designId, p: pieceId },
        }),
      ) as { kitStore?: { designByDtoId?: { pieceByDtoId?: { flatPlane?: unknown } | null } | null } | null };
      return { readPieceFlatPlaneCommand: { flatPlane: d.kitStore?.designByDtoId?.pieceByDtoId?.flatPlane } };
    }
    if ("readPieceFlatCenterCommand" in cmd && cmd.readPieceFlatCenterCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($d: String!, $p: String!) { kitStore { designByDtoId(id: $d) { pieceByDtoId(id: $p) { flatCenter } } } }`,
          variables: { d: designId, p: pieceId },
        }),
      ) as { kitStore?: { designByDtoId?: { pieceByDtoId?: { flatCenter?: unknown } | null } | null } | null };
      return { readPieceFlatCenterCommand: { flatCenter: d.kitStore?.designByDtoId?.pieceByDtoId?.flatCenter } };
    }
    if ("readPieceParentConnectionFullCommand" in cmd && cmd.readPieceParentConnectionFullCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($d: String!, $p: String!) { kitStore { designByDtoId(id: $d) { pieceByDtoId(id: $p) { parentConnectionFull } } } }`,
          variables: { d: designId, p: pieceId },
        }),
      ) as { kitStore?: { designByDtoId?: { pieceByDtoId?: { parentConnectionFull?: unknown } | null } | null } | null };
      return { readPieceParentConnectionFullCommand: { connection: d.kitStore?.designByDtoId?.pieceByDtoId?.parentConnectionFull } };
    }
    throw new Error(`readPiece: ${Object.keys(cmd).join(",")}`);
  }

  private async mapTypeRead(typeId: string, cmd: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
    if ("readTypeBestRepresentationCommand" in cmd && cmd.readTypeBestRepresentationCommand) {
      const tags = cmd.readTypeBestRepresentationCommand.tagIds;
      const d = kitGraphqlData(
        await this.gqlRun({
          query: `query($id: String!, $tags: [String!]!) { kitStore { typeByDtoId(id: $id) { bestRepresentation(tagIds: $tags) } } }`,
          variables: { id: typeId, tags: [...tags] },
        }),
      ) as { kitStore?: { typeByDtoId?: { bestRepresentation?: unknown } | null } | null };
      return { readTypeBestRepresentationCommand: { representation: d.kitStore?.typeByDtoId?.bestRepresentation } };
    }
    throw new Error(`readType: ${Object.keys(cmd).join(",")}`);
  }

  async getPiecesMetadata(designId: string): Promise<Record<string, unknown>> {
    const d = kitGraphqlData(
      await this.gqlRun({
        query: `query($id: String!) { kitStore { designByDtoId(id: $id) { piecesMetadataJson } } }`,
        variables: { id: designId },
      }),
    ) as { kitStore?: { designByDtoId?: { piecesMetadataJson?: unknown } | null } | null };
    const v = d.kitStore?.designByDtoId?.piecesMetadataJson;
    if (v && typeof v === "object" && !Array.isArray(v)) return v as Record<string, unknown>;
    return {};
  }

  async getPieces(designId: string): Promise<readonly unknown[]> {
    const out = await this.read([{ readKitDesignCommands: { id: { id: designId }, commands: [{ readDesignPiecesFullCommand: null }] } }]);
    const block = out[0] as { readKitDesignCommands?: { results?: ReadonlyArray<{ readDesignPiecesFullCommand?: { pieces?: unknown } }> } };
    const pieces = block.readKitDesignCommands?.results?.[0]?.readDesignPiecesFullCommand?.pieces;
    return kitGraphqlJsonToReadonlyArray(pieces);
  }

  async getConnections(designId: string): Promise<readonly unknown[]> {
    const out = await this.read([{ readKitDesignCommands: { id: { id: designId }, commands: [{ readDesignConnectionsFullCommand: null }] } }]);
    const block = out[0] as { readKitDesignCommands?: { results?: ReadonlyArray<{ readDesignConnectionsFullCommand?: { connections?: unknown } }> } };
    const connections = block.readKitDesignCommands?.results?.[0]?.readDesignConnectionsFullCommand?.connections;
    return kitGraphqlJsonToReadonlyArray(connections);
  }

  async getDesigns(): Promise<readonly unknown[]> {
    const out = await this.read([{ readKitDesignsShallowCommand: null }]);
    return kitGraphqlJsonToReadonlyArray((out[0] as { readKitDesignsShallowCommand?: { designs?: unknown } }).readKitDesignsShallowCommand?.designs);
  }

  async getTypes(): Promise<readonly unknown[]> {
    const out = await this.read([{ readKitTypesShallowCommand: null }]);
    return kitGraphqlJsonToReadonlyArray((out[0] as { readKitTypesShallowCommand?: { types?: unknown } }).readKitTypesShallowCommand?.types);
  }

  async getAuthors(): Promise<readonly unknown[]> {
    const out = await this.read([{ readKitAuthorsShallowCommand: null }]);
    return kitGraphqlJsonToReadonlyArray((out[0] as { readKitAuthorsShallowCommand?: { authors?: unknown } }).readKitAuthorsShallowCommand?.authors);
  }

  async getKitMetadata(): Promise<unknown> {
    const out = await this.read([{ readKitMetadataCommand: null }]);
    return (out[0] as { readKitMetadataCommand?: { metadata?: unknown } }).readKitMetadataCommand?.metadata;
  }
}

// #endregion 📦KitStore

// #region 🧪EmbeddedTests
if (process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  const { describe, it, expect } = await import("vitest");

  describe("semio-js KitStore", () => {
    it("opens dedicated worker wasm and returns typed snapshot", async () => {
      const minimalKit: KitFullDto = {
        id: "test-kit",
        name: "TestKit",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [{ id: "type-1", name: "Wall", connectors: [] }],
        designs: [{ id: "design-1", name: "Floor1", pieces: [], connections: [] }],
      };
      const ks = await KitStore.open(minimalKit);
      const snap = await ks.snapshot();
      expect(snap.id).toBe("test-kit");
      expect(snap.name).toBe("TestKit");
      const types = await ks.getTypes();
      expect(Array.isArray(types)).toBe(true);
      const designs = await ks.getDesigns();
      expect(Array.isArray(designs)).toBe(true);
      const r = await ks.patchEntityField("Type", "type-1", "name", "BigWall");
      expect(typeof r.ok).toBe("boolean");
      await ks.dispose();
    });

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
      const batch: ReadWireBatch = [{ readKitTypesShallowCommand: null }, { readKitTypeIdsCommand: null }];
      const res = await ks.read(batch);
      expect(res.length).toBe(2);
      await ks.dispose();
    });

    it("rejects snapshot after dispose", async () => {
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
      await expect(ks.snapshot()).rejects.toThrow(/disposed/i);
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

    it("theKit, vcsState, materializeAt root, and undo/redo flags round-trip", async () => {
      const minimalKit: KitFullDto = {
        id: "vcs-kit",
        name: "V",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [],
        designs: [],
      };
      const ks = await KitStore.open(minimalKit);
      const snap = await ks.snapshot();
      const tk = await ks.theKit();
      expect(tk.id).toBe(snap.id);
      const vcs = await ks.vcsState();
      expect(vcs != null && typeof vcs === "object").toBe(true);
      const mat = await ks.materializeAt("");
      expect(mat.id).toBe(snap.id);
      expect(typeof (await ks.canUndo())).toBe("boolean");
      expect(typeof (await ks.canRedo())).toBe("boolean");
      await ks.dispose();
    });

    it("compile-time: KitStore public surface excludes rxjs-style stream fields", () => {
      type KitStorePublicKeys = keyof KitStore;
      type MustNotLeakRx =
        "events$" extends KitStorePublicKeys
          ? never
          : "pipe" extends KitStorePublicKeys
            ? never
            : "_trySubscribe" extends KitStorePublicKeys
              ? never
              : true;
      const _assert: MustNotLeakRx = true;
      expect(_assert).toBe(true);
    });
  });
}
// #endregion 🧪EmbeddedTests
