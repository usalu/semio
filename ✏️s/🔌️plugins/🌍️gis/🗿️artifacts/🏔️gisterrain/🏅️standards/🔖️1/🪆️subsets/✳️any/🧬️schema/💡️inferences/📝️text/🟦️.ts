/** 📝️ Text representation for `gis.gisterrain.inference`. */
export type GisTerrainInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainInferenceTextGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainInferenceTextGuardRefusal(at, why);
};

type gisGisterrainInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainInferenceTextGuardReject(at, "value is not an object");
export const gisGisterrainInferenceTextGuardArray = (value: unknown, at: string, bounds: gisGisterrainInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainInferenceTextGuardString = (value: unknown, at: string, bounds: gisGisterrainInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainInferenceTextGuardReject(at, "value is not a boolean"));
export const gisGisterrainInferenceTextGuardNumber = (value: unknown, at: string, bounds: gisGisterrainInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainInferenceTextGuardInteger = (value: unknown, at: string, bounds: gisGisterrainInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainInferenceTextGuardNumber(value, at, bounds) : gisGisterrainInferenceTextGuardReject(at, "value is not an integer");
export const gisGisterrainInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainInferenceText(value: unknown, at = "$"): GisTerrainInferenceText {
  return gisGisterrainInferenceTextGuardObject(value, `${at}`);
}
