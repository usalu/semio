/** 📝️ Text representation for `lowpoly.lowpoly.mutations`. */
export type LowpolyMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyMutationsTextGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyMutationsTextGuardRefusal(at, why);
};

type lowpolyLowpolyMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyMutationsTextGuardReject(at, "value is not an object");
export const lowpolyLowpolyMutationsTextGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyMutationsTextGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyMutationsTextGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyMutationsTextGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyMutationsTextGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyMutationsTextGuardNumber(value, at, bounds) : lowpolyLowpolyMutationsTextGuardReject(at, "value is not an integer");
export const lowpolyLowpolyMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyMutationsText(value: unknown, at = "$"): LowpolyMutationsText {
  return lowpolyLowpolyMutationsTextGuardObject(value, `${at}`);
}
