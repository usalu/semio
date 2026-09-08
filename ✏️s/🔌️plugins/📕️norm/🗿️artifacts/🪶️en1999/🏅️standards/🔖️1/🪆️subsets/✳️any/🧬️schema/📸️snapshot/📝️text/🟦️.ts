/** 📝️ Text representation for `norm.en1999.snapshot`. */
export type En1999SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1999SnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1999SnapshotTextGuardReject = (at: string, why: string): never => {
  throw new normEn1999SnapshotTextGuardRefusal(at, why);
};

type normEn1999SnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1999SnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1999SnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1999SnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1999SnapshotTextGuardReject(at, "value is not an object");
export const normEn1999SnapshotTextGuardArray = (value: unknown, at: string, bounds: normEn1999SnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1999SnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1999SnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1999SnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1999SnapshotTextGuardString = (value: unknown, at: string, bounds: normEn1999SnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1999SnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1999SnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1999SnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1999SnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1999SnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1999SnapshotTextGuardReject(at, "value is not a boolean"));
export const normEn1999SnapshotTextGuardNumber = (value: unknown, at: string, bounds: normEn1999SnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1999SnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1999SnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1999SnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1999SnapshotTextGuardInteger = (value: unknown, at: string, bounds: normEn1999SnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1999SnapshotTextGuardNumber(value, at, bounds) : normEn1999SnapshotTextGuardReject(at, "value is not an integer");
export const normEn1999SnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1999SnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1999SnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1999SnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1999SnapshotText(value: unknown, at = "$"): En1999SnapshotText {
  return normEn1999SnapshotTextGuardObject(value, `${at}`);
}
