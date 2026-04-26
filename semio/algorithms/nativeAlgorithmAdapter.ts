// #region Header
// semio/algorithms/nativeAlgorithmAdapter.ts
// Specs: Route algorithm work to in-browser TypeScript (WASM `KitStore` + GraphQL reads) or to the engine REST native-algorithms endpoint by language. Output designs always apply the full forward diff for flattened views.
// Summary: TS path uses `@semio/js` `KitStore` (flatten_map, dragPieces, movePieces); domain types live in `@semio/react`. Non-TS languages POST to `/api/native-algorithms/execute`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion Header

/// <reference types="vite/client" />

import { KitStore, type KitFullDto as WasmKitFullDto } from "@semio/js";
import type {
  CoordinatePlain as Coordinate,
  Design,
  DesignDiff,
  DesignDiffOperationResult,
  DesignOperationResult,
  DesignPlain,
  FlatMerkleCacheEntry,
  Kit,
  MoveVector,
  OperationResult,
} from "@semio/react";
import {
  Design as DesignEntity,
  Kit as KitEntity,
  asKitInstance,
  normalizeDesignCopyResult,
  normalizeDesignDiffResult,
  normalizeDesignFlattenResult,
} from "@semio/react";

// #region 🧠Flatten Merkle Cache (TS path only)
// Legacy hook: WASM `DesignStore` owns flatten caches; this map is kept for API compatibility only.
const flatMerkleCacheByDesign: Map<string, { [pieceId: string]: FlatMerkleCacheEntry }> = new Map();
const getFlatMerkleCache = (designId: string): { [pieceId: string]: FlatMerkleCacheEntry } | undefined => flatMerkleCacheByDesign.get(designId);
const setFlatMerkleCache = (designId: string, cache: { [pieceId: string]: FlatMerkleCacheEntry }): void => {
  flatMerkleCacheByDesign.set(designId, cache);
};
// #endregion 🧠Flatten Merkle Cache (TS path only)

/** Language toolbar values; MUST stay aligned with `.storybook/withLanguage` AlgorithmLanguage. */
export type NativeAlgorithmLanguage = "ts" | "python" | "rust" | "go" | "csharp";

export type NativeAlgorithmOperation = "flatten" | "delete" | "drag" | "copy" | "paste";

export interface NativeAlgorithmExecutePayload {
  readonly operation: NativeAlgorithmOperation;
  readonly kit: Kit;
  readonly design: Design;
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly connectionIds: readonly string[];
}

function engineOrigin(): string {
  const fromVite = import.meta.env?.VITE_SEMIO_NATIVE_ALGORITHM_REST as string | undefined;
  const fromEngine = import.meta.env?.VITE_SEMIO_ENGINE_ORIGIN as string | undefined;
  if (fromVite) return fromVite.replace(/\/$/, "");
  if (fromEngine) return fromEngine.replace(/\/$/, "");
  if (typeof globalThis !== "undefined" && typeof (globalThis as unknown as { location?: { port?: string } }).location?.port === "string") {
    const port = (globalThis as unknown as { location: { port: string } }).location.port;
    if (port === "6007" || port === "6006") {
      return "";
    }
  }
  return "http://127.0.0.1:2507";
}

interface NativeAlgorithmRestRequestBody {
  language: Exclude<NativeAlgorithmLanguage, "ts">;
  operation: NativeAlgorithmOperation;
  kit: Kit;
  design: Design;
  designId: string;
  pieceIds: string[];
  connectionIds: string[];
}

interface NativeAlgorithmRestResponseBody {
  result?: unknown;
  error?: string;
}

async function postNativeAlgorithm(body: NativeAlgorithmRestRequestBody): Promise<unknown> {
  const origin = engineOrigin();
  const url = `${origin}/api/native-algorithms/execute`;
  const res = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  const json = (await res.json()) as NativeAlgorithmRestResponseBody;
  if (!res.ok) {
    throw new Error(json.error ?? `Native algorithm HTTP ${res.status}`);
  }
  if (json.error) {
    throw new Error(json.error);
  }
  return json.result;
}

function asDesignDiff(value: unknown): DesignDiff {
  return value as DesignDiff;
}

function cloneDesignWithDiff(kit: Kit, base: Design, diff: DesignDiff): Design {
  void kit;
  const plain = (base as DesignEntity).toPlain?.() ?? (JSON.parse(JSON.stringify(base)) as DesignPlain);
  const d = new DesignEntity(plain);
  d.applyDiff(diff);
  return d;
}

async function openWasmKit(kit: Kit): Promise<KitStore> {
  const dto = JSON.parse(JSON.stringify(asKitInstance(kit).toJSON())) as WasmKitFullDto;
  return KitStore.open(dto);
}

type FlattenRow = { pieceId: string; plane: unknown; center: unknown };

async function gqlFlattenMap(ks: KitStore, designId: string): Promise<readonly FlattenRow[]> {
  const rows = await ks.readDesignFlattenMap(designId);
  return rows as FlattenRow[];
}

function piecesDiffFromFlattenRows(rows: readonly FlattenRow[]): NonNullable<DesignDiff["pieces"]> {
  return {
    updated: rows.map((r) => ({
      piece: { id: r.pieceId },
      diff: { plane: r.plane as never, center: r.center as never },
    })),
  };
}

/**
 * Runs flatten in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeFlattenDesign(kit: Kit, designId: string, language: NativeAlgorithmLanguage): Promise<DesignOperationResult> {
  if (language === "ts") {
    void getFlatMerkleCache(designId);
    void setFlatMerkleCache;
    const design = asKitInstance(kit).designs?.find((d) => d.id === designId);
    if (!design) {
      return { ok: false, errors: [{ code: "native-flatten.design-not-found", message: `nativeFlattenDesign: design ${designId} not found in kit` }] };
    }
    let ks: KitStore | undefined;
    try {
      ks = await openWasmKit(kit);
      const rows = await gqlFlattenMap(ks, designId);
      const conns = design.connections ?? [];
      const forward: DesignDiff = {
        pieces: piecesDiffFromFlattenRows(rows),
        connections: conns.length ? { removed: conns.map((c) => ({ id: c.id })) } : undefined,
      };
      return { ok: true, design, diff: { forward, reverse: {} } };
    } catch (e) {
      return { ok: false, errors: [{ code: "native-flatten.wasm", message: String(e) }] };
    } finally {
      if (ks) await ks.dispose();
    }
  }
  const d = (kit.designs ?? []).find((x) => x.id === designId);
  if (!d) {
    return { ok: false, errors: [{ code: "native-flatten.design-not-found", message: `nativeFlattenDesign: design ${designId} not found in kit` }] };
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "flatten",
    kit,
    design: d,
    designId,
    pieceIds: [],
    connectionIds: [],
  });
  return normalizeDesignFlattenResult(raw);
}

/**
 * Runs delete-pieces in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeDeletePieces(kit: Kit, design: Design, pieceIds: readonly string[], connectionIds: readonly string[], language: NativeAlgorithmLanguage): Promise<DesignDiffOperationResult> {
  if (language === "ts") {
    const d = design instanceof DesignEntity ? design : new DesignEntity(design as DesignPlain);
    return d.deletePiecesAndConnectionsDiff([...pieceIds], [...connectionIds]);
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "delete",
    kit,
    design,
    designId: design.id,
    pieceIds: [...pieceIds],
    connectionIds: [...connectionIds],
  });
  return normalizeDesignDiffResult(raw);
}

/**
 * Returns the flat design used as a display base for input and diff windows.
 */
export async function nativeFlatDesign(kit: Kit, designId: string, language: NativeAlgorithmLanguage): Promise<Design | null> {
  const result = await nativeFlattenDesign(kit, designId, language);
  if (!result.ok) return null;
  const design = (kit.designs ?? []).find((d) => d.id === designId);
  if (!design) return null;
  return cloneDesignWithDiff(kit, design, { pieces: result.diff.forward.pieces });
}

/**
 * Returns the flat design produced by fully applying the flatten forward diff.
 */
export async function nativeFlattenedDesign(kit: Kit, designId: string, language: NativeAlgorithmLanguage): Promise<Design | null> {
  const result = await nativeFlattenDesign(kit, designId, language);
  if (!result.ok) return null;
  const design = (kit.designs ?? []).find((d) => d.id === designId);
  if (!design) return null;
  return cloneDesignWithDiff(kit, design, result.diff.forward);
}

/**
 * Runs drag in-process: WASM flatten map, UV drag on flat centers, WASM `dragPieces`, then flatten for output.
 */
export async function nativeDragPieces(kit: Kit, rawDesign: Design, pieceIds: readonly string[], offset: Coordinate, _language: NativeAlgorithmLanguage): Promise<{ inputDesign: Design; output: Design; dragDiff: DesignDiff }> {
  const designId = rawDesign.id;
  void getFlatMerkleCache(designId);
  let ks0: KitStore | undefined;
  let ks1: KitStore | undefined;
  try {
    ks0 = await openWasmKit(kit);
    const preRows = await gqlFlattenMap(ks0, designId);
    await ks0.dispose();
    ks0 = undefined;
    const prePieceDiff = piecesDiffFromFlattenRows(preRows);
    const flatDesign = cloneDesignWithDiff(kit, rawDesign, { pieces: prePieceDiff });
    const piecesDesign: Design = { id: flatDesign.id, name: flatDesign.name, pieces: (flatDesign.pieces ?? []).filter((p) => pieceIds.includes(p.id)) } as Design;
    const dragDiff = DesignEntity.prototype.dragBySelection.call(flatDesign, piecesDesign, offset);
    const updatedRaw = cloneDesignWithDiff(kit, rawDesign, dragDiff);
    const updatedKit: Kit = { ...asKitInstance(kit).toJSON(), designs: (kit.designs ?? []).map((d) => (d.id === designId ? updatedRaw : d)) } as Kit;
    ks1 = await openWasmKit(updatedKit);
    const postRows = await gqlFlattenMap(ks1, designId);
    const postPieceDiff = piecesDiffFromFlattenRows(postRows);
    const output = cloneDesignWithDiff(updatedKit, updatedRaw, { pieces: postPieceDiff });
    setFlatMerkleCache(designId, {});
    return { inputDesign: flatDesign, output, dragDiff };
  } finally {
    if (ks0) await ks0.dispose();
    if (ks1) await ks1.dispose();
  }
}

/**
 * Runs move in-process via WASM `movePieces` + flatten maps for display.
 */
export async function nativeMovePieces(kit: Kit, rawDesign: Design, pieceIds: readonly string[], vector: MoveVector, _language: NativeAlgorithmLanguage): Promise<{ inputDesign: Design; output: Design; moveDiff: DesignDiff }> {
  const designId = rawDesign.id;
  let ks: KitStore | undefined;
  try {
    ks = await openWasmKit(kit);
    const preRows = await gqlFlattenMap(ks, designId);
    const prePieceDiff = piecesDiffFromFlattenRows(preRows);
    const flatDesign = cloneDesignWithDiff(kit, rawDesign, { pieces: prePieceDiff });
    const r = await ks.movePieces(designId, [...pieceIds], vector.gap, vector.shift, vector.rise);
    if (!r.ok) {
      throw new Error(String((r as { ok: false; error?: { message?: string } }).error?.message ?? "movePieces failed"));
    }
    const postRows = await gqlFlattenMap(ks, designId);
    const moveDiff: DesignDiff = {
      pieces: {
        updated: pieceIds
          .map((id) => {
            const a = preRows.find((x) => x.pieceId === id)?.center as { u?: number; v?: number } | undefined;
            const b = postRows.find((x) => x.pieceId === id)?.center as { u?: number; v?: number } | undefined;
            if (a == null || b == null) return null;
            return { piece: { id }, diff: { center: { u: (b.u ?? 0) - (a.u ?? 0), v: (b.v ?? 0) - (a.v ?? 0) } } };
          })
          .filter((x): x is NonNullable<typeof x> => x != null),
      },
    };
    const updatedRaw = cloneDesignWithDiff(kit, rawDesign, moveDiff);
    const updatedKit: Kit = { ...asKitInstance(kit).toJSON(), designs: (kit.designs ?? []).map((d) => (d.id === designId ? updatedRaw : d)) } as Kit;
    const postPieceDiff = piecesDiffFromFlattenRows(postRows);
    const output = cloneDesignWithDiff(updatedKit, updatedRaw, { pieces: postPieceDiff });
    return { inputDesign: flatDesign, output, moveDiff };
  } finally {
    if (ks) await ks.dispose();
  }
}

/**
 * Runs copy-design in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeCopyDesign(kit: Kit, design: Design, pieceIds: readonly string[], connectionIds: readonly string[], language: NativeAlgorithmLanguage): Promise<OperationResult<Design>> {
  if (language === "ts") {
    return KitEntity.ensure(kit).copyDesignOp(design, [...pieceIds], [...connectionIds]);
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "copy",
    kit,
    design,
    designId: design.id,
    pieceIds: [...pieceIds],
    connectionIds: [...connectionIds],
  });
  return normalizeDesignCopyResult(raw);
}

/**
 * Runs paste-design in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativePasteDesign(kit: Kit, source: Design, target: Design, anchoring: string, coordinate: Coordinate | undefined, language: NativeAlgorithmLanguage): Promise<DesignDiff> {
  if (language === "ts") {
    return KitEntity.ensure(kit).pasteDesignOp(source, target, anchoring, coordinate);
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "paste",
    kit,
    design: source,
    designId: target.id,
    pieceIds: [],
    connectionIds: [],
  });
  return asDesignDiff(raw);
}
