// #region Header
// semio/algorithms/nativeAlgorithmAdapter.ts
// Specs: Route algorithm work to in-browser TypeScript or to the engine REST native-algorithms endpoint by language. Output designs always have the operation diff fully applied (no withDiff overlay, no connection preservation).
// Summary: Single adapter: @semio/js for ts, POST /api/native-algorithms/execute for python, go, rust. All flat/output designs are produced by applying the full forward diff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion Header

import type { Coord, Design, DesignDiff, DesignDiffOperationResult, DesignOperationResult, FlatMerkleCacheEntry, Kit, MoveVector, OperationResult } from "@semio/js";
import { applyDesignDiff, normalizeDesignCopyResult, normalizeDesignDiffResult, normalizeDesignFlattenResult } from "@semio/js";

// #region 🧠Flatten Merkle Cache (TS path only)
// Per-designGuid cache reused across nativeFlattenDesign / nativeDragPieces / nativeMovePieces (all TS path)
// so consecutive flatten calls only redo matrix math for pieces whose merkle inputs actually changed.
// Native (python/rust/go/csharp) paths don't share this cache — each REST call is stateless.
const flatMerkleCacheByDesign: Map<string, { [pieceGuid: string]: FlatMerkleCacheEntry }> = new Map();
const getFlatMerkleCache = (designGuid: string): { [pieceGuid: string]: FlatMerkleCacheEntry } | undefined => flatMerkleCacheByDesign.get(designGuid);
const setFlatMerkleCache = (designGuid: string, cache: { [pieceGuid: string]: FlatMerkleCacheEntry }): void => {
  flatMerkleCacheByDesign.set(designGuid, cache);
};
// #endregion 🧠Flatten Merkle Cache (TS path only)

/** Language toolbar values; MUST stay aligned with `.storybook/withLanguage` AlgorithmLanguage. */
export type NativeAlgorithmLanguage = "ts" | "python" | "rust" | "go" | "csharp";

export type NativeAlgorithmOperation = "flatten" | "delete" | "drag" | "copy" | "paste";

export interface NativeAlgorithmExecutePayload {
  readonly operation: NativeAlgorithmOperation;
  readonly kit: Kit;
  readonly design: Design;
  readonly designGuid: string;
  readonly pieceGuids: readonly string[];
  readonly connectionGuids: readonly string[];
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

/**
 * POST body for semio engine POST /api/native-algorithms/execute.
 */
interface NativeAlgorithmRestRequestBody {
  language: Exclude<NativeAlgorithmLanguage, "ts">;
  operation: NativeAlgorithmOperation;
  kit: Kit;
  design: Design;
  designGuid: string;
  pieceGuids: string[];
  connectionGuids: string[];
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

/**
 * Runs flatten in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeFlattenDesign(kit: Kit, designGuid: string, language: NativeAlgorithmLanguage): Promise<DesignOperationResult> {
  if (language === "ts") {
    const { flattenDesignCached } = await import("@semio/js");
    const { result, cache } = flattenDesignCached(kit, designGuid, getFlatMerkleCache(designGuid));
    setFlatMerkleCache(designGuid, cache);
    return result;
  }
  const design = (kit.designs ?? []).find((d) => d.guid === designGuid);
  if (!design) {
    return { ok: false, errors: [{ code: "native-flatten.design-not-found", message: `nativeFlattenDesign: design ${designGuid} not found in kit` }] };
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "flatten",
    kit,
    design,
    designGuid,
    pieceGuids: [],
    connectionGuids: [],
  });
  return normalizeDesignFlattenResult(raw);
}

/**
 * Runs delete-pieces in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeDeletePieces(kit: Kit, design: Design, pieceGuids: readonly string[], connectionGuids: readonly string[], language: NativeAlgorithmLanguage): Promise<DesignDiffOperationResult> {
  if (language === "ts") {
    const { deletePiecesAndConnectionsInDesign } = await import("@semio/js");
    return deletePiecesAndConnectionsInDesign(kit, design, [...pieceGuids], [...connectionGuids]);
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "delete",
    kit,
    design,
    designGuid: design.guid,
    pieceGuids: [...pieceGuids],
    connectionGuids: [...connectionGuids],
  });
  return normalizeDesignDiffResult(raw);
}

/**
 * Returns the flat design used as a display base for input and diff windows.
 * Applies only the piece updates from the flatten diff while keeping the original connections,
 * so the diagram can render the connections that the diff is about to remove.
 * For an output window, use {@link nativeFlattenedDesign} so the full diff is applied.
 */
export async function nativeFlatDesign(kit: Kit, designGuid: string, language: NativeAlgorithmLanguage): Promise<Design | null> {
  const result = await nativeFlattenDesign(kit, designGuid, language);
  if (!result.ok) return null;
  const design = (kit.designs ?? []).find((d) => d.guid === designGuid);
  if (!design) return null;
  return applyDesignDiff(JSON.parse(JSON.stringify(design)), { pieces: result.change.forward.pieces });
}

/**
 * Returns the flat design produced by fully applying the flatten forward diff.
 * The flatten diff removes all connections (they are absorbed into piece planes/centers),
 * so the returned design has no connections. Use this for output windows where the rule
 * is "diff fully applied, not withDiff overlay".
 */
export async function nativeFlattenedDesign(kit: Kit, designGuid: string, language: NativeAlgorithmLanguage): Promise<Design | null> {
  const result = await nativeFlattenDesign(kit, designGuid, language);
  if (!result.ok) return null;
  const design = (kit.designs ?? []).find((d) => d.guid === designGuid);
  if (!design) return null;
  return applyDesignDiff(JSON.parse(JSON.stringify(design)), result.change.forward);
}

/**
 * Runs drag in-process: flattens, applies {@link dragPiecesInDesign}, re-flattens, and returns
 * the flat input (pre-drag), the flat output (post-drag), and the drag diff. Drag's diff only
 * updates piece centers, so both flat designs keep their connections for display by applying
 * only the piece updates from the (re-)flatten diff.
 */
export async function nativeDragPieces(kit: Kit, rawDesign: Design, pieceGuids: readonly string[], offset: Coord, _language: NativeAlgorithmLanguage): Promise<{ inputDesign: Design; output: Design; dragDiff: DesignDiff }> {
  const { dragPiecesInDesign, applyDesignDiff: apply, flattenDesignCached } = await import("@semio/js");
  const designGuid = rawDesign.guid;
  const preFlat = flattenDesignCached(kit, designGuid, getFlatMerkleCache(designGuid));
  if (!preFlat.result.ok) {
    throw new Error(preFlat.result.errors.map((e) => e.message).join("; "));
  }
  setFlatMerkleCache(designGuid, preFlat.cache);
  const flatDesign = apply(JSON.parse(JSON.stringify(rawDesign)), { pieces: preFlat.result.change.forward.pieces });
  const piecesDesign: Design = { guid: flatDesign.guid, name: flatDesign.name, pieces: (flatDesign.pieces ?? []).filter((p) => pieceGuids.includes(p.guid)) };
  const dragDiff = dragPiecesInDesign(flatDesign, piecesDesign, offset);
  const updatedRaw = apply(rawDesign, dragDiff);
  const updatedKit: Kit = { ...kit, designs: (kit.designs ?? []).map((d) => (d.guid === designGuid ? updatedRaw : d)) };
  const postFlat = flattenDesignCached(updatedKit, designGuid, preFlat.cache);
  if (!postFlat.result.ok) {
    throw new Error(postFlat.result.errors.map((e) => e.message).join("; "));
  }
  setFlatMerkleCache(designGuid, postFlat.cache);
  const output = apply(updatedRaw, { pieces: postFlat.result.change.forward.pieces });
  return { inputDesign: flatDesign, output, dragDiff };
}

/**
 * Runs move in-process: flattens, applies {@link movePiecesInDesign} (needs kit types for parent connector frames), re-flattens, and returns
 * the flat input (pre-move), the flat output (post-move), and the move diff. Move's diff only
 * updates piece planes/centers, so both flat designs keep their connections for display by
 * applying only the piece updates from the (re-)flatten diff.
 */
export async function nativeMovePieces(kit: Kit, rawDesign: Design, pieceGuids: readonly string[], vector: MoveVector, _language: NativeAlgorithmLanguage): Promise<{ inputDesign: Design; output: Design; moveDiff: DesignDiff }> {
  const { movePiecesInDesign, applyDesignDiff: apply, flattenDesignCached } = await import("@semio/js");
  const designGuid = rawDesign.guid;
  const preFlat = flattenDesignCached(kit, designGuid, getFlatMerkleCache(designGuid));
  if (!preFlat.result.ok) {
    throw new Error(preFlat.result.errors.map((e) => e.message).join("; "));
  }
  setFlatMerkleCache(designGuid, preFlat.cache);
  const flatDesign = apply(JSON.parse(JSON.stringify(rawDesign)), { pieces: preFlat.result.change.forward.pieces });
  const piecesDesign: Design = { guid: flatDesign.guid, name: flatDesign.name, pieces: (flatDesign.pieces ?? []).filter((p) => pieceGuids.includes(p.guid)) };
  const moveDiff = movePiecesInDesign(kit, flatDesign, piecesDesign, vector);
  const updatedRaw = apply(rawDesign, moveDiff);
  const updatedKit: Kit = { ...kit, designs: (kit.designs ?? []).map((d) => (d.guid === designGuid ? updatedRaw : d)) };
  const postFlat = flattenDesignCached(updatedKit, designGuid, preFlat.cache);
  if (!postFlat.result.ok) {
    throw new Error(postFlat.result.errors.map((e) => e.message).join("; "));
  }
  setFlatMerkleCache(designGuid, postFlat.cache);
  const output = apply(updatedRaw, { pieces: postFlat.result.change.forward.pieces });
  return { inputDesign: flatDesign, output, moveDiff };
}

/**
 * Runs copy-design in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeCopyDesign(kit: Kit, design: Design, pieceGuids: readonly string[], connectionGuids: readonly string[], language: NativeAlgorithmLanguage): Promise<OperationResult<Design>> {
  if (language === "ts") {
    const { copyDesign } = await import("@semio/js");
    return copyDesign(kit, design, [...pieceGuids], [...connectionGuids]);
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "copy",
    kit,
    design,
    designGuid: design.guid,
    pieceGuids: [...pieceGuids],
    connectionGuids: [...connectionGuids],
  });
  return normalizeDesignCopyResult(raw);
}

/**
 * Runs paste-design in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativePasteDesign(kit: Kit, source: Design, target: Design, anchoring: string, coord: Coord | undefined, language: NativeAlgorithmLanguage): Promise<DesignDiff> {
  if (language === "ts") {
    const { pasteDesign } = await import("@semio/js");
    return pasteDesign(kit, source, target, anchoring, coord);
  }
  const raw = await postNativeAlgorithm({
    language,
    operation: "paste",
    kit,
    design: source,
    designGuid: target.guid,
    pieceGuids: [],
    connectionGuids: [],
  });
  return asDesignDiff(raw);
}
