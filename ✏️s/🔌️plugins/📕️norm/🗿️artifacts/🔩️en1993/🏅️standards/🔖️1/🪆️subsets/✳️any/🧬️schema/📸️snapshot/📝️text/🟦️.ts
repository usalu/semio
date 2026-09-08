/** 📝️ Text representation for `norm.en1993.snapshot`. */
export type En1993SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1993SnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1993SnapshotTextGuardReject = (at: string, why: string): never => {
  throw new normEn1993SnapshotTextGuardRefusal(at, why);
};

type normEn1993SnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1993SnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1993SnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1993SnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1993SnapshotTextGuardReject(at, "value is not an object");
export const normEn1993SnapshotTextGuardArray = (value: unknown, at: string, bounds: normEn1993SnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1993SnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1993SnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1993SnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1993SnapshotTextGuardString = (value: unknown, at: string, bounds: normEn1993SnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1993SnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1993SnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1993SnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1993SnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1993SnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1993SnapshotTextGuardReject(at, "value is not a boolean"));
export const normEn1993SnapshotTextGuardNumber = (value: unknown, at: string, bounds: normEn1993SnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1993SnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1993SnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1993SnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1993SnapshotTextGuardInteger = (value: unknown, at: string, bounds: normEn1993SnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1993SnapshotTextGuardNumber(value, at, bounds) : normEn1993SnapshotTextGuardReject(at, "value is not an integer");
export const normEn1993SnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1993SnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1993SnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1993SnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1993SnapshotText(value: unknown, at = "$"): En1993SnapshotText {
  return normEn1993SnapshotTextGuardObject(value, `${at}`);
}
