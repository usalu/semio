/** 📝️ Text representation for `shooting.shooting.mutations`. */
export type ShootingMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingMutationsTextGuardReject = (at: string, why: string): never => {
  throw new shootingShootingMutationsTextGuardRefusal(at, why);
};

type shootingShootingMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingMutationsTextGuardReject(at, "value is not an object");
export const shootingShootingMutationsTextGuardArray = (value: unknown, at: string, bounds: shootingShootingMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingMutationsTextGuardString = (value: unknown, at: string, bounds: shootingShootingMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingMutationsTextGuardReject(at, "value is not a boolean"));
export const shootingShootingMutationsTextGuardNumber = (value: unknown, at: string, bounds: shootingShootingMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingMutationsTextGuardInteger = (value: unknown, at: string, bounds: shootingShootingMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingMutationsTextGuardNumber(value, at, bounds) : shootingShootingMutationsTextGuardReject(at, "value is not an integer");
export const shootingShootingMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingMutationsText(value: unknown, at = "$"): ShootingMutationsText {
  return shootingShootingMutationsTextGuardObject(value, `${at}`);
}
