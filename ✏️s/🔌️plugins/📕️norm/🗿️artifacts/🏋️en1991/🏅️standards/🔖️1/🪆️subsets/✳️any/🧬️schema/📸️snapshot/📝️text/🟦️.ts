/** 📝️ Text representation for `norm.en1991.snapshot`. */
export type En1991SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1991SnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1991SnapshotTextGuardReject = (at: string, why: string): never => {
  throw new normEn1991SnapshotTextGuardRefusal(at, why);
};

type normEn1991SnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1991SnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1991SnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1991SnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1991SnapshotTextGuardReject(at, "value is not an object");
export const normEn1991SnapshotTextGuardArray = (value: unknown, at: string, bounds: normEn1991SnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1991SnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1991SnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1991SnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1991SnapshotTextGuardString = (value: unknown, at: string, bounds: normEn1991SnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1991SnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1991SnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1991SnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1991SnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1991SnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1991SnapshotTextGuardReject(at, "value is not a boolean"));
export const normEn1991SnapshotTextGuardNumber = (value: unknown, at: string, bounds: normEn1991SnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1991SnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1991SnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1991SnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1991SnapshotTextGuardInteger = (value: unknown, at: string, bounds: normEn1991SnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1991SnapshotTextGuardNumber(value, at, bounds) : normEn1991SnapshotTextGuardReject(at, "value is not an integer");
export const normEn1991SnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1991SnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1991SnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1991SnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1991SnapshotText(value: unknown, at = "$"): En1991SnapshotText {
  return normEn1991SnapshotTextGuardObject(value, `${at}`);
}
