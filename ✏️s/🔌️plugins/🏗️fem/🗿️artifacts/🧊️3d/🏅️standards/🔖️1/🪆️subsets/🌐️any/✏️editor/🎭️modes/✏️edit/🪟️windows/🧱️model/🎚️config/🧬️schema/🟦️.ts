import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
/** 🧬️ Exact FEM 3D model window configuration. */
export interface Fem3dModelWindowConfig {
  camera: Viewport3dOrbit;
}
const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row) && key !== "resultSourceId") throw new TypeError(`${at}.${key} is required`);
  return row;
};
export function parseFem3dModelWindowConfig(value: unknown): Fem3dModelWindowConfig {
  const row = exact(value, "$", ["camera"]);
  const parsed = { camera: parseViewport3dOrbit(row.camera) };
  return parsed;
}
