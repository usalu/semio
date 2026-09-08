/** 📝️ Text representation for `norm.en1995.diff`. */
export type En1995DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1995DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1995DiffTextGuardReject = (at: string, why: string): never => {
  throw new normEn1995DiffTextGuardRefusal(at, why);
};

type normEn1995DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1995DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1995DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1995DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1995DiffTextGuardReject(at, "value is not an object");
export const normEn1995DiffTextGuardArray = (value: unknown, at: string, bounds: normEn1995DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1995DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1995DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1995DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1995DiffTextGuardString = (value: unknown, at: string, bounds: normEn1995DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1995DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1995DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1995DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1995DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1995DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1995DiffTextGuardReject(at, "value is not a boolean"));
export const normEn1995DiffTextGuardNumber = (value: unknown, at: string, bounds: normEn1995DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1995DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1995DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1995DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1995DiffTextGuardInteger = (value: unknown, at: string, bounds: normEn1995DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1995DiffTextGuardNumber(value, at, bounds) : normEn1995DiffTextGuardReject(at, "value is not an integer");
export const normEn1995DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1995DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1995DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1995DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1995DiffText(value: unknown, at = "$"): En1995DiffText {
  return normEn1995DiffTextGuardObject(value, `${at}`);
}
