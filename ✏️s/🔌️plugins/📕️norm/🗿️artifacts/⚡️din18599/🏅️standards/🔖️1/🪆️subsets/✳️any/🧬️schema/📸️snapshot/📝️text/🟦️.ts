/** 📝️ Text representation for `norm.din18599.snapshot`. */
export type Din18599SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin18599SnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin18599SnapshotTextGuardReject = (at: string, why: string): never => {
  throw new normDin18599SnapshotTextGuardRefusal(at, why);
};

type normDin18599SnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin18599SnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin18599SnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin18599SnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin18599SnapshotTextGuardReject(at, "value is not an object");
export const normDin18599SnapshotTextGuardArray = (value: unknown, at: string, bounds: normDin18599SnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin18599SnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin18599SnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin18599SnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin18599SnapshotTextGuardString = (value: unknown, at: string, bounds: normDin18599SnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin18599SnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin18599SnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin18599SnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin18599SnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin18599SnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin18599SnapshotTextGuardReject(at, "value is not a boolean"));
export const normDin18599SnapshotTextGuardNumber = (value: unknown, at: string, bounds: normDin18599SnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin18599SnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin18599SnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin18599SnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin18599SnapshotTextGuardInteger = (value: unknown, at: string, bounds: normDin18599SnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin18599SnapshotTextGuardNumber(value, at, bounds) : normDin18599SnapshotTextGuardReject(at, "value is not an integer");
export const normDin18599SnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin18599SnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin18599SnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin18599SnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin18599SnapshotText(value: unknown, at = "$"): Din18599SnapshotText {
  return normDin18599SnapshotTextGuardObject(value, `${at}`);
}
