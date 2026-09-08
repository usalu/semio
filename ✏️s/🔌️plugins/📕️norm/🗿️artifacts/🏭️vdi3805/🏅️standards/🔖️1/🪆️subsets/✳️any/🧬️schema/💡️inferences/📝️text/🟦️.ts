/** 📝️ Text representation for `norm.vdi3805.inference`. */
export type Vdi3805InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normVdi3805InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normVdi3805InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normVdi3805InferenceTextGuardRefusal(at, why);
};

type normVdi3805InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normVdi3805InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normVdi3805InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normVdi3805InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normVdi3805InferenceTextGuardReject(at, "value is not an object");
export const normVdi3805InferenceTextGuardArray = (value: unknown, at: string, bounds: normVdi3805InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normVdi3805InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normVdi3805InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normVdi3805InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normVdi3805InferenceTextGuardString = (value: unknown, at: string, bounds: normVdi3805InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normVdi3805InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normVdi3805InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normVdi3805InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normVdi3805InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normVdi3805InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normVdi3805InferenceTextGuardReject(at, "value is not a boolean"));
export const normVdi3805InferenceTextGuardNumber = (value: unknown, at: string, bounds: normVdi3805InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normVdi3805InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normVdi3805InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normVdi3805InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normVdi3805InferenceTextGuardInteger = (value: unknown, at: string, bounds: normVdi3805InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normVdi3805InferenceTextGuardNumber(value, at, bounds) : normVdi3805InferenceTextGuardReject(at, "value is not an integer");
export const normVdi3805InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normVdi3805InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normVdi3805InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normVdi3805InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVdi3805InferenceText(value: unknown, at = "$"): Vdi3805InferenceText {
  return normVdi3805InferenceTextGuardObject(value, `${at}`);
}
