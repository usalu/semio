/** 📝️ Text representation for `gis.gisterrain.snapshot`. */
export type GisTerrainSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainSnapshotTextGuardRefusal(at, why);
};

type gisGisterrainSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainSnapshotTextGuardReject(at, "value is not an object");
export const gisGisterrainSnapshotTextGuardArray = (value: unknown, at: string, bounds: gisGisterrainSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainSnapshotTextGuardString = (value: unknown, at: string, bounds: gisGisterrainSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainSnapshotTextGuardReject(at, "value is not a boolean"));
export const gisGisterrainSnapshotTextGuardNumber = (value: unknown, at: string, bounds: gisGisterrainSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainSnapshotTextGuardInteger = (value: unknown, at: string, bounds: gisGisterrainSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainSnapshotTextGuardNumber(value, at, bounds) : gisGisterrainSnapshotTextGuardReject(at, "value is not an integer");
export const gisGisterrainSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainSnapshotText(value: unknown, at = "$"): GisTerrainSnapshotText {
  return gisGisterrainSnapshotTextGuardObject(value, `${at}`);
}
