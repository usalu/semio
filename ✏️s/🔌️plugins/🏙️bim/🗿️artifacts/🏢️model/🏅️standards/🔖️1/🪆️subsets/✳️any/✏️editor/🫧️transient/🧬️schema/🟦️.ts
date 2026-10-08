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
/** 🎚️ Ephemeral local interaction state of one exact BIM window. */
export interface BimWindowTransient {
  engagementInput: string;
  pointerGeneration: number;
}
/** 🧬️ The one `set` mutation of BimWindowTransient: every field. */
export type BimWindowTransientMutation = { kind: "set" } & BimWindowTransient;
/** 🚪️ Parses one exact BimWindowTransient. */
export function parseBimWindowTransient(value: unknown): BimWindowTransient {
  const row = exact(value, "$", ["engagementInput", "pointerGeneration"]);
  return {
    engagementInput: text(row.engagementInput, "$.engagementInput"),
    pointerGeneration: count(row.pointerGeneration, "$.pointerGeneration"),
  };
}
/** 🔁️ Applies one exact BimWindowTransient mutation. */
export function applyBimWindowTransientMutation(_base: BimWindowTransient, mutation: BimWindowTransientMutation): BimWindowTransient {
  const { kind: _kind, ...fields } = mutation;
  return parseBimWindowTransient(fields);
}
