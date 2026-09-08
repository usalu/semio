/** 🧬️ Fem3dPresence — empty: selection is command-transient payload (not shareable state), and camera / result display already live on `Fem3dConfig` as local-ui. Mirrors `Fem3dPresence` in `../🦀️.rs`. */
export interface Fem3dPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class fem3dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const fem3dPresenceGuardReject = (at: string, why: string): never => {
  throw new fem3dPresenceGuardRefusal(at, why);
};

type fem3dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type fem3dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type fem3dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const fem3dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : fem3dPresenceGuardReject(at, "value is not an object");
export const fem3dPresenceGuardArray = (value: unknown, at: string, bounds: fem3dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return fem3dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) fem3dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) fem3dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const fem3dPresenceGuardString = (value: unknown, at: string, bounds: fem3dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return fem3dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) fem3dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) fem3dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) fem3dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const fem3dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : fem3dPresenceGuardReject(at, "value is not a boolean"));
export const fem3dPresenceGuardNumber = (value: unknown, at: string, bounds: fem3dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return fem3dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) fem3dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) fem3dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const fem3dPresenceGuardInteger = (value: unknown, at: string, bounds: fem3dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? fem3dPresenceGuardNumber(value, at, bounds) : fem3dPresenceGuardReject(at, "value is not an integer");
export const fem3dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : fem3dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const fem3dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : fem3dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dPresence(value: unknown, at = "$"): Fem3dPresence {
  return fem3dPresenceGuardObject(value, `${at}`);
}
