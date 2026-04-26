// #region 🧲Header
// WASM `KitStore` bridge for `@semio/react`: `KitStoreClient`, `LiveKitRoot`, read hubs, command facade.
// 2026 Ueli Saluz <ueli@semio-tech.com> — GNU LGPL-3.0 or later
// #endregion 🧲Header

// #region 📥Imports
import {
  KitStore as SemioWasmKitStore,
  type BackboneConfig,
  type BackboneStatusDto,
  type ConflictResolution,
  type ReadDesignCommand,
  type ReadKitCommand,
  type ReadPieceCommand,
  type SetError,
  type SetResult,
} from "@semio/js";
import { Subscription } from "rxjs";
import type { Kit, KitHostStore } from "./kitEntities";
import { applyKitClientSnapshotToLocalStore, type SemioKitBridge } from "./kitEntities";

export { applyKitClientSnapshotToLocalStore } from "./kitEntities";
// #endregion 📥Imports

// #region 🔌KitStoreClientTypes

export type KitStoreExecuteResult = { ok: true; result: unknown } | { ok: false; error: SetError };

export type WriteStatus =
  | { kind: "readonly"; pending: 0; lastError?: SetError }
  | { kind: "idle"; pending: 0; lastError?: SetError }
  | { kind: "pending"; pending: number }
  | { kind: "error"; pending: 0; lastError?: SetError };

/** @emoji 🧾 Sketchpad string-command context/result (opaque JSON). */
export type KitCommandContext = Record<string, unknown>;
export type KitCommandResult = Record<string, unknown>;

export type KitTypedShellCommand = {
  readonly kind: "setEntityField";
  readonly entityKind: string;
  readonly id: string;
  readonly field: string;
  readonly value: unknown;
};

export type KitCommandFacade = { runMutation(cmd: KitTypedShellCommand): Promise<SetResult> };

export type KitStoreReadSnap = { readonly version: number; readonly data: unknown; readonly pending: number };

export type KitDesignReadKind = "metadata" | "pieces" | "connections";
export type KitShallowListKind = "designs" | "types" | "authors";
export type KitViewCatalogKey = "typeIds" | "typesMetadata" | "designIds" | "designsMetadata";

/** @emoji 🧾 Browser / test kit RPC surface used by React hooks (wraps {@link SemioWasmKitStore}). */
export type KitStoreClient = SemioKitBridge & {
  clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult>;
  dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult>;
  movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult>;
  fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult>;
  flattenDesign(designId: string): Promise<SetResult>;
  expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult>;
  deleteConnection(designId: string, connectionId: string): Promise<SetResult>;
  changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult>;
  pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult>;
  createHangingPieces(designId: string, typeIds: readonly string[], plane: unknown): Promise<SetResult>;
  createConnectedPiece(
    designId: string,
    parentPiece: string,
    parentPort: string,
    childType: string,
    childPort: string,
  ): Promise<SetResult>;
  createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult>;
  patchEntityField(entityKind: string, id: string, field: string, value: unknown): Promise<SetResult>;
  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult>;
  removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult>;
  undo(): Promise<SetResult>;
  redo(): Promise<SetResult>;
  canUndo(): Promise<boolean>;
  canRedo(): Promise<boolean>;
  getPiecesMetadata(designId: string): Promise<Record<string, unknown>>;
  getPieces(designId: string): Promise<readonly unknown[]>;
  getConnections(designId: string): Promise<readonly unknown[]>;
  getDesigns(): Promise<readonly unknown[]>;
  getTypes(): Promise<readonly unknown[]>;
  getAuthors(): Promise<readonly unknown[]>;
  getKitMetadata(): Promise<unknown>;
  backboneStatus(): Promise<BackboneStatusDto>;
  attachBackbone(cfg: BackboneConfig): Promise<unknown>;
  detachBackbone(): Promise<unknown>;
  listConflicts(): Promise<unknown>;
  resolveConflict(id: string, strategy: ConflictResolution): Promise<unknown>;
  syncNow(): Promise<unknown>;
  kitGraphql(): LiveKitRoot;
  subscribe(cb: (ev?: unknown) => void): () => void;
  dispose(): void;
};

// #endregion 🔌KitStoreClientTypes

// #region 🧰ReadHelpers

function firstDesignPieceResult(out: readonly unknown[], cmdKey: string): unknown {
  const row = out[0] as { readKitDesignCommands?: { results?: readonly unknown[] } };
  const r0 = row.readKitDesignCommands?.results?.[0] as Record<string, unknown> | undefined;
  if (!r0) return undefined;
  const inner = r0.readDesignPieceCommands as { results?: readonly unknown[] } | undefined;
  const p0 = inner?.results?.[0] as Record<string, unknown> | undefined;
  if (!p0) return undefined;
  const block = p0[cmdKey] as Record<string, unknown> | undefined;
  return block;
}

function firstDesignResult(out: readonly unknown[], cmdKey: string): unknown {
  const row = out[0] as { readKitDesignCommands?: { results?: readonly unknown[] } };
  const r0 = row.readKitDesignCommands?.results?.[0] as Record<string, unknown> | undefined;
  if (!r0) return undefined;
  return r0[cmdKey];
}

// #endregion 🧰ReadHelpers

// #region 📦LiveKitRoot

/** @emoji 🧭 Graph-shaped reads routed through {@link SemioWasmKitStore.read} (no legacy JS kit graph). */
export class LiveKitRoot {
  constructor(private readonly ks: SemioWasmKitStore) {}

  piece(designId: string, pieceId: string): LivePiece {
    return new LivePiece(this.ks, designId, pieceId);
  }

  design(designId: string): LiveDesign {
    return new LiveDesign(this.ks, designId);
  }

  type(typeId: string): LiveType {
    return new LiveType(this.ks, typeId);
  }

  readColoredConnectors(): Promise<readonly unknown[]> {
    return this.ks.read([{ readKitColoredConnectorsCommand: null }]).then((out) => {
      const rows = (out[0] as { readKitColoredConnectorsCommand?: { rows?: unknown } }).readKitColoredConnectorsCommand?.rows;
      return Array.isArray(rows) ? rows : [];
    });
  }
}

class LivePiece {
  constructor(
    private readonly ks: SemioWasmKitStore,
    private readonly designId: string,
    private readonly pieceId: string,
  ) {}

  private run(cmd: ReadPieceCommand): Promise<readonly unknown[]> {
    const batch: ReadKitCommand[] = [
      {
        readKitDesignCommands: {
          id: { id: this.designId },
          commands: [{ readDesignPieceCommands: { id: { id: this.pieceId }, commands: [cmd] } }],
        },
      },
    ];
    return this.ks.read(batch);
  }

  readFlatPlane(): Promise<unknown> {
    return this.run({ readPieceFlatPlaneCommand: null }).then((out) => (firstDesignPieceResult(out, "readPieceFlatPlaneCommand") as { flatPlane?: unknown } | undefined)?.flatPlane);
  }

  readFlatCenter(): Promise<unknown> {
    return this.run({ readPieceFlatCenterCommand: null }).then((out) => (firstDesignPieceResult(out, "readPieceFlatCenterCommand") as { flatCenter?: unknown } | undefined)?.flatCenter);
  }

  readParentConnectionFull(): Promise<unknown> {
    return this.run({ readPieceParentConnectionFullCommand: null }).then(
      (out) => (firstDesignPieceResult(out, "readPieceParentConnectionFullCommand") as { connection?: unknown } | undefined)?.connection,
    );
  }
}

class LiveDesign {
  constructor(
    private readonly ks: SemioWasmKitStore,
    private readonly designId: string,
  ) {}

  private run(cmd: ReadDesignCommand): Promise<readonly unknown[]> {
    return this.ks.read([{ readKitDesignCommands: { id: { id: this.designId }, commands: [cmd] } }]);
  }

  readIncludedDesigns(): Promise<unknown> {
    return this.run({ readDesignIncludedDesignsCommand: null }).then(
      (out) => (firstDesignResult(out, "readDesignIncludedDesignsCommand") as { designs?: unknown } | undefined)?.designs,
    );
  }

  readClusterableGroups(selection: readonly string[]): Promise<unknown> {
    const cmd: ReadDesignCommand = {
      readDesignClusterableGroupsCommand: { selection: selection.map((id) => ({ id })) },
    };
    return this.run(cmd).then((out) => (firstDesignResult(out, "readDesignClusterableGroupsCommand") as { groups?: unknown } | undefined)?.groups);
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
      const blk = firstDesignResult(out, "readDesignReplaceableCatalogCommand") as
        | { types?: readonly unknown[]; designs?: readonly unknown[] }
        | undefined;
      const toIds = (xs: readonly unknown[] | undefined) =>
        (xs ?? []).map((x) => (typeof x === "string" ? x : (x as { id?: string })?.id)).filter((x): x is string => typeof x === "string");
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
    private readonly ks: SemioWasmKitStore,
    private readonly typeId: string,
  ) {}

  readBestRepresentation(tagIds: readonly string[]): Promise<unknown> {
    return this.ks
      .read([
        {
          readKitTypeCommands: {
            id: { id: this.typeId },
            commands: [{ readTypeBestRepresentationCommand: { tagIds: [...tagIds] } }],
          },
        },
      ])
      .then((out) => {
        const row = out[0] as { readKitTypeCommands?: { results?: readonly unknown[] } };
        const r0 = row.readKitTypeCommands?.results?.[0] as Record<string, unknown> | undefined;
        const rep = r0?.readTypeBestRepresentationCommand as { representation?: unknown } | undefined;
        return rep?.representation;
      });
  }
}

// #endregion 📦LiveKitRoot

// #region 🪜LiveReadHub

export class SemioKitLiveReadStore {
  private readonly snap = new Map<string, KitStoreReadSnap>();
  private readonly regs: Array<{
    key: string;
    fetch: () => Promise<unknown>;
    affects: (ev: unknown) => boolean;
    onChange: () => void;
  }> = [];
  private off: (() => void) | undefined;

  constructor(private readonly client: KitStoreClient) {
    this.off = client.subscribe((ev) => {
      for (const r of this.regs) {
        if (r.affects(ev)) void this.poll(r);
      }
    });
  }

  subscribe(key: string, fetch: () => Promise<unknown>, affects: (ev: unknown) => boolean, onChange: () => void): () => void {
    const r = { key, fetch, affects, onChange };
    this.regs.push(r);
    void this.poll(r);
    return () => {
      this.regs.splice(this.regs.indexOf(r), 1);
    };
  }

  getSnapshot(key: string): KitStoreReadSnap {
    return this.snap.get(key) ?? { version: 0, data: [], pending: 0 };
  }

  private async poll(r: { key: string; fetch: () => Promise<unknown>; onChange: () => void }): Promise<void> {
    const cur = this.snap.get(r.key) ?? { version: 0, data: [], pending: 0 };
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

// #endregion 🪜LiveReadHub

// #region 🪜KitViewStores

export class SemioKitViewStore {
  constructor(private readonly client: KitStoreClient) {}

  subscribe(_key: KitViewCatalogKey, onChange: () => void): () => void {
    return this.client.subscribe(() => onChange());
  }

  getSnapshot(key: KitViewCatalogKey): unknown {
    const dto = (this.client as WasmKitStoreClient).getDto() as {
      types?: readonly { id?: string; name?: string }[];
      designs?: readonly { id?: string; name?: string }[];
    };
    if (key === "typeIds") return (dto.types ?? []).map((t) => String(t.id ?? ""));
    if (key === "designIds") return (dto.designs ?? []).map((d) => String(d.id ?? ""));
    if (key === "typesMetadata") return (dto.types ?? []).map((t) => ({ id: t.id, name: t.name }));
    if (key === "designsMetadata") return (dto.designs ?? []).map((d) => ({ id: d.id, name: d.name }));
    return [];
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

export class SemioKitDesignReadStore {
  constructor(private readonly client: KitStoreClient) {}

  subscribe(_designId: string, _field: KitDesignReadKind, onChange: () => void): () => void {
    return this.client.subscribe(() => onChange());
  }

  getSnapshot(designId: string, field: KitDesignReadKind): KitStoreReadSnap {
    const dto = (this.client as WasmKitStoreClient).getDto() as {
      designs?: readonly { id?: string; pieces?: readonly unknown[]; connections?: readonly unknown[] }[];
    };
    const d = (dto.designs ?? []).find((x) => String(x.id) === String(designId));
    if (!d) return { version: 0, data: field === "metadata" ? {} : [], pending: 0 };
    if (field === "pieces") return { version: 0, data: [...(d.pieces ?? [])], pending: 0 };
    if (field === "connections") return { version: 0, data: [...(d.connections ?? [])], pending: 0 };
    const meta: Record<string, unknown> = {};
    for (const p of d.pieces ?? []) {
      if (p && typeof p === "object" && "id" in (p as object)) meta[String((p as { id: string }).id)] = p;
    }
    return { version: 0, data: meta, pending: 0 };
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

export class SemioKitShallowListReadStore {
  constructor(private readonly client: KitStoreClient) {}

  subscribe(_kind: KitShallowListKind, onChange: () => void): () => void {
    return this.client.subscribe(() => onChange());
  }

  getSnapshot(kind: KitShallowListKind): KitStoreReadSnap {
    const dto = (this.client as WasmKitStoreClient).getDto() as {
      designs?: readonly unknown[];
      types?: readonly unknown[];
      authors?: readonly unknown[];
    };
    if (kind === "designs") return { version: 0, data: [...(dto.designs ?? [])], pending: 0 };
    if (kind === "types") return { version: 0, data: [...(dto.types ?? [])], pending: 0 };
    return { version: 0, data: [...(dto.authors ?? [])], pending: 0 };
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

// #endregion 🪜KitViewStores

// #region 🧰EventFilters

export const kitEventAffectsCanUndoRedo = (_ev?: unknown) => true;
export const kitEventAffectsPieceLiveRead = (_ev?: unknown, _d?: string, _p?: string) => true;
export const kitEventAffectsReplaceableCatalogRead = (_ev?: unknown, _d?: string, _s?: ReadonlySet<string>) => true;
export const kitEventAffectsDesignQualitySumRead = (_ev?: unknown, _d?: string, _q?: string) => true;
export const kitEventAffectsTypeScopedRead = (_ev?: unknown, _t?: string) => true;
export const kitEventAffectsKitColoredConnectorsRead = (_ev?: unknown) => true;

// #endregion 🧰EventFilters

// #region 📦WasmKitStoreClient

export class WasmKitStoreClient implements KitStoreClient {
  private readonly listeners = new Set<(ev?: unknown) => void>();
  private sub: Subscription | undefined;

  constructor(private readonly ks: SemioWasmKitStore) {
    this.sub = this.ks.events$.subscribe((ev) => {
      for (const l of this.listeners) l(ev);
    });
  }

  /** @internal For read-store adapters. */
  internalKs(): SemioWasmKitStore {
    return this.ks;
  }

  getDto(): Record<string, unknown> {
    return this.ks.getCachedKit() as Record<string, unknown>;
  }

  async getSnapshot(): Promise<Record<string, unknown>> {
    return (await this.ks.snapshot()) as Record<string, unknown>;
  }

  subscribe(cb: (ev?: unknown) => void): () => void {
    this.listeners.add(cb);
    return () => {
      this.listeners.delete(cb);
    };
  }

  dispose(): void {
    this.sub?.unsubscribe();
    this.sub = undefined;
    this.listeners.clear();
    void this.ks.dispose();
  }

  kitGraphql(): LiveKitRoot {
    return new LiveKitRoot(this.ks);
  }

  patchEntityField(entityKind: string, id: string, field: string, value: unknown): Promise<SetResult> {
    return this.ks.patchEntityField(entityKind, id, field, value);
  }

  addChild(parentKind: string, parentId: string, childKind: string, dto: unknown): Promise<SetResult> {
    return this.ks.addChild(parentKind, parentId, childKind, dto);
  }

  removeChild(parentKind: string, parentId: string, childKind: string, childId: string): Promise<SetResult> {
    return this.ks.removeChild(parentKind, parentId, childKind, childId);
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

  pasteDesignSelection(designId: string, selection: unknown, plane: unknown): Promise<SetResult> {
    return this.ks.pasteDesignSelection(designId, selection, plane);
  }

  createHangingPieces(designId: string, typeIds: readonly string[], plane: unknown): Promise<SetResult> {
    return this.ks.createHangingPieces(designId, typeIds, plane);
  }

  createConnectedPiece(
    designId: string,
    parentPiece: string,
    parentPort: string,
    childType: string,
    childPort: string,
  ): Promise<SetResult> {
    return this.ks.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort);
  }

  createFixedPiece(designId: string, typeId: string, plane: unknown): Promise<SetResult> {
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

  getPiecesMetadata(designId: string): Promise<Record<string, unknown>> {
    return this.ks.getPiecesMetadata(designId);
  }

  getPieces(designId: string): Promise<readonly unknown[]> {
    return this.ks.getPieces(designId);
  }

  getConnections(designId: string): Promise<readonly unknown[]> {
    return this.ks.getConnections(designId);
  }

  getDesigns(): Promise<readonly unknown[]> {
    return this.ks.getDesigns();
  }

  getTypes(): Promise<readonly unknown[]> {
    return this.ks.getTypes();
  }

  getAuthors(): Promise<readonly unknown[]> {
    return this.ks.getAuthors();
  }

  getKitMetadata(): Promise<unknown> {
    return this.ks.getKitMetadata();
  }

  backboneStatus(): Promise<BackboneStatusDto> {
    return this.ks.backboneStatus();
  }

  attachBackbone(cfg: BackboneConfig): Promise<unknown> {
    return this.ks.attachBackbone(cfg);
  }

  detachBackbone(): Promise<unknown> {
    return this.ks.detachBackbone();
  }

  listConflicts(): Promise<unknown> {
    return this.ks.listConflicts();
  }

  resolveConflict(id: string, strategy: ConflictResolution): Promise<unknown> {
    return this.ks.resolveConflict(id, strategy);
  }

  syncNow(): Promise<unknown> {
    return this.ks.syncNow();
  }
}

class FallbackKitClient implements KitStoreClient {
  private readonly listeners = new Set<(ev?: unknown) => void>();
  constructor(private readonly kit: Kit) {}

  getDto(): Record<string, unknown> {
    return this.kit.toJSON() as Record<string, unknown>;
  }

  async getSnapshot(): Promise<Record<string, unknown>> {
    return this.getDto();
  }

  subscribe(cb: (ev?: unknown) => void): () => void {
    this.listeners.add(cb);
    return () => {
      this.listeners.delete(cb);
    };
  }

  dispose(): void {
    this.listeners.clear();
  }

  kitGraphql(): LiveKitRoot {
    throw new Error("kitGraphql unavailable in fallback kit client");
  }

  private notify(): void {
    for (const l of this.listeners) l({});
  }

  async patchEntityField(entityKind: string, id: string, field: string, value: unknown): Promise<SetResult> {
    void entityKind;
    void id;
    void field;
    void value;
    this.notify();
    return { ok: true };
  }

  async addChild(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }

  async removeChild(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }

  async clusterPieces(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async dragPieces(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async movePieces(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async fixPieces(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async flattenDesign(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async expandDesign(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async deleteConnection(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async changePieceType(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async pasteDesignSelection(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async createHangingPieces(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async createConnectedPiece(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async createFixedPiece(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async undo(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async redo(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async canUndo(): Promise<boolean> {
    return false;
  }
  async canRedo(): Promise<boolean> {
    return false;
  }
  async getPiecesMetadata(): Promise<Record<string, unknown>> {
    return {};
  }
  async getPieces(): Promise<readonly unknown[]> {
    return [];
  }
  async getConnections(): Promise<readonly unknown[]> {
    return [];
  }
  async getDesigns(): Promise<readonly unknown[]> {
    return [];
  }
  async getTypes(): Promise<readonly unknown[]> {
    return [];
  }
  async getAuthors(): Promise<readonly unknown[]> {
    return [];
  }
  async getKitMetadata(): Promise<unknown> {
    return this.kit.toJSON();
  }
  async backboneStatus(): Promise<BackboneStatusDto> {
    return {};
  }
  async attachBackbone(): Promise<unknown> {
    return {};
  }
  async detachBackbone(): Promise<unknown> {
    return {};
  }
  async listConflicts(): Promise<unknown> {
    return [];
  }
  async resolveConflict(): Promise<unknown> {
    return {};
  }
  async syncNow(): Promise<unknown> {
    return {};
  }
}

export async function createKitStoreClient(opts: { initialKit: Kit; forceFallback?: boolean }): Promise<KitStoreClient> {
  if (opts.forceFallback) return new FallbackKitClient(opts.initialKit);
  const ks = await SemioWasmKitStore.open(opts.initialKit.toJSON() as never, {
    forceInline: typeof Worker === "undefined",
  });
  return new WasmKitStoreClient(ks);
}

const facades = new WeakMap<KitStoreClient, KitCommandFacade>();

export function acquireSemioKitCommandFacade(client: KitStoreClient): KitCommandFacade {
  let f = facades.get(client);
  if (!f) {
    f = {
      runMutation: async (cmd: KitTypedShellCommand): Promise<SetResult> => {
        if (cmd.kind !== "setEntityField") return { ok: false, error: { kind: "NotSupported", message: "command" } };
        return client.patchEntityField(cmd.entityKind, cmd.id, cmd.field, cmd.value);
      },
    };
    facades.set(client, f);
  }
  return f;
}

export function releaseSemioKitCommandFacade(client: KitStoreClient): void {
  facades.delete(client);
}

export function createKitCommandEngineExplicitOrigin(_store: KitHostStore): { execute: (...args: unknown[]) => Promise<unknown> } {
  return {
    execute: async (..._args: unknown[]) => ({ ok: false, error: "use KitStoreClient" }),
  };
}

export function createKitCommandEngine(store: KitHostStore): ReturnType<typeof createKitCommandEngineExplicitOrigin> {
  return createKitCommandEngineExplicitOrigin(store);
}

export async function executeSemioKitCommand(store: KitHostStore, command: string, _origin: string, ...args: unknown[]): Promise<unknown> {
  const bridge = (store as { __semioKitBridge?: KitStoreClient }).__semioKitBridge;
  if (!bridge) return { ok: false, error: "no kit bridge" };
  if (command === "semio.kit.undo") return bridge.undo();
  if (command === "semio.kit.redo") return bridge.redo();
  if (command === "semio.kit.addFile" && args[0] && args[1]) {
    return bridge.addChild("Kit", String((bridge.getDto() as { id?: string }).id ?? ""), "File", args[0]);
  }
  void args;
  return { ok: false, error: `unhandled ${command}` };
}

// #endregion 📦WasmKitStoreClient
