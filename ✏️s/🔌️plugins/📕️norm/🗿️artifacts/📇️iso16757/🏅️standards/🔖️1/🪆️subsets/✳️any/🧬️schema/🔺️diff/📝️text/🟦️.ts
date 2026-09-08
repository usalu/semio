/** 📝️ Text representation for `norm.iso16757.diff`. */
export type Iso16757DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normIso16757DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normIso16757DiffTextGuardReject = (at: string, why: string): never => {
  throw new normIso16757DiffTextGuardRefusal(at, why);
};

type normIso16757DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normIso16757DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normIso16757DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normIso16757DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normIso16757DiffTextGuardReject(at, "value is not an object");
export const normIso16757DiffTextGuardArray = (value: unknown, at: string, bounds: normIso16757DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normIso16757DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normIso16757DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normIso16757DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normIso16757DiffTextGuardString = (value: unknown, at: string, bounds: normIso16757DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normIso16757DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normIso16757DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normIso16757DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normIso16757DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normIso16757DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normIso16757DiffTextGuardReject(at, "value is not a boolean"));
export const normIso16757DiffTextGuardNumber = (value: unknown, at: string, bounds: normIso16757DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normIso16757DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normIso16757DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normIso16757DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normIso16757DiffTextGuardInteger = (value: unknown, at: string, bounds: normIso16757DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normIso16757DiffTextGuardNumber(value, at, bounds) : normIso16757DiffTextGuardReject(at, "value is not an integer");
export const normIso16757DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normIso16757DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normIso16757DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normIso16757DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIso16757DiffText(value: unknown, at = "$"): Iso16757DiffText {
  return normIso16757DiffTextGuardObject(value, `${at}`);
}
