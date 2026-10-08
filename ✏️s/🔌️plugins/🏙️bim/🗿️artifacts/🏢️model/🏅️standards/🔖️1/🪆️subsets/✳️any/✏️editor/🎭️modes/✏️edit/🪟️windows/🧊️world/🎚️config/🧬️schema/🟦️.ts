import { parseViewport3dOrbit, type Viewport3dOrbit } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🟦️.ts";
const exact = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError(`${at} must be an object`);
  const row = value as Record<string, unknown>;
  for (const key of Object.keys(row)) if (!keys.includes(key)) throw new TypeError(`${at}.${key} is unknown`);
  for (const key of keys) if (!(key in row)) throw new TypeError(`${at}.${key} is required`);
  return row;
};
const text = (value: unknown, at: string): string => { if (typeof value !== "string") throw new TypeError(`${at} must be a string`); return value; };
const num = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isFinite(value)) throw new TypeError(`${at} must be a finite number`); return value; };
const count = (value: unknown, at: string): number => { if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new TypeError(`${at} must be a non-negative safe integer`); return value; };
const flag = (value: unknown, at: string): boolean => { if (typeof value !== "boolean") throw new TypeError(`${at} must be a boolean`); return value; };
const list = <T,>(value: unknown, at: string, item: (entry: unknown, at: string) => T): T[] => { if (!Array.isArray(value)) throw new TypeError(`${at} must be an array`); return value.map((entry, index) => item(entry, `${at}[${index}]`)); };
/** 🧬️ BimWorldProjection. */
export interface BimWorldProjection {
  kind: string;
  orthographicView: string;
  axonometricVariant: string;
  axonometricAngleA: number;
  axonometricAngleB: number;
  axonometricQuadrant: string;
  obliqueVariant: string;
  obliqueAngle: number;
  obliqueDepth: number;
  onePointAxis: string;
  fov: number;
  twoPointShift: number;
  curvilinearFov: number;
  curvilinearStrength: number;
  curvilinearMapping: string;
}
export function parseBimWorldProjection(value: unknown, at = "$"): BimWorldProjection {
  const row = exact(value, at, ["kind", "orthographicView", "axonometricVariant", "axonometricAngleA", "axonometricAngleB", "axonometricQuadrant", "obliqueVariant", "obliqueAngle", "obliqueDepth", "onePointAxis", "fov", "twoPointShift", "curvilinearFov", "curvilinearStrength", "curvilinearMapping"]);
  return {
    kind: text(row.kind, `${at}.kind`),
    orthographicView: text(row.orthographicView, `${at}.orthographicView`),
    axonometricVariant: text(row.axonometricVariant, `${at}.axonometricVariant`),
    axonometricAngleA: num(row.axonometricAngleA, `${at}.axonometricAngleA`),
    axonometricAngleB: num(row.axonometricAngleB, `${at}.axonometricAngleB`),
    axonometricQuadrant: text(row.axonometricQuadrant, `${at}.axonometricQuadrant`),
    obliqueVariant: text(row.obliqueVariant, `${at}.obliqueVariant`),
    obliqueAngle: num(row.obliqueAngle, `${at}.obliqueAngle`),
    obliqueDepth: num(row.obliqueDepth, `${at}.obliqueDepth`),
    onePointAxis: text(row.onePointAxis, `${at}.onePointAxis`),
    fov: num(row.fov, `${at}.fov`),
    twoPointShift: num(row.twoPointShift, `${at}.twoPointShift`),
    curvilinearFov: num(row.curvilinearFov, `${at}.curvilinearFov`),
    curvilinearStrength: num(row.curvilinearStrength, `${at}.curvilinearStrength`),
    curvilinearMapping: text(row.curvilinearMapping, `${at}.curvilinearMapping`),
  };
}
/** 🎚️ Persisted camera, projection, storey visibility and section plane of one exact BIM world window. */
export interface BimWorldWindowConfig {
  camera: Viewport3dOrbit;
  projection: BimWorldProjection;
  isolatedStorey: string;
  hiddenStoreys: string[];
  sectionEnabled: boolean;
  sectionAxis: string;
  sectionOffset: number;
  framed: boolean;
}
/** 🧬️ The one `replace` mutation of BimWorldWindowConfig: its payload is the window's whole configuration; its diff names only the fields that differ from the base. */
export type BimWorldWindowConfigMutation = { kind: "replace"; config: BimWorldWindowConfig };
/** 🔺️ Sparse diff of BimWorldWindowConfig: exactly the fields a mutation changes. */
export type BimWorldWindowConfigDiff = Partial<BimWorldWindowConfig>;
/** 🚪️ Parses one exact BimWorldWindowConfig. */
export function parseBimWorldWindowConfig(value: unknown): BimWorldWindowConfig {
  const row = exact(value, "$", ["camera", "projection", "isolatedStorey", "hiddenStoreys", "sectionEnabled", "sectionAxis", "sectionOffset", "framed"]);
  return {
    camera: parseViewport3dOrbit(row.camera),
    projection: parseBimWorldProjection(row.projection, "$.projection"),
    isolatedStorey: text(row.isolatedStorey, "$.isolatedStorey"),
    hiddenStoreys: list(row.hiddenStoreys, "$.hiddenStoreys", text),
    sectionEnabled: flag(row.sectionEnabled, "$.sectionEnabled"),
    sectionAxis: text(row.sectionAxis, "$.sectionAxis"),
    sectionOffset: num(row.sectionOffset, "$.sectionOffset"),
    framed: flag(row.framed, "$.framed"),
  };
}
/** 🔺️ The sparse diff one exact BimWorldWindowConfig mutation produces over `base`. */
export function diffBimWorldWindowConfigMutation(base: BimWorldWindowConfig, mutation: BimWorldWindowConfigMutation): BimWorldWindowConfigDiff {
  const next = parseBimWorldWindowConfig(mutation.config);
  const diff: Record<string, unknown> = {};
  for (const key of Object.keys(next) as (keyof BimWorldWindowConfig)[]) if (JSON.stringify(next[key]) !== JSON.stringify(base[key])) diff[key] = next[key];
  return diff as BimWorldWindowConfigDiff;
}
/** 🔁️ Applies one exact BimWorldWindowConfig mutation through its sparse diff. */
export function applyBimWorldWindowConfigMutation(base: BimWorldWindowConfig, mutation: BimWorldWindowConfigMutation): BimWorldWindowConfig {
  return { ...base, ...diffBimWorldWindowConfigMutation(base, mutation) };
}
