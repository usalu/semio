/** 📝️ Text representation for `gis.gismap.inference`. */
export type GisMapInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGismapInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGismapInferenceTextGuardReject = (at: string, why: string): never => {
  throw new gisGismapInferenceTextGuardRefusal(at, why);
};

type gisGismapInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGismapInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGismapInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGismapInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGismapInferenceTextGuardReject(at, "value is not an object");
export const gisGismapInferenceTextGuardArray = (value: unknown, at: string, bounds: gisGismapInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGismapInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGismapInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGismapInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGismapInferenceTextGuardString = (value: unknown, at: string, bounds: gisGismapInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGismapInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGismapInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGismapInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGismapInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGismapInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGismapInferenceTextGuardReject(at, "value is not a boolean"));
export const gisGismapInferenceTextGuardNumber = (value: unknown, at: string, bounds: gisGismapInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGismapInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGismapInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGismapInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGismapInferenceTextGuardInteger = (value: unknown, at: string, bounds: gisGismapInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGismapInferenceTextGuardNumber(value, at, bounds) : gisGismapInferenceTextGuardReject(at, "value is not an integer");
export const gisGismapInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGismapInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGismapInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGismapInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisMapInferenceText(value: unknown, at = "$"): GisMapInferenceText {
  return gisGismapInferenceTextGuardObject(value, `${at}`);
}
