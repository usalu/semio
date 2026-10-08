/** 📝️ Text representation for `bim.model.diff`. */
export type ModelDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelDiffTextGuardReject = (at: string, why: string): never => {
  throw new bimModelDiffTextGuardRefusal(at, why);
};

type bimModelDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type bimModelDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelDiffTextGuardReject(at, "value is not an object");
export const bimModelDiffTextGuardArray = (value: unknown, at: string, bounds: bimModelDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelDiffTextGuardString = (value: unknown, at: string, bounds: bimModelDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelDiffTextGuardReject(at, "value is not a boolean"));
export const bimModelDiffTextGuardNumber = (value: unknown, at: string, bounds: bimModelDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const bimModelDiffTextGuardInteger = (value: unknown, at: string, bounds: bimModelDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelDiffTextGuardNumber(value, at, bounds) : bimModelDiffTextGuardReject(at, "value is not an integer");
export const bimModelDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseModelDiffText(value: unknown, at = "$"): ModelDiffText {
  return bimModelDiffTextGuardObject(value, `${at}`);
}
