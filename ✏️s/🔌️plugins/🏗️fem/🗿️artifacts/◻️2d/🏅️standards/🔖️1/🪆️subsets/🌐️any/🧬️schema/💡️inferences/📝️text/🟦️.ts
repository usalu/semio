/** 📝️ Text representation for `fem.fem2d.inference`. */
export type Fem2dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new femFem2dInferenceTextGuardRefusal(at, why);
};

type femFem2dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dInferenceTextGuardReject(at, "value is not an object");
export const femFem2dInferenceTextGuardArray = (value: unknown, at: string, bounds: femFem2dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dInferenceTextGuardString = (value: unknown, at: string, bounds: femFem2dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dInferenceTextGuardReject(at, "value is not a boolean"));
export const femFem2dInferenceTextGuardNumber = (value: unknown, at: string, bounds: femFem2dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dInferenceTextGuardInteger = (value: unknown, at: string, bounds: femFem2dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dInferenceTextGuardNumber(value, at, bounds) : femFem2dInferenceTextGuardReject(at, "value is not an integer");
export const femFem2dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dInferenceText(value: unknown, at = "$"): Fem2dInferenceText {
  return femFem2dInferenceTextGuardObject(value, `${at}`);
}
