import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";
/** 🧬️ Exact FEM 2D model window configuration. */
export interface Fem2dModelWindowConfig {
  camera: Viewport2d;
}
const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row) && key !== "resultSourceId") throw new TypeError(`${at}.${key} is required`);
  return row;
};
export function parseFem2dModelWindowConfig(value: unknown): Fem2dModelWindowConfig {
  const row = exact(value, "$", ["camera"]);
  const parsed = { camera: parseViewport2d(row.camera) };
  return parsed;
}
