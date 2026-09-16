import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
/** 🧭️ Which handles the transform gumball of one exact FEM 3D model window draws. */
export interface Fem3dGumballConfig {
  moveAxes: boolean;
  movePlanes: boolean;
  rotate: boolean;
  scaleAxes: boolean;
  scaleUniform: boolean;
}
/** 🧬️ Exact FEM 3D model window configuration. */
export interface Fem3dModelWindowConfig {
  camera: Viewport3dOrbit;
  gumball: Fem3dGumballConfig;
}
const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row)) throw new TypeError(`${at}.${key} is required`);
  return row;
};
const flag = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`); return value; };
/** 🧭️ Admits one gumball handle set from an external value. */
export function parseFem3dGumballConfig(value: unknown): Fem3dGumballConfig {
  const row = exact(value, "$.gumball", ["moveAxes", "movePlanes", "rotate", "scaleAxes", "scaleUniform"]);
  return {
    moveAxes: flag(row.moveAxes, "$.gumball.moveAxes"),
    movePlanes: flag(row.movePlanes, "$.gumball.movePlanes"),
    rotate: flag(row.rotate, "$.gumball.rotate"),
    scaleAxes: flag(row.scaleAxes, "$.gumball.scaleAxes"),
    scaleUniform: flag(row.scaleUniform, "$.gumball.scaleUniform"),
  };
}
export function parseFem3dModelWindowConfig(value: unknown): Fem3dModelWindowConfig {
  const row = exact(value, "$", ["camera", "gumball"]);
  return { camera: parseViewport3dOrbit(row.camera), gumball: parseFem3dGumballConfig(row.gumball) };
}
