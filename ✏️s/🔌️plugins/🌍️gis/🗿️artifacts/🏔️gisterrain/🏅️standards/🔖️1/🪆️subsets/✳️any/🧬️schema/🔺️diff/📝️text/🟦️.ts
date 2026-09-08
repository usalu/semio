/** 📝️ Text representation for `gis.gisterrain.diff`. */
export type GisTerrainDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainDiffTextGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainDiffTextGuardRefusal(at, why);
};

type gisGisterrainDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainDiffTextGuardReject(at, "value is not an object");
export const gisGisterrainDiffTextGuardArray = (value: unknown, at: string, bounds: gisGisterrainDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainDiffTextGuardString = (value: unknown, at: string, bounds: gisGisterrainDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainDiffTextGuardReject(at, "value is not a boolean"));
export const gisGisterrainDiffTextGuardNumber = (value: unknown, at: string, bounds: gisGisterrainDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainDiffTextGuardInteger = (value: unknown, at: string, bounds: gisGisterrainDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainDiffTextGuardNumber(value, at, bounds) : gisGisterrainDiffTextGuardReject(at, "value is not an integer");
export const gisGisterrainDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainDiffText(value: unknown, at = "$"): GisTerrainDiffText {
  return gisGisterrainDiffTextGuardObject(value, `${at}`);
}
