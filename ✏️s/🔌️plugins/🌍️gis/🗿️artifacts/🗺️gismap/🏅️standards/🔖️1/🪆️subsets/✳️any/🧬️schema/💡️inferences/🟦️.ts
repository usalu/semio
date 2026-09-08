/** 💡️ GIS map inference schema — per-collection feature counts + geographic bounding box. */

export interface GisMapBounds {
  lonMin: number;
  lonMax: number;
  latMin: number;
  latMax: number;
}

export interface GisMapInference {
  /** @derived */
  positionCount: number;
  /** @derived */
  routeCount: number;
  /** @derived */
  regionCount: number;
  /** @derived */
  bounds: GisMapBounds | null;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGismapInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGismapInferenceGuardReject = (at: string, why: string): never => {
  throw new gisGismapInferenceGuardRefusal(at, why);
};

type gisGismapInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGismapInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGismapInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGismapInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGismapInferenceGuardReject(at, "value is not an object");
export const gisGismapInferenceGuardArray = (value: unknown, at: string, bounds: gisGismapInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGismapInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGismapInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGismapInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGismapInferenceGuardString = (value: unknown, at: string, bounds: gisGismapInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGismapInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGismapInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGismapInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGismapInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGismapInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGismapInferenceGuardReject(at, "value is not a boolean"));
export const gisGismapInferenceGuardNumber = (value: unknown, at: string, bounds: gisGismapInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGismapInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGismapInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGismapInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGismapInferenceGuardInteger = (value: unknown, at: string, bounds: gisGismapInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGismapInferenceGuardNumber(value, at, bounds) : gisGismapInferenceGuardReject(at, "value is not an integer");
export const gisGismapInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGismapInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGismapInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGismapInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisMapBounds(value: unknown, at = "$"): GisMapBounds {
  const row = gisGismapInferenceGuardObject(value, at);
  return {
    lonMin: gisGismapInferenceGuardNumber(row["lonMin"], `${at}.lonMin`),
    lonMax: gisGismapInferenceGuardNumber(row["lonMax"], `${at}.lonMax`),
    latMin: gisGismapInferenceGuardNumber(row["latMin"], `${at}.latMin`),
    latMax: gisGismapInferenceGuardNumber(row["latMax"], `${at}.latMax`),
  };
}
