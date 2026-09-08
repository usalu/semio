/** 💡️ GIS terrain inference schema — geographic bounding box + position count of the `map:in`
 * overlay decoded from `importedFeaturesJson`. */

export interface GisTerrainBounds {
  lonMin: number;
  lonMax: number;
  latMin: number;
  latMax: number;
}

export interface GisTerrainInference {
  /** @derived */
  positionCount: number;
  /** @derived */
  bounds: GisTerrainBounds | null;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainInferenceGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainInferenceGuardRefusal(at, why);
};

type gisGisterrainInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainInferenceGuardReject(at, "value is not an object");
export const gisGisterrainInferenceGuardArray = (value: unknown, at: string, bounds: gisGisterrainInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainInferenceGuardString = (value: unknown, at: string, bounds: gisGisterrainInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainInferenceGuardReject(at, "value is not a boolean"));
export const gisGisterrainInferenceGuardNumber = (value: unknown, at: string, bounds: gisGisterrainInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainInferenceGuardInteger = (value: unknown, at: string, bounds: gisGisterrainInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainInferenceGuardNumber(value, at, bounds) : gisGisterrainInferenceGuardReject(at, "value is not an integer");
export const gisGisterrainInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainBounds(value: unknown, at = "$"): GisTerrainBounds {
  const row = gisGisterrainInferenceGuardObject(value, at);
  return {
    lonMin: gisGisterrainInferenceGuardNumber(row["lonMin"], `${at}.lonMin`),
    lonMax: gisGisterrainInferenceGuardNumber(row["lonMax"], `${at}.lonMax`),
    latMin: gisGisterrainInferenceGuardNumber(row["latMin"], `${at}.latMin`),
    latMax: gisGisterrainInferenceGuardNumber(row["latMax"], `${at}.latMax`),
  };
}
