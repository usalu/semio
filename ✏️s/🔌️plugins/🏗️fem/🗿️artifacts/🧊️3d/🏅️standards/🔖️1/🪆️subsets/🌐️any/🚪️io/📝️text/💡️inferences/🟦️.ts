/** 📝️ Text representation for `fem.fem3d.inference`. */
export type Fem3dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new femFem3dInferenceTextGuardRefusal(at, why);
};

type femFem3dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dInferenceTextGuardReject(at, "value is not an object");
export const femFem3dInferenceTextGuardArray = (value: unknown, at: string, bounds: femFem3dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dInferenceTextGuardString = (value: unknown, at: string, bounds: femFem3dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dInferenceTextGuardReject(at, "value is not a boolean"));
export const femFem3dInferenceTextGuardNumber = (value: unknown, at: string, bounds: femFem3dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dInferenceTextGuardInteger = (value: unknown, at: string, bounds: femFem3dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dInferenceTextGuardNumber(value, at, bounds) : femFem3dInferenceTextGuardReject(at, "value is not an integer");
export const femFem3dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dInferenceText(value: unknown, at = "$"): Fem3dInferenceText {
  return femFem3dInferenceTextGuardObject(value, `${at}`);
}
