/** 📝️ Text representation for `procedural.generation3d.inference`. */
export type Generation3dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dInferenceTextGuardRefusal(at, why);
};

type proceduralGeneration3dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dInferenceTextGuardReject(at, "value is not an object");
export const proceduralGeneration3dInferenceTextGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dInferenceTextGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dInferenceTextGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dInferenceTextGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dInferenceTextGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dInferenceTextGuardNumber(value, at, bounds) : proceduralGeneration3dInferenceTextGuardReject(at, "value is not an integer");
export const proceduralGeneration3dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dInferenceText(value: unknown, at = "$"): Generation3dInferenceText {
  return proceduralGeneration3dInferenceTextGuardObject(value, `${at}`);
}
