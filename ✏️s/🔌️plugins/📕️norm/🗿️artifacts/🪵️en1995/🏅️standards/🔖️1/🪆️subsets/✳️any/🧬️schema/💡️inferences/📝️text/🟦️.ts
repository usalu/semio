/** 📝️ Text representation for `norm.en1995.inference`. */
export type En1995InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1995InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1995InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normEn1995InferenceTextGuardRefusal(at, why);
};

type normEn1995InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1995InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1995InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1995InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1995InferenceTextGuardReject(at, "value is not an object");
export const normEn1995InferenceTextGuardArray = (value: unknown, at: string, bounds: normEn1995InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1995InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1995InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1995InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1995InferenceTextGuardString = (value: unknown, at: string, bounds: normEn1995InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1995InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1995InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1995InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1995InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1995InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1995InferenceTextGuardReject(at, "value is not a boolean"));
export const normEn1995InferenceTextGuardNumber = (value: unknown, at: string, bounds: normEn1995InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1995InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1995InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1995InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1995InferenceTextGuardInteger = (value: unknown, at: string, bounds: normEn1995InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1995InferenceTextGuardNumber(value, at, bounds) : normEn1995InferenceTextGuardReject(at, "value is not an integer");
export const normEn1995InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1995InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1995InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1995InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1995InferenceText(value: unknown, at = "$"): En1995InferenceText {
  return normEn1995InferenceTextGuardObject(value, `${at}`);
}
