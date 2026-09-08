/** 📝️ Text representation for `procedural.generation3d.diff`. */
export type Generation3dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dDiffTextGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dDiffTextGuardRefusal(at, why);
};

type proceduralGeneration3dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dDiffTextGuardReject(at, "value is not an object");
export const proceduralGeneration3dDiffTextGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dDiffTextGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dDiffTextGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dDiffTextGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dDiffTextGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dDiffTextGuardNumber(value, at, bounds) : proceduralGeneration3dDiffTextGuardReject(at, "value is not an integer");
export const proceduralGeneration3dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dDiffText(value: unknown, at = "$"): Generation3dDiffText {
  return proceduralGeneration3dDiffTextGuardObject(value, `${at}`);
}
