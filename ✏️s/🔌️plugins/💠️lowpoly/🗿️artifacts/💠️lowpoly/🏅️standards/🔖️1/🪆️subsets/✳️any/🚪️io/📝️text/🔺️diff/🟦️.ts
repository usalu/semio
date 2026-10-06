/** 📝️ Text representation for `lowpoly.lowpoly.diff`. */
export type LowpolyDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyDiffTextGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyDiffTextGuardRefusal(at, why);
};

type lowpolyLowpolyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyDiffTextGuardReject(at, "value is not an object");
export const lowpolyLowpolyDiffTextGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyDiffTextGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyDiffTextGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyDiffTextGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyDiffTextGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyDiffTextGuardNumber(value, at, bounds) : lowpolyLowpolyDiffTextGuardReject(at, "value is not an integer");
export const lowpolyLowpolyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyDiffText(value: unknown, at = "$"): LowpolyDiffText {
  return lowpolyLowpolyDiffTextGuardObject(value, `${at}`);
}
