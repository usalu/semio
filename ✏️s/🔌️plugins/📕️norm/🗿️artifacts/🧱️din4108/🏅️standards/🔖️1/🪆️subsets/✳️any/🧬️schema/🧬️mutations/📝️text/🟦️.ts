/** 📝️ Text representation for `norm.din4108.mutations`. */
export type Din4108MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin4108MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin4108MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normDin4108MutationsTextGuardRefusal(at, why);
};

type normDin4108MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin4108MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin4108MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin4108MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin4108MutationsTextGuardReject(at, "value is not an object");
export const normDin4108MutationsTextGuardArray = (value: unknown, at: string, bounds: normDin4108MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin4108MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin4108MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin4108MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin4108MutationsTextGuardString = (value: unknown, at: string, bounds: normDin4108MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin4108MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin4108MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin4108MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin4108MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin4108MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin4108MutationsTextGuardReject(at, "value is not a boolean"));
export const normDin4108MutationsTextGuardNumber = (value: unknown, at: string, bounds: normDin4108MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin4108MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin4108MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin4108MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin4108MutationsTextGuardInteger = (value: unknown, at: string, bounds: normDin4108MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin4108MutationsTextGuardNumber(value, at, bounds) : normDin4108MutationsTextGuardReject(at, "value is not an integer");
export const normDin4108MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin4108MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin4108MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin4108MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin4108MutationsText(value: unknown, at = "$"): Din4108MutationsText {
  return normDin4108MutationsTextGuardObject(value, `${at}`);
}
