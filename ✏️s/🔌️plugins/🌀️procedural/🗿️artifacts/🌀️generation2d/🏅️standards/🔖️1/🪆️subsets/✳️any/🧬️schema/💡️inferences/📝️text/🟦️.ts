/** 📝️ Text representation for `procedural.generation2d.inference`. */
export type Generation2dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration2dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration2dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration2dInferenceTextGuardRefusal(at, why);
};

type proceduralGeneration2dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration2dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration2dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration2dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration2dInferenceTextGuardReject(at, "value is not an object");
export const proceduralGeneration2dInferenceTextGuardArray = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration2dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration2dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration2dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration2dInferenceTextGuardString = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration2dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration2dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration2dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration2dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration2dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration2dInferenceTextGuardReject(at, "value is not a boolean"));
export const proceduralGeneration2dInferenceTextGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration2dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration2dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration2dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration2dInferenceTextGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration2dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration2dInferenceTextGuardNumber(value, at, bounds) : proceduralGeneration2dInferenceTextGuardReject(at, "value is not an integer");
export const proceduralGeneration2dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration2dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration2dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration2dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration2dInferenceText(value: unknown, at = "$"): Generation2dInferenceText {
  return proceduralGeneration2dInferenceTextGuardObject(value, `${at}`);
}
