/** 📝️ Text representation for `norm.en1996.mutations`. */
export type En1996MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1996MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1996MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normEn1996MutationsTextGuardRefusal(at, why);
};

type normEn1996MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1996MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1996MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1996MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1996MutationsTextGuardReject(at, "value is not an object");
export const normEn1996MutationsTextGuardArray = (value: unknown, at: string, bounds: normEn1996MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1996MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1996MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1996MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1996MutationsTextGuardString = (value: unknown, at: string, bounds: normEn1996MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1996MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1996MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1996MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1996MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1996MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1996MutationsTextGuardReject(at, "value is not a boolean"));
export const normEn1996MutationsTextGuardNumber = (value: unknown, at: string, bounds: normEn1996MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1996MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1996MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1996MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1996MutationsTextGuardInteger = (value: unknown, at: string, bounds: normEn1996MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1996MutationsTextGuardNumber(value, at, bounds) : normEn1996MutationsTextGuardReject(at, "value is not an integer");
export const normEn1996MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1996MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1996MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1996MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1996MutationsText(value: unknown, at = "$"): En1996MutationsText {
  return normEn1996MutationsTextGuardObject(value, `${at}`);
}
