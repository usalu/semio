/** 📝️ Text representation for `procedural.generation3d.mutations`. */
export type Generation3dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dMutationsTextGuardRefusal(at, why);
};

type proceduralGeneration3dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dMutationsTextGuardReject(at, "value is not an object");
export const proceduralGeneration3dMutationsTextGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dMutationsTextGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dMutationsTextGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dMutationsTextGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dMutationsTextGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dMutationsTextGuardNumber(value, at, bounds) : proceduralGeneration3dMutationsTextGuardReject(at, "value is not an integer");
export const proceduralGeneration3dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dMutationsText(value: unknown, at = "$"): Generation3dMutationsText {
  return proceduralGeneration3dMutationsTextGuardObject(value, `${at}`);
}
