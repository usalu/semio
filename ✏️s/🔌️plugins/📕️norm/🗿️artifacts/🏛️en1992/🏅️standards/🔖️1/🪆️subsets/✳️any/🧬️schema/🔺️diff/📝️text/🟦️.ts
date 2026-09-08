/** 📝️ Text representation for `norm.en1992.diff`. */
export type En1992DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1992DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1992DiffTextGuardReject = (at: string, why: string): never => {
  throw new normEn1992DiffTextGuardRefusal(at, why);
};

type normEn1992DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1992DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1992DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1992DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1992DiffTextGuardReject(at, "value is not an object");
export const normEn1992DiffTextGuardArray = (value: unknown, at: string, bounds: normEn1992DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1992DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1992DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1992DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1992DiffTextGuardString = (value: unknown, at: string, bounds: normEn1992DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1992DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1992DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1992DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1992DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1992DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1992DiffTextGuardReject(at, "value is not a boolean"));
export const normEn1992DiffTextGuardNumber = (value: unknown, at: string, bounds: normEn1992DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1992DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1992DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1992DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1992DiffTextGuardInteger = (value: unknown, at: string, bounds: normEn1992DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1992DiffTextGuardNumber(value, at, bounds) : normEn1992DiffTextGuardReject(at, "value is not an integer");
export const normEn1992DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1992DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1992DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1992DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1992DiffText(value: unknown, at = "$"): En1992DiffText {
  return normEn1992DiffTextGuardObject(value, `${at}`);
}
