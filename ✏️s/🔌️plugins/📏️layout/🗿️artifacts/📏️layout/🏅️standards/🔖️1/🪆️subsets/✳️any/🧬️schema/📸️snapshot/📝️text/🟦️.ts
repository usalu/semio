/** 📝️ Text representation for `layout.layout.snapshot`. */
export type LayoutSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutSnapshotTextGuardRefusal(at, why);
};

type layoutLayoutSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutSnapshotTextGuardReject(at, "value is not an object");
export const layoutLayoutSnapshotTextGuardArray = (value: unknown, at: string, bounds: layoutLayoutSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutSnapshotTextGuardString = (value: unknown, at: string, bounds: layoutLayoutSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutSnapshotTextGuardReject(at, "value is not a boolean"));
export const layoutLayoutSnapshotTextGuardNumber = (value: unknown, at: string, bounds: layoutLayoutSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutSnapshotTextGuardInteger = (value: unknown, at: string, bounds: layoutLayoutSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutSnapshotTextGuardNumber(value, at, bounds) : layoutLayoutSnapshotTextGuardReject(at, "value is not an integer");
export const layoutLayoutSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutSnapshotText(value: unknown, at = "$"): LayoutSnapshotText {
  return layoutLayoutSnapshotTextGuardObject(value, `${at}`);
}
