import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";
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
/** 🎚️ Persisted navigation and storey of one exact BIM plan window. */
export interface BimPlanWindowConfig {
  storey: string;
  cutHeight: number;
  framed: boolean;
  viewport: Viewport2d;
}
/** 🧬️ The one `snapshot` mutation of BimPlanWindowConfig: the whole configuration replaces the previous one. */
export type BimPlanWindowConfigMutation = { kind: "snapshot"; config: BimPlanWindowConfig };
/** 🚪️ Parses one exact BimPlanWindowConfig. */
export function parseBimPlanWindowConfig(value: unknown): BimPlanWindowConfig {
  const row = exact(value, "$", ["storey", "cutHeight", "framed", "viewport"]);
  return {
    storey: text(row.storey, "$.storey"),
    cutHeight: num(row.cutHeight, "$.cutHeight"),
    framed: flag(row.framed, "$.framed"),
    viewport: parseViewport2d(row.viewport),
  };
}
/** 🔁️ Applies one exact BimPlanWindowConfig mutation. */
export function applyBimPlanWindowConfigMutation(_base: BimPlanWindowConfig, mutation: BimPlanWindowConfigMutation): BimPlanWindowConfig {
  return parseBimPlanWindowConfig(mutation.config);
}
