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
/** 🎚️ Persisted schedule choice and mode of one exact BIM schedule window. */
export interface BimScheduleWindowConfig {
  schedule: string;
  editing: boolean;
}
/** 🧬️ The one `snapshot` mutation of BimScheduleWindowConfig: the whole configuration replaces the previous one. */
export type BimScheduleWindowConfigMutation = { kind: "snapshot"; config: BimScheduleWindowConfig };
/** 🚪️ Parses one exact BimScheduleWindowConfig. */
export function parseBimScheduleWindowConfig(value: unknown): BimScheduleWindowConfig {
  const row = exact(value, "$", ["schedule", "editing"]);
  return {
    schedule: text(row.schedule, "$.schedule"),
    editing: flag(row.editing, "$.editing"),
  };
}
/** 🔁️ Applies one exact BimScheduleWindowConfig mutation. */
export function applyBimScheduleWindowConfigMutation(_base: BimScheduleWindowConfig, mutation: BimScheduleWindowConfigMutation): BimScheduleWindowConfig {
  return parseBimScheduleWindowConfig(mutation.config);
}
