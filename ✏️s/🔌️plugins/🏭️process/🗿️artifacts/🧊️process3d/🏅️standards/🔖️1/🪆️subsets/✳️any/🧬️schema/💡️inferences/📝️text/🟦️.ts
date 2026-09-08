/** 📝️ Text representation for `process.process3d.inference`. */
export type Process3dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new processProcess3dInferenceTextGuardRefusal(at, why);
};

type processProcess3dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dInferenceTextGuardReject(at, "value is not an object");
export const processProcess3dInferenceTextGuardArray = (value: unknown, at: string, bounds: processProcess3dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dInferenceTextGuardString = (value: unknown, at: string, bounds: processProcess3dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dInferenceTextGuardReject(at, "value is not a boolean"));
export const processProcess3dInferenceTextGuardNumber = (value: unknown, at: string, bounds: processProcess3dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dInferenceTextGuardInteger = (value: unknown, at: string, bounds: processProcess3dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dInferenceTextGuardNumber(value, at, bounds) : processProcess3dInferenceTextGuardReject(at, "value is not an integer");
export const processProcess3dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dInferenceText(value: unknown, at = "$"): Process3dInferenceText {
  return processProcess3dInferenceTextGuardObject(value, `${at}`);
}
