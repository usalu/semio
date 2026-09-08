/** 🧬️ EquationPresence — empty: graph edits are document mutations, and viewport / locale live in `EquationConfig`; no shareable live surface state yet. Mirrors `EquationPresence` in `../🦀️.rs`. */
export interface EquationPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class equationEquationPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const equationEquationPresenceGuardReject = (at: string, why: string): never => {
  throw new equationEquationPresenceGuardRefusal(at, why);
};

type equationEquationPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type equationEquationPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type equationEquationPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const equationEquationPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : equationEquationPresenceGuardReject(at, "value is not an object");
export const equationEquationPresenceGuardArray = (value: unknown, at: string, bounds: equationEquationPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return equationEquationPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) equationEquationPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) equationEquationPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const equationEquationPresenceGuardString = (value: unknown, at: string, bounds: equationEquationPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return equationEquationPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) equationEquationPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) equationEquationPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) equationEquationPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const equationEquationPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : equationEquationPresenceGuardReject(at, "value is not a boolean"));
export const equationEquationPresenceGuardNumber = (value: unknown, at: string, bounds: equationEquationPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return equationEquationPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) equationEquationPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) equationEquationPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const equationEquationPresenceGuardInteger = (value: unknown, at: string, bounds: equationEquationPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? equationEquationPresenceGuardNumber(value, at, bounds) : equationEquationPresenceGuardReject(at, "value is not an integer");
export const equationEquationPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : equationEquationPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const equationEquationPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : equationEquationPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEquationPresence(value: unknown, at = "$"): EquationPresence {
  return equationEquationPresenceGuardObject(value, `${at}`);
}
