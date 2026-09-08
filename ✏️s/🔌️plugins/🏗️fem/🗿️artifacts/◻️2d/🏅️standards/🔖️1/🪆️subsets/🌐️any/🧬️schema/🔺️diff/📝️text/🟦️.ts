/** 📝️ Text representation for `fem.fem2d.diff`. */
export type Fem2dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dDiffTextGuardReject = (at: string, why: string): never => {
  throw new femFem2dDiffTextGuardRefusal(at, why);
};

type femFem2dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dDiffTextGuardReject(at, "value is not an object");
export const femFem2dDiffTextGuardArray = (value: unknown, at: string, bounds: femFem2dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dDiffTextGuardString = (value: unknown, at: string, bounds: femFem2dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dDiffTextGuardReject(at, "value is not a boolean"));
export const femFem2dDiffTextGuardNumber = (value: unknown, at: string, bounds: femFem2dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dDiffTextGuardInteger = (value: unknown, at: string, bounds: femFem2dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dDiffTextGuardNumber(value, at, bounds) : femFem2dDiffTextGuardReject(at, "value is not an integer");
export const femFem2dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dDiffText(value: unknown, at = "$"): Fem2dDiffText {
  return femFem2dDiffTextGuardObject(value, `${at}`);
}
