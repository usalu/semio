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
/** 🎚️ Persisted sheet and navigation of one exact BIM sheet window. */
export interface BimSheetWindowConfig {
  sheet: string;
  framed: boolean;
  viewport: Viewport2d;
}
/** 🧬️ The one `replace` mutation of BimSheetWindowConfig: its payload is the window's whole configuration; its diff names only the fields that differ from the base. */
export type BimSheetWindowConfigMutation = { kind: "replace"; config: BimSheetWindowConfig };
/** 🔺️ Sparse diff of BimSheetWindowConfig: exactly the fields a mutation changes. */
export type BimSheetWindowConfigDiff = Partial<BimSheetWindowConfig>;
/** 🚪️ Parses one exact BimSheetWindowConfig. */
export function parseBimSheetWindowConfig(value: unknown): BimSheetWindowConfig {
  const row = exact(value, "$", ["sheet", "framed", "viewport"]);
  return {
    sheet: text(row.sheet, "$.sheet"),
    framed: flag(row.framed, "$.framed"),
    viewport: parseViewport2d(row.viewport),
  };
}
/** 🔺️ The sparse diff one exact BimSheetWindowConfig mutation produces over `base`. */
export function diffBimSheetWindowConfigMutation(base: BimSheetWindowConfig, mutation: BimSheetWindowConfigMutation): BimSheetWindowConfigDiff {
  const next = parseBimSheetWindowConfig(mutation.config);
  const diff: Record<string, unknown> = {};
  for (const key of Object.keys(next) as (keyof BimSheetWindowConfig)[]) if (JSON.stringify(next[key]) !== JSON.stringify(base[key])) diff[key] = next[key];
  return diff as BimSheetWindowConfigDiff;
}
/** 🔁️ Applies one exact BimSheetWindowConfig mutation through its sparse diff. */
export function applyBimSheetWindowConfigMutation(base: BimSheetWindowConfig, mutation: BimSheetWindowConfigMutation): BimSheetWindowConfig {
  return { ...base, ...diffBimSheetWindowConfigMutation(base, mutation) };
}
