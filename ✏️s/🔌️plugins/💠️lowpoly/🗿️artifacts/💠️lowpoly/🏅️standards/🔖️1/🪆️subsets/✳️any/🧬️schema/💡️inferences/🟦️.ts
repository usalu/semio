/** 💡️ Lowpoly inference schema — object count + 3d bounding box across every object's transform
 * position. */

export interface LowpolyBounds {
  min: [number, number, number];
  max: [number, number, number];
}

export interface LowpolyInference {
  /** @derived */
  objectCount: number;
  /** @derived */
  bounds: LowpolyBounds | null;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyInferenceGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyInferenceGuardRefusal(at, why);
};

type lowpolyLowpolyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyInferenceGuardReject(at, "value is not an object");
export const lowpolyLowpolyInferenceGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyInferenceGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyInferenceGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyInferenceGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyInferenceGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyInferenceGuardNumber(value, at, bounds) : lowpolyLowpolyInferenceGuardReject(at, "value is not an integer");
export const lowpolyLowpolyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyBounds(value: unknown, at = "$"): LowpolyBounds {
  const row = lowpolyLowpolyInferenceGuardObject(value, at);
  return {
    min: lowpolyLowpolyInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: lowpolyLowpolyInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => lowpolyLowpolyInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}
