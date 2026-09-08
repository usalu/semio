/** 📝️ Text representation for `gis.gismap.snapshot`. */
export type GisMapSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGismapSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGismapSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new gisGismapSnapshotTextGuardRefusal(at, why);
};

type gisGismapSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGismapSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGismapSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGismapSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGismapSnapshotTextGuardReject(at, "value is not an object");
export const gisGismapSnapshotTextGuardArray = (value: unknown, at: string, bounds: gisGismapSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGismapSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGismapSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGismapSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGismapSnapshotTextGuardString = (value: unknown, at: string, bounds: gisGismapSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGismapSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGismapSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGismapSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGismapSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGismapSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGismapSnapshotTextGuardReject(at, "value is not a boolean"));
export const gisGismapSnapshotTextGuardNumber = (value: unknown, at: string, bounds: gisGismapSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGismapSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGismapSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGismapSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGismapSnapshotTextGuardInteger = (value: unknown, at: string, bounds: gisGismapSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGismapSnapshotTextGuardNumber(value, at, bounds) : gisGismapSnapshotTextGuardReject(at, "value is not an integer");
export const gisGismapSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGismapSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGismapSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGismapSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisMapSnapshotText(value: unknown, at = "$"): GisMapSnapshotText {
  return gisGismapSnapshotTextGuardObject(value, `${at}`);
}
