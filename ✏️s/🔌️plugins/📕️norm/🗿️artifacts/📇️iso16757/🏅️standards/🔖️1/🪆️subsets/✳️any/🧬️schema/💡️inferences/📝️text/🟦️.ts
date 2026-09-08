/** 📝️ Text representation for `norm.iso16757.inference`. */
export type Iso16757InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normIso16757InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normIso16757InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normIso16757InferenceTextGuardRefusal(at, why);
};

type normIso16757InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normIso16757InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normIso16757InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normIso16757InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normIso16757InferenceTextGuardReject(at, "value is not an object");
export const normIso16757InferenceTextGuardArray = (value: unknown, at: string, bounds: normIso16757InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normIso16757InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normIso16757InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normIso16757InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normIso16757InferenceTextGuardString = (value: unknown, at: string, bounds: normIso16757InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normIso16757InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normIso16757InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normIso16757InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normIso16757InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normIso16757InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normIso16757InferenceTextGuardReject(at, "value is not a boolean"));
export const normIso16757InferenceTextGuardNumber = (value: unknown, at: string, bounds: normIso16757InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normIso16757InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normIso16757InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normIso16757InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normIso16757InferenceTextGuardInteger = (value: unknown, at: string, bounds: normIso16757InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normIso16757InferenceTextGuardNumber(value, at, bounds) : normIso16757InferenceTextGuardReject(at, "value is not an integer");
export const normIso16757InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normIso16757InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normIso16757InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normIso16757InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIso16757InferenceText(value: unknown, at = "$"): Iso16757InferenceText {
  return normIso16757InferenceTextGuardObject(value, `${at}`);
}
