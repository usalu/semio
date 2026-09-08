/** 📝️ Text representation for `demonstrator.playground.snapshot`. */
export type PlaygroundSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundSnapshotTextGuardRefusal(at, why);
};

type demonstratorPlaygroundSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundSnapshotTextGuardReject(at, "value is not an object");
export const demonstratorPlaygroundSnapshotTextGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundSnapshotTextGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundSnapshotTextGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundSnapshotTextGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundSnapshotTextGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundSnapshotTextGuardNumber(value, at, bounds) : demonstratorPlaygroundSnapshotTextGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundSnapshotText(value: unknown, at = "$"): PlaygroundSnapshotText {
  return demonstratorPlaygroundSnapshotTextGuardObject(value, `${at}`);
}
