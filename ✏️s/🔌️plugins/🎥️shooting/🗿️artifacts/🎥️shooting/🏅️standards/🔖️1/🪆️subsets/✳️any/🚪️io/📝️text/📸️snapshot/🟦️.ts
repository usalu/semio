/** 📝️ Text representation for `shooting.shooting.snapshot`. */
export type ShootingSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new shootingShootingSnapshotTextGuardRefusal(at, why);
};

type shootingShootingSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingSnapshotTextGuardReject(at, "value is not an object");
export const shootingShootingSnapshotTextGuardArray = (value: unknown, at: string, bounds: shootingShootingSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingSnapshotTextGuardString = (value: unknown, at: string, bounds: shootingShootingSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingSnapshotTextGuardReject(at, "value is not a boolean"));
export const shootingShootingSnapshotTextGuardNumber = (value: unknown, at: string, bounds: shootingShootingSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingSnapshotTextGuardInteger = (value: unknown, at: string, bounds: shootingShootingSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingSnapshotTextGuardNumber(value, at, bounds) : shootingShootingSnapshotTextGuardReject(at, "value is not an integer");
export const shootingShootingSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingSnapshotText(value: unknown, at = "$"): ShootingSnapshotText {
  return shootingShootingSnapshotTextGuardObject(value, `${at}`);
}
