/** 📝️ Text representation for `lowpoly.lowpoly.snapshot`. */
export type LowpolySnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolySnapshotTextGuardRefusal(at, why);
};

type lowpolyLowpolySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolySnapshotTextGuardReject(at, "value is not an object");
export const lowpolyLowpolySnapshotTextGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolySnapshotTextGuardString = (value: unknown, at: string, bounds: lowpolyLowpolySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolySnapshotTextGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolySnapshotTextGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolySnapshotTextGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolySnapshotTextGuardNumber(value, at, bounds) : lowpolyLowpolySnapshotTextGuardReject(at, "value is not an integer");
export const lowpolyLowpolySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolySnapshotText(value: unknown, at = "$"): LowpolySnapshotText {
  return lowpolyLowpolySnapshotTextGuardObject(value, `${at}`);
}
