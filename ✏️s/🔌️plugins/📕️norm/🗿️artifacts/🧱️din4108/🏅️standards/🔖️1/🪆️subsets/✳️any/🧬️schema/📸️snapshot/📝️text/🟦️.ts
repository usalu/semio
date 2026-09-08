/** 📝️ Text representation for `norm.din4108.snapshot`. */
export type Din4108SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin4108SnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin4108SnapshotTextGuardReject = (at: string, why: string): never => {
  throw new normDin4108SnapshotTextGuardRefusal(at, why);
};

type normDin4108SnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin4108SnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin4108SnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin4108SnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin4108SnapshotTextGuardReject(at, "value is not an object");
export const normDin4108SnapshotTextGuardArray = (value: unknown, at: string, bounds: normDin4108SnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin4108SnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin4108SnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin4108SnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin4108SnapshotTextGuardString = (value: unknown, at: string, bounds: normDin4108SnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin4108SnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin4108SnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin4108SnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin4108SnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin4108SnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin4108SnapshotTextGuardReject(at, "value is not a boolean"));
export const normDin4108SnapshotTextGuardNumber = (value: unknown, at: string, bounds: normDin4108SnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin4108SnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin4108SnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin4108SnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin4108SnapshotTextGuardInteger = (value: unknown, at: string, bounds: normDin4108SnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin4108SnapshotTextGuardNumber(value, at, bounds) : normDin4108SnapshotTextGuardReject(at, "value is not an integer");
export const normDin4108SnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin4108SnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin4108SnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin4108SnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin4108SnapshotText(value: unknown, at = "$"): Din4108SnapshotText {
  return normDin4108SnapshotTextGuardObject(value, `${at}`);
}
