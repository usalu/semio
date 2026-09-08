/** 📝️ Text representation for `norm.en1993.mutations`. */
export type En1993MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1993MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1993MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normEn1993MutationsTextGuardRefusal(at, why);
};

type normEn1993MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1993MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1993MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1993MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1993MutationsTextGuardReject(at, "value is not an object");
export const normEn1993MutationsTextGuardArray = (value: unknown, at: string, bounds: normEn1993MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1993MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1993MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1993MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1993MutationsTextGuardString = (value: unknown, at: string, bounds: normEn1993MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1993MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1993MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1993MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1993MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1993MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1993MutationsTextGuardReject(at, "value is not a boolean"));
export const normEn1993MutationsTextGuardNumber = (value: unknown, at: string, bounds: normEn1993MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1993MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1993MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1993MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1993MutationsTextGuardInteger = (value: unknown, at: string, bounds: normEn1993MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1993MutationsTextGuardNumber(value, at, bounds) : normEn1993MutationsTextGuardReject(at, "value is not an integer");
export const normEn1993MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1993MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1993MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1993MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1993MutationsText(value: unknown, at = "$"): En1993MutationsText {
  return normEn1993MutationsTextGuardObject(value, `${at}`);
}
