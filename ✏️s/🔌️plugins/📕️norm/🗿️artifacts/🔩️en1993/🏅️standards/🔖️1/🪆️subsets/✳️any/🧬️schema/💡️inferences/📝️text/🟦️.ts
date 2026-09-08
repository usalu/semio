/** 📝️ Text representation for `norm.en1993.inference`. */
export type En1993InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1993InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1993InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normEn1993InferenceTextGuardRefusal(at, why);
};

type normEn1993InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1993InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1993InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1993InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1993InferenceTextGuardReject(at, "value is not an object");
export const normEn1993InferenceTextGuardArray = (value: unknown, at: string, bounds: normEn1993InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1993InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1993InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1993InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1993InferenceTextGuardString = (value: unknown, at: string, bounds: normEn1993InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1993InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1993InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1993InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1993InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1993InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1993InferenceTextGuardReject(at, "value is not a boolean"));
export const normEn1993InferenceTextGuardNumber = (value: unknown, at: string, bounds: normEn1993InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1993InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1993InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1993InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1993InferenceTextGuardInteger = (value: unknown, at: string, bounds: normEn1993InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1993InferenceTextGuardNumber(value, at, bounds) : normEn1993InferenceTextGuardReject(at, "value is not an integer");
export const normEn1993InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1993InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1993InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1993InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1993InferenceText(value: unknown, at = "$"): En1993InferenceText {
  return normEn1993InferenceTextGuardObject(value, `${at}`);
}
