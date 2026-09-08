/** 💡️ Fem3d inference schema — bounds (3d extent + node/element counts). */

export interface Fem3dBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Fem3dBounds {
  boundingBox: Fem3dBoundingBox;
  nodeCount: number;
  elementCount: number;
}

export interface Fem3dInference {
  /** @derived */
  bounds: Fem3dBounds;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dInferenceGuardReject = (at: string, why: string): never => {
  throw new femFem3dInferenceGuardRefusal(at, why);
};

type femFem3dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dInferenceGuardReject(at, "value is not an object");
export const femFem3dInferenceGuardArray = (value: unknown, at: string, bounds: femFem3dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dInferenceGuardString = (value: unknown, at: string, bounds: femFem3dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dInferenceGuardReject(at, "value is not a boolean"));
export const femFem3dInferenceGuardNumber = (value: unknown, at: string, bounds: femFem3dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dInferenceGuardInteger = (value: unknown, at: string, bounds: femFem3dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dInferenceGuardNumber(value, at, bounds) : femFem3dInferenceGuardReject(at, "value is not an integer");
export const femFem3dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dInference(value: unknown, at = "$"): Fem3dInference {
  const row = femFem3dInferenceGuardObject(value, at);
  return {
    bounds: parseFem3dBounds(row["bounds"], `${at}.bounds`),
  };
}

export function parseFem3dBoundingBox(value: unknown, at = "$"): Fem3dBoundingBox {
  const row = femFem3dInferenceGuardObject(value, at);
  return {
    min: femFem3dInferenceGuardArray(row["min"], `${at}.min`, {"minItems": 3, "maxItems": 3}).map((item, index) => femFem3dInferenceGuardNumber(item, `${at}.min[${index}]`)),
    max: femFem3dInferenceGuardArray(row["max"], `${at}.max`, {"minItems": 3, "maxItems": 3}).map((item, index) => femFem3dInferenceGuardNumber(item, `${at}.max[${index}]`)),
  };
}

export function parseFem3dBounds(value: unknown, at = "$"): Fem3dBounds {
  const row = femFem3dInferenceGuardObject(value, at);
  return {
    boundingBox: parseFem3dBoundingBox(row["boundingBox"], `${at}.boundingBox`),
    nodeCount: femFem3dInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    elementCount: femFem3dInferenceGuardInteger(row["elementCount"], `${at}.elementCount`, {"minimum": 0}),
  };
}
