/** 📝️ Text representation for `shooting.shooting.diff`. */
export type ShootingDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingDiffTextGuardReject = (at: string, why: string): never => {
  throw new shootingShootingDiffTextGuardRefusal(at, why);
};

type shootingShootingDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingDiffTextGuardReject(at, "value is not an object");
export const shootingShootingDiffTextGuardArray = (value: unknown, at: string, bounds: shootingShootingDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingDiffTextGuardString = (value: unknown, at: string, bounds: shootingShootingDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingDiffTextGuardReject(at, "value is not a boolean"));
export const shootingShootingDiffTextGuardNumber = (value: unknown, at: string, bounds: shootingShootingDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingDiffTextGuardInteger = (value: unknown, at: string, bounds: shootingShootingDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingDiffTextGuardNumber(value, at, bounds) : shootingShootingDiffTextGuardReject(at, "value is not an integer");
export const shootingShootingDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingDiffText(value: unknown, at = "$"): ShootingDiffText {
  return shootingShootingDiffTextGuardObject(value, `${at}`);
}
