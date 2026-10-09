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
/** 🎚️ Persisted section or elevation view and navigation of one exact BIM section window. */
export interface BimSectionWindowConfig {
  view: string;
  framed: boolean;
  viewport: Viewport2d;
}
/** 🧬️ The one `replace` mutation of BimSectionWindowConfig: its payload is the window's whole configuration; its diff names only the fields that differ from the base. */
export type BimSectionWindowConfigMutation = { kind: "replace"; config: BimSectionWindowConfig };
/** 🔺️ Sparse diff of BimSectionWindowConfig: exactly the fields a mutation changes. */
export type BimSectionWindowConfigDiff = Partial<BimSectionWindowConfig>;
/** 🚪️ Parses one exact BimSectionWindowConfig. */
export function parseBimSectionWindowConfig(value: unknown): BimSectionWindowConfig {
  const row = exact(value, "$", ["view", "framed", "viewport"]);
  return {
    view: text(row.view, "$.view"),
    framed: flag(row.framed, "$.framed"),
    viewport: parseViewport2d(row.viewport),
  };
}
/** 🔺️ The sparse diff one exact BimSectionWindowConfig mutation produces over `base`. */
export function diffBimSectionWindowConfigMutation(base: BimSectionWindowConfig, mutation: BimSectionWindowConfigMutation): BimSectionWindowConfigDiff {
  const next = parseBimSectionWindowConfig(mutation.config);
  const diff: Record<string, unknown> = {};
  for (const key of Object.keys(next) as (keyof BimSectionWindowConfig)[]) if (JSON.stringify(next[key]) !== JSON.stringify(base[key])) diff[key] = next[key];
  return diff as BimSectionWindowConfigDiff;
}
/** 🔁️ Applies one exact BimSectionWindowConfig mutation through its sparse diff. */
export function applyBimSectionWindowConfigMutation(base: BimSectionWindowConfig, mutation: BimSectionWindowConfigMutation): BimSectionWindowConfig {
  return { ...base, ...diffBimSectionWindowConfigMutation(base, mutation) };
}
