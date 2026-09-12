import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";
import { parseResultMode, type ResultMode } from "../../../../../../../../../../../../../⚙️engine/🖥️app-surface/🧬️schema/👁️result-mode/🟦️.ts";
/** 🧬️ Exact FEM 2D results window configuration. */
export interface Fem2dResultsWindowConfig {
  camera: Viewport2d;
  resultSourceId?: string | null;
  resultMode: ResultMode;
  resultModeIndex: number;
}
const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row) && key !== "resultSourceId") throw new TypeError(`${at}.${key} is required`);
  return row;
};
const text = (value: unknown, at: string): string => { if (typeof value !== "string") throw new TypeError(`${at} must be a string`); return value; };
const finite = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be finite`); return value; };
const natural = (value: unknown, at: string): number => { const result = finite(value, at); if (!Number.isInteger(result) || result < 0) throw new TypeError(`${at} must be a natural number`); return result; };
export function parseFem2dResultsWindowConfig(value: unknown): Fem2dResultsWindowConfig {
  const row = exact(value, "$", ["camera", "resultSourceId", "resultMode", "resultModeIndex"]);
  const parsed = { camera: parseViewport2d(row.camera) };
  const mode = parseResultMode(row.resultMode);
  const resultSourceId = row.resultSourceId === undefined || row.resultSourceId === null ? row.resultSourceId : text(row.resultSourceId, "$.resultSourceId");
  return { ...parsed, ...(resultSourceId === undefined ? {} : { resultSourceId }), resultMode: mode, resultModeIndex: natural(row.resultModeIndex, "$.resultModeIndex") };
}
