/** 📝️ Text representation for `process.process3d.diff`. */
export type Process3dDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dDiffTextGuardReject = (at: string, why: string): never => {
  throw new processProcess3dDiffTextGuardRefusal(at, why);
};

type processProcess3dDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dDiffTextGuardReject(at, "value is not an object");
export const processProcess3dDiffTextGuardArray = (value: unknown, at: string, bounds: processProcess3dDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dDiffTextGuardString = (value: unknown, at: string, bounds: processProcess3dDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dDiffTextGuardReject(at, "value is not a boolean"));
export const processProcess3dDiffTextGuardNumber = (value: unknown, at: string, bounds: processProcess3dDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dDiffTextGuardInteger = (value: unknown, at: string, bounds: processProcess3dDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dDiffTextGuardNumber(value, at, bounds) : processProcess3dDiffTextGuardReject(at, "value is not an integer");
export const processProcess3dDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dDiffText(value: unknown, at = "$"): Process3dDiffText {
  return processProcess3dDiffTextGuardObject(value, `${at}`);
}
