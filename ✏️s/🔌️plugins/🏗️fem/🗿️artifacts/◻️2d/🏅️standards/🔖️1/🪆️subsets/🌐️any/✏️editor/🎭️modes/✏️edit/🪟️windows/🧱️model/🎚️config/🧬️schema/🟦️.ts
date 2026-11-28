import { parseViewport2d, type Viewport2d } from "@semio-tech/framework/ui/viewport/2d/schema";
/** 🧭️ Which handles the transform gumball of one exact FEM 2D model window draws. */
export interface Fem2dGumballConfig {
  moveAxes: boolean;
  rotate: boolean;
  scaleAxes: boolean;
  scaleUniform: boolean;
}
/** 🧬️ Exact FEM 2D model window configuration. */
export interface Fem2dModelWindowConfig {
  camera: Viewport2d;
  gumball: Fem2dGumballConfig;
}
const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row) && key !== "resultSourceId") throw new TypeError(`${at}.${key} is required`);
  return row;
};
const flag = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`); return value; };
/** 🧭️ Admits one gumball handle set from an external value. */
export function parseFem2dGumballConfig(value: unknown): Fem2dGumballConfig {
  const row = exact(value, "$.gumball", ["moveAxes", "rotate", "scaleAxes", "scaleUniform"]);
  return {
    moveAxes: flag(row.moveAxes, "$.gumball.moveAxes"),
    rotate: flag(row.rotate, "$.gumball.rotate"),
    scaleAxes: flag(row.scaleAxes, "$.gumball.scaleAxes"),
    scaleUniform: flag(row.scaleUniform, "$.gumball.scaleUniform"),
  };
}
export function parseFem2dModelWindowConfig(value: unknown): Fem2dModelWindowConfig {
  const row = exact(value, "$", ["camera", "gumball"]);
  return { camera: parseViewport2d(row.camera), gumball: parseFem2dGumballConfig(row.gumball) };
}
