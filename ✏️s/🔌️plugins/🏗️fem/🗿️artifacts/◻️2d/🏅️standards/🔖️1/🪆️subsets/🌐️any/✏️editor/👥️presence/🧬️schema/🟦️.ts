/** 🧬️ Fem2dPresence — empty: selection is command-transient payload (not shareable state), and camera / result display already live on `Fem2dConfig` as local-ui. Mirrors `Fem2dPresence` in `../🦀️.rs`. */
export interface Fem2dPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class fem2dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const fem2dPresenceGuardReject = (at: string, why: string): never => {
  throw new fem2dPresenceGuardRefusal(at, why);
};

type fem2dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type fem2dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type fem2dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const fem2dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : fem2dPresenceGuardReject(at, "value is not an object");
export const fem2dPresenceGuardArray = (value: unknown, at: string, bounds: fem2dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return fem2dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) fem2dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) fem2dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const fem2dPresenceGuardString = (value: unknown, at: string, bounds: fem2dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return fem2dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) fem2dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) fem2dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) fem2dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const fem2dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : fem2dPresenceGuardReject(at, "value is not a boolean"));
export const fem2dPresenceGuardNumber = (value: unknown, at: string, bounds: fem2dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return fem2dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) fem2dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) fem2dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const fem2dPresenceGuardInteger = (value: unknown, at: string, bounds: fem2dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? fem2dPresenceGuardNumber(value, at, bounds) : fem2dPresenceGuardReject(at, "value is not an integer");
export const fem2dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : fem2dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const fem2dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : fem2dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dPresence(value: unknown, at = "$"): Fem2dPresence {
  return fem2dPresenceGuardObject(value, `${at}`);
}
