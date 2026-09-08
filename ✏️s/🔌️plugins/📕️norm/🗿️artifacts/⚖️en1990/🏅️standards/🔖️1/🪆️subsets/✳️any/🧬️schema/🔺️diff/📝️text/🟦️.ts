/** 📝️ Text representation for `norm.en1990.diff`. */
export type En1990DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1990DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1990DiffTextGuardReject = (at: string, why: string): never => {
  throw new normEn1990DiffTextGuardRefusal(at, why);
};

type normEn1990DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1990DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1990DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1990DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1990DiffTextGuardReject(at, "value is not an object");
export const normEn1990DiffTextGuardArray = (value: unknown, at: string, bounds: normEn1990DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1990DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1990DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1990DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1990DiffTextGuardString = (value: unknown, at: string, bounds: normEn1990DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1990DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1990DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1990DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1990DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1990DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1990DiffTextGuardReject(at, "value is not a boolean"));
export const normEn1990DiffTextGuardNumber = (value: unknown, at: string, bounds: normEn1990DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1990DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1990DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1990DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1990DiffTextGuardInteger = (value: unknown, at: string, bounds: normEn1990DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1990DiffTextGuardNumber(value, at, bounds) : normEn1990DiffTextGuardReject(at, "value is not an integer");
export const normEn1990DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1990DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1990DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1990DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1990DiffText(value: unknown, at = "$"): En1990DiffText {
  return normEn1990DiffTextGuardObject(value, `${at}`);
}
