/** 🧬️ FormsPresence — empty: forms has no multi-user shareable live state yet; blueprint and try-wizard view state stays in `FormsConfig`. Mirrors `FormsPresence` in `../🦀️.rs`. */
export interface FormsPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class formsFormsPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const formsFormsPresenceGuardReject = (at: string, why: string): never => {
  throw new formsFormsPresenceGuardRefusal(at, why);
};

type formsFormsPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type formsFormsPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type formsFormsPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const formsFormsPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : formsFormsPresenceGuardReject(at, "value is not an object");
export const formsFormsPresenceGuardArray = (value: unknown, at: string, bounds: formsFormsPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return formsFormsPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) formsFormsPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) formsFormsPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const formsFormsPresenceGuardString = (value: unknown, at: string, bounds: formsFormsPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return formsFormsPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) formsFormsPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) formsFormsPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) formsFormsPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const formsFormsPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : formsFormsPresenceGuardReject(at, "value is not a boolean"));
export const formsFormsPresenceGuardNumber = (value: unknown, at: string, bounds: formsFormsPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return formsFormsPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) formsFormsPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) formsFormsPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const formsFormsPresenceGuardInteger = (value: unknown, at: string, bounds: formsFormsPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? formsFormsPresenceGuardNumber(value, at, bounds) : formsFormsPresenceGuardReject(at, "value is not an integer");
export const formsFormsPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : formsFormsPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const formsFormsPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : formsFormsPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFormsPresence(value: unknown, at = "$"): FormsPresence {
  return formsFormsPresenceGuardObject(value, `${at}`);
}
