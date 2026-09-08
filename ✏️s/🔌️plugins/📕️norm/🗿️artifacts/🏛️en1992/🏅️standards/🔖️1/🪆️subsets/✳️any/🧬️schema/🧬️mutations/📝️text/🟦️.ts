/** 📝️ Text representation for `norm.en1992.mutations`. */
export type En1992MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1992MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1992MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normEn1992MutationsTextGuardRefusal(at, why);
};

type normEn1992MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1992MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1992MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1992MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1992MutationsTextGuardReject(at, "value is not an object");
export const normEn1992MutationsTextGuardArray = (value: unknown, at: string, bounds: normEn1992MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1992MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1992MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1992MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1992MutationsTextGuardString = (value: unknown, at: string, bounds: normEn1992MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1992MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1992MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1992MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1992MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1992MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1992MutationsTextGuardReject(at, "value is not a boolean"));
export const normEn1992MutationsTextGuardNumber = (value: unknown, at: string, bounds: normEn1992MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1992MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1992MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1992MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1992MutationsTextGuardInteger = (value: unknown, at: string, bounds: normEn1992MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1992MutationsTextGuardNumber(value, at, bounds) : normEn1992MutationsTextGuardReject(at, "value is not an integer");
export const normEn1992MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1992MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1992MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1992MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1992MutationsText(value: unknown, at = "$"): En1992MutationsText {
  return normEn1992MutationsTextGuardObject(value, `${at}`);
}
