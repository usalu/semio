/** 📝️ Text representation for `procedural.generation3d.snapshot`. */
export type Generation3dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dSnapshotTextGuardRefusal(at, why);
};

type proceduralGeneration3dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dSnapshotTextGuardReject(at, "value is not an object");
export const proceduralGeneration3dSnapshotTextGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dSnapshotTextGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dSnapshotTextGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dSnapshotTextGuardNumber(value, at, bounds) : proceduralGeneration3dSnapshotTextGuardReject(at, "value is not an integer");
export const proceduralGeneration3dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dSnapshotText(value: unknown, at = "$"): Generation3dSnapshotText {
  return proceduralGeneration3dSnapshotTextGuardObject(value, `${at}`);
}
