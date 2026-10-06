/** 📝️ Text representation for `fem.fem2d.mutations`. */
export type Fem2dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem2dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem2dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new femFem2dMutationsTextGuardRefusal(at, why);
};

type femFem2dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem2dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem2dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem2dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem2dMutationsTextGuardReject(at, "value is not an object");
export const femFem2dMutationsTextGuardArray = (value: unknown, at: string, bounds: femFem2dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem2dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem2dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem2dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem2dMutationsTextGuardString = (value: unknown, at: string, bounds: femFem2dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem2dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem2dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem2dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem2dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem2dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem2dMutationsTextGuardReject(at, "value is not a boolean"));
export const femFem2dMutationsTextGuardNumber = (value: unknown, at: string, bounds: femFem2dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem2dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem2dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem2dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem2dMutationsTextGuardInteger = (value: unknown, at: string, bounds: femFem2dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem2dMutationsTextGuardNumber(value, at, bounds) : femFem2dMutationsTextGuardReject(at, "value is not an integer");
export const femFem2dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem2dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem2dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem2dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem2dMutationsText(value: unknown, at = "$"): Fem2dMutationsText {
  return femFem2dMutationsTextGuardObject(value, `${at}`);
}
