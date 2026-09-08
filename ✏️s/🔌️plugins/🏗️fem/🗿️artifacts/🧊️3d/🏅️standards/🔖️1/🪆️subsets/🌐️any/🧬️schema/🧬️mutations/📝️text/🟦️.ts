/** 📝️ Text representation for `fem.fem3d.mutations`. */
export type Fem3dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new femFem3dMutationsTextGuardRefusal(at, why);
};

type femFem3dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dMutationsTextGuardReject(at, "value is not an object");
export const femFem3dMutationsTextGuardArray = (value: unknown, at: string, bounds: femFem3dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dMutationsTextGuardString = (value: unknown, at: string, bounds: femFem3dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dMutationsTextGuardReject(at, "value is not a boolean"));
export const femFem3dMutationsTextGuardNumber = (value: unknown, at: string, bounds: femFem3dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dMutationsTextGuardInteger = (value: unknown, at: string, bounds: femFem3dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dMutationsTextGuardNumber(value, at, bounds) : femFem3dMutationsTextGuardReject(at, "value is not an integer");
export const femFem3dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dMutationsText(value: unknown, at = "$"): Fem3dMutationsText {
  return femFem3dMutationsTextGuardObject(value, `${at}`);
}
