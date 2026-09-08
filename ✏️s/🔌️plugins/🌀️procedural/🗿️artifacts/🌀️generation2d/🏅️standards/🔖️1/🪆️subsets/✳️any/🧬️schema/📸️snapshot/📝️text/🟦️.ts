/** 📝️ Text representation for `procedural.generation2d.snapshot`. */
export type Generation2dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration2dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration2dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration2dSnapshotTextGuardRefusal(at, why);
};

type proceduralGeneration2dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration2dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration2dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration2dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration2dSnapshotTextGuardReject(at, "value is not an object");
export const proceduralGeneration2dSnapshotTextGuardArray = (value: unknown, at: string, bounds: proceduralGeneration2dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration2dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration2dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration2dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration2dSnapshotTextGuardString = (value: unknown, at: string, bounds: proceduralGeneration2dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration2dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration2dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration2dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration2dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration2dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration2dSnapshotTextGuardReject(at, "value is not a boolean"));
export const proceduralGeneration2dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration2dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration2dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration2dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration2dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration2dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration2dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration2dSnapshotTextGuardNumber(value, at, bounds) : proceduralGeneration2dSnapshotTextGuardReject(at, "value is not an integer");
export const proceduralGeneration2dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration2dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration2dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration2dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration2dSnapshotText(value: unknown, at = "$"): Generation2dSnapshotText {
  return proceduralGeneration2dSnapshotTextGuardObject(value, `${at}`);
}
