/** 📝️ Text representation for `fem.fem2d.snapshot`. */
export type Fem2dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new femFem2dSnapshotTextGuardRefusal(at, why);
};

type femFem2dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dSnapshotTextGuardReject(at, "value is not an object");
export const femFem2dSnapshotTextGuardArray = (value: unknown, at: string, bounds: femFem2dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dSnapshotTextGuardString = (value: unknown, at: string, bounds: femFem2dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dSnapshotTextGuardReject(at, "value is not a boolean"));
export const femFem2dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: femFem2dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: femFem2dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dSnapshotTextGuardNumber(value, at, bounds) : femFem2dSnapshotTextGuardReject(at, "value is not an integer");
export const femFem2dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dSnapshotText(value: unknown, at = "$"): Fem2dSnapshotText {
  return femFem2dSnapshotTextGuardObject(value, `${at}`);
}
