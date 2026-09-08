/** 📝️ Text representation for `norm.din16798.snapshot`. */
export type Din16798SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin16798SnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin16798SnapshotTextGuardReject = (at: string, why: string): never => {
  throw new normDin16798SnapshotTextGuardRefusal(at, why);
};

type normDin16798SnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin16798SnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin16798SnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin16798SnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin16798SnapshotTextGuardReject(at, "value is not an object");
export const normDin16798SnapshotTextGuardArray = (value: unknown, at: string, bounds: normDin16798SnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin16798SnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin16798SnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin16798SnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin16798SnapshotTextGuardString = (value: unknown, at: string, bounds: normDin16798SnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin16798SnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin16798SnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin16798SnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin16798SnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin16798SnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin16798SnapshotTextGuardReject(at, "value is not a boolean"));
export const normDin16798SnapshotTextGuardNumber = (value: unknown, at: string, bounds: normDin16798SnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin16798SnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin16798SnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin16798SnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin16798SnapshotTextGuardInteger = (value: unknown, at: string, bounds: normDin16798SnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin16798SnapshotTextGuardNumber(value, at, bounds) : normDin16798SnapshotTextGuardReject(at, "value is not an integer");
export const normDin16798SnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin16798SnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin16798SnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin16798SnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin16798SnapshotText(value: unknown, at = "$"): Din16798SnapshotText {
  return normDin16798SnapshotTextGuardObject(value, `${at}`);
}
