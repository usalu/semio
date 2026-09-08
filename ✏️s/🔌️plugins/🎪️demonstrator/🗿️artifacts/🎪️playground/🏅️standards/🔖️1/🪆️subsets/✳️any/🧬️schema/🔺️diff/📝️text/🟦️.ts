/** 📝️ Text representation for `demonstrator.playground.diff`. */
export type PlaygroundDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundDiffTextGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundDiffTextGuardRefusal(at, why);
};

type demonstratorPlaygroundDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundDiffTextGuardReject(at, "value is not an object");
export const demonstratorPlaygroundDiffTextGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundDiffTextGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundDiffTextGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundDiffTextGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundDiffTextGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundDiffTextGuardNumber(value, at, bounds) : demonstratorPlaygroundDiffTextGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundDiffText(value: unknown, at = "$"): PlaygroundDiffText {
  return demonstratorPlaygroundDiffTextGuardObject(value, `${at}`);
}
