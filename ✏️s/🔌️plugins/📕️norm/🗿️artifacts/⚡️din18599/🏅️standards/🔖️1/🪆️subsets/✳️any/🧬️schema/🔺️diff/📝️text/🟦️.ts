/** 📝️ Text representation for `norm.din18599.diff`. */
export type Din18599DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin18599DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin18599DiffTextGuardReject = (at: string, why: string): never => {
  throw new normDin18599DiffTextGuardRefusal(at, why);
};

type normDin18599DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin18599DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin18599DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin18599DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin18599DiffTextGuardReject(at, "value is not an object");
export const normDin18599DiffTextGuardArray = (value: unknown, at: string, bounds: normDin18599DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin18599DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin18599DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin18599DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin18599DiffTextGuardString = (value: unknown, at: string, bounds: normDin18599DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin18599DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin18599DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin18599DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin18599DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin18599DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin18599DiffTextGuardReject(at, "value is not a boolean"));
export const normDin18599DiffTextGuardNumber = (value: unknown, at: string, bounds: normDin18599DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin18599DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin18599DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin18599DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin18599DiffTextGuardInteger = (value: unknown, at: string, bounds: normDin18599DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin18599DiffTextGuardNumber(value, at, bounds) : normDin18599DiffTextGuardReject(at, "value is not an integer");
export const normDin18599DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin18599DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin18599DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin18599DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin18599DiffText(value: unknown, at = "$"): Din18599DiffText {
  return normDin18599DiffTextGuardObject(value, `${at}`);
}
