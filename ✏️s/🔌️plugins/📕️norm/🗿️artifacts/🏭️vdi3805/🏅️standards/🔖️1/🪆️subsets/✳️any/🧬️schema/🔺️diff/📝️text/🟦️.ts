/** 📝️ Text representation for `norm.vdi3805.diff`. */
export type Vdi3805DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normVdi3805DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normVdi3805DiffTextGuardReject = (at: string, why: string): never => {
  throw new normVdi3805DiffTextGuardRefusal(at, why);
};

type normVdi3805DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normVdi3805DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normVdi3805DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normVdi3805DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normVdi3805DiffTextGuardReject(at, "value is not an object");
export const normVdi3805DiffTextGuardArray = (value: unknown, at: string, bounds: normVdi3805DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normVdi3805DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normVdi3805DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normVdi3805DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normVdi3805DiffTextGuardString = (value: unknown, at: string, bounds: normVdi3805DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normVdi3805DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normVdi3805DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normVdi3805DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normVdi3805DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normVdi3805DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normVdi3805DiffTextGuardReject(at, "value is not a boolean"));
export const normVdi3805DiffTextGuardNumber = (value: unknown, at: string, bounds: normVdi3805DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normVdi3805DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normVdi3805DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normVdi3805DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normVdi3805DiffTextGuardInteger = (value: unknown, at: string, bounds: normVdi3805DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normVdi3805DiffTextGuardNumber(value, at, bounds) : normVdi3805DiffTextGuardReject(at, "value is not an integer");
export const normVdi3805DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normVdi3805DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normVdi3805DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normVdi3805DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVdi3805DiffText(value: unknown, at = "$"): Vdi3805DiffText {
  return normVdi3805DiffTextGuardObject(value, `${at}`);
}
