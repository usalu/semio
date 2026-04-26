// #region 🧲Header
// WASM `KitStore` bridge for `@semio/react` (subpath `@semio/js/kitWasmBridge`): `KitStoreClient`, `LiveKitRoot`, read hubs, command facade; kit DTO/entity graph merged from former `kitEntities.ts` (`🧩KitEntitiesMerged`).
// 2026 Ueli Saluz <ueli@semio-tech.com> — GNU LGPL-3.0 or later
// #endregion 🧲Header

// #region 📥Imports
import {
  KitStore as SemioWasmKitStore,
  type BackboneConfig,
  type BackboneStatusDto,
  type ConflictResolution,
  type KitFullDto as SemioKitWireDto,
  type ReadWireBatch,
  type SetError,
  type SetResult,
} from "./index.js";
import { z } from "zod";
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

  private run(cmd: Readonly<Record<string, unknown>>): Promise<ReadWireBatch> {
    const batch: ReadWireBatch = [
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

  private run(cmd: Readonly<Record<string, unknown>>): Promise<ReadWireBatch> {
    return this.ks.read([{ readKitDesignCommands: { id: { id: this.designId }, commands: [cmd] } }]);
  }

  readIncludedDesigns(): Promise<unknown> {
    return this.run({ readDesignIncludedDesignsCommand: null }).then(
      (out) => (firstDesignResult(out, "readDesignIncludedDesignsCommand") as { designs?: unknown } | undefined)?.designs,
    );
  }

  readClusterableGroups(selection: readonly string[]): Promise<unknown> {
    const cmd: Readonly<Record<string, unknown>> = {
      readDesignClusterableGroupsCommand: { selection: selection.map((id) => ({ id })) },
    };
    return this.run(cmd).then((out) => (firstDesignResult(out, "readDesignClusterableGroupsCommand") as { groups?: unknown } | undefined)?.groups);
  }

  readQualitySum(qualityId: string): Promise<number> {
    const cmd: Readonly<Record<string, unknown>> = { readDesignQualitySumCommand: { qualityId: { id: qualityId } } };
    return this.run(cmd).then((out) => {
      const s = (firstDesignResult(out, "readDesignQualitySumCommand") as { sum?: number } | undefined)?.sum;
      return typeof s === "number" && !Number.isNaN(s) ? s : 0;
    });
  }

  readReplaceableCatalog(selection: readonly string[]): Promise<{ types: string[]; designs: string[] }> {
    const cmd: Readonly<Record<string, unknown>> = {
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
  private readonly offKit: () => void;
  private lastDto: Record<string, unknown> = {};

  constructor(private readonly ks: SemioWasmKitStore) {
    this.offKit = this.ks.subscribe(() => {
      void this.refreshDtoFromStore();
      for (const l of this.listeners) l(undefined);
    });
  }

  private async refreshDtoFromStore(): Promise<void> {
    try {
      this.lastDto = (await this.ks.snapshot()) as Record<string, unknown>;
    } catch {
      /* ignore */
    }
  }

  /** @internal For read-store adapters. */
  internalKs(): SemioWasmKitStore {
    return this.ks;
  }

  getDto(): Record<string, unknown> {
    return this.lastDto;
  }

  async getSnapshot(): Promise<Record<string, unknown>> {
    const s = (await this.ks.snapshot()) as Record<string, unknown>;
    this.lastDto = s;
    return s;
  }

  subscribe(cb: (ev?: unknown) => void): () => void {
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
  constructor(private readonly kit: SemioKitWireDto) {}

  getDto(): Record<string, unknown> {
    return this.kit as Record<string, unknown>;
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
    return this.kit;
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

export async function createKitStoreClient(opts: { initialKit: SemioKitWireDto; forceFallback?: boolean }): Promise<KitStoreClient> {
  if (opts.forceFallback) return new FallbackKitClient(opts.initialKit);
  const ks = await SemioWasmKitStore.open(opts.initialKit);
  const c = new WasmKitStoreClient(ks);
  await c.getSnapshot();
  return c;
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

export type AttributeId = { id: Id };
export type LocationId = { id: Id };
export type AuthorId = { id: Id };
export type FileId = { id: Id };
export type FolderId = { id: Id };
export type BenchmarkId = { id: Id };
export type QualityId = { id: Id };
export type PortId = { id: Id };
export type PropId = { id: Id };
export type RepresentationId = { id: Id };
export type ConnectorId = { id: Id };
export type TypeId = { id: Id };
export type LayerId = { id: Id };
export type PieceId = { id: Id };
export type GroupId = { id: Id };
export type ConnectionId = { id: Id };
export type StatId = { id: Id };
export type DesignId = { id: Id };
export type KitId = { id: Id };
export type TagId = { id: Id };
export type ConceptId = { id: Id };
export type FamilyId = { id: Id };

export const AttributeIdSchema = z.object({ id: z.string() });
export const LocationIdSchema = z.object({ id: z.string() });
export const AuthorIdSchema = z.object({ id: z.string() });
export const FileIdSchema = z.object({ id: z.string() });
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
export type CoordinatePlain = z.infer<typeof CoordinateSchema>;
export class Coordinate implements CoordinatePlain {
  u!: number;
  v!: number;
  constructor(plain: CoordinatePlain) {
    Object.assign(this, CoordinateSchema.parse(plain));
  }
  static from(plain: CoordinatePlain): Coordinate { return new Coordinate(plain); }
  toPlain(): CoordinatePlain { return CoordinateSchema.parse(this as unknown as CoordinatePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Coordinate { return new Coordinate(CoordinateSchema.parse(JSON.parse(json))); }
}
export const CoordinateDiffSchema = CoordinateSchema.partial();
export type CoordinateDiff = z.infer<typeof CoordinateDiffSchema>;
// #endregion Coordinate

// #region Vec
export const VecSchema = z.object({ u: z.number(), v: z.number() });
export type VecPlain = z.infer<typeof VecSchema>;
export class Vec implements VecPlain {
  u!: number;
  v!: number;
  constructor(plain: VecPlain) { Object.assign(this, VecSchema.parse(plain)); }
  static from(plain: VecPlain): Vec { return new Vec(plain); }
  static fromPlain(plain: VecPlain): Vec { return new Vec(plain); }
  toPlain(): VecPlain { return VecSchema.parse(this as unknown as VecPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Vec { return new Vec(VecSchema.parse(JSON.parse(json))); }
}
export const VecDiffSchema = VecSchema.partial();
export type VecDiff = z.infer<typeof VecDiffSchema>;
// #endregion Vec

// #region Point
export const PointSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type PointPlain = z.infer<typeof PointSchema>;
export class Point implements PointPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: PointPlain) { Object.assign(this, PointSchema.parse(plain)); }
  static from(plain: PointPlain): Point { return new Point(plain); }
  static fromPlain(plain: PointPlain): Point { return new Point(plain); }
  toPlain(): PointPlain { return PointSchema.parse(this as unknown as PointPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Point { return new Point(PointSchema.parse(JSON.parse(json))); }
}
export const PointDiffSchema = PointSchema.partial();
export type PointDiff = z.infer<typeof PointDiffSchema>;
// #endregion Point

// #region Vector
export const VectorSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type VectorPlain = z.infer<typeof VectorSchema>;
export class Vector implements VectorPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: VectorPlain) { Object.assign(this, VectorSchema.parse(plain)); }
  static from(plain: VectorPlain): Vector { return new Vector(plain); }
  static fromPlain(plain: VectorPlain): Vector { return new Vector(plain); }
  toPlain(): VectorPlain { return VectorSchema.parse(this as unknown as VectorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Vector { return new Vector(VectorSchema.parse(JSON.parse(json))); }
}
export const VectorDiffSchema = VectorSchema.partial();
export type VectorDiff = z.infer<typeof VectorDiffSchema>;
// #endregion Vector

// #region Plane
export const PlaneSchema = z.object({ origin: PointSchema, xAxis: VectorSchema, yAxis: VectorSchema });
export type PlanePlain = z.infer<typeof PlaneSchema>;
export class Plane implements PlanePlain {
  origin!: Point;
  xAxis!: Vector;
  yAxis!: Vector;
  constructor(plain: PlanePlain) {
    const p = PlaneSchema.parse(plain);
    this.origin = new Point(p.origin);
    this.xAxis = new Vector(p.xAxis);
    this.yAxis = new Vector(p.yAxis);
  }
  static from(plain: PlanePlain): Plane { return new Plane(plain); }
  static fromPlain(plain: PlanePlain): Plane { return new Plane(plain); }
  toPlain(): PlanePlain { return PlaneSchema.parse(this as unknown as PlanePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Plane { return new Plane(PlaneSchema.parse(JSON.parse(json))); }
  // Removed: averageWith, average, rounded — geometry computation moved to semio/rs (Requirement 1.14)
}
export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true })
  .extend({ origin: PointDiffSchema, xAxis: VectorDiffSchema, yAxis: VectorDiffSchema }).partial();
export type PlaneDiff = z.infer<typeof PlaneDiffSchema>;
// #endregion Plane

// #region Camera
export const CameraSchema = z.object({ position: PointSchema, forward: VectorSchema, up: VectorSchema });
export type CameraPlain = z.infer<typeof CameraSchema>;
export class Camera implements CameraPlain {
  position!: Point;
  forward!: Vector;
  up!: Vector;
  constructor(plain: CameraPlain) {
    const p = CameraSchema.parse(plain);
    this.position = new Point(p.position);
    this.forward = new Vector(p.forward);
    this.up = new Vector(p.up);
  }
  static from(plain: CameraPlain): Camera { return new Camera(plain); }
  static fromPlain(plain: CameraPlain): Camera { return new Camera(plain); }
  toPlain(): CameraPlain { return CameraSchema.parse(this as unknown as CameraPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Camera { return new Camera(CameraSchema.parse(JSON.parse(json))); }
}
export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true })
  .extend({ position: PointDiffSchema, forward: VectorDiffSchema, up: VectorDiffSchema }).partial();
export type CameraDiff = z.infer<typeof CameraDiffSchema>;
// #endregion Camera

// #endregion Weak Entities

// #region Attribute
const DateProperty = () => z.string().optional();
export const AttributeSchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), definition: z.string().optional() });
export type AttributePlain = z.infer<typeof AttributeSchema>;
export class Attribute implements AttributePlain {
  id!: string; key!: string; value?: string; definition?: string;
  constructor(plain: AttributePlain) { Object.assign(this, AttributeSchema.parse(plain)); }
  static from(plain: AttributePlain): Attribute { return new Attribute(plain); }
  static fromPlain(plain: AttributePlain): Attribute { return new Attribute(plain); }
  static createId(id: string): AttributeId { return { id }; }
  static areSameId(a: AttributeId, b: AttributeId): boolean { return a.id === b.id; }
  toPlain(): AttributePlain { return AttributeSchema.parse(this as unknown as AttributePlain); }
  toJson(): string { return JSON.stringify(this.toPlain()); }
  static fromJson(json: string): Attribute { return new Attribute(AttributeSchema.parse(JSON.parse(json))); }
}
export const AttributeMetadataDtoSchema = AttributeSchema;
export type AttributeMetadataDto = z.infer<typeof AttributeMetadataDtoSchema>;
export const AttributeShallowSchema = AttributeSchema;
export type AttributeShallow = z.infer<typeof AttributeShallowSchema>;
export const AttributeDiffSchema = AttributeSchema.partial();
export type AttributeDiff = z.infer<typeof AttributeDiffSchema>;
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type AttributesDiff = z.infer<typeof AttributesDiffSchema>;
// #endregion Attribute

// #region Location
export const LocationSchema = z.object({ id: z.string(), longitude: z.number().optional(), latitude: z.number().optional(), altitude: z.number().optional(), attributes: z.array(AttributeSchema).optional() });
export type LocationPlain = z.infer<typeof LocationSchema>;
export class Location implements LocationPlain {
  id!: string; longitude?: number; latitude?: number; altitude?: number; attributes?: Attribute[];
  constructor(plain: LocationPlain) { const p = LocationSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: LocationPlain): Location { return new Location(plain); }
  static fromPlain(plain: LocationPlain): Location { return new Location(plain); }
  static createId(id: string): LocationId { return { id }; }
  static areSameId(a: LocationId, b: LocationId): boolean { return a.id === b.id; }
  toPlain(): LocationPlain { return LocationSchema.parse(this as unknown as LocationPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Location { return new Location(LocationSchema.parse(JSON.parse(json))); }
}
export const LocationMetadataDtoSchema = LocationSchema;
export type LocationMetadataDto = z.infer<typeof LocationMetadataDtoSchema>;
export const LocationShallowSchema = LocationSchema;
export type LocationShallow = z.infer<typeof LocationShallowSchema>;
export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LocationDiff = z.infer<typeof LocationDiffSchema>;
// #endregion Location

// #region Author
export const AuthorSchema = z.object({ id: z.string(), name: z.string(), email: z.string().optional(), role: z.string().optional(), rank: z.number().optional() });
export type AuthorPlain = z.infer<typeof AuthorSchema>;
export class Author implements AuthorPlain {
  id!: string; name!: string; email?: string; role?: string; rank?: number;
  constructor(plain: AuthorPlain) { Object.assign(this, AuthorSchema.parse(plain)); }
  static from(plain: AuthorPlain): Author { return new Author(plain); }
  static fromPlain(plain: AuthorPlain): Author { return new Author(plain); }
  static createId(id: string): AuthorId { return { id }; }
  static areSameId(a: AuthorId, b: AuthorId): boolean { return a.id === b.id; }
  toPlain(): AuthorPlain { return AuthorSchema.parse(this as unknown as AuthorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Author { return new Author(AuthorSchema.parse(JSON.parse(json))); }
}
export const AuthorMetadataDtoSchema = AuthorSchema;
export type AuthorMetadataDto = z.infer<typeof AuthorMetadataDtoSchema>;
export const AuthorShallowSchema = AuthorSchema;
export type AuthorShallow = z.infer<typeof AuthorShallowSchema>;
export const AuthorDiffSchema = AuthorSchema.partial();
export type AuthorDiff = z.infer<typeof AuthorDiffSchema>;
export const AuthorsDiffSchema = z.object({ removed: z.array(AuthorIdSchema).optional(), updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type AuthorsDiff = z.infer<typeof AuthorsDiffSchema>;
// #endregion Author

// #region File
export const FileSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  url: z.string().optional(),
  remote: z.string().optional(),
  mime: z.string().optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  description: z.string().optional(),
  blob: z.union([z.string(), z.custom<Blob>((v) => typeof Blob !== "undefined" && v instanceof Blob)]).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type FilePlain = z.infer<typeof FileSchema>;
export class File implements FilePlain {
  id!: string;
  name?: string;
  url?: string;
  remote?: string;
  mime?: string;
  size?: number;
  hash?: string;
  description?: string;
  blob?: string | Blob;
  createdAt?: string;
  updatedAt?: string;
  constructor(plain: FilePlain) { Object.assign(this, FileSchema.parse(plain)); }
  static from(plain: FilePlain): File { return new File(plain); }
  static fromPlain(plain: FilePlain): File { return new File(plain); }
  static createId(id: string): FileId { return { id }; }
  static areSameId(a: FileId, b: FileId): boolean { return a.id === b.id; }
  toPlain(): FilePlain { return FileSchema.parse(this as unknown as FilePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): File { return new File(FileSchema.parse(JSON.parse(json))); }
}
export const FileMetadataDtoSchema = FileSchema;
export type FileMetadataDto = z.infer<typeof FileMetadataDtoSchema>;
export const FileShallowSchema = FileSchema;
export type FileShallow = z.infer<typeof FileShallowSchema>;
export const FileDiffSchema = FileSchema.partial();
export type FileDiff = z.infer<typeof FileDiffSchema>;
export const FilesDiffSchema = z.object({ removed: z.array(FileIdSchema).optional(), updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FilesDiff = z.infer<typeof FilesDiffSchema>;
// #endregion File

// #region Folder
export const FolderSchema = z.object({ id: z.string(), path: z.string(), description: z.string().optional() });
export type FolderPlain = z.infer<typeof FolderSchema>;
export class Folder implements FolderPlain {
  id!: string; path!: string; description?: string;
  constructor(plain: FolderPlain) { Object.assign(this, FolderSchema.parse(plain)); }
  static from(plain: FolderPlain): Folder { return new Folder(plain); }
  static fromPlain(plain: FolderPlain): Folder { return new Folder(plain); }
  static createId(id: string): FolderId { return { id }; }
  static areSameId(a: FolderId, b: FolderId): boolean { return a.id === b.id; }
  toPlain(): FolderPlain { return FolderSchema.parse(this as unknown as FolderPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Folder { return new Folder(FolderSchema.parse(JSON.parse(json))); }
}
export const FolderMetadataDtoSchema = FolderSchema;
export type FolderMetadataDto = z.infer<typeof FolderMetadataDtoSchema>;
export const FolderShallowSchema = FolderSchema;
export type FolderShallow = z.infer<typeof FolderShallowSchema>;
export const FolderDiffSchema = FolderSchema.partial();
export type FolderDiff = z.infer<typeof FolderDiffSchema>;
export const FoldersDiffSchema = z.object({ removed: z.array(FolderIdSchema).optional(), updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FoldersDiff = z.infer<typeof FoldersDiffSchema>;
// #endregion Folder

// #region Benchmark
export const BenchmarkSchema = z.object({ id: z.string(), name: z.string(), min: z.number().optional(), max: z.number().optional(), minExcluded: z.boolean().optional(), maxExcluded: z.boolean().optional() });
export type BenchmarkPlain = z.infer<typeof BenchmarkSchema>;
export class Benchmark implements BenchmarkPlain {
  id!: string; name!: string; min?: number; max?: number; minExcluded?: boolean; maxExcluded?: boolean;
  constructor(plain: BenchmarkPlain) { Object.assign(this, BenchmarkSchema.parse(plain)); }
  static from(plain: BenchmarkPlain): Benchmark { return new Benchmark(plain); }
  static fromPlain(plain: BenchmarkPlain): Benchmark { return new Benchmark(plain); }
  static createId(id: string): BenchmarkId { return { id }; }
  static areSameId(a: BenchmarkId, b: BenchmarkId): boolean { return a.id === b.id; }
  toPlain(): BenchmarkPlain { return BenchmarkSchema.parse(this as unknown as BenchmarkPlain); }
  toJson(): string { return JSON.stringify(this.toPlain()); }
  static fromJson(json: string): Benchmark { return new Benchmark(BenchmarkSchema.parse(JSON.parse(json))); }
}
export const BenchmarkMetadataDtoSchema = BenchmarkSchema;
export type BenchmarkMetadataDto = z.infer<typeof BenchmarkMetadataDtoSchema>;
export const BenchmarkShallowSchema = BenchmarkSchema;
export type BenchmarkShallow = z.infer<typeof BenchmarkShallowSchema>;
export const BenchmarkDiffSchema = BenchmarkSchema.partial();
export type BenchmarkDiff = z.infer<typeof BenchmarkDiffSchema>;
export const BenchmarksDiffSchema = z.object({ removed: z.array(BenchmarkIdSchema).optional(), updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type BenchmarksDiff = z.infer<typeof BenchmarksDiffSchema>;
// #endregion Benchmark

// #region Quality
export const QualitySchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), unit: z.string().optional(), definition: z.string().optional(), description: z.string().optional(), benchmarks: z.array(BenchmarkSchema).optional() });
export type QualityPlain = z.infer<typeof QualitySchema>;
export class Quality implements QualityPlain {
  id!: string; key!: string; value?: string; unit?: string; definition?: string; description?: string; benchmarks?: Benchmark[];
  constructor(plain: QualityPlain) { const p = QualitySchema.parse(plain); Object.assign(this, p); this.benchmarks = p.benchmarks?.map((b) => new Benchmark(b)); }
  static from(plain: QualityPlain): Quality { return new Quality(plain); }
  static fromPlain(plain: QualityPlain): Quality { return new Quality(plain); }
  static createId(id: string): QualityId { return { id }; }
  static areSameId(a: QualityId, b: QualityId): boolean { return a.id === b.id; }
  toPlain(): QualityPlain { return QualitySchema.parse(this as unknown as QualityPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Quality { return new Quality(QualitySchema.parse(JSON.parse(json))); }
}
export const QualityMetadataDtoSchema = QualitySchema.omit({ benchmarks: true });
export type QualityMetadataDto = z.infer<typeof QualityMetadataDtoSchema>;
export const QualityShallowSchema = QualitySchema;
export type QualityShallow = z.infer<typeof QualityShallowSchema>;
export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true }).extend({ benchmarks: BenchmarksDiffSchema.optional() });
export type QualityDiff = z.infer<typeof QualityDiffSchema>;
export const QualitiesDiffSchema = z.object({ removed: z.array(QualityIdSchema).optional(), updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type QualitiesDiff = z.infer<typeof QualitiesDiffSchema>;
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
export type PortPlain = z.infer<typeof PortSchema>;
export class Port implements PortPlain {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  compatibleFamilies?: FamilyId[];
  mandatory?: boolean;
  t?: number;
  point?: Point;
  direction?: Vector;
  compatiblePorts?: PortId[];
  qualities?: Quality[];
  attributes?: Attribute[];
  maxChildren?: number;
  constructor(plain: PortPlain) { const p = PortSchema.parse(plain); Object.assign(this, p); this.point = p.point ? new Point(p.point) : undefined; this.direction = p.direction ? new Vector(p.direction) : undefined; this.qualities = p.qualities?.map((q) => new Quality(q)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: PortPlain): Port { return new Port(plain); }
  static fromPlain(plain: PortPlain): Port { return new Port(plain); }
  static createId(id: string): PortId { return { id }; }
  static areSameId(a: PortId, b: PortId): boolean { return a.id === b.id; }
  toPlain(): PortPlain { return PortSchema.parse(this as unknown as PortPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Port { return new Port(PortSchema.parse(JSON.parse(json))); }
}
export const PortMetadataDtoSchema = PortSchema.omit({ qualities: true, attributes: true });
export type PortMetadataDto = z.infer<typeof PortMetadataDtoSchema>;
export const PortShallowSchema = PortSchema;
export type PortShallow = z.infer<typeof PortShallowSchema>;
export const PortDiffSchema = PortSchema.partial().omit({ qualities: true, attributes: true }).extend({ qualities: QualitiesDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PortDiff = z.infer<typeof PortDiffSchema>;
export const PortsDiffSchema = z.object({ removed: z.array(PortIdSchema).optional(), updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PortsDiff = z.infer<typeof PortsDiffSchema>;
// #endregion Port

// #region Family
export const FamilySchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), icon: z.string().optional(), ports: z.array(PortSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type FamilyPlain = z.infer<typeof FamilySchema>;
export class Family implements FamilyPlain {
  id!: string; name!: string; description?: string; icon?: string; ports?: Port[]; attributes?: Attribute[];
  constructor(plain: FamilyPlain) { const p = FamilySchema.parse(plain); Object.assign(this, p); this.ports = p.ports?.map((x) => new Port(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: FamilyPlain): Family { return new Family(plain); }
  static fromPlain(plain: FamilyPlain): Family { return new Family(plain); }
  static createId(id: string): FamilyId { return { id }; }
  static areSameId(a: FamilyId, b: FamilyId): boolean { return a.id === b.id; }
  toPlain(): FamilyPlain { return FamilySchema.parse(this as unknown as FamilyPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Family { return new Family(FamilySchema.parse(JSON.parse(json))); }
}
export const FamilyMetadataDtoSchema = FamilySchema.omit({ ports: true, attributes: true });
export type FamilyMetadataDto = z.infer<typeof FamilyMetadataDtoSchema>;
export const FamilyShallowSchema = FamilySchema;
export type FamilyShallow = z.infer<typeof FamilyShallowSchema>;
export const FamilyDiffSchema = FamilySchema.partial().omit({ ports: true, attributes: true }).extend({ ports: PortsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type FamilyDiff = z.infer<typeof FamilyDiffSchema>;
export const FamiliesDiffSchema = z.object({ removed: z.array(FamilyIdSchema).optional(), updated: z.array(z.object({ family: FamilyIdSchema, diff: FamilyDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FamiliesDiff = z.infer<typeof FamiliesDiffSchema>;
// #endregion Family

// #region Prop
export const PropSchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), unit: z.string().optional(), quality: QualityIdSchema.optional() });
export type PropPlain = z.infer<typeof PropSchema>;
export class Prop implements PropPlain {
  id!: string; key!: string; value?: string; unit?: string; quality?: QualityId;
  constructor(plain: PropPlain) { Object.assign(this, PropSchema.parse(plain)); }
  static from(plain: PropPlain): Prop { return new Prop(plain); }
  static fromPlain(plain: PropPlain): Prop { return new Prop(plain); }
  static createId(id: string): PropId { return { id }; }
  static areSameId(a: PropId, b: PropId): boolean { return a.id === b.id; }
  toPlain(): PropPlain { return PropSchema.parse(this as unknown as PropPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Prop { return new Prop(PropSchema.parse(JSON.parse(json))); }
}
export const PropMetadataDtoSchema = PropSchema;
export type PropMetadataDto = z.infer<typeof PropMetadataDtoSchema>;
export const PropShallowSchema = PropSchema;
export type PropShallow = z.infer<typeof PropShallowSchema>;
export const PropDiffSchema = PropSchema.partial();
export type PropDiff = z.infer<typeof PropDiffSchema>;
export const PropsDiffSchema = z.object({ removed: z.array(PropIdSchema).optional(), updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PropsDiff = z.infer<typeof PropsDiffSchema>;
// #endregion Prop

// #region Tag
export const TagSchema = z.object({ id: z.string(), name: z.string(), order: z.number().optional() });
export type TagPlain = z.infer<typeof TagSchema>;
export class Tag implements TagPlain {
  id!: string; name!: string; order?: number;
  constructor(plain: TagPlain) { Object.assign(this, TagSchema.parse(plain)); }
  static from(plain: TagPlain): Tag { return new Tag(plain); }
  static fromPlain(plain: TagPlain): Tag { return new Tag(plain); }
  static createId(id: string): TagId { return { id }; }
  static areSameId(a: TagId, b: TagId): boolean { return a.id === b.id; }
  toPlain(): TagPlain { return TagSchema.parse(this as unknown as TagPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Tag { return new Tag(TagSchema.parse(JSON.parse(json))); }
}
export const TagMetadataDtoSchema = TagSchema;
export type TagMetadataDto = z.infer<typeof TagMetadataDtoSchema>;
export const TagShallowSchema = TagSchema;
export type TagShallow = z.infer<typeof TagShallowSchema>;
export const TagDiffSchema = TagSchema.partial();
export type TagDiff = z.infer<typeof TagDiffSchema>;
export const TagsDiffSchema = z.object({ removed: z.array(TagIdSchema).optional(), updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TagsDiff = z.infer<typeof TagsDiffSchema>;
// #endregion Tag

// #region Concept
export const ConceptSchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), order: z.number().optional() });
export type ConceptPlain = z.infer<typeof ConceptSchema>;
export class Concept implements ConceptPlain {
  id!: string; name!: string; description?: string; order?: number;
  constructor(plain: ConceptPlain) { Object.assign(this, ConceptSchema.parse(plain)); }
  static from(plain: ConceptPlain): Concept { return new Concept(plain); }
  static fromPlain(plain: ConceptPlain): Concept { return new Concept(plain); }
  static createId(id: string): ConceptId { return { id }; }
  static areSameId(a: ConceptId, b: ConceptId): boolean { return a.id === b.id; }
  toPlain(): ConceptPlain { return ConceptSchema.parse(this as unknown as ConceptPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Concept { return new Concept(ConceptSchema.parse(JSON.parse(json))); }
}
export const ConceptMetadataDtoSchema = ConceptSchema;
export type ConceptMetadataDto = z.infer<typeof ConceptMetadataDtoSchema>;
export const ConceptShallowSchema = ConceptSchema;
export type ConceptShallow = z.infer<typeof ConceptShallowSchema>;
export const ConceptDiffSchema = ConceptSchema.partial();
export type ConceptDiff = z.infer<typeof ConceptDiffSchema>;
export const ConceptsDiffSchema = z.object({ removed: z.array(ConceptIdSchema).optional(), updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConceptsDiff = z.infer<typeof ConceptsDiffSchema>;
// #endregion Concept

// #region Representation
export const RepresentationSchema = z.object({ id: z.string(), name: z.string().optional(), tags: z.array(TagIdSchema).optional(), file: FileIdSchema, description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type RepresentationPlain = z.infer<typeof RepresentationSchema>;
export class Representation implements RepresentationPlain {
  id!: string; name?: string; tags?: TagId[]; file!: FileId; description?: string; attributes?: Attribute[];
  constructor(plain: RepresentationPlain) { const p = RepresentationSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: RepresentationPlain): Representation { return new Representation(plain); }
  static fromPlain(plain: RepresentationPlain): Representation { return new Representation(plain); }
  static createId(id: string): RepresentationId { return { id }; }
  static areSameId(a: RepresentationId, b: RepresentationId): boolean { return a.id === b.id; }
  toPlain(): RepresentationPlain { return RepresentationSchema.parse(this as unknown as RepresentationPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Representation { return new Representation(RepresentationSchema.parse(JSON.parse(json))); }
}
export const RepresentationMetadataDtoSchema = RepresentationSchema.omit({ tags: true, attributes: true });
export type RepresentationMetadataDto = z.infer<typeof RepresentationMetadataDtoSchema>;
export const RepresentationShallowSchema = RepresentationSchema;
export type RepresentationShallow = z.infer<typeof RepresentationShallowSchema>;
export const RepresentationDiffSchema = RepresentationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type RepresentationDiff = z.infer<typeof RepresentationDiffSchema>;
export const RepresentationsDiffSchema = z.object({ removed: z.array(RepresentationIdSchema).optional(), updated: z.array(z.object({ representation: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type RepresentationsDiff = z.infer<typeof RepresentationsDiffSchema>;
// Removed: selectBestRepresentation, filterRepresentationsByTagIds, getAvailableTagIdsForRepresentations, getAllTagIdsFromRepresentations, findRepresentation, areSameRepresentation, SUPPORTED_3D_EXTENSIONS, isSupportedRepresentationExtension, validateRepresentationFile, RepresentationFileValidation — representation selection logic moved to semio/rs (Requirement 1.3)
// #endregion Representation

// #region Connector
export const ConnectorSchema = z.object({ id: z.string(), name: z.string().optional(), t: z.number(), point: PointSchema, direction: VectorSchema, description: z.string().optional(), port: PortIdSchema.optional(), mandatory: z.boolean().optional(), maxChildren: z.number().int().optional(), props: z.array(PropSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type ConnectorPlain = z.infer<typeof ConnectorSchema>;
export class Connector implements ConnectorPlain {
  id!: string; name?: string; t!: number; point!: Point; direction!: Vector; description?: string; port?: PortId; mandatory?: boolean; maxChildren?: number; props?: Prop[]; attributes?: Attribute[];
  constructor(plain: ConnectorPlain) { const p = ConnectorSchema.parse(plain); Object.assign(this, p); this.point = new Point(p.point); this.direction = new Vector(p.direction); this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: ConnectorPlain): Connector { return new Connector(plain); }
  static fromPlain(plain: ConnectorPlain): Connector { return new Connector(plain); }
  static createId(id: string): ConnectorId { return { id }; }
  static areSameId(a: ConnectorId, b: ConnectorId): boolean { return a.id === b.id; }
  toPlain(): ConnectorPlain { return ConnectorSchema.parse(this as unknown as ConnectorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Connector { return new Connector(ConnectorSchema.parse(JSON.parse(json))); }
}
export const ConnectorMetadataDtoSchema = ConnectorSchema.omit({ props: true, attributes: true });
export type ConnectorMetadataDto = z.infer<typeof ConnectorMetadataDtoSchema>;
export const ConnectorShallowSchema = ConnectorSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type ConnectorShallow = z.infer<typeof ConnectorShallowSchema>;
export const ConnectorDiffSchema = ConnectorSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({ point: PointDiffSchema.optional(), direction: VectorDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), maxChildren: z.number().int().nullable().optional() });
export type ConnectorDiff = z.infer<typeof ConnectorDiffSchema>;
export const ConnectorsDiffSchema = z.object({ removed: z.array(ConnectorIdSchema).optional(), updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectorsDiff = z.infer<typeof ConnectorsDiffSchema>;
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
export type TypePlain = z.infer<typeof TypeSchema>;
export class Type {
  id!: string;
  name!: string;
  parent?: { id: string };
  families?: FamilyId[];
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
  location?: LocationId;
  authors?: AuthorId[];
  concepts?: ConceptId[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  lifecycle?: EntityLifecycle;
  deletedByUserId?: string;
  deletedByDisplayName?: string;
  deletedAt?: string;
  deletedInChangeId?: string;
  constructor(plain: TypePlain) { const p = TypeSchema.parse(plain); Object.assign(this, p); this.representations = p.representations?.map((m) => new Representation(m)); this.connectors = p.connectors?.map((c) => new Connector(c)); this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static fromPlain(plain: TypePlain): Type { return new Type(plain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Type { return Type.fromPlain(TypeSchema.parse(JSON.parse(json))); }
  toPlain(): TypePlain { return TypeSchema.parse({ ...(this as unknown as TypePlain) }); }
  static createId(id: string): TypeId { return { id }; }
  static areSameId(a: TypeId, b: TypeId): boolean { return a.id === b.id; }
  /** @emoji 🖼️ Picks a representation for scene rendering (`@semio/ui`); first match until WASM metadata is wired. */
  static pickBestRepresentation(representations: readonly Representation[], _tagIds: readonly string[]): Representation | undefined {
    void _tagIds;
    return representations[0];
  }
}
export const TypeMetadataDtoSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
export type TypeMetadataDto = z.infer<typeof TypeMetadataDtoSchema>;
export const TypeShallowSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true }).extend({ representations: z.array(RepresentationMetadataDtoSchema).optional(), connectors: z.array(ConnectorMetadataDtoSchema).optional(), props: z.array(PropMetadataDtoSchema).optional(), attributes: z.array(AttributeMetadataDtoSchema).optional() });
export type TypeShallow = z.infer<typeof TypeShallowSchema>;
export const TypeDiffSchema = TypeSchema.partial().omit({ representations: true, connectors: true, props: true, attributes: true }).extend({ representations: RepresentationsDiffSchema.optional(), connectors: ConnectorsDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), description: z.string().nullable().optional(), icon: z.string().nullable().optional(), image: z.string().nullable().optional(), location: LocationIdSchema.nullable().optional(), folder: z.string().nullable().optional(), concepts: z.array(ConceptIdSchema).nullable().optional(), authors: z.array(AuthorIdSchema).nullable().optional(), families: z.array(FamilyIdSchema).nullable().optional(), lifecycle: z.enum(["active", "deleted"]).optional(), deletedByUserId: z.string().nullable().optional(), deletedByDisplayName: z.string().nullable().optional(), deletedAt: z.string().nullable().optional(), deletedInChangeId: z.string().nullable().optional() });
export type TypeDiff = z.infer<typeof TypeDiffSchema>;
export const TypesDiffSchema = z.object({ removed: z.array(TypeIdSchema).optional(), updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TypesDiff = z.infer<typeof TypesDiffSchema>;
// #endregion Type

// #region Layer
export const LayerSchema = z.object({ id: z.string(), path: z.string(), isHidden: z.boolean().optional(), isLocked: z.boolean().optional(), color: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type LayerPlain = z.infer<typeof LayerSchema>;
export class Layer implements LayerPlain {
  id!: string; path!: string; isHidden?: boolean; isLocked?: boolean; color?: string; description?: string; attributes?: Attribute[];
  constructor(plain: LayerPlain) { const p = LayerSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: LayerPlain): Layer { return new Layer(plain); }
  static fromPlain(plain: LayerPlain): Layer { return new Layer(plain); }
  static createId(id: string): LayerId { return { id }; }
  static areSameId(a: LayerId, b: LayerId): boolean { return a.id === b.id; }
  toPlain(): LayerPlain { return LayerSchema.parse(this as unknown as LayerPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Layer { return new Layer(LayerSchema.parse(JSON.parse(json))); }
}
export const LayerMetadataDtoSchema = LayerSchema.omit({ attributes: true });
export type LayerMetadataDto = z.infer<typeof LayerMetadataDtoSchema>;
export const LayerShallowSchema = LayerSchema;
export type LayerShallow = z.infer<typeof LayerShallowSchema>;
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LayerDiff = z.infer<typeof LayerDiffSchema>;
export const LayersDiffSchema = z.object({ removed: z.array(LayerIdSchema).optional(), updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type LayersDiff = z.infer<typeof LayersDiffSchema>;
// #endregion Layer

// #region Piece
export const PieceSchema = z.object({ id: z.string(), name: z.string().optional(), type: TypeIdSchema.optional(), design: DesignIdSchema.optional(), plane: PlaneSchema.optional(), center: CoordinateSchema.optional(), scale: z.number().optional(), mirrorPlane: PlaneSchema.optional(), isHidden: z.boolean().optional(), isLocked: z.boolean().optional(), color: z.string().optional(), description: z.string().optional(), props: z.array(PropSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type PiecePlain = z.infer<typeof PieceSchema>;
export class Piece {
  id!: string; name?: string; type?: TypeId; design?: DesignId; plane?: Plane; center?: Coordinate; scale?: number; mirrorPlane?: Plane; isHidden?: boolean; isLocked?: boolean; color?: string; description?: string; props?: Prop[]; attributes?: Attribute[];
  constructor(plain: PiecePlain) { const p = PieceSchema.parse(plain); Object.assign(this, p); this.plane = p.plane ? new Plane(p.plane) : undefined; this.center = p.center ? new Coordinate(p.center) : undefined; this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined; this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static fromPlain(plain: PiecePlain): Piece { return new Piece(plain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Piece { return new Piece(PieceSchema.parse(JSON.parse(json))); }
  toPlain(): PiecePlain { return PieceSchema.parse(this as unknown as PiecePlain); }
  static createId(id: string): PieceId { return { id }; }
  static areSameId(a: PieceId, b: PieceId): boolean { return a.id === b.id; }

  /** @emoji 🧭 Whether this piece wires a nested design id (schema hooks). */
  wireDesignAsPieceId(): boolean {
    return Boolean(this.design?.id);
  }

  /** @emoji 🧭 Wired type id for schema hooks. */
  wireTypeId(): { id: string } | undefined {
    return this.type ? { id: this.type.id } : undefined;
  }

  /** @emoji 🧭 Flat plane DTO for UI (structural truth in `semio/rs` reads). */
  flatPlane(): unknown {
    return this.plane ? this.plane.toPlain() : undefined;
  }

  /** @emoji 🧭 Flat center UV for UI. */
  flatCenter(): unknown {
    return this.center ? this.center.toPlain() : undefined;
  }

  /** @emoji 🧭 Alternative types for replaceable UI (populated from reads in full hosts). */
  alternativeTypes(): readonly Type[] {
    return [];
  }
}
export const PieceMetadataDtoSchema = PieceSchema.omit({ props: true, attributes: true });
export type PieceMetadataDto = z.infer<typeof PieceMetadataDtoSchema>;
export const PieceShallowSchema = PieceSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type PieceShallow = z.infer<typeof PieceShallowSchema>;
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({ plane: PlaneDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PieceDiff = z.infer<typeof PieceDiffSchema>;
export const PiecesDiffSchema = z.object({ removed: z.array(PieceIdSchema).optional(), updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PiecesDiff = z.infer<typeof PiecesDiffSchema>;
// Removed: isFixedPiece, findPiece, findPieceConnections, findConnectorForPieceInConnection, getPieceRepresentationFileIds, getPieceRepresentationUrls, resolvePieceTypeForFlatten — domain logic moved to semio/rs
// #endregion Piece

// #region Group
export const GroupSchema = z.object({ id: z.string(), pieces: z.array(PieceIdSchema), color: z.string().optional(), name: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type GroupPlain = z.infer<typeof GroupSchema>;
export class Group implements GroupPlain {
  id!: string; pieces!: PieceId[]; color?: string; name?: string; description?: string; attributes?: Attribute[];
  constructor(plain: GroupPlain) { const p = GroupSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: GroupPlain): Group { return new Group(plain); }
  static fromPlain(plain: GroupPlain): Group { return new Group(plain); }
  static createId(id: string): GroupId { return { id }; }
  static areSameId(a: GroupId, b: GroupId): boolean { return a.id === b.id; }
  toPlain(): GroupPlain { return GroupSchema.parse(this as unknown as GroupPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Group { return new Group(GroupSchema.parse(JSON.parse(json))); }
}
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type GroupDiff = z.infer<typeof GroupDiffSchema>;
export const GroupsDiffSchema = z.object({ removed: z.array(GroupIdSchema).optional(), updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type GroupsDiff = z.infer<typeof GroupsDiffSchema>;
export const GroupMetadataDtoSchema = GroupSchema.omit({ pieces: true, attributes: true });
export type GroupMetadataDto = z.infer<typeof GroupMetadataDtoSchema>;
export const GroupShallowSchema = GroupSchema;
export type GroupShallow = z.infer<typeof GroupShallowSchema>;
// #endregion Group

// #region Side
export const SideSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SidePlain = z.infer<typeof SideSchema>;
export class Side {
  #pieceId!: string;
  #designPieceId?: string;
  #connectorId?: string;
  constructor(plain: SidePlain) { const p = SideSchema.parse(plain); this.#pieceId = p.piece.id; this.#designPieceId = p.designPiece?.id; this.#connectorId = p.connector?.id; }
  get piece(): PieceId { return { id: this.#pieceId }; }
  get designPiece(): PieceId | undefined { if (!this.#designPieceId) return undefined; return { id: this.#designPieceId }; }
  get connector(): ConnectorId | undefined { return this.#connectorId !== undefined ? { id: this.#connectorId } : undefined; }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Side { return new Side(SideSchema.parse(JSON.parse(json))); }
  static from(plain: SidePlain): Side { return new Side(plain); }
  static fromPlain(plain: SidePlain): Side { return new Side(plain); }
  toPlain(): SidePlain { return SideSchema.parse({ piece: { id: this.#pieceId }, designPiece: this.#designPieceId ? { id: this.#designPieceId } : undefined, connector: this.#connectorId ? { id: this.#connectorId } : undefined }); }
}
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = z.infer<typeof SideDiffSchema>;
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SideIdPlain = z.infer<typeof SideIdSchema>;
export class SideId implements SideIdPlain {
  piece!: PieceId; designPiece?: PieceId; connector?: ConnectorId;
  constructor(plain: SideIdPlain) { Object.assign(this, SideIdSchema.parse(plain)); }
  static from(plain: SideIdPlain): SideId { return new SideId(plain); }
  toPlain(): SideIdPlain { return SideIdSchema.parse(this as unknown as SideIdPlain); }
}
export const SidesDiffSchema = z.object({ removed: z.array(SideIdSchema).optional(), updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type SidesDiff = z.infer<typeof SidesDiffSchema>;
// #endregion Side

// #region Connection
export const ConnectionSchema = z.object({ id: z.string(), connected: SideSchema, connecting: SideSchema, gap: z.number().optional(), shift: z.number().optional(), rise: z.number().optional(), rotation: z.number().optional(), turn: z.number().optional(), tilt: z.number().optional(), u: z.number().optional(), v: z.number().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type ConnectionPlain = z.infer<typeof ConnectionSchema>;
export class Connection implements ConnectionPlain {
  id!: string; connected!: Side; connecting!: Side; gap?: number; shift?: number; rise?: number; rotation?: number; turn?: number; tilt?: number; u?: number; v?: number; description?: string; attributes?: Attribute[];
  constructor(plain: ConnectionPlain) { const p = ConnectionSchema.parse(plain); Object.assign(this, p); this.connected = new Side(p.connected); this.connecting = new Side(p.connecting); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Connection { return new Connection(ConnectionSchema.parse(JSON.parse(json))); }
  static from(plain: ConnectionPlain): Connection { return new Connection(plain); }
  static fromPlain(plain: ConnectionPlain): Connection { return new Connection(plain); }
  static createId(id: string): ConnectionId { return { id }; }
  static areSameId(a: ConnectionId, b: ConnectionId): boolean { return a.id === b.id; }
  toPlain(): ConnectionPlain { return ConnectionSchema.parse({ id: this.id, connected: this.connected.toPlain(), connecting: this.connecting.toPlain(), gap: this.gap, shift: this.shift, rise: this.rise, rotation: this.rotation, turn: this.turn, tilt: this.tilt, u: this.u, v: this.v, description: this.description, attributes: this.attributes?.map((a) => a.toPlain()) } as unknown as ConnectionPlain); }
}
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ id: true, connected: true, connecting: true, attributes: true }).extend({ connected: SideDiffSchema.optional(), connecting: SideDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type ConnectionDiff = z.infer<typeof ConnectionDiffSchema>;
export const ConnectionsDiffSchema = z.object({ removed: z.array(ConnectionIdSchema).optional(), updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectionsDiff = z.infer<typeof ConnectionsDiffSchema>;
export const ConnectionMetadataDtoSchema = ConnectionSchema.omit({ attributes: true });
export type ConnectionMetadataDto = z.infer<typeof ConnectionMetadataDtoSchema>;
export const ConnectionShallowSchema = ConnectionSchema;
export type ConnectionShallow = z.infer<typeof ConnectionShallowSchema>;
// #endregion Connection

// #region Stat
export const StatSchema = z.object({ id: z.string(), quality: QualityIdSchema, unit: z.string().optional(), min: z.number().optional(), minExcluded: z.boolean().optional(), max: z.number().optional(), maxExcluded: z.boolean().optional() });
export type StatPlain = z.infer<typeof StatSchema>;
export class Stat implements StatPlain {
  id!: string; quality!: QualityId; unit?: string; min?: number; minExcluded?: boolean; max?: number; maxExcluded?: boolean;
  constructor(plain: StatPlain) { Object.assign(this, StatSchema.parse(plain)); }
  static from(plain: StatPlain): Stat { return new Stat(plain); }
  static fromPlain(plain: StatPlain): Stat { return new Stat(plain); }
  static createId(id: string): StatId { return { id }; }
  static areSameId(a: StatId, b: StatId): boolean { return a.id === b.id; }
  toPlain(): StatPlain { return StatSchema.parse(this as unknown as StatPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Stat { return new Stat(StatSchema.parse(JSON.parse(json))); }
}
export const StatDiffSchema = StatSchema.partial();
export type StatDiff = z.infer<typeof StatDiffSchema>;
export const StatsDiffSchema = z.object({ removed: z.array(StatIdSchema).optional(), updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type StatsDiff = z.infer<typeof StatsDiffSchema>;
export const StatMetadataDtoSchema = StatSchema;
export type StatMetadataDto = z.infer<typeof StatMetadataDtoSchema>;
export const StatShallowSchema = StatSchema;
export type StatShallow = z.infer<typeof StatShallowSchema>;
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
export type DesignPlain = z.infer<typeof DesignSchema>;

export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).partial().extend({ pieces: PiecesDiffSchema.optional(), connections: ConnectionsDiffSchema.optional(), stats: StatsDiffSchema.optional(), props: PropsDiffSchema.optional(), layers: LayersDiffSchema.optional(), groups: GroupsDiffSchema.optional(), authors: AuthorsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type DesignDiff = z.infer<typeof DesignDiffSchema>;
export const DesignsDiffSchema = z.object({ removed: z.array(DesignIdSchema).optional(), updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type DesignsDiff = z.infer<typeof DesignsDiffSchema>;

/** @emoji ⚠️ Algorithm adapter / native REST error row. */
export type AlgorithmError = { readonly code: string; readonly message: string };
export type DesignDiffOperationResult = { readonly ok: true; readonly diff: DesignDiff } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };
export type OperationResult<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };

/** @emoji 🔧 Gap/shift/rise knobs for structural move previews (algorithms UI). */
export type MoveVector = { readonly gap: number; readonly shift: number; readonly rise: number };

/** @emoji 📌 Paste anchoring modes for copy/paste algorithm stories. */
export type PasteDesignAnchoringKind =
  | "original"
  | "middle"
  | "centroid"
  | "bottomLeft"
  | "bottomRight"
  | "topLeft"
  | "topRight";

/** @emoji 🧠 Optional per-piece flatten cache row (TS algorithm path; opaque to callers). */
export type FlatMerkleCacheEntry = Readonly<Record<string, unknown>>;

export class Design {
  id!: string;
  name!: string;
  parent?: { id: string };
  families?: FamilyId[];
  isAbstract?: boolean;
  folder?: string;
  pieces?: Piece[];
  _connections?: Connection[];
  stats?: Stat[];
  props?: Prop[];
  layers?: Layer[];
  activeLayer?: LayerId;
  groups?: Group[];
  canScale?: boolean;
  canMirror?: boolean;
  unit?: string;
  location?: LocationId;
  authors?: AuthorId[];
  concepts?: ConceptId[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;
  get connections(): Connection[] | undefined {
    return this._connections;
  }
  constructor(plain: DesignPlain | Design) {
    const wire: DesignPlain = plain instanceof Design ? plain.toPlain() : plain;
    const p = DesignSchema.parse(wire);
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
  static fromPlain(plain: DesignPlain): Design { return new Design(plain); }
  toPlain(): DesignPlain {
    return DesignSchema.parse({
      ...(this as unknown as DesignPlain),
      pieces: this.pieces?.map((x) => x.toPlain()),
      connections: this._connections?.map((x) => x.toPlain()),
      stats: this.stats?.map((x) => x.toPlain()),
      props: this.props?.map((x) => x.toPlain()),
      layers: this.layers?.map((x) => x.toPlain()),
      groups: this.groups?.map((x) => x.toPlain()),
      attributes: this.attributes?.map((x) => x.toPlain()),
    });
  }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Design { return new Design(DesignSchema.parse(JSON.parse(json))); }
  static createId(id: string): DesignId { return { id }; }
  static areSameId(a: DesignId, b: DesignId): boolean { return a.id === b.id; }

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
    const plain = design instanceof Design ? design.toPlain() : DesignSchema.parse(design as unknown as DesignPlain);
    const n = new Design(plain);
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
          const c = p.center ? p.center.toPlain() : { u: 0, v: 0 };
          p.center = new Coordinate({ ...c, ...d.center });
        }
        if (d.plane && p.plane) {
          const pl = p.plane.toPlain();
          const o = d.plane.origin ? { ...pl.origin, ...d.plane.origin } : pl.origin;
          const xa = d.plane.xAxis ? { ...pl.xAxis, ...d.plane.xAxis } : pl.xAxis;
          const ya = d.plane.yAxis ? { ...pl.yAxis, ...d.plane.yAxis } : pl.yAxis;
          p.plane = new Plane({ origin: o, xAxis: xa, yAxis: ya });
        }
      }
    }
    if (diff.pieces?.added?.length) {
      this.pieces = [...(this.pieces ?? []), ...diff.pieces.added.map((x) => new Piece(PieceSchema.parse(x as PiecePlain)))];
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
      this._connections = [
        ...(this._connections ?? []),
        ...diff.connections.added.map((x) => new Connection(ConnectionSchema.parse(x as z.infer<typeof ConnectionSchema>))),
      ];
    }
  }

  /** @emoji 🧾 Selection drag in flat UV space (piece centers only; algorithm preview). */
  dragBySelection(piecesDesign: Design, offset: CoordinatePlain): DesignDiff {
    const du = offset.u ?? 0;
    const dv = offset.v ?? 0;
    const sel = new Set((piecesDesign.pieces ?? []).map((p) => p.id));
    const updated = (this.pieces ?? [])
      .filter((p) => sel.has(p.id))
      .map((p) => {
        const c = p.center?.toPlain() ?? { u: 0, v: 0 };
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

export type DesignOperationResult =
  | { readonly ok: true; readonly design: Design; readonly diff: { forward: DesignDiff; reverse: DesignDiff } }
  | { readonly ok: false; readonly errors: readonly AlgorithmError[] };

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
export type DesignMetadataDto = z.infer<typeof DesignMetadataDtoSchema>;
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true }).extend({ pieces: z.array(PieceMetadataDtoSchema).optional(), connections: z.array(ConnectionMetadataDtoSchema).optional(), stats: z.array(StatMetadataDtoSchema).optional(), props: z.array(PropMetadataDtoSchema).optional(), layers: z.array(LayerMetadataDtoSchema).optional(), groups: z.array(GroupMetadataDtoSchema).optional(), attributes: z.array(AttributeMetadataDtoSchema).optional() });
export type DesignShallow = z.infer<typeof DesignShallowSchema>;
// Removed: addPieceToDesignDiff, setPieceInDesignDiff, removePieceFromDesignDiff, addPiecesToDesignDiff, setPiecesInDesignDiff, removePiecesFromDesignDiff, addConnectionToDesignDiff, setConnectionInDesignDiff, removeConnectionFromDesignDiff, addConnectionsToDesignDiff, setConnectionsInDesignDiff, removeConnectionsFromDesignDiff, mergeDesigns, orientDesign, duplicateDesignDiffForIsolation — design-diff builder functions moved to semio/rs (Requirement 3.7)
// #endregion Design

// #region Kit
export const KitKindSchema = z.enum(["dev", "local", "archive", "remote", "transport"]);
export type KitKind = z.infer<typeof KitKindSchema>;
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;

export const KitFullDtoSchema = z.object({ id: z.string(), name: z.string(), version: z.string().optional(), types: z.array(TypeSchema).optional(), designs: z.array(DesignSchema).optional(), tags: z.array(TagSchema).optional(), concepts: z.array(ConceptSchema).optional(), families: z.array(FamilySchema).optional(), qualities: z.array(QualitySchema).optional(), files: z.array(FileSchema).optional(), folders: z.array(FolderSchema).optional(), authors: z.array(AuthorSchema).optional(), remote: z.string().optional(), homepage: z.string().optional(), license: z.string().optional(), preview: z.string().optional(), icon: z.string().optional(), image: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional(), createdAt: DateProperty(), updatedAt: DateProperty() });
export type KitFullDto = z.infer<typeof KitFullDtoSchema>;

export class Kit {
  /** @emoji 📌 Anchoring kinds exposed to copy/paste algorithm UI. */
  static readonly pasteDesignAnchoringKinds: readonly PasteDesignAnchoringKind[] = [
    "original",
    "middle",
    "centroid",
    "bottomLeft",
    "bottomRight",
    "topLeft",
    "topRight",
  ];

  /** @emoji 🧭 Normalizes plain/DTO kit records to a {@link Kit} entity (replaces legacy `Kit.ensure`). */
  static ensure(kit: Kit | KitFullDto): Kit {
    return kit instanceof Kit ? kit : Kit.fromPlain(KitFullDtoSchema.parse(kit));
  }

  /** @emoji 📋 Copy selection (TS path stub — use REST language or extend with KitStore batch). */
  copyDesignOp(_design: Design, _pieceIds: readonly string[], _connectionIds: readonly string[]): OperationResult<Design> {
    void _design;
    void _pieceIds;
    void _connectionIds;
    return { ok: false, errors: [{ code: "native.copy.ts", message: "nativeCopyDesign(ts): not wired to WASM batch yet; switch language or implement batch copy." }] };
  }

  /** @emoji 📋 Paste selection (TS path stub). */
  pasteDesignOp(_source: Design, _target: Design, _anchoring: string, _coordinate: CoordinatePlain | undefined): DesignDiff {
    void _source;
    void _target;
    void _anchoring;
    void _coordinate;
    return {};
  }

  id!: string; name!: string; version?: string; types?: Type[]; designs?: Design[]; tags?: Tag[]; concepts?: Concept[]; families?: Family[]; qualities?: Quality[]; files?: File[]; folders?: Folder[]; authors?: Author[]; remote?: string; homepage?: string; license?: string; preview?: string; icon?: string; image?: string; description?: string; attributes?: Attribute[]; createdAt!: string; updatedAt!: string;
  constructor(data: KitFullDto) {
    const p = KitFullDtoSchema.parse(data);
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
  static fromPlain(data: KitFullDto): Kit { return new Kit(data); }
  toPlain(): KitFullDto {
    return KitFullDtoSchema.parse({
      ...(this as unknown as KitFullDto),
      types: this.types?.map((t) => t.toPlain()),
      designs: this.designs?.map((d) => d.toPlain()),
      tags: this.tags?.map((t) => t.toPlain()),
      concepts: this.concepts?.map((c) => c.toPlain()),
      families: this.families?.map((f) => f.toPlain()),
      qualities: this.qualities?.map((q) => q.toPlain()),
      files: this.files?.map((f) => f.toPlain()),
      folders: this.folders?.map((f) => f.toPlain()),
      authors: this.authors?.map((a) => a.toPlain()),
      attributes: this.attributes?.map((a) => a.toPlain()),
    });
  }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Kit { return Kit.fromPlain(KitFullDtoSchema.parse(JSON.parse(json))); }
  toJSON(): KitFullDto { return this.toPlain(); }
  static createId(id: string): KitId { return { id }; }
  static areSameId(a: KitId, b: KitId): boolean { return a.id === b.id; }

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
  flattenDesignCachedOp(
    designId: string,
    _prev?: { [pieceId: string]: FlatMerkleCacheEntry },
  ): { result: DesignOperationResult; cache: { [pieceId: string]: FlatMerkleCacheEntry } } {
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
            plane: (p.plane?.toPlain() as unknown) ?? defaultPlane,
            center: p.center?.toPlain() ?? { u: 0, v: 0 },
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
  return input instanceof Kit ? input : Kit.fromPlain(KitFullDtoSchema.parse(input as KitFullDto));
}

/**
 * @emoji 🧾 Pulls the authoritative DTO from `kitClient` into a host {@link KitHostStore} (no React; call after GQL events).
 */
/** @emoji 🧾 Minimal bridge surface used when applying WASM snapshots onto a host store. */
export type SemioKitBridge = { getDto(): unknown; getSnapshot(): Promise<unknown> };

export async function applyKitClientSnapshotToLocalStore(kitClient: SemioKitBridge, store: KitHostStore): Promise<void> {
  try {
    await kitClient.getSnapshot();
  } catch {
    /* keep last cached DTO from the client */
  }
  try {
    const incoming = kitClient.getDto();
    const curJson = store.getSnapshot().kit.toJSON();
    if (JSON.stringify(incoming) === JSON.stringify(curJson)) return;
    store.replace(asKitInstance(incoming as KitLike));
  } catch {
    try {
      store.replace(asKitInstance(kitClient.getDto() as KitLike));
    } catch {
      /* ignore */
    }
  }
}

/** @emoji 🧭 Local/sync facet on every kit store snapshot (WASM or file-backed; hooks read `sync.readonly` etc). */
export type KitSyncSnapshot = { status: string; dirty: boolean; readonly: boolean; lastSyncedAt: string | null; error: unknown | null };
export const DEFAULT_KIT_SYNC: Readonly<KitSyncSnapshot> = Object.freeze({ status: "idle", dirty: false, readonly: false, lastSyncedAt: null, error: null });
export type KitHostStoreSnapshot = { kit: Kit; sync: KitSyncSnapshot };
export type KitHostStore = { getSnapshot(): KitHostStoreSnapshot; subscribe(onChange: () => void): () => void; replace(kit: Kit): void };

export class InMemoryKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal Used by `inferPersistenceFromInit` in @semio/react. */
  readonly name = "InMemoryKitStore";
  constructor(seed: Kit) { this._kit = seed; }
  getSnapshot(): KitHostStoreSnapshot {
    const c = (this as InMemoryKitStore & { __semioKitBridge?: SemioKitBridge }).__semioKitBridge;
    const kit = c ? asKitInstance(c.getDto() as any) : this._kit;
    return { kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) { this.listeners.add(onChange); return () => { this.listeners.delete(onChange); }; }
  replace(kit: Kit) { this._kit = kit; for (const l of this.listeners) { try { l(); } catch { /* ignore */ } } }
}

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

export class JsonFileKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal */
  readonly name = "JsonFileKitStore";
  private constructor(private readonly adapter: KitJsonFileAdapter, seed: Kit) { this._kit = seed; }
  static async create(adapter: KitJsonFileAdapter) {
    const json = await adapter.read();
    const seed = json.trim() === "" ? asKitInstance({ id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }) : Kit.fromPlain(KitFullDtoSchema.parse(JSON.parse(json)));
    return new JsonFileKitStore(adapter, seed);
  }
  getSnapshot(): KitHostStoreSnapshot {
    const c = (this as JsonFileKitStore & { __semioKitBridge?: SemioKitBridge }).__semioKitBridge;
    const kit = c ? asKitInstance(c.getDto() as any) : this._kit;
    return { kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) { this.listeners.add(onChange); return () => { this.listeners.delete(onChange); }; }
  replace(kit: Kit) { this._kit = kit; for (const l of this.listeners) l(); void this.adapter.write(JSON.stringify(kit.toJSON())); }
}

export class FolderKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal */
  readonly name = "FolderKitStore";
  private constructor(private readonly adapter: KitFolderAdapter, seed: Kit) { this._kit = seed; }
  static async create(adapter: KitFolderAdapter, initial?: KitFullDto) {
    const bytes = await adapter.readKit();
    if (bytes != null && bytes.length > 0) {
      try { const t = new TextDecoder().decode(bytes); return new FolderKitStore(adapter, Kit.fromPlain(KitFullDtoSchema.parse(JSON.parse(t)))); } catch { /* fall through */ }
    }
    return new FolderKitStore(adapter, asKitInstance(initial ?? { id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }));
  }
  getSnapshot(): KitHostStoreSnapshot {
    const c = (this as FolderKitStore & { __semioKitBridge?: SemioKitBridge }).__semioKitBridge;
    const kit = c ? asKitInstance(c.getDto() as any) : this._kit;
    return { kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) { this.listeners.add(onChange); return () => { this.listeners.delete(onChange); }; }
  replace(kit: Kit) { this._kit = kit; for (const l of this.listeners) l(); void (async () => { try { const enc = new TextEncoder().encode(JSON.stringify(kit.toJSON())); await this.adapter.writeKit(enc); } catch { /* ignore */ } })(); }
}

export async function createJsonFileKitStore(adapter: KitJsonFileAdapter) { return await JsonFileKitStore.create(adapter); }
export async function createFolderKitStore(adapter: KitFolderAdapter, initial?: KitFullDto) { return await FolderKitStore.create(adapter, initial); }

export type SessionKitStoreConfig = { serverUrl: string; sessionId?: string; kitName?: string; personId?: string; clientId?: string; authToken?: string; readOnly?: boolean };
/** @emoji 🧭 Placeholder session store: in-memory until hub sync is host-wired. */
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
    upload: async (k, f, p, blob) => { storage.set(key(k, f, p), blob); return `memory://${key(k, f, p)}`; },
    download: async (k, f, p) => { const b = storage.get(key(k, f, p)); if (!b) throw new Error(`missing ${key(k, f, p)}`); return b; },
    delete: async (k, f, p) => { storage.delete(key(k, f, p)); },
    getUrl: (k, f, p) => `memory://${key(k, f, p)}`,
  };
};

/** @emoji 🧾 Async resolve + cache; warms {@link getExistingKitFileProvider} after first await. */
export async function getKitFileProvider(kitStore: KitHostStore, kitId: string): Promise<KitFileProvider> {
  const st = getOrCreateKitFileState(kitStore);
  st._cachedProviderByKitId = st._cachedProviderByKitId ?? new Map();
  const hit = st._cachedProviderByKitId.get(kitId);
  if (hit) { st._lastSyncProvider = hit; return hit; }
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
  if (prev) { try { URL.revokeObjectURL(prev); } catch { /* ignore */ } }
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
export type KitDiff = z.infer<typeof KitDiffSchema>;
// #endregion Kit

// #region KitImportHelpers
/** @emoji 🧾 Decode kit bytes as JSON DTO (host handles archives before calling). */
export function importKitToPlain(buf: ArrayBuffer | Uint8Array): KitFullDto {
  const u8 = buf instanceof Uint8Array ? buf : new Uint8Array(buf);
  const text = new TextDecoder().decode(u8);
  return KitFullDtoSchema.parse(JSON.parse(text));
}
// #endregion KitImportHelpers

// #region EntityStoreStubs
/** @emoji 🧭 Sketchpad app-store scaffolding; kit graph commands use {@link KitStoreClient} only. */
export class KitEntityStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class DesignStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class TypeStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class PieceStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class ConnectionStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class FamilyStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class FileStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
export class FolderStore {
  constructor(
    public readonly parent: unknown,
    public readonly entityId: unknown,
    public readonly state: unknown,
  ) {}
}
// #endregion EntityStoreStubs
// #endregion 🧩KitEntitiesMerged
