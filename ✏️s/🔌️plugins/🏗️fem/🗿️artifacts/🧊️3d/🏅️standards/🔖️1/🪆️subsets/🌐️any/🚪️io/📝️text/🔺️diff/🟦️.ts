/** 📝️ Text representation for `fem.fem3d.diff`. */
export type Fem3dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class femFem3dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const femFem3dDiffTextGuardReject = (at: string, why: string): never => {
  throw new femFem3dDiffTextGuardRefusal(at, why);
};

type femFem3dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type femFem3dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type femFem3dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const femFem3dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : femFem3dDiffTextGuardReject(at, "value is not an object");
export const femFem3dDiffTextGuardArray = (value: unknown, at: string, bounds: femFem3dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return femFem3dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) femFem3dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) femFem3dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const femFem3dDiffTextGuardString = (value: unknown, at: string, bounds: femFem3dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return femFem3dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) femFem3dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) femFem3dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) femFem3dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const femFem3dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : femFem3dDiffTextGuardReject(at, "value is not a boolean"));
export const femFem3dDiffTextGuardNumber = (value: unknown, at: string, bounds: femFem3dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return femFem3dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) femFem3dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) femFem3dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const femFem3dDiffTextGuardInteger = (value: unknown, at: string, bounds: femFem3dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? femFem3dDiffTextGuardNumber(value, at, bounds) : femFem3dDiffTextGuardReject(at, "value is not an integer");
export const femFem3dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : femFem3dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const femFem3dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : femFem3dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFem3dDiffText(value: unknown, at = "$"): Fem3dDiffText {
  return femFem3dDiffTextGuardObject(value, `${at}`);
}
