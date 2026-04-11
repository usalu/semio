// #region Header
// semio/algorithms/nativeAlgorithmAdapter.ts
// Specs: Route algorithm work to in-browser TypeScript or to the engine REST native-algorithms endpoint by language.
// Summary: Single adapter: @semio/js for ts, POST /api/native-algorithms/execute for python, go, rust.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion Header

import type { Coord, Design, DesignDiff, DesignDiffOperationResult, DesignOperationResult, Kit, OperationResult } from "@semio/js";
import { normalizeDesignCopyResult, normalizeDesignDiffResult, normalizeDesignFlattenResult } from "@semio/js";

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
    const { flattenDesign } = await import("@semio/js");
    return flattenDesign(kit, designGuid);
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
export async function nativeDeletePieces(
  kit: Kit,
  design: Design,
  pieceGuids: readonly string[],
  connectionGuids: readonly string[],
  language: NativeAlgorithmLanguage,
): Promise<DesignDiffOperationResult> {
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
 * Runs drag in-process: flattens internally, applies {@link dragPiecesInDesign} on the flat view,
 * applies that diff to the raw design, re-flattens, and returns the final flat design plus the drag diff for UI preview.
 */
export async function nativeDragPieces(
  kit: Kit,
  rawDesign: Design,
  pieceGuids: readonly string[],
  offset: Coord,
  _language: NativeAlgorithmLanguage,
): Promise<{ output: Design; dragDiff: DesignDiff }> {
  const { dragPiecesInDesign, applyDesignDiff, flattenDesign } = await import("@semio/js");
  const fc = flattenDesign(kit, rawDesign.guid);
  if (!fc.ok) {
    throw new Error(fc.errors.map((e) => e.message).join("; "));
  }
  const flatDesign = applyDesignDiff(JSON.parse(JSON.stringify(rawDesign)), fc.change.forward);
  const piecesDesign: Design = { guid: flatDesign.guid, name: flatDesign.name, pieces: (flatDesign.pieces ?? []).filter((p) => pieceGuids.includes(p.guid)) };
  const dragDiff = dragPiecesInDesign(flatDesign, piecesDesign, offset);
  const updatedRaw = applyDesignDiff(rawDesign, dragDiff);
  const updatedKit: Kit = { ...kit, designs: (kit.designs ?? []).map((d) => (d.guid === rawDesign.guid ? updatedRaw : d)) };
  const flatChange = flattenDesign(updatedKit, rawDesign.guid);
  if (!flatChange.ok) {
    throw new Error(flatChange.errors.map((e) => e.message).join("; "));
  }
  const output = applyDesignDiff(updatedRaw, flatChange.change.forward);
  return { output, dragDiff };
}

/**
 * Runs copy-design in the chosen language: TypeScript in-process or native backends via REST.
 */
export async function nativeCopyDesign(
  kit: Kit,
  design: Design,
  pieceGuids: readonly string[],
  connectionGuids: readonly string[],
  language: NativeAlgorithmLanguage,
): Promise<OperationResult<Design>> {
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
