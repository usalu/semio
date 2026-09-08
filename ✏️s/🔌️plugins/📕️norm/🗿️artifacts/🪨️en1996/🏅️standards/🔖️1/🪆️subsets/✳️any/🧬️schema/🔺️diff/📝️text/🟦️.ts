/** 📝️ Text representation for `norm.en1996.diff`. */
export type En1996DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1996DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1996DiffTextGuardReject = (at: string, why: string): never => {
  throw new normEn1996DiffTextGuardRefusal(at, why);
};

type normEn1996DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1996DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1996DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1996DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1996DiffTextGuardReject(at, "value is not an object");
export const normEn1996DiffTextGuardArray = (value: unknown, at: string, bounds: normEn1996DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1996DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1996DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1996DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1996DiffTextGuardString = (value: unknown, at: string, bounds: normEn1996DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1996DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1996DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1996DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1996DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1996DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1996DiffTextGuardReject(at, "value is not a boolean"));
export const normEn1996DiffTextGuardNumber = (value: unknown, at: string, bounds: normEn1996DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1996DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1996DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1996DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1996DiffTextGuardInteger = (value: unknown, at: string, bounds: normEn1996DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1996DiffTextGuardNumber(value, at, bounds) : normEn1996DiffTextGuardReject(at, "value is not an integer");
export const normEn1996DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1996DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1996DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1996DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1996DiffText(value: unknown, at = "$"): En1996DiffText {
  return normEn1996DiffTextGuardObject(value, `${at}`);
}
