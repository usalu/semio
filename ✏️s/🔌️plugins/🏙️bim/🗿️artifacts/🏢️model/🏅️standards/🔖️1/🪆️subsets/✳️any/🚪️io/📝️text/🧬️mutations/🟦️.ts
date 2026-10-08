/** 📝️ Text representation for `bim.model.mutations`. */
export type ModelMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelMutationsTextGuardReject = (at: string, why: string): never => {
  throw new bimModelMutationsTextGuardRefusal(at, why);
};

type bimModelMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type bimModelMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelMutationsTextGuardReject(at, "value is not an object");
export const bimModelMutationsTextGuardArray = (value: unknown, at: string, bounds: bimModelMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelMutationsTextGuardString = (value: unknown, at: string, bounds: bimModelMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelMutationsTextGuardReject(at, "value is not a boolean"));
export const bimModelMutationsTextGuardNumber = (value: unknown, at: string, bounds: bimModelMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const bimModelMutationsTextGuardInteger = (value: unknown, at: string, bounds: bimModelMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelMutationsTextGuardNumber(value, at, bounds) : bimModelMutationsTextGuardReject(at, "value is not an integer");
export const bimModelMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseModelMutationsText(value: unknown, at = "$"): ModelMutationsText {
  return bimModelMutationsTextGuardObject(value, `${at}`);
}
