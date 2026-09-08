/** 📝️ Text representation for `norm.din4108.inference`. */
export type Din4108InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin4108InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin4108InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normDin4108InferenceTextGuardRefusal(at, why);
};

type normDin4108InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin4108InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin4108InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin4108InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin4108InferenceTextGuardReject(at, "value is not an object");
export const normDin4108InferenceTextGuardArray = (value: unknown, at: string, bounds: normDin4108InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin4108InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin4108InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin4108InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin4108InferenceTextGuardString = (value: unknown, at: string, bounds: normDin4108InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin4108InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin4108InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin4108InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin4108InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin4108InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin4108InferenceTextGuardReject(at, "value is not a boolean"));
export const normDin4108InferenceTextGuardNumber = (value: unknown, at: string, bounds: normDin4108InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin4108InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin4108InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin4108InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin4108InferenceTextGuardInteger = (value: unknown, at: string, bounds: normDin4108InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin4108InferenceTextGuardNumber(value, at, bounds) : normDin4108InferenceTextGuardReject(at, "value is not an integer");
export const normDin4108InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin4108InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin4108InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin4108InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin4108InferenceText(value: unknown, at = "$"): Din4108InferenceText {
  return normDin4108InferenceTextGuardObject(value, `${at}`);
}
