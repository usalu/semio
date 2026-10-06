/** 📝️ Text representation for `shooting.shooting.inference`. */
export type ShootingInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class shootingShootingInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const shootingShootingInferenceTextGuardReject = (at: string, why: string): never => {
  throw new shootingShootingInferenceTextGuardRefusal(at, why);
};

type shootingShootingInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type shootingShootingInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type shootingShootingInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const shootingShootingInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : shootingShootingInferenceTextGuardReject(at, "value is not an object");
export const shootingShootingInferenceTextGuardArray = (value: unknown, at: string, bounds: shootingShootingInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return shootingShootingInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) shootingShootingInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) shootingShootingInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const shootingShootingInferenceTextGuardString = (value: unknown, at: string, bounds: shootingShootingInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return shootingShootingInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) shootingShootingInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) shootingShootingInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) shootingShootingInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const shootingShootingInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : shootingShootingInferenceTextGuardReject(at, "value is not a boolean"));
export const shootingShootingInferenceTextGuardNumber = (value: unknown, at: string, bounds: shootingShootingInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return shootingShootingInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) shootingShootingInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) shootingShootingInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const shootingShootingInferenceTextGuardInteger = (value: unknown, at: string, bounds: shootingShootingInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? shootingShootingInferenceTextGuardNumber(value, at, bounds) : shootingShootingInferenceTextGuardReject(at, "value is not an integer");
export const shootingShootingInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : shootingShootingInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const shootingShootingInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : shootingShootingInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseShootingInferenceText(value: unknown, at = "$"): ShootingInferenceText {
  return shootingShootingInferenceTextGuardObject(value, `${at}`);
}
