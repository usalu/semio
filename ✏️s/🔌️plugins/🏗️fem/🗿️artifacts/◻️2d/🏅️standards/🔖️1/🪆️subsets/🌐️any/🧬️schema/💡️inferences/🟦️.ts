/** 💡️ Fem2d inference schema — bounds (plan-view extent + node/element counts). */

export interface Fem2dBoundingBox {
  min: [number, number];
  max: [number, number];
}

export interface Fem2dBounds {
  boundingBox: Fem2dBoundingBox;
  nodeCount: number;
  elementCount: number;
}

export interface Fem2dInference {
  /** @derived */
  bounds: Fem2dBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dInferenceGuardReject = (at: string, why: string): never => {
  throw new femFem2dInferenceGuardRefusal(at, why);
};

type femFem2dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dInferenceGuardReject(at, "value is not an object");
export const femFem2dInferenceGuardArray = (value: unknown, at: string, bounds: femFem2dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dInferenceGuardString = (value: unknown, at: string, bounds: femFem2dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dInferenceGuardReject(at, "value is not a boolean"));
export const femFem2dInferenceGuardNumber = (value: unknown, at: string, bounds: femFem2dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dInferenceGuardInteger = (value: unknown, at: string, bounds: femFem2dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dInferenceGuardNumber(value, at, bounds) : femFem2dInferenceGuardReject(at, "value is not an integer");
export const femFem2dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dInference(value: unknown, at = "$"): Fem2dInference {
  const row = femFem2dInferenceGuardObject(value, at);
  return {
    bounds: parseFem2dBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseFem2dBoundingBox(value: unknown, at = "$"): Fem2dBoundingBox {
  const row = femFem2dInferenceGuardObject(value, at);
  return {
    min: femFem2dInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 2, "maxItems": 2}).map((item, index) => femFem2dInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: femFem2dInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 2, "maxItems": 2}).map((item, index) => femFem2dInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}

export function parseFem2dBounds(value: unknown, at = "$"): Fem2dBounds {
  const row = femFem2dInferenceGuardObject(value, at);
  return {
    boundingBox: parseFem2dBoundingBox(row["boundingBox"], `${at}.boundingBox`),
    nodeCount: femFem2dInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    elementCount: femFem2dInferenceGuardInteger(row["elementCount"], `${at}.elementCount`, {"minimum": 0}),
  };
}
