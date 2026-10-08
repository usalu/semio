/** 📝️ Text representation for `bim.model.inference`. */
export type ModelInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelInferenceTextGuardReject = (at: string, why: string): never => {
  throw new bimModelInferenceTextGuardRefusal(at, why);
};

type bimModelInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type bimModelInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelInferenceTextGuardReject(at, "value is not an object");
export const bimModelInferenceTextGuardArray = (value: unknown, at: string, bounds: bimModelInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelInferenceTextGuardString = (value: unknown, at: string, bounds: bimModelInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelInferenceTextGuardReject(at, "value is not a boolean"));
export const bimModelInferenceTextGuardNumber = (value: unknown, at: string, bounds: bimModelInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const bimModelInferenceTextGuardInteger = (value: unknown, at: string, bounds: bimModelInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelInferenceTextGuardNumber(value, at, bounds) : bimModelInferenceTextGuardReject(at, "value is not an integer");
export const bimModelInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseModelInferenceText(value: unknown, at = "$"): ModelInferenceText {
  return bimModelInferenceTextGuardObject(value, `${at}`);
}
