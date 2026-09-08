/** 📝️ Text representation for `norm.iso16757.mutations`. */
export type Iso16757MutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normIso16757MutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normIso16757MutationsTextGuardReject = (at: string, why: string): never => {
  throw new normIso16757MutationsTextGuardRefusal(at, why);
};

type normIso16757MutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normIso16757MutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normIso16757MutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normIso16757MutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normIso16757MutationsTextGuardReject(at, "value is not an object");
export const normIso16757MutationsTextGuardArray = (value: unknown, at: string, bounds: normIso16757MutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normIso16757MutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normIso16757MutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normIso16757MutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normIso16757MutationsTextGuardString = (value: unknown, at: string, bounds: normIso16757MutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normIso16757MutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normIso16757MutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normIso16757MutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normIso16757MutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normIso16757MutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normIso16757MutationsTextGuardReject(at, "value is not a boolean"));
export const normIso16757MutationsTextGuardNumber = (value: unknown, at: string, bounds: normIso16757MutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normIso16757MutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normIso16757MutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normIso16757MutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normIso16757MutationsTextGuardInteger = (value: unknown, at: string, bounds: normIso16757MutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normIso16757MutationsTextGuardNumber(value, at, bounds) : normIso16757MutationsTextGuardReject(at, "value is not an integer");
export const normIso16757MutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normIso16757MutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normIso16757MutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normIso16757MutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIso16757MutationsText(value: unknown, at = "$"): Iso16757MutationsText {
  return normIso16757MutationsTextGuardObject(value, `${at}`);
}
