/** 📝️ Text representation for `fem.fem3d.snapshot`. */
export type Fem3dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new femFem3dSnapshotTextGuardRefusal(at, why);
};

type femFem3dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dSnapshotTextGuardReject(at, "value is not an object");
export const femFem3dSnapshotTextGuardArray = (value: unknown, at: string, bounds: femFem3dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dSnapshotTextGuardString = (value: unknown, at: string, bounds: femFem3dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dSnapshotTextGuardReject(at, "value is not a boolean"));
export const femFem3dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: femFem3dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: femFem3dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dSnapshotTextGuardNumber(value, at, bounds) : femFem3dSnapshotTextGuardReject(at, "value is not an integer");
export const femFem3dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dSnapshotText(value: unknown, at = "$"): Fem3dSnapshotText {
  return femFem3dSnapshotTextGuardObject(value, `${at}`);
}
