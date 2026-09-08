/** 📝️ Text representation for `process.process3d.snapshot`. */
export type Process3dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new processProcess3dSnapshotTextGuardRefusal(at, why);
};

type processProcess3dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dSnapshotTextGuardReject(at, "value is not an object");
export const processProcess3dSnapshotTextGuardArray = (value: unknown, at: string, bounds: processProcess3dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dSnapshotTextGuardString = (value: unknown, at: string, bounds: processProcess3dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dSnapshotTextGuardReject(at, "value is not a boolean"));
export const processProcess3dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: processProcess3dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: processProcess3dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dSnapshotTextGuardNumber(value, at, bounds) : processProcess3dSnapshotTextGuardReject(at, "value is not an integer");
export const processProcess3dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dSnapshotText(value: unknown, at = "$"): Process3dSnapshotText {
  return processProcess3dSnapshotTextGuardObject(value, `${at}`);
}
