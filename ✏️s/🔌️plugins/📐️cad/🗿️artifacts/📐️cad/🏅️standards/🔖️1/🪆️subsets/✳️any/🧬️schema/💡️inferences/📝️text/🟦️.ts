/** 📝️ Text representation for `cad.cad.inference`. */
export type CadInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class cadCadInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const cadCadInferenceTextGuardReject = (at: string, why: string): never => {
  throw new cadCadInferenceTextGuardRefusal(at, why);
};

type cadCadInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type cadCadInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type cadCadInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const cadCadInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : cadCadInferenceTextGuardReject(at, "value is not an object");
export const cadCadInferenceTextGuardArray = (value: unknown, at: string, bounds: cadCadInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return cadCadInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) cadCadInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) cadCadInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const cadCadInferenceTextGuardString = (value: unknown, at: string, bounds: cadCadInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return cadCadInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) cadCadInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) cadCadInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) cadCadInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const cadCadInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : cadCadInferenceTextGuardReject(at, "value is not a boolean"));
export const cadCadInferenceTextGuardNumber = (value: unknown, at: string, bounds: cadCadInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return cadCadInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) cadCadInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) cadCadInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const cadCadInferenceTextGuardInteger = (value: unknown, at: string, bounds: cadCadInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? cadCadInferenceTextGuardNumber(value, at, bounds) : cadCadInferenceTextGuardReject(at, "value is not an integer");
export const cadCadInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : cadCadInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const cadCadInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : cadCadInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCadInferenceText(value: unknown, at = "$"): CadInferenceText {
  return cadCadInferenceTextGuardObject(value, `${at}`);
}
