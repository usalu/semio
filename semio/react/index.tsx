// #region ⚛️Header

// Standalone React hooks bundle for semio.

// #endregion ⚛️Header

// #region ⚛️Imports

import {
  applyKitClientSnapshotToLocalStore,
  AuthorSchema,
  buildSchemaEntityChangeCommands,
  ConceptSchema,
  ConnectionSchema,
  ConnectorSchema,
  createFolderKitStore,
  createJsonFileKitStore,
  createKitStoreClient,
  createSessionKitStore,
  getKitClientReadScope,
  kitReadScopeKey,
  kitStoreFromKitStoreClient,
  normalizeKitFullDtoFolderPaths,
  theKitReadScope,
  DesignMetadataDtoSchema,
  DesignSchema,
  DesignShallowSchema,
  FamilySchema,
  FileSchema,
  FolderSchema,
  isKitCommandLifecycleEvent,
  KIT_RENAME_STATUS_IDLE,
  kitEventAffectsCanUndoRedo,
  kitEventAffectsDesignQualitySumRead,
  kitEventAffectsKitColoredConnectorsRead,
  kitEventAffectsPieceLiveRead,
  kitEventAffectsReplaceableCatalogRead,
  kitEventAffectsTypeScopedRead,
  kitEventTouchesDesign,
  resolveDesignIdForPieceOrConnection,
  submitKitChangeCommands,
  kitStoreClientAddChildByKind,
  kitStoreClientAddConnection,
  kitStoreClientAddPiece,
  kitStoreClientRemoveChildByKind,
  kitStoreClientRemovePiece,
  kitStoreClientUpdateConnection,
  kitStoreClientUpdatePiece,
  type JsonValue,
  type KitDesignReadKind,
  type KitFullDto,
  type KitShallowListKind,
  type KitStoreReadSnap,
  type KitViewCatalogKey,
  type WriteStatus,
  writeKitStoreClientSchemaField,
  getSemioKitLiveReadStore,
} from "@semio/js";
import {
  asKitInstance,
  Attribute,
  Author,
  Camera,
  Concept,
  Connection,
  ConnectionStore,
  Coordinate,
  createKitFileObjectUrl,
  Design,
  DesignStore,
  DiffStatus,
  FamilyStore,
  fetchReadableKitFileBlob,
  File,
  File as SemioFile,
  FileStore,
  Folder,
  FolderStore,
  getExistingKitFileProvider,
  getKitFileProvider,
  getKitFileStoragePath,
  getKitPorts,
  getOrCreateKitFileState,
  getReadableKitFileUrl,
  getStoredKitFileUrls,
  ICON_WIDTH,
  id,
  InMemoryKitStore,
  isBrowserReadableFileUrl,
  Kit,
  KitEntityStore,
  Piece,
  PieceSchema,
  PieceStore,
  Plane,
  Point,
  PropSchema,
  Quality,
  QualitySchema,
  Representation,
  RepresentationSchema,
  Tag,
  TagSchema,
  TOLERANCE,
  Type,
  TypeMetadataDtoSchema,
  TypeSchema,
  TypeShallowSchema,
  TypeStore,
  Vector,
} from "@semio/js";
import type {
  BackboneConfig,
  BackboneStatusDto,
  ChangeKitCommand,
  ConflictResolution,
  ConnectionDiff,
  ConnectionIdDto,
  ConnectionPlain,
  DesignDiff,
  DesignIdDto,
  DesignMetadataDto,
  DesignPlain,
  DesignShallow,
  KitBinaryStore,
  KitConflict,
  KitEvent,
  KitFileState,
  KitFolderAdapter,
  KitChildEntityKind,
  KitHostStore,
  KitHostStoreSnapshot,
  KitJsonFileAdapter,
  KitLike,
  KitReadScope,
  KitStoreClient,
  KitWriteScope,
  PieceDiff,
  PieceIdDto,
  PiecePlain,
  PiecePlacementRowDto,
  PlanePlain,
  KitJsonTreeDto,
  SetError,
  SetResult,
  TypeDiff,
  TypeIdDto,
  TypeMetadataDto,
  TypePlain,
  TypeShallow,
} from "@semio/js";
import type { ReactNode, SetStateAction } from "react";
import * as React from "react";

// #endregion ⚛️Imports

// #region 🔖KitHostCommandDispatch
/** @emoji 🪪 Authoritative kit DTO from a host store (plain `kit` snapshots may omit class `toJSON`). */
function __kitHostPlainDtoFromStore(store: KitHostStore): KitFullDto {
  const snapKit = store.getSnapshot().kit as KitLike;
  if (typeof (snapKit as { toJSON?: unknown }).toJSON === "function") return (snapKit as Kit).toJSON();
  return asKitInstance(snapKit).toJSON();
}

/** @emoji 🪪 Resolves {@link FolderKitStore} adapter when present (runtime field on class instance). */
function __kitFolderAdapter(store: KitHostStore): KitFolderAdapter | null {
  const n = String((store as { name?: string }).name ?? "");
  if (n !== "FolderKitStore") return null;
  const a = (store as { adapter?: KitFolderAdapter }).adapter;
  return a ?? null;
}

function __kitFolderChainPath(dto: KitFullDto, folderId: string | undefined): string {
  if (!folderId) return "";
  const folders = (dto.folders ?? []) as Array<{ id?: string; path?: string; name?: string; parent?: { id?: string } }>;
  const byId = new Map(folders.map((f) => [String(f.id), f]));
  const segs: string[] = [];
  let cur: string | undefined = folderId;
  const visiting = new Set<string>();
  while (cur) {
    if (visiting.has(cur)) break;
    visiting.add(cur);
    const f = byId.get(cur);
    if (!f) break;
    segs.unshift(String(f.name ?? f.id ?? cur));
    const pid = f.parent?.id != null ? String(f.parent.id) : "";
    cur = pid || undefined;
  }
  return segs.join("/");
}

/** @emoji 🪪 Relative folder path for adapter I/O (prefer materialized `path` from {@link normalizeKitFullDtoFolderPaths}). */
function __kitFolderStorageRelPath(dto: KitFullDto, folderId: string | undefined): string {
  if (!folderId) return "";
  const folders = (dto.folders ?? []) as Array<{ id?: string; path?: string; name?: string; parent?: { id?: string } }>;
  const f = folders.find((x) => String(x.id) === folderId);
  if (!f) return folderId;
  if (typeof f.path === "string" && f.path.length > 0) return f.path.replace(/\\/g, "/");
  return __kitFolderChainPath(dto, folderId);
}

function __kitFileRelPath(dto: KitFullDto, file: { name?: string; folder?: { id?: string } }): string {
  const base = file.folder?.id ? __kitFolderStorageRelPath(dto, String(file.folder.id)) : "";
  const n = String(file.name ?? "file");
  return base ? `${base}/${n}` : n;
}

function __kitSubfolderRelPath(dto: KitFullDto, folderId: string): string {
  return __kitFolderStorageRelPath(dto, folderId);
}

/** @emoji 🪪 Normalizes {@link Id} handles and plain strings for kit graph RPCs. */
function __kitHostIdStr(x: unknown): string {
  if (x == null) return "";
  if (typeof x === "string") return x;
  if (typeof x === "object" && x !== null && "id" in x) return String((x as { id: unknown }).id);
  return String(x);
}

function __kitHostBridge(store: KitHostStore): KitStoreClient | undefined {
  return (store as KitHostStore & { __semioKitBridge?: KitStoreClient }).__semioKitBridge;
}

/**
 * @emoji 📣 Typed kit graph side-effect for kit/design apps (replaces string `kitWire`).
 * Sketchpad MUST use this shape instead of `{ command: string; args: unknown[] }`.
 */
export type KitHostGraphOp =
  | { op: "deleteKitSelection"; typeIds: readonly TypeIdDto[]; designIds: readonly DesignIdDto[] }
  | { op: "addKitChildType"; body: TypePlain }
  | { op: "addKitChildTypes"; bodies: readonly TypePlain[] }
  | { op: "removeKitType"; id: TypeIdDto }
  | { op: "removeKitTypes"; ids: readonly TypeIdDto[] }
  | { op: "addKitChildDesign"; body: DesignPlain }
  | { op: "addKitChildDesigns"; bodies: readonly DesignPlain[] }
  | { op: "removeKitDesign"; id: DesignIdDto }
  | { op: "removeKitDesigns"; ids: readonly DesignIdDto[] }
  | { op: "setEntityPatch"; entity: "Type"; id: TypeIdDto; patch: TypeDiff } | { op: "setEntityPatch"; entity: "Design"; id: DesignIdDto; patch: DesignDiff }
  | { op: "patchTypes"; updates: readonly { type: TypeIdDto; diff: TypeDiff }[] }
  | { op: "patchDesigns"; updates: readonly { design: DesignIdDto; diff: DesignDiff }[] }
  | { op: "addDesignPiece"; designId: DesignIdDto; piece: PiecePlain }
  | { op: "addDesignPieces"; designId: DesignIdDto; pieces: readonly PiecePlain[] }
  | { op: "removeDesignPiece"; designId: DesignIdDto; pieceId: PieceIdDto }
  | { op: "removeDesignPieces"; designId: DesignIdDto; pieceIds: readonly PieceIdDto[] }
  | { op: "addDesignConnection"; designId: DesignIdDto; connection: ConnectionPlain }
  | { op: "addDesignConnections"; designId: DesignIdDto; connections: readonly ConnectionPlain[] }
  | { op: "deleteConnection"; designId: DesignIdDto; connectionId: ConnectionIdDto }
  | { op: "deleteConnections"; designId: DesignIdDto; connectionIds: readonly ConnectionIdDto[] }
  | { op: "patchPiece"; designId: DesignIdDto; pieceId: PieceIdDto; diff: PieceDiff }
  | { op: "patchPieces"; designId: DesignIdDto; updates: readonly { piece: PieceIdDto; diff: PieceDiff }[] }
  | { op: "patchConnection"; designId: DesignIdDto; connectionId: ConnectionIdDto; diff: ConnectionDiff }
  | { op: "patchConnectionMany"; designId: DesignIdDto; rows: readonly { id: string; diff: ConnectionDiff }[] }
  | { op: "clusterPieces"; designId: DesignIdDto; pieceIds: readonly PieceIdDto[]; clusterName: string }
  | { op: "expandDesign"; parentDesignId: DesignIdDto; nestedDesignId: DesignIdDto }
  | { op: "removeDesignPiecesAndConnections"; designId: DesignIdDto; pieceIds: readonly PieceIdDto[]; connectionIds: readonly ConnectionIdDto[] };

/** @emoji 🧾 Applies {@link KitHostGraphOp} through the live {@link KitStoreClient} bridge. */
export async function applyKitHostGraphOp(host: KitHostStore, op: KitHostGraphOp): Promise<SetResult> {
  const bridge = __kitHostBridge(host);
  if (!bridge) return { ok: false, error: { kind: "Internal", message: "applyKitHostGraphOp: no kit bridge" } };
  switch (op.op) {
    case "deleteKitSelection": {
      for (const t of op.typeIds) {
        const r = await kitStoreClientRemoveChildByKind(bridge, "Type", __kitHostIdStr(t));
        if (!r.ok) return r;
      }
      for (const d of op.designIds) {
        const r = await kitStoreClientRemoveChildByKind(bridge, "Design", __kitHostIdStr(d));
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "addKitChildType":
      return kitStoreClientAddChildByKind(bridge, "Type", op.body);
    case "addKitChildTypes": {
      for (const b of op.bodies) {
        const r = await kitStoreClientAddChildByKind(bridge, "Type", b);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "removeKitType":
      return kitStoreClientRemoveChildByKind(bridge, "Type", __kitHostIdStr(op.id));
    case "removeKitTypes": {
      for (const x of op.ids) {
        const r = await kitStoreClientRemoveChildByKind(bridge, "Type", __kitHostIdStr(x));
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "addKitChildDesign":
      return kitStoreClientAddChildByKind(bridge, "Design", op.body);
    case "addKitChildDesigns": {
      for (const b of op.bodies) {
        const r = await kitStoreClientAddChildByKind(bridge, "Design", b);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "removeKitDesign":
      return kitStoreClientRemoveChildByKind(bridge, "Design", __kitHostIdStr(op.id));
    case "removeKitDesigns": {
      for (const x of op.ids) {
        const r = await kitStoreClientRemoveChildByKind(bridge, "Design", __kitHostIdStr(x));
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "setEntityPatch": {
      const id = __kitHostIdStr(op.id);
      const patch = op.patch && typeof op.patch === "object" ? (op.patch as Record<string, unknown>) : {};
      for (const [k, v] of Object.entries(patch)) {
        if (v === undefined) continue;
        const r = await writeKitStoreClientSchemaField(bridge, op.entity, k, v, id);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "patchTypes": {
      for (const u of op.updates) {
        const id = __kitHostIdStr(u.type);
        const patch = u.diff && typeof u.diff === "object" ? (u.diff as Record<string, unknown>) : {};
        for (const [k, v] of Object.entries(patch)) {
          if (v === undefined) continue;
          const r = await writeKitStoreClientSchemaField(bridge, "Type", k, v, id);
          if (!r.ok) return r;
        }
      }
      return { ok: true };
    }
    case "patchDesigns": {
      for (const u of op.updates) {
        const id = __kitHostIdStr(u.design);
        const patch = u.diff && typeof u.diff === "object" ? (u.diff as Record<string, unknown>) : {};
        for (const [k, v] of Object.entries(patch)) {
          if (v === undefined) continue;
          const r = await writeKitStoreClientSchemaField(bridge, "Design", k, v, id);
          if (!r.ok) return r;
        }
      }
      return { ok: true };
    }
    case "addDesignPiece":
      return kitStoreClientAddPiece(bridge, __kitHostIdStr(op.designId), op.piece);
    case "addDesignPieces": {
      for (const p of op.pieces) {
        const r = await kitStoreClientAddPiece(bridge, __kitHostIdStr(op.designId), p);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "removeDesignPiece":
      return kitStoreClientRemovePiece(bridge, __kitHostIdStr(op.designId), __kitHostIdStr(op.pieceId));
    case "removeDesignPieces": {
      for (const p of op.pieceIds) {
        const r = await kitStoreClientRemovePiece(bridge, __kitHostIdStr(op.designId), __kitHostIdStr(p));
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "addDesignConnection":
      return kitStoreClientAddConnection(bridge, __kitHostIdStr(op.designId), op.connection);
    case "addDesignConnections": {
      for (const c of op.connections) {
        const r = await kitStoreClientAddConnection(bridge, __kitHostIdStr(op.designId), c);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "deleteConnection":
      return bridge.deleteConnection(__kitHostIdStr(op.designId), __kitHostIdStr(op.connectionId));
    case "deleteConnections": {
      for (const c of op.connectionIds) {
        const r = await bridge.deleteConnection(__kitHostIdStr(op.designId), __kitHostIdStr(c));
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "patchPiece":
      return kitStoreClientUpdatePiece(bridge, __kitHostIdStr(op.designId), __kitHostIdStr(op.pieceId), op.diff);
    case "patchPieces": {
      const did = __kitHostIdStr(op.designId);
      for (const u of op.updates) {
        const pid = __kitHostIdStr(u.piece);
        const r = await kitStoreClientUpdatePiece(bridge, did, pid, u.diff);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "patchConnection":
      return kitStoreClientUpdateConnection(bridge, __kitHostIdStr(op.designId), __kitHostIdStr(op.connectionId), op.diff);
    case "patchConnectionMany": {
      for (const row of op.rows) {
        const r = await kitStoreClientUpdateConnection(bridge, __kitHostIdStr(op.designId), row.id, row.diff);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    case "clusterPieces":
      return bridge.clusterPieces(
        __kitHostIdStr(op.designId),
        op.pieceIds.map((x) => __kitHostIdStr(x)),
        op.clusterName,
      );
    case "expandDesign":
      return bridge.expandDesign(__kitHostIdStr(op.parentDesignId), __kitHostIdStr(op.nestedDesignId));
    case "removeDesignPiecesAndConnections": {
      const did = __kitHostIdStr(op.designId);
      for (const c of op.connectionIds) {
        const r = await bridge.deleteConnection(did, __kitHostIdStr(c));
        if (!r.ok) return r;
      }
      for (const p of op.pieceIds) {
        const r = await kitStoreClientRemovePiece(bridge, did, __kitHostIdStr(p));
        if (!r.ok) return r;
      }
      return { ok: true };
    }
  }
  return { ok: false, error: { kind: "Internal", message: "applyKitHostGraphOp: unreachable" } };
}

/** @emoji 🧾 VCS undo via the optional {@link KitStoreClient} bridge (typed alternative to `executeSemioKitCommand` undo). */
export async function kitHostUndo(store: KitHostStore): Promise<SetResult> {
  const bridge = __kitHostBridge(store);
  if (!bridge) return { ok: false, error: { kind: "Internal", message: "kitHostUndo: no kit bridge" } };
  return bridge.undo();
}

/** @emoji 🧾 VCS redo via the optional {@link KitStoreClient} bridge (typed alternative to `executeSemioKitCommand` redo). */
export async function kitHostRedo(store: KitHostStore): Promise<SetResult> {
  const bridge = __kitHostBridge(store);
  if (!bridge) return { ok: false, error: { kind: "Internal", message: "kitHostRedo: no kit bridge" } };
  return bridge.redo();
}

/** @emoji 🧾 String-command entry retained for sketchpad host flows; prefer {@link applyKitHostGraphOp} for graph edits. */
export async function executeSemioKitCommand(store: KitHostStore, command: string, _origin: string, ...args: unknown[]): Promise<unknown> {
  const bridge = __kitHostBridge(store);
  if (command === "semio.kit.undo") {
    if (!bridge) return { ok: false, error: "no kit bridge" };
    return bridge.undo();
  }
  if (command === "semio.kit.redo") {
    if (!bridge) return { ok: false, error: "no kit bridge" };
    return bridge.redo();
  }
  if (command === "semio.kit.addFile" && args[0]) {
    const file = FileSchema.parse(args[0]);
    const blobArg = args[1];
    if (bridge) return bridge.submitChangeKitCommands([{ addFile: { file } }]);
    const snap = __kitHostPlainDtoFromStore(store);
    const nextFiles = [...((snap.files as unknown[]) ?? []), file as unknown];
    const merged = normalizeKitFullDtoFolderPaths({ ...(snap as object), files: nextFiles } as unknown as KitFullDto);
    store.replace(Kit.fromDto(merged));
    const adapter = __kitFolderAdapter(store);
    if (adapter && typeof Blob !== "undefined" && blobArg instanceof Blob) {
      try {
        const rel = __kitFileRelPath(merged, file as { name?: string; folder?: { id?: string } });
        await adapter.writeFile(rel, blobArg);
      } catch {
        /* ignore */
      }
    }
    return { ok: true };
  }
  if (command === "semio.kit.import") {
    void _origin;
    return { ok: false, error: "semio.kit.import: not wired in this build" };
  }
  if (command === "semio.kit.export") {
    return { ok: true };
  }
  if (command === "semio.kit.patchQuality" && args[0] != null && args[1]) {
    if (!bridge) return { ok: false, error: { kind: "Internal", message: "no kit bridge" } };
    const qid = __kitHostIdStr(args[0]);
    const patch = args[1] && typeof args[1] === "object" ? (args[1] as Record<string, unknown>) : {};
    for (const [k, v] of Object.entries(patch)) {
      if (v === undefined) continue;
      const r = await writeKitStoreClientSchemaField(bridge, "Quality", k, v, qid);
      if (!r.ok) return r;
    }
    return { ok: true };
  }
  if ((command === "semio.kit.addChildType" || command === "semio.kit.createType") && args[0]) {
    if (!bridge) return { ok: false, error: "no kit bridge" };
    return kitStoreClientAddChildByKind(bridge, "Type", args[0]);
  }
  if (command === "semio.kit.moveToFolder" && args.length >= 3) {
    const entityId = __kitHostIdStr(args[0]);
    const kind = String(args[1] ?? "");
    const folderId = __kitHostIdStr(args[2]);
    const beforeDto = __kitHostPlainDtoFromStore(store);
    const plain = JSON.parse(JSON.stringify(beforeDto)) as Record<string, unknown>;
    if (kind === "type") {
      const rows = (plain.types as unknown[] | undefined) ?? [];
      for (const t of rows) {
        if (t && typeof t === "object" && (t as { id?: string }).id === entityId) (t as { folder?: string }).folder = folderId;
      }
    } else if (kind === "design") {
      const rows = (plain.designs as unknown[] | undefined) ?? [];
      for (const d of rows) {
        if (d && typeof d === "object" && (d as { id?: string }).id === entityId) (d as { folder?: string }).folder = folderId;
      }
    } else if (kind === "quality") {
      const rows = (plain.qualities as unknown[] | undefined) ?? [];
      for (const q of rows) {
        if (q && typeof q === "object" && (q as { id?: string }).id === entityId) (q as { folder?: string }).folder = folderId;
      }
    } else if (kind === "file") {
      const rows = (plain.files as unknown[] | undefined) ?? [];
      for (const f of rows) {
        if (f && typeof f === "object" && (f as { id?: string }).id === entityId) (f as { folder?: { id: string } }).folder = { id: folderId };
      }
    } else if (kind === "folder") {
      const rows = (plain.folders as unknown[] | undefined) ?? [];
      for (const fo of rows) {
        if (fo && typeof fo === "object" && (fo as { id?: string }).id === entityId) {
          (fo as { parent?: { id: string } }).parent = { id: folderId };
          delete (fo as { path?: string }).path;
        }
      }
    } else {
      return { ok: false, error: { kind: "InvalidValue", message: `moveToFolder: unknown kind ${kind}` } };
    }
    const afterDto = asKitInstance(plain as KitLike).toJSON();
    const adapter = __kitFolderAdapter(store);
    if (adapter) {
      try {
        if (kind === "file") {
          const filesBefore = (beforeDto.files ?? []) as Array<{ id?: string; name?: string; folder?: { id?: string } }>;
          const fileBefore = filesBefore.find((f) => String(f.id) === entityId);
          if (fileBefore) {
            const fromP = __kitFileRelPath(beforeDto, fileBefore);
            const fileAfter = ((afterDto.files ?? []) as typeof filesBefore).find((f) => String(f.id) === entityId);
            if (fileAfter) {
              const toP = __kitFileRelPath(afterDto, fileAfter);
              if (fromP !== toP) await adapter.moveEntry(fromP, toP);
            }
          }
        } else if (kind === "folder") {
          const fromP = __kitSubfolderRelPath(beforeDto, entityId);
          const toP = __kitSubfolderRelPath(afterDto, entityId);
          if (fromP !== toP) await adapter.moveEntry(fromP, toP);
        }
      } catch {
        /* ignore */
      }
    }
    store.replace(asKitInstance(plain as never));
    return { ok: true };
  }
  if (command === "semio.kit.createFolder" && args[0]) {
    if (bridge) return kitStoreClientAddChildByKind(bridge, "Folder", args[0]);
    const snap = __kitHostPlainDtoFromStore(store);
    const nextFolders = [...((snap.folders as unknown[]) ?? []), args[0] as Record<string, unknown>];
    const merged = normalizeKitFullDtoFolderPaths({ ...(snap as object), folders: nextFolders } as unknown as KitFullDto);
    store.replace(Kit.fromDto(merged));
    const adapter = __kitFolderAdapter(store);
    if (adapter) {
      try {
        const nf = args[0] as { id?: string; name?: string; parent?: { id?: string } };
        const rel = __kitSubfolderRelPath(merged, String(nf.id ?? ""));
        if (rel) await (adapter.createDirectory as (path: string) => Promise<void>)(rel);
      } catch {
        /* ignore */
      }
    }
    return { ok: true };
  }
  if (command === "semio.kit.updateFolder" && args[0] && args[1]) {
    if (bridge) {
      const fid = __kitHostIdStr(args[0]);
      const patch = args[1] as Record<string, unknown>;
      for (const [k, v] of Object.entries(patch)) {
        const r = await writeKitStoreClientSchemaField(bridge, "Folder", k, v, fid);
        if (!r.ok) return r;
      }
      return { ok: true };
    }
    const fid = __kitHostIdStr(args[0]);
    const patch = args[1] as Record<string, unknown>;
    const snap = __kitHostPlainDtoFromStore(store);
    const folders = ((snap.folders as unknown[]) ?? []).map((row) => {
      if (row && typeof row === "object" && (row as { id?: string }).id === fid) {
        const mergedRow = { ...(row as object), ...patch } as Record<string, unknown>;
        delete mergedRow.path;
        return mergedRow;
      }
      return row;
    });
    const merged = normalizeKitFullDtoFolderPaths({ ...(snap as object), folders } as unknown as KitFullDto);
    const adapter = __kitFolderAdapter(store);
    if (adapter) {
      try {
        const fromP = __kitSubfolderRelPath(snap, fid);
        const toP = __kitSubfolderRelPath(merged, fid);
        if (fromP !== toP) await adapter.moveEntry(fromP, toP);
      } catch {
        /* ignore */
      }
    }
    store.replace(Kit.fromDto(merged));
    return { ok: true };
  }
  return { ok: false, error: { kind: "NotSupported", message: `unhandled ${command}` } };
}

/** @emoji 🧾 Memoized kit string-command engine for legacy callers (sketchpad). */
export function createKitCommandEngineExplicitOrigin(store: KitHostStore): { execute: (...args: unknown[]) => Promise<unknown> } {
  return {
    execute: async (command: unknown, origin: unknown, ...rest: unknown[]) =>
      executeSemioKitCommand(store, String(command), String(origin ?? ""), ...rest),
  };
}

/** @emoji 🧾 Default {@link createKitCommandEngineExplicitOrigin} wrapper. */
export function createKitCommandEngine(store: KitHostStore): ReturnType<typeof createKitCommandEngineExplicitOrigin> {
  return createKitCommandEngineExplicitOrigin(store);
}
// #endregion 🔖KitHostCommandDispatch

export type { BackboneConfig, BackboneStatusDto, ConflictResolution, KitConflict, KitReadScope, KitWriteScope, SetError, SetResult } from "@semio/js";
export { getKitClientReadScope, kitReadScopeKey, kitStoreFromKitStoreClient, theKitReadScope } from "@semio/js";
export type { KitBinaryStore, KitFileState } from "@semio/js";
export type { KitHostStore, KitHostStoreSnapshot } from "@semio/js";
export type {
  KitStoreExecuteResult,
  KitDesignReadKind,
  KitShallowListKind,
  KitStoreReadSnap,
  KitViewCatalogKey,
  WriteStatus,
} from "@semio/js";
export { DesignStore, TypeStore, PieceStore, ConnectionStore, FamilyStore, FileStore, FolderStore, KitEntityStore } from "@semio/js";
export {
  SemioKitDesignReadStore,
  SemioKitLiveReadStore,
  SemioKitShallowListReadStore,
  SemioKitViewStore,
  getSemioKitDesignReadStore,
  getSemioKitLiveReadStore,
  getSemioKitShallowListReadStore,
  getSemioKitViewStore,
} from "@semio/js";

// #region ⚛️Types

// Live-read snapshot hub is implemented in `semio/js` (`getSemioKitLiveReadStore`); hooks use `useSyncExternalStore` here.

export type HookTriad<T> = readonly [T, (next: SetStateAction<T>) => Promise<SetResult>, WriteStatus];
/** Read-only async-backed value + {@link WriteStatus} (no setter). */
export type HookRead<T> = readonly [T | undefined, WriteStatus];

export type SchemaPropertyEvent = {
  key: string;
  typeName: string;
  fieldName: string;
  id?: string;
  previous: unknown;
  current: unknown;
  requestId?: string;
  commandKind?: string;
  phase?: string;
};

export type MemoryBackboneConfig = {
  kind?: "memory";
  initialKit?: KitLike;
};

export type DevBackboneConfig = {
  kind: "dev";
  filePath: string;
};

export type LocalBackboneConfig = {
  kind: "local";
  folderPath: string;
};

export type RemoteBackboneConfig = {
  kind: "remote";
  serverUrl: string;
  sessionId?: string;
  kitName?: string;
  personId?: string;
  clientId?: string;
  authToken?: string;
  readOnly?: boolean;
};

/** How a kit is opened (memory, dev JSON, local folder, remote) — for {@link KitScope} `backbone` / registry. */
export type KitBackboneConfig = MemoryBackboneConfig | DevBackboneConfig | LocalBackboneConfig | RemoteBackboneConfig;

type IndexedSchemaReference = {
  typeName: string;
  id?: string;
  path: Array<string | number>;
  value: any;
};

type IndexedSchemaState = {
  plain: any;
  kit: Kit;
  kitId?: string;
  byId: Map<string, IndexedSchemaReference[]>;
  byType: Map<string, IndexedSchemaReference[]>;
};

export type SchemaScope = {
  typeName: string;
  id?: string;
  path: Array<string | number>;
};

export type KitRuntimeContextValue = {
  store: KitHostStore;
  snapshot: KitHostStoreSnapshot;
  state: IndexedSchemaState;
  recentEvents: SchemaPropertyEvent[];
  recentSetRejections: SetError[];
  pushSetRejection: (e: SetError) => void;
  canWrite: boolean;
  /** Active kit id: {@link KitScope} `kitId` when set, otherwise `snapshot.kit.id`. */
  kitId?: string;
  /** Optional open/backbone config when the kit is not from {@link KitRegistryProvider}. */
  kitBackbone?: KitBackboneConfig;
  kitClient: KitStoreClient | null;
  setFieldValue: (typeName: string, fieldName: string, next: SetStateAction<any>, id?: string, scope?: SchemaScope | null) => Promise<SetResult>;
  setObjectValue: (typeName: string, next: SetStateAction<any>, id?: string, scope?: SchemaScope | null) => Promise<SetResult>;
};

/** @emoji 📌Persistence metadata for an open kit in the registry. */
export type KitPersistenceInfo = { kind: "temporary" | "file" | "folder" | "remote"; path?: string; url?: string };

/** @emoji 📌Desktop / webview: inject folder/file/session kit store creation (shell owns I/O). */
export type SketchpadKitStoreFactory = (kit: Kit) => KitHostStore | Promise<KitHostStore>;

export type SketchpadKitKindAvailability = {
  temporary: boolean;
  file: boolean;
  folder: boolean;
  remote: boolean;
};

// #endregion ⚛️Types

// #region ⚛️Constants

const ROOT_COLLECTION_TYPE_BY_KEY: Record<string, string> = {
  types: "Type",
  designs: "Design",
  tags: "Tag",
  concepts: "Concept",
  families: "Family",
  ports: "Port",
  qualities: "Quality",
  files: "File",
  folders: "Folder",
  authors: "Author",
  pieces: "Piece",
  connections: "ConnectionStore",
  benchmarks: "Benchmark",
  representations: "RepresentationStore",
  connectors: "ConnectorStore",
  stats: "Stat",
  props: "Prop",
  layers: "Layer",
  groups: "Group",
  attributes: "Attribute",
  sessions: "KitSession",
  transactions: "KitTransaction",
  pendingCandidates: "KitChangeCandidate",
  activeConflicts: "KitConflict",
  activeTransactions: "KitTransaction",
  changes: "KitChange",
  undoStack: "KitChange",
  redoStack: "KitChange",
  votes: "KitCandidateVote",
  requestedFrom: "KitSession",
  actions: "SessionWarningAction",
  nodes: "KitHistoryEntry",
};

const NESTED_TYPE_BY_KEY: Record<string, string> = {
  plane: "Plane",
  mirrorPlane: "Plane",
  flatPlane: "Plane",
  center: "Coordinate",
  flatCenter: "Coordinate",
  offset: "Coordinate",
  origin: "Point",
  point: "Point",
  position: "Point",
  xAxis: "Vector",
  yAxis: "Vector",
  forward: "Vector",
  up: "Vector",
  direction: "Vector",
  connected: "Side",
  connecting: "Side",
  piece: "Piece",
  designPiece: "Piece",
  parentPiece: "Piece",
  childPiece: "Piece",
  parentConnection: "ConnectionStore",
  childConnections: "ConnectionStore",
  activeDesign: "Design",
  type: "Type",
  design: "Design",
  quality: "Quality",
  folder: "Folder",
  createdBy: "Author",
  updatedBy: "Author",
  port: "Port",
  connector: "ConnectorStore",
  childConnector: "ConnectorStore",
  parentConnector: "ConnectorStore",
  actor: "Actor",
  session: "KitSession",
  client: "KitClientInfo",
  warning: "KitSessionWarning",
  selection: "KitSessionSelection",
  validation: "KitValidationResult",
  candidate: "KitChangeCandidate",
  conflict: "KitConflict",
  change: "KitChange",
  transaction: "KitTransaction",
  store: "KitStore",
  history: "KitHistory",
  backbone: "KitBackbone",
  historyEntry: "KitHistoryEntry",
  export: "KitArchiveExport",
  pageInfo: "PageInfo",
};

const NEVER_WRITABLE_FIELDS = new Set([
  "hash",
  "kind",
  "flatPlane",
  "flatCenter",
  "parentPiece",
  "parentConnection",
  "childPieces",
  "childConnections",
  "alternatives",
  "alternativeTypes",
  "alternativeDesigns",
  "childPiece",
  "childConnector",
  "parentPiece",
  "parentConnector",
  "fixedPieces",
]);

// #endregion ⚛️Constants

// #region ⚛️Utilities

/** @emoji 🧾 `useSyncExternalStore` with a derived snapshot and a custom equality for fewer rerenders. */
export function useSemioStoreSelector<T, S>(
  store: { getSnapshot(): T; subscribe(onChange: () => void): () => void },
  select: (snap: T) => S,
  isEqual: (a: S, b: S) => boolean = (a, b) => Object.is(a, b),
): S {
  const last = React.useRef<{ snap: T; out: S } | null>(null);
  const get = React.useCallback((): S => {
    const snap = store.getSnapshot();
    const cached = last.current;
    if (cached && cached.snap === snap) return cached.out;
    const out = select(snap);
    if (cached && isEqual(cached.out, out)) {
      last.current = { snap, out: cached.out };
      return cached.out;
    }
    last.current = { snap, out };
    return out;
  }, [store, select, isEqual]);
  return React.useSyncExternalStore(store.subscribe, get, get);
}

function noop(): void {}

async function noopAsyncSet(_next?: unknown): Promise<SetResult> {
  return { ok: true } as const;
}

/** Stable {@link WriteStatus} for schema hooks outside {@link KitScope} (avoids React #520 from fresh object identity). */
const SCHEMA_HOOK_READONLY_STATUS = Object.freeze({ kind: "readonly" as const, pending: 0 });
const SCHEMA_HOOK_IDLE_STATUS = Object.freeze({ kind: "idle" as const, pending: 0 });

/** @emoji 🪪 Frozen pending for {@link useKitName} rust rename — stable identity while rename is in flight. */
const USE_KIT_NAME_PENDING_STATUS = Object.freeze({ kind: "pending" as const, pending: 1 });

/** @emoji 🪪 Same semantic {@link WriteStatus} → reuse prior reference (triads + `useWriteIndicator` subscribers). */
function writeStatusEquivalent(a: WriteStatus, b: WriteStatus): boolean {
  if (a === b) return true;
  if (a.kind !== b.kind) return false;
  if (a.kind === "pending" && b.kind === "pending") {
    return a.pending === b.pending && (a as { lastError?: SetError }).lastError === (b as { lastError?: SetError }).lastError;
  }
  if (a.kind === "error" && b.kind === "error") {
    return a.lastError === b.lastError;
  }
  return true;
}

function kitIdFromRuntime(runtime: KitRuntimeContextValue): string | null {
  const g = runtime.kitId ?? (runtime.snapshot as { kit?: { id?: string } }).kit?.id;
  return g != null && g !== "" ? String(g) : null;
}

function deepClone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value));
}

function deepEqual(a: any, b: any): boolean {
  if (a === b) return true;
  if (a == null || b == null) return a == null && b == null;
  if (typeof a !== typeof b) return false;
  if (Array.isArray(a)) {
    if (!Array.isArray(b) || a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!deepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  if (typeof a === "object") {
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;
    for (const key of keysA) {
      if (!deepEqual(a[key], b[key])) return false;
    }
    return true;
  }
  return false;
}

function getFieldDataKey(typeName: string, fieldName: string): string {
  if (fieldName === "id") return "id";
  if (typeName === "Kit" && fieldName === "release") return "version";
  return fieldName;
}

function getSchemaFieldName(typeName: string, dataKey: string): string {
  if (dataKey === "id") return "id";
  if (typeName === "Kit" && dataKey === "version") return "release";
  return dataKey;
}

function getByPath(root: any, path: Array<string | number>): any {
  let current = root;
  for (const segment of path) {
    if (current == null) return undefined;
    current = current[segment as any];
  }
  return current;
}

function setByPath(root: any, path: Array<string | number>, value: any): void {
  if (path.length === 0) return;
  const parent = getByPath(root, path.slice(0, -1));
  if (parent == null) return;
  parent[path[path.length - 1] as any] = value;
}

function inferTypeName(parentTypeName: string | undefined, key: string | undefined): string | undefined {
  if (!key) return parentTypeName;
  if (ROOT_COLLECTION_TYPE_BY_KEY[key]) return ROOT_COLLECTION_TYPE_BY_KEY[key];
  if (NESTED_TYPE_BY_KEY[key]) return NESTED_TYPE_BY_KEY[key];
  return parentTypeName;
}

function scanSchemaState(root: any): IndexedSchemaState {
  const byId = new Map<string, IndexedSchemaReference[]>();
  const byType = new Map<string, IndexedSchemaReference[]>();

  function push(ref: IndexedSchemaReference): void {
    if (ref.id) {
      const existing = byId.get(ref.id) ?? [];
      existing.push(ref);
      byId.set(ref.id, existing);
    }
    const existing = byType.get(ref.typeName) ?? [];
    existing.push(ref);
    byType.set(ref.typeName, existing);
  }

  function walk(value: any, path: Array<string | number>, typeName: string | undefined): void {
    if (value == null) return;
    if (Array.isArray(value)) {
      const collectionName = typeof path[path.length - 1] === "string" ? (path[path.length - 1] as string) : undefined;
      const childTypeName = inferTypeName(typeName, collectionName);
      value.forEach((entry, index) => walk(entry, [...path, index], childTypeName));
      return;
    }
    if (typeof value !== "object") return;
    const resolvedTypeName = typeName ?? "Kit";
    const idValue = typeof value.id === "string" ? value.id : undefined;
    push({ typeName: resolvedTypeName, id: idValue, path, value });
    for (const [key, entry] of Object.entries(value)) {
      walk(entry, [...path, key], inferTypeName(resolvedTypeName, key));
    }
  }

  walk(root, [], "Kit");

  return {
    plain: root,
    kit: asKitInstance(root),
    kitId: root?.id,
    byId,
    byType,
  };
}

function collectIds(value: any, target: Set<string>): void {
  if (value == null) return;
  if (Array.isArray(value)) {
    for (const entry of value) collectIds(entry, target);
    return;
  }
  if (typeof value !== "object") return;
  if (typeof value.id === "string") target.add(value.id);
  for (const entry of Object.values(value)) collectIds(entry, target);
}

function resolveReference(index: IndexedSchemaState, typeName: string, id?: string, scope?: SchemaScope | null): IndexedSchemaReference | undefined {
  if (typeName === "Kit") return index.byType.get("Kit")?.[0];
  if (id) {
    const matches = index.byId.get(id) ?? [];
    return matches.find((entry) => entry.typeName === typeName) ?? matches[0];
  }
  if (scope && scope.typeName === typeName) {
    return { typeName, id: scope.id, path: scope.path, value: getByPath(index.plain, scope.path) };
  }
  const typeMatches = index.byType.get(typeName) ?? [];
  if (typeMatches.length === 1) return typeMatches[0];
  return undefined;
}

function findLivePiece(kit: Kit, pieceId: string): { piece: Piece; design: Design } | undefined {
  for (const design of kit.designs ?? []) {
    const piece = design.pieces?.find((entry) => entry.id === pieceId);
    if (piece) return { piece, design };
  }
  return undefined;
}

function findLiveConnection(kit: Kit, connectionId: string): { connection: any; design: Design } | undefined {
  for (const design of kit.designs ?? []) {
    const connection = design._connections?.find((entry) => entry.id === connectionId);
    if (connection) return { connection, design };
  }
  return undefined;
}

function findLiveEntity(kit: Kit, typeName: string, id?: string): any {
  if (typeName === "Kit") return kit;
  if (!id) return undefined;
  if (typeName === "Piece") return findLivePiece(kit, id)?.piece;
  if (typeName === "ConnectionStore") return findLiveConnection(kit, id)?.connection;
  if (typeName === "Type") return kit.findType(id);
  if (typeName === "Design") return kit.findDesign(id);
  if (typeName === "Port") return getKitPorts(kit).find((entry) => entry.id === id);
  if (typeName === "Quality") return kit.qualities?.find((entry) => entry.id === id);
  if (typeName === "File") return kit.files?.find((entry) => entry.id === id);
  if (typeName === "Folder") return kit.folders?.find((entry) => entry.id === id);
  if (typeName === "Author") return kit.authors?.find((entry) => entry.id === id);
  if (typeName === "Tag") return kit.tags?.find((entry) => entry.id === id);
  if (typeName === "Concept") return kit.concepts?.find((entry) => entry.id === id);
  if (typeName === "Family") return kit.families?.find((entry) => entry.id === id);
  if (typeName === "RepresentationStore") {
    for (const entry of kit.types ?? []) {
      const match = entry.representations?.find((representation) => representation.id === id);
      if (match) return match;
    }
  }
  if (typeName === "ConnectorStore") {
    for (const entry of kit.types ?? []) {
      const match = entry.connectors?.find((connector) => connector.id === id);
      if (match) return match;
    }
  }
  if (typeName === "Benchmark") {
    for (const entry of kit.qualities ?? []) {
      const match = entry.benchmarks?.find((benchmark) => benchmark.id === id);
      if (match) return match;
    }
  }
  return undefined;
}

function readCustomFieldValue(state: IndexedSchemaState, typeName: string, fieldName: string, id?: string): any {
  if (typeName === "Kit" && fieldName === "release") return (state.kit as any).version;
  if (typeName === "Piece") {
    const found = id ? findLivePiece(state.kit, id) : undefined;
    if (!found) return undefined;
    const { piece, design } = found;
    if (fieldName === "kind") return piece.dtoDesignAsPieceId() ? "DESIGN" : piece.dtoTypeId() ? "TYPE" : undefined;
    if (fieldName === "flatPlane") return piece.flatPlane();
    if (fieldName === "flatCenter") return piece.flatCenter();
    if (fieldName === "parentPiece") {
      try {
        return state.kit.findParentPieceInDesign(design.id, piece.id);
      } catch {
        return undefined;
      }
    }
    if (fieldName === "parentConnection") {
      try {
        return state.kit.findParentConnectionForPieceInDesign(design.id, piece.id);
      } catch {
        return undefined;
      }
    }
    if (fieldName === "childPieces") {
      try {
        return state.kit.findChildrenPiecesInDesign(design.id, piece.id);
      } catch {
        return [];
      }
    }
    if (fieldName === "childConnections") {
      try {
        const metadata = state.kit.piecesMetadataFor(design.id);
        if (!metadata.ok || !metadata.diff) return [];
        return (design._connections ?? []).filter((connection) => {
          try {
            const connectedId = connection.connected.piece.id;
            const connectingId = connection.connecting.piece.id;
            if (connectedId === piece.id) return metadata.diff.get(connectingId)?.parentPieceId === piece.id;
            if (connectingId === piece.id) return metadata.diff.get(connectedId)?.parentPieceId === piece.id;
            return false;
          } catch {
            return false;
          }
        });
      } catch {
        return [];
      }
    }
    if (fieldName === "alternativeTypes") return piece.alternativeTypes();
    if (fieldName === "alternativeDesigns") {
      const alt = piece.design as (Design & { getDesignFamily?: () => readonly { id: string }[] }) | undefined;
      if (!alt || typeof alt.getDesignFamily !== "function") return [];
      try {
        return alt.getDesignFamily!().filter((entry) => entry.id !== alt.id);
      } catch {
        return [];
      }
    }
    if (fieldName === "alternatives") {
      return [...(piece.alternativeTypes() ?? []).map((entry) => ({ type: entry, design: undefined })), ...(readCustomFieldValue(state, typeName, "alternativeDesigns", id) ?? []).map((entry: any) => ({ type: undefined, design: entry }))];
    }
  }
  if (typeName === "ConnectionStore") {
    const found = id ? findLiveConnection(state.kit, id) : undefined;
    if (!found) return undefined;
    const { connection } = found;
    if (fieldName === "childPiece") return connection.connecting?.piece;
    if (fieldName === "parentPiece") return connection.connected?.piece;
    if (fieldName === "childConnector") return connection.connecting?.connector;
    if (fieldName === "parentConnector") return connection.connected?.connector;
  }
  if (typeName === "Type" && fieldName === "fixedPieces") {
    const liveType = id ? state.kit.findType(id) : undefined;
    if (!liveType) return [];
    const pieces: Piece[] = [];
    for (const design of state.kit.designs ?? []) {
      for (const piece of design.pieces ?? []) {
        if (piece.dtoTypeId()?.id === liveType.id) pieces.push(piece);
      }
    }
    return pieces;
  }
  return undefined;
}

function readSchemaFieldValue(state: IndexedSchemaState, typeName: string, fieldName: string, id?: string, scope?: SchemaScope | null): any {
  const custom = readCustomFieldValue(state, typeName, fieldName, id);
  if (custom !== undefined) return custom;
  const ref = resolveReference(state, typeName, id, scope);
  if (!ref) return undefined;
  const key = getFieldDataKey(typeName, fieldName);
  return ref.value?.[key];
}

function isWritableField(state: IndexedSchemaState, typeName: string, fieldName: string, id?: string, scope?: SchemaScope | null): boolean {
  if (NEVER_WRITABLE_FIELDS.has(fieldName)) return false;
  const ref = resolveReference(state, typeName, id, scope);
  if (!ref) return false;
  const key = getFieldDataKey(typeName, fieldName);
  if (fieldName === "hash") return false;
  return ref.value != null && (Object.prototype.hasOwnProperty.call(ref.value, key) || ref.value[key] !== undefined);
}

function normalizeNextValue(current: any, fieldName: string, next: any): any {
  if (typeof next === "string" && current && typeof current === "object" && "id" in current) {
    return { id: next };
  }
  if ((fieldName === "type" || fieldName === "design" || fieldName === "piece" || fieldName === "designPiece" || fieldName === "connector") && typeof next === "string") {
    return { id: next };
  }
  return next;
}

function nextValueFromAction<T>(current: T, next: SetStateAction<T>): T {
  return typeof next === "function" ? (next as (value: T) => T)(current) : next;
}

function normalizeStateInput(input: KitHostStoreSnapshot | KitLike | IndexedSchemaState): IndexedSchemaState {
  if ((input as IndexedSchemaState).byId instanceof Map) return input as IndexedSchemaState;
  if ((input as KitHostStoreSnapshot).kit) {
    const snapshot = input as KitHostStoreSnapshot;
    return scanSchemaState(snapshot.kit.toJSON());
  }
  const kit = asKitInstance(input as KitLike);
  return scanSchemaState(kit.toJSON());
}

function collectChangedObjectFields(typeName: string, previousValue: any, nextValue: any): string[] {
  const dataKeys = new Set<string>();
  if (previousValue && typeof previousValue === "object") {
    for (const dataKey of Object.keys(previousValue)) dataKeys.add(dataKey);
  }
  if (nextValue && typeof nextValue === "object") {
    for (const dataKey of Object.keys(nextValue)) dataKeys.add(dataKey);
  }
  const fieldNames: string[] = [];
  for (const dataKey of dataKeys) {
    if (!deepEqual(previousValue?.[dataKey], nextValue?.[dataKey])) {
      fieldNames.push(getSchemaFieldName(typeName, dataKey));
    }
  }
  return fieldNames;
}

export function diffSchemaPropertyEvents(previousInput: KitHostStoreSnapshot | KitLike | IndexedSchemaState, nextInput: KitHostStoreSnapshot | KitLike | IndexedSchemaState): SchemaPropertyEvent[] {
  const previous = normalizeStateInput(previousInput);
  const next = normalizeStateInput(nextInput);
  const dirtyIds = new Set<string>();
  const allIds = new Set<string>([...(previous.byId.keys() ?? []), ...(next.byId.keys() ?? [])]);

  for (const idValue of allIds) {
    const previousRef = (previous.byId.get(idValue) ?? [])[0];
    const nextRef = (next.byId.get(idValue) ?? [])[0];
    if (!deepEqual(previousRef?.value, nextRef?.value)) {
      dirtyIds.add(idValue);
      collectIds(previousRef?.value, dirtyIds);
      collectIds(nextRef?.value, dirtyIds);
    }
  }

  const events: SchemaPropertyEvent[] = [];
  for (const idValue of dirtyIds) {
    const previousRef = (previous.byId.get(idValue) ?? [])[0];
    const nextRef = (next.byId.get(idValue) ?? [])[0];
    const typeName = nextRef?.typeName ?? previousRef?.typeName;
    if (!typeName) continue;
    for (const fieldName of collectChangedObjectFields(typeName, previousRef?.value, nextRef?.value)) {
      const previousValue = readSchemaFieldValue(previous, typeName, fieldName, idValue);
      const nextValue = readSchemaFieldValue(next, typeName, fieldName, idValue);
      if (!deepEqual(previousValue, nextValue)) {
        events.push({ key: `${typeName}.${fieldName}`, typeName, fieldName, id: idValue, previous: previousValue, current: nextValue });
      }
    }
  }

  if (!deepEqual(previous.plain, next.plain) && next.kitId) {
    for (const fieldName of collectChangedObjectFields("Kit", previous.plain, next.plain)) {
      const previousValue = readSchemaFieldValue(previous, "Kit", fieldName, previous.kitId);
      const nextValue = readSchemaFieldValue(next, "Kit", fieldName, next.kitId);
      if (!deepEqual(previousValue, nextValue)) {
        events.push({ key: `Kit.${fieldName}`, typeName: "Kit", fieldName, id: next.kitId, previous: previousValue, current: nextValue });
      }
    }
  }

  return events;
}

async function createNodeJsonFileAdapter(filePath: string) {
  const fs = await import("node:fs/promises");
  const path = await import("node:path");
  return {
    async read() {
      try {
        return await fs.readFile(filePath, "utf8");
      } catch {
        return "";
      }
    },
    async write(json: string) {
      await fs.mkdir(path.dirname(filePath), { recursive: true });
      await fs.writeFile(filePath, json, "utf8");
    },
  };
}

async function createNodeFolderAdapter(folderPath: string) {
  const fs = await import("node:fs/promises");
  const syncFs = await import("node:fs");
  const path = await import("node:path");
  const kitDbPath = path.join(folderPath, ".semio", "kit.db");

  async function listRecursive(currentPath: string, prefix: string = ""): Promise<string[]> {
    try {
      const entries = await fs.readdir(currentPath, { withFileTypes: true });
      const files: string[] = [];
      for (const entry of entries) {
        const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
        const absolute = path.join(currentPath, entry.name);
        if (entry.isDirectory()) {
          files.push(...(await listRecursive(absolute, relative)));
        } else {
          if (relative !== ".semio/kit.db") files.push(relative.replace(/\\/g, "/"));
        }
      }
      return files;
    } catch {
      return [];
    }
  }

  return {
    async readKit() {
      try {
        return new Uint8Array(await fs.readFile(kitDbPath));
      } catch {
        return undefined;
      }
    },
    async writeKit(data: Uint8Array) {
      await fs.mkdir(path.dirname(kitDbPath), { recursive: true });
      await fs.writeFile(kitDbPath, data);
    },
    async readFile(relativePath: string) {
      try {
        const data = await fs.readFile(path.join(folderPath, relativePath));
        return new Blob([data]);
      } catch {
        return undefined;
      }
    },
    async writeFile(relativePath: string, blob: Blob) {
      const absolutePath = path.join(folderPath, relativePath);
      await fs.mkdir(path.dirname(absolutePath), { recursive: true });
      await fs.writeFile(absolutePath, new Uint8Array(await blob.arrayBuffer()));
    },
    async deleteFile(relativePath: string) {
      await fs.rm(path.join(folderPath, relativePath), { force: true });
    },
    async createDirectory(relativePath: string) {
      await fs.mkdir(path.join(folderPath, relativePath), { recursive: true });
    },
    async moveEntry(fromPath: string, toPath: string) {
      await fs.mkdir(path.dirname(path.join(folderPath, toPath)), { recursive: true });
      await fs.rename(path.join(folderPath, fromPath), path.join(folderPath, toPath));
    },
    async listFiles() {
      await fs.mkdir(folderPath, { recursive: true });
      return listRecursive(folderPath);
    },
    watch(callback: () => void) {
      const watcher = syncFs.watch(folderPath, { recursive: true }, () => callback());
      return () => watcher.close();
    },
  };
}

async function createStoreFromBackbone(backbone: KitBackboneConfig | undefined, initialKit?: KitLike): Promise<KitHostStore> {
  const resolvedBackbone = backbone?.kind ? backbone : ({ kind: "memory", initialKit } as MemoryBackboneConfig);
  if (resolvedBackbone.kind === "memory") {
    const seed = resolvedBackbone.initialKit ?? initialKit ?? { id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() };
    return new InMemoryKitStore(asKitInstance(seed));
  }
  if (resolvedBackbone.kind === "dev") {
    return createJsonFileKitStore(await createNodeJsonFileAdapter(resolvedBackbone.filePath));
  }
  if (resolvedBackbone.kind === "local") {
    return createFolderKitStore(await createNodeFolderAdapter(resolvedBackbone.folderPath), initialKit ? (asKitInstance(initialKit).toJSON() as any) : undefined);
  }
  if (resolvedBackbone.kind === "remote") {
    return createSessionKitStore({
      serverUrl: resolvedBackbone.serverUrl,
      sessionId: resolvedBackbone.sessionId,
      kitName: resolvedBackbone.kitName,
      personId: resolvedBackbone.personId,
      clientId: resolvedBackbone.clientId,
      authToken: resolvedBackbone.authToken,
      readOnly: resolvedBackbone.readOnly,
    });
  }
  const k = (resolvedBackbone as { kind?: string }).kind;
  throw new Error(`semio/react: unsupported backbone${k ? ` (kind: ${k})` : ""}`);
}

/** @emoji 📌Derives file/folder/remote/temporary from backbone or store constructor. */
function inferPersistenceFromInit(init: { backbone?: KitBackboneConfig; store?: KitHostStore }): KitPersistenceInfo {
  const b = init.backbone;
  if (b && "kind" in b) {
    if (b.kind === "memory" || b.kind === undefined) return { kind: "temporary" };
    if (b.kind === "dev") return { kind: "file", path: b.filePath };
    if (b.kind === "local") return { kind: "folder", path: b.folderPath };
    if (b.kind === "remote") return { kind: "remote", url: b.serverUrl };
  }
  if (init.store) {
    const n = String((init.store as any).constructor?.name ?? "");
    if (n === "FolderKitStore") return { kind: "folder" };
    if (n === "JsonFileKitStore") return { kind: "file" };
    if (n.includes("Session") || n.includes("Remote")) return { kind: "remote" };
  }
  return { kind: "temporary" };
}

// #endregion ⚛️Utilities

// #region ⚛️Context

const KitRuntimeContext = React.createContext<KitRuntimeContextValue | null>(null);

/**
 * @emoji 🧭 One bridge: host kit id, materialized read scope, and optional VCS write anchors (set by {@link KitScope}).
 */
export type SemioKitScopedView = {
  kitId: string;
  kitReadScope: KitReadScope;
  kitWriteScope: KitWriteScope | null;
  /** @emoji 🌱 When {@link KitAlternativeSelectionProvider} is present: chosen alternative id, or `null` for the kit main line. */
  selectedAlternativeId: string | null;
};

const SemioKitScopedViewContext = React.createContext<SemioKitScopedView | null>(null);

/** @emoji 🧭 `null` outside {@link KitScope}. */
export function useSemioKitScopedView(): SemioKitScopedView | null {
  return React.useContext(SemioKitScopedViewContext);
}

/**
 * @emoji 🧭 Active {@link KitReadScope} for read hooks: {@link SemioKitScopedViewContext} when inside {@link KitScope}, else this default (main line).
 */
export const KitDataScopeContext = React.createContext<KitReadScope>(theKitReadScope);
export function useKitDataScope(): KitReadScope {
  const s = React.useContext(SemioKitScopedViewContext);
  if (s) return s.kitReadScope;
  return React.useContext(KitDataScopeContext);
}

/** @internal Dependency token so read hooks re-subscribe when {@link KitDataScopeContext} changes. */
function useKitDataScopeKey(): string {
  return kitReadScopeKey(useKitDataScope());
}
/** @emoji 📌 Current {@link SchemaScope} from nearest entity scope provider (TypeScope, DesignScope, …). */
export const SchemaScopeContext = React.createContext<SchemaScope | null>(null);

// #region KitRegistry

export type KitRegistryEntry = {
  store: KitHostStore;
  kitClient: KitStoreClient;
  refs: number;
  /** @emoji 📌How this kit is persisted; derived at {@link KitRegistryValue.open} time. */
  persistence: KitPersistenceInfo;
};

export type KitRegistryValue = {
  activeKitId: string | undefined;
  setActiveKit: (id: string | undefined) => void;
  /** @emoji 📌Open or bump refcount for a kit. */
  open: (id: string, init: { backbone?: KitBackboneConfig; initialKit?: KitLike; store?: KitHostStore; kitClient?: KitStoreClient; readScope?: KitReadScope }) => Promise<void>;
  /** @emoji 📌In-memory kit; returns new id. */
  openTemporary: (initialKit?: KitLike) => Promise<string>;
  /** @emoji 📌Json-file store from adapter. */
  openJsonFile: (kitId: string, adapter: KitJsonFileAdapter) => Promise<void>;
  /** @emoji 📌Folder store from adapter. */
  openFolder: (kitId: string, adapter: KitFolderAdapter, initialKit?: any) => Promise<void>;
  /** @emoji 📌Remote / session store. */
  openRemote: (kitId: string, config: RemoteBackboneConfig) => Promise<void>;
  close: (id: string) => void;
  get: (id: string) => KitRegistryEntry | undefined;
  list: () => string[];
  status: (id: string) => "idle" | "loading" | "ready" | "error";
};

type RegistryRow = {
  store: KitHostStore;
  kitClient: KitStoreClient;
  refs: number;
  unsub: () => void;
  persistence: KitPersistenceInfo;
};

const KitRegistryContext = React.createContext<KitRegistryValue | null>(null);

/** @internal Listeners when open kits set changes (Canvas windows may sit outside React context). */
const _kitRegistryListListeners = new Set<() => void>();
function emitKitRegistryListChanged(): void {
  for (const l of _kitRegistryListListeners) {
    try {
      l();
    } catch {
      /* ignore */
    }
  }
}

/** @emoji 🧾 Subscribe to registry open/close (new or removed kit ids), for {@link useOpenKitShallows}. */
export function subscribeKitRegistryListChanged(onChange: () => void): () => void {
  _kitRegistryListListeners.add(onChange);
  return () => {
    _kitRegistryListListeners.delete(onChange);
  };
}

// #region 🌱KitAlternativeSelection
/** @emoji 🌱 One row in the host alternative dropdown (id + display name from rs). */
export type KitAlternativeSummary = { readonly id: string; readonly name: string };

export type KitAlternativeSelectionContextValue = {
  readonly selectedAlternativeId: string | null;
  readonly setSelectedAlternativeId: (id: string | null) => void;
  readonly alternatives: ReadonlyArray<KitAlternativeSummary>;
};

const __kitAltNoopSet = (_id: string | null): void => {
  void _id;
};

const KIT_ALTERNATIVE_EMPTY_LIST: ReadonlyArray<KitAlternativeSummary> = Object.freeze([]);

const KIT_ALTERNATIVE_SELECTION_DEFAULT: KitAlternativeSelectionContextValue = Object.freeze({
  selectedAlternativeId: null,
  setSelectedAlternativeId: __kitAltNoopSet,
  alternatives: KIT_ALTERNATIVE_EMPTY_LIST,
});

const KitAlternativeSelectionContext = React.createContext<KitAlternativeSelectionContextValue>(KIT_ALTERNATIVE_SELECTION_DEFAULT);

type KitStoreWithVcs = { vcsState(): Promise<Record<string, unknown>> };

/** @emoji 🌱 Normalizes `wip.alternatives` whether rs returns a flat list or a relay {@link AlternativeConnection}. */
function __normalizeWipAlternatives(wip: unknown): KitAlternativeSummary[] {
  if (wip == null || typeof wip !== "object") return [];
  const altRaw = (wip as { alternatives?: unknown }).alternatives;
  if (altRaw == null) return [];
  if (Array.isArray(altRaw)) {
    const next: KitAlternativeSummary[] = [];
    for (const r of altRaw) {
      const id = String((r as { id?: unknown })?.id ?? "").trim();
      if (id === "") continue;
      next.push({ id, name: String((r as { name?: unknown })?.name ?? "") });
    }
    return next;
  }
  const edges = (altRaw as { edges?: readonly { node?: { id?: unknown; name?: unknown } | null }[] | null }).edges;
  if (!Array.isArray(edges)) return [];
  const next: KitAlternativeSummary[] = [];
  for (const e of edges) {
    const n = e?.node;
    if (!n) continue;
    const id = String(n.id ?? "").trim();
    if (id === "") continue;
    next.push({ id, name: String(n.name ?? "") });
  }
  return next;
}

/**
 * @emoji 🌱 Host wires the open kit id so {@link kitStoreFromKitStoreClient} can list graph alternatives (rs `wip.alternatives`).
 * When {@link kitId} is omitted, selection state is inert and alternatives stay empty.
 */
export function KitAlternativeSelectionProvider(props: { readonly kitId?: string; readonly children: React.ReactNode }): React.ReactElement {
  const { kitId, children } = props;
  const registry = React.useContext(KitRegistryContext);
  const [registryTick, setRegistryTick] = React.useState(0);
  const [selectedAlternativeId, setSelectedAlternativeId] = React.useState<string | null>(null);
  const [alternatives, setAlternatives] = React.useState<ReadonlyArray<KitAlternativeSummary>>(KIT_ALTERNATIVE_EMPTY_LIST);

  React.useEffect(() => subscribeKitRegistryListChanged(() => setRegistryTick((t) => t + 1)), []);

  const kitClient = React.useMemo(() => (kitId && registry ? registry.get(kitId)?.kitClient ?? null : null), [kitId, registry, registryTick]);

  React.useEffect(() => {
    if (!kitClient) {
      setAlternatives(KIT_ALTERNATIVE_EMPTY_LIST);
      return;
    }
    const ks = kitStoreFromKitStoreClient(kitClient) as KitStoreWithVcs | null;
    if (!ks || typeof ks.vcsState !== "function") {
      setAlternatives(KIT_ALTERNATIVE_EMPTY_LIST);
      return;
    }
    let cancelled = false;
    const pull = async () => {
      try {
        const v = await ks.vcsState();
        if (cancelled) return;
        const wip = v["wip"];
        const next = __normalizeWipAlternatives(wip);
        setAlternatives(next.length === 0 ? KIT_ALTERNATIVE_EMPTY_LIST : Object.freeze(next));
      } catch {
        if (!cancelled) setAlternatives(KIT_ALTERNATIVE_EMPTY_LIST);
      }
    };
    void pull();
    const offKit = kitClient.subscribe(() => {
      void pull();
    });
    return () => {
      cancelled = true;
      offKit();
    };
  }, [kitClient]);

  React.useEffect(() => {
    if (selectedAlternativeId == null) return;
    if (!alternatives.some((a) => a.id === selectedAlternativeId)) {
      setSelectedAlternativeId(null);
    }
  }, [alternatives, selectedAlternativeId]);

  const value = React.useMemo<KitAlternativeSelectionContextValue>(
    () => ({
      selectedAlternativeId,
      setSelectedAlternativeId,
      alternatives,
    }),
    [selectedAlternativeId, alternatives],
  );

  return React.createElement(KitAlternativeSelectionContext.Provider, { value }, children);
}

/** @emoji 🌱 Current alternative id, or `null` for the kit main line. */
export function useKitAlternativeSelection(): readonly [string | null, (id: string | null) => void] {
  const v = React.useContext(KitAlternativeSelectionContext);
  return [v.selectedAlternativeId, v.setSelectedAlternativeId] as const;
}

/** @emoji 🌱 Alternatives from rs (read-only), for host dropdowns. */
export function useKitAlternatives(): ReadonlyArray<KitAlternativeSummary> {
  return React.useContext(KitAlternativeSelectionContext).alternatives;
}

// #endregion 🌱KitAlternativeSelection

/** @internal For {@link SketchpadStore} and other non-hook callers. Cleared on {@link KitRegistryProvider} unmount. */
let _semioKitRegistryBridge: KitRegistryValue | null = null;
export function getKitRegistryBridge(): KitRegistryValue | null {
  return _semioKitRegistryBridge;
}

export function KitRegistryProvider({ children }: { children: ReactNode }): React.ReactElement {
  const rowsRef = React.useRef(new Map<string, RegistryRow>());
  const loadingRef = React.useRef(new Set<string>());
  const errRef = React.useRef(new Map<string, Error>());
  const [registryEpoch, bump] = React.useReducer((x: number) => x + 1, 0);
  const [activeKitId, setActiveKitId] = React.useState<string | undefined>(undefined);

  const open = React.useCallback(
    async (kitId: string, init: { backbone?: KitBackboneConfig; initialKit?: KitLike; store?: KitHostStore; kitClient?: KitStoreClient; readScope?: KitReadScope }) => {
      const cur = rowsRef.current.get(kitId);
      if (cur) {
        cur.refs += 1;
        bump();
        return;
      }
      loadingRef.current.add(kitId);
      errRef.current.delete(kitId);
      bump();
      try {
        const store = init.store ?? (await createStoreFromBackbone(init.backbone, init.initialKit));
        const persistence = init.store ? inferPersistenceFromInit({ backbone: init.backbone, store: init.store }) : inferPersistenceFromInit({ backbone: init.backbone, store });
        let kitClient = init.kitClient ?? null;
        if (!kitClient) {
          let initialDto: KitFullDto;
          try {
            initialDto = __kitHostPlainDtoFromStore(store);
          } catch (toJsonErr) {
            if (init.initialKit != null) {
              initialDto = normalizeKitFullDtoFolderPaths(init.initialKit as KitFullDto);
            } else {
              throw toJsonErr instanceof Error ? toJsonErr : new Error(String(toJsonErr));
            }
          }
          try {
            kitClient = await createKitStoreClient({
              initialKit: initialDto,
              forceFallback: shouldForceKitClientFallback(),
              readScope: init.readScope,
            });
          } catch (wasmErr) {
            console.warn("[semio/react] createKitStoreClient (wasm) failed; retrying with in-memory fallback client", wasmErr);
            kitClient = await createKitStoreClient({
              initialKit: initialDto,
              forceFallback: true,
              readScope: init.readScope,
            });
          }
        }
        (store as any).__semioKitBridge = kitClient;
        if (init.readScope) {
          try {
            kitClient.setKitReadScope(init.readScope);
          } catch {
            /* ignore */
          }
        }
        const unsub = kitClient.subscribe(() => {
          void applyKitClientSnapshotToLocalStore(kitClient, store);
        });
        rowsRef.current.set(kitId, { store, kitClient, refs: 1, unsub, persistence });
        emitKitRegistryListChanged();
      } catch (e) {
        const err = e instanceof Error ? e : new Error(String(e));
        errRef.current.set(kitId, err);
        throw err;
      } finally {
        loadingRef.current.delete(kitId);
        bump();
      }
    },
    [bump],
  );

  const close = React.useCallback(
    (kitId: string) => {
      const row = rowsRef.current.get(kitId);
      if (!row) return;
      row.refs -= 1;
      if (row.refs <= 0) {
        row.unsub();
        try {
          (row.store as any).__semioKitBridgeUnsub?.();
          delete (row.store as any).__semioKitBridgeUnsub;
          delete (row.store as any).__semioKitBridge;
          delete (row.store as any).__semioKitClient;
        } catch { /* ignore */ }
        row.kitClient.dispose();
        rowsRef.current.delete(kitId);
        setActiveKitId((cur) => (cur === kitId ? undefined : cur));
        emitKitRegistryListChanged();
      }
      bump();
    },
    [bump],
  );

  const value = React.useMemo<KitRegistryValue>(
    () => ({
      get activeKitId() {
        return activeKitId;
      },
      setActiveKit: (i) => {
        setActiveKitId(i);
      },
      open,
      async openTemporary(initialKit) {
        const k = id();
        await open(k, { backbone: { kind: "memory", initialKit }, initialKit });
        return k;
      },
      async openJsonFile(kitId, adapter) {
        const store = await createJsonFileKitStore(adapter);
        const filePath = String((adapter as any).filePath ?? (adapter as any).path ?? "browser-adapter");
        await open(kitId, { store, backbone: { kind: "dev", filePath } });
      },
      async openFolder(kitId, adapter, initialKit) {
        const store = await createFolderKitStore(adapter, initialKit);
        const folderPath = String((adapter as any).folderPath ?? (adapter as any).path ?? ".");
        await open(kitId, { store, backbone: { kind: "local", folderPath } });
      },
      async openRemote(kitId, config) {
        await open(kitId, { backbone: config });
      },
      close,
      get(kitId) {
        const row = rowsRef.current.get(kitId);
        if (!row) return undefined;
        return { store: row.store, kitClient: row.kitClient, refs: row.refs, persistence: row.persistence };
      },
      list() {
        return Array.from(rowsRef.current.keys());
      },
      status(kitId) {
        if (loadingRef.current.has(kitId)) return "loading";
        if (errRef.current.has(kitId)) return "error";
        if (rowsRef.current.has(kitId)) return "ready";
        return "idle";
      },
    }),
    [activeKitId, open, close, registryEpoch],
  );

  _semioKitRegistryBridge = value;
  React.useLayoutEffect(() => {
    return () => {
      _semioKitRegistryBridge = null;
      _kitRegistryListListeners.clear();
    };
  }, []);

  return React.createElement(KitRegistryContext.Provider, { value }, children);
}

export function useKitRegistry(): KitRegistryValue {
  const v = React.useContext(KitRegistryContext);
  if (!v) throw new Error("useKitRegistry must be used within <KitRegistryProvider>.");
  return v;
}

/** Like {@link useKitRegistry} but returns `null` when no provider is mounted. */
export function useKitRegistrySafe(): KitRegistryValue | null {
  return React.useContext(KitRegistryContext);
}

/**
 * @emoji 📦 Host root for kit scopes: delegates to {@link KitRegistryProvider}. Pass `initialKit` for future eager `openKit` bootstrap (thin hooks plan).
 */
export function KitStoreProvider({
  children,
  initialKit: _initialKit,
}: {
  children: ReactNode;
  /** Reserved: seed kit DTO when the JS `openKit` surface owns the worker (WIP). */
  initialKit?: KitLike;
}): React.ReactElement {
  void _initialKit;
  return React.createElement(KitRegistryProvider, null, children);
}

// #region 🖊️SketchpadDefaultKitFactories
/** @emoji 📄 Browser SPA: pick a `.json` kit via File System Access API or a read-only file input fallback. */
export function createDefaultBrowserSketchpadFileKitStoreFactory(): SketchpadKitStoreFactory {
  return async (_kit: Kit) => {
    if (typeof window !== "undefined" && "showOpenFilePicker" in window) {
      const [fileHandle] = await (window as any).showOpenFilePicker({
        types: [
          {
            description: "Semio Kit JSON",
            accept: { "application/json": [".json"] },
          },
        ],
      });
      const adapter: KitJsonFileAdapter = {
        read: async () => {
          const file = await fileHandle.getFile();
          return file.text();
        },
        write: async (json: string) => {
          const writable = await fileHandle.createWritable();
          await writable.write(json);
          await writable.close();
        },
      };
      return createJsonFileKitStore(adapter);
    }
    return new Promise<KitHostStore>((resolve, reject) => {
      const input = document.createElement("input");
      input.type = "file";
      input.accept = ".json";
      input.onchange = async (e) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (!file) {
          reject(new Error("No file selected"));
          return;
        }
        const text = await file.text();
        const adapter: KitJsonFileAdapter = {
          read: async () => text,
          write: async (_json: string) => {
            console.warn("[semio/react] File System Access API not available; kit cannot be saved to the original file.");
          },
        };
        resolve(await createJsonFileKitStore(adapter));
      };
      input.oncancel = () => reject(new Error("File picker cancelled"));
      input.click();
    });
  };
}

/** @emoji 🌐 Browser SPA: remote kit via session transport; `kit.name` must hold the server URL. */
export function createDefaultBrowserSketchpadRemoteKitStoreFactory(): SketchpadKitStoreFactory {
  return async (kit: Kit) => {
    const serverUrl = kit.name;
    if (!serverUrl) throw new Error("No server URL provided for remote kit");
    return createSessionKitStore({
      serverUrl,
    });
  };
}

/** @emoji 🧩 VS Code webview: JSON read/write via extension `postMessage` and injected `__SEMIO_KIT_JSON__`. */
export function createVscodeWebviewSketchpadFileKitStoreFactory(vscodeApi: { postMessage: (msg: unknown) => void }): SketchpadKitStoreFactory {
  return async (kit: Kit) => {
    const adapter: KitJsonFileAdapter = {
      read: async () => {
        if (typeof window === "undefined") {
          return JSON.stringify((kit as any).toJSON?.() ?? kit);
        }
        const injected = (window as any).__SEMIO_KIT_JSON__;
        if (injected == null) {
          return JSON.stringify((kit as any).toJSON?.() ?? kit);
        }
        return typeof injected === "string" ? injected : JSON.stringify(injected);
      },
      write: async (json: string) => {
        vscodeApi.postMessage({ kind: "kit.save", content: json });
      },
    };
    return createJsonFileKitStore(adapter);
  };
}
// #endregion 🖊️SketchpadDefaultKitFactories

// #endregion KitRegistry

function useKitRuntime(): KitRuntimeContextValue {
  const runtime = React.useContext(KitRuntimeContext);
  if (!runtime) throw new Error("semio/react hooks must be used inside <KitScope>.");
  return runtime;
}

function shouldForceKitClientFallback(): boolean {
  if ((import.meta as any)?.env?.MODE === "test") return true;
  if (typeof process !== "undefined" && (process as { env?: Record<string, string | undefined> }).env?.SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS === "1") return true;
  return false;
}

/** Like {@link useKitRuntime} but returns `null` outside {@link KitScope} (no throw). */
export function useKitRuntimeSafe(): KitRuntimeContextValue | null {
  return React.useContext(KitRuntimeContext);
}

/**
 * @internal Public only for low-level / Storybook; app code should use {@link KitScope} hooks.
 * Returns the WASM {@link KitStoreClient} when inside {@link KitScope}, or `null`.
 */
export function useKitStoreClient(): KitStoreClient | null {
  const runtime = useKitRuntime();
  return runtime.kitClient;
}

/** Active kit id from {@link KitScope} runtime, or `undefined` outside a scope. */
export function useActiveKitId(): string | undefined {
  return React.useContext(KitRuntimeContext)?.kitId;
}

/** @emoji 📌 Shell / tab kit id (DOM bridge, R3F); complements {@link useActiveKitId} from {@link KitRuntimeContext}. */
export type KitShellScopeValue = { id: string };

const KitShellScopeContext = React.createContext<KitShellScopeValue | null>(null);

/** @emoji 📌 Provider for tab-scoped kit id (sketchpad shell, scene bridge). */
export function KitShellScopeProvider(props: { id: string; children: React.ReactNode }): React.ReactElement {
  return React.createElement(KitShellScopeContext.Provider, { value: { id: props.id } }, props.children as any);
}

export function useKitShellScope(): KitShellScopeValue | null {
  return React.useContext(KitShellScopeContext);
}

/** @emoji 📌 Same context as {@link KitShellScopeContext}; stable name for kit tab scope. */
export const KitScopeContext = KitShellScopeContext;

/**
 * @emoji 📌 Resolves kit id: explicit argument, then {@link useKitShellScope}, then {@link useActiveKitId}.
 */
export function useResolvedKitIdentifier(explicitKitId?: string): string | undefined {
  const semio = useSemioKitScopedView();
  const bridged = useKitShellScope();
  const active = useActiveKitId();
  if (explicitKitId != null && String(explicitKitId) !== "") return String(explicitKitId);
  if (semio != null && String(semio.kitId) !== "") return String(semio.kitId);
  if (bridged?.id) return bridged.id;
  if (active != null && active !== "") return active;
  return undefined;
}

/** @emoji 📌 Active kit scope `{ id }` from shell context or runtime (same resolution as sketchpad `useKitScope`). */
export function useKitScope(): KitShellScopeValue | null {
  const bridged = useKitShellScope();
  if (bridged) return bridged;
  const g = useActiveKitId();
  return g != null && g !== "" ? { id: g } : null;
}

export function useIsInKitScope(): boolean {
  return useKitScope() != null;
}

/**
 * @emoji 📌 Live {@link KitHostStoreSnapshot} for the resolved kit when it matches the current {@link KitRuntimeContext}.
 */
export function useKitStoreSnapshot(explicitKitId?: string): KitHostStoreSnapshot | null {
  const runtime = useKitRuntimeSafe();
  const effectiveKitId = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const snapshotRef = React.useRef<KitHostStoreSnapshot | null>(null);
  const storeRef = React.useRef<KitHostStore | null>(null);
  const subscribe = React.useCallback(
    (onStoreChange: () => void) => {
      if (runtime && effectiveKitId && runtime.kitId === effectiveKitId) {
        return runtime.store.subscribe(onStoreChange);
      }
      return () => {};
    },
    [runtime, effectiveKitId, scopeKey],
  );
  const getSnapshot = React.useCallback(() => {
    if (!runtime || !effectiveKitId || runtime.kitId !== effectiveKitId) {
      snapshotRef.current = null;
      storeRef.current = null;
      return null;
    }
    const st = runtime.store;
    if (storeRef.current !== st) {
      storeRef.current = st;
      snapshotRef.current = null;
    }
    const snap = st.getSnapshot();
    const prev = snapshotRef.current;
    if (
      prev &&
      prev.kit === snap.kit &&
      prev.sync.status === snap.sync.status &&
      prev.sync.dirty === snap.sync.dirty &&
      prev.sync.readonly === snap.sync.readonly &&
      prev.sync.lastSyncedAt === snap.sync.lastSyncedAt &&
      prev.sync.error === snap.sync.error
    ) {
      return prev;
    }
    snapshotRef.current = snap;
    return snap;
  }, [runtime, effectiveKitId, scopeKey]);
  return React.useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

const EMPTY_KIT_READ_SNAP: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: Object.freeze([]) as readonly unknown[],
  pending: 0,
}) as KitStoreReadSnap;

/**
 * @emoji 📌 `useSyncExternalStore` on a `getSnapshot`/`subscribe` pair with a stable server snapshot.
 * Pairs with {@link getSemioKitLiveReadStore} from `@semio/js` and {@link getSemioKitDesignReadStore} / {@link getSemioKitShallowListReadStore}.
 */
export function useSemioReadSnap<T extends KitStoreReadSnap>(
  subscribe: (onStoreChange: () => void) => () => void,
  getSnapshot: () => T,
  getServerSnapshot: () => T = getSnapshot,
): T {
  return React.useSyncExternalStore(
    React.useCallback(
      (onChange) => subscribe(onChange),
      [subscribe],
    ),
    getSnapshot,
    getServerSnapshot,
  );
}

function kitReadonlyTriad<T>(value: T): HookTriad<T> {
  return [value, noopAsyncSet, { kind: "readonly" as const, pending: 0 }];
}

const EMPTY_KIT_TYPE_IDS: readonly string[] = [];
const EMPTY_KIT_TYPES_METADATA: readonly unknown[] = [];
const EMPTY_KIT_DESIGN_IDS: readonly string[] = [];
const EMPTY_KIT_DESIGNS_METADATA: readonly unknown[] = [];

/** 🧾 Stable {@link KitStoreReadSnap} identities for {@link useSemioReadSnap} idle branches (avoid React #520). */
const EMPTY_KIT_READ_SNAP_WITH_TYPE_IDS: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: EMPTY_KIT_TYPE_IDS,
  pending: 0,
}) as KitStoreReadSnap;
const EMPTY_KIT_READ_SNAP_WITH_TYPES_METADATA: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: EMPTY_KIT_TYPES_METADATA,
  pending: 0,
}) as KitStoreReadSnap;
const EMPTY_KIT_READ_SNAP_WITH_DESIGN_IDS: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: EMPTY_KIT_DESIGN_IDS,
  pending: 0,
}) as KitStoreReadSnap;
const EMPTY_KIT_READ_SNAP_WITH_DESIGNS_METADATA: KitStoreReadSnap = Object.freeze({
  version: 0,
  data: EMPTY_KIT_DESIGNS_METADATA,
  pending: 0,
}) as KitStoreReadSnap;
const EMPTY_KIT_READ_SNAP_FALSE: KitStoreReadSnap = Object.freeze({ version: 0, data: false as unknown, pending: 0 }) as KitStoreReadSnap;
const EMPTY_KIT_READ_SNAP_ZERO: KitStoreReadSnap = Object.freeze({ version: 0, data: 0 as unknown, pending: 0 }) as KitStoreReadSnap;

/** @emoji 📌 Type ids from live kit graph (`ReadKitTypeIdsCommand`); async hub + {@link KitStore.kindRowIds}. */
export function useTypesIds(explicitKitId?: string): HookTriad<readonly string[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-tids:${scopeKey}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_TYPE_IDS;
          return ks.kindRowIds(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP_WITH_TYPE_IDS;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? (snap.data as readonly string[]) : EMPTY_KIT_TYPE_IDS;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : snap.pending > 0
        ? { kind: "pending", pending: snap.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Per-kind metadata rows (`ReadKitTypesMetadataCommand`) via {@link KitStore.kindMetadataRows}. */
export function useTypesMetadata(explicitKitId?: string): HookTriad<readonly unknown[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-tmeta:${scopeKey}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_TYPES_METADATA;
          return ks.kindMetadataRows(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP_WITH_TYPES_METADATA;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : EMPTY_KIT_TYPES_METADATA;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : snap.pending > 0
        ? { kind: "pending", pending: snap.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Design ids from live kit graph (`ReadKitDesignIdsCommand`) via {@link KitStore.designRowIds}. */
export function useDesignsIds(explicitKitId?: string): HookTriad<readonly string[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-dids:${scopeKey}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_DESIGN_IDS;
          return ks.designRowIds(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP_WITH_DESIGN_IDS;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? (snap.data as readonly string[]) : EMPTY_KIT_DESIGN_IDS;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : snap.pending > 0
        ? { kind: "pending", pending: snap.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Per-design metadata rows (`ReadKitDesignsMetadataCommand`) via {@link KitStore.designMetadataRows}. */
export function useDesignsMetadata(explicitKitId?: string): HookTriad<readonly unknown[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-dmeta-res:${scopeKey}:${resolved ?? ""}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_DESIGNS_METADATA;
          return ks.designMetadataRows(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP_WITH_DESIGNS_METADATA;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : EMPTY_KIT_DESIGNS_METADATA;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : snap.pending > 0
        ? { kind: "pending", pending: snap.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Full kit store snapshot triad (read-only). */
export function useKitSnapshotTriad(explicitKitId?: string): HookTriad<KitHostStoreSnapshot | null> {
  const snap = useKitStoreSnapshot(explicitKitId);
  return kitReadonlyTriad(snap);
}

const EMPTY_KIT_ENTITY_LIST: any[] = [];

/** @emoji 📌 Kit `types` from `KitStore.read` `readKitFullCommand` (RS materialization, not host DTO scan). */
export function useTypesFull(explicitKitId?: string): HookTriad<any[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-types-full:${scopeKey}:${resolved ?? ""}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_ENTITY_LIST;
          const out = await ks.read(readScope, [{ readKitFullCommand: null }]);
          const row = out[0];
          if (!row || !("readKitFullCommand" in row)) return EMPTY_KIT_ENTITY_LIST;
          const t = row.readKitFullCommand.full.types;
          return Array.isArray(t) ? t : EMPTY_KIT_ENTITY_LIST;
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const s = useSemioReadSnap(subscribe, getSnap, getSnap);
  const raw = s.data;
  const value = Array.isArray(raw) ? raw : EMPTY_KIT_ENTITY_LIST;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : s.pending > 0
        ? { kind: "pending", pending: s.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Kit `designs` from `KitStore.read` `readKitFullCommand` (RS materialization). */
export function useDesignsFull(explicitKitId?: string): HookTriad<any[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-designs-full:${scopeKey}:${resolved ?? ""}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_ENTITY_LIST;
          const out = await ks.read(readScope, [{ readKitFullCommand: null }]);
          const row = out[0];
          if (!row || !("readKitFullCommand" in row)) return EMPTY_KIT_ENTITY_LIST;
          const t = row.readKitFullCommand.full.designs;
          return Array.isArray(t) ? t : EMPTY_KIT_ENTITY_LIST;
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const s = useSemioReadSnap(subscribe, getSnap, getSnap);
  const raw = s.data;
  const value = Array.isArray(raw) ? raw : EMPTY_KIT_ENTITY_LIST;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : s.pending > 0
        ? { kind: "pending", pending: s.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Kit `files` from `KitStore.read` `readKitFullCommand` (RS materialization). */
export function useFilesFull(explicitKitId?: string): HookTriad<any[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-files-full:${scopeKey}:${resolved ?? ""}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_ENTITY_LIST;
          const out = await ks.read(readScope, [{ readKitFullCommand: null }]);
          const row = out[0];
          if (!row || !("readKitFullCommand" in row)) return EMPTY_KIT_ENTITY_LIST;
          const t = row.readKitFullCommand.full.files;
          return Array.isArray(t) ? t : EMPTY_KIT_ENTITY_LIST;
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const s = useSemioReadSnap(subscribe, getSnap, getSnap);
  const raw = s.data;
  const value = Array.isArray(raw) ? raw : EMPTY_KIT_ENTITY_LIST;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : s.pending > 0
        ? { kind: "pending", pending: s.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

/** @emoji 📌 Kit `tags` from `KitStore.read` `readKitFullCommand` (RS materialization). */
export function useTagsFull(explicitKitId?: string): HookTriad<any[]> {
  const runtime = useKitRuntime();
  const resolved = useResolvedKitIdentifier(explicitKitId);
  const scopeKey = useKitDataScopeKey();
  const readScope = useKitDataScope();
  const key = `k-tags-full:${scopeKey}:${resolved ?? ""}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return EMPTY_KIT_ENTITY_LIST;
          const out = await ks.read(readScope, [{ readKitFullCommand: null }]);
          const row = out[0];
          if (!row || !("readKitFullCommand" in row)) return EMPTY_KIT_ENTITY_LIST;
          const t = row.readKitFullCommand.full.tags;
          return Array.isArray(t) ? t : EMPTY_KIT_ENTITY_LIST;
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, runtime.kitId, resolved, scopeKey, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !resolved || runtime.kitId !== resolved) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, runtime.kitId, resolved, key]);
  const s = useSemioReadSnap(subscribe, getSnap, getSnap);
  const raw = s.data;
  const value = Array.isArray(raw) ? raw : EMPTY_KIT_ENTITY_LIST;
  const status: WriteStatus =
    !runtime.kitClient || !resolved || runtime.kitId !== resolved
      ? { kind: "readonly", pending: 0 }
      : s.pending > 0
        ? { kind: "pending", pending: s.pending }
        : { kind: "idle", pending: 0 };
  return [value, noopAsyncSet, status] as const;
}

export type KitScopeProps = {
  store?: KitHostStore;
  /** When set with <KitRegistryProvider>, uses the registry entry for this kit (warm WASM worker). */
  kitId?: string;
  /** When provided (e.g. from registry), skips creating a new worker client. */
  kitClient?: KitStoreClient | null;
  /** @emoji 🧭 Initial {@link KitReadScope} for `createKitStoreClient` / read materialization (omit to follow {@link KitAlternativeSelectionProvider}, else main line). */
  kitReadScope?: KitReadScope;
  /** @emoji 🧭 When set, pins {@link KitStoreClient.setKitWriteScope} for batched kit mutations (omit to auto-bootstrap per gesture). */
  kitWriteScope?: KitWriteScope | null;
  backbone?: KitBackboneConfig;
  initialKit?: KitLike;
  children: ReactNode;
  fallback?: ReactNode;
};

export function KitScope({
  store: externalStore,
  kitId: kitIdProp,
  kitClient: kitClientProp,
  kitReadScope: kitReadScopeProp,
  kitWriteScope: kitWriteScopeProp,
  backbone,
  initialKit,
  children,
  fallback = null,
}: KitScopeProps): React.ReactElement | null {
  const altSel = React.useContext(KitAlternativeSelectionContext);
  const effectiveKitReadScope = React.useMemo((): KitReadScope => {
    if (kitReadScopeProp !== undefined) return kitReadScopeProp;
    const altId = altSel.selectedAlternativeId;
    return altId ? { alternative: { alternativeId: altId } } : theKitReadScope;
  }, [kitReadScopeProp, altSel.selectedAlternativeId]);

  const registry = React.useContext(KitRegistryContext);
  if (kitIdProp && !registry) {
    throw new Error("semio/react: <KitScope kitId={...}> must be wrapped in <KitRegistryProvider>.");
  }
  const registryEntry = kitIdProp && registry ? registry.get(kitIdProp) : undefined;

  const [internalStore, setInternalStore] = React.useState<KitHostStore | null>(externalStore ?? null);
  const [kitClientState, setKitClientState] = React.useState<KitStoreClient | null>(kitClientProp ?? null);

  React.useEffect(() => {
    if (kitIdProp) return;
    if (externalStore) {
      setInternalStore(externalStore);
      return;
    }
    let disposed = false;
    createStoreFromBackbone(backbone, initialKit).then((store) => {
      if (!disposed) setInternalStore(store);
    });
    return () => {
      disposed = true;
    };
  }, [kitIdProp, externalStore, backbone, initialKit]);

  React.useEffect(() => {
    if (kitIdProp) return;
    if (kitClientProp !== undefined) {
      setKitClientState(kitClientProp);
      return;
    }
    const st = externalStore ?? internalStore;
    if (!st) return;
    let cancelled = false;
    let client: KitStoreClient | null = null;
    void createKitStoreClient({ initialKit: st.getSnapshot().kit.toJSON(), forceFallback: shouldForceKitClientFallback(), readScope: effectiveKitReadScope }).then((c) => {
      if (cancelled) {
        c.dispose();
        return;
      }
      client = c;
      setKitClientState(c);
    });
    return () => {
      cancelled = true;
      if (client) {
        client.dispose();
      }
      setKitClientState(null);
    };
  }, [kitIdProp, externalStore, internalStore, kitClientProp, effectiveKitReadScope]);

  const store = kitIdProp && registryEntry ? registryEntry.store : (externalStore ?? internalStore);
  const kitClient = kitIdProp && registryEntry ? registryEntry.kitClient : (kitClientProp ?? kitClientState);

  React.useEffect(() => {
    if (!kitClient) return;
    kitClient.setKitReadScope(effectiveKitReadScope);
  }, [kitClient, effectiveKitReadScope]);

  React.useEffect(() => {
    if (!kitClient) return;
    if (kitWriteScopeProp === undefined) return;
    kitClient.setKitWriteScope(kitWriteScopeProp);
  }, [kitClient, kitWriteScopeProp]);

  React.useEffect(() => {
    if (!kitClient) return;
    if (kitWriteScopeProp !== undefined) return;
    kitClient.setKitWriteScope(null);
  }, [kitClient, kitWriteScopeProp, altSel.selectedAlternativeId]);

  if (kitIdProp && registry && !registryEntry) return React.createElement(React.Fragment, null, fallback);
  if (!store) return React.createElement(React.Fragment, null, fallback);

  React.useEffect(() => {
    if (kitIdProp) return;
    if (!kitClient) return;
    return kitClient.subscribe(() => {
      void applyKitClientSnapshotToLocalStore(kitClient, store);
    });
  }, [kitClient, store, kitIdProp]);

  const snapshotRef = React.useRef<KitHostStoreSnapshot | null>(null);
  const getSnapshot = React.useCallback(() => {
    const snap = store.getSnapshot();
    const prev = snapshotRef.current;
    if (
      prev &&
      prev.kit === snap.kit &&
      prev.sync.status === snap.sync.status &&
      prev.sync.dirty === snap.sync.dirty &&
      prev.sync.readonly === snap.sync.readonly &&
      prev.sync.lastSyncedAt === snap.sync.lastSyncedAt &&
      prev.sync.error === snap.sync.error
    ) {
      return prev;
    }
    snapshotRef.current = snap;
    return snap;
  }, [store]);

  const snapshot = React.useSyncExternalStore(
    React.useCallback((listener) => store.subscribe(listener), [store]),
    getSnapshot,
    getSnapshot,
  );

  const state = React.useMemo(() => scanSchemaState(snapshot.kit.toJSON()), [snapshot]);
  const previousStateRef = React.useRef<IndexedSchemaState | null>(null);
  const [recentEvents, setRecentEvents] = React.useState<SchemaPropertyEvent[]>([]);

  React.useEffect(() => {
    const previous = previousStateRef.current;
    if (previous) {
      const nextEvents = diffSchemaPropertyEvents(previous, state);
      if (nextEvents.length > 0) {
        setRecentEvents((existing) => [...existing, ...nextEvents].slice(-500));
      }
    }
    previousStateRef.current = state;
  }, [state]);

  const [recentSetRejections, setRecentSetRejections] = React.useState<SetError[]>([]);
  const pushSetRejection = React.useCallback((e: SetError) => {
    setRecentSetRejections((r) => [...r.slice(-99), e]);
  }, []);

  React.useEffect(() => {
    if (!kitClient) return;
    return kitClient.subscribe((event: KitEvent) => {
      if (!isKitCommandLifecycleEvent(event)) return;
      const command = event.semioKitCommand;
      if (command.error) pushSetRejection(command.error);
      setRecentEvents((existing) => [
        ...existing,
        {
          key: `KitCommand:${command.requestId}:${command.phase}`,
          typeName: "KitCommand",
          fieldName: command.phase,
          id: command.requestId,
          previous: undefined,
          current: command.error ?? command.commandKind,
          requestId: command.requestId,
          commandKind: command.commandKind,
          phase: command.phase,
        },
      ].slice(-500));
    });
  }, [kitClient, pushSetRejection]);

  const setFieldValue = React.useCallback(
    async (typeName: string, fieldName: string, next: SetStateAction<any>, idValue?: string, scope?: SchemaScope | null): Promise<SetResult> => {
      if (!kitClient) {
        const e: SetError = { kind: "Internal", message: "kit client required for mutations" };
        pushSetRejection(e);
        return { ok: false, error: e };
      }
      if (snapshot.sync.readonly) {
        return { ok: false, error: { kind: "Readonly", message: "read-only" } };
      }
      const currentState = scanSchemaState(store.getSnapshot().kit.toJSON());
      if (!isWritableField(currentState, typeName, fieldName, idValue, scope)) {
        return { ok: false, error: { kind: "Readonly", message: "not writable" } };
      }
      const ref = resolveReference(currentState, typeName, idValue, scope);
      if (!ref) {
        return { ok: false, error: { kind: "NotFound", message: "entity" } };
      }
      const key = getFieldDataKey(typeName, fieldName);
      const currentValue = readSchemaFieldValue(currentState, typeName, fieldName, idValue, scope);
      const nextResolved = nextValueFromAction(currentValue, next);
      const val = normalizeNextValue(currentValue, fieldName, nextResolved);
      const entityId = typeName === "Kit" ? String(currentState.kitId ?? ref.id ?? (ref as any).value?.id ?? "") : String(ref.id ?? "");
      if (!entityId) {
        const e: SetError = { kind: "NotFound", message: "missing id" };
        pushSetRejection(e);
        return { ok: false, error: e };
      }
      const r = await writeKitStoreClientSchemaField(kitClient, typeName, key, val, entityId);
      if (!r.ok) pushSetRejection(r.error);
      return r;
    },
    [kitClient, store, snapshot.sync.readonly, pushSetRejection],
  );

  const setObjectValue = React.useCallback(
    async (typeName: string, next: SetStateAction<any>, idValue?: string, scope?: SchemaScope | null): Promise<SetResult> => {
      if (!kitClient) {
        const e: SetError = { kind: "Internal", message: "kit client required for mutations" };
        pushSetRejection(e);
        return { ok: false, error: e };
      }
      if (snapshot.sync.readonly) {
        return { ok: false, error: { kind: "Readonly", message: "read-only" } };
      }
      const currentState = scanSchemaState(store.getSnapshot().kit.toJSON());
      const ref = resolveReference(currentState, typeName, idValue, scope);
      if (!ref) {
        return { ok: false, error: { kind: "NotFound", message: "entity" } };
      }
      const prev = ref.value;
      const nextObj = nextValueFromAction(prev, next);
      const fieldNames = collectChangedObjectFields(typeName, prev, nextObj);
      const entityId = typeName === "Kit" ? String(currentState.kitId ?? ref.id ?? (ref as any).value?.id ?? "") : String(ref.id ?? "");
      if (!entityId) {
        const e: SetError = { kind: "NotFound", message: "missing id" };
        pushSetRejection(e);
        return { ok: false, error: e };
      }
      for (const fn of fieldNames) {
        if (!isWritableField(currentState, typeName, fn, idValue, scope)) continue;
        const dataKey = getFieldDataKey(typeName, fn);
        const v = (nextObj as any)?.[dataKey];
        const r = await writeKitStoreClientSchemaField(kitClient, typeName, dataKey, v, entityId);
        if (!r.ok) {
          pushSetRejection(r.error);
          return r;
        }
      }
      return { ok: true } as const;
    },
    [kitClient, store, snapshot.sync.readonly, pushSetRejection],
  );

  const activeKitId = kitIdProp ?? snapshot.kit?.id;

  const semioKitScopedView = React.useMemo<SemioKitScopedView>(
    () => ({
      kitId: String(activeKitId ?? ""),
      kitReadScope: effectiveKitReadScope,
      kitWriteScope: kitWriteScopeProp === undefined ? (kitClient?.getKitWriteScope() ?? null) : kitWriteScopeProp,
      selectedAlternativeId: altSel.selectedAlternativeId,
    }),
    [activeKitId, effectiveKitReadScope, kitWriteScopeProp, kitClient, altSel.selectedAlternativeId],
  );

  const value = React.useMemo<KitRuntimeContextValue>(
    () => ({
      store,
      snapshot,
      state,
      recentEvents,
      recentSetRejections,
      pushSetRejection,
      canWrite: !snapshot.sync.readonly,
      kitId: activeKitId,
      kitBackbone: backbone,
      kitClient,
      setFieldValue,
      setObjectValue,
    }),
    [store, snapshot, state, recentEvents, recentSetRejections, pushSetRejection, activeKitId, backbone, kitClient, setFieldValue, setObjectValue],
  );

  return React.createElement(
    SemioKitScopedViewContext.Provider,
    { value: semioKitScopedView },
    React.createElement(KitRuntimeContext.Provider, { value }, children),
  );
}

type EntityScopeProps = { id?: string; children: ReactNode };

function useEntityScope(typeName: string, idValue?: string): SchemaScope {
  const runtime = useKitRuntime();
  const parentScope = React.useContext(SchemaScopeContext);
  const ref = resolveReference(runtime.state, typeName, idValue, parentScope);
  return ref ? { typeName, id: ref.id, path: ref.path } : { typeName, id: idValue, path: [] };
}

export function PieceScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Piece", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function TypeScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Type", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function DesignScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Design", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConnectionScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("ConnectionStore", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function PortScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Port", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function QualityScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Quality", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FileScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("File", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FolderScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Folder", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function AuthorScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Author", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

/** @emoji 📌 App-route aliases matching sketchpad `*ScopeProvider` names; same components as entity scopes. */
export const AuthorScopeProvider = AuthorScope;
export const TypeScopeProvider = TypeScope;
export const QualityScopeProvider = QualityScope;
export const DesignScopeProvider = DesignScope;
export const PieceScopeProvider = PieceScope;
export const ConnectionScopeProvider = ConnectionScope;

export function TagScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Tag", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConceptScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Concept", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FamilyScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Family", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function RepresentationScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("RepresentationStore", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConnectorScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("ConnectorStore", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function BenchmarkScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Benchmark", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function LayerScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Layer", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function GroupScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Group", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function StatScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Stat", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function PropScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Prop", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function AttributeScope({ id: idValue, children }: EntityScopeProps): React.ReactElement {
  const scope = useEntityScope("Attribute", idValue);
  return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

/** Read the current {@link SchemaScope} from the nearest entity {@link TypeScope} (or sibling). */
export function useSchemaScope(): SchemaScope | null {
  return React.useContext(SchemaScopeContext);
}

/** @emoji 📌 Minimal {@link SchemaScope} when only entity id is known (e.g. R3F context bridge). */
export function schemaScopeForEntityId(typeName: string, entityId: string): SchemaScope {
  return { typeName, id: entityId, path: [] };
}

function useEntityScopeId(typeName: string): { id: string } | null {
  const s = useSchemaScope();
  return s?.typeName === typeName && s.id ? { id: s.id } : null;
}

export function useAuthorScope(): { id: string } | null {
  return useEntityScopeId("Author");
}

export function useIsInAuthorScope(): boolean {
  return useAuthorScope() != null;
}

export function useTypeScope(): { id: string } | null {
  return useEntityScopeId("Type");
}

export function useIsInTypeScope(): boolean {
  return useTypeScope() != null;
}

export function useQualityScope(): { id: string } | null {
  return useEntityScopeId("Quality");
}

export function useIsInQualityScope(): boolean {
  return useQualityScope() != null;
}

export function useDesignScope(): { id: string } | null {
  return useEntityScopeId("Design");
}

export function useIsInDesignScope(): boolean {
  return useDesignScope() != null;
}

export function usePieceScope(): { id: string } | null {
  return useEntityScopeId("Piece");
}

export function useConnectionScope(): { id: string } | null {
  return useEntityScopeId("ConnectionStore");
}

// #endregion ⚛️Context

// #region ⚛️Core Hooks

function resolveRustFieldTarget(runtime: KitRuntimeContextValue, typeName: string, fieldName: string, idValue: string | undefined, scope: SchemaScope | null): { kind: string; id: string; field: string } | null {
  if (!runtime.kitClient) return null;
  const kitRustField = (field: string): string | null => {
    switch (field) {
      case "name":
      case "description":
      case "icon":
      case "image":
      case "homepage":
      case "license":
        return field;
      case "release":
        return "version";
      default:
        return null;
    }
  };
  if (typeName === "Piece" && (fieldName === "name" || fieldName === "color")) {
    const g = idValue ?? scope?.id;
    if (!g) return null;
    return { kind: "Piece", id: g, field: fieldName };
  }
  if (typeName === "Kit") {
    const field = kitRustField(fieldName);
    if (!field) return null;
    return { kind: "Kit", id: runtime.snapshot.kit.id, field };
  }
  if (typeName === "Design" && fieldName === "name") {
    const g = idValue ?? scope?.id;
    if (!g) return null;
    return { kind: "Design", id: g, field: "name" };
  }
  if (typeName === "Type" && fieldName === "name") {
    const g = idValue ?? scope?.id;
    if (!g) return null;
    return { kind: "Type", id: g, field: "name" };
  }
  return null;
}

export function useSchemaEvents(filter?: Partial<Pick<SchemaPropertyEvent, "typeName" | "fieldName" | "id" | "key">>): SchemaPropertyEvent[] {
  const runtime = useKitRuntime();
  return React.useMemo(() => {
    if (!filter) return runtime.recentEvents;
    return runtime.recentEvents.filter((event) => {
      if (filter.typeName && event.typeName !== filter.typeName) return false;
      if (filter.fieldName && event.fieldName !== filter.fieldName) return false;
      if (filter.id && event.id !== filter.id) return false;
      if (filter.key && event.key !== filter.key) return false;
      return true;
    });
  }, [runtime.recentEvents, filter]);
}

export function useSetErrors(filter?: Partial<{ entityKind: string; id: string }>): SetError[] {
  const runtime = useKitRuntime();
  return React.useMemo(() => {
    if (!filter) return runtime.recentSetRejections;
    return runtime.recentSetRejections.filter((e) => {
      if (filter.entityKind && e.entity?.kind !== filter.entityKind) return false;
      if (filter.id && e.entity?.id !== filter.id) return false;
      return true;
    });
  }, [runtime.recentSetRejections, filter]);
}

export function useWriteQueue(): { pending: number; byEntity: Record<string, number> } {
  const runtime = useKitRuntime();
  return React.useMemo(() => ({ pending: 0, byEntity: {} }), [runtime.snapshot.sync.status]);
}

export function useKitSync(): { status: "idle" | "loading" | "saving" | "error"; lastError?: SetError } {
  const runtime = useKitRuntime();
  const s = runtime.snapshot.sync;
  if (s.status === "loading") return { status: "loading" };
  if (s.status === "saving") return { status: "saving" };
  if (s.status === "error")
    return {
      status: "error",
      lastError: { kind: "Internal", message: s.error instanceof Error ? s.error.message : String(s.error ?? "") },
    };
  return { status: "idle" };
}

function kitCtlSetError(e: unknown): SetError {
  return { kind: "Internal", message: e instanceof Error ? e.message : String(e) };
}

/** Polls {@link KitStoreClient.backboneStatus} when a WASM client is mounted (e.g. coordinator / semio-store parity in the browser). */
export function useBackboneStatus(pollMs: number = 5000): {
  status: BackboneStatusDto | null;
  pending: boolean;
  lastError?: SetError;
  refresh: () => Promise<void>;
} {
  const runtime = useKitRuntime();
  const client = runtime.kitClient;
  const [status, setStatus] = React.useState<BackboneStatusDto | null>(null);
  const [pending, setPending] = React.useState(false);
  const [lastError, setLastError] = React.useState<SetError | undefined>(undefined);

  const refresh = React.useCallback(async () => {
    if (!client?.backboneStatus) return;
    setPending(true);
    setLastError(undefined);
    try {
      const s = await client.backboneStatus();
      setStatus(s);
    } catch (e) {
      const err = kitCtlSetError(e);
      setLastError(err);
      runtime.pushSetRejection(err);
    } finally {
      setPending(false);
    }
  }, [client, runtime.pushSetRejection]);

  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  React.useEffect(() => {
    if (!client || pollMs <= 0) return;
    const t = setInterval(() => {
      void refresh();
    }, pollMs);
    return () => clearInterval(t);
  }, [client, pollMs, refresh]);

  return { status, pending, lastError, refresh };
}

export function useAttachBackbone(): {
  attach: (cfg: BackboneConfig) => Promise<SetResult>;
  detach: () => Promise<SetResult>;
  pending: boolean;
  lastError?: SetError;
} {
  const runtime = useKitRuntime();
  const client = runtime.kitClient;
  const [pending, setPending] = React.useState(false);
  const [lastError, setLastError] = React.useState<SetError | undefined>(undefined);

  const attach = React.useCallback(
    async (cfg: BackboneConfig) => {
      if (!client?.attachBackbone || !client?.detachBackbone) {
        const err: SetError = { kind: "Internal", message: "no KitStoreClient on runtime" };
        setLastError(err);
        runtime.pushSetRejection(err);
        return { ok: false, error: err } as const;
      }
      setPending(true);
      setLastError(undefined);
      try {
        const r = await client.attachBackbone(cfg);
        if (!r.ok) {
          setLastError(r.error);
          runtime.pushSetRejection(r.error);
        }
        return r;
      } catch (e) {
        const err = kitCtlSetError(e);
        setLastError(err);
        runtime.pushSetRejection(err);
        return { ok: false, error: err } as const;
      } finally {
        setPending(false);
      }
    },
    [client, runtime.pushSetRejection],
  );

  const detach = React.useCallback(async () => {
    if (!client?.detachBackbone) {
      const err: SetError = { kind: "Internal", message: "no KitStoreClient on runtime" };
      setLastError(err);
      runtime.pushSetRejection(err);
      return { ok: false, error: err } as const;
    }
    setPending(true);
    setLastError(undefined);
    try {
      const r = await client.detachBackbone();
      if (!r.ok) {
        setLastError(r.error);
        runtime.pushSetRejection(r.error);
      }
      return r;
    } catch (e) {
      const err = kitCtlSetError(e);
      setLastError(err);
      runtime.pushSetRejection(err);
      return { ok: false, error: err } as const;
    } finally {
      setPending(false);
    }
  }, [client, runtime.pushSetRejection]);

  return { attach, detach, pending, lastError };
}

export function useDetachBackbone(): {
  detach: () => Promise<SetResult>;
  pending: boolean;
  lastError?: SetError;
} {
  const runtime = useKitRuntime();
  const client = runtime.kitClient;
  const [pending, setPending] = React.useState(false);
  const [lastError, setLastError] = React.useState<SetError | undefined>(undefined);

  const detach = React.useCallback(async () => {
    if (!client?.detachBackbone) {
      const err: SetError = { kind: "Internal", message: "no KitStoreClient on runtime" };
      setLastError(err);
      runtime.pushSetRejection(err);
      return { ok: false, error: err } as const;
    }
    setPending(true);
    setLastError(undefined);
    try {
      const r = await client.detachBackbone();
      if (!r.ok) {
        setLastError(r.error);
        runtime.pushSetRejection(r.error);
      }
      return r;
    } catch (e) {
      const err = kitCtlSetError(e);
      setLastError(err);
      runtime.pushSetRejection(err);
      return { ok: false, error: err } as const;
    } finally {
      setPending(false);
    }
  }, [client, runtime.pushSetRejection]);

  return { detach, pending, lastError };
}

export function useListConflicts(): {
  conflicts: KitConflict[];
  refresh: () => Promise<void>;
  pending: boolean;
  lastError?: SetError;
} {
  const runtime = useKitRuntime();
  const client = runtime.kitClient;
  const [conflicts, setConflicts] = React.useState<KitConflict[]>([]);
  const [pending, setPending] = React.useState(false);
  const [lastError, setLastError] = React.useState<SetError | undefined>(undefined);

  const refresh = React.useCallback(async () => {
    if (!client?.listConflicts) return;
    setPending(true);
    setLastError(undefined);
    try {
      const rows = await client.listConflicts();
      setConflicts(rows);
    } catch (e) {
      const err = kitCtlSetError(e);
      setLastError(err);
      runtime.pushSetRejection(err);
      setConflicts([]);
    } finally {
      setPending(false);
    }
  }, [client, runtime.pushSetRejection]);

  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  return { conflicts, refresh, pending, lastError };
}

export function useResolveConflict(): {
  resolve: (id: string, strategy: ConflictResolution) => Promise<SetResult>;
  pending: boolean;
  lastError?: SetError;
} {
  const runtime = useKitRuntime();
  const client = runtime.kitClient;
  const [pending, setPending] = React.useState(false);
  const [lastError, setLastError] = React.useState<SetError | undefined>(undefined);

  const resolve = React.useCallback(
    async (id: string, strategy: ConflictResolution) => {
      if (!client?.resolveConflict) {
        const err: SetError = { kind: "Internal", message: "no KitStoreClient on runtime" };
        setLastError(err);
        runtime.pushSetRejection(err);
        return { ok: false, error: err } as const;
      }
      setPending(true);
      setLastError(undefined);
      try {
        const r = await client.resolveConflict(id, strategy);
        if (!r.ok) {
          setLastError(r.error);
          runtime.pushSetRejection(r.error);
        }
        return r;
      } catch (e) {
        const err = kitCtlSetError(e);
        setLastError(err);
        runtime.pushSetRejection(err);
        return { ok: false, error: err } as const;
      } finally {
        setPending(false);
      }
    },
    [client, runtime.pushSetRejection],
  );

  return { resolve, pending, lastError };
}

export function useSyncNow(): {
  sync: () => Promise<SetResult>;
  pending: boolean;
  lastError?: SetError;
} {
  const runtime = useKitRuntime();
  const client = runtime.kitClient;
  const [pending, setPending] = React.useState(false);
  const [lastError, setLastError] = React.useState<SetError | undefined>(undefined);

  const sync = React.useCallback(async () => {
    if (!client?.syncNow) {
      const err: SetError = { kind: "Internal", message: "no KitStoreClient on runtime" };
      setLastError(err);
      runtime.pushSetRejection(err);
      return { ok: false, error: err } as const;
    }
    setPending(true);
    setLastError(undefined);
    try {
      const r = await client.syncNow();
      if (!r.ok) {
        setLastError(r.error);
        runtime.pushSetRejection(r.error);
      }
      return r;
    } catch (e) {
      const err = kitCtlSetError(e);
      setLastError(err);
      runtime.pushSetRejection(err);
      return { ok: false, error: err } as const;
    } finally {
      setPending(false);
    }
  }, [client, runtime.pushSetRejection]);

  return { sync, pending, lastError };
}

export function useWriteIndicator(status: WriteStatus): {
  disabled: boolean;
  spinning: boolean;
  error?: SetError;
  warning?: SetError;
} {
  if (status.kind === "readonly") return { disabled: true, spinning: false };
  if (status.kind === "pending") return { disabled: true, spinning: true, error: undefined, warning: undefined };
  if (status.kind === "error") return { disabled: false, spinning: false, error: status.lastError };
  return { disabled: false, spinning: false };
}

export function useOptimistic<T>(triad: HookTriad<T>): {
  display: T;
  draft: T;
  setDraft: (next: SetStateAction<T>) => void;
  commit: () => Promise<SetResult>;
  reset: () => void;
  status: WriteStatus;
  dirty: boolean;
} {
  const [value, setValue, status] = triad;
  const [draft, setDraft] = React.useState<T | undefined>(undefined);
  const dirty = draft !== undefined;
  const display = (dirty ? draft : value) as T;
  const commit = React.useCallback(async () => {
    if (draft === undefined) return { ok: true } as const;
    const r = await setValue(draft);
    if (r.ok) setDraft(undefined);
    return r;
  }, [draft, setValue]);
  const reset = React.useCallback(() => setDraft(undefined), []);
  const setDraftFn = React.useCallback(
    (next: SetStateAction<T>) => {
      setDraft((d) => {
        const base = (d !== undefined ? d : value) as T;
        return typeof next === "function" ? (next as (p: T) => T)(base) : next;
      });
    },
    [value],
  );
  return {
    display,
    draft: (draft !== undefined ? draft : value) as T,
    setDraft: setDraftFn,
    commit,
    reset,
    status,
    dirty,
  };
}

/**
 * Local draft over a {@link HookTriad}: mirror server value, edit locally, {@link commit} async-writes;
 * on rejection the draft is kept; {@link status} comes from the triad for {@link useWriteIndicator}.
 */
export function useDraft<T>(triad: HookTriad<T>): {
  value: T;
  setDraft: (next: SetStateAction<T>) => void;
  commit: () => Promise<SetResult>;
  reset: () => void;
  status: WriteStatus;
  error: SetError | undefined;
} {
  const [server, setServer, status] = triad;
  const [draft, setDraft] = React.useState<T | undefined>(undefined);
  const value = (draft !== undefined ? draft : server) as T;
  const setDraftFn = React.useCallback(
    (next: SetStateAction<T>) => {
      setDraft((d) => {
        const base = (d !== undefined ? d : server) as T;
        return typeof next === "function" ? (next as (p: T) => T)(base) : next;
      });
    },
    [server],
  );
  const commit = React.useCallback(async () => {
    if (draft === undefined) return { ok: true } as const;
    const r = await setServer(draft);
    if (r.ok) setDraft(undefined);
    return r;
  }, [draft, setServer]);
  const reset = React.useCallback(() => setDraft(undefined), []);
  const error = status.kind === "error" ? status.lastError : undefined;
  return { value, setDraft: setDraftFn, commit, reset, status, error };
}

// #region 🎛️KitStoreClient command hooks (WASM / worker RPCs)

export function useClusterPieces(): {
  run: (designId: string, pieceIds: string[], clusterName: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceIds: string[], clusterName: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.clusterPieces(designId, pieceIds, clusterName);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useDragPieces(): {
  run: (designId: string, pieceIds: string[], du: number, dv: number) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceIds: string[], du: number, dv: number) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.dragPieces(designId, pieceIds, du, dv);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useMovePieces(): {
  run: (designId: string, pieceIds: string[], gap: number, shift: number, rise: number) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceIds: string[], gap: number, shift: number, rise: number) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.movePieces(designId, pieceIds, gap, shift, rise);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useFixPieces(): {
  run: (designId: string, pieceIds: string[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceIds: string[]) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.fixPieces(designId, pieceIds);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useFlattenDesign(): { run: (designId: string) => Promise<SetResult>; status: WriteStatus } {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.flattenDesign(designId);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useExpandDesign(): {
  run: (parentDesignId: string, nestedDesignId: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (parentDesignId: string, nestedDesignId: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.expandDesign(parentDesignId, nestedDesignId);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useDeleteConnection(): {
  run: (designId: string, connectionId: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, connectionId: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.deleteConnection(designId, connectionId);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useChangePieceType(): {
  run: (designId: string, pieceId: string, newTypeId: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceId: string, newTypeId: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.changePieceType(designId, pieceId, newTypeId);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useUndo(): { run: () => Promise<SetResult>; status: WriteStatus } {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(async () => {
    if (!runtime.kitClient || !runtime.canWrite) {
      const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
      setStatus({ kind: "error", pending: 0, lastError: e });
      return { ok: false, error: e } as const;
    }
    setStatus({ kind: "pending", pending: 1 });
    const r = await runtime.kitClient.undo();
    if (!r.ok) {
      runtime.pushSetRejection(r.error);
      setStatus({ kind: "error", pending: 0, lastError: r.error });
      return r;
    }
    setStatus({ kind: "idle", pending: 0 });
    return r;
  }, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
  return { run, status };
}

export function useRedo(): { run: () => Promise<SetResult>; status: WriteStatus } {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(async () => {
    if (!runtime.kitClient || !runtime.canWrite) {
      const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
      setStatus({ kind: "error", pending: 0, lastError: e });
      return { ok: false, error: e } as const;
    }
    setStatus({ kind: "pending", pending: 1 });
    const r = await runtime.kitClient.redo();
    if (!r.ok) {
      runtime.pushSetRejection(r.error);
      setStatus({ kind: "error", pending: 0, lastError: r.error });
      return r;
    }
    setStatus({ kind: "idle", pending: 0 });
    return r;
  }, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
  return { run, status };
}

export function useCanUndo(): HookTriad<boolean> {
  const runtime = useKitRuntime();
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe("canUndo", () => c.canUndo(), kitEventAffectsCanUndoRedo, onChange);
    },
    [runtime.kitClient],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient) return EMPTY_KIT_READ_SNAP_FALSE;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot("canUndo");
  }, [runtime.kitClient]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const v = snap.data === true;
  const st: WriteStatus = !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [v, noopAsyncSet, st] as const;
}

export function useCanRedo(): HookTriad<boolean> {
  const runtime = useKitRuntime();
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe("canRedo", () => c.canRedo(), kitEventAffectsCanUndoRedo, onChange);
    },
    [runtime.kitClient],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient) return EMPTY_KIT_READ_SNAP_FALSE;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot("canRedo");
  }, [runtime.kitClient]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const v = snap.data === true;
  const st: WriteStatus = !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [v, noopAsyncSet, st] as const;
}

function useKitAddToKit(childKind: string): {
  run: (dto: unknown) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (dto: unknown) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      const kg = kitIdFromRuntime(runtime);
      if (!kg) {
        const e: SetError = { kind: "NotFound", message: "no active kit" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientAddChildByKind(runtime.kitClient, childKind, dto);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime, childKind],
  );
  return { run, status };
}

function useKitRemoveFromKit(childKind: string): {
  run: (childId: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (childId: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      const kg = kitIdFromRuntime(runtime);
      if (!kg) {
        const e: SetError = { kind: "NotFound", message: "no active kit" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientRemoveChildByKind(runtime.kitClient, childKind, childId);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime, childKind],
  );
  return { run, status };
}

export const useCreateAuthor = () => useKitAddToKit("Author");
export const useDeleteAuthor = () => useKitRemoveFromKit("Author");
export const useUpdateAuthor = (): {
  run: (authorId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (authorId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Author", authorId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreateType = () => useKitAddToKit("Type");
export const useDeleteType = () => useKitRemoveFromKit("Type");
export const useUpdateType = (): {
  run: (typeId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (typeId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Type", typeId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreateDesign = () => useKitAddToKit("Design");
export const useDeleteDesign = () => useKitRemoveFromKit("Design");
export const useUpdateDesign = (): {
  run: (designId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Design", designId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreateQuality = () => useKitAddToKit("Quality");
export const useDeleteQuality = () => useKitRemoveFromKit("Quality");
export const useUpdateQuality = (): {
  run: (qualityId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (qualityId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Quality", qualityId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreatePort = () => useKitAddToKit("Port");
export const useDeletePort = () => useKitRemoveFromKit("Port");
export const useUpdatePort = (): {
  run: (portId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (portId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Port", portId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreateTag = () => useKitAddToKit("Tag");
export const useDeleteTag = () => useKitRemoveFromKit("Tag");
export const useUpdateTag = (): {
  run: (tagId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (tagId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Tag", tagId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreateConcept = () => useKitAddToKit("Concept");
export const useDeleteConcept = () => useKitRemoveFromKit("Concept");

export const useAddFile = () => useKitAddToKit("File");
export const useRemoveFile = () => useKitRemoveFromKit("File");
export const useUpdateFile = (): {
  run: (fileId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (fileId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("File", fileId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export const useCreateFolder = () => useKitAddToKit("Folder");
export const useDeleteFolder = () => useKitRemoveFromKit("Folder");
export const useUpdateFolder = (): {
  run: (folderId: string, patch: Record<string, unknown>) => Promise<SetResult>;
  status: WriteStatus;
} => {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (folderId: string, patch: Record<string, unknown>) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const [field, value] of Object.entries(patch)) {
        const cmds = buildSchemaEntityChangeCommands("Folder", folderId, field, value, null);
        if (!cmds.length) continue;
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
};

export function useMoveToFolder(): {
  run: (fileId: string, targetFolderId: string | null) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (fileId: string, targetFolderId: string | null) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      void fileId;
      void targetFolderId;
      const r: SetResult = {
        ok: false,
        error: { kind: "NotSupported", message: "file.folder move is not mapped to ChangeKitCommand yet" },
      };
      runtime.pushSetRejection(r.error);
      setStatus({ kind: "error", pending: 0, lastError: r.error });
      return r;
    },
    [runtime],
  );
  return { run, status };
}

export type KitArtifactFolderKind = "type" | "design" | "quality" | "file" | "folder";

/**
 * Move a kit artifact into a folder (or to root) — not yet mapped to typed {@link KitStoreClient.submitChangeKitCommands}.
 */
export function useMoveKitArtifactToFolder(): {
  run: (artifactKind: KitArtifactFolderKind, artifactId: string, folderId: string | null) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (artifactKind: KitArtifactFolderKind, artifactId: string, folderId: string | null) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      void artifactId;
      void folderId;
      const r: SetResult = {
        ok: false,
        error: { kind: "NotSupported", message: `move kit artifact folder: ${artifactKind} (not wired to ChangeKitCommand yet)` },
      };
      runtime.pushSetRejection(r.error);
      setStatus({ kind: "error", pending: 0, lastError: r.error });
      return r;
    },
    [runtime],
  );
  return { run, status };
}

export function useImportKit(): { run: () => Promise<SetResult>; status: WriteStatus } {
  const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
  const run = React.useCallback(async () => {
    return {
      ok: false,
      error: { kind: "InvalidValue", message: "useImportKit is wired from sketchpadMachine / host; not a KitStoreClient RPC" },
    } as const;
  }, []);
  return { run, status };
}

export function useExportKit(): { run: () => Promise<SetResult>; status: WriteStatus } {
  const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
  const run = React.useCallback(async () => {
    return {
      ok: false,
      error: { kind: "InvalidValue", message: "useExportKit is wired from sketchpadMachine / host; not a KitStoreClient RPC" },
    } as const;
  }, []);
  return { run, status };
}

export function useAddConnections(): {
  run: (designId: string, connections: unknown[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const add = useAddConnection();
  const run = React.useCallback(
    async (designId: string, connections: unknown[]) => {
      for (const c of connections) {
        const r = await add.run(designId, c);
        if (!r.ok) return r;
      }
      return { ok: true } as const;
    },
    [add],
  );
  return { run, status: add.status };
}

export const useRemoveConnection = useDeleteConnection;

export function useRemoveConnections(): {
  run: (designId: string, connectionIds: string[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, connectionIds: string[]) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const cg of connectionIds) {
        const r = await runtime.kitClient.deleteConnection(designId, cg);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime],
  );
  return { run, status };
}

export function useDeleteSelected(): { run: () => Promise<SetResult>; status: WriteStatus } {
  const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
  const run = React.useCallback(async () => {
    return { ok: false, error: { kind: "InvalidValue", message: "useDeleteSelected is UI/selection; use sketchpad actor" } } as const;
  }, []);
  return { run, status };
}

export function useDeselectAll(): { run: () => Promise<SetResult>; status: WriteStatus } {
  const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
  const run = React.useCallback(async () => {
    return { ok: false, error: { kind: "InvalidValue", message: "useDeselectAll is UI/selection; use sketchpad actor" } } as const;
  }, []);
  return { run, status };
}

export function usePasteDesignSelection(): {
  run: (designId: string, selection: KitJsonTreeDto, plane?: PlanePlain | null) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, selection: KitJsonTreeDto, plane?: PlanePlain | null) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.pasteDesignSelection(designId, selection, plane ?? null);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useCreateHangingPieces(): {
  run: (designId: string, typeIds: string[], plane: PlanePlain) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, typeIds: string[], plane: PlanePlain) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.createHangingPieces(designId, typeIds, plane);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useCreateConnectedPiece(): {
  run: (designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useCreateFixedPiece(): {
  run: (designId: string, typeId: string, plane: PlanePlain) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, typeId: string, plane: PlanePlain) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await runtime.kitClient.createFixedPiece(designId, typeId, plane);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useDeletePiece(): {
  run: (designId: string, pieceId: string) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceId: string) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientRemovePiece(runtime.kitClient, designId, pieceId);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useCreatePiece(): {
  run: (designId: string, piece: unknown) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, piece: unknown) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientAddPiece(runtime.kitClient, designId, piece);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

/** @alias {@link useCreatePiece} */
export const useAddPiece = useCreatePiece;

export function useAddPieces(): {
  run: (designId: string, pieces: unknown[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const c = useCreatePiece();
  const run = React.useCallback(
    async (designId: string, pieces: unknown[]) => {
      for (const p of pieces) {
        const r = await c.run(designId, p);
        if (!r.ok) return r;
      }
      return { ok: true } as const;
    },
    [c],
  );
  return { run, status: c.status };
}

/** @alias {@link useDeletePiece} */
export const useRemovePiece = useDeletePiece;

export function useRemovePieces(): {
  run: (designId: string, pieceIds: string[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const del = useDeletePiece();
  const run = React.useCallback(
    async (designId: string, pieceIds: string[]) => {
      for (const g of pieceIds) {
        const r = await del.run(designId, g);
        if (!r.ok) return r;
      }
      return { ok: true } as const;
    },
    [del],
  );
  return { run, status: del.status };
}

export function useAddConnection(): {
  run: (designId: string, connection: unknown) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, connection: unknown) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientAddConnection(runtime.kitClient, designId, connection);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useUpdatePiece(): {
  run: (designId: string, pieceId: string, patch: unknown) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, pieceId: string, patch: unknown) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientUpdatePiece(runtime.kitClient, designId, pieceId, patch);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useUpdatePieces(): {
  run: (designId: string, updates: { id: string; diff: unknown }[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, updates: { id: string; diff: unknown }[]) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const u of updates) {
        const r = await kitStoreClientUpdatePiece(runtime.kitClient, designId, u.id, u.diff);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useUpdateConnection(): {
  run: (designId: string, connectionId: string, patch: unknown) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, connectionId: string, patch: unknown) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      const r = await kitStoreClientUpdateConnection(runtime.kitClient, designId, connectionId, patch);
      if (!r.ok) {
        runtime.pushSetRejection(r.error);
        setStatus({ kind: "error", pending: 0, lastError: r.error });
        return r;
      }
      setStatus({ kind: "idle", pending: 0 });
      return r;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

export function useUpdateConnections(): {
  run: (designId: string, updates: { id: string; diff: unknown }[]) => Promise<SetResult>;
  status: WriteStatus;
} {
  const runtime = useKitRuntime();
  const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
  const run = React.useCallback(
    async (designId: string, updates: { id: string; diff: unknown }[]) => {
      if (!runtime.kitClient || !runtime.canWrite) {
        const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
        setStatus({ kind: "error", pending: 0, lastError: e });
        return { ok: false, error: e } as const;
      }
      setStatus({ kind: "pending", pending: 1 });
      for (const u of updates) {
        const r = await kitStoreClientUpdateConnection(runtime.kitClient, designId, u.id, u.diff);
        if (!r.ok) {
          runtime.pushSetRejection(r.error);
          setStatus({ kind: "error", pending: 0, lastError: r.error });
          return r;
        }
      }
      setStatus({ kind: "idle", pending: 0 });
      return { ok: true } as const;
    },
    [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
  );
  return { run, status };
}

/** @emoji 🧾 Normalizes live-read snapshot data into a piece hierarchy metadata map (Map or plain record from older hubs). */
function __semioPiecesPlacementMapFromReadSnap(data: unknown): ReadonlyMap<string, PiecePlacementRowDto> {
  if (data instanceof Map) return data as ReadonlyMap<string, PiecePlacementRowDto>;
  if (data && typeof data === "object" && !Array.isArray(data)) {
    return new Map(Object.entries(data as Record<string, PiecePlacementRowDto>));
  }
  return new Map();
}

/** Piece hierarchy + flat pose map from the Rust GraphQL worker (`getPiecesMetadata`) via {@link DesignStore.readPiecesPlacementMetadataMap}. */
export function usePiecesMetadataMap(designId?: string): HookRead<ReadonlyMap<string, PiecePlacementRowDto>> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `pmd:${designId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return new Map<string, PiecePlacementRowDto>();
          return ks.design(d, readScope).readPiecesPlacementMetadataMap();
        },
        (ev) => kitEventTouchesDesign(ev as KitEvent, d),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = __semioPiecesPlacementMapFromReadSnap(snap.data);
  const status: WriteStatus = !designId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/** @emoji 📌 Piece DTO rows for a design via {@link DesignStore.readPiecesFullRows}. */
export function useKitPieces(designId?: string): HookRead<any[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `k-pieces:${designId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readPiecesFullRows();
        },
        (ev) => kitEventTouchesDesign(ev as KitEvent, d),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : [];
  const status: WriteStatus = !designId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/** @emoji 📌 Connection DTO rows for a design via {@link DesignStore.readConnectionsFullRows}. */
export function useKitConnections(designId?: string): HookRead<any[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `k-conns:${designId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readConnectionsFullRows();
        },
        (ev) => kitEventTouchesDesign(ev as KitEvent, d),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : [];
  const status: WriteStatus = !designId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/** @emoji 📌 Shallow design catalog rows via {@link KitStore.getDesigns}. */
export function useKitDesignsShallow(): HookRead<any[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `k-dshallow:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.getDesigns(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : [];
  const status: WriteStatus = !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/** @emoji 📌 Shallow kind catalog rows via {@link KitStore.getTypes}. */
export function useKitTypesShallow(): HookRead<any[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `k-tshallow:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.getTypes(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : [];
  const status: WriteStatus = !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/** @emoji 📌 Shallow author rows via {@link KitStore.getAuthors}. */
export function useKitAuthorsShallow(): HookRead<any[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `k-ashallow:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.getAuthors(readScope);
        },
        kitEventAffectsCanUndoRedo,
        onChange,
      );
    },
    [runtime.kitClient, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : [];
  const status: WriteStatus = !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

const EMPTY_SCOPED_PIECES: any[] = [];
const EMPTY_SCOPED_CONNECTIONS: any[] = [];

/** @emoji 📌 Piece rows for the active {@link DesignScope} (sketchpad-style, no HookTriad). */
export function usePieces(): any[] {
  const designId = useDesignScope()?.id;
  const [arr] = useKitPieces(designId);
  return Array.isArray(arr) ? arr : EMPTY_SCOPED_PIECES;
}

/** @emoji 📌 Connection rows for the active {@link DesignScope} (sketchpad-style, no HookTriad). */
export function useConnections(): any[] {
  const designId = useDesignScope()?.id;
  const [arr] = useKitConnections(designId);
  return Array.isArray(arr) ? arr : EMPTY_SCOPED_CONNECTIONS;
}

/** Merged read/write status for collection hooks (see {@link useTypes}). */
function mergeWriteStatuses(...statuses: WriteStatus[]): WriteStatus {
  const readonly_ = statuses.some((s) => s.kind === "readonly");
  if (readonly_) {
    return { kind: "readonly", pending: 0 };
  }
  let pending = 0;
  for (const s of statuses) {
    if (s.kind === "pending") {
      pending += s.pending;
    }
  }
  if (pending > 0) {
    return { kind: "pending", pending };
  }
  const err = statuses.find((s) => s.kind === "error");
  if (err && err.kind === "error") {
    return err;
  }
  return { kind: "idle", pending: 0 };
}

/**
 * Full kit `types` collection: full + shallow DTOs, metadata, ids, CRUD, and combined status.
 * Prefer this over ad-hoc {@link useKitTypesShallow} + separate create/delete hooks in app code.
 */
export type UseTypesResult = {
  types: any[];
  shallowTypes: TypeShallow[];
  typesMetadata: TypeMetadataDto[];
  typeIds: string[];
  createType: (dto: unknown) => Promise<SetResult>;
  deleteType: (typeId: string) => Promise<SetResult>;
  status: WriteStatus;
};

export function useTypes(): UseTypesResult {
  const [types, rpcStatus] = useKitTypesShallow();
  const { run: createType, status: createStatus } = useCreateType();
  const { run: deleteType, status: deleteStatus } = useDeleteType();

  const shallowTypes = React.useMemo(() => {
    if (!Array.isArray(types)) {
      return [];
    }
    return types.map((t) => TypeShallowSchema.parse((t as Type).toDto()));
  }, [types]);

  const typesMetadata = React.useMemo(() => {
    if (!Array.isArray(types)) {
      return [];
    }
    return types.map((t) => TypeMetadataDtoSchema.parse((t as Type).toDto()));
  }, [types]);

  const typeIds = React.useMemo(() => {
    if (!Array.isArray(types)) {
      return [];
    }
    return types.map((t) => t?.id).filter((x): x is string => typeof x === "string");
  }, [types]);

  const status = React.useMemo(() => mergeWriteStatuses(rpcStatus, createStatus, deleteStatus), [rpcStatus, createStatus, deleteStatus]);

  return {
    types: Array.isArray(types) ? types : [],
    shallowTypes,
    typesMetadata,
    typeIds,
    createType,
    deleteType,
    status,
  };
}

/**
 * Full kit `designs` collection: full + shallow DTOs, metadata, ids, CRUD, and combined status.
 */
export type UseDesignsResult = {
  designs: any[];
  shallowDesigns: DesignShallow[];
  designsMetadata: DesignMetadataDto[];
  designIds: string[];
  createDesign: (dto: unknown) => Promise<SetResult>;
  deleteDesign: (designId: string) => Promise<SetResult>;
  status: WriteStatus;
};

export function useDesigns(): UseDesignsResult {
  const [designs, rpcStatus] = useKitDesignsShallow();
  const { run: createDesign, status: createStatus } = useCreateDesign();
  const { run: deleteDesign, status: deleteStatus } = useDeleteDesign();

  const shallowDesigns = React.useMemo(() => {
    if (!Array.isArray(designs)) {
      return [];
    }
    return designs.map((d) => DesignShallowSchema.parse((d as Design).toDto()));
  }, [designs]);

  const designsMetadata = React.useMemo(() => {
    if (!Array.isArray(designs)) {
      return [];
    }
    return designs.map((d) => DesignMetadataDtoSchema.parse((d as Design).toDto()));
  }, [designs]);

  const designIds = React.useMemo(() => {
    if (!Array.isArray(designs)) {
      return [];
    }
    return designs.map((d) => d?.id).filter((x): x is string => typeof x === "string");
  }, [designs]);

  const status = React.useMemo(() => mergeWriteStatuses(rpcStatus, createStatus, deleteStatus), [rpcStatus, createStatus, deleteStatus]);

  return {
    designs: Array.isArray(designs) ? designs : [],
    shallowDesigns,
    designsMetadata,
    designIds,
    createDesign,
    deleteDesign,
    status,
  };
}

/** @emoji 📌 Author rows (`authorsShallowJson`) — use {@link useKitAuthorsShallow} for the raw `[value, status]` pair. */
export function useAuthors(): HookRead<any[]> {
  return useKitAuthorsShallow();
}

export function usePieceMetadata(designId?: string, pieceId?: string): HookRead<PiecePlacementRowDto | undefined> {
  const [map, status] = usePiecesMetadataMap(designId);
  const value = React.useMemo(() => (pieceId ? map?.get(pieceId) : undefined), [map, pieceId]);
  return [value, status] as const;
}

/**
 * Flattened piece plane from {@link PieceStore.readFlatPlane} (`readPieceFlatPlaneCommand`).
 */
export function usePieceFlatPlane(designId?: string, pieceId?: string): HookRead<any> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `pfp:${designId ?? ""}:${pieceId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId || !pieceId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const p = pieceId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return undefined;
          return ks.piece(d, p, readScope).readFlatPlane();
        },
        (ev) => kitEventAffectsPieceLiveRead(ev, d, p),
        onChange,
      );
    },
    [runtime.kitClient, designId, pieceId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId || !pieceId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, pieceId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const status: WriteStatus = !designId || !pieceId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [snap.data, status] as const;
}

/** Flattened piece center from {@link PieceStore.readFlatCenter} (`readPieceFlatCenterCommand`). */
export function usePieceFlatCenter(designId?: string, pieceId?: string): HookRead<any> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `pfc:${designId ?? ""}:${pieceId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId || !pieceId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const p = pieceId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return undefined;
          return ks.piece(d, p, readScope).readFlatCenter();
        },
        (ev) => kitEventAffectsPieceLiveRead(ev, d, p),
        onChange,
      );
    },
    [runtime.kitClient, designId, pieceId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId || !pieceId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, pieceId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const status: WriteStatus = !designId || !pieceId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [snap.data, status] as const;
}

export function useIsConnectedPiece(designId?: string, pieceId?: string): HookRead<boolean> {
  const [meta, status] = usePieceMetadata(designId, pieceId);
  const value = React.useMemo(() => !!meta?.parentPieceId, [meta]);
  return [value, status] as const;
}

export function usePieceDepth(designId?: string, pieceId?: string): HookRead<number> {
  const [meta, status] = usePieceMetadata(designId, pieceId);
  const value = React.useMemo(() => (typeof meta?.depth === "number" ? meta.depth : 0), [meta]);
  return [value, status] as const;
}

export function useFixedPieceId(designId?: string, pieceId?: string): HookRead<string | undefined> {
  const [meta, status] = usePieceMetadata(designId, pieceId);
  const value = React.useMemo(() => meta?.fixedPieceId, [meta]);
  return [value, status] as const;
}

export function useParentPieceId(designId?: string, pieceId?: string): HookRead<string | undefined> {
  const [meta, status] = usePieceMetadata(designId, pieceId);
  const value = React.useMemo(() => meta?.parentPieceId ?? undefined, [meta]);
  return [value, status] as const;
}

export function usePieceParentConnection(designId?: string, pieceId?: string): HookRead<any | undefined> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `ppc:${designId ?? ""}:${pieceId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId || !pieceId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const p = pieceId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return undefined;
          return ks.piece(d, p, readScope).readParentConnectionFull();
        },
        (ev) => kitEventAffectsPieceLiveRead(ev, d, p),
        onChange,
      );
    },
    [runtime.kitClient, designId, pieceId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId || !pieceId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, pieceId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const status: WriteStatus = !designId || !pieceId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [snap.data, status] as const;
}

export function useIncludedDesigns(designId?: string): HookRead<any[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `inc:${designId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readIncludedDesigns();
        },
        (ev) => kitEventAffectsReplaceableCatalogRead(ev, d, new Set()),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = Array.isArray(snap.data) ? snap.data : [];
  const status: WriteStatus = !designId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/**
 * Clusterable piece id groups for explode / cluster UI (`readDesignClusterableGroupsCommand`).
 */
export function useDesignClusterableGroups(designId?: string, selection?: ReadonlyArray<string>): HookRead<ReadonlyArray<ReadonlyArray<{ readonly id: string }>>> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const selectionDep = React.useMemo(() => JSON.stringify(selection ?? []), [selection]);
  const key = `clu:${designId ?? ""}:${selectionDep}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const s = selection ?? [];
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readClusterableGroups(s);
        },
        (ev) => kitEventAffectsReplaceableCatalogRead(ev, d, new Set(s)),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key, selectionDep, selection],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = (Array.isArray(snap.data) ? snap.data : []) as ReadonlyArray<ReadonlyArray<{ readonly id: string }>>;
  const status: WriteStatus = !designId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/** Sum of prop values linked to `qualityId` in the design (`readDesignQualitySumCommand`). */
export function useDesignQualitySum(designId?: string, qualityId?: string): HookRead<number> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `dqs:${designId ?? ""}:${qualityId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId || !qualityId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const q = qualityId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return 0;
          const v = await ks.design(d, readScope).readQualitySum(q);
          return typeof v === "number" && !Number.isNaN(v) ? v : 0;
        },
        (ev) => kitEventAffectsDesignQualitySumRead(ev, d, q),
        onChange,
      );
    },
    [runtime.kitClient, designId, qualityId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId || !qualityId) return EMPTY_KIT_READ_SNAP_ZERO;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, qualityId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = typeof snap.data === "number" && !Number.isNaN(snap.data) ? snap.data : 0;
  const status: WriteStatus = !designId || !qualityId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

/**
 * Best matching representation for a kind given tag ids via {@link TypeStore.readBestRepresentation}.
 */
export function useTypeBestRepresentation(typeId?: string, tagIds?: ReadonlyArray<string>): HookRead<unknown> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const tagDep = React.useMemo(() => JSON.stringify(tagIds ?? []), [tagIds]);
  const tags = tagIds ?? [];
  const key = `tbr:${typeId ?? ""}:${tagDep}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !typeId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const t = typeId;
      const tg = tags;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return undefined;
          return ks.type(t, readScope).readBestRepresentation(tg);
        },
        (ev) => kitEventAffectsTypeScopedRead(ev, t),
        onChange,
      );
    },
    [runtime.kitClient, typeId, readScope, key, tagDep, tags],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !typeId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, typeId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const status: WriteStatus = !typeId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [snap.data, status] as const;
}

/** Colored connector rows from `session.wip` materialized store `types { connectors { color { css } } }`. */
export function useKitColoredConnectors(): HookRead<ReadonlyArray<unknown>> {
  const runtime = useKitRuntime();
  const key = "kcc";
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        () => c.readColoredConnectors().then((v) => (Array.isArray(v) ? v : [])),
        kitEventAffectsKitColoredConnectorsRead,
        onChange,
      );
    },
    [runtime.kitClient],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = (Array.isArray(snap.data) ? snap.data : []) as ReadonlyArray<unknown>;
  const status: WriteStatus = !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

export function useReplacableTypes(designId?: string, pieceIds?: string[]): HookRead<string[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const pieceKey = pieceIds?.join("\u0000") ?? "";
  const key = `rpt:${designId ?? ""}:${pieceKey}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      const sel = pieceIds ?? [];
      if (!runtime.kitClient || !designId || !sel.length) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const s = sel;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readReplaceableCatalogTypes(s);
        },
        (ev) => kitEventAffectsReplaceableCatalogRead(ev, d, new Set(s)),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key, pieceKey],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId || !pieceKey) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key, pieceKey]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = (Array.isArray(snap.data) ? snap.data : []) as string[];
  const status: WriteStatus = !designId || !pieceIds?.length || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

export function useReplacableDesigns(designId?: string, pieceIds?: string[]): HookRead<string[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const pieceKey = pieceIds?.join("\u0000") ?? "";
  const key = `rpd:${designId ?? ""}:${pieceKey}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      const sel = pieceIds ?? [];
      if (!runtime.kitClient || !designId || !sel.length) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      const s = sel;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readReplaceableCatalogDesigns(s);
        },
        (ev) => kitEventAffectsReplaceableCatalogRead(ev, d, new Set(s)),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key, pieceKey],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId || !pieceKey) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key, pieceKey]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = (Array.isArray(snap.data) ? snap.data : []) as string[];
  const status: WriteStatus = !designId || !pieceIds?.length || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

export function useExplodeableDesignNodes(designId?: string): HookRead<string[]> {
  const runtime = useKitRuntime();
  const readScope = useKitDataScope();
  const key = `exd:${designId ?? ""}:${kitReadScopeKey(readScope)}`;
  const subscribe = React.useCallback(
    (onChange: () => void) => {
      if (!runtime.kitClient || !designId) {
        onChange();
        return () => {};
      }
      const c = runtime.kitClient;
      const d = designId;
      return getSemioKitLiveReadStore(c).subscribe(
        key,
        async () => {
          const ks = kitStoreFromKitStoreClient(c);
          if (!ks) return [];
          return ks.design(d, readScope).readIncludedDesignIds();
        },
        (ev) => kitEventAffectsReplaceableCatalogRead(ev, d, new Set()),
        onChange,
      );
    },
    [runtime.kitClient, designId, readScope, key],
  );
  const getSnap = React.useCallback(() => {
    if (!runtime.kitClient || !designId) return EMPTY_KIT_READ_SNAP;
    return getSemioKitLiveReadStore(runtime.kitClient).getSnapshot(key);
  }, [runtime.kitClient, designId, key]);
  const snap = useSemioReadSnap(subscribe, getSnap, getSnap);
  const value = (Array.isArray(snap.data) ? snap.data : []) as string[];
  const status: WriteStatus = !designId || !runtime.kitClient
    ? { kind: "readonly", pending: 0 }
    : snap.pending > 0
      ? { kind: "pending", pending: snap.pending }
      : { kind: "idle", pending: 0 };
  return [value, status] as const;
}

// #endregion 🎛️KitStoreClient command hooks

export function useKitStore(): HookTriad<KitHostStore> {
  const runtime = useKitRuntime();
  return [runtime.store, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
}

export function useKitSnapshot(): HookTriad<KitHostStoreSnapshot> {
  const runtime = useKitRuntime();
  return [runtime.snapshot, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
}

const EMPTY_KIT_FILE_URLS = new Map<string, string>();

/**
 * Resolves a map of file keys to blob: URLs for the current kit (same data as the former sketchpad `useFileUrls`).
 */
export function useKitStoredFileUrls(): Map<string, string> {
  const [kitStore] = useKitStore();
  const cachedRef = React.useRef<{ key: string; map: Map<string, string> } | null>(null);
  const subscribe = React.useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.subscribe(callback);
    },
    [kitStore],
  );
  const getSnapshot = React.useCallback(() => {
    if (!kitStore) return EMPTY_KIT_FILE_URLS;
    const next = getStoredKitFileUrls(kitStore);
    const key = Array.from(next.entries())
      .map(([k, v]) => `${k}=${v}`)
      .join("|");
    if (cachedRef.current?.key === key) return cachedRef.current.map;
    cachedRef.current = { key, map: next };
    return next;
  }, [kitStore]);
  return React.useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

/** @emoji 📌Alias for {@link useKitStoredFileUrls} (sketchpad compatibility). */
export const useFileUrls = useKitStoredFileUrls;

/**
 * @emoji 📌Readable URL for a kit file (provider, embedded, or remote) — thin wrapper over `@semio/js` kit file helpers.
 */
export function useKitFileUrl(fileId: string | undefined): HookTriad<string | null> {
  const [kitStore] = useKitStore();
  const subscribe = React.useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.subscribe(callback);
    },
    [kitStore],
  );
  const getSnapshot = React.useCallback(() => {
    if (!kitStore || !fileId) return null;
    const kit = kitStore.getSnapshot().kit;
    const fileState = getOrCreateKitFileState(kitStore);
    const file = kit.files?.find((f: { id: string }) => f.id === fileId);
    if (!file) return null;
    const readableUrl = getReadableKitFileUrl(fileState, file);
    if (readableUrl) return readableUrl;
    const provider = getExistingKitFileProvider(kitStore);
    if (!provider) {
      return file.remote && isBrowserReadableFileUrl(file.remote) ? file.remote : null;
    }
    const storagePath = getKitFileStoragePath(kit, file);
    const providerUrl = provider.getUrl(kit.id, file.id, storagePath);
    if (!providerUrl) {
      return file.remote && isBrowserReadableFileUrl(file.remote) ? file.remote : null;
    }
    fileState.providerUrls.set(fileId, providerUrl);
    if (isBrowserReadableFileUrl(providerUrl)) return providerUrl;
    return file.remote && isBrowserReadableFileUrl(file.remote) ? file.remote : null;
  }, [kitStore, fileId]);
  const url = React.useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
  return [url, noopAsyncSet, { kind: "readonly" as const, pending: 0 }];
}

/**
 * @emoji 📌Resolves a blob: / object URL for a file (IndexedDB, binary store, provider, or fetch).
 */
export function useKitFileBlobUrl(fileId: string | undefined): {
  url: string | null;
  loading: boolean;
  error?: SetError;
  refresh: () => Promise<void>;
} {
  const [kitStore] = useKitStore();
  const [url, setUrl] = React.useState<string | null>(null);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<SetError | undefined>(undefined);
  const run = React.useCallback(async () => {
    if (!kitStore || !fileId) {
      setUrl(null);
      return;
    }
    setLoading(true);
    setError(undefined);
    try {
      const fileState = getOrCreateKitFileState(kitStore);
      const cached = fileState.objectUrls.get(fileId);
      if (cached) {
        setUrl(cached);
        return;
      }
      const pending = fileState.pendingBlobDownloads.get(fileId);
      if (pending) {
        const u = await pending;
        setUrl(u);
        return;
      }
      const p = (async () => {
        const kit = kitStore.getSnapshot().kit;
        const file = kit.files?.find((f: { id: string }) => f.id === fileId);
        if (!file) return null;
        const cachedBlob = fileState.blobs.get(fileId);
        if (cachedBlob) {
          return createKitFileObjectUrl(kitStore, fileId, cachedBlob);
        }
        const binary = kitStore as KitBinaryStore;
        if (typeof binary.readFile === "function") {
          const storagePath = getKitFileStoragePath(kit, file);
          const blob = await binary.readFile(storagePath);
          if (blob) {
            fileState.blobs.set(fileId, blob);
            return createKitFileObjectUrl(kitStore, fileId, blob);
          }
        }
        const provider = await getKitFileProvider(kitStore, kit.id);
        if (provider) {
          const storagePath = getKitFileStoragePath(kit, file);
          const blob = await provider.download(kit.id, file.id, storagePath);
          fileState.blobs.set(fileId, blob);
          const providerUrl = provider.getUrl(kit.id, file.id, storagePath);
          if (providerUrl) fileState.providerUrls.set(fileId, providerUrl);
          return createKitFileObjectUrl(kitStore, fileId, blob);
        }
        const readableUrl = getReadableKitFileUrl(fileState, file);
        if (readableUrl) {
          const blob = await fetchReadableKitFileBlob(readableUrl);
          if (blob) {
            fileState.blobs.set(fileId, blob);
            return createKitFileObjectUrl(kitStore, fileId, blob);
          }
        }
        return null;
      })();
      fileState.pendingBlobDownloads.set(fileId, p);
      const resolved = await p;
      fileState.pendingBlobDownloads.delete(fileId);
      setUrl(resolved);
    } catch (e) {
      setError({ kind: "Internal" as const, message: String(e) });
      setUrl(null);
    } finally {
      setLoading(false);
    }
  }, [kitStore, fileId]);
  React.useEffect(() => {
    void run();
  }, [run]);
  React.useEffect(() => {
    if (!kitStore) return () => {};
    return kitStore.subscribe(() => {
      void run();
    });
  }, [kitStore, run]);
  return { url, loading, error, refresh: run };
}

/**
 * @emoji 📌Embeds a dropped blob as data URL on the kit file record (`JsonFileKitStore` / compatible).
 */
export function useEmbedKitFile(): { run: (fileId: string, blob: Blob) => Promise<SetResult>; status: WriteStatus } {
  const [kitStore] = useKitStore();
  const [pending, setPending] = React.useState(0);
  const run = React.useCallback(
    async (fileId: string, blob: Blob) => {
      if (!kitStore) return { ok: false, error: { kind: "NotFound" as const, message: "no kit store" } };
      const embed = (kitStore as any).embedFileBlob;
      if (typeof embed !== "function") {
        return { ok: false, error: { kind: "NotSupported" as const, message: "embedFileBlob" } };
      }
      setPending((n) => n + 1);
      try {
        await embed.call(kitStore, fileId, blob);
        return { ok: true } as const;
      } catch (e) {
        return { ok: false, error: { kind: "Internal" as const, message: String(e) } };
      } finally {
        setPending((n) => n - 1);
      }
    },
    [kitStore],
  );
  const st: WriteStatus = pending > 0 ? { kind: "pending", pending } : { kind: "idle", pending: 0 };
  return { run, status: st };
}

/**
 * @emoji 📌Binary sidecar I/O on folder-backed or compatible stores.
 */
export function useKitBinary(): {
  read: (path: string) => Promise<Blob | null>;
  write: (path: string, blob: Blob) => Promise<void>;
  delete: (path: string) => Promise<void>;
  mkdir: (path: string) => Promise<void>;
  move: (from: string, to: string) => Promise<void>;
} {
  const [kitStore] = useKitStore();
  return React.useMemo(() => {
    const bs = kitStore as KitBinaryStore;
    if (!kitStore) {
      return {
        read: async () => null,
        write: async () => {},
        delete: async () => {},
        mkdir: async () => {},
        move: async () => {},
      };
    }
    return {
      read: (path: string) => (typeof bs.readFile === "function" ? bs.readFile(path) : Promise.resolve(null)),
      write: (path: string, blob: Blob) => (typeof bs.writeFile === "function" ? bs.writeFile(path, blob) : Promise.resolve()),
      delete: (path: string) => (typeof bs.deleteFile === "function" ? bs.deleteFile(path) : Promise.resolve()),
      mkdir: (path: string) => (typeof bs.createDirectory === "function" ? bs.createDirectory(path) : Promise.resolve()),
      move: (from: string, to: string) => (typeof bs.moveEntry === "function" ? bs.moveEntry(from, to) : Promise.resolve()),
    };
  }, [kitStore]);
}

/**
 * @emoji 📌Live `KitFileState` (provider URLs, blobs, object URLs) for the current `KitHostStore`.
 */
export function useKitFileState(): HookTriad<KitFileState> {
  const [kitStore] = useKitStore();
  const subscribe = React.useCallback(
    (cb: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.subscribe(cb);
    },
    [kitStore],
  );
  const snap = React.useCallback(() => {
    if (!kitStore) {
      return null as unknown as KitFileState;
    }
    return getOrCreateKitFileState(kitStore);
  }, [kitStore]);
  const state = React.useSyncExternalStore(subscribe, snap, snap);
  return [state, noopAsyncSet, { kind: "readonly" as const, pending: 0 }];
}

/**
 * @emoji 📌File / folder / remote / temporary kind for the current kit (registry or backbone).
 */
export function useKitPersistenceKind(): HookTriad<KitPersistenceInfo["kind"]> {
  const registry = useKitRegistrySafe();
  const activeFromRegistry = registry?.activeKitId;
  const fromKit = useActiveKitId();
  const active = activeFromRegistry ?? fromKit;
  const runtime = useKitRuntimeSafe();
  const pkind = React.useMemo(() => {
    if (registry && active) {
      const ent = registry.get(active);
      if (ent) return ent.persistence.kind;
    }
    if (runtime?.kitBackbone) {
      return inferPersistenceFromInit({ backbone: runtime.kitBackbone, store: runtime.store }).kind;
    }
    if (runtime?.store) {
      return inferPersistenceFromInit({ store: runtime.store }).kind;
    }
    return "temporary" as const;
  }, [registry, active, runtime]);
  return [pkind, noopAsyncSet, { kind: "readonly" as const, pending: 0 }];
}

/**
 * @emoji 📌Path/url metadata for the current kit’s persistence.
 */
export function useKitPersistenceSource(): HookTriad<KitPersistenceInfo | undefined> {
  const registry = useKitRegistrySafe();
  const active = registry?.activeKitId ?? useActiveKitId();
  const runtime = useKitRuntimeSafe();
  const v = React.useMemo(() => {
    if (registry && active) {
      return registry.get(active)?.persistence;
    }
    if (runtime?.kitBackbone) {
      return inferPersistenceFromInit({ backbone: runtime.kitBackbone, store: runtime.store });
    }
    if (runtime?.store) {
      return inferPersistenceFromInit({ store: runtime.store });
    }
    return { kind: "temporary" as const };
  }, [registry, active, runtime]);
  return [v, noopAsyncSet, { kind: "readonly" as const, pending: 0 }];
}

/** @internal Stable snapshot for {@link useOpenKitGuids} (avoids useSyncExternalStore thrash). */
const _OPEN_KIT_GUIDS_EMPTY = Object.freeze([] as readonly string[]);
let _openKitGuidsSnapKey = "";
let _openKitGuidsSnapArr: readonly string[] = _OPEN_KIT_GUIDS_EMPTY;

function getOpenKitGuidsSnapshot(): readonly string[] {
  const r = getKitRegistryBridge();
  const sorted = (r?.list() ?? []).slice().sort();
  const key = sorted.join("|");
  if (key !== _openKitGuidsSnapKey) {
    _openKitGuidsSnapKey = key;
    _openKitGuidsSnapArr = sorted.length === 0 ? _OPEN_KIT_GUIDS_EMPTY : (Object.freeze(sorted) as readonly string[]);
  }
  return _openKitGuidsSnapArr;
}

/** @emoji 📌Listed kit ids in the registry (empty if no provider); uses bridge so LayoutCanvas windows see ids without React context. */
export function useOpenKitGuids(): string[] {
  const snap = React.useSyncExternalStore(subscribeKitRegistryListChanged, getOpenKitGuidsSnapshot, getOpenKitGuidsSnapshot);
  return snap.length === 0 ? [] : [...snap];
}

/** @emoji 📌Active kit: registry selection first, else current {@link KitScope} id. */
export function useActiveKitGuid(): string | undefined {
  const r = useKitRegistrySafe();
  const fromProvider = useActiveKitId();
  return r?.activeKitId ?? fromProvider;
}

/**
 * @emoji 🪝 Attach read-only `snapshot` / `fileUrls` on {@link KitHostStore} for legacy design-store selectors; graph mutations use {@link applyKitHostGraphOp} / {@link executeSemioKitCommand}.
 */
export function attachSketchpadKitReadShell(kitStore: KitHostStore): void {
  const s = kitStore as any;
  if (s.__sketchpadKitUi) return;
  s.__sketchpadKitUi = true;
  s.snapshot = () => kitStore.getSnapshot().kit;
  Object.defineProperty(s, "fileUrls", { get: () => getStoredKitFileUrls(kitStore), configurable: true });
}

/** @emoji 🧾 Memoized {@link createKitCommandEngineExplicitOrigin} for a host {@link KitHostStore} (null when no store). */
export function useKitCommandEngineExplicitOrigin(kitStore: KitHostStore | null): ReturnType<typeof createKitCommandEngineExplicitOrigin> | null {
  return React.useMemo(() => (kitStore ? createKitCommandEngineExplicitOrigin(kitStore) : null), [kitStore]);
}

/** @emoji 📌 Shallow kit rows for every open registry kit; subscribes per {@link KitHostStore} + registry list changes via bridge (canvas portals lack {@link KitRegistryContext}). */
export function useOpenKitShallows(): Kit[] {
  const [tick, setTick] = React.useState(0);
  React.useEffect(() => {
    const bump = () => setTick((t) => t + 1);
    const wireStores = (): (() => void)[] => {
      const reg = getKitRegistryBridge();
      if (!reg) return [];
      return reg
        .list()
        .map((kid) => reg.get(kid))
        .filter((e): e is KitRegistryEntry => e != null)
        .map((e) => e.store.subscribe(bump));
    };
    let storeUnsubs = wireStores();
    const offList = subscribeKitRegistryListChanged(() => {
      storeUnsubs.forEach((u) => u());
      storeUnsubs = wireStores();
      bump();
    });
    return () => {
      offList();
      storeUnsubs.forEach((u) => u());
    };
  }, []);
  return React.useMemo(() => {
    const reg = getKitRegistryBridge();
    if (!reg) return [];
    return reg
      .list()
      .map((kid) => reg.get(kid))
      .filter((e): e is KitRegistryEntry => e != null)
      .map((e) => e.store.getSnapshot().kit);
  }, [tick]);
}

/** @emoji 📌 True when {@link KitRegistryValue} holds the kit id (updates when kits open/close). */
export function useRegistryHasKit(kitId: string): boolean {
  const ids = useOpenKitGuids();
  return ids.includes(kitId);
}

/** @emoji 📌 Persistence kind for a registry kit (undefined if not open); uses bridge for canvas portals without registry context. */
export function useRegistryKitPersistenceKind(kitId: string): KitPersistenceInfo["kind"] | undefined {
  const [tick, setTick] = React.useState(0);
  React.useEffect(() => {
    const bump = () => setTick((t) => t + 1);
    const wireStores = (): (() => void)[] => {
      const reg = getKitRegistryBridge();
      const ent = reg?.get(kitId);
      if (!ent) return [];
      return [ent.store.subscribe(bump)];
    };
    let storeUnsubs = wireStores();
    const offList = subscribeKitRegistryListChanged(() => {
      storeUnsubs.forEach((u) => u());
      storeUnsubs = wireStores();
      bump();
    });
    return () => {
      offList();
      storeUnsubs.forEach((u) => u());
    };
  }, [kitId]);
  return React.useMemo(() => getKitRegistryBridge()?.get(kitId)?.persistence.kind, [kitId, tick]);
}

/** @emoji 📌Alias hooks for explicit "by id" call sites. */
export const useAuthorById = useAuthorTriad;
export const useQualityById = useQualityTriad;
export const useTypeById = useTypeTriad;
export const useConnectionById = useConnectionTriad;
export const usePieceById = usePieceTriad;
export const useDesignById = useDesignTriad;

function useSchemaObjectState(typeName: string, idValue?: string): HookTriad<any> {
  const runtime = useKitRuntimeSafe();
  const scope = React.useContext(SchemaScopeContext);
  if (!runtime) {
    return [undefined, noopAsyncSet, SCHEMA_HOOK_READONLY_STATUS] as const;
  }
  const ref = resolveReference(runtime.state, typeName, idValue, scope);
  const value = ref?.value;
  const canWrite = runtime.canWrite && !!ref;
  const setValue = React.useCallback(
    async (next: SetStateAction<any>) => {
      if (!canWrite) return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
      return await runtime.setObjectValue(typeName, next, idValue, scope);
    },
    [runtime, typeName, idValue, scope, canWrite],
  );
  const status: WriteStatus = canWrite ? SCHEMA_HOOK_IDLE_STATUS : SCHEMA_HOOK_READONLY_STATUS;
  return [value, setValue, status] as const;
}

function useSchemaFieldState(typeName: string, fieldName: string, idValue?: string): HookTriad<any> {
  const runtime = useKitRuntimeSafe();
  const scope = React.useContext(SchemaScopeContext);
  if (!runtime) {
    return [undefined, noopAsyncSet, SCHEMA_HOOK_READONLY_STATUS] as const;
  }
  const value = readSchemaFieldValue(runtime.state, typeName, fieldName, idValue, scope);
  /** DTO index allows writes when the Rust `submitKitChangeCommands` path does not cover this field. */
  const schemaScanWritable = runtime.canWrite && isWritableField(runtime.state, typeName, fieldName, idValue, scope);
  const rustTarget = React.useMemo(() => resolveRustFieldTarget(runtime, typeName, fieldName, idValue, scope), [runtime.kitClient, runtime.snapshot.kit.id, runtime.canWrite, typeName, fieldName, idValue, scope]);
  const [pending, setPending] = React.useState(0);
  const [lastErr, setLastErr] = React.useState<SetError | undefined>(undefined);

  const setValue = React.useCallback(
    async (next: SetStateAction<any>) => {
      const resolved = typeof next === "function" ? (next as (p: any) => any)(value) : next;
      if (rustTarget && runtime.kitClient) {
        if (!runtime.canWrite) {
          return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
        }
        setPending((p) => p + 1);
        setLastErr(undefined);
        let designId: string | null = null;
        if (rustTarget.kind === "Piece" || rustTarget.kind === "ConnectionStore") {
          designId = await resolveDesignIdForPieceOrConnection(runtime.kitClient, rustTarget.kind, rustTarget.id);
        }
        const cmds = buildSchemaEntityChangeCommands(rustTarget.kind, rustTarget.id, rustTarget.field, resolved, designId);
        if (!cmds.length) {
          setPending((p) => p - 1);
          const e: SetError = { kind: "NotSupported", message: `${rustTarget.kind}.${rustTarget.field}` };
          setLastErr(e);
          runtime.pushSetRejection(e);
          return { ok: false, error: e };
        }
        const r = await submitKitChangeCommands(runtime.kitClient, cmds);
        setPending((p) => p - 1);
        if (!r.ok) {
          setLastErr(r.error);
          runtime.pushSetRejection(r.error);
          return r;
        }
        return r;
      }
      if (!schemaScanWritable) {
        return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
      }
      return await runtime.setFieldValue(typeName, fieldName, resolved, idValue, scope);
    },
    [runtime, rustTarget, schemaScanWritable, typeName, fieldName, idValue, scope, value],
  );

  const fieldWriteStatusRef = React.useRef<WriteStatus>(SCHEMA_HOOK_IDLE_STATUS);
  const status = React.useMemo((): WriteStatus => {
    const next: WriteStatus =
      rustTarget && runtime.kitClient
        ? !runtime.canWrite
          ? SCHEMA_HOOK_READONLY_STATUS
          : pending > 0
            ? ({ kind: "pending", pending, lastError: lastErr } as WriteStatus)
            : lastErr
              ? ({ kind: "error", pending: 0, lastError: lastErr } as const)
              : SCHEMA_HOOK_IDLE_STATUS
        : schemaScanWritable
          ? SCHEMA_HOOK_IDLE_STATUS
          : SCHEMA_HOOK_READONLY_STATUS;
    const prev = fieldWriteStatusRef.current;
    if (writeStatusEquivalent(prev, next)) return prev;
    fieldWriteStatusRef.current = next;
    return next;
  }, [rustTarget, runtime.kitClient, runtime.canWrite, pending, lastErr, schemaScanWritable]);

  return [value, setValue, status] as const;
}

// #endregion ⚛️Core Hooks

// #region ⚛️Direct Domain Exports

/** Re-exports of kit entities + WASM bridge; sketchpad UI helpers live in `@semio/sketchpad`. */
export {
  applyKitClientSnapshotToLocalStore,
  asKitInstance,
  Attribute,
  Author,
  Camera,
  Concept,
  Connection,
  Coordinate,
  createFolderKitStore,
  createJsonFileKitStore,
  createKitFileObjectUrl,
  createSessionKitStore,
  Design,
  DiffStatus,
  Folder,
  fetchReadableKitFileBlob,
  getExistingKitFileProvider,
  getKitFileProvider,
  getKitFileStoragePath,
  getKitPorts,
  getOrCreateKitFileState,
  getReadableKitFileUrl,
  getStoredKitFileUrls,
  ICON_WIDTH,
  id,
  InMemoryKitStore,
  isBrowserReadableFileUrl,
  Kit,
  KitFullDtoSchema,
  Piece,
  Plane,
  Point,
  Quality,
  Representation,
  File,
  File as SemioFile,
  Tag,
  TOLERANCE,
  Type,
  Vector,
} from "@semio/js";
export { KitStore } from "@semio/js";
export type { ReadBatch, ReadBatchResult, ReadBatchItem } from "@semio/js";
export type {
  AuthorIdDto,
  ConnectionDiff,
  ConnectionIdDto,
  Connector,
  CoordinatePlain,
  DesignDiff,
  DesignMetadataDto,
  DesignDiffOperationResult,
  DesignOperationResult,
  DesignPlain,
  DesignShallow,
  FlatMerkleCacheEntry,
  Id,
  KitDiff,
  KitFolderAdapter,
  KitJsonFileAdapter,
  MoveVector,
  OperationResult,
  PieceDiff,
  PieceIdDto,
  Port,
  QualityDiff,
  TypeDiff,
  TypeShallow,
  TypeMetadataDto,
} from "@semio/js";
export { normalizeDesignCopyResult, normalizeDesignDiffResult, normalizeDesignFlattenResult } from "@semio/js";
export type { KitCommandContext, KitCommandResult } from "@semio/js";

export function useJSON(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("JSON", idValue);
}

export function useActorKind(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ActorKind", idValue);
}

export function useActor(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Actor", idValue);
}

export function useActorId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Actor", "id", idValue);
}

export function useActorName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Actor", "name", idValue);
}

export function useActorEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Actor", "email", idValue);
}

export function useActorColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Actor", "color", idValue);
}

export function useUser(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("User", idValue);
}

export function useUserHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("User", "hash", idValue);
}

export function useUserId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("User", "id", idValue);
}

export function useUserName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("User", "name", idValue);
}

export function useUserEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("User", "email", idValue);
}

export function useUserColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("User", "color", idValue);
}

export function useAgent(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Agent", idValue);
}

export function useAgentHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Agent", "hash", idValue);
}

export function useAgentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Agent", "id", idValue);
}

export function useAgentLlm(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Agent", "llm", idValue);
}

export function useAgentName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Agent", "name", idValue);
}

export function useAgentEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Agent", "email", idValue);
}

export function useAgentColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Agent", "color", idValue);
}

export function useSessionActorInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionActorInput", idValue);
}

export function useSessionActorInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionActorInput", "id", idValue);
}

export function useSessionActorInputKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionActorInput", "kind", idValue);
}

export function useSessionActorInputLlm(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionActorInput", "llm", idValue);
}

export function useSessionActorInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionActorInput", "name", idValue);
}

export function useSessionActorInputEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionActorInput", "email", idValue);
}

export function useSessionActorInputColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionActorInput", "color", idValue);
}

export function useCoordinate(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Coordinate", idValue);
}

export function useCoordinateHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Coordinate", "hash", idValue);
}

export function useCoordinateU(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Coordinate", "u", idValue);
}

export function useCoordinateV(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Coordinate", "v", idValue);
}

export function useCoordinateInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CoordinateInput", idValue);
}

export function useCoordinateInputU(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CoordinateInput", "u", idValue);
}

export function useCoordinateInputV(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CoordinateInput", "v", idValue);
}

export function usePoint(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Point", idValue);
}

export function usePointHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Point", "hash", idValue);
}

export function usePointX(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Point", "x", idValue);
}

export function usePointY(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Point", "y", idValue);
}

export function usePointZ(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Point", "z", idValue);
}

export function usePointInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PointInput", idValue);
}

export function usePointInputX(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PointInput", "x", idValue);
}

export function usePointInputY(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PointInput", "y", idValue);
}

export function usePointInputZ(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PointInput", "z", idValue);
}

export function useVector(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Vector", idValue);
}

export function useVectorHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Vector", "hash", idValue);
}

export function useVectorX(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Vector", "x", idValue);
}

export function useVectorY(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Vector", "y", idValue);
}

export function useVectorZ(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Vector", "z", idValue);
}

export function useVectorInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("VectorInput", idValue);
}

export function useVectorInputX(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VectorInput", "x", idValue);
}

export function useVectorInputY(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VectorInput", "y", idValue);
}

export function useVectorInputZ(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VectorInput", "z", idValue);
}

export function usePlane(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Plane", idValue);
}

export function usePlaneHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Plane", "hash", idValue);
}

export function usePlaneOrigin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Plane", "origin", idValue);
}

export function usePlaneXAxis(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Plane", "xAxis", idValue);
}

export function usePlaneYAxis(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Plane", "yAxis", idValue);
}

export function usePlaneInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PlaneInput", idValue);
}

export function usePlaneInputOrigin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PlaneInput", "origin", idValue);
}

export function usePlaneInputXAxis(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PlaneInput", "xAxis", idValue);
}

export function usePlaneInputYAxis(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PlaneInput", "yAxis", idValue);
}

export function useCamera(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Camera", idValue);
}

export function useCameraHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Camera", "hash", idValue);
}

export function useCameraPosition(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Camera", "position", idValue);
}

export function useCameraForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Camera", "forward", idValue);
}

export function useCameraUp(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Camera", "up", idValue);
}

export function useCameraInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CameraInput", idValue);
}

export function useCameraInputPosition(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CameraInput", "position", idValue);
}

export function useCameraInputForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CameraInput", "forward", idValue);
}

export function useCameraInputUp(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CameraInput", "up", idValue);
}

export function useAttribute(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Attribute", idValue);
}

export function useAttributeHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Attribute", "hash", idValue);
}

export function useAttributeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Attribute", "id", idValue);
}

export function useAttributeKey(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Attribute", "key", idValue);
}

export function useAttributeValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Attribute", "value", idValue);
}

export function useAttributeDefinition(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Attribute", "definition", idValue);
}

export function useAttributeInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("AttributeInput", idValue);
}

export function useAttributeInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AttributeInput", "id", idValue);
}

export function useAttributeInputKey(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AttributeInput", "key", idValue);
}

export function useAttributeInputValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AttributeInput", "value", idValue);
}

export function useAttributeInputDefinition(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AttributeInput", "definition", idValue);
}

export function useLocation(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Location", idValue);
}

export function useLocationHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Location", "hash", idValue);
}

export function useLocationLongitude(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Location", "longitude", idValue);
}

export function useLocationLatitude(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Location", "latitude", idValue);
}

export function useLocationAltitude(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Location", "altitude", idValue);
}

export function useLocationAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Location", "attributes", idValue);
}

export function useLocationInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("LocationInput", idValue);
}

export function useLocationInputLongitude(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LocationInput", "longitude", idValue);
}

export function useLocationInputLatitude(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LocationInput", "latitude", idValue);
}

export function useLocationInputAltitude(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LocationInput", "altitude", idValue);
}

export function useLocationInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LocationInput", "attributes", idValue);
}

export function useAuthorTriad(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Author", idValue);
}

export function useAuthorHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Author", "hash", idValue);
}

export function useAuthorId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Author", "id", idValue);
}

export function useAuthorName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Author", "name", idValue);
}

export function useAuthorEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Author", "email", idValue);
}

export function useAuthorAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Author", "attributes", idValue);
}

export function useAuthorInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("AuthorInput", idValue);
}

export function useAuthorInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorInput", "id", idValue);
}

export function useAuthorInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorInput", "name", idValue);
}

export function useAuthorInputEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorInput", "email", idValue);
}

export function useAuthorInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorInput", "attributes", idValue);
}

export function useAuthorPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("AuthorPatchInput", idValue);
}

export function useAuthorPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorPatchInput", "name", idValue);
}

export function useAuthorPatchInputEmail(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorPatchInput", "email", idValue);
}

export function useAuthorPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AuthorPatchInput", "attributes", idValue);
}

export function useFolder(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Folder", idValue);
}

export function useFolderHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "hash", idValue);
}

export function useFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "id", idValue);
}

export function useFolderKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "kit", idValue);
}

export function useFolderName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "name", idValue);
}

export function useFolderParent(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "parent", idValue);
}

export function useFolderChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "children", idValue);
}

export function useFolderDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "description", idValue);
}

export function useFolderAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "attributes", idValue);
}

export function useFolderCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "createdAt", idValue);
}

export function useFolderCreatedBy(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "createdBy", idValue);
}

export function useFolderUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "updatedAt", idValue);
}

export function useFolderUpdatedBy(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Folder", "updatedBy", idValue);
}

export function useFolderInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FolderInput", idValue);
}

export function useFolderInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "id", idValue);
}

export function useFolderInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "name", idValue);
}

export function useFolderInputParentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "parentId", idValue);
}

export function useFolderInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "description", idValue);
}

export function useFolderInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "attributes", idValue);
}

export function useFolderInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "createdAt", idValue);
}

export function useFolderInputCreatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "createdById", idValue);
}

export function useFolderInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "updatedAt", idValue);
}

export function useFolderInputUpdatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderInput", "updatedById", idValue);
}

export function useFolderPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FolderPatchInput", idValue);
}

export function useFolderPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "name", idValue);
}

export function useFolderPatchInputParentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "parentId", idValue);
}

export function useFolderPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "description", idValue);
}

export function useFolderPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "attributes", idValue);
}

export function useFolderPatchInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "createdAt", idValue);
}

export function useFolderPatchInputCreatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "createdById", idValue);
}

export function useFolderPatchInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "updatedAt", idValue);
}

export function useFolderPatchInputUpdatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FolderPatchInput", "updatedById", idValue);
}

export function useFile(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("File", idValue);
}

export function useFileHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "hash", idValue);
}

export function useFileId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "id", idValue);
}

export function useFileKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "kit", idValue);
}

export function useFileName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "name", idValue);
}

export function useFileRemote(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "remote", idValue);
}

export function useFileFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "folder", idValue);
}

export function useFileSize(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "size", idValue);
}

export function useFileContentHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "contentHash", idValue);
}

export function useFileBlob(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "blob", idValue);
}

export function useFileMime(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "mime", idValue);
}

export function useFileCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "createdAt", idValue);
}

export function useFileCreatedBy(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "createdBy", idValue);
}

export function useFileUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "updatedAt", idValue);
}

export function useFileUpdatedBy(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("File", "updatedBy", idValue);
}

export function useFileInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FileInput", idValue);
}

export function useFileInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "id", idValue);
}

export function useFileInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "name", idValue);
}

export function useFileInputRemote(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "remote", idValue);
}

export function useFileInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "folderId", idValue);
}

export function useFileInputSize(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "size", idValue);
}

export function useFileInputContentHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "contentHash", idValue);
}

export function useFileInputBlob(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "blob", idValue);
}

export function useFileInputMime(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "mime", idValue);
}

export function useFileInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "createdAt", idValue);
}

export function useFileInputCreatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "createdById", idValue);
}

export function useFileInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "updatedAt", idValue);
}

export function useFileInputUpdatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FileInput", "updatedById", idValue);
}

export function useFilePatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FilePatchInput", idValue);
}

export function useFilePatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "name", idValue);
}

export function useFilePatchInputRemote(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "remote", idValue);
}

export function useFilePatchInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "folderId", idValue);
}

export function useFilePatchInputSize(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "size", idValue);
}

export function useFilePatchInputContentHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "contentHash", idValue);
}

export function useFilePatchInputBlob(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "blob", idValue);
}

export function useFilePatchInputMime(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "mime", idValue);
}

export function useFilePatchInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "createdAt", idValue);
}

export function useFilePatchInputCreatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "createdById", idValue);
}

export function useFilePatchInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "updatedAt", idValue);
}

export function useFilePatchInputUpdatedById(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FilePatchInput", "updatedById", idValue);
}

export function useBenchmark(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Benchmark", idValue);
}

export function useBenchmarkHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "hash", idValue);
}

export function useBenchmarkId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "id", idValue);
}

export function useBenchmarkQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "quality", idValue);
}

export function useBenchmarkName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "name", idValue);
}

export function useBenchmarkIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "icon", idValue);
}

export function useBenchmarkMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "min", idValue);
}

export function useBenchmarkMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "minExcluded", idValue);
}

export function useBenchmarkMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "max", idValue);
}

export function useBenchmarkMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "maxExcluded", idValue);
}

export function useBenchmarkAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Benchmark", "attributes", idValue);
}

export function useBenchmarkInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("BenchmarkInput", idValue);
}

export function useBenchmarkInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "id", idValue);
}

export function useBenchmarkInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "name", idValue);
}

export function useBenchmarkInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "icon", idValue);
}

export function useBenchmarkInputMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "min", idValue);
}

export function useBenchmarkInputMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "minExcluded", idValue);
}

export function useBenchmarkInputMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "max", idValue);
}

export function useBenchmarkInputMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "maxExcluded", idValue);
}

export function useBenchmarkInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BenchmarkInput", "attributes", idValue);
}

export function useQualityTriad(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Quality", idValue);
}

export function useQualityHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "hash", idValue);
}

export function useQualityId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "id", idValue);
}

export function useQualityKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "kit", idValue);
}

export function useQualityKey(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "key", idValue);
}

export function useQualityName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "name", idValue);
}

export function useQualityDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "description", idValue);
}

export function useQualityUri(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "uri", idValue);
}

export function useQualityKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "kind", idValue);
}

export function useQualityFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "folder", idValue);
}

export function useQualityCanScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "canScale", idValue);
}

export function useQualityDefaultSiUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "defaultSiUnit", idValue);
}

export function useQualityDefaultImperialUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "defaultImperialUnit", idValue);
}

export function useQualityMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "min", idValue);
}

export function useQualityIsMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "isMinExcluded", idValue);
}

export function useQualityMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "max", idValue);
}

export function useQualityIsMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "isMaxExcluded", idValue);
}

export function useQualityDefaultValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "defaultValue", idValue);
}

export function useQualityFormula(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "formula", idValue);
}

export function useQualityIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "icon", idValue);
}

export function useQualityImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "image", idValue);
}

export function useQualityUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "unit", idValue);
}

export function useQualityBenchmarks(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "benchmarks", idValue);
}

export function useQualityAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Quality", "attributes", idValue);
}

export function useQualityInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("QualityInput", idValue);
}

export function useQualityInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "id", idValue);
}

export function useQualityInputKey(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "key", idValue);
}

export function useQualityInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "name", idValue);
}

export function useQualityInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "description", idValue);
}

export function useQualityInputUri(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "uri", idValue);
}

export function useQualityInputKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "kind", idValue);
}

export function useQualityInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "folderId", idValue);
}

export function useQualityInputCanScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "canScale", idValue);
}

export function useQualityInputDefaultSiUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "defaultSiUnit", idValue);
}

export function useQualityInputDefaultImperialUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "defaultImperialUnit", idValue);
}

export function useQualityInputMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "min", idValue);
}

export function useQualityInputIsMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "isMinExcluded", idValue);
}

export function useQualityInputMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "max", idValue);
}

export function useQualityInputIsMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "isMaxExcluded", idValue);
}

export function useQualityInputDefaultValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "defaultValue", idValue);
}

export function useQualityInputFormula(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "formula", idValue);
}

export function useQualityInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "icon", idValue);
}

export function useQualityInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "image", idValue);
}

export function useQualityInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "unit", idValue);
}

export function useQualityInputBenchmarks(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "benchmarks", idValue);
}

export function useQualityInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityInput", "attributes", idValue);
}

export function useQualityPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("QualityPatchInput", idValue);
}

export function useQualityPatchInputKey(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "key", idValue);
}

export function useQualityPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "name", idValue);
}

export function useQualityPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "description", idValue);
}

export function useQualityPatchInputUri(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "uri", idValue);
}

export function useQualityPatchInputKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "kind", idValue);
}

export function useQualityPatchInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "folderId", idValue);
}

export function useQualityPatchInputCanScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "canScale", idValue);
}

export function useQualityPatchInputDefaultSiUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "defaultSiUnit", idValue);
}

export function useQualityPatchInputDefaultImperialUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "defaultImperialUnit", idValue);
}

export function useQualityPatchInputMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "min", idValue);
}

export function useQualityPatchInputIsMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "isMinExcluded", idValue);
}

export function useQualityPatchInputMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "max", idValue);
}

export function useQualityPatchInputIsMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "isMaxExcluded", idValue);
}

export function useQualityPatchInputDefaultValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "defaultValue", idValue);
}

export function useQualityPatchInputFormula(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "formula", idValue);
}

export function useQualityPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "icon", idValue);
}

export function useQualityPatchInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "image", idValue);
}

export function useQualityPatchInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "unit", idValue);
}

export function useQualityPatchInputBenchmarks(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "benchmarks", idValue);
}

export function useQualityPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("QualityPatchInput", "attributes", idValue);
}

export function usePort(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Port", idValue);
}

export function usePortHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "hash", idValue);
}

export function usePortId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "id", idValue);
}

export function usePortKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "kit", idValue);
}

export function usePortName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "name", idValue);
}

export function usePortDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "description", idValue);
}

export function usePortIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "icon", idValue);
}

export function usePortMaxChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "maxChildren", idValue);
}

export function usePortCompatiblePorts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "compatiblePorts", idValue);
}

export function usePortAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Port", "attributes", idValue);
}

export function usePortInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PortInput", idValue);
}

export function usePortInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "id", idValue);
}

export function usePortInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "name", idValue);
}

export function usePortInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "description", idValue);
}

export function usePortInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "icon", idValue);
}

export function usePortInputMaxChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "maxChildren", idValue);
}

export function usePortInputCompatiblePortIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "compatiblePortIds", idValue);
}

export function usePortInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortInput", "attributes", idValue);
}

export function usePortPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PortPatchInput", idValue);
}

export function usePortPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortPatchInput", "name", idValue);
}

export function usePortPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortPatchInput", "description", idValue);
}

export function usePortPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortPatchInput", "icon", idValue);
}

export function usePortPatchInputMaxChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortPatchInput", "maxChildren", idValue);
}

export function usePortPatchInputCompatiblePortIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortPatchInput", "compatiblePortIds", idValue);
}

export function usePortPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PortPatchInput", "attributes", idValue);
}

export function useProp(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Prop", idValue);
}

export function usePropHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "hash", idValue);
}

export function usePropId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "id", idValue);
}

export function usePropKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "kit", idValue);
}

export function usePropQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "quality", idValue);
}

export function usePropValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "value", idValue);
}

export function usePropUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "unit", idValue);
}

export function usePropAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Prop", "attributes", idValue);
}

export function usePropInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PropInput", idValue);
}

export function usePropInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PropInput", "id", idValue);
}

export function usePropInputQualityId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PropInput", "qualityId", idValue);
}

export function usePropInputValue(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PropInput", "value", idValue);
}

export function usePropInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PropInput", "unit", idValue);
}

export function usePropInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PropInput", "attributes", idValue);
}

export function useTag(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Tag", idValue);
}

export function useTagHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "hash", idValue);
}

export function useTagId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "id", idValue);
}

export function useTagKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "kit", idValue);
}

export function useTagName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "name", idValue);
}

export function useTagDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "description", idValue);
}

export function useTagIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "icon", idValue);
}

export function useTagAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Tag", "attributes", idValue);
}

export function useTagInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TagInput", idValue);
}

export function useTagInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagInput", "id", idValue);
}

export function useTagInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagInput", "name", idValue);
}

export function useTagInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagInput", "description", idValue);
}

export function useTagInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagInput", "icon", idValue);
}

export function useTagInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagInput", "attributes", idValue);
}

export function useTagPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TagPatchInput", idValue);
}

export function useTagPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagPatchInput", "name", idValue);
}

export function useTagPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagPatchInput", "description", idValue);
}

export function useTagPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagPatchInput", "icon", idValue);
}

export function useTagPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TagPatchInput", "attributes", idValue);
}

export function useConcept(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Concept", idValue);
}

export function useConceptHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "hash", idValue);
}

export function useConceptId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "id", idValue);
}

export function useConceptKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "kit", idValue);
}

export function useConceptName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "name", idValue);
}

export function useConceptDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "description", idValue);
}

export function useConceptIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "icon", idValue);
}

export function useConceptAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Concept", "attributes", idValue);
}

export function useConceptInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConceptInput", idValue);
}

export function useConceptInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptInput", "id", idValue);
}

export function useConceptInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptInput", "name", idValue);
}

export function useConceptInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptInput", "description", idValue);
}

export function useConceptInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptInput", "icon", idValue);
}

export function useConceptInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptInput", "attributes", idValue);
}

export function useConceptPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConceptPatchInput", idValue);
}

export function useConceptPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptPatchInput", "name", idValue);
}

export function useConceptPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptPatchInput", "description", idValue);
}

export function useConceptPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptPatchInput", "icon", idValue);
}

export function useConceptPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConceptPatchInput", "attributes", idValue);
}

export function useFamily(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Family", idValue);
}

export function useFamilyHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "hash", idValue);
}

export function useFamilyId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "id", idValue);
}

export function useFamilyKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "kit", idValue);
}

export function useFamilyName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "name", idValue);
}

export function useFamilyDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "description", idValue);
}

export function useFamilyIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "icon", idValue);
}

export function useFamilyPorts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "ports", idValue);
}

export function useFamilyAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Family", "attributes", idValue);
}

export function useFamilyInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FamilyInput", idValue);
}

export function useFamilyInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyInput", "id", idValue);
}

export function useFamilyInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyInput", "name", idValue);
}

export function useFamilyInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyInput", "description", idValue);
}

export function useFamilyInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyInput", "icon", idValue);
}

export function useFamilyInputPorts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyInput", "ports", idValue);
}

export function useFamilyInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyInput", "attributes", idValue);
}

export function useFamilyPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FamilyPatchInput", idValue);
}

export function useFamilyPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyPatchInput", "name", idValue);
}

export function useFamilyPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyPatchInput", "description", idValue);
}

export function useFamilyPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyPatchInput", "icon", idValue);
}

export function useFamilyPatchInputPorts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyPatchInput", "ports", idValue);
}

export function useFamilyPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FamilyPatchInput", "attributes", idValue);
}

export function useRepresentation(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("RepresentationStore", idValue);
}

export function useRepresentationHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "hash", idValue);
}

export function useRepresentationId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "id", idValue);
}

export function useRepresentationType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "type", idValue);
}

export function useRepresentationName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "name", idValue);
}

export function useRepresentationTags(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "tags", idValue);
}

export function useRepresentationFile(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "file", idValue);
}

export function useRepresentationDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "description", idValue);
}

export function useRepresentationAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationStore", "attributes", idValue);
}

export function useRepresentationInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("RepresentationInput", idValue);
}

export function useRepresentationInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationInput", "id", idValue);
}

export function useRepresentationInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationInput", "name", idValue);
}

export function useRepresentationInputTagIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationInput", "tagIds", idValue);
}

export function useRepresentationInputFileId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationInput", "fileId", idValue);
}

export function useRepresentationInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationInput", "description", idValue);
}

export function useRepresentationInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("RepresentationInput", "attributes", idValue);
}

export function useConnector(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConnectorStore", idValue);
}

export function useConnectorHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "hash", idValue);
}

export function useConnectorId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "id", idValue);
}

export function useConnectorType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "type", idValue);
}

export function useConnectorName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "name", idValue);
}

export function useConnectorT(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "t", idValue);
}

export function useConnectorPoint(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "point", idValue);
}

export function useConnectorDirection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "direction", idValue);
}

export function useConnectorDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "description", idValue);
}

export function useConnectorPort(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "port", idValue);
}

export function useConnectorMandatory(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "mandatory", idValue);
}

export function useConnectorMaxChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "maxChildren", idValue);
}

export function useConnectorProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "props", idValue);
}

export function useConnectorAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "attributes", idValue);
}

export function useConnectorPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "pieces", idValue);
}

export function useConnectorCompatibleConnectors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorStore", "compatibleConnectors", idValue);
}

export function useConnectorInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConnectorInput", idValue);
}

export function useConnectorInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "id", idValue);
}

export function useConnectorInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "name", idValue);
}

export function useConnectorInputT(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "t", idValue);
}

export function useConnectorInputPoint(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "point", idValue);
}

export function useConnectorInputDirection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "direction", idValue);
}

export function useConnectorInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "description", idValue);
}

export function useConnectorInputPortId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "portId", idValue);
}

export function useConnectorInputMandatory(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "mandatory", idValue);
}

export function useConnectorInputMaxChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "maxChildren", idValue);
}

export function useConnectorInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "props", idValue);
}

export function useConnectorInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectorInput", "attributes", idValue);
}

export function useTypeTriad(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Type", idValue);
}

export function useTypeHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "hash", idValue);
}

export function useTypeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "id", idValue);
}

export function useTypeKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "kit", idValue);
}

export function useTypeName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "name", idValue);
}

export function useTypeParent(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "parent", idValue);
}

export function useTypeChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "children", idValue);
}

export function useTypeIsAbstract(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "isAbstract", idValue);
}

export function useTypeFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "folder", idValue);
}

export function useTypeRepresentations(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "representations", idValue);
}

export function useTypeConnectors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "connectors", idValue);
}

export function useTypeProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "props", idValue);
}

export function useTypeStock(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "stock", idValue);
}

export function useTypeVirtual(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "virtual", idValue);
}

export function useTypeUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "unit", idValue);
}

export function useTypeCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "createdAt", idValue);
}

export function useTypeUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "updatedAt", idValue);
}

export function useTypeLocation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "location", idValue);
}

export function useTypeAuthors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "authors", idValue);
}

export function useTypeConcepts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "concepts", idValue);
}

export function useTypeIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "icon", idValue);
}

export function useTypeImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "image", idValue);
}

export function useTypeDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "description", idValue);
}

export function useTypeAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "attributes", idValue);
}

export function useTypeFixedPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Type", "fixedPieces", idValue);
}

export function useTypeInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TypeInput", idValue);
}

export function useTypeInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "id", idValue);
}

export function useTypeInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "name", idValue);
}

export function useTypeInputParentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "parentId", idValue);
}

export function useTypeInputIsAbstract(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "isAbstract", idValue);
}

export function useTypeInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "folderId", idValue);
}

export function useTypeInputRepresentations(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "representations", idValue);
}

export function useTypeInputConnectors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "connectors", idValue);
}

export function useTypeInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "props", idValue);
}

export function useTypeInputStock(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "stock", idValue);
}

export function useTypeInputVirtual(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "virtual", idValue);
}

export function useTypeInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "unit", idValue);
}

export function useTypeInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "createdAt", idValue);
}

export function useTypeInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "updatedAt", idValue);
}

export function useTypeInputLocation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "location", idValue);
}

export function useTypeInputAuthorIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "authorIds", idValue);
}

export function useTypeInputConceptIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "conceptIds", idValue);
}

export function useTypeInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "icon", idValue);
}

export function useTypeInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "image", idValue);
}

export function useTypeInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "description", idValue);
}

export function useTypeInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypeInput", "attributes", idValue);
}

export function useTypePatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TypePatchInput", idValue);
}

export function useTypePatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "name", idValue);
}

export function useTypePatchInputParentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "parentId", idValue);
}

export function useTypePatchInputIsAbstract(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "isAbstract", idValue);
}

export function useTypePatchInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "folderId", idValue);
}

export function useTypePatchInputRepresentations(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "representations", idValue);
}

export function useTypePatchInputConnectors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "connectors", idValue);
}

export function useTypePatchInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "props", idValue);
}

export function useTypePatchInputStock(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "stock", idValue);
}

export function useTypePatchInputVirtual(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "virtual", idValue);
}

export function useTypePatchInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "unit", idValue);
}

export function useTypePatchInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "createdAt", idValue);
}

export function useTypePatchInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "updatedAt", idValue);
}

export function useTypePatchInputLocation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "location", idValue);
}

export function useTypePatchInputAuthorIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "authorIds", idValue);
}

export function useTypePatchInputConceptIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "conceptIds", idValue);
}

export function useTypePatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "icon", idValue);
}

export function useTypePatchInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "image", idValue);
}

export function useTypePatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "description", idValue);
}

export function useTypePatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TypePatchInput", "attributes", idValue);
}

export function useLayer(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Layer", idValue);
}

export function useLayerHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "hash", idValue);
}

export function useLayerId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "id", idValue);
}

export function useLayerDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "design", idValue);
}

export function useLayerPath(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "path", idValue);
}

export function useLayerIsHidden(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "isHidden", idValue);
}

export function useLayerIsLocked(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "isLocked", idValue);
}

export function useLayerColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "color", idValue);
}

export function useLayerDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "description", idValue);
}

export function useLayerAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Layer", "attributes", idValue);
}

export function useLayerInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("LayerInput", idValue);
}

export function useLayerInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "id", idValue);
}

export function useLayerInputPath(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "path", idValue);
}

export function useLayerInputIsHidden(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "isHidden", idValue);
}

export function useLayerInputIsLocked(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "isLocked", idValue);
}

export function useLayerInputColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "color", idValue);
}

export function useLayerInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "description", idValue);
}

export function useLayerInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("LayerInput", "attributes", idValue);
}

export function useSide(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Side", idValue);
}

export function useSideHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Side", "hash", idValue);
}

export function useSideConnection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Side", "connection", idValue);
}

export function useSidePiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Side", "piece", idValue);
}

export function useSideDesignPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Side", "designPiece", idValue);
}

export function useSideConnector(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Side", "connector", idValue);
}

export function useSideInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SideInput", idValue);
}

export function useSideInputPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SideInput", "pieceId", idValue);
}

export function useSideInputDesignPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SideInput", "designPieceId", idValue);
}

export function useSideInputConnectorId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SideInput", "connectorId", idValue);
}

export function useConnectionTriad(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConnectionStore", idValue);
}

export function useConnectionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "hash", idValue);
}

export function useConnectionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "id", idValue);
}

export function useConnectionDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "design", idValue);
}

export function useConnectionConnected(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "connected", idValue);
}

export function useConnectionConnecting(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "connecting", idValue);
}

export function useConnectionGap(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "gap", idValue);
}

export function useConnectionShift(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "shift", idValue);
}

export function useConnectionRise(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "rise", idValue);
}

export function useConnectionRotation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "rotation", idValue);
}

export function useConnectionTurn(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "turn", idValue);
}

export function useConnectionTilt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "tilt", idValue);
}

export function useConnectionU(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "u", idValue);
}

export function useConnectionV(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "v", idValue);
}

export function useConnectionDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "description", idValue);
}

export function useConnectionAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "attributes", idValue);
}

export function useConnectionChildPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "childPiece", idValue);
}

export function useConnectionChildConnector(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "childConnector", idValue);
}

export function useConnectionParentPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "parentPiece", idValue);
}

export function useConnectionParentConnector(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionStore", "parentConnector", idValue);
}

export function useConnectionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConnectionInput", idValue);
}

export function useConnectionInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "id", idValue);
}

export function useConnectionInputConnected(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "connected", idValue);
}

export function useConnectionInputConnecting(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "connecting", idValue);
}

export function useConnectionInputGap(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "gap", idValue);
}

export function useConnectionInputShift(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "shift", idValue);
}

export function useConnectionInputRise(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "rise", idValue);
}

export function useConnectionInputRotation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "rotation", idValue);
}

export function useConnectionInputTurn(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "turn", idValue);
}

export function useConnectionInputTilt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "tilt", idValue);
}

export function useConnectionInputU(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "u", idValue);
}

export function useConnectionInputV(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "v", idValue);
}

export function useConnectionInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "description", idValue);
}

export function useConnectionInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionInput", "attributes", idValue);
}

export function useConnectionPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConnectionPatchInput", idValue);
}

export function useConnectionPatchInputConnected(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "connected", idValue);
}

export function useConnectionPatchInputConnecting(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "connecting", idValue);
}

export function useConnectionPatchInputGap(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "gap", idValue);
}

export function useConnectionPatchInputShift(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "shift", idValue);
}

export function useConnectionPatchInputRise(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "rise", idValue);
}

export function useConnectionPatchInputRotation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "rotation", idValue);
}

export function useConnectionPatchInputTurn(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "turn", idValue);
}

export function useConnectionPatchInputTilt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "tilt", idValue);
}

export function useConnectionPatchInputU(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "u", idValue);
}

export function useConnectionPatchInputV(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "v", idValue);
}

export function useConnectionPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "description", idValue);
}

export function useConnectionPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionPatchInput", "attributes", idValue);
}

export function useStat(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Stat", idValue);
}

export function useStatHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "hash", idValue);
}

export function useStatId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "id", idValue);
}

export function useStatDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "design", idValue);
}

export function useStatQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "quality", idValue);
}

export function useStatUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "unit", idValue);
}

export function useStatMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "min", idValue);
}

export function useStatMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "minExcluded", idValue);
}

export function useStatMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "max", idValue);
}

export function useStatMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Stat", "maxExcluded", idValue);
}

export function useStatInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("StatInput", idValue);
}

export function useStatInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "id", idValue);
}

export function useStatInputQualityId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "qualityId", idValue);
}

export function useStatInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "unit", idValue);
}

export function useStatInputMin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "min", idValue);
}

export function useStatInputMinExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "minExcluded", idValue);
}

export function useStatInputMax(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "max", idValue);
}

export function useStatInputMaxExcluded(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StatInput", "maxExcluded", idValue);
}

export function usePieceKindEnum(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PieceKind", idValue);
}

export function useBlueprint(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Blueprint", idValue);
}

export function useBlueprintType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Blueprint", "type", idValue);
}

export function useBlueprintDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Blueprint", "design", idValue);
}

export function usePieceTriad(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Piece", idValue);
}

export function usePieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "id", idValue);
}

export function usePieceHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "hash", idValue);
}

export function usePieceName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "name", idValue);
}

export function usePiecePlane(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "plane", idValue);
}

export function usePieceCenter(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "center", idValue);
}

export function usePieceScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "scale", idValue);
}

export function usePieceMirrorPlane(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "mirrorPlane", idValue);
}

export function usePieceIsHidden(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "isHidden", idValue);
}

export function usePieceIsLocked(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "isLocked", idValue);
}

export function usePieceColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "color", idValue);
}

export function usePieceDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "description", idValue);
}

export function usePieceKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "kind", idValue);
}

export function usePieceType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "type", idValue);
}

export function usePieceDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "design", idValue);
}

export function usePieceProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "props", idValue);
}

export function usePieceAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "attributes", idValue);
}

export function usePieceParentPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "parentPiece", idValue);
}

export function usePieceChildPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "childPieces", idValue);
}

export function usePieceChildConnections(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "childConnections", idValue);
}

export function usePieceAlternatives(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "alternatives", idValue);
}

export function usePieceAlternativeTypes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "alternativeTypes", idValue);
}

export function usePieceAlternativeDesigns(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Piece", "alternativeDesigns", idValue);
}

export function usePieceInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PieceInput", idValue);
}

export function usePieceInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "id", idValue);
}

export function usePieceInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "name", idValue);
}

export function usePieceInputTypeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "typeId", idValue);
}

export function usePieceInputDesignReferenceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "designReferenceId", idValue);
}

export function usePieceInputPlane(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "plane", idValue);
}

export function usePieceInputCenter(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "center", idValue);
}

export function usePieceInputScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "scale", idValue);
}

export function usePieceInputMirrorPlane(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "mirrorPlane", idValue);
}

export function usePieceInputIsHidden(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "isHidden", idValue);
}

export function usePieceInputIsLocked(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "isLocked", idValue);
}

export function usePieceInputColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "color", idValue);
}

export function usePieceInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "description", idValue);
}

export function usePieceInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "props", idValue);
}

export function usePieceInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceInput", "attributes", idValue);
}

export function usePiecePatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PiecePatchInput", idValue);
}

export function usePiecePatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "name", idValue);
}

export function usePiecePatchInputTypeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "typeId", idValue);
}

export function usePiecePatchInputDesignReferenceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "designReferenceId", idValue);
}

export function usePiecePatchInputPlane(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "plane", idValue);
}

export function usePiecePatchInputCenter(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "center", idValue);
}

export function usePiecePatchInputScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "scale", idValue);
}

export function usePiecePatchInputMirrorPlane(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "mirrorPlane", idValue);
}

export function usePiecePatchInputIsHidden(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "isHidden", idValue);
}

export function usePiecePatchInputIsLocked(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "isLocked", idValue);
}

export function usePiecePatchInputColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "color", idValue);
}

export function usePiecePatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "description", idValue);
}

export function usePiecePatchInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "props", idValue);
}

export function usePiecePatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PiecePatchInput", "attributes", idValue);
}

export function useGroup(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Group", idValue);
}

export function useGroupHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "hash", idValue);
}

export function useGroupId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "id", idValue);
}

export function useGroupDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "design", idValue);
}

export function useGroupPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "pieces", idValue);
}

export function useGroupColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "color", idValue);
}

export function useGroupName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "name", idValue);
}

export function useGroupDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "description", idValue);
}

export function useGroupAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Group", "attributes", idValue);
}

export function useGroupInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("GroupInput", idValue);
}

export function useGroupInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("GroupInput", "id", idValue);
}

export function useGroupInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("GroupInput", "pieceIds", idValue);
}

export function useGroupInputColor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("GroupInput", "color", idValue);
}

export function useGroupInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("GroupInput", "name", idValue);
}

export function useGroupInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("GroupInput", "description", idValue);
}

export function useGroupInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("GroupInput", "attributes", idValue);
}

export function useDesignTriad(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Design", idValue);
}

/** @emoji 📌 Scoped entity read: merges {@link useAuthorScope} id with optional explicit id; selector optional. */
export function useAuthor<T = unknown>(selector?: (entity: any) => T, idValue?: string, _deep?: boolean): T | any | null {
  const resolvedId = useAuthorScope()?.id ?? idValue;
  const [obj] = useAuthorTriad(resolvedId);
  if (obj == null) return null;
  return selector ? selector(obj) : obj;
}

/** @emoji 📌 Scoped entity read for {@link TypeScope}. */
export function useType<T = unknown>(selector?: (entity: any) => T, idValue?: string, _deep?: boolean): T | any | null {
  const resolvedId = useTypeScope()?.id ?? idValue;
  const [obj] = useTypeTriad(resolvedId);
  if (obj == null) return null;
  return selector ? selector(obj) : obj;
}

/** @emoji 📌 Scoped entity read for {@link QualityScope}. */
export function useQuality<T = unknown>(selector?: (entity: any) => T, idValue?: string, _deep?: boolean): T | any | null {
  const resolvedId = useQualityScope()?.id ?? idValue;
  const [obj] = useQualityTriad(resolvedId);
  if (obj == null) return null;
  return selector ? selector(obj) : obj;
}

/** @emoji 📌 Scoped entity read for {@link DesignScope}; middle arg kept for sketchpad call shape (unused). */
export function useDesign<T = unknown>(selector?: (entity: any) => T, _deep?: boolean, idValue?: string): T | any | null {
  const resolvedId = useDesignScope()?.id ?? idValue;
  const [obj] = useDesignTriad(resolvedId);
  if (obj == null) return null;
  return selector ? selector(obj) : obj;
}

/** @emoji 📌 Scoped entity read for {@link PieceScope}. */
export function usePiece<T = unknown>(selector?: (entity: any) => T, idValue?: string, _deep?: boolean): T | any | null {
  const resolvedId = usePieceScope()?.id ?? idValue;
  const [obj] = usePieceTriad(resolvedId);
  if (obj == null) return null;
  return selector ? selector(obj) : obj;
}

/** @emoji 📌 Scoped entity read for {@link ConnectionScope}. */
export function useConnection<T = unknown>(selector?: (entity: any) => T, idValue?: string, _deep?: boolean): T | any | null {
  const resolvedId = useConnectionScope()?.id ?? idValue;
  const [obj] = useConnectionTriad(resolvedId);
  if (obj == null) return null;
  return selector ? selector(obj) : obj;
}

export function useDesignHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "hash", idValue);
}

export function useDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "id", idValue);
}

export function useDesignKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "kit", idValue);
}

export function useDesignName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "name", idValue);
}

export function useDesignParent(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "parent", idValue);
}

export function useDesignChildren(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "children", idValue);
}

export function useDesignIsAbstract(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "isAbstract", idValue);
}

export function useDesignFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "folder", idValue);
}

export function useDesignPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "pieces", idValue);
}

export function useDesignConnections(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "connections", idValue);
}

export function useDesignStats(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "stats", idValue);
}

export function useDesignProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "props", idValue);
}

export function useDesignLayers(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "layers", idValue);
}

export function useDesignActiveLayer(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "activeLayer", idValue);
}

export function useDesignGroups(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "groups", idValue);
}

export function useDesignCanScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "canScale", idValue);
}

export function useDesignCanMirror(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "canMirror", idValue);
}

export function useDesignUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "unit", idValue);
}

export function useDesignLocation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "location", idValue);
}

export function useDesignAuthors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "authors", idValue);
}

export function useDesignConcepts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "concepts", idValue);
}

export function useDesignIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "icon", idValue);
}

export function useDesignImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "image", idValue);
}

export function useDesignDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "description", idValue);
}

export function useDesignAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "attributes", idValue);
}

export function useDesignCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "createdAt", idValue);
}

export function useDesignUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Design", "updatedAt", idValue);
}

export function useDesignInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DesignInput", idValue);
}

export function useDesignInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "id", idValue);
}

export function useDesignInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "name", idValue);
}

export function useDesignInputParentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "parentId", idValue);
}

export function useDesignInputIsAbstract(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "isAbstract", idValue);
}

export function useDesignInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "folderId", idValue);
}

export function useDesignInputPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "pieces", idValue);
}

export function useDesignInputConnections(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "connections", idValue);
}

export function useDesignInputStats(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "stats", idValue);
}

export function useDesignInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "props", idValue);
}

export function useDesignInputLayers(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "layers", idValue);
}

export function useDesignInputActiveLayerId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "activeLayerId", idValue);
}

export function useDesignInputGroups(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "groups", idValue);
}

export function useDesignInputCanScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "canScale", idValue);
}

export function useDesignInputCanMirror(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "canMirror", idValue);
}

export function useDesignInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "unit", idValue);
}

export function useDesignInputLocation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "location", idValue);
}

export function useDesignInputAuthorIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "authorIds", idValue);
}

export function useDesignInputConceptIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "conceptIds", idValue);
}

export function useDesignInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "icon", idValue);
}

export function useDesignInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "image", idValue);
}

export function useDesignInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "description", idValue);
}

export function useDesignInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "attributes", idValue);
}

export function useDesignInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "createdAt", idValue);
}

export function useDesignInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignInput", "updatedAt", idValue);
}

export function useDesignPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DesignPatchInput", idValue);
}

export function useDesignPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "name", idValue);
}

export function useDesignPatchInputParentId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "parentId", idValue);
}

export function useDesignPatchInputIsAbstract(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "isAbstract", idValue);
}

export function useDesignPatchInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "folderId", idValue);
}

export function useDesignPatchInputStats(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "stats", idValue);
}

export function useDesignPatchInputProps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "props", idValue);
}

export function useDesignPatchInputLayers(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "layers", idValue);
}

export function useDesignPatchInputActiveLayerId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "activeLayerId", idValue);
}

export function useDesignPatchInputGroups(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "groups", idValue);
}

export function useDesignPatchInputCanScale(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "canScale", idValue);
}

export function useDesignPatchInputCanMirror(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "canMirror", idValue);
}

export function useDesignPatchInputUnit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "unit", idValue);
}

export function useDesignPatchInputLocation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "location", idValue);
}

export function useDesignPatchInputAuthorIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "authorIds", idValue);
}

export function useDesignPatchInputConceptIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "conceptIds", idValue);
}

export function useDesignPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "icon", idValue);
}

export function useDesignPatchInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "image", idValue);
}

export function useDesignPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "description", idValue);
}

export function useDesignPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "attributes", idValue);
}

export function useDesignPatchInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "createdAt", idValue);
}

export function useDesignPatchInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DesignPatchInput", "updatedAt", idValue);
}

export function useKit(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Kit", idValue);
}

export function useKitHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "hash", idValue);
}

export function useKitId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "id", idValue);
}

export function useKitName(idValue?: string): HookTriad<any> {
  const schemaTriad = useSchemaFieldState("Kit", "name", idValue);
  const schemaStatus = schemaTriad[2];
  const runtime = useKitRuntimeSafe();
  const ks = React.useMemo(() => {
    const c = runtime?.kitClient ?? null;
    return c ? kitStoreFromKitStoreClient(c) : null;
  }, [runtime?.kitClient]);
  const schemaValue = schemaTriad[0];

  const storeSubName = React.useCallback(
    (onChange: () => void) => {
      if (ks) return ks.subscribeKitName(onChange);
      if (runtime?.store) return runtime.store.subscribe(onChange);
      return () => {};
    },
    [ks, runtime?.store],
  );

  /** @emoji 🪪 Snapshot must be referentially stable (reads `kitName$` BehaviorSubject value or local store snapshot) — `runtime.snapshot.kit?.name` is a string so identity is safe. */
  const snapName = React.useCallback(() => {
    if (ks) return ks.getKitNameSnapshot();
    const kn = (runtime?.store?.getSnapshot()?.kit as Kit | undefined)?.name;
    if (kn != null) return String(kn);
    return String(schemaValue ?? "");
  }, [ks, runtime?.store, schemaValue]);

  const liveName = React.useSyncExternalStore(storeSubName, snapName, snapName);

  const storeSubRename = React.useCallback(
    (onChange: () => void) => {
      if (!ks) return () => {};
      return ks.subscribeRenameStatus(onChange);
    },
    [ks],
  );

  /** @emoji 🪪 Stable identity required: returns the BehaviorSubject's current value (cached) or {@link KIT_RENAME_STATUS_IDLE}. */
  const snapRename = React.useCallback(() => {
    if (!ks) return KIT_RENAME_STATUS_IDLE;
    return ks.getRenameStatusSnapshot();
  }, [ks]);

  const renameSnap = React.useSyncExternalStore(storeSubRename, snapRename, snapRename);

  const setter = React.useCallback(
    async (next: React.SetStateAction<any>) => {
      const cur = ks ? liveName : schemaValue;
      const v = typeof next === "function" ? (next as (p: any) => any)(cur) : next;
      if (ks) {
        // 🟢 Each logical edit (focus → enter / blur) is wrapped in its own rs-side transaction:
        //   1. open a fresh transaction on the active draft
        //   2. send the rename op (which records into that transaction)
        //   3. on success commit the transaction (finalize); on failure abort it.
        // This satisfies the architectural rule that "every kit change operation must happen within a draft and a transaction".
        const opened = await ks.openKitWriteTransaction();
        if (!opened.ok) {
          return { ok: false, error: opened.error } as const;
        }
        const r = await ks.rename(String(v));
        if (r.ok) {
          // 🧹 Commit failures are rare and shouldn't mask the rename success — surface but don't override.
          await ks.finalizeKitWriteTransaction().catch(() => undefined);
          return { ok: true } as const;
        }
        await ks.abortKitWriteTransaction().catch(() => undefined);
        return { ok: false, error: r.error! } as const;
      }
      return schemaTriad[1](next);
    },
    [ks, liveName, schemaTriad, schemaValue],
  );

  const renameKind = renameSnap.kind;
  const renameErrorMessage = renameKind === "error" ? renameSnap.message : undefined;
  const kitRenameErrRef = React.useRef<{ msg: string; status: WriteStatus } | null>(null);

  const status: WriteStatus = React.useMemo((): WriteStatus => {
    if (!ks) return schemaStatus;
    if (renameKind === "pending") return USE_KIT_NAME_PENDING_STATUS;
    if (renameKind === "error" && renameErrorMessage !== undefined) {
      const cached = kitRenameErrRef.current;
      if (cached && cached.msg === renameErrorMessage) return cached.status;
      const lastError = { kind: "InvalidValue" as const, message: renameErrorMessage };
      const st = { kind: "error" as const, pending: 0 as const, lastError };
      kitRenameErrRef.current = { msg: renameErrorMessage, status: st };
      return st;
    }
    kitRenameErrRef.current = null;
    return SCHEMA_HOOK_IDLE_STATUS;
  }, [ks, schemaStatus, renameKind, renameErrorMessage]);

  const displayName = ks ? liveName : schemaValue;
  return [displayName, setter, status] as const;
}

export function useKitRelease(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "release", idValue);
}

export function useKitVersion(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "release", idValue);
}

export function useKitTags(explicitKitId?: string): HookTriad<any> {
  return useTagsFull(explicitKitId);
}

export function useKitConcepts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "concepts", idValue);
}

export function useKitFamilies(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "families", idValue);
}

export function useKitPorts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "ports", idValue);
}

export function useKitQualities(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "qualities", idValue);
}

export function useKitFiles(explicitKitId?: string): HookTriad<any> {
  return useFilesFull(explicitKitId);
}

export function useKitFolders(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "folders", idValue);
}

export function useKitAuthors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "authors", idValue);
}

export function useKitRemote(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "remote", idValue);
}

export function useKitHomepage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "homepage", idValue);
}

export function useKitLicense(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "license", idValue);
}

export function useKitPreview(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "preview", idValue);
}

export function useKitIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "icon", idValue);
}

export function useKitImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "image", idValue);
}

export function useKitDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "description", idValue);
}

export function useKitAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "attributes", idValue);
}

export function useKitCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "createdAt", idValue);
}

export function useKitUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Kit", "updatedAt", idValue);
}

export function useKitInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitInput", idValue);
}

export function useKitInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "id", idValue);
}

export function useKitInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "name", idValue);
}

export function useKitInputRelease(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "release", idValue);
}

export function useKitInputTypes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "types", idValue);
}

export function useKitInputDesigns(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "designs", idValue);
}

export function useKitInputTags(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "tags", idValue);
}

export function useKitInputConcepts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "concepts", idValue);
}

export function useKitInputFamilies(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "families", idValue);
}

export function useKitInputPorts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "ports", idValue);
}

export function useKitInputQualities(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "qualities", idValue);
}

export function useKitInputFiles(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "files", idValue);
}

export function useKitInputFolders(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "folders", idValue);
}

export function useKitInputAuthors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "authors", idValue);
}

export function useKitInputRemote(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "remote", idValue);
}

export function useKitInputHomepage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "homepage", idValue);
}

export function useKitInputLicense(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "license", idValue);
}

export function useKitInputPreview(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "preview", idValue);
}

export function useKitInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "icon", idValue);
}

export function useKitInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "image", idValue);
}

export function useKitInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "description", idValue);
}

export function useKitInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "attributes", idValue);
}

export function useKitInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "createdAt", idValue);
}

export function useKitInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInput", "updatedAt", idValue);
}

export function useKitPatchInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitPatchInput", idValue);
}

export function useKitPatchInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "name", idValue);
}

export function useKitPatchInputRelease(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "release", idValue);
}

export function useKitPatchInputRemote(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "remote", idValue);
}

export function useKitPatchInputHomepage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "homepage", idValue);
}

export function useKitPatchInputLicense(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "license", idValue);
}

export function useKitPatchInputPreview(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "preview", idValue);
}

export function useKitPatchInputIcon(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "icon", idValue);
}

export function useKitPatchInputImage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "image", idValue);
}

export function useKitPatchInputDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "description", idValue);
}

export function useKitPatchInputAttributes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "attributes", idValue);
}

export function useKitPatchInputCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "createdAt", idValue);
}

export function useKitPatchInputUpdatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitPatchInput", "updatedAt", idValue);
}

export function useBackboneKind(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("BackboneKind", idValue);
}

export function useKitBackbone(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitBackbone", idValue);
}

export function useKitBackboneHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "hash", idValue);
}

export function useKitBackboneKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "kind", idValue);
}

export function useKitBackboneEndpoint(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "endpoint", idValue);
}

export function useKitBackboneAuthoritative(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "authoritative", idValue);
}

export function useKitBackboneLinearHistory(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "linearHistory", idValue);
}

export function useKitBackboneConnected(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "connected", idValue);
}

export function useKitBackboneTimeoutSeconds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "timeoutSeconds", idValue);
}

export function useKitBackboneCurrentHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "currentHash", idValue);
}

export function useKitBackboneLastInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "lastInteractionIndex", idValue);
}

export function useKitBackbonePendingCandidateCount(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitBackbone", "pendingCandidateCount", idValue);
}

export function useKitClientInfo(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitClientInfo", idValue);
}

export function useKitClientInfoHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfo", "hash", idValue);
}

export function useKitClientInfoId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfo", "id", idValue);
}

export function useKitClientInfoName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfo", "name", idValue);
}

export function useKitClientInfoVersion(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfo", "version", idValue);
}

export function useKitClientInfoPlatform(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfo", "platform", idValue);
}

export function useKitClientInfoInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitClientInfoInput", idValue);
}

export function useKitClientInfoInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfoInput", "id", idValue);
}

export function useKitClientInfoInputName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfoInput", "name", idValue);
}

export function useKitClientInfoInputVersion(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfoInput", "version", idValue);
}

export function useKitClientInfoInputPlatform(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitClientInfoInput", "platform", idValue);
}

export function useSessionState(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionState", idValue);
}

export function useSessionWarningActionKindEnum(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionWarningActionKind", idValue);
}

export function useSessionWarningAction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionWarningAction", idValue);
}

export function useSessionWarningActionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionWarningAction", "hash", idValue);
}

export function useSessionWarningActionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionWarningAction", "kind", idValue);
}

export function useSessionWarningActionLabel(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionWarningAction", "label", idValue);
}

export function useKitSessionWarningEntity(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitSessionWarning", idValue);
}

export function useKitSessionWarningHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionWarning", "hash", idValue);
}

export function useKitSessionWarningCode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionWarning", "code", idValue);
}

export function useKitSessionWarningMessage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionWarning", "message", idValue);
}

export function useKitSessionWarningActions(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionWarning", "actions", idValue);
}

export function useSessionConnectorSelection(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionConnectorSelection", idValue);
}

export function useSessionConnectorSelectionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelection", "hash", idValue);
}

export function useSessionConnectorSelectionPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelection", "piece", idValue);
}

export function useSessionConnectorSelectionDesignPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelection", "designPiece", idValue);
}

export function useSessionConnectorSelectionConnector(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelection", "connector", idValue);
}

export function useSessionConnectorSelectionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionConnectorSelectionInput", idValue);
}

export function useSessionConnectorSelectionInputPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelectionInput", "pieceId", idValue);
}

export function useSessionConnectorSelectionInputDesignPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelectionInput", "designPieceId", idValue);
}

export function useSessionConnectorSelectionInputConnectorId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionConnectorSelectionInput", "connectorId", idValue);
}

export function useKitSessionSelectionEntity(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitSessionSelection", idValue);
}

export function useKitSessionSelectionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "hash", idValue);
}

export function useKitSessionSelectionActiveDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "activeDesign", idValue);
}

export function useKitSessionSelectionPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "pieces", idValue);
}

export function useKitSessionSelectionConnections(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "connections", idValue);
}

export function useKitSessionSelectionConnectors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "connectors", idValue);
}

export function useKitSessionSelectionRepresentations(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "representations", idValue);
}

export function useKitSessionSelectionDesigns(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "designs", idValue);
}

export function useKitSessionSelectionTypes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "types", idValue);
}

export function useKitSessionSelectionReplacementTypeCandidates(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "replacementTypeCandidates", idValue);
}

export function useKitSessionSelectionReplacementDesignCandidates(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "replacementDesignCandidates", idValue);
}

export function useKitSessionSelectionBoundaryConnectorCount(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSessionSelection", "boundaryConnectorCount", idValue);
}

export function useSessionSelectionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SessionSelectionInput", idValue);
}

export function useSessionSelectionInputActiveDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "activeDesignId", idValue);
}

export function useSessionSelectionInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "pieceIds", idValue);
}

export function useSessionSelectionInputConnectionIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "connectionIds", idValue);
}

export function useSessionSelectionInputConnectorSelections(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "connectorSelections", idValue);
}

export function useSessionSelectionInputRepresentationIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "representationIds", idValue);
}

export function useSessionSelectionInputDesignIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "designIds", idValue);
}

export function useSessionSelectionInputTypeIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SessionSelectionInput", "typeIds", idValue);
}

export function useKitSession(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitSession", idValue);
}

export function useKitSessionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "hash", idValue);
}

export function useKitSessionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "id", idValue);
}

export function useKitSessionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "kit", idValue);
}

export function useKitSessionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "actor", idValue);
}

export function useKitSessionClient(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "client", idValue);
}

export function useKitSessionState(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "state", idValue);
}

export function useKitSessionStrictMode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "strictMode", idValue);
}

export function useKitSessionTimeoutSeconds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "timeoutSeconds", idValue);
}

export function useKitSessionStartedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "startedAt", idValue);
}

export function useKitSessionLastSeenAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "lastSeenAt", idValue);
}

export function useKitSessionExpiresAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "expiresAt", idValue);
}

export function useKitSessionDisconnectedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "disconnectedAt", idValue);
}

export function useKitSessionLocked(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "locked", idValue);
}

export function useKitSessionCanReconnect(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "canReconnect", idValue);
}

export function useKitSessionCanSaveLocalChanges(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "canSaveLocalChanges", idValue);
}

export function useKitSessionWarning(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "warning", idValue);
}

export function useKitSessionSelection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "selection", idValue);
}

export function useKitSessionActiveTransactions(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitSession", "activeTransactions", idValue);
}

export function useValidationSeverity(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ValidationSeverity", idValue);
}

export function useValidationNote(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ValidationNote", idValue);
}

export function useValidationNoteHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ValidationNote", "hash", idValue);
}

export function useValidationNoteSeverity(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ValidationNote", "severity", idValue);
}

export function useValidationNoteCode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ValidationNote", "code", idValue);
}

export function useValidationNotePath(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ValidationNote", "path", idValue);
}

export function useValidationNoteEntityId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ValidationNote", "entityId", idValue);
}

export function useValidationNoteMessage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ValidationNote", "message", idValue);
}

export function useKitValidationResult(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitValidationResult", idValue);
}

export function useKitValidationResultHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "hash", idValue);
}

export function useKitValidationResultOk(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "ok", idValue);
}

export function useKitValidationResultImmutable(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "immutable", idValue);
}

export function useKitValidationResultStrict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "strict", idValue);
}

export function useKitValidationResultErrors(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "errors", idValue);
}

export function useKitValidationResultWarnings(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "warnings", idValue);
}

export function useKitValidationResultInfos(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitValidationResult", "infos", idValue);
}

export function useKitConflictStatusEnum(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitConflictStatus", idValue);
}

export function useKitConflictKindEnum(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitConflictKind", idValue);
}

export function useConflictResolutionKind(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConflictResolutionKind", idValue);
}

export function useConflictResolutionOption(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConflictResolutionOption", idValue);
}

export function useConflictResolutionOptionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConflictResolutionOption", "hash", idValue);
}

export function useConflictResolutionOptionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConflictResolutionOption", "id", idValue);
}

export function useConflictResolutionOptionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConflictResolutionOption", "kind", idValue);
}

export function useConflictResolutionOptionLabel(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConflictResolutionOption", "label", idValue);
}

export function useConflictResolutionOptionDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConflictResolutionOption", "description", idValue);
}

export function useConflictResolutionOptionPatchPreview(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConflictResolutionOption", "patchPreview", idValue);
}

export function useKitConflict(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitConflict", idValue);
}

export function useKitConflictHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "hash", idValue);
}

export function useKitConflictId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "id", idValue);
}

export function useKitConflictKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "kit", idValue);
}

export function useKitConflictSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "session", idValue);
}

export function useKitConflictCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "candidate", idValue);
}

export function useKitConflictStatus(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "status", idValue);
}

export function useKitConflictKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "kind", idValue);
}

export function useKitConflictTitle(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "title", idValue);
}

export function useKitConflictMessage(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "message", idValue);
}

export function useKitConflictBlocking(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "blocking", idValue);
}

export function useKitConflictStrict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "strict", idValue);
}

export function useKitConflictNotes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "notes", idValue);
}

export function useKitConflictOptions(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "options", idValue);
}

export function useKitConflictCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "createdAt", idValue);
}

export function useKitConflictResolvedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitConflict", "resolvedAt", idValue);
}

export function useKitCommandKind(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitCommandKind", idValue);
}

export function useKitCommandDescriptor(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitCommandDescriptor", idValue);
}

export function useKitCommandDescriptorHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandDescriptor", "hash", idValue);
}

export function useKitCommandDescriptorKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandDescriptor", "kind", idValue);
}

export function useKitCommandDescriptorMutatesKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandDescriptor", "mutatesKit", idValue);
}

export function useKitCommandDescriptorSessionScoped(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandDescriptor", "sessionScoped", idValue);
}

export function useKitCommandDescriptorRequiresConsensus(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandDescriptor", "requiresConsensus", idValue);
}

export function useKitCommandDescriptorDescription(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandDescriptor", "description", idValue);
}

export function useKitChange(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitChange", idValue);
}

export function useKitChangeHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "hash", idValue);
}

export function useKitChangeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "id", idValue);
}

export function useKitChangeKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "kind", idValue);
}

export function useKitChangeSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "summary", idValue);
}

export function useKitChangeOrigin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "origin", idValue);
}

export function useKitChangeActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "actor", idValue);
}

export function useKitChangeSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "session", idValue);
}

export function useKitChangeTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "transaction", idValue);
}

export function useKitChangeForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "forward", idValue);
}

export function useKitChangeBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "backward", idValue);
}

export function useKitChangeValidation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "validation", idValue);
}

export function useKitChangeCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "createdAt", idValue);
}

export function useKitChangeAppliedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChange", "appliedAt", idValue);
}

export function useKitCandidateStatus(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitCandidateStatus", idValue);
}

export function useCandidateVoteState(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CandidateVoteState", idValue);
}

export function useKitCandidateVote(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitCandidateVote", idValue);
}

export function useKitCandidateVoteHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCandidateVote", "hash", idValue);
}

export function useKitCandidateVoteSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCandidateVote", "session", idValue);
}

export function useKitCandidateVoteState(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCandidateVote", "state", idValue);
}

export function useKitCandidateVoteReason(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCandidateVote", "reason", idValue);
}

export function useKitCandidateVoteRespondedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCandidateVote", "respondedAt", idValue);
}

export function useKitCandidateVoteResolutionOptionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCandidateVote", "resolutionOptionId", idValue);
}

export function useKitChangeCandidate(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitChangeCandidate", idValue);
}

export function useKitChangeCandidateHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "hash", idValue);
}

export function useKitChangeCandidateId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "id", idValue);
}

export function useKitChangeCandidateKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "kit", idValue);
}

export function useKitChangeCandidateKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "kind", idValue);
}

export function useKitChangeCandidateSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "summary", idValue);
}

export function useKitChangeCandidateProposedBy(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "proposedBy", idValue);
}

export function useKitChangeCandidateActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "actor", idValue);
}

export function useKitChangeCandidateTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "transaction", idValue);
}

export function useKitChangeCandidateStatus(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "status", idValue);
}

export function useKitChangeCandidateRequestedFrom(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "requestedFrom", idValue);
}

export function useKitChangeCandidateVotes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "votes", idValue);
}

export function useKitChangeCandidateValidation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "validation", idValue);
}

export function useKitChangeCandidatePreview(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "preview", idValue);
}

export function useKitChangeCandidateProposedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "proposedAt", idValue);
}

export function useKitChangeCandidateExpiresAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "expiresAt", idValue);
}

export function useKitChangeCandidateDecidedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitChangeCandidate", "decidedAt", idValue);
}

export function useTransactionState(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TransactionState", idValue);
}

export function useKitTransaction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitTransaction", idValue);
}

export function useKitTransactionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "hash", idValue);
}

export function useKitTransactionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "id", idValue);
}

export function useKitTransactionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "kit", idValue);
}

export function useKitTransactionLabel(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "label", idValue);
}

export function useKitTransactionState(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "state", idValue);
}

export function useKitTransactionStartedBy(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "startedBy", idValue);
}

export function useKitTransactionParent(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "parent", idValue);
}

export function useKitTransactionStartedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "startedAt", idValue);
}

export function useKitTransactionFinalizedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "finalizedAt", idValue);
}

export function useKitTransactionAbortedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "abortedAt", idValue);
}

export function useKitTransactionChanges(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "changes", idValue);
}

export function useKitTransactionUndoStack(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "undoStack", idValue);
}

export function useKitTransactionRedoStack(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "redoStack", idValue);
}

export function useKitTransactionCanUndo(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "canUndo", idValue);
}

export function useKitTransactionCanRedo(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "canRedo", idValue);
}

export function useKitTransactionSquashedChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitTransaction", "squashedChange", idValue);
}

export function useKitHistoryEntry(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitHistoryEntry", idValue);
}

export function useKitHistoryEntryHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "hash", idValue);
}

export function useKitHistoryEntryId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "id", idValue);
}

export function useKitHistoryEntryIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "index", idValue);
}

export function useKitHistoryEntryTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "transaction", idValue);
}

export function useKitHistoryEntryCommandKinds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "commandKinds", idValue);
}

export function useKitHistoryEntrySummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "summary", idValue);
}

export function useKitHistoryEntrySquashedChangeCount(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "squashedChangeCount", idValue);
}

export function useKitHistoryEntryChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "change", idValue);
}

export function useKitHistoryEntryCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "createdAt", idValue);
}

export function useKitHistoryEntryFinalizedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "finalizedAt", idValue);
}

export function useKitHistoryEntryUndoneAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryEntry", "undoneAt", idValue);
}

export function useKitHistoryPage(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitHistoryPage", idValue);
}

export function useKitHistoryPageHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryPage", "hash", idValue);
}

export function useKitHistoryPageNodes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryPage", "nodes", idValue);
}

export function useKitHistoryPagePageInfo(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryPage", "pageInfo", idValue);
}

export function useKitHistoryPageTotalCount(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistoryPage", "totalCount", idValue);
}

export function useKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitInteraction", idValue);
}

export function useKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "id", idValue);
}

export function useKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "hash", idValue);
}

export function useKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "index", idValue);
}

export function useKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "kit", idValue);
}

export function useKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "kind", idValue);
}

export function useKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "actor", idValue);
}

export function useKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "session", idValue);
}

export function useKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "transaction", idValue);
}

export function useKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "candidate", idValue);
}

export function useKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "change", idValue);
}

export function useKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "conflict", idValue);
}

export function useKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "summary", idValue);
}

export function useKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "metadata", idValue);
}

export function useKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteraction", "createdAt", idValue);
}

export function useChangeKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ChangeKitInteraction", idValue);
}

export function useChangeKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "id", idValue);
}

export function useChangeKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "hash", idValue);
}

export function useChangeKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "index", idValue);
}

export function useChangeKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "kit", idValue);
}

export function useChangeKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "kind", idValue);
}

export function useChangeKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "actor", idValue);
}

export function useChangeKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "session", idValue);
}

export function useChangeKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "transaction", idValue);
}

export function useChangeKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "candidate", idValue);
}

export function useChangeKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "change", idValue);
}

export function useChangeKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "conflict", idValue);
}

export function useChangeKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "summary", idValue);
}

export function useChangeKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "metadata", idValue);
}

export function useChangeKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "createdAt", idValue);
}

export function useChangeKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "forward", idValue);
}

export function useChangeKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangeKitInteraction", "backward", idValue);
}

export function useSetSessionSelectionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SetSessionSelectionKitInteraction", idValue);
}

export function useSetSessionSelectionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "id", idValue);
}

export function useSetSessionSelectionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "hash", idValue);
}

export function useSetSessionSelectionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "index", idValue);
}

export function useSetSessionSelectionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "kit", idValue);
}

export function useSetSessionSelectionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "kind", idValue);
}

export function useSetSessionSelectionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "actor", idValue);
}

export function useSetSessionSelectionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "session", idValue);
}

export function useSetSessionSelectionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "transaction", idValue);
}

export function useSetSessionSelectionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "candidate", idValue);
}

export function useSetSessionSelectionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "change", idValue);
}

export function useSetSessionSelectionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "conflict", idValue);
}

export function useSetSessionSelectionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "summary", idValue);
}

export function useSetSessionSelectionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "metadata", idValue);
}

export function useSetSessionSelectionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "createdAt", idValue);
}

export function useSetSessionSelectionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "forward", idValue);
}

export function useSetSessionSelectionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "backward", idValue);
}

export function useSetSessionSelectionKitInteractionMode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "mode", idValue);
}

export function useSetSessionSelectionKitInteractionSelection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "selection", idValue);
}

export function useSetSessionSelectionKitInteractionPreviousSelection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionKitInteraction", "previousSelection", idValue);
}

export function useCreateAuthorKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateAuthorKitInteraction", idValue);
}

export function useCreateAuthorKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "id", idValue);
}

export function useCreateAuthorKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "hash", idValue);
}

export function useCreateAuthorKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "index", idValue);
}

export function useCreateAuthorKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "kit", idValue);
}

export function useCreateAuthorKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "kind", idValue);
}

export function useCreateAuthorKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "actor", idValue);
}

export function useCreateAuthorKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "session", idValue);
}

export function useCreateAuthorKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "transaction", idValue);
}

export function useCreateAuthorKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "candidate", idValue);
}

export function useCreateAuthorKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "change", idValue);
}

export function useCreateAuthorKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "conflict", idValue);
}

export function useCreateAuthorKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "summary", idValue);
}

export function useCreateAuthorKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "metadata", idValue);
}

export function useCreateAuthorKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "createdAt", idValue);
}

export function useCreateAuthorKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "forward", idValue);
}

export function useCreateAuthorKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "backward", idValue);
}

export function useCreateAuthorKitInteractionAuthor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorKitInteraction", "author", idValue);
}

export function useUpdateAuthorKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateAuthorKitInteraction", idValue);
}

export function useUpdateAuthorKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "id", idValue);
}

export function useUpdateAuthorKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "hash", idValue);
}

export function useUpdateAuthorKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "index", idValue);
}

export function useUpdateAuthorKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "kit", idValue);
}

export function useUpdateAuthorKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "kind", idValue);
}

export function useUpdateAuthorKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "actor", idValue);
}

export function useUpdateAuthorKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "session", idValue);
}

export function useUpdateAuthorKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "transaction", idValue);
}

export function useUpdateAuthorKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "candidate", idValue);
}

export function useUpdateAuthorKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "change", idValue);
}

export function useUpdateAuthorKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "conflict", idValue);
}

export function useUpdateAuthorKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "summary", idValue);
}

export function useUpdateAuthorKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "metadata", idValue);
}

export function useUpdateAuthorKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "createdAt", idValue);
}

export function useUpdateAuthorKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "forward", idValue);
}

export function useUpdateAuthorKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "backward", idValue);
}

export function useUpdateAuthorKitInteractionAuthor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "author", idValue);
}

export function useUpdateAuthorKitInteractionPreviousAuthor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorKitInteraction", "previousAuthor", idValue);
}

export function useDeleteAuthorKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteAuthorKitInteraction", idValue);
}

export function useDeleteAuthorKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "id", idValue);
}

export function useDeleteAuthorKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "hash", idValue);
}

export function useDeleteAuthorKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "index", idValue);
}

export function useDeleteAuthorKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "kit", idValue);
}

export function useDeleteAuthorKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "kind", idValue);
}

export function useDeleteAuthorKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "actor", idValue);
}

export function useDeleteAuthorKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "session", idValue);
}

export function useDeleteAuthorKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "transaction", idValue);
}

export function useDeleteAuthorKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "candidate", idValue);
}

export function useDeleteAuthorKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "change", idValue);
}

export function useDeleteAuthorKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "conflict", idValue);
}

export function useDeleteAuthorKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "summary", idValue);
}

export function useDeleteAuthorKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "metadata", idValue);
}

export function useDeleteAuthorKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "createdAt", idValue);
}

export function useDeleteAuthorKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "forward", idValue);
}

export function useDeleteAuthorKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "backward", idValue);
}

export function useDeleteAuthorKitInteractionPreviousAuthor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorKitInteraction", "previousAuthor", idValue);
}

export function useCreateTypeKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateTypeKitInteraction", idValue);
}

export function useCreateTypeKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "id", idValue);
}

export function useCreateTypeKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "hash", idValue);
}

export function useCreateTypeKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "index", idValue);
}

export function useCreateTypeKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "kit", idValue);
}

export function useCreateTypeKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "kind", idValue);
}

export function useCreateTypeKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "actor", idValue);
}

export function useCreateTypeKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "session", idValue);
}

export function useCreateTypeKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "transaction", idValue);
}

export function useCreateTypeKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "candidate", idValue);
}

export function useCreateTypeKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "change", idValue);
}

export function useCreateTypeKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "conflict", idValue);
}

export function useCreateTypeKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "summary", idValue);
}

export function useCreateTypeKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "metadata", idValue);
}

export function useCreateTypeKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "createdAt", idValue);
}

export function useCreateTypeKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "forward", idValue);
}

export function useCreateTypeKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "backward", idValue);
}

export function useCreateTypeKitInteractionType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeKitInteraction", "type", idValue);
}

export function useUpdateTypeKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateTypeKitInteraction", idValue);
}

export function useUpdateTypeKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "id", idValue);
}

export function useUpdateTypeKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "hash", idValue);
}

export function useUpdateTypeKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "index", idValue);
}

export function useUpdateTypeKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "kit", idValue);
}

export function useUpdateTypeKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "kind", idValue);
}

export function useUpdateTypeKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "actor", idValue);
}

export function useUpdateTypeKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "session", idValue);
}

export function useUpdateTypeKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "transaction", idValue);
}

export function useUpdateTypeKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "candidate", idValue);
}

export function useUpdateTypeKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "change", idValue);
}

export function useUpdateTypeKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "conflict", idValue);
}

export function useUpdateTypeKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "summary", idValue);
}

export function useUpdateTypeKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "metadata", idValue);
}

export function useUpdateTypeKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "createdAt", idValue);
}

export function useUpdateTypeKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "forward", idValue);
}

export function useUpdateTypeKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "backward", idValue);
}

export function useUpdateTypeKitInteractionType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "type", idValue);
}

export function useUpdateTypeKitInteractionPreviousType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeKitInteraction", "previousType", idValue);
}

export function useDeleteTypeKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteTypeKitInteraction", idValue);
}

export function useDeleteTypeKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "id", idValue);
}

export function useDeleteTypeKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "hash", idValue);
}

export function useDeleteTypeKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "index", idValue);
}

export function useDeleteTypeKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "kit", idValue);
}

export function useDeleteTypeKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "kind", idValue);
}

export function useDeleteTypeKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "actor", idValue);
}

export function useDeleteTypeKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "session", idValue);
}

export function useDeleteTypeKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "transaction", idValue);
}

export function useDeleteTypeKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "candidate", idValue);
}

export function useDeleteTypeKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "change", idValue);
}

export function useDeleteTypeKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "conflict", idValue);
}

export function useDeleteTypeKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "summary", idValue);
}

export function useDeleteTypeKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "metadata", idValue);
}

export function useDeleteTypeKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "createdAt", idValue);
}

export function useDeleteTypeKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "forward", idValue);
}

export function useDeleteTypeKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "backward", idValue);
}

export function useDeleteTypeKitInteractionPreviousType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeKitInteraction", "previousType", idValue);
}

export function useCreateDesignKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateDesignKitInteraction", idValue);
}

export function useCreateDesignKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "id", idValue);
}

export function useCreateDesignKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "hash", idValue);
}

export function useCreateDesignKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "index", idValue);
}

export function useCreateDesignKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "kit", idValue);
}

export function useCreateDesignKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "kind", idValue);
}

export function useCreateDesignKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "actor", idValue);
}

export function useCreateDesignKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "session", idValue);
}

export function useCreateDesignKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "transaction", idValue);
}

export function useCreateDesignKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "candidate", idValue);
}

export function useCreateDesignKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "change", idValue);
}

export function useCreateDesignKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "conflict", idValue);
}

export function useCreateDesignKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "summary", idValue);
}

export function useCreateDesignKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "metadata", idValue);
}

export function useCreateDesignKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "createdAt", idValue);
}

export function useCreateDesignKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "forward", idValue);
}

export function useCreateDesignKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "backward", idValue);
}

export function useCreateDesignKitInteractionDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignKitInteraction", "design", idValue);
}

export function useUpdateDesignKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateDesignKitInteraction", idValue);
}

export function useUpdateDesignKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "id", idValue);
}

export function useUpdateDesignKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "hash", idValue);
}

export function useUpdateDesignKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "index", idValue);
}

export function useUpdateDesignKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "kit", idValue);
}

export function useUpdateDesignKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "kind", idValue);
}

export function useUpdateDesignKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "actor", idValue);
}

export function useUpdateDesignKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "session", idValue);
}

export function useUpdateDesignKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "transaction", idValue);
}

export function useUpdateDesignKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "candidate", idValue);
}

export function useUpdateDesignKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "change", idValue);
}

export function useUpdateDesignKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "conflict", idValue);
}

export function useUpdateDesignKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "summary", idValue);
}

export function useUpdateDesignKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "metadata", idValue);
}

export function useUpdateDesignKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "createdAt", idValue);
}

export function useUpdateDesignKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "forward", idValue);
}

export function useUpdateDesignKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "backward", idValue);
}

export function useUpdateDesignKitInteractionDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "design", idValue);
}

export function useUpdateDesignKitInteractionPreviousDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignKitInteraction", "previousDesign", idValue);
}

export function useDeleteDesignKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteDesignKitInteraction", idValue);
}

export function useDeleteDesignKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "id", idValue);
}

export function useDeleteDesignKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "hash", idValue);
}

export function useDeleteDesignKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "index", idValue);
}

export function useDeleteDesignKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "kit", idValue);
}

export function useDeleteDesignKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "kind", idValue);
}

export function useDeleteDesignKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "actor", idValue);
}

export function useDeleteDesignKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "session", idValue);
}

export function useDeleteDesignKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "transaction", idValue);
}

export function useDeleteDesignKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "candidate", idValue);
}

export function useDeleteDesignKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "change", idValue);
}

export function useDeleteDesignKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "conflict", idValue);
}

export function useDeleteDesignKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "summary", idValue);
}

export function useDeleteDesignKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "metadata", idValue);
}

export function useDeleteDesignKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "createdAt", idValue);
}

export function useDeleteDesignKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "forward", idValue);
}

export function useDeleteDesignKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "backward", idValue);
}

export function useDeleteDesignKitInteractionPreviousDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignKitInteraction", "previousDesign", idValue);
}

export function useCreateQualityKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateQualityKitInteraction", idValue);
}

export function useCreateQualityKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "id", idValue);
}

export function useCreateQualityKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "hash", idValue);
}

export function useCreateQualityKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "index", idValue);
}

export function useCreateQualityKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "kit", idValue);
}

export function useCreateQualityKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "kind", idValue);
}

export function useCreateQualityKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "actor", idValue);
}

export function useCreateQualityKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "session", idValue);
}

export function useCreateQualityKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "transaction", idValue);
}

export function useCreateQualityKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "candidate", idValue);
}

export function useCreateQualityKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "change", idValue);
}

export function useCreateQualityKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "conflict", idValue);
}

export function useCreateQualityKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "summary", idValue);
}

export function useCreateQualityKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "metadata", idValue);
}

export function useCreateQualityKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "createdAt", idValue);
}

export function useCreateQualityKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "forward", idValue);
}

export function useCreateQualityKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "backward", idValue);
}

export function useCreateQualityKitInteractionQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityKitInteraction", "quality", idValue);
}

export function useUpdateQualityKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateQualityKitInteraction", idValue);
}

export function useUpdateQualityKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "id", idValue);
}

export function useUpdateQualityKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "hash", idValue);
}

export function useUpdateQualityKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "index", idValue);
}

export function useUpdateQualityKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "kit", idValue);
}

export function useUpdateQualityKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "kind", idValue);
}

export function useUpdateQualityKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "actor", idValue);
}

export function useUpdateQualityKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "session", idValue);
}

export function useUpdateQualityKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "transaction", idValue);
}

export function useUpdateQualityKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "candidate", idValue);
}

export function useUpdateQualityKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "change", idValue);
}

export function useUpdateQualityKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "conflict", idValue);
}

export function useUpdateQualityKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "summary", idValue);
}

export function useUpdateQualityKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "metadata", idValue);
}

export function useUpdateQualityKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "createdAt", idValue);
}

export function useUpdateQualityKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "forward", idValue);
}

export function useUpdateQualityKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "backward", idValue);
}

export function useUpdateQualityKitInteractionQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "quality", idValue);
}

export function useUpdateQualityKitInteractionPreviousQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityKitInteraction", "previousQuality", idValue);
}

export function useDeleteQualityKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteQualityKitInteraction", idValue);
}

export function useDeleteQualityKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "id", idValue);
}

export function useDeleteQualityKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "hash", idValue);
}

export function useDeleteQualityKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "index", idValue);
}

export function useDeleteQualityKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "kit", idValue);
}

export function useDeleteQualityKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "kind", idValue);
}

export function useDeleteQualityKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "actor", idValue);
}

export function useDeleteQualityKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "session", idValue);
}

export function useDeleteQualityKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "transaction", idValue);
}

export function useDeleteQualityKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "candidate", idValue);
}

export function useDeleteQualityKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "change", idValue);
}

export function useDeleteQualityKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "conflict", idValue);
}

export function useDeleteQualityKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "summary", idValue);
}

export function useDeleteQualityKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "metadata", idValue);
}

export function useDeleteQualityKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "createdAt", idValue);
}

export function useDeleteQualityKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "forward", idValue);
}

export function useDeleteQualityKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "backward", idValue);
}

export function useDeleteQualityKitInteractionPreviousQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityKitInteraction", "previousQuality", idValue);
}

export function useCreatePortKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreatePortKitInteraction", idValue);
}

export function useCreatePortKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "id", idValue);
}

export function useCreatePortKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "hash", idValue);
}

export function useCreatePortKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "index", idValue);
}

export function useCreatePortKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "kit", idValue);
}

export function useCreatePortKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "kind", idValue);
}

export function useCreatePortKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "actor", idValue);
}

export function useCreatePortKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "session", idValue);
}

export function useCreatePortKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "transaction", idValue);
}

export function useCreatePortKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "candidate", idValue);
}

export function useCreatePortKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "change", idValue);
}

export function useCreatePortKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "conflict", idValue);
}

export function useCreatePortKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "summary", idValue);
}

export function useCreatePortKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "metadata", idValue);
}

export function useCreatePortKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "createdAt", idValue);
}

export function useCreatePortKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "forward", idValue);
}

export function useCreatePortKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "backward", idValue);
}

export function useCreatePortKitInteractionPort(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortKitInteraction", "port", idValue);
}

export function useUpdatePortKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdatePortKitInteraction", idValue);
}

export function useUpdatePortKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "id", idValue);
}

export function useUpdatePortKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "hash", idValue);
}

export function useUpdatePortKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "index", idValue);
}

export function useUpdatePortKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "kit", idValue);
}

export function useUpdatePortKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "kind", idValue);
}

export function useUpdatePortKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "actor", idValue);
}

export function useUpdatePortKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "session", idValue);
}

export function useUpdatePortKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "transaction", idValue);
}

export function useUpdatePortKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "candidate", idValue);
}

export function useUpdatePortKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "change", idValue);
}

export function useUpdatePortKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "conflict", idValue);
}

export function useUpdatePortKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "summary", idValue);
}

export function useUpdatePortKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "metadata", idValue);
}

export function useUpdatePortKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "createdAt", idValue);
}

export function useUpdatePortKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "forward", idValue);
}

export function useUpdatePortKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "backward", idValue);
}

export function useUpdatePortKitInteractionPort(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "port", idValue);
}

export function useUpdatePortKitInteractionPreviousPort(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortKitInteraction", "previousPort", idValue);
}

export function useDeletePortKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeletePortKitInteraction", idValue);
}

export function useDeletePortKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "id", idValue);
}

export function useDeletePortKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "hash", idValue);
}

export function useDeletePortKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "index", idValue);
}

export function useDeletePortKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "kit", idValue);
}

export function useDeletePortKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "kind", idValue);
}

export function useDeletePortKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "actor", idValue);
}

export function useDeletePortKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "session", idValue);
}

export function useDeletePortKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "transaction", idValue);
}

export function useDeletePortKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "candidate", idValue);
}

export function useDeletePortKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "change", idValue);
}

export function useDeletePortKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "conflict", idValue);
}

export function useDeletePortKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "summary", idValue);
}

export function useDeletePortKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "metadata", idValue);
}

export function useDeletePortKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "createdAt", idValue);
}

export function useDeletePortKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "forward", idValue);
}

export function useDeletePortKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "backward", idValue);
}

export function useDeletePortKitInteractionPreviousPort(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortKitInteraction", "previousPort", idValue);
}

export function useCreateFamilyKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFamilyKitInteraction", idValue);
}

export function useCreateFamilyKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "id", idValue);
}

export function useCreateFamilyKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "hash", idValue);
}

export function useCreateFamilyKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "index", idValue);
}

export function useCreateFamilyKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "kit", idValue);
}

export function useCreateFamilyKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "kind", idValue);
}

export function useCreateFamilyKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "actor", idValue);
}

export function useCreateFamilyKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "session", idValue);
}

export function useCreateFamilyKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "transaction", idValue);
}

export function useCreateFamilyKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "candidate", idValue);
}

export function useCreateFamilyKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "change", idValue);
}

export function useCreateFamilyKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "conflict", idValue);
}

export function useCreateFamilyKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "summary", idValue);
}

export function useCreateFamilyKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "metadata", idValue);
}

export function useCreateFamilyKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "createdAt", idValue);
}

export function useCreateFamilyKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "forward", idValue);
}

export function useCreateFamilyKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "backward", idValue);
}

export function useCreateFamilyKitInteractionFamily(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyKitInteraction", "family", idValue);
}

export function useUpdateFamilyKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateFamilyKitInteraction", idValue);
}

export function useUpdateFamilyKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "id", idValue);
}

export function useUpdateFamilyKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "hash", idValue);
}

export function useUpdateFamilyKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "index", idValue);
}

export function useUpdateFamilyKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "kit", idValue);
}

export function useUpdateFamilyKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "kind", idValue);
}

export function useUpdateFamilyKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "actor", idValue);
}

export function useUpdateFamilyKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "session", idValue);
}

export function useUpdateFamilyKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "transaction", idValue);
}

export function useUpdateFamilyKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "candidate", idValue);
}

export function useUpdateFamilyKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "change", idValue);
}

export function useUpdateFamilyKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "conflict", idValue);
}

export function useUpdateFamilyKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "summary", idValue);
}

export function useUpdateFamilyKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "metadata", idValue);
}

export function useUpdateFamilyKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "createdAt", idValue);
}

export function useUpdateFamilyKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "forward", idValue);
}

export function useUpdateFamilyKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "backward", idValue);
}

export function useUpdateFamilyKitInteractionFamily(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "family", idValue);
}

export function useUpdateFamilyKitInteractionPreviousFamily(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyKitInteraction", "previousFamily", idValue);
}

export function useDeleteFamilyKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteFamilyKitInteraction", idValue);
}

export function useDeleteFamilyKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "id", idValue);
}

export function useDeleteFamilyKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "hash", idValue);
}

export function useDeleteFamilyKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "index", idValue);
}

export function useDeleteFamilyKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "kit", idValue);
}

export function useDeleteFamilyKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "kind", idValue);
}

export function useDeleteFamilyKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "actor", idValue);
}

export function useDeleteFamilyKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "session", idValue);
}

export function useDeleteFamilyKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "transaction", idValue);
}

export function useDeleteFamilyKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "candidate", idValue);
}

export function useDeleteFamilyKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "change", idValue);
}

export function useDeleteFamilyKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "conflict", idValue);
}

export function useDeleteFamilyKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "summary", idValue);
}

export function useDeleteFamilyKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "metadata", idValue);
}

export function useDeleteFamilyKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "createdAt", idValue);
}

export function useDeleteFamilyKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "forward", idValue);
}

export function useDeleteFamilyKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "backward", idValue);
}

export function useDeleteFamilyKitInteractionPreviousFamily(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyKitInteraction", "previousFamily", idValue);
}

export function useCreateTagKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateTagKitInteraction", idValue);
}

export function useCreateTagKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "id", idValue);
}

export function useCreateTagKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "hash", idValue);
}

export function useCreateTagKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "index", idValue);
}

export function useCreateTagKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "kit", idValue);
}

export function useCreateTagKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "kind", idValue);
}

export function useCreateTagKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "actor", idValue);
}

export function useCreateTagKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "session", idValue);
}

export function useCreateTagKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "transaction", idValue);
}

export function useCreateTagKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "candidate", idValue);
}

export function useCreateTagKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "change", idValue);
}

export function useCreateTagKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "conflict", idValue);
}

export function useCreateTagKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "summary", idValue);
}

export function useCreateTagKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "metadata", idValue);
}

export function useCreateTagKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "createdAt", idValue);
}

export function useCreateTagKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "forward", idValue);
}

export function useCreateTagKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "backward", idValue);
}

export function useCreateTagKitInteractionTag(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagKitInteraction", "tag", idValue);
}

export function useUpdateTagKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateTagKitInteraction", idValue);
}

export function useUpdateTagKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "id", idValue);
}

export function useUpdateTagKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "hash", idValue);
}

export function useUpdateTagKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "index", idValue);
}

export function useUpdateTagKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "kit", idValue);
}

export function useUpdateTagKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "kind", idValue);
}

export function useUpdateTagKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "actor", idValue);
}

export function useUpdateTagKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "session", idValue);
}

export function useUpdateTagKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "transaction", idValue);
}

export function useUpdateTagKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "candidate", idValue);
}

export function useUpdateTagKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "change", idValue);
}

export function useUpdateTagKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "conflict", idValue);
}

export function useUpdateTagKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "summary", idValue);
}

export function useUpdateTagKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "metadata", idValue);
}

export function useUpdateTagKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "createdAt", idValue);
}

export function useUpdateTagKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "forward", idValue);
}

export function useUpdateTagKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "backward", idValue);
}

export function useUpdateTagKitInteractionTag(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "tag", idValue);
}

export function useUpdateTagKitInteractionPreviousTag(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagKitInteraction", "previousTag", idValue);
}

export function useDeleteTagKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteTagKitInteraction", idValue);
}

export function useDeleteTagKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "id", idValue);
}

export function useDeleteTagKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "hash", idValue);
}

export function useDeleteTagKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "index", idValue);
}

export function useDeleteTagKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "kit", idValue);
}

export function useDeleteTagKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "kind", idValue);
}

export function useDeleteTagKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "actor", idValue);
}

export function useDeleteTagKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "session", idValue);
}

export function useDeleteTagKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "transaction", idValue);
}

export function useDeleteTagKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "candidate", idValue);
}

export function useDeleteTagKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "change", idValue);
}

export function useDeleteTagKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "conflict", idValue);
}

export function useDeleteTagKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "summary", idValue);
}

export function useDeleteTagKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "metadata", idValue);
}

export function useDeleteTagKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "createdAt", idValue);
}

export function useDeleteTagKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "forward", idValue);
}

export function useDeleteTagKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "backward", idValue);
}

export function useDeleteTagKitInteractionPreviousTag(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagKitInteraction", "previousTag", idValue);
}

export function useCreateConceptKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConceptKitInteraction", idValue);
}

export function useCreateConceptKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "id", idValue);
}

export function useCreateConceptKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "hash", idValue);
}

export function useCreateConceptKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "index", idValue);
}

export function useCreateConceptKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "kit", idValue);
}

export function useCreateConceptKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "kind", idValue);
}

export function useCreateConceptKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "actor", idValue);
}

export function useCreateConceptKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "session", idValue);
}

export function useCreateConceptKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "transaction", idValue);
}

export function useCreateConceptKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "candidate", idValue);
}

export function useCreateConceptKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "change", idValue);
}

export function useCreateConceptKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "conflict", idValue);
}

export function useCreateConceptKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "summary", idValue);
}

export function useCreateConceptKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "metadata", idValue);
}

export function useCreateConceptKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "createdAt", idValue);
}

export function useCreateConceptKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "forward", idValue);
}

export function useCreateConceptKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "backward", idValue);
}

export function useCreateConceptKitInteractionConcept(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptKitInteraction", "concept", idValue);
}

export function useUpdateConceptKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateConceptKitInteraction", idValue);
}

export function useUpdateConceptKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "id", idValue);
}

export function useUpdateConceptKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "hash", idValue);
}

export function useUpdateConceptKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "index", idValue);
}

export function useUpdateConceptKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "kit", idValue);
}

export function useUpdateConceptKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "kind", idValue);
}

export function useUpdateConceptKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "actor", idValue);
}

export function useUpdateConceptKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "session", idValue);
}

export function useUpdateConceptKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "transaction", idValue);
}

export function useUpdateConceptKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "candidate", idValue);
}

export function useUpdateConceptKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "change", idValue);
}

export function useUpdateConceptKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "conflict", idValue);
}

export function useUpdateConceptKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "summary", idValue);
}

export function useUpdateConceptKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "metadata", idValue);
}

export function useUpdateConceptKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "createdAt", idValue);
}

export function useUpdateConceptKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "forward", idValue);
}

export function useUpdateConceptKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "backward", idValue);
}

export function useUpdateConceptKitInteractionConcept(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "concept", idValue);
}

export function useUpdateConceptKitInteractionPreviousConcept(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptKitInteraction", "previousConcept", idValue);
}

export function useDeleteConceptKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteConceptKitInteraction", idValue);
}

export function useDeleteConceptKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "id", idValue);
}

export function useDeleteConceptKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "hash", idValue);
}

export function useDeleteConceptKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "index", idValue);
}

export function useDeleteConceptKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "kit", idValue);
}

export function useDeleteConceptKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "kind", idValue);
}

export function useDeleteConceptKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "actor", idValue);
}

export function useDeleteConceptKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "session", idValue);
}

export function useDeleteConceptKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "transaction", idValue);
}

export function useDeleteConceptKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "candidate", idValue);
}

export function useDeleteConceptKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "change", idValue);
}

export function useDeleteConceptKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "conflict", idValue);
}

export function useDeleteConceptKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "summary", idValue);
}

export function useDeleteConceptKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "metadata", idValue);
}

export function useDeleteConceptKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "createdAt", idValue);
}

export function useDeleteConceptKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "forward", idValue);
}

export function useDeleteConceptKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "backward", idValue);
}

export function useDeleteConceptKitInteractionPreviousConcept(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptKitInteraction", "previousConcept", idValue);
}

export function useCreateFileKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFileKitInteraction", idValue);
}

export function useCreateFileKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "id", idValue);
}

export function useCreateFileKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "hash", idValue);
}

export function useCreateFileKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "index", idValue);
}

export function useCreateFileKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "kit", idValue);
}

export function useCreateFileKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "kind", idValue);
}

export function useCreateFileKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "actor", idValue);
}

export function useCreateFileKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "session", idValue);
}

export function useCreateFileKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "transaction", idValue);
}

export function useCreateFileKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "candidate", idValue);
}

export function useCreateFileKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "change", idValue);
}

export function useCreateFileKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "conflict", idValue);
}

export function useCreateFileKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "summary", idValue);
}

export function useCreateFileKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "metadata", idValue);
}

export function useCreateFileKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "createdAt", idValue);
}

export function useCreateFileKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "forward", idValue);
}

export function useCreateFileKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "backward", idValue);
}

export function useCreateFileKitInteractionFile(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileKitInteraction", "file", idValue);
}

export function useUpdateFileKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateFileKitInteraction", idValue);
}

export function useUpdateFileKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "id", idValue);
}

export function useUpdateFileKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "hash", idValue);
}

export function useUpdateFileKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "index", idValue);
}

export function useUpdateFileKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "kit", idValue);
}

export function useUpdateFileKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "kind", idValue);
}

export function useUpdateFileKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "actor", idValue);
}

export function useUpdateFileKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "session", idValue);
}

export function useUpdateFileKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "transaction", idValue);
}

export function useUpdateFileKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "candidate", idValue);
}

export function useUpdateFileKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "change", idValue);
}

export function useUpdateFileKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "conflict", idValue);
}

export function useUpdateFileKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "summary", idValue);
}

export function useUpdateFileKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "metadata", idValue);
}

export function useUpdateFileKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "createdAt", idValue);
}

export function useUpdateFileKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "forward", idValue);
}

export function useUpdateFileKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "backward", idValue);
}

export function useUpdateFileKitInteractionFile(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "file", idValue);
}

export function useUpdateFileKitInteractionPreviousFile(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileKitInteraction", "previousFile", idValue);
}

export function useDeleteFileKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteFileKitInteraction", idValue);
}

export function useDeleteFileKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "id", idValue);
}

export function useDeleteFileKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "hash", idValue);
}

export function useDeleteFileKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "index", idValue);
}

export function useDeleteFileKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "kit", idValue);
}

export function useDeleteFileKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "kind", idValue);
}

export function useDeleteFileKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "actor", idValue);
}

export function useDeleteFileKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "session", idValue);
}

export function useDeleteFileKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "transaction", idValue);
}

export function useDeleteFileKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "candidate", idValue);
}

export function useDeleteFileKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "change", idValue);
}

export function useDeleteFileKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "conflict", idValue);
}

export function useDeleteFileKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "summary", idValue);
}

export function useDeleteFileKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "metadata", idValue);
}

export function useDeleteFileKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "createdAt", idValue);
}

export function useDeleteFileKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "forward", idValue);
}

export function useDeleteFileKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "backward", idValue);
}

export function useDeleteFileKitInteractionPreviousFile(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileKitInteraction", "previousFile", idValue);
}

export function useCreateFolderKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFolderKitInteraction", idValue);
}

export function useCreateFolderKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "id", idValue);
}

export function useCreateFolderKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "hash", idValue);
}

export function useCreateFolderKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "index", idValue);
}

export function useCreateFolderKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "kit", idValue);
}

export function useCreateFolderKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "kind", idValue);
}

export function useCreateFolderKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "actor", idValue);
}

export function useCreateFolderKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "session", idValue);
}

export function useCreateFolderKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "transaction", idValue);
}

export function useCreateFolderKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "candidate", idValue);
}

export function useCreateFolderKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "change", idValue);
}

export function useCreateFolderKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "conflict", idValue);
}

export function useCreateFolderKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "summary", idValue);
}

export function useCreateFolderKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "metadata", idValue);
}

export function useCreateFolderKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "createdAt", idValue);
}

export function useCreateFolderKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "forward", idValue);
}

export function useCreateFolderKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "backward", idValue);
}

export function useCreateFolderKitInteractionFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderKitInteraction", "folder", idValue);
}

export function useUpdateFolderKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateFolderKitInteraction", idValue);
}

export function useUpdateFolderKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "id", idValue);
}

export function useUpdateFolderKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "hash", idValue);
}

export function useUpdateFolderKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "index", idValue);
}

export function useUpdateFolderKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "kit", idValue);
}

export function useUpdateFolderKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "kind", idValue);
}

export function useUpdateFolderKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "actor", idValue);
}

export function useUpdateFolderKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "session", idValue);
}

export function useUpdateFolderKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "transaction", idValue);
}

export function useUpdateFolderKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "candidate", idValue);
}

export function useUpdateFolderKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "change", idValue);
}

export function useUpdateFolderKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "conflict", idValue);
}

export function useUpdateFolderKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "summary", idValue);
}

export function useUpdateFolderKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "metadata", idValue);
}

export function useUpdateFolderKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "createdAt", idValue);
}

export function useUpdateFolderKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "forward", idValue);
}

export function useUpdateFolderKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "backward", idValue);
}

export function useUpdateFolderKitInteractionFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "folder", idValue);
}

export function useUpdateFolderKitInteractionPreviousFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderKitInteraction", "previousFolder", idValue);
}

export function useDeleteFolderKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteFolderKitInteraction", idValue);
}

export function useDeleteFolderKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "id", idValue);
}

export function useDeleteFolderKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "hash", idValue);
}

export function useDeleteFolderKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "index", idValue);
}

export function useDeleteFolderKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "kit", idValue);
}

export function useDeleteFolderKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "kind", idValue);
}

export function useDeleteFolderKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "actor", idValue);
}

export function useDeleteFolderKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "session", idValue);
}

export function useDeleteFolderKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "transaction", idValue);
}

export function useDeleteFolderKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "candidate", idValue);
}

export function useDeleteFolderKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "change", idValue);
}

export function useDeleteFolderKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "conflict", idValue);
}

export function useDeleteFolderKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "summary", idValue);
}

export function useDeleteFolderKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "metadata", idValue);
}

export function useDeleteFolderKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "createdAt", idValue);
}

export function useDeleteFolderKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "forward", idValue);
}

export function useDeleteFolderKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "backward", idValue);
}

export function useDeleteFolderKitInteractionPreviousFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderKitInteraction", "previousFolder", idValue);
}

export function useMoveArtifactToFolderKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("MoveArtifactToFolderKitInteraction", idValue);
}

export function useMoveArtifactToFolderKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "id", idValue);
}

export function useMoveArtifactToFolderKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "hash", idValue);
}

export function useMoveArtifactToFolderKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "index", idValue);
}

export function useMoveArtifactToFolderKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "kit", idValue);
}

export function useMoveArtifactToFolderKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "kind", idValue);
}

export function useMoveArtifactToFolderKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "actor", idValue);
}

export function useMoveArtifactToFolderKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "session", idValue);
}

export function useMoveArtifactToFolderKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "transaction", idValue);
}

export function useMoveArtifactToFolderKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "candidate", idValue);
}

export function useMoveArtifactToFolderKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "change", idValue);
}

export function useMoveArtifactToFolderKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "conflict", idValue);
}

export function useMoveArtifactToFolderKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "summary", idValue);
}

export function useMoveArtifactToFolderKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "metadata", idValue);
}

export function useMoveArtifactToFolderKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "createdAt", idValue);
}

export function useMoveArtifactToFolderKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "forward", idValue);
}

export function useMoveArtifactToFolderKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "backward", idValue);
}

export function useMoveArtifactToFolderKitInteractionArtifactKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "artifactKind", idValue);
}

export function useMoveArtifactToFolderKitInteractionArtifactId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "artifactId", idValue);
}

export function useMoveArtifactToFolderKitInteractionFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "folder", idValue);
}

export function useMoveArtifactToFolderKitInteractionPreviousFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "previousFolder", idValue);
}

export function useCreatePieceKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreatePieceKitInteraction", idValue);
}

export function useCreatePieceKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "id", idValue);
}

export function useCreatePieceKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "hash", idValue);
}

export function useCreatePieceKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "index", idValue);
}

export function useCreatePieceKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "kit", idValue);
}

export function useCreatePieceKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "kind", idValue);
}

export function useCreatePieceKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "actor", idValue);
}

export function useCreatePieceKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "session", idValue);
}

export function useCreatePieceKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "transaction", idValue);
}

export function useCreatePieceKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "candidate", idValue);
}

export function useCreatePieceKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "change", idValue);
}

export function useCreatePieceKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "conflict", idValue);
}

export function useCreatePieceKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "summary", idValue);
}

export function useCreatePieceKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "metadata", idValue);
}

export function useCreatePieceKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "createdAt", idValue);
}

export function useCreatePieceKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "forward", idValue);
}

export function useCreatePieceKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceKitInteraction", "backward", idValue);
}

export function useCreatePiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreatePiecesKitInteraction", idValue);
}

export function useCreatePiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "id", idValue);
}

export function useCreatePiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "hash", idValue);
}

export function useCreatePiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "index", idValue);
}

export function useCreatePiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "kit", idValue);
}

export function useCreatePiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "kind", idValue);
}

export function useCreatePiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "actor", idValue);
}

export function useCreatePiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "session", idValue);
}

export function useCreatePiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "transaction", idValue);
}

export function useCreatePiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "candidate", idValue);
}

export function useCreatePiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "change", idValue);
}

export function useCreatePiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "conflict", idValue);
}

export function useCreatePiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "summary", idValue);
}

export function useCreatePiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "metadata", idValue);
}

export function useCreatePiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "createdAt", idValue);
}

export function useCreatePiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "forward", idValue);
}

export function useCreatePiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesKitInteraction", "backward", idValue);
}

export function useUpdatePieceKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdatePieceKitInteraction", idValue);
}

export function useUpdatePieceKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "id", idValue);
}

export function useUpdatePieceKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "hash", idValue);
}

export function useUpdatePieceKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "index", idValue);
}

export function useUpdatePieceKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "kit", idValue);
}

export function useUpdatePieceKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "kind", idValue);
}

export function useUpdatePieceKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "actor", idValue);
}

export function useUpdatePieceKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "session", idValue);
}

export function useUpdatePieceKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "transaction", idValue);
}

export function useUpdatePieceKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "candidate", idValue);
}

export function useUpdatePieceKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "change", idValue);
}

export function useUpdatePieceKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "conflict", idValue);
}

export function useUpdatePieceKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "summary", idValue);
}

export function useUpdatePieceKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "metadata", idValue);
}

export function useUpdatePieceKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "createdAt", idValue);
}

export function useUpdatePieceKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "forward", idValue);
}

export function useUpdatePieceKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceKitInteraction", "backward", idValue);
}

export function useUpdatePiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdatePiecesKitInteraction", idValue);
}

export function useUpdatePiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "id", idValue);
}

export function useUpdatePiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "hash", idValue);
}

export function useUpdatePiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "index", idValue);
}

export function useUpdatePiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "kit", idValue);
}

export function useUpdatePiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "kind", idValue);
}

export function useUpdatePiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "actor", idValue);
}

export function useUpdatePiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "session", idValue);
}

export function useUpdatePiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "transaction", idValue);
}

export function useUpdatePiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "candidate", idValue);
}

export function useUpdatePiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "change", idValue);
}

export function useUpdatePiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "conflict", idValue);
}

export function useUpdatePiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "summary", idValue);
}

export function useUpdatePiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "metadata", idValue);
}

export function useUpdatePiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "createdAt", idValue);
}

export function useUpdatePiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "forward", idValue);
}

export function useUpdatePiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesKitInteraction", "backward", idValue);
}

export function useDeletePieceKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeletePieceKitInteraction", idValue);
}

export function useDeletePieceKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "id", idValue);
}

export function useDeletePieceKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "hash", idValue);
}

export function useDeletePieceKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "index", idValue);
}

export function useDeletePieceKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "kit", idValue);
}

export function useDeletePieceKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "kind", idValue);
}

export function useDeletePieceKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "actor", idValue);
}

export function useDeletePieceKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "session", idValue);
}

export function useDeletePieceKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "transaction", idValue);
}

export function useDeletePieceKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "candidate", idValue);
}

export function useDeletePieceKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "change", idValue);
}

export function useDeletePieceKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "conflict", idValue);
}

export function useDeletePieceKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "summary", idValue);
}

export function useDeletePieceKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "metadata", idValue);
}

export function useDeletePieceKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "createdAt", idValue);
}

export function useDeletePieceKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "forward", idValue);
}

export function useDeletePieceKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceKitInteraction", "backward", idValue);
}

export function useDeletePiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeletePiecesKitInteraction", idValue);
}

export function useDeletePiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "id", idValue);
}

export function useDeletePiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "hash", idValue);
}

export function useDeletePiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "index", idValue);
}

export function useDeletePiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "kit", idValue);
}

export function useDeletePiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "kind", idValue);
}

export function useDeletePiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "actor", idValue);
}

export function useDeletePiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "session", idValue);
}

export function useDeletePiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "transaction", idValue);
}

export function useDeletePiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "candidate", idValue);
}

export function useDeletePiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "change", idValue);
}

export function useDeletePiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "conflict", idValue);
}

export function useDeletePiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "summary", idValue);
}

export function useDeletePiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "metadata", idValue);
}

export function useDeletePiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "createdAt", idValue);
}

export function useDeletePiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "forward", idValue);
}

export function useDeletePiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesKitInteraction", "backward", idValue);
}

export function useCreateConnectionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConnectionKitInteraction", idValue);
}

export function useCreateConnectionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "id", idValue);
}

export function useCreateConnectionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "hash", idValue);
}

export function useCreateConnectionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "index", idValue);
}

export function useCreateConnectionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "kit", idValue);
}

export function useCreateConnectionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "kind", idValue);
}

export function useCreateConnectionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "actor", idValue);
}

export function useCreateConnectionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "session", idValue);
}

export function useCreateConnectionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "transaction", idValue);
}

export function useCreateConnectionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "candidate", idValue);
}

export function useCreateConnectionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "change", idValue);
}

export function useCreateConnectionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "conflict", idValue);
}

export function useCreateConnectionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "summary", idValue);
}

export function useCreateConnectionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "metadata", idValue);
}

export function useCreateConnectionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "createdAt", idValue);
}

export function useCreateConnectionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "forward", idValue);
}

export function useCreateConnectionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionKitInteraction", "backward", idValue);
}

export function useCreateConnectionsKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConnectionsKitInteraction", idValue);
}

export function useCreateConnectionsKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "id", idValue);
}

export function useCreateConnectionsKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "hash", idValue);
}

export function useCreateConnectionsKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "index", idValue);
}

export function useCreateConnectionsKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "kit", idValue);
}

export function useCreateConnectionsKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "kind", idValue);
}

export function useCreateConnectionsKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "actor", idValue);
}

export function useCreateConnectionsKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "session", idValue);
}

export function useCreateConnectionsKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "transaction", idValue);
}

export function useCreateConnectionsKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "candidate", idValue);
}

export function useCreateConnectionsKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "change", idValue);
}

export function useCreateConnectionsKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "conflict", idValue);
}

export function useCreateConnectionsKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "summary", idValue);
}

export function useCreateConnectionsKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "metadata", idValue);
}

export function useCreateConnectionsKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "createdAt", idValue);
}

export function useCreateConnectionsKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "forward", idValue);
}

export function useCreateConnectionsKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsKitInteraction", "backward", idValue);
}

export function useUpdateConnectionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateConnectionKitInteraction", idValue);
}

export function useUpdateConnectionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "id", idValue);
}

export function useUpdateConnectionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "hash", idValue);
}

export function useUpdateConnectionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "index", idValue);
}

export function useUpdateConnectionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "kit", idValue);
}

export function useUpdateConnectionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "kind", idValue);
}

export function useUpdateConnectionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "actor", idValue);
}

export function useUpdateConnectionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "session", idValue);
}

export function useUpdateConnectionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "transaction", idValue);
}

export function useUpdateConnectionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "candidate", idValue);
}

export function useUpdateConnectionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "change", idValue);
}

export function useUpdateConnectionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "conflict", idValue);
}

export function useUpdateConnectionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "summary", idValue);
}

export function useUpdateConnectionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "metadata", idValue);
}

export function useUpdateConnectionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "createdAt", idValue);
}

export function useUpdateConnectionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "forward", idValue);
}

export function useUpdateConnectionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionKitInteraction", "backward", idValue);
}

export function useUpdateConnectionsKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateConnectionsKitInteraction", idValue);
}

export function useUpdateConnectionsKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "id", idValue);
}

export function useUpdateConnectionsKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "hash", idValue);
}

export function useUpdateConnectionsKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "index", idValue);
}

export function useUpdateConnectionsKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "kit", idValue);
}

export function useUpdateConnectionsKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "kind", idValue);
}

export function useUpdateConnectionsKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "actor", idValue);
}

export function useUpdateConnectionsKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "session", idValue);
}

export function useUpdateConnectionsKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "transaction", idValue);
}

export function useUpdateConnectionsKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "candidate", idValue);
}

export function useUpdateConnectionsKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "change", idValue);
}

export function useUpdateConnectionsKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "conflict", idValue);
}

export function useUpdateConnectionsKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "summary", idValue);
}

export function useUpdateConnectionsKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "metadata", idValue);
}

export function useUpdateConnectionsKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "createdAt", idValue);
}

export function useUpdateConnectionsKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "forward", idValue);
}

export function useUpdateConnectionsKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsKitInteraction", "backward", idValue);
}

export function useDeleteConnectionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteConnectionKitInteraction", idValue);
}

export function useDeleteConnectionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "id", idValue);
}

export function useDeleteConnectionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "hash", idValue);
}

export function useDeleteConnectionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "index", idValue);
}

export function useDeleteConnectionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "kit", idValue);
}

export function useDeleteConnectionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "kind", idValue);
}

export function useDeleteConnectionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "actor", idValue);
}

export function useDeleteConnectionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "session", idValue);
}

export function useDeleteConnectionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "transaction", idValue);
}

export function useDeleteConnectionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "candidate", idValue);
}

export function useDeleteConnectionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "change", idValue);
}

export function useDeleteConnectionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "conflict", idValue);
}

export function useDeleteConnectionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "summary", idValue);
}

export function useDeleteConnectionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "metadata", idValue);
}

export function useDeleteConnectionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "createdAt", idValue);
}

export function useDeleteConnectionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "forward", idValue);
}

export function useDeleteConnectionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionKitInteraction", "backward", idValue);
}

export function useDeleteConnectionsKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteConnectionsKitInteraction", idValue);
}

export function useDeleteConnectionsKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "id", idValue);
}

export function useDeleteConnectionsKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "hash", idValue);
}

export function useDeleteConnectionsKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "index", idValue);
}

export function useDeleteConnectionsKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "kit", idValue);
}

export function useDeleteConnectionsKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "kind", idValue);
}

export function useDeleteConnectionsKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "actor", idValue);
}

export function useDeleteConnectionsKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "session", idValue);
}

export function useDeleteConnectionsKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "transaction", idValue);
}

export function useDeleteConnectionsKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "candidate", idValue);
}

export function useDeleteConnectionsKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "change", idValue);
}

export function useDeleteConnectionsKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "conflict", idValue);
}

export function useDeleteConnectionsKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "summary", idValue);
}

export function useDeleteConnectionsKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "metadata", idValue);
}

export function useDeleteConnectionsKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "createdAt", idValue);
}

export function useDeleteConnectionsKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "forward", idValue);
}

export function useDeleteConnectionsKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsKitInteraction", "backward", idValue);
}

export function useDeleteSelectionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteSelectionKitInteraction", idValue);
}

export function useDeleteSelectionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "id", idValue);
}

export function useDeleteSelectionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "hash", idValue);
}

export function useDeleteSelectionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "index", idValue);
}

export function useDeleteSelectionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "kit", idValue);
}

export function useDeleteSelectionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "kind", idValue);
}

export function useDeleteSelectionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "actor", idValue);
}

export function useDeleteSelectionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "session", idValue);
}

export function useDeleteSelectionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "transaction", idValue);
}

export function useDeleteSelectionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "candidate", idValue);
}

export function useDeleteSelectionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "change", idValue);
}

export function useDeleteSelectionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "conflict", idValue);
}

export function useDeleteSelectionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "summary", idValue);
}

export function useDeleteSelectionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "metadata", idValue);
}

export function useDeleteSelectionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "createdAt", idValue);
}

export function useDeleteSelectionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "forward", idValue);
}

export function useDeleteSelectionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionKitInteraction", "backward", idValue);
}

export function useFixPiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FixPiecesKitInteraction", idValue);
}

export function useFixPiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "id", idValue);
}

export function useFixPiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "hash", idValue);
}

export function useFixPiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "index", idValue);
}

export function useFixPiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "kit", idValue);
}

export function useFixPiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "kind", idValue);
}

export function useFixPiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "actor", idValue);
}

export function useFixPiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "session", idValue);
}

export function useFixPiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "transaction", idValue);
}

export function useFixPiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "candidate", idValue);
}

export function useFixPiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "change", idValue);
}

export function useFixPiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "conflict", idValue);
}

export function useFixPiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "summary", idValue);
}

export function useFixPiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "metadata", idValue);
}

export function useFixPiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "createdAt", idValue);
}

export function useFixPiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "forward", idValue);
}

export function useFixPiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesKitInteraction", "backward", idValue);
}

export function useClusterPiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ClusterPiecesKitInteraction", idValue);
}

export function useClusterPiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "id", idValue);
}

export function useClusterPiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "hash", idValue);
}

export function useClusterPiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "index", idValue);
}

export function useClusterPiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "kit", idValue);
}

export function useClusterPiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "kind", idValue);
}

export function useClusterPiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "actor", idValue);
}

export function useClusterPiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "session", idValue);
}

export function useClusterPiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "transaction", idValue);
}

export function useClusterPiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "candidate", idValue);
}

export function useClusterPiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "change", idValue);
}

export function useClusterPiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "conflict", idValue);
}

export function useClusterPiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "summary", idValue);
}

export function useClusterPiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "metadata", idValue);
}

export function useClusterPiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "createdAt", idValue);
}

export function useClusterPiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "forward", idValue);
}

export function useClusterPiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesKitInteraction", "backward", idValue);
}

export function useExpandDesignReferenceKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ExpandDesignReferenceKitInteraction", idValue);
}

export function useExpandDesignReferenceKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "id", idValue);
}

export function useExpandDesignReferenceKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "hash", idValue);
}

export function useExpandDesignReferenceKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "index", idValue);
}

export function useExpandDesignReferenceKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "kit", idValue);
}

export function useExpandDesignReferenceKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "kind", idValue);
}

export function useExpandDesignReferenceKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "actor", idValue);
}

export function useExpandDesignReferenceKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "session", idValue);
}

export function useExpandDesignReferenceKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "transaction", idValue);
}

export function useExpandDesignReferenceKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "candidate", idValue);
}

export function useExpandDesignReferenceKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "change", idValue);
}

export function useExpandDesignReferenceKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "conflict", idValue);
}

export function useExpandDesignReferenceKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "summary", idValue);
}

export function useExpandDesignReferenceKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "metadata", idValue);
}

export function useExpandDesignReferenceKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "createdAt", idValue);
}

export function useExpandDesignReferenceKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "forward", idValue);
}

export function useExpandDesignReferenceKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "backward", idValue);
}

export function useFlattenDesignKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FlattenDesignKitInteraction", idValue);
}

export function useFlattenDesignKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "id", idValue);
}

export function useFlattenDesignKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "hash", idValue);
}

export function useFlattenDesignKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "index", idValue);
}

export function useFlattenDesignKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "kit", idValue);
}

export function useFlattenDesignKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "kind", idValue);
}

export function useFlattenDesignKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "actor", idValue);
}

export function useFlattenDesignKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "session", idValue);
}

export function useFlattenDesignKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "transaction", idValue);
}

export function useFlattenDesignKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "candidate", idValue);
}

export function useFlattenDesignKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "change", idValue);
}

export function useFlattenDesignKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "conflict", idValue);
}

export function useFlattenDesignKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "summary", idValue);
}

export function useFlattenDesignKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "metadata", idValue);
}

export function useFlattenDesignKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "createdAt", idValue);
}

export function useFlattenDesignKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "forward", idValue);
}

export function useFlattenDesignKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignKitInteraction", "backward", idValue);
}

export function useDragPiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DragPiecesKitInteraction", idValue);
}

export function useDragPiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "id", idValue);
}

export function useDragPiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "hash", idValue);
}

export function useDragPiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "index", idValue);
}

export function useDragPiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "kit", idValue);
}

export function useDragPiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "kind", idValue);
}

export function useDragPiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "actor", idValue);
}

export function useDragPiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "session", idValue);
}

export function useDragPiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "transaction", idValue);
}

export function useDragPiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "candidate", idValue);
}

export function useDragPiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "change", idValue);
}

export function useDragPiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "conflict", idValue);
}

export function useDragPiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "summary", idValue);
}

export function useDragPiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "metadata", idValue);
}

export function useDragPiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "createdAt", idValue);
}

export function useDragPiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "forward", idValue);
}

export function useDragPiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesKitInteraction", "backward", idValue);
}

export function useMovePiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("MovePiecesKitInteraction", idValue);
}

export function useMovePiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "id", idValue);
}

export function useMovePiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "hash", idValue);
}

export function useMovePiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "index", idValue);
}

export function useMovePiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "kit", idValue);
}

export function useMovePiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "kind", idValue);
}

export function useMovePiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "actor", idValue);
}

export function useMovePiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "session", idValue);
}

export function useMovePiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "transaction", idValue);
}

export function useMovePiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "candidate", idValue);
}

export function useMovePiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "change", idValue);
}

export function useMovePiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "conflict", idValue);
}

export function useMovePiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "summary", idValue);
}

export function useMovePiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "metadata", idValue);
}

export function useMovePiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "createdAt", idValue);
}

export function useMovePiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "forward", idValue);
}

export function useMovePiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesKitInteraction", "backward", idValue);
}

export function useCreateFixedPieceKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFixedPieceKitInteraction", idValue);
}

export function useCreateFixedPieceKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "id", idValue);
}

export function useCreateFixedPieceKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "hash", idValue);
}

export function useCreateFixedPieceKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "index", idValue);
}

export function useCreateFixedPieceKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "kit", idValue);
}

export function useCreateFixedPieceKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "kind", idValue);
}

export function useCreateFixedPieceKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "actor", idValue);
}

export function useCreateFixedPieceKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "session", idValue);
}

export function useCreateFixedPieceKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "transaction", idValue);
}

export function useCreateFixedPieceKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "candidate", idValue);
}

export function useCreateFixedPieceKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "change", idValue);
}

export function useCreateFixedPieceKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "conflict", idValue);
}

export function useCreateFixedPieceKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "summary", idValue);
}

export function useCreateFixedPieceKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "metadata", idValue);
}

export function useCreateFixedPieceKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "createdAt", idValue);
}

export function useCreateFixedPieceKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "forward", idValue);
}

export function useCreateFixedPieceKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceKitInteraction", "backward", idValue);
}

export function useCreateConnectedPieceKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConnectedPieceKitInteraction", idValue);
}

export function useCreateConnectedPieceKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "id", idValue);
}

export function useCreateConnectedPieceKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "hash", idValue);
}

export function useCreateConnectedPieceKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "index", idValue);
}

export function useCreateConnectedPieceKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "kit", idValue);
}

export function useCreateConnectedPieceKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "kind", idValue);
}

export function useCreateConnectedPieceKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "actor", idValue);
}

export function useCreateConnectedPieceKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "session", idValue);
}

export function useCreateConnectedPieceKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "transaction", idValue);
}

export function useCreateConnectedPieceKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "candidate", idValue);
}

export function useCreateConnectedPieceKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "change", idValue);
}

export function useCreateConnectedPieceKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "conflict", idValue);
}

export function useCreateConnectedPieceKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "summary", idValue);
}

export function useCreateConnectedPieceKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "metadata", idValue);
}

export function useCreateConnectedPieceKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "createdAt", idValue);
}

export function useCreateConnectedPieceKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "forward", idValue);
}

export function useCreateConnectedPieceKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceKitInteraction", "backward", idValue);
}

export function useCreateHangingPiecesKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateHangingPiecesKitInteraction", idValue);
}

export function useCreateHangingPiecesKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "id", idValue);
}

export function useCreateHangingPiecesKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "hash", idValue);
}

export function useCreateHangingPiecesKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "index", idValue);
}

export function useCreateHangingPiecesKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "kit", idValue);
}

export function useCreateHangingPiecesKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "kind", idValue);
}

export function useCreateHangingPiecesKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "actor", idValue);
}

export function useCreateHangingPiecesKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "session", idValue);
}

export function useCreateHangingPiecesKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "transaction", idValue);
}

export function useCreateHangingPiecesKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "candidate", idValue);
}

export function useCreateHangingPiecesKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "change", idValue);
}

export function useCreateHangingPiecesKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "conflict", idValue);
}

export function useCreateHangingPiecesKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "summary", idValue);
}

export function useCreateHangingPiecesKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "metadata", idValue);
}

export function useCreateHangingPiecesKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "createdAt", idValue);
}

export function useCreateHangingPiecesKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "forward", idValue);
}

export function useCreateHangingPiecesKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesKitInteraction", "backward", idValue);
}

export function useChangePieceTypeKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ChangePieceTypeKitInteraction", idValue);
}

export function useChangePieceTypeKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "id", idValue);
}

export function useChangePieceTypeKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "hash", idValue);
}

export function useChangePieceTypeKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "index", idValue);
}

export function useChangePieceTypeKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "kit", idValue);
}

export function useChangePieceTypeKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "kind", idValue);
}

export function useChangePieceTypeKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "actor", idValue);
}

export function useChangePieceTypeKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "session", idValue);
}

export function useChangePieceTypeKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "transaction", idValue);
}

export function useChangePieceTypeKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "candidate", idValue);
}

export function useChangePieceTypeKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "change", idValue);
}

export function useChangePieceTypeKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "conflict", idValue);
}

export function useChangePieceTypeKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "summary", idValue);
}

export function useChangePieceTypeKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "metadata", idValue);
}

export function useChangePieceTypeKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "createdAt", idValue);
}

export function useChangePieceTypeKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "forward", idValue);
}

export function useChangePieceTypeKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeKitInteraction", "backward", idValue);
}

export function useChangePiecesTypeKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ChangePiecesTypeKitInteraction", idValue);
}

export function useChangePiecesTypeKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "id", idValue);
}

export function useChangePiecesTypeKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "hash", idValue);
}

export function useChangePiecesTypeKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "index", idValue);
}

export function useChangePiecesTypeKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "kit", idValue);
}

export function useChangePiecesTypeKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "kind", idValue);
}

export function useChangePiecesTypeKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "actor", idValue);
}

export function useChangePiecesTypeKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "session", idValue);
}

export function useChangePiecesTypeKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "transaction", idValue);
}

export function useChangePiecesTypeKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "candidate", idValue);
}

export function useChangePiecesTypeKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "change", idValue);
}

export function useChangePiecesTypeKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "conflict", idValue);
}

export function useChangePiecesTypeKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "summary", idValue);
}

export function useChangePiecesTypeKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "metadata", idValue);
}

export function useChangePiecesTypeKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "createdAt", idValue);
}

export function useChangePiecesTypeKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "forward", idValue);
}

export function useChangePiecesTypeKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeKitInteraction", "backward", idValue);
}

export function usePasteDesignSelectionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PasteDesignSelectionKitInteraction", idValue);
}

export function usePasteDesignSelectionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "id", idValue);
}

export function usePasteDesignSelectionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "hash", idValue);
}

export function usePasteDesignSelectionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "index", idValue);
}

export function usePasteDesignSelectionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "kit", idValue);
}

export function usePasteDesignSelectionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "kind", idValue);
}

export function usePasteDesignSelectionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "actor", idValue);
}

export function usePasteDesignSelectionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "session", idValue);
}

export function usePasteDesignSelectionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "transaction", idValue);
}

export function usePasteDesignSelectionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "candidate", idValue);
}

export function usePasteDesignSelectionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "change", idValue);
}

export function usePasteDesignSelectionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "conflict", idValue);
}

export function usePasteDesignSelectionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "summary", idValue);
}

export function usePasteDesignSelectionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "metadata", idValue);
}

export function usePasteDesignSelectionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "createdAt", idValue);
}

export function usePasteDesignSelectionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "forward", idValue);
}

export function usePasteDesignSelectionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionKitInteraction", "backward", idValue);
}

export function useImportKitKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ImportKitKitInteraction", idValue);
}

export function useImportKitKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "id", idValue);
}

export function useImportKitKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "hash", idValue);
}

export function useImportKitKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "index", idValue);
}

export function useImportKitKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "kit", idValue);
}

export function useImportKitKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "kind", idValue);
}

export function useImportKitKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "actor", idValue);
}

export function useImportKitKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "session", idValue);
}

export function useImportKitKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "transaction", idValue);
}

export function useImportKitKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "candidate", idValue);
}

export function useImportKitKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "change", idValue);
}

export function useImportKitKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "conflict", idValue);
}

export function useImportKitKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "summary", idValue);
}

export function useImportKitKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "metadata", idValue);
}

export function useImportKitKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "createdAt", idValue);
}

export function useImportKitKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "forward", idValue);
}

export function useImportKitKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitKitInteraction", "backward", idValue);
}

export function useResetKitKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ResetKitKitInteraction", idValue);
}

export function useResetKitKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "id", idValue);
}

export function useResetKitKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "hash", idValue);
}

export function useResetKitKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "index", idValue);
}

export function useResetKitKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "kit", idValue);
}

export function useResetKitKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "kind", idValue);
}

export function useResetKitKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "actor", idValue);
}

export function useResetKitKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "session", idValue);
}

export function useResetKitKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "transaction", idValue);
}

export function useResetKitKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "candidate", idValue);
}

export function useResetKitKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "change", idValue);
}

export function useResetKitKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "conflict", idValue);
}

export function useResetKitKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "summary", idValue);
}

export function useResetKitKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "metadata", idValue);
}

export function useResetKitKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "createdAt", idValue);
}

export function useResetKitKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "forward", idValue);
}

export function useResetKitKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitKitInteraction", "backward", idValue);
}

export function useExportKitKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ExportKitKitInteraction", idValue);
}

export function useExportKitKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "id", idValue);
}

export function useExportKitKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "hash", idValue);
}

export function useExportKitKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "index", idValue);
}

export function useExportKitKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "kit", idValue);
}

export function useExportKitKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "kind", idValue);
}

export function useExportKitKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "actor", idValue);
}

export function useExportKitKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "session", idValue);
}

export function useExportKitKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "transaction", idValue);
}

export function useExportKitKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "candidate", idValue);
}

export function useExportKitKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "change", idValue);
}

export function useExportKitKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "conflict", idValue);
}

export function useExportKitKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "summary", idValue);
}

export function useExportKitKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "metadata", idValue);
}

export function useExportKitKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "createdAt", idValue);
}

export function useExportKitKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "forward", idValue);
}

export function useExportKitKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitKitInteraction", "backward", idValue);
}

export function useStartKitSessionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("StartKitSessionKitInteraction", idValue);
}

export function useStartKitSessionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "id", idValue);
}

export function useStartKitSessionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "hash", idValue);
}

export function useStartKitSessionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "index", idValue);
}

export function useStartKitSessionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "kit", idValue);
}

export function useStartKitSessionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "kind", idValue);
}

export function useStartKitSessionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "actor", idValue);
}

export function useStartKitSessionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "session", idValue);
}

export function useStartKitSessionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "transaction", idValue);
}

export function useStartKitSessionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "candidate", idValue);
}

export function useStartKitSessionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "change", idValue);
}

export function useStartKitSessionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "conflict", idValue);
}

export function useStartKitSessionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "summary", idValue);
}

export function useStartKitSessionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "metadata", idValue);
}

export function useStartKitSessionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "createdAt", idValue);
}

export function useStartKitSessionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "forward", idValue);
}

export function useStartKitSessionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionKitInteraction", "backward", idValue);
}

export function useHeartbeatKitSessionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("HeartbeatKitSessionKitInteraction", idValue);
}

export function useHeartbeatKitSessionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "id", idValue);
}

export function useHeartbeatKitSessionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "hash", idValue);
}

export function useHeartbeatKitSessionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "index", idValue);
}

export function useHeartbeatKitSessionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "kit", idValue);
}

export function useHeartbeatKitSessionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "kind", idValue);
}

export function useHeartbeatKitSessionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "actor", idValue);
}

export function useHeartbeatKitSessionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "session", idValue);
}

export function useHeartbeatKitSessionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "transaction", idValue);
}

export function useHeartbeatKitSessionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "candidate", idValue);
}

export function useHeartbeatKitSessionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "change", idValue);
}

export function useHeartbeatKitSessionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "conflict", idValue);
}

export function useHeartbeatKitSessionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "summary", idValue);
}

export function useHeartbeatKitSessionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "metadata", idValue);
}

export function useHeartbeatKitSessionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "createdAt", idValue);
}

export function useHeartbeatKitSessionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "forward", idValue);
}

export function useHeartbeatKitSessionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "backward", idValue);
}

export function useEndKitSessionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("EndKitSessionKitInteraction", idValue);
}

export function useEndKitSessionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "id", idValue);
}

export function useEndKitSessionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "hash", idValue);
}

export function useEndKitSessionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "index", idValue);
}

export function useEndKitSessionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "kit", idValue);
}

export function useEndKitSessionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "kind", idValue);
}

export function useEndKitSessionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "actor", idValue);
}

export function useEndKitSessionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "session", idValue);
}

export function useEndKitSessionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "transaction", idValue);
}

export function useEndKitSessionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "candidate", idValue);
}

export function useEndKitSessionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "change", idValue);
}

export function useEndKitSessionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "conflict", idValue);
}

export function useEndKitSessionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "summary", idValue);
}

export function useEndKitSessionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "metadata", idValue);
}

export function useEndKitSessionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "createdAt", idValue);
}

export function useEndKitSessionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "forward", idValue);
}

export function useEndKitSessionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionKitInteraction", "backward", idValue);
}

export function useReconnectKitSessionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ReconnectKitSessionKitInteraction", idValue);
}

export function useReconnectKitSessionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "id", idValue);
}

export function useReconnectKitSessionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "hash", idValue);
}

export function useReconnectKitSessionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "index", idValue);
}

export function useReconnectKitSessionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "kit", idValue);
}

export function useReconnectKitSessionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "kind", idValue);
}

export function useReconnectKitSessionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "actor", idValue);
}

export function useReconnectKitSessionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "session", idValue);
}

export function useReconnectKitSessionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "transaction", idValue);
}

export function useReconnectKitSessionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "candidate", idValue);
}

export function useReconnectKitSessionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "change", idValue);
}

export function useReconnectKitSessionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "conflict", idValue);
}

export function useReconnectKitSessionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "summary", idValue);
}

export function useReconnectKitSessionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "metadata", idValue);
}

export function useReconnectKitSessionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "createdAt", idValue);
}

export function useReconnectKitSessionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "forward", idValue);
}

export function useReconnectKitSessionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionKitInteraction", "backward", idValue);
}

export function useBeginKitTransactionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("BeginKitTransactionKitInteraction", idValue);
}

export function useBeginKitTransactionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "id", idValue);
}

export function useBeginKitTransactionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "hash", idValue);
}

export function useBeginKitTransactionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "index", idValue);
}

export function useBeginKitTransactionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "kit", idValue);
}

export function useBeginKitTransactionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "kind", idValue);
}

export function useBeginKitTransactionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "actor", idValue);
}

export function useBeginKitTransactionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "session", idValue);
}

export function useBeginKitTransactionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "transaction", idValue);
}

export function useBeginKitTransactionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "candidate", idValue);
}

export function useBeginKitTransactionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "change", idValue);
}

export function useBeginKitTransactionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "conflict", idValue);
}

export function useBeginKitTransactionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "summary", idValue);
}

export function useBeginKitTransactionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "metadata", idValue);
}

export function useBeginKitTransactionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "createdAt", idValue);
}

export function useBeginKitTransactionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "forward", idValue);
}

export function useBeginKitTransactionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionKitInteraction", "backward", idValue);
}

export function useFinalizeKitTransactionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FinalizeKitTransactionKitInteraction", idValue);
}

export function useFinalizeKitTransactionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "id", idValue);
}

export function useFinalizeKitTransactionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "hash", idValue);
}

export function useFinalizeKitTransactionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "index", idValue);
}

export function useFinalizeKitTransactionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "kit", idValue);
}

export function useFinalizeKitTransactionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "kind", idValue);
}

export function useFinalizeKitTransactionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "actor", idValue);
}

export function useFinalizeKitTransactionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "session", idValue);
}

export function useFinalizeKitTransactionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "transaction", idValue);
}

export function useFinalizeKitTransactionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "candidate", idValue);
}

export function useFinalizeKitTransactionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "change", idValue);
}

export function useFinalizeKitTransactionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "conflict", idValue);
}

export function useFinalizeKitTransactionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "summary", idValue);
}

export function useFinalizeKitTransactionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "metadata", idValue);
}

export function useFinalizeKitTransactionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "createdAt", idValue);
}

export function useFinalizeKitTransactionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "forward", idValue);
}

export function useFinalizeKitTransactionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "backward", idValue);
}

export function useAbortKitTransactionKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("AbortKitTransactionKitInteraction", idValue);
}

export function useAbortKitTransactionKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "id", idValue);
}

export function useAbortKitTransactionKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "hash", idValue);
}

export function useAbortKitTransactionKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "index", idValue);
}

export function useAbortKitTransactionKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "kit", idValue);
}

export function useAbortKitTransactionKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "kind", idValue);
}

export function useAbortKitTransactionKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "actor", idValue);
}

export function useAbortKitTransactionKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "session", idValue);
}

export function useAbortKitTransactionKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "transaction", idValue);
}

export function useAbortKitTransactionKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "candidate", idValue);
}

export function useAbortKitTransactionKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "change", idValue);
}

export function useAbortKitTransactionKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "conflict", idValue);
}

export function useAbortKitTransactionKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "summary", idValue);
}

export function useAbortKitTransactionKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "metadata", idValue);
}

export function useAbortKitTransactionKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "createdAt", idValue);
}

export function useAbortKitTransactionKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "forward", idValue);
}

export function useAbortKitTransactionKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionKitInteraction", "backward", idValue);
}

export function useTransactionStepKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TransactionStepKitInteraction", idValue);
}

export function useTransactionStepKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "id", idValue);
}

export function useTransactionStepKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "hash", idValue);
}

export function useTransactionStepKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "index", idValue);
}

export function useTransactionStepKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "kit", idValue);
}

export function useTransactionStepKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "kind", idValue);
}

export function useTransactionStepKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "actor", idValue);
}

export function useTransactionStepKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "session", idValue);
}

export function useTransactionStepKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "transaction", idValue);
}

export function useTransactionStepKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "candidate", idValue);
}

export function useTransactionStepKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "change", idValue);
}

export function useTransactionStepKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "conflict", idValue);
}

export function useTransactionStepKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "summary", idValue);
}

export function useTransactionStepKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "metadata", idValue);
}

export function useTransactionStepKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "createdAt", idValue);
}

export function useTransactionStepKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "forward", idValue);
}

export function useTransactionStepKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepKitInteraction", "backward", idValue);
}

export function useHistoryStepKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("HistoryStepKitInteraction", idValue);
}

export function useHistoryStepKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "id", idValue);
}

export function useHistoryStepKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "hash", idValue);
}

export function useHistoryStepKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "index", idValue);
}

export function useHistoryStepKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "kit", idValue);
}

export function useHistoryStepKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "kind", idValue);
}

export function useHistoryStepKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "actor", idValue);
}

export function useHistoryStepKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "session", idValue);
}

export function useHistoryStepKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "transaction", idValue);
}

export function useHistoryStepKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "candidate", idValue);
}

export function useHistoryStepKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "change", idValue);
}

export function useHistoryStepKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "conflict", idValue);
}

export function useHistoryStepKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "summary", idValue);
}

export function useHistoryStepKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "metadata", idValue);
}

export function useHistoryStepKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "createdAt", idValue);
}

export function useHistoryStepKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "forward", idValue);
}

export function useHistoryStepKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepKitInteraction", "backward", idValue);
}

export function useVoteOnKitChangeCandidateKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("VoteOnKitChangeCandidateKitInteraction", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "id", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "hash", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "index", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "kit", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "kind", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "actor", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "session", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "transaction", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "candidate", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "change", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "conflict", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "summary", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "metadata", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "createdAt", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "forward", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "backward", idValue);
}

export function useResolveKitConflictKitInteraction(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ResolveKitConflictKitInteraction", idValue);
}

export function useResolveKitConflictKitInteractionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "id", idValue);
}

export function useResolveKitConflictKitInteractionHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "hash", idValue);
}

export function useResolveKitConflictKitInteractionIndex(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "index", idValue);
}

export function useResolveKitConflictKitInteractionKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "kit", idValue);
}

export function useResolveKitConflictKitInteractionKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "kind", idValue);
}

export function useResolveKitConflictKitInteractionActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "actor", idValue);
}

export function useResolveKitConflictKitInteractionSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "session", idValue);
}

export function useResolveKitConflictKitInteractionTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "transaction", idValue);
}

export function useResolveKitConflictKitInteractionCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "candidate", idValue);
}

export function useResolveKitConflictKitInteractionChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "change", idValue);
}

export function useResolveKitConflictKitInteractionConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "conflict", idValue);
}

export function useResolveKitConflictKitInteractionSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "summary", idValue);
}

export function useResolveKitConflictKitInteractionMetadata(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "metadata", idValue);
}

export function useResolveKitConflictKitInteractionCreatedAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "createdAt", idValue);
}

export function useResolveKitConflictKitInteractionForward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "forward", idValue);
}

export function useResolveKitConflictKitInteractionBackward(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictKitInteraction", "backward", idValue);
}

export function useKitInteractionPage(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitInteractionPage", idValue);
}

export function useKitInteractionPageHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteractionPage", "hash", idValue);
}

export function useKitInteractionPageNodes(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteractionPage", "nodes", idValue);
}

export function useKitInteractionPagePageInfo(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteractionPage", "pageInfo", idValue);
}

export function useKitInteractionPageTotalCount(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitInteractionPage", "totalCount", idValue);
}

export function useKitHistory(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitHistory", idValue);
}

export function useKitHistoryHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistory", "hash", idValue);
}

export function useKitHistoryCanUndo(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistory", "canUndo", idValue);
}

export function useKitHistoryCanRedo(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistory", "canRedo", idValue);
}

export function useKitHistoryTotalCount(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistory", "totalCount", idValue);
}

export function useKitHistoryHead(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitHistory", "head", idValue);
}

export function useKitStoreEntity(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitStore", idValue);
}

export function useKitStoreHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "hash", idValue);
}

export function useKitStoreKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "kit", idValue);
}

export function useKitStoreBackbone(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "backbone", idValue);
}

export function useKitStoreSessions(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "sessions", idValue);
}

export function useKitStoreTransactions(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "transactions", idValue);
}

export function useKitStorePendingCandidates(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "pendingCandidates", idValue);
}

export function useKitStoreActiveConflicts(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "activeConflicts", idValue);
}

export function useKitStoreValidation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "validation", idValue);
}

export function useKitStoreHistory(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "history", idValue);
}

export function useKitStoreBlockedByConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "blockedByConflict", idValue);
}

export function useKitStoreStrictMode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStore", "strictMode", idValue);
}

export function useArtifactKind(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ArtifactKind", idValue);
}

export function useSelectionMutationMode(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SelectionMutationMode", idValue);
}

export function useKitArchiveExport(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitArchiveExport", idValue);
}

export function useKitArchiveExportHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitArchiveExport", "hash", idValue);
}

export function useKitArchiveExportFileName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitArchiveExport", "fileName", idValue);
}

export function useKitArchiveExportUrl(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitArchiveExport", "url", idValue);
}

export function useKitArchiveExportExpiresAt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitArchiveExport", "expiresAt", idValue);
}

export function useKitMutationResult(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitMutationResult", idValue);
}

export function useKitMutationResultHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "hash", idValue);
}

export function useKitMutationResultAccepted(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "accepted", idValue);
}

export function useKitMutationResultKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "kind", idValue);
}

export function useKitMutationResultSummary(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "summary", idValue);
}

export function useKitMutationResultStore(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "store", idValue);
}

export function useKitMutationResultKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "kit", idValue);
}

export function useKitMutationResultSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "session", idValue);
}

export function useKitMutationResultTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "transaction", idValue);
}

export function useKitMutationResultCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "candidate", idValue);
}

export function useKitMutationResultChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "change", idValue);
}

export function useKitMutationResultHistoryEntry(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "historyEntry", idValue);
}

export function useKitMutationResultConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "conflict", idValue);
}

export function useKitMutationResultValidation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "validation", idValue);
}

export function useKitMutationResultExport(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitMutationResult", "export", idValue);
}

export function useKitCommandContextInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitCommandContextInput", idValue);
}

export function useKitCommandContextInputKitId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandContextInput", "kitId", idValue);
}

export function useKitCommandContextInputSessionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandContextInput", "sessionId", idValue);
}

export function useKitCommandContextInputTransactionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandContextInput", "transactionId", idValue);
}

export function useKitCommandContextInputOrigin(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandContextInput", "origin", idValue);
}

export function useKitCommandContextInputExpectedHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandContextInput", "expectedHash", idValue);
}

export function useKitCommandContextInputStrictMode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitCommandContextInput", "strictMode", idValue);
}

export function useStartKitSessionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("StartKitSessionInput", idValue);
}

export function useStartKitSessionInputKitId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionInput", "kitId", idValue);
}

export function useStartKitSessionInputActor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionInput", "actor", idValue);
}

export function useStartKitSessionInputClient(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionInput", "client", idValue);
}

export function useStartKitSessionInputStrictMode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("StartKitSessionInput", "strictMode", idValue);
}

export function useHeartbeatKitSessionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("HeartbeatKitSessionInput", idValue);
}

export function useHeartbeatKitSessionInputKitId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionInput", "kitId", idValue);
}

export function useHeartbeatKitSessionInputSessionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionInput", "sessionId", idValue);
}

export function useHeartbeatKitSessionInputLastKnownHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HeartbeatKitSessionInput", "lastKnownHash", idValue);
}

export function useEndKitSessionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("EndKitSessionInput", idValue);
}

export function useEndKitSessionInputKitId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionInput", "kitId", idValue);
}

export function useEndKitSessionInputSessionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("EndKitSessionInput", "sessionId", idValue);
}

export function useReconnectKitSessionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ReconnectKitSessionInput", idValue);
}

export function useReconnectKitSessionInputKitId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionInput", "kitId", idValue);
}

export function useReconnectKitSessionInputSessionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionInput", "sessionId", idValue);
}

export function useReconnectKitSessionInputClient(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionInput", "client", idValue);
}

export function useReconnectKitSessionInputLastKnownHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ReconnectKitSessionInput", "lastKnownHash", idValue);
}

export function useSetSessionSelectionCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("SetSessionSelectionCommandInput", idValue);
}

export function useSetSessionSelectionCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionCommandInput", "context", idValue);
}

export function useSetSessionSelectionCommandInputMode(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionCommandInput", "mode", idValue);
}

export function useSetSessionSelectionCommandInputSelection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("SetSessionSelectionCommandInput", "selection", idValue);
}

export function useBeginKitTransactionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("BeginKitTransactionInput", idValue);
}

export function useBeginKitTransactionInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionInput", "context", idValue);
}

export function useBeginKitTransactionInputLabel(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionInput", "label", idValue);
}

export function useBeginKitTransactionInputParentTransactionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("BeginKitTransactionInput", "parentTransactionId", idValue);
}

export function useFinalizeKitTransactionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FinalizeKitTransactionInput", idValue);
}

export function useFinalizeKitTransactionInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionInput", "context", idValue);
}

export function useFinalizeKitTransactionInputTransactionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FinalizeKitTransactionInput", "transactionId", idValue);
}

export function useAbortKitTransactionInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("AbortKitTransactionInput", idValue);
}

export function useAbortKitTransactionInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionInput", "context", idValue);
}

export function useAbortKitTransactionInputTransactionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("AbortKitTransactionInput", "transactionId", idValue);
}

export function useTransactionStepInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("TransactionStepInput", idValue);
}

export function useTransactionStepInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepInput", "context", idValue);
}

export function useTransactionStepInputTransactionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("TransactionStepInput", "transactionId", idValue);
}

export function useHistoryStepInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("HistoryStepInput", idValue);
}

export function useHistoryStepInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepInput", "context", idValue);
}

export function useHistoryStepInputSteps(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("HistoryStepInput", "steps", idValue);
}

export function useVoteOnKitChangeCandidateInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("VoteOnKitChangeCandidateInput", idValue);
}

export function useVoteOnKitChangeCandidateInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateInput", "context", idValue);
}

export function useVoteOnKitChangeCandidateInputCandidateId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateInput", "candidateId", idValue);
}

export function useVoteOnKitChangeCandidateInputState(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateInput", "state", idValue);
}

export function useVoteOnKitChangeCandidateInputReason(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateInput", "reason", idValue);
}

export function useVoteOnKitChangeCandidateInputResolutionOptionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("VoteOnKitChangeCandidateInput", "resolutionOptionId", idValue);
}

export function useResolveKitConflictInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ResolveKitConflictInput", idValue);
}

export function useResolveKitConflictInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictInput", "context", idValue);
}

export function useResolveKitConflictInputConflictId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictInput", "conflictId", idValue);
}

export function useResolveKitConflictInputOptionId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictInput", "optionId", idValue);
}

export function useResolveKitConflictInputPayload(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResolveKitConflictInput", "payload", idValue);
}

export function useCreateAuthorCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateAuthorCommandInput", idValue);
}

export function useCreateAuthorCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorCommandInput", "context", idValue);
}

export function useCreateAuthorCommandInputAuthor(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateAuthorCommandInput", "author", idValue);
}

export function useUpdateAuthorCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateAuthorCommandInput", idValue);
}

export function useUpdateAuthorCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorCommandInput", "context", idValue);
}

export function useUpdateAuthorCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorCommandInput", "id", idValue);
}

export function useUpdateAuthorCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateAuthorCommandInput", "patch", idValue);
}

export function useDeleteAuthorCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteAuthorCommandInput", idValue);
}

export function useDeleteAuthorCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorCommandInput", "context", idValue);
}

export function useDeleteAuthorCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteAuthorCommandInput", "id", idValue);
}

export function useCreateTypeCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateTypeCommandInput", idValue);
}

export function useCreateTypeCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeCommandInput", "context", idValue);
}

export function useCreateTypeCommandInputType(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTypeCommandInput", "type", idValue);
}

export function useUpdateTypeCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateTypeCommandInput", idValue);
}

export function useUpdateTypeCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeCommandInput", "context", idValue);
}

export function useUpdateTypeCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeCommandInput", "id", idValue);
}

export function useUpdateTypeCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTypeCommandInput", "patch", idValue);
}

export function useDeleteTypeCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteTypeCommandInput", idValue);
}

export function useDeleteTypeCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeCommandInput", "context", idValue);
}

export function useDeleteTypeCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTypeCommandInput", "id", idValue);
}

export function useCreateDesignCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateDesignCommandInput", idValue);
}

export function useCreateDesignCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignCommandInput", "context", idValue);
}

export function useCreateDesignCommandInputDesign(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateDesignCommandInput", "design", idValue);
}

export function useUpdateDesignCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateDesignCommandInput", idValue);
}

export function useUpdateDesignCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignCommandInput", "context", idValue);
}

export function useUpdateDesignCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignCommandInput", "id", idValue);
}

export function useUpdateDesignCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateDesignCommandInput", "patch", idValue);
}

export function useDeleteDesignCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteDesignCommandInput", idValue);
}

export function useDeleteDesignCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignCommandInput", "context", idValue);
}

export function useDeleteDesignCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteDesignCommandInput", "id", idValue);
}

export function useCreateQualityCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateQualityCommandInput", idValue);
}

export function useCreateQualityCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityCommandInput", "context", idValue);
}

export function useCreateQualityCommandInputQuality(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateQualityCommandInput", "quality", idValue);
}

export function useUpdateQualityCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateQualityCommandInput", idValue);
}

export function useUpdateQualityCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityCommandInput", "context", idValue);
}

export function useUpdateQualityCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityCommandInput", "id", idValue);
}

export function useUpdateQualityCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateQualityCommandInput", "patch", idValue);
}

export function useDeleteQualityCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteQualityCommandInput", idValue);
}

export function useDeleteQualityCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityCommandInput", "context", idValue);
}

export function useDeleteQualityCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteQualityCommandInput", "id", idValue);
}

export function useCreatePortCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreatePortCommandInput", idValue);
}

export function useCreatePortCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortCommandInput", "context", idValue);
}

export function useCreatePortCommandInputPort(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePortCommandInput", "port", idValue);
}

export function useUpdatePortCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdatePortCommandInput", idValue);
}

export function useUpdatePortCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortCommandInput", "context", idValue);
}

export function useUpdatePortCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortCommandInput", "id", idValue);
}

export function useUpdatePortCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePortCommandInput", "patch", idValue);
}

export function useDeletePortCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeletePortCommandInput", idValue);
}

export function useDeletePortCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortCommandInput", "context", idValue);
}

export function useDeletePortCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePortCommandInput", "id", idValue);
}

export function useCreateFamilyCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFamilyCommandInput", idValue);
}

export function useCreateFamilyCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyCommandInput", "context", idValue);
}

export function useCreateFamilyCommandInputFamily(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFamilyCommandInput", "family", idValue);
}

export function useUpdateFamilyCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateFamilyCommandInput", idValue);
}

export function useUpdateFamilyCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyCommandInput", "context", idValue);
}

export function useUpdateFamilyCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyCommandInput", "id", idValue);
}

export function useUpdateFamilyCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFamilyCommandInput", "patch", idValue);
}

export function useDeleteFamilyCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteFamilyCommandInput", idValue);
}

export function useDeleteFamilyCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyCommandInput", "context", idValue);
}

export function useDeleteFamilyCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFamilyCommandInput", "id", idValue);
}

export function useCreateTagCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateTagCommandInput", idValue);
}

export function useCreateTagCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagCommandInput", "context", idValue);
}

export function useCreateTagCommandInputTag(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateTagCommandInput", "tag", idValue);
}

export function useUpdateTagCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateTagCommandInput", idValue);
}

export function useUpdateTagCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagCommandInput", "context", idValue);
}

export function useUpdateTagCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagCommandInput", "id", idValue);
}

export function useUpdateTagCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateTagCommandInput", "patch", idValue);
}

export function useDeleteTagCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteTagCommandInput", idValue);
}

export function useDeleteTagCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagCommandInput", "context", idValue);
}

export function useDeleteTagCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteTagCommandInput", "id", idValue);
}

export function useCreateConceptCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConceptCommandInput", idValue);
}

export function useCreateConceptCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptCommandInput", "context", idValue);
}

export function useCreateConceptCommandInputConcept(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConceptCommandInput", "concept", idValue);
}

export function useUpdateConceptCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateConceptCommandInput", idValue);
}

export function useUpdateConceptCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptCommandInput", "context", idValue);
}

export function useUpdateConceptCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptCommandInput", "id", idValue);
}

export function useUpdateConceptCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConceptCommandInput", "patch", idValue);
}

export function useDeleteConceptCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteConceptCommandInput", idValue);
}

export function useDeleteConceptCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptCommandInput", "context", idValue);
}

export function useDeleteConceptCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConceptCommandInput", "id", idValue);
}

export function useCreateFileCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFileCommandInput", idValue);
}

export function useCreateFileCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileCommandInput", "context", idValue);
}

export function useCreateFileCommandInputFile(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFileCommandInput", "file", idValue);
}

export function useUpdateFileCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateFileCommandInput", idValue);
}

export function useUpdateFileCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileCommandInput", "context", idValue);
}

export function useUpdateFileCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileCommandInput", "id", idValue);
}

export function useUpdateFileCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFileCommandInput", "patch", idValue);
}

export function useDeleteFileCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteFileCommandInput", idValue);
}

export function useDeleteFileCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileCommandInput", "context", idValue);
}

export function useDeleteFileCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFileCommandInput", "id", idValue);
}

export function useCreateFolderCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFolderCommandInput", idValue);
}

export function useCreateFolderCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderCommandInput", "context", idValue);
}

export function useCreateFolderCommandInputFolder(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFolderCommandInput", "folder", idValue);
}

export function useUpdateFolderCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateFolderCommandInput", idValue);
}

export function useUpdateFolderCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderCommandInput", "context", idValue);
}

export function useUpdateFolderCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderCommandInput", "id", idValue);
}

export function useUpdateFolderCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateFolderCommandInput", "patch", idValue);
}

export function useDeleteFolderCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteFolderCommandInput", idValue);
}

export function useDeleteFolderCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderCommandInput", "context", idValue);
}

export function useDeleteFolderCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteFolderCommandInput", "id", idValue);
}

export function useMoveArtifactToFolderCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("MoveArtifactToFolderCommandInput", idValue);
}

export function useMoveArtifactToFolderCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderCommandInput", "context", idValue);
}

export function useMoveArtifactToFolderCommandInputArtifactKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderCommandInput", "artifactKind", idValue);
}

export function useMoveArtifactToFolderCommandInputArtifactId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderCommandInput", "artifactId", idValue);
}

export function useMoveArtifactToFolderCommandInputFolderId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MoveArtifactToFolderCommandInput", "folderId", idValue);
}

export function useCreatePieceCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreatePieceCommandInput", idValue);
}

export function useCreatePieceCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceCommandInput", "context", idValue);
}

export function useCreatePieceCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceCommandInput", "designId", idValue);
}

export function useCreatePieceCommandInputPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePieceCommandInput", "piece", idValue);
}

export function useCreatePiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreatePiecesCommandInput", idValue);
}

export function useCreatePiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesCommandInput", "context", idValue);
}

export function useCreatePiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesCommandInput", "designId", idValue);
}

export function useCreatePiecesCommandInputPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreatePiecesCommandInput", "pieces", idValue);
}

export function usePieceUpdateInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PieceUpdateInput", idValue);
}

export function usePieceUpdateInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceUpdateInput", "id", idValue);
}

export function usePieceUpdateInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PieceUpdateInput", "patch", idValue);
}

export function useUpdatePieceCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdatePieceCommandInput", idValue);
}

export function useUpdatePieceCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceCommandInput", "context", idValue);
}

export function useUpdatePieceCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceCommandInput", "designId", idValue);
}

export function useUpdatePieceCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceCommandInput", "id", idValue);
}

export function useUpdatePieceCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePieceCommandInput", "patch", idValue);
}

export function useUpdatePiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdatePiecesCommandInput", idValue);
}

export function useUpdatePiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesCommandInput", "context", idValue);
}

export function useUpdatePiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesCommandInput", "designId", idValue);
}

export function useUpdatePiecesCommandInputUpdates(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdatePiecesCommandInput", "updates", idValue);
}

export function useDeletePieceCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeletePieceCommandInput", idValue);
}

export function useDeletePieceCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceCommandInput", "context", idValue);
}

export function useDeletePieceCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceCommandInput", "designId", idValue);
}

export function useDeletePieceCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePieceCommandInput", "id", idValue);
}

export function useDeletePiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeletePiecesCommandInput", idValue);
}

export function useDeletePiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesCommandInput", "context", idValue);
}

export function useDeletePiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesCommandInput", "designId", idValue);
}

export function useDeletePiecesCommandInputIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeletePiecesCommandInput", "ids", idValue);
}

export function useCreateConnectionCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConnectionCommandInput", idValue);
}

export function useCreateConnectionCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionCommandInput", "context", idValue);
}

export function useCreateConnectionCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionCommandInput", "designId", idValue);
}

export function useCreateConnectionCommandInputConnection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionCommandInput", "connection", idValue);
}

export function useCreateConnectionsCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConnectionsCommandInput", idValue);
}

export function useCreateConnectionsCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsCommandInput", "context", idValue);
}

export function useCreateConnectionsCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsCommandInput", "designId", idValue);
}

export function useCreateConnectionsCommandInputConnections(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectionsCommandInput", "connections", idValue);
}

export function useConnectionUpdateInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ConnectionUpdateInput", idValue);
}

export function useConnectionUpdateInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionUpdateInput", "id", idValue);
}

export function useConnectionUpdateInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ConnectionUpdateInput", "patch", idValue);
}

export function useUpdateConnectionCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateConnectionCommandInput", idValue);
}

export function useUpdateConnectionCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionCommandInput", "context", idValue);
}

export function useUpdateConnectionCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionCommandInput", "designId", idValue);
}

export function useUpdateConnectionCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionCommandInput", "id", idValue);
}

export function useUpdateConnectionCommandInputPatch(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionCommandInput", "patch", idValue);
}

export function useUpdateConnectionsCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("UpdateConnectionsCommandInput", idValue);
}

export function useUpdateConnectionsCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsCommandInput", "context", idValue);
}

export function useUpdateConnectionsCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsCommandInput", "designId", idValue);
}

export function useUpdateConnectionsCommandInputUpdates(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("UpdateConnectionsCommandInput", "updates", idValue);
}

export function useDeleteConnectionCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteConnectionCommandInput", idValue);
}

export function useDeleteConnectionCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionCommandInput", "context", idValue);
}

export function useDeleteConnectionCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionCommandInput", "designId", idValue);
}

export function useDeleteConnectionCommandInputId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionCommandInput", "id", idValue);
}

export function useDeleteConnectionsCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteConnectionsCommandInput", idValue);
}

export function useDeleteConnectionsCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsCommandInput", "context", idValue);
}

export function useDeleteConnectionsCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsCommandInput", "designId", idValue);
}

export function useDeleteConnectionsCommandInputIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteConnectionsCommandInput", "ids", idValue);
}

export function useDeleteSelectionCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DeleteSelectionCommandInput", idValue);
}

export function useDeleteSelectionCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionCommandInput", "context", idValue);
}

export function useDeleteSelectionCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionCommandInput", "designId", idValue);
}

export function useDeleteSelectionCommandInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionCommandInput", "pieceIds", idValue);
}

export function useDeleteSelectionCommandInputConnectionIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DeleteSelectionCommandInput", "connectionIds", idValue);
}

export function useFixPiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FixPiecesCommandInput", idValue);
}

export function useFixPiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesCommandInput", "context", idValue);
}

export function useFixPiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesCommandInput", "designId", idValue);
}

export function useFixPiecesCommandInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FixPiecesCommandInput", "pieceIds", idValue);
}

export function useClusterPiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ClusterPiecesCommandInput", idValue);
}

export function useClusterPiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesCommandInput", "context", idValue);
}

export function useClusterPiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesCommandInput", "designId", idValue);
}

export function useClusterPiecesCommandInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesCommandInput", "pieceIds", idValue);
}

export function useClusterPiecesCommandInputNewDesignName(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ClusterPiecesCommandInput", "newDesignName", idValue);
}

export function useExpandDesignReferenceCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ExpandDesignReferenceCommandInput", idValue);
}

export function useExpandDesignReferenceCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceCommandInput", "context", idValue);
}

export function useExpandDesignReferenceCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceCommandInput", "designId", idValue);
}

export function useExpandDesignReferenceCommandInputReferencedDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExpandDesignReferenceCommandInput", "referencedDesignId", idValue);
}

export function useFlattenDesignCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("FlattenDesignCommandInput", idValue);
}

export function useFlattenDesignCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignCommandInput", "context", idValue);
}

export function useFlattenDesignCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("FlattenDesignCommandInput", "designId", idValue);
}

export function useDragPiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("DragPiecesCommandInput", idValue);
}

export function useDragPiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesCommandInput", "context", idValue);
}

export function useDragPiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesCommandInput", "designId", idValue);
}

export function useDragPiecesCommandInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesCommandInput", "pieceIds", idValue);
}

export function useDragPiecesCommandInputOffset(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("DragPiecesCommandInput", "offset", idValue);
}

export function useMovePiecesVectorInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("MovePiecesVectorInput", idValue);
}

export function useMovePiecesVectorInputShift(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesVectorInput", "shift", idValue);
}

export function useMovePiecesVectorInputGap(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesVectorInput", "gap", idValue);
}

export function useMovePiecesVectorInputRise(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesVectorInput", "rise", idValue);
}

export function useMovePiecesVectorInputRotation(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesVectorInput", "rotation", idValue);
}

export function useMovePiecesVectorInputTurn(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesVectorInput", "turn", idValue);
}

export function useMovePiecesVectorInputTilt(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesVectorInput", "tilt", idValue);
}

export function useMovePiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("MovePiecesCommandInput", idValue);
}

export function useMovePiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesCommandInput", "context", idValue);
}

export function useMovePiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesCommandInput", "designId", idValue);
}

export function useMovePiecesCommandInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesCommandInput", "pieceIds", idValue);
}

export function useMovePiecesCommandInputVector(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("MovePiecesCommandInput", "vector", idValue);
}

export function useCreateFixedPieceCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateFixedPieceCommandInput", idValue);
}

export function useCreateFixedPieceCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceCommandInput", "context", idValue);
}

export function useCreateFixedPieceCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceCommandInput", "designId", idValue);
}

export function useCreateFixedPieceCommandInputPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateFixedPieceCommandInput", "piece", idValue);
}

export function useCreateConnectedPieceCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateConnectedPieceCommandInput", idValue);
}

export function useCreateConnectedPieceCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceCommandInput", "context", idValue);
}

export function useCreateConnectedPieceCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceCommandInput", "designId", idValue);
}

export function useCreateConnectedPieceCommandInputPiece(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceCommandInput", "piece", idValue);
}

export function useCreateConnectedPieceCommandInputConnection(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateConnectedPieceCommandInput", "connection", idValue);
}

export function useCreateHangingPiecesCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("CreateHangingPiecesCommandInput", idValue);
}

export function useCreateHangingPiecesCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "context", idValue);
}

export function useCreateHangingPiecesCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "designId", idValue);
}

export function useCreateHangingPiecesCommandInputPieces(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "pieces", idValue);
}

export function useCreateHangingPiecesCommandInputParentPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentPieceId", idValue);
}

export function useCreateHangingPiecesCommandInputParentDesignPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentDesignPieceId", idValue);
}

export function useCreateHangingPiecesCommandInputParentConnectorId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentConnectorId", idValue);
}

export function useCreateHangingPiecesCommandInputConnectionTemplate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("CreateHangingPiecesCommandInput", "connectionTemplate", idValue);
}

export function useChangePieceTypeCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ChangePieceTypeCommandInput", idValue);
}

export function useChangePieceTypeCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeCommandInput", "context", idValue);
}

export function useChangePieceTypeCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeCommandInput", "designId", idValue);
}

export function useChangePieceTypeCommandInputPieceId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeCommandInput", "pieceId", idValue);
}

export function useChangePieceTypeCommandInputTypeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePieceTypeCommandInput", "typeId", idValue);
}

export function useChangePiecesTypeCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ChangePiecesTypeCommandInput", idValue);
}

export function useChangePiecesTypeCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeCommandInput", "context", idValue);
}

export function useChangePiecesTypeCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeCommandInput", "designId", idValue);
}

export function useChangePiecesTypeCommandInputPieceIds(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeCommandInput", "pieceIds", idValue);
}

export function useChangePiecesTypeCommandInputTypeId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ChangePiecesTypeCommandInput", "typeId", idValue);
}

export function usePasteDesignSelectionCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("PasteDesignSelectionCommandInput", idValue);
}

export function usePasteDesignSelectionCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionCommandInput", "context", idValue);
}

export function usePasteDesignSelectionCommandInputDesignId(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionCommandInput", "designId", idValue);
}

export function usePasteDesignSelectionCommandInputPayload(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionCommandInput", "payload", idValue);
}

export function usePasteDesignSelectionCommandInputOffset(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("PasteDesignSelectionCommandInput", "offset", idValue);
}

export function useImportKitCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ImportKitCommandInput", idValue);
}

export function useImportKitCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitCommandInput", "context", idValue);
}

export function useImportKitCommandInputSourceUrl(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitCommandInput", "sourceUrl", idValue);
}

export function useImportKitCommandInputArchiveBase64(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ImportKitCommandInput", "archiveBase64", idValue);
}

export function useResetKitCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ResetKitCommandInput", idValue);
}

export function useResetKitCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitCommandInput", "context", idValue);
}

export function useResetKitCommandInputSourceUrl(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitCommandInput", "sourceUrl", idValue);
}

export function useResetKitCommandInputArchiveBase64(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitCommandInput", "archiveBase64", idValue);
}

export function useResetKitCommandInputKit(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ResetKitCommandInput", "kit", idValue);
}

export function useExportKitCommandInput(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("ExportKitCommandInput", idValue);
}

export function useExportKitCommandInputContext(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("ExportKitCommandInput", "context", idValue);
}

export function useQuery(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Query", idValue);
}

export function useQueryKitCommandCatalog(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("Query", "kitCommandCatalog", idValue);
}

export function useMutation(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("Mutation", idValue);
}

export function useKitStoreEventKindEnum(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitStoreEventKind", idValue);
}

export function useKitStoreEvent(idValue?: string): HookTriad<any> {
  return useSchemaObjectState("KitStoreEvent", idValue);
}

export function useKitStoreEventHash(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "hash", idValue);
}

export function useKitStoreEventKind(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "kind", idValue);
}

export function useKitStoreEventStore(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "store", idValue);
}

export function useKitStoreEventInteraction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "interaction", idValue);
}

export function useKitStoreEventChange(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "change", idValue);
}

export function useKitStoreEventCandidate(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "candidate", idValue);
}

export function useKitStoreEventConflict(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "conflict", idValue);
}

export function useKitStoreEventSession(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "session", idValue);
}

export function useKitStoreEventTransaction(idValue?: string): HookTriad<any> {
  return useSchemaFieldState("KitStoreEvent", "transaction", idValue);
}

export const schemaHooks = Object.freeze({
  useJSON,
  useActorKind,
  useActor,
  useActorId,
  useActorName,
  useActorEmail,
  useActorColor,
  useUser,
  useUserHash,
  useUserId,
  useUserName,
  useUserEmail,
  useUserColor,
  useReconnectKitSessionKitInteraction,
  useDragPiecesKitInteractionIndex,
  useDragPiecesKitInteractionKit,
  useDragPiecesKitInteractionKind,
  useDragPiecesKitInteractionActor,
  useDragPiecesKitInteractionSession,
  useDragPiecesKitInteractionTransaction,
  useDragPiecesKitInteractionForward,
  useDragPiecesKitInteractionBackward,
  usePasteDesignSelectionKitInteractionChange,
  usePasteDesignSelectionKitInteractionConflict,
  usePasteDesignSelectionKitInteractionSummary,
  usePasteDesignSelectionKitInteractionMetadata,
  usePasteDesignSelectionKitInteractionCreatedAt,
  usePasteDesignSelectionKitInteractionForward,
  usePasteDesignSelectionKitInteractionBackward,
  useImportKitKitInteractionId,
  useImportKitKitInteractionHash,
  useImportKitKitInteractionIndex,
  useImportKitKitInteractionKit,
  useImportKitKitInteractionKind,
  useImportKitKitInteractionActor,
  useImportKitKitInteractionSession,
  useImportKitKitInteractionTransaction,
  useImportKitKitInteractionCandidate,
  useImportKitKitInteractionChange,
  useImportKitKitInteractionConflict,
  useImportKitKitInteractionSummary,
  useImportKitKitInteractionMetadata,
  useImportKitKitInteractionCreatedAt,
  useImportKitKitInteractionForward,
  useImportKitKitInteractionBackward,
  useResetKitKitInteractionId,
  useResetKitKitInteractionHash,
  useResetKitKitInteractionIndex,
  useResetKitKitInteractionKit,
  useResetKitKitInteractionKind,
  useResetKitKitInteractionActor,
  useResetKitKitInteractionSession,
  useResetKitKitInteractionTransaction,
  useResetKitKitInteractionCandidate,
  useResetKitKitInteractionChange,
  useResetKitKitInteractionConflict,
  useResetKitKitInteractionSummary,
  useResetKitKitInteractionMetadata,
  useResetKitKitInteractionCreatedAt,
  useResetKitKitInteractionForward,
  useResetKitKitInteractionBackward,
  useExportKitKitInteractionId,
  useExportKitKitInteractionHash,
  useExportKitKitInteractionIndex,
  useExportKitKitInteractionKit,
  useExportKitKitInteractionKind,
  useExportKitKitInteractionActor,
  useExportKitKitInteractionSession,
  useExportKitKitInteractionTransaction,
  useExportKitKitInteractionCandidate,
  useExportKitKitInteractionChange,
  useExportKitKitInteractionConflict,
  useExportKitKitInteractionSummary,
  useExportKitKitInteractionMetadata,
  useExportKitKitInteractionCreatedAt,
  useExportKitKitInteractionForward,
  useExportKitKitInteractionBackward,
  useStartKitSessionKitInteractionId,
  useStartKitSessionKitInteractionHash,
  useStartKitSessionKitInteractionIndex,
  useStartKitSessionKitInteractionKit,
  useStartKitSessionKitInteractionKind,
  useStartKitSessionKitInteractionActor,
  useStartKitSessionKitInteractionSession,
  useStartKitSessionKitInteractionTransaction,
  useStartKitSessionKitInteractionCandidate,
  useStartKitSessionKitInteractionChange,
  useStartKitSessionKitInteractionConflict,
  useStartKitSessionKitInteractionSummary,
  useStartKitSessionKitInteractionMetadata,
  useStartKitSessionKitInteractionCreatedAt,
  useStartKitSessionKitInteractionForward,
  useStartKitSessionKitInteractionBackward,
  useHeartbeatKitSessionKitInteractionId,
  useHeartbeatKitSessionKitInteractionHash,
  useHeartbeatKitSessionKitInteractionIndex,
  useHeartbeatKitSessionKitInteractionKit,
  useHeartbeatKitSessionKitInteractionKind,
  useHeartbeatKitSessionKitInteractionActor,
  useHeartbeatKitSessionKitInteractionSession,
  useHeartbeatKitSessionKitInteractionTransaction,
  useHeartbeatKitSessionKitInteractionCandidate,
  useHeartbeatKitSessionKitInteractionChange,
  useHeartbeatKitSessionKitInteractionConflict,
  useHeartbeatKitSessionKitInteractionSummary,
  useHeartbeatKitSessionKitInteractionMetadata,
  useHeartbeatKitSessionKitInteractionCreatedAt,
  useHeartbeatKitSessionKitInteractionForward,
  useHeartbeatKitSessionKitInteractionBackward,
  useEndKitSessionKitInteractionId,
  useEndKitSessionKitInteractionHash,
  useEndKitSessionKitInteractionIndex,
  useEndKitSessionKitInteractionKit,
  useEndKitSessionKitInteractionKind,
  useEndKitSessionKitInteractionActor,
  useEndKitSessionKitInteractionSession,
  useEndKitSessionKitInteractionTransaction,
  useEndKitSessionKitInteractionCandidate,
  useEndKitSessionKitInteractionChange,
  useEndKitSessionKitInteractionConflict,
  useEndKitSessionKitInteractionSummary,
  useEndKitSessionKitInteractionMetadata,
  useEndKitSessionKitInteractionCreatedAt,
  useEndKitSessionKitInteractionForward,
  useEndKitSessionKitInteractionBackward,
  useReconnectKitSessionKitInteractionId,
  useReconnectKitSessionKitInteractionHash,
  useReconnectKitSessionKitInteractionIndex,
  useReconnectKitSessionKitInteractionKit,
  useReconnectKitSessionKitInteractionKind,
  useReconnectKitSessionKitInteractionActor,
  useReconnectKitSessionKitInteractionSession,
  useReconnectKitSessionKitInteractionTransaction,
  useReconnectKitSessionKitInteractionCandidate,
  useReconnectKitSessionKitInteractionChange,
  useReconnectKitSessionKitInteractionConflict,
  useReconnectKitSessionKitInteractionSummary,
  useReconnectKitSessionKitInteractionMetadata,
  useReconnectKitSessionKitInteractionCreatedAt,
  useReconnectKitSessionKitInteractionForward,
  useReconnectKitSessionKitInteractionBackward,
  useBeginKitTransactionKitInteractionId,
  useBeginKitTransactionKitInteractionHash,
  useBeginKitTransactionKitInteractionIndex,
  useBeginKitTransactionKitInteractionKit,
  useBeginKitTransactionKitInteractionKind,
  useBeginKitTransactionKitInteractionActor,
  useBeginKitTransactionKitInteractionSession,
  useBeginKitTransactionKitInteractionTransaction,
  useBeginKitTransactionKitInteractionCandidate,
  useBeginKitTransactionKitInteractionChange,
  useBeginKitTransactionKitInteractionConflict,
  useBeginKitTransactionKitInteractionSummary,
  useBeginKitTransactionKitInteractionMetadata,
  useBeginKitTransactionKitInteractionCreatedAt,
  useBeginKitTransactionKitInteractionForward,
  useBeginKitTransactionKitInteractionBackward,
  useFinalizeKitTransactionKitInteractionId,
  useFinalizeKitTransactionKitInteractionHash,
  useFinalizeKitTransactionKitInteractionIndex,
  useFinalizeKitTransactionKitInteractionKit,
  useFinalizeKitTransactionKitInteractionKind,
  useFinalizeKitTransactionKitInteractionActor,
  useFinalizeKitTransactionKitInteractionSession,
  useFinalizeKitTransactionKitInteractionTransaction,
  useFinalizeKitTransactionKitInteractionCandidate,
  useFinalizeKitTransactionKitInteractionChange,
  useFinalizeKitTransactionKitInteractionConflict,
  useFinalizeKitTransactionKitInteractionSummary,
  useFinalizeKitTransactionKitInteractionMetadata,
  useFinalizeKitTransactionKitInteractionCreatedAt,
  useFinalizeKitTransactionKitInteractionForward,
  useFinalizeKitTransactionKitInteractionBackward,
  useAbortKitTransactionKitInteractionId,
  useAbortKitTransactionKitInteractionHash,
  useAbortKitTransactionKitInteractionIndex,
  useAbortKitTransactionKitInteractionKit,
  useAbortKitTransactionKitInteractionKind,
  useAbortKitTransactionKitInteractionActor,
  useAbortKitTransactionKitInteractionSession,
  useAbortKitTransactionKitInteractionTransaction,
  useAbortKitTransactionKitInteractionCandidate,
  useAbortKitTransactionKitInteractionChange,
  useAbortKitTransactionKitInteractionConflict,
  useAbortKitTransactionKitInteractionSummary,
  useAbortKitTransactionKitInteractionMetadata,
  useAbortKitTransactionKitInteractionCreatedAt,
  useAbortKitTransactionKitInteractionForward,
  useAbortKitTransactionKitInteractionBackward,
  useTransactionStepKitInteractionId,
  useTransactionStepKitInteractionHash,
  useTransactionStepKitInteractionIndex,
  useTransactionStepKitInteractionKit,
  useTransactionStepKitInteractionKind,
  useTransactionStepKitInteractionActor,
  useTransactionStepKitInteractionSession,
  useTransactionStepKitInteractionTransaction,
  useTransactionStepKitInteractionCandidate,
  useTransactionStepKitInteractionChange,
  useTransactionStepKitInteractionConflict,
  useTransactionStepKitInteractionSummary,
  useTransactionStepKitInteractionMetadata,
  useTransactionStepKitInteractionCreatedAt,
  useTransactionStepKitInteractionForward,
  useTransactionStepKitInteractionBackward,
  useHistoryStepKitInteractionId,
  useHistoryStepKitInteractionHash,
  useHistoryStepKitInteractionIndex,
  useHistoryStepKitInteractionKit,
  useHistoryStepKitInteractionKind,
  useHistoryStepKitInteractionActor,
  useHistoryStepKitInteractionSession,
  useHistoryStepKitInteractionTransaction,
  useHistoryStepKitInteractionCandidate,
  useHistoryStepKitInteractionChange,
  useHistoryStepKitInteractionConflict,
  useHistoryStepKitInteractionSummary,
  useHistoryStepKitInteractionMetadata,
  useHistoryStepKitInteractionCreatedAt,
  useHistoryStepKitInteractionForward,
  useHistoryStepKitInteractionBackward,
  useVoteOnKitChangeCandidateKitInteractionId,
  useVoteOnKitChangeCandidateKitInteractionHash,
  useVoteOnKitChangeCandidateKitInteractionIndex,
  useVoteOnKitChangeCandidateKitInteractionKit,
  useVoteOnKitChangeCandidateKitInteractionKind,
  useVoteOnKitChangeCandidateKitInteractionActor,
  useVoteOnKitChangeCandidateKitInteractionSession,
  useVoteOnKitChangeCandidateKitInteractionTransaction,
  useVoteOnKitChangeCandidateKitInteractionCandidate,
  useVoteOnKitChangeCandidateKitInteractionChange,
  useVoteOnKitChangeCandidateKitInteractionConflict,
  useVoteOnKitChangeCandidateKitInteractionSummary,
  useVoteOnKitChangeCandidateKitInteractionMetadata,
  useVoteOnKitChangeCandidateKitInteractionCreatedAt,
  useVoteOnKitChangeCandidateKitInteractionForward,
  useVoteOnKitChangeCandidateKitInteractionBackward,
  useResolveKitConflictKitInteractionId,
  useResolveKitConflictKitInteractionHash,
  useResolveKitConflictKitInteractionIndex,
  useResolveKitConflictKitInteractionKit,
  useResolveKitConflictKitInteractionKind,
  useResolveKitConflictKitInteractionActor,
  useResolveKitConflictKitInteractionSession,
  useResolveKitConflictKitInteractionTransaction,
  useResolveKitConflictKitInteractionCandidate,
  useResolveKitConflictKitInteractionChange,
  useResolveKitConflictKitInteractionConflict,
  useResolveKitConflictKitInteractionSummary,
  useResolveKitConflictKitInteractionMetadata,
  useResolveKitConflictKitInteractionCreatedAt,
  useResolveKitConflictKitInteractionForward,
  useResolveKitConflictKitInteractionBackward,
  useKitInteractionPageHash,
  useKitInteractionPageNodes,
  useKitInteractionPagePageInfo,
  useKitInteractionPageTotalCount,
  useKitHistoryHash,
  useKitHistoryCanUndo,
  useKitHistoryCanRedo,
  useKitHistoryTotalCount,
  useKitHistoryHead,
  useKitStoreHash,
  useKitStoreKit,
  useKitStoreBackbone,
  useKitStoreSessions,
  useKitStoreTransactions,
  useKitStorePendingCandidates,
  useKitStoreActiveConflicts,
  useKitStoreValidation,
  useKitStoreHistory,
  useKitStoreBlockedByConflict,
  useKitStoreStrictMode,
  useKitArchiveExportHash,
  useKitArchiveExportFileName,
  useKitArchiveExportUrl,
  useKitArchiveExportExpiresAt,
  useKitMutationResultHash,
  useKitMutationResultAccepted,
  useKitMutationResultKind,
  useKitMutationResultSummary,
  useKitMutationResultStore,
  useKitMutationResultKit,
  useKitMutationResultSession,
  useKitMutationResultTransaction,
  useKitMutationResultCandidate,
  useKitMutationResultChange,
  useKitMutationResultHistoryEntry,
  useKitMutationResultConflict,
  useKitMutationResultValidation,
  useKitMutationResultExport,
  useKitCommandContextInputKitId,
  useKitCommandContextInputSessionId,
  useKitCommandContextInputTransactionId,
  useKitCommandContextInputOrigin,
  useKitCommandContextInputExpectedHash,
  useKitCommandContextInputStrictMode,
  useStartKitSessionInputKitId,
  useStartKitSessionInputActor,
  useStartKitSessionInputClient,
  useStartKitSessionInputStrictMode,
  useHeartbeatKitSessionInputKitId,
  useHeartbeatKitSessionInputSessionId,
  useHeartbeatKitSessionInputLastKnownHash,
  useEndKitSessionInputKitId,
  useEndKitSessionInputSessionId,
  useReconnectKitSessionInputKitId,
  useReconnectKitSessionInputSessionId,
  useReconnectKitSessionInputClient,
  useReconnectKitSessionInputLastKnownHash,
  useSetSessionSelectionCommandInputContext,
  useSetSessionSelectionCommandInputMode,
  useSetSessionSelectionCommandInputSelection,
  useBeginKitTransactionInputContext,
  useBeginKitTransactionInputLabel,
  useBeginKitTransactionInputParentTransactionId,
  useFinalizeKitTransactionInputContext,
  useFinalizeKitTransactionInputTransactionId,
  useAbortKitTransactionInputContext,
  useAbortKitTransactionInputTransactionId,
  useTransactionStepInputContext,
  useTransactionStepInputTransactionId,
  useHistoryStepInputContext,
  useHistoryStepInputSteps,
  useVoteOnKitChangeCandidateInputContext,
  useVoteOnKitChangeCandidateInputCandidateId,
  useVoteOnKitChangeCandidateInputState,
  useVoteOnKitChangeCandidateInputReason,
  useVoteOnKitChangeCandidateInputResolutionOptionId,
  useResolveKitConflictInputContext,
  useResolveKitConflictInputConflictId,
  useResolveKitConflictInputOptionId,
  useResolveKitConflictInputPayload,
  useCreateAuthorCommandInputContext,
  useCreateAuthorCommandInputAuthor,
  useUpdateAuthorCommandInputContext,
  useUpdateAuthorCommandInputId,
  useUpdateAuthorCommandInputPatch,
  useDeleteAuthorCommandInputContext,
  useDeleteAuthorCommandInputId,
  useCreateTypeCommandInputContext,
  useCreateTypeCommandInputType,
  useUpdateTypeCommandInputContext,
  useUpdateTypeCommandInputId,
  useUpdateTypeCommandInputPatch,
  useDeleteTypeCommandInputContext,
  useDeleteTypeCommandInputId,
  useCreateDesignCommandInputContext,
  useCreateDesignCommandInputDesign,
  useUpdateDesignCommandInputContext,
  useUpdateDesignCommandInputId,
  useUpdateDesignCommandInputPatch,
  useDeleteDesignCommandInputContext,
  useDeleteDesignCommandInputId,
  useCreateQualityCommandInputContext,
  useCreateQualityCommandInputQuality,
  useUpdateQualityCommandInputContext,
  useUpdateQualityCommandInputId,
  useUpdateQualityCommandInputPatch,
  useDeleteQualityCommandInputContext,
  useDeleteQualityCommandInputId,
  useCreatePortCommandInputContext,
  useCreatePortCommandInputPort,
  useUpdatePortCommandInputContext,
  useUpdatePortCommandInputId,
  useUpdatePortCommandInputPatch,
  useDeletePortCommandInputContext,
  useDeletePortCommandInputId,
  useCreateFamilyCommandInputContext,
  useCreateFamilyCommandInputFamily,
  useUpdateFamilyCommandInputContext,
  useUpdateFamilyCommandInputId,
  useUpdateFamilyCommandInputPatch,
  useDeleteFamilyCommandInputContext,
  useDeleteFamilyCommandInputId,
  useCreateTagCommandInputContext,
  useCreateTagCommandInputTag,
  useUpdateTagCommandInputContext,
  useUpdateTagCommandInputId,
  useUpdateTagCommandInputPatch,
  useDeleteTagCommandInputContext,
  useDeleteTagCommandInputId,
  useCreateConceptCommandInputContext,
  useCreateConceptCommandInputConcept,
  useUpdateConceptCommandInputContext,
  useUpdateConceptCommandInputId,
  useUpdateConceptCommandInputPatch,
  useDeleteConceptCommandInputContext,
  useDeleteConceptCommandInputId,
  useCreateFileCommandInputContext,
  useCreateFileCommandInputFile,
  useUpdateFileCommandInputContext,
  useUpdateFileCommandInputId,
  useUpdateFileCommandInputPatch,
  useDeleteFileCommandInputContext,
  useDeleteFileCommandInputId,
  useCreateFolderCommandInputContext,
  useCreateFolderCommandInputFolder,
  useUpdateFolderCommandInputContext,
  useUpdateFolderCommandInputId,
  useUpdateFolderCommandInputPatch,
  useDeleteFolderCommandInputContext,
  useDeleteFolderCommandInputId,
  useMoveArtifactToFolderCommandInputContext,
  useMoveArtifactToFolderCommandInputArtifactKind,
  useMoveArtifactToFolderCommandInputArtifactId,
  useMoveArtifactToFolderCommandInputFolderId,
  useCreatePieceCommandInputContext,
  useCreatePieceCommandInputDesignId,
  useCreatePieceCommandInputPiece,
  useCreatePiecesCommandInputContext,
  useCreatePiecesCommandInputDesignId,
  useCreatePiecesCommandInputPieces,
  usePieceUpdateInputId,
  usePieceUpdateInputPatch,
  useUpdatePieceCommandInputContext,
  useUpdatePieceCommandInputDesignId,
  useUpdatePieceCommandInputId,
  useUpdatePieceCommandInputPatch,
  useUpdatePiecesCommandInputContext,
  useUpdatePiecesCommandInputDesignId,
  useUpdatePiecesCommandInputUpdates,
  useDeletePieceCommandInputContext,
  useDeletePieceCommandInputDesignId,
  useDeletePieceCommandInputId,
  useDeletePiecesCommandInputContext,
  useDeletePiecesCommandInputDesignId,
  useDeletePiecesCommandInputIds,
  useCreateConnectionCommandInputContext,
  useCreateConnectionCommandInputDesignId,
  useCreateConnectionCommandInputConnection,
  useCreateConnectionsCommandInputContext,
  useCreateConnectionsCommandInputDesignId,
  useCreateConnectionsCommandInputConnections,
  useConnectionUpdateInputId,
  useConnectionUpdateInputPatch,
  useUpdateConnectionCommandInputContext,
  useUpdateConnectionCommandInputDesignId,
  useUpdateConnectionCommandInputId,
  useUpdateConnectionCommandInputPatch,
  useUpdateConnectionsCommandInputContext,
  useUpdateConnectionsCommandInputDesignId,
  useUpdateConnectionsCommandInputUpdates,
  useDeleteConnectionCommandInputContext,
  useDeleteConnectionCommandInputDesignId,
  useDeleteConnectionCommandInputId,
  useDeleteConnectionsCommandInputContext,
  useDeleteConnectionsCommandInputDesignId,
  useDeleteConnectionsCommandInputIds,
  useDeleteSelectionCommandInputContext,
  useDeleteSelectionCommandInputDesignId,
  useDeleteSelectionCommandInputPieceIds,
  useDeleteSelectionCommandInputConnectionIds,
  useFixPiecesCommandInputContext,
  useFixPiecesCommandInputDesignId,
  useFixPiecesCommandInputPieceIds,
  useClusterPiecesCommandInputContext,
  useClusterPiecesCommandInputDesignId,
  useClusterPiecesCommandInputPieceIds,
  useClusterPiecesCommandInputNewDesignName,
  useExpandDesignReferenceCommandInputContext,
  useExpandDesignReferenceCommandInputDesignId,
  useExpandDesignReferenceCommandInputReferencedDesignId,
  useFlattenDesignCommandInputContext,
  useFlattenDesignCommandInputDesignId,
  useDragPiecesCommandInputContext,
  useDragPiecesCommandInputDesignId,
  useDragPiecesCommandInputPieceIds,
  useDragPiecesCommandInputOffset,
  useMovePiecesVectorInputShift,
  useMovePiecesVectorInputGap,
  useMovePiecesVectorInputRise,
  useMovePiecesVectorInputRotation,
  useMovePiecesVectorInputTurn,
  useMovePiecesVectorInputTilt,
  useMovePiecesCommandInputContext,
  useMovePiecesCommandInputDesignId,
  useMovePiecesCommandInputPieceIds,
  useMovePiecesCommandInputVector,
  useCreateFixedPieceCommandInputContext,
  useCreateFixedPieceCommandInputDesignId,
  useCreateFixedPieceCommandInputPiece,
  useCreateConnectedPieceCommandInputContext,
  useCreateConnectedPieceCommandInputDesignId,
  useCreateConnectedPieceCommandInputPiece,
  useCreateConnectedPieceCommandInputConnection,
  useCreateHangingPiecesCommandInputContext,
  useCreateHangingPiecesCommandInputDesignId,
  useCreateHangingPiecesCommandInputPieces,
  useCreateHangingPiecesCommandInputParentPieceId,
  useCreateHangingPiecesCommandInputParentDesignPieceId,
  useCreateHangingPiecesCommandInputParentConnectorId,
  useCreateHangingPiecesCommandInputConnectionTemplate,
  useChangePieceTypeCommandInputContext,
  useChangePieceTypeCommandInputDesignId,
  useChangePieceTypeCommandInputPieceId,
  useChangePieceTypeCommandInputTypeId,
  useChangePiecesTypeCommandInputContext,
  useChangePiecesTypeCommandInputDesignId,
  useChangePiecesTypeCommandInputPieceIds,
  useChangePiecesTypeCommandInputTypeId,
  usePasteDesignSelectionCommandInputContext,
  usePasteDesignSelectionCommandInputDesignId,
  usePasteDesignSelectionCommandInputPayload,
  usePasteDesignSelectionCommandInputOffset,
  useImportKitCommandInputContext,
  useImportKitCommandInputSourceUrl,
  useImportKitCommandInputArchiveBase64,
  useResetKitCommandInputContext,
  useResetKitCommandInputSourceUrl,
  useResetKitCommandInputArchiveBase64,
  useResetKitCommandInputKit,
  useExportKitCommandInputContext,
  useQueryKitCommandCatalog,
  useKitStoreEventHash,
  useKitStoreEventKind,
  useKitStoreEventStore,
  useKitStoreEventInteraction,
  useKitStoreEventChange,
  useKitStoreEventCandidate,
  useKitStoreEventConflict,
  useKitStoreEventSession,
  useKitStoreEventTransaction,
});

export function useSchemaHook(hookName: string, idValue?: string): HookTriad<any> {
  const hook = (schemaHooks as Readonly<Record<string, (idValue?: string) => HookTriad<unknown> | undefined>>)[hookName];
  if (typeof hook !== "function") {
    return [undefined, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
  }
  return hook(idValue) as HookTriad<any>;
}

// #endregion ⚛️Direct Domain Exports

// #region ⚛️Embedded tests
const shouldRunReactEmbeddedTests =
  (typeof process !== "undefined" && process.env.SEMIO_REACT_RUN_EMBEDDED_TESTS === "1") || (typeof (globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ !== "undefined" && (globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ === true);

if (shouldRunReactEmbeddedTests) {
  const { describe, expect, it } = await import("vitest");
  const { act, cleanup, render, waitFor } = await import("@testing-library/react");
  const { InMemoryKitStore, asKitInstance, kitReadScopeKey, theKitReadScope } = await import("@semio/js");

  const kitJsonFromStore = (store: KitHostStore) => {
    const host = store as KitHostStore & { _kit?: { toJSON: () => unknown } };
    if (((store as any).__semioKitBridge || (store as any).__semioKitClient) && host._kit) return host._kit.toJSON();
    return store.getSnapshot().kit.toJSON();
  };

  const createTestKitClient = (store: KitHostStore): KitStoreClient =>
    ({
      fetchFullKit: async () => kitJsonFromStore(store) as KitFullDto,
      kitReadScope: theKitReadScope,
      submitChangeKitCommands: async (commands: readonly ChangeKitCommand[]) => {
        const kit: KitFullDto = JSON.parse(JSON.stringify(kitJsonFromStore(store))) as KitFullDto;
        for (const cmd of commands) {
          const c = cmd as Record<string, unknown>;
          if ("name" in c && c.name && typeof c.name === "object") {
            const nm = String((c.name as { name?: string }).name ?? "");
            if (nm.trim() === "") return { ok: false, error: { kind: "IllegalName", message: "name cannot be empty" } };
            (kit as { name: string }).name = nm;
          }
          if ("description" in c && c.description && typeof c.description === "object")
            (kit as { description?: string }).description =
              (c.description as { description?: string | null }).description ?? undefined;
          if ("icon" in c && c.icon && typeof c.icon === "object")
            (kit as { icon?: string }).icon = (c.icon as { icon?: string | null }).icon ?? undefined;
          if ("image" in c && c.image && typeof c.image === "object")
            (kit as { image?: string }).image = (c.image as { image?: string | null }).image ?? undefined;
          if ("version" in c && c.version && typeof c.version === "object")
            (kit as { version?: string }).version = (c.version as { version?: string | null }).version ?? undefined;
          if ("homepage" in c && c.homepage && typeof c.homepage === "object")
            (kit as { homepage?: string }).homepage = (c.homepage as { homepage?: string | null }).homepage ?? undefined;
          if ("license" in c && c.license && typeof c.license === "object")
            (kit as { license?: string }).license = (c.license as { license?: string | null }).license ?? undefined;
        }
        store.replace(asKitInstance(kit));
        return { ok: true };
      },
      readPieceFlatPlane: async () => null,
      readPieceFlatCenter: async () => null,
      readPieceParentConnectionFull: async () => null,
      readDesignIncludedDesigns: async () => [],
      readDesignClusterableGroups: async () => [],
      readDesignQualitySum: async () => 0,
      readTypeBestRepresentation: async () => null,
      readColoredConnectors: async () => [],
      readDesignReplaceableCatalogTypes: async () => [],
      readDesignReplaceableCatalogDesigns: async () => [],
      readDesignIncludedDesignIds: async () => [],
      kitGraphql: () => {
        throw new Error("kitGraphql not available in embedded test client");
      },
      clusterPieces: async () => ({ ok: true }),
      dragPieces: async () => ({ ok: true }),
      movePieces: async () => ({ ok: true }),
      fixPieces: async () => ({ ok: true }),
      flattenDesign: async () => ({ ok: true }),
      expandDesign: async () => ({ ok: true }),
      deleteConnection: async () => ({ ok: true }),
      changePieceType: async () => ({ ok: true }),
      pasteDesignSelection: async () => ({ ok: true }),
      createHangingPieces: async () => ({ ok: true }),
      createConnectedPiece: async () => ({ ok: true }),
      createFixedPiece: async () => ({ ok: true }),
      getPiecesMetadata: async () => new Map(),
      getPieces: async () => [],
      getConnections: async () => [],
      getDesigns: async () => [],
      getTypes: async () => [],
      getAuthors: async () => [],
      getKitMetadata: async () => {
        const k = kitJsonFromStore(store) as KitFullDto;
        return { id: String(k.id ?? ""), name: String(k.name ?? "") };
      },
      undo: async () => ({ ok: true }),
      redo: async () => ({ ok: true }),
      canUndo: async () => false,
      canRedo: async () => false,
      backboneStatus: async () => ({ attached: false, kind: null, backboneTip: null, pendingWipCheckpoints: 0 }),
      attachBackbone: async () => ({ ok: true } as const),
      detachBackbone: async () => ({ ok: true } as const),
      listConflicts: async () => [] as const,
      resolveConflict: async () => ({ ok: true } as const),
      syncNow: async () => ({ ok: true } as const),
      subscribeKitName: () => () => {},
      getKitNameSnapshot: () => String((kitJsonFromStore(store) as KitFullDto).name ?? ""),
      subscribeRenameStatus: () => () => {},
      getRenameStatusSnapshot: () => KIT_RENAME_STATUS_IDLE,
      rename: async () => ({ ok: false, requestId: "", error: { kind: "NotSupported", message: "embedded test client" } }),
      getKitWriteScope: () => null,
      setKitWriteScope: () => {},
      finalizeKitWriteTransaction: async () => ({ ok: true }),
      abortKitWriteTransaction: async () => ({ ok: true }),
      subscribe: (cb: (ev: any) => void) => store.subscribe(() => cb({ kind: "test" })),
      setKitReadScope: (_s: import("@semio/js").KitReadScope) => {},
      dispose: () => {},
    }) as unknown as KitStoreClient;

  describe("pipeline hooks", () => {
    it("useKitName rejects empty required name via kit client", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [
          {
            id: "d1",
            name: "D",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            pieces: [{ id: "p1", name: "N" }],
          },
        ],
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let setName: ((v: any) => Promise<any>) | undefined;
      let lastStatus: WriteStatus | undefined;
      let client: KitStoreClient | null = null;

      function Probe() {
        const triad = useKitName();
        setName = triad[1];
        lastStatus = triad[2];
        client = useKitStoreClient();
        return null;
      }

      render(React.createElement(KitScope, { store, kitClient, children: React.createElement(Probe) }));

      await waitFor(() => {
        expect(setName).toBeDefined();
        expect(client).not.toBeNull();
      });
      const r = await setName!("");
      expect(r.ok).toBe(false);
      await waitFor(() => expect(lastStatus?.kind).toBe("error"));
    });

    it("embedded kit client stub exposes read promise methods used by live-read hooks", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const c = createTestKitClient(store);
      expect(typeof c.readPieceFlatPlane).toBe("function");
      expect(typeof c.readDesignIncludedDesigns).toBe("function");
      expect(typeof c.readDesignReplaceableCatalogTypes).toBe("function");
    });

    it("kit metadata hooks write through the kit client", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let setName: ((v: any) => Promise<any>) | undefined;
      let setRelease: ((v: any) => Promise<any>) | undefined;
      let setDescription: ((v: any) => Promise<any>) | undefined;
      let setIcon: ((v: any) => Promise<any>) | undefined;
      let setImage: ((v: any) => Promise<any>) | undefined;
      let setHomepage: ((v: any) => Promise<any>) | undefined;
      let setLicense: ((v: any) => Promise<any>) | undefined;
      let client: KitStoreClient | null = null;

      function Probe() {
        setName = useKitName()[1];
        setRelease = useKitRelease()[1];
        setDescription = useKitDescription()[1];
        setIcon = useKitIcon()[1];
        setImage = useKitImage()[1];
        setHomepage = useKitHomepage()[1];
        setLicense = useKitLicense()[1];
        client = useKitStoreClient();
        return null;
      }

      render(React.createElement(KitScope, { store, kitClient, children: React.createElement(Probe) }));
      await waitFor(() => {
        expect(setLicense).toBeDefined();
        expect(client).not.toBeNull();
      });

      expect((await setName!("Renamed Kit")).ok).toBe(true);
      expect((await setRelease!("1.2.3")).ok).toBe(true);
      expect((await setDescription!("Updated description")).ok).toBe(true);
      expect((await setIcon!("spark")).ok).toBe(true);
      expect((await setImage!("kit.png")).ok).toBe(true);
      expect((await setHomepage!("https://semio.example")).ok).toBe(true);
      expect((await setLicense!("LGPL-3.0-or-later")).ok).toBe(true);

      await waitFor(() => {
        const next = store.getSnapshot().kit.toJSON();
        expect(next.name).toBe("Renamed Kit");
        expect(next.version).toBe("1.2.3");
        expect(next.description).toBe("Updated description");
        expect(next.icon).toBe("spark");
        expect(next.image).toBe("kit.png");
        expect(next.homepage).toBe("https://semio.example");
        expect(next.license).toBe("LGPL-3.0-or-later");
      });
    });

    it("usePieceFlatPlane subscribes narrowly: FlattenInvalidated for one piece rerenders only that hook", async () => {
      const kit = asKitInstance({
        id: "k-gran",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const listeners = new Set<(ev: import("@semio/js").KitEvent) => void>();
      const mockKs = {
        piece(d: string, p: string, _scope: unknown) {
          void _scope;
          return {
            readFlatPlane: async () => ({ tag: `${d}:${p}`, origin: [0, 0, 0] as const }),
          };
        },
      };
      const kitClient = createTestKitClient(store) as KitStoreClient & { internalKs?: () => unknown };
      kitClient.internalKs = () => mockKs as unknown as import("@semio/js").KitStore;
      kitClient.subscribe = (cb: (ev: import("@semio/js").KitEvent) => void) => {
        listeners.add(cb);
        return () => {
          listeners.delete(cb);
        };
      };

      const renders = { p1: 0, p2: 0 };

      function Piece1() {
        usePieceFlatPlane("d1", "p1");
        renders.p1 += 1;
        return null;
      }
      function Piece2() {
        usePieceFlatPlane("d1", "p2");
        renders.p2 += 1;
        return null;
      }

      render(
        React.createElement(
          KitScope,
          { store, kitClient, children: React.createElement(React.Fragment, null, React.createElement(Piece1), React.createElement(Piece2)) },
        ),
      );

      await waitFor(() => {
        expect(renders.p1).toBeGreaterThan(0);
        expect(renders.p2).toBeGreaterThan(0);
      });

      const afterIdle = { p1: renders.p1, p2: renders.p2 };

      await act(async () => {
        const ev = { FlattenInvalidated: { design: "d1", pieces: ["p1"] } } as import("@semio/js").KitEvent;
        for (const l of [...listeners]) l(ev);
      });

      await waitFor(() => {
        expect(renders.p1).toBeGreaterThan(afterIdle.p1);
        expect(renders.p2).toBe(afterIdle.p2);
      });
    });
  });

  describe("KitRegistry + useOptimistic", () => {
    it("registry open/close refcounts and useOptimistic keeps draft until commit", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let reg: ReturnType<typeof useKitRegistry> | null = null;
      function RegProbe() {
        reg = useKitRegistry();
        return null;
      }
      render(React.createElement(KitRegistryProvider, null, React.createElement(RegProbe)));
      await waitFor(() => expect(reg).not.toBeNull());
      await reg!.open("k1", { store, kitClient });
      expect(reg!.get("k1")?.refs).toBe(1);
      await reg!.open("k1", { store });
      expect(reg!.get("k1")?.refs).toBe(2);
      reg!.close("k1");
      expect(reg!.get("k1")?.refs).toBe(1);
      reg!.close("k1");
      expect(reg!.get("k1")).toBeUndefined();

      const triad: HookTriad<string> = ["hello", async () => ({ ok: true }) as const, { kind: "idle", pending: 0 }];
      let opt: ReturnType<typeof useOptimistic<string>> | null = null;
      function OptProbe() {
        opt = useOptimistic(triad);
        return null;
      }
      render(React.createElement(OptProbe));
      await waitFor(() => expect(opt).not.toBeNull());
      expect(opt!.dirty).toBe(false);
    });
  });

  describe("getKitRegistryBridge", () => {
    it("is non-null under KitRegistryProvider and null after unmount", async () => {
      const { unmount } = render(React.createElement(KitRegistryProvider, { children: React.createElement("div", null, "x") }));
      const b = getKitRegistryBridge();
      expect(b).not.toBeNull();
      expect(typeof b!.list).toBe("function");
      unmount();
      await waitFor(() => expect(getKitRegistryBridge()).toBeNull());
    });
  });

  describe("useOpenKitGuids + useActiveKitGuid", () => {
    it("mirrors registry list() and activeKitId after open", async () => {
      const kit = asKitInstance({
        id: "k-open",
        name: "OpenK",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let openIds: string[] = [];
      let active: string | undefined;
      function Probe() {
        openIds = useOpenKitGuids();
        active = useActiveKitGuid();
        return null;
      }
      render(React.createElement(KitRegistryProvider, null, React.createElement(Probe)));
      const b = getKitRegistryBridge();
      expect(b).not.toBeNull();
      await b!.open("k-open", { store, kitClient });
      b!.setActiveKit("k-open");
      await waitFor(() => {
        expect(openIds).toContain("k-open");
        expect(active).toBe("k-open");
      });
    });
  });

  describe("useOpenKitShallows + useRegistryHasKit + useRegistryKitPersistenceKind", () => {
    it("returns empty shallows when no KitRegistryProvider (Home table shell)", () => {
      cleanup();
      let shallows: Kit[] = [];
      function Probe() {
        shallows = useOpenKitShallows();
        return null;
      }
      render(React.createElement(Probe));
      expect(shallows).toEqual([]);
    });

    it("reflects registry kit snapshots and persistence kind", async () => {
      const kit = asKitInstance({
        id: "k-shallow",
        name: "ShallowK",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let shallows: Kit[] = [];
      let hasKit = false;
      let pkind: KitPersistenceInfo["kind"] | undefined;
      function Probe() {
        shallows = useOpenKitShallows();
        hasKit = useRegistryHasKit("k-shallow");
        pkind = useRegistryKitPersistenceKind("k-shallow");
        return null;
      }
      const { unmount } = render(React.createElement(KitRegistryProvider, null, React.createElement(Probe)));
      const b = getKitRegistryBridge();
      expect(b).not.toBeNull();
      await b!.open("k-shallow", { store, kitClient });
      await waitFor(() => expect(b!.list()).toContain("k-shallow"));
      await waitFor(() => {
        expect(hasKit).toBe(true);
        expect(pkind).toBe("temporary");
        expect(shallows.some((s) => s.id === "k-shallow" && s.name === "ShallowK")).toBe(true);
      });
      unmount();
      await waitFor(() => expect(getKitRegistryBridge()).toBeNull());
    });
  });

  describe("KitStoreClient stub RPC hooks", () => {
    it("records kit command request lifecycle events from the store client", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [{ id: "d1", name: "D", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString(), pieces: [{ id: "p1", name: "P" }], connections: [] }],
      });
      const store = new InMemoryKitStore(kit);
      const listeners = new Set<(event: any) => void>();
      const emit = (event: any) => {
        for (const listener of listeners) listener(event);
      };
      const stub = {
        ...createTestKitClient(store),
        subscribe: (cb: (ev: any) => void) => {
          listeners.add(cb);
          return () => listeners.delete(cb);
        },
        clusterPieces: async () => {
          await new Promise((resolve) => setTimeout(resolve, 0));
          emit({ semioKitCommand: { requestId: "r1", commandKind: "clusterPieces", phase: "accepted" } });
          emit({ semioKitCommand: { requestId: "r1", commandKind: "clusterPieces", phase: "failed", error: { kind: "InvalidValue", message: "bad cluster" } } });
          return { ok: false, error: { kind: "InvalidValue", message: "bad cluster" }, requestId: "r1" };
        },
      } as unknown as KitStoreClient;
      let events: SchemaPropertyEvent[] = [];
      let errors: SetError[] = [];
      function Probe() {
        const { run } = useClusterPieces();
        events = useSchemaEvents({ typeName: "KitCommand" });
        errors = useSetErrors();
        const ran = React.useRef(false);
        React.useEffect(() => {
          if (ran.current) return;
          ran.current = true;
          void run("d1", ["p1"], "C");
        }, [run]);
        return null;
      }
      render(React.createElement(KitScope, { store, kitClient: stub, children: React.createElement(Probe) }));
      await waitFor(() => expect(events.some((event) => event.requestId === "r1" && event.phase === "failed")).toBe(true));
      expect(errors.some((error) => error.message === "bad cluster")).toBe(true);
    });

    it("useClusterPieces forwards failures to useSetErrors", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [
          {
            id: "d1",
            name: "D",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            pieces: [{ id: "p1", name: "P" }],
            connections: [],
          },
        ],
      });
      const store = new InMemoryKitStore(kit);
      const stub: KitStoreClient = {
        fetchFullKit: async () => store.getSnapshot().kit.toJSON() as KitFullDto,
        kitReadScope: theKitReadScope,
        getKitWriteScope: () => null,
        setKitWriteScope: () => {},
        finalizeKitWriteTransaction: async () => ({ ok: true }) as const,
        abortKitWriteTransaction: async () => ({ ok: true }) as const,
        submitChangeKitCommands: async () => ({ ok: true }) as const,
        kitGraphql: () => {
          throw new Error("no gql");
        },
        clusterPieces: async () => ({ ok: false, error: { kind: "InvalidValue", message: "stub-cluster" } }),
        dragPieces: async () => ({ ok: true }) as const,
        movePieces: async () => ({ ok: true }) as const,
        fixPieces: async () => ({ ok: true }) as const,
        flattenDesign: async () => ({ ok: true }) as const,
        expandDesign: async () => ({ ok: true }) as const,
        deleteConnection: async () => ({ ok: true }) as const,
        changePieceType: async () => ({ ok: true }) as const,
        pasteDesignSelection: async () => ({ ok: true }) as const,
        createHangingPieces: async () => ({ ok: true }) as const,
        createConnectedPiece: async () => ({ ok: true }) as const,
        createFixedPiece: async () => ({ ok: true }) as const,
        getPiecesMetadata: async () => new Map(),
        getPieces: async () => [],
        getConnections: async () => [],
        getDesigns: async () => [],
        getTypes: async () => [],
        getAuthors: async () => [],
        getKitMetadata: async () => {
          const k = store.getSnapshot().kit.toJSON() as KitFullDto;
          return { id: String(k.id ?? ""), name: String(k.name ?? "") };
        },
        undo: async () => ({ ok: true }) as const,
        redo: async () => ({ ok: true }) as const,
        canUndo: async () => false,
        canRedo: async () => false,
        backboneStatus: async () => ({ attached: false, kind: null, backboneTip: null, pendingWipCheckpoints: 0 }),
        attachBackbone: async () => ({ ok: true } as const),
        detachBackbone: async () => ({ ok: true } as const),
        listConflicts: async () => [],
        resolveConflict: async () => ({ ok: true } as const),
        syncNow: async () => ({ ok: true } as const),
        subscribeKitName: () => () => {},
        getKitNameSnapshot: () => "",
        subscribeRenameStatus: () => () => {},
        getRenameStatusSnapshot: () => KIT_RENAME_STATUS_IDLE,
        rename: async () => ({ ok: false, requestId: "", error: { kind: "NotSupported", message: "stub" } }),
        readPieceFlatPlane: async () => null,
        readPieceFlatCenter: async () => null,
        readPieceParentConnectionFull: async () => null,
        readDesignIncludedDesigns: async () => [],
        readDesignClusterableGroups: async () => [],
        readDesignQualitySum: async () => 0,
        readTypeBestRepresentation: async () => null,
        readColoredConnectors: async () => [],
        readDesignReplaceableCatalogTypes: async () => [],
        readDesignReplaceableCatalogDesigns: async () => [],
        readDesignIncludedDesignIds: async () => [],
        subscribe: () => () => {},
        setKitReadScope: () => {},
        dispose: () => {},
      } as KitStoreClient;
      let seen: SetError[] = [];
      function Probe() {
        const { run } = useClusterPieces();
        seen = useSetErrors();
        const ran = React.useRef(false);
        React.useEffect(() => {
          if (ran.current) return;
          ran.current = true;
          void run("d1", ["p1"], "C");
        }, [run]);
        return null;
      }
      render(React.createElement(KitScope, { store, kitClient: stub, children: React.createElement(Probe) }));
      await waitFor(() => expect(seen.length).toBeGreaterThan(0));
      expect(seen[0]?.message).toContain("stub-cluster");
    });
  });

  describe("kit data scope", () => {
    it("KitScope kitReadScope prop drives setKitReadScope and useKitDataScope (checkpoint line)", async () => {
      const log: string[] = [];
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        types: [],
        designs: [],
      });
      const store = new InMemoryKitStore(kit);
      const base = createTestKitClient(store);
      const client: KitStoreClient = {
        ...base,
        setKitReadScope: (s) => {
          log.push(kitReadScopeKey(s));
        },
      };
      const ck: KitReadScope = { checkpoint: { checkpointId: "cpx" } };
      let got: KitReadScope | null = null;
      function Leaf() {
        got = useKitDataScope();
        return null;
      }
      const tree = React.createElement(KitScope, {
        store,
        kitClient: client,
        kitReadScope: ck,
        children: React.createElement(Leaf, null),
      });
      const { unmount } = render(tree);
      await waitFor(() => {
        if (!got || !("checkpoint" in got) || (got as { checkpoint: { checkpointId: string } }).checkpoint.checkpointId !== "cpx") {
          throw new Error("not ready");
        }
      });
      const ckKey = kitReadScopeKey({ checkpoint: { checkpointId: "cpx" } });
      expect(log).toContain(ckKey);
      unmount();
    });

    it("KitScope without kitReadScope follows KitAlternativeSelectionProvider alternative id", async () => {
      const log: string[] = [];
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        types: [],
        designs: [],
      });
      const store = new InMemoryKitStore(kit);
      const base = createTestKitClient(store);
      const client: KitStoreClient = {
        ...base,
        setKitReadScope: (s) => {
          log.push(kitReadScopeKey(s));
        },
      };
      function Probe() {
        const [, setAlt] = useKitAlternativeSelection();
        React.useEffect(() => {
          setAlt("alt-7");
        }, [setAlt]);
        useKitDataScope();
        return null;
      }
      const tree = React.createElement(
        KitAlternativeSelectionProvider,
        {},
        React.createElement(KitScope, {
          store,
          kitClient: client,
          children: React.createElement(Probe, null),
        }),
      );
      const { unmount } = render(tree);
      await waitFor(() => {
        expect(log).toContain(kitReadScopeKey({ alternative: { alternativeId: "alt-7" } }));
      });
      unmount();
    });
  });

  describe("useDraft", () => {
    it("keeps local draft and does not clear it when commit rejects", async () => {
      const triad: HookTriad<string> = [
        "server",
        async (next) => {
          const v = typeof next === "function" ? (next as (p: string) => string)("server") : next;
          if (v === "reject") return { ok: false, error: { kind: "InvalidValue", message: "rejected" } } as const;
          return { ok: true } as const;
        },
        { kind: "idle", pending: 0 },
      ];
      let snap: ReturnType<typeof useDraft<string>> | null = null;
      function P() {
        snap = useDraft(triad);
        return null;
      }
      render(React.createElement(P));
      await waitFor(() => expect(snap).not.toBeNull());
      await act(async () => {
        snap!.setDraft("reject");
      });
      const r = await act(async () => snap!.commit());
      expect(r.ok).toBe(false);
      expect(snap!.value).toBe("reject");
    });

    it("clears draft when commit succeeds", async () => {
      const triad: HookTriad<string> = [
        "server",
        async (next) => {
          const v = typeof next === "function" ? (next as (p: string) => string)("server") : next;
          return { ok: true } as const;
        },
        { kind: "idle", pending: 0 },
      ];
      let snap: ReturnType<typeof useDraft<string>> | null = null;
      function P() {
        snap = useDraft(triad);
        return null;
      }
      render(React.createElement(P));
      await waitFor(() => expect(snap).not.toBeNull());
      await act(async () => {
        snap!.setDraft("edited");
      });
      expect(snap!.value).toBe("edited");
      const r = await act(async () => snap!.commit());
      expect(r.ok).toBe(true);
      expect(snap!.value).toBe("server");
    });

    it("two useDraft instances do not share draft state", async () => {
      const triadA: HookTriad<string> = ["a", async () => ({ ok: true }) as const, { kind: "idle", pending: 0 }];
      const triadB: HookTriad<string> = ["b", async () => ({ ok: true }) as const, { kind: "idle", pending: 0 }];
      let sa: ReturnType<typeof useDraft<string>> | null = null;
      let sb: ReturnType<typeof useDraft<string>> | null = null;
      function P() {
        sa = useDraft(triadA);
        sb = useDraft(triadB);
        return null;
      }
      render(React.createElement(P));
      await waitFor(() => expect(sa && sb).toBeTruthy());
      await act(async () => {
        sa!.setDraft("only-a");
        sb!.setDraft("only-b");
      });
      expect(sa!.value).toBe("only-a");
      expect(sb!.value).toBe("only-b");
    });
  });
}
// #endregion ⚛️Embedded tests
