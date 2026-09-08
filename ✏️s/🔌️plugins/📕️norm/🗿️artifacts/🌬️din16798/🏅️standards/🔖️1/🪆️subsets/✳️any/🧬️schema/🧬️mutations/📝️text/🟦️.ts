/** 📝️ Text representation for `norm.din16798.mutations`. */
export type Din16798MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin16798MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin16798MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normDin16798MutationsTextGuardRefusal(at, why);
};

type normDin16798MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin16798MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin16798MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin16798MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin16798MutationsTextGuardReject(at, "value is not an object");
export const normDin16798MutationsTextGuardArray = (value: unknown, at: string, bounds: normDin16798MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin16798MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin16798MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin16798MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin16798MutationsTextGuardString = (value: unknown, at: string, bounds: normDin16798MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin16798MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin16798MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin16798MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin16798MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin16798MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin16798MutationsTextGuardReject(at, "value is not a boolean"));
export const normDin16798MutationsTextGuardNumber = (value: unknown, at: string, bounds: normDin16798MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin16798MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin16798MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin16798MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin16798MutationsTextGuardInteger = (value: unknown, at: string, bounds: normDin16798MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin16798MutationsTextGuardNumber(value, at, bounds) : normDin16798MutationsTextGuardReject(at, "value is not an integer");
export const normDin16798MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin16798MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin16798MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin16798MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin16798MutationsText(value: unknown, at = "$"): Din16798MutationsText {
  return normDin16798MutationsTextGuardObject(value, `${at}`);
}
