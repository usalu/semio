/** 📝️ Text representation for `norm.vdi3805.mutations`. */
export type Vdi3805MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normVdi3805MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normVdi3805MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normVdi3805MutationsTextGuardRefusal(at, why);
};

type normVdi3805MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normVdi3805MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normVdi3805MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normVdi3805MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normVdi3805MutationsTextGuardReject(at, "value is not an object");
export const normVdi3805MutationsTextGuardArray = (value: unknown, at: string, bounds: normVdi3805MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normVdi3805MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normVdi3805MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normVdi3805MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normVdi3805MutationsTextGuardString = (value: unknown, at: string, bounds: normVdi3805MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normVdi3805MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normVdi3805MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normVdi3805MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normVdi3805MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normVdi3805MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normVdi3805MutationsTextGuardReject(at, "value is not a boolean"));
export const normVdi3805MutationsTextGuardNumber = (value: unknown, at: string, bounds: normVdi3805MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normVdi3805MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normVdi3805MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normVdi3805MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normVdi3805MutationsTextGuardInteger = (value: unknown, at: string, bounds: normVdi3805MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normVdi3805MutationsTextGuardNumber(value, at, bounds) : normVdi3805MutationsTextGuardReject(at, "value is not an integer");
export const normVdi3805MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normVdi3805MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normVdi3805MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normVdi3805MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVdi3805MutationsText(value: unknown, at = "$"): Vdi3805MutationsText {
  return normVdi3805MutationsTextGuardObject(value, `${at}`);
}
