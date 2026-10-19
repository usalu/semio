import { decodeResultSourceIdentity } from "../../../../../../../../../../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🏷️result-source/🟦️.ts";
import { parseViewport3dOrbit, type Viewport3dOrbit } from "@semio-tech/framework/ui/viewport/3d/schema";
import { parseResultMode, type ResultMode } from "@semio-tech/fem-js/engine/app-surface/schema/result-mode";
import { parseFemResultsAnimation, type FemResultsAnimation } from "@semio-tech/fem-js/engine/app-surface/schema/results-animation";
/** 🧬️ Exact FEM 3D results window configuration. */
export interface Fem3dResultsWindowConfig {
  camera: Viewport3dOrbit;
  resultSourceId?: string | null;
  resultMode: ResultMode;
  resultModeIndex: number;
  animation: FemResultsAnimation;
}
const exact = (value: unknown, at: string, keys: readonly string[], optional: readonly string[] = []): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row) && !optional.includes(key)) throw new TypeError(`${at}.${key} is required`);
  return row;
};
const text = (value: unknown, at: string): string => { if (typeof value !== "string") throw new TypeError(`${at} must be a string`); return value; };
const finite = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be finite`); return value; };
const natural = (value: unknown, at: string): number => { const result = finite(value, at); if (!Number.isInteger(result) || result < 0) throw new TypeError(`${at} must be a natural number`); return result; };
export function parseFem3dResultsWindowConfig(value: unknown): Fem3dResultsWindowConfig {
  const row = exact(value, "$", ["camera", "resultSourceId", "resultMode", "resultModeIndex", "animation"], ["resultSourceId"]);
  const parsed = { camera: parseViewport3dOrbit(row.camera) };
  const mode = parseResultMode(row.resultMode);
  const resultSourceId = row.resultSourceId === undefined || row.resultSourceId === null ? row.resultSourceId : text(row.resultSourceId, "$.resultSourceId");
  if (resultSourceId !== undefined && resultSourceId !== null) {
    const source = decodeResultSourceIdentity(resultSourceId);
    if (mode === "buckling" && source.kind !== "loadCase") throw new TypeError("$.resultSourceId must name a load case for buckling");
  }
  return {
    ...parsed,
    ...(resultSourceId === undefined ? {} : { resultSourceId }),
    resultMode: mode,
    resultModeIndex: natural(row.resultModeIndex, "$.resultModeIndex"),
    animation: parseFemResultsAnimation(row.animation),
  };
}
