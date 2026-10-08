/** 📝️ Text representation for `bim.model.snapshot`. */
export type ModelSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new bimModelSnapshotTextGuardRefusal(at, why);
};

type bimModelSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type bimModelSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelSnapshotTextGuardReject(at, "value is not an object");
export const bimModelSnapshotTextGuardArray = (value: unknown, at: string, bounds: bimModelSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelSnapshotTextGuardString = (value: unknown, at: string, bounds: bimModelSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelSnapshotTextGuardReject(at, "value is not a boolean"));
export const bimModelSnapshotTextGuardNumber = (value: unknown, at: string, bounds: bimModelSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const bimModelSnapshotTextGuardInteger = (value: unknown, at: string, bounds: bimModelSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelSnapshotTextGuardNumber(value, at, bounds) : bimModelSnapshotTextGuardReject(at, "value is not an integer");
export const bimModelSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseModelSnapshotText(value: unknown, at = "$"): ModelSnapshotText {
  return bimModelSnapshotTextGuardObject(value, `${at}`);
}
