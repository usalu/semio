/** 📝️ Text representation for `lowpoly.lowpoly.inference`. */
export type LowpolyInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyInferenceTextGuardRefusal(at, why);
};

type lowpolyLowpolyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyInferenceTextGuardReject(at, "value is not an object");
export const lowpolyLowpolyInferenceTextGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyInferenceTextGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyInferenceTextGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyInferenceTextGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyInferenceTextGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyInferenceTextGuardNumber(value, at, bounds) : lowpolyLowpolyInferenceTextGuardReject(at, "value is not an integer");
export const lowpolyLowpolyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyInferenceText(value: unknown, at = "$"): LowpolyInferenceText {
  return lowpolyLowpolyInferenceTextGuardObject(value, `${at}`);
}
