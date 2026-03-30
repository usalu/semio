// #region 🔖Header
// 💻 semio/algorithms/nativeAlgorithmAdapter.ts
// Specs: Route algorithm work to in-browser TypeScript or to the engine REST native-algorithms endpoint by language.
// Summary: Single adapter: @semio/js for ts, POST /api/native-algorithms/execute for python, go, rust.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Coord, Design, DesignChange, DesignDiff, Kit } from "@semio/js";

/** Language toolbar values; MUST stay aligned with `.storybook/withLanguage` AlgorithmLanguage. */
export type NativeAlgorithmLanguage = "ts" | "python" | "rust" | "go";

export type NativeAlgorithmOperation = "flatten" | "delete" | "drag";

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
 * POST body for semio engine `POST /api/native-algorithms/execute`.
 * [👤semio📚algorithms💻nativealgorithmadapter🔖restpayload](repo://p/u/semio/b/l/algorithms/f/nativeAlgorithmAdapter.ts/s/Native/d/i/RestPayload)
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

function asDesignChange(value: unknown): DesignChange {
  return value as DesignChange;
}

function asDesignDiff(value: unknown): DesignDiff {
  return value as DesignDiff;
}

/**
 * Runs flatten in the chosen language: TypeScript in-process or native backends via REST.
 * [👤semio📚algorithms💻nativealgorithmadapter🛠️nativeflattendesign](repo://p/u/semio/b/l/algorithms/f/nativeAlgorithmAdapter.ts/s/Native/d/i/nativeFlattenDesign)
 */
export async function nativeFlattenDesign(kit: Kit, designGuid: string, language: NativeAlgorithmLanguage): Promise<DesignChange> {
  if (language === "ts") {
    const { flattenDesign } = await import("@semio/js");
    return flattenDesign(kit, designGuid);
  }
  const design = (kit.designs ?? []).find((d) => d.guid === designGuid);
  if (!design) {
    throw new Error(`nativeFlattenDesign: design ${designGuid} not found in kit`);
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
  return asDesignChange(raw);
}

/**
 * Runs delete-pieces in the chosen language: TypeScript in-process or native backends via REST.
 * [👤semio📚algorithms💻nativealgorithmadapter🛠️nativedeletepieces](repo://p/u/semio/b/l/algorithms/f/nativeAlgorithmAdapter.ts/s/Native/d/i/nativeDeletePieces)
 */
export async function nativeDeletePieces(kit: Kit, design: Design, pieceGuids: readonly string[], connectionGuids: readonly string[], language: NativeAlgorithmLanguage): Promise<DesignDiff> {
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
  return asDesignDiff(raw);
}

/**
 * Runs drag-pieces in TypeScript using dragPiecesInDesign from @semio/js.
 * Drag is a pure geometric operation that runs in-process regardless of language selection.
 * [👤semio📚algorithms💻nativealgorithmadapter🛠️nativedragpieces](repo://p/u/semio/b/l/algorithms/f/nativeAlgorithmAdapter.ts/s/Native/d/i/nativeDragPieces)
 */
export async function nativeDragPieces(design: Design, pieceGuids: readonly string[], offset: Coord, _language: NativeAlgorithmLanguage): Promise<DesignDiff> {
  const { dragPiecesInDesign } = await import("@semio/js");
  const piecesDesign: Design = { guid: design.guid, name: design.name, pieces: (design.pieces ?? []).filter((p) => pieceGuids.includes(p.guid)) };
  return dragPiecesInDesign(design, piecesDesign, offset);
}
