/** 📝️ Text representation for `process.process3d.mutations`. */
export type Process3dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class processProcess3dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const processProcess3dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new processProcess3dMutationsTextGuardRefusal(at, why);
};

type processProcess3dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type processProcess3dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type processProcess3dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const processProcess3dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : processProcess3dMutationsTextGuardReject(at, "value is not an object");
export const processProcess3dMutationsTextGuardArray = (value: unknown, at: string, bounds: processProcess3dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return processProcess3dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) processProcess3dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) processProcess3dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const processProcess3dMutationsTextGuardString = (value: unknown, at: string, bounds: processProcess3dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return processProcess3dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) processProcess3dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) processProcess3dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) processProcess3dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const processProcess3dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : processProcess3dMutationsTextGuardReject(at, "value is not a boolean"));
export const processProcess3dMutationsTextGuardNumber = (value: unknown, at: string, bounds: processProcess3dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return processProcess3dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) processProcess3dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) processProcess3dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const processProcess3dMutationsTextGuardInteger = (value: unknown, at: string, bounds: processProcess3dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? processProcess3dMutationsTextGuardNumber(value, at, bounds) : processProcess3dMutationsTextGuardReject(at, "value is not an integer");
export const processProcess3dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : processProcess3dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const processProcess3dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : processProcess3dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProcess3dMutationsText(value: unknown, at = "$"): Process3dMutationsText {
  return processProcess3dMutationsTextGuardObject(value, `${at}`);
}
