/** 📝️ Text representation for `gis.gismap.diff`. */
export type GisMapDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGismapDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGismapDiffTextGuardReject = (at: string, why: string): never => {
  throw new gisGismapDiffTextGuardRefusal(at, why);
};

type gisGismapDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGismapDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGismapDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGismapDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGismapDiffTextGuardReject(at, "value is not an object");
export const gisGismapDiffTextGuardArray = (value: unknown, at: string, bounds: gisGismapDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGismapDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGismapDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGismapDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGismapDiffTextGuardString = (value: unknown, at: string, bounds: gisGismapDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGismapDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGismapDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGismapDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGismapDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGismapDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGismapDiffTextGuardReject(at, "value is not a boolean"));
export const gisGismapDiffTextGuardNumber = (value: unknown, at: string, bounds: gisGismapDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGismapDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGismapDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGismapDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGismapDiffTextGuardInteger = (value: unknown, at: string, bounds: gisGismapDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGismapDiffTextGuardNumber(value, at, bounds) : gisGismapDiffTextGuardReject(at, "value is not an integer");
export const gisGismapDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGismapDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGismapDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGismapDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisMapDiffText(value: unknown, at = "$"): GisMapDiffText {
  return gisGismapDiffTextGuardObject(value, `${at}`);
}
