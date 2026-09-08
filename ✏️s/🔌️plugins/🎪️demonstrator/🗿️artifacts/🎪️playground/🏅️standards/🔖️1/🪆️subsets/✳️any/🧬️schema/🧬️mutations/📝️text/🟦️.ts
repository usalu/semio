/** 📝️ Text representation for `demonstrator.playground.mutations`. */
export type PlaygroundMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundMutationsTextGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundMutationsTextGuardRefusal(at, why);
};

type demonstratorPlaygroundMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundMutationsTextGuardReject(at, "value is not an object");
export const demonstratorPlaygroundMutationsTextGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundMutationsTextGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundMutationsTextGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundMutationsTextGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundMutationsTextGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundMutationsTextGuardNumber(value, at, bounds) : demonstratorPlaygroundMutationsTextGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundMutationsText(value: unknown, at = "$"): PlaygroundMutationsText {
  return demonstratorPlaygroundMutationsTextGuardObject(value, `${at}`);
}
