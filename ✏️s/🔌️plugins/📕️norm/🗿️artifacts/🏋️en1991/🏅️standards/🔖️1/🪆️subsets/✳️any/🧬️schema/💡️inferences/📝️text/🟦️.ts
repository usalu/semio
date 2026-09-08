/** 📝️ Text representation for `norm.en1991.inference`. */
export type En1991InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1991InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1991InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normEn1991InferenceTextGuardRefusal(at, why);
};

type normEn1991InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1991InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1991InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1991InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1991InferenceTextGuardReject(at, "value is not an object");
export const normEn1991InferenceTextGuardArray = (value: unknown, at: string, bounds: normEn1991InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1991InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1991InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1991InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1991InferenceTextGuardString = (value: unknown, at: string, bounds: normEn1991InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1991InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1991InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1991InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1991InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1991InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1991InferenceTextGuardReject(at, "value is not a boolean"));
export const normEn1991InferenceTextGuardNumber = (value: unknown, at: string, bounds: normEn1991InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1991InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1991InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1991InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1991InferenceTextGuardInteger = (value: unknown, at: string, bounds: normEn1991InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1991InferenceTextGuardNumber(value, at, bounds) : normEn1991InferenceTextGuardReject(at, "value is not an integer");
export const normEn1991InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1991InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1991InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1991InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1991InferenceText(value: unknown, at = "$"): En1991InferenceText {
  return normEn1991InferenceTextGuardObject(value, `${at}`);
}
