/** 💡️ Process3d inference schema — stockBounds (world-space AABB) + stepCount. */

export interface BoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Process3dInference {
  /** @derived */
  stockBounds: BoundingBox;
  /** @derived */
  stepCount: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dInferenceGuardReject = (at: string, why: string): never => {
  throw new processProcess3dInferenceGuardRefusal(at, why);
};

type processProcess3dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dInferenceGuardReject(at, "value is not an object");
export const processProcess3dInferenceGuardArray = (value: unknown, at: string, bounds: processProcess3dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dInferenceGuardString = (value: unknown, at: string, bounds: processProcess3dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dInferenceGuardReject(at, "value is not a boolean"));
export const processProcess3dInferenceGuardNumber = (value: unknown, at: string, bounds: processProcess3dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dInferenceGuardInteger = (value: unknown, at: string, bounds: processProcess3dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dInferenceGuardNumber(value, at, bounds) : processProcess3dInferenceGuardReject(at, "value is not an integer");
export const processProcess3dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dInference(value: unknown, at = "$"): Process3dInference {
  const row = processProcess3dInferenceGuardObject(value, at);
  return {
    stockBounds: parseBoundingBox(row["stockBounds"], `${at}.stockBounds`),
    stepCount: processProcess3dInferenceGuardInteger(row["stepCount"], `${at}.stepCount`, {"minimum": 0}),
  };
}

export function parseBoundingBox(value: unknown, at = "$"): BoundingBox {
  const row = processProcess3dInferenceGuardObject(value, at);
  return {
    min: processProcess3dInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => processProcess3dInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: processProcess3dInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => processProcess3dInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}
