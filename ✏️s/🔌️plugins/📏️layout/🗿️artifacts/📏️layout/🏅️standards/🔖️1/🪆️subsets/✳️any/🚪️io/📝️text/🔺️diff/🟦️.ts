/** 📝️ Text representation for `layout.layout.diff`. */
export type LayoutDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutDiffTextGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutDiffTextGuardRefusal(at, why);
};

type layoutLayoutDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutDiffTextGuardReject(at, "value is not an object");
export const layoutLayoutDiffTextGuardArray = (value: unknown, at: string, bounds: layoutLayoutDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutDiffTextGuardString = (value: unknown, at: string, bounds: layoutLayoutDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutDiffTextGuardReject(at, "value is not a boolean"));
export const layoutLayoutDiffTextGuardNumber = (value: unknown, at: string, bounds: layoutLayoutDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutDiffTextGuardInteger = (value: unknown, at: string, bounds: layoutLayoutDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutDiffTextGuardNumber(value, at, bounds) : layoutLayoutDiffTextGuardReject(at, "value is not an integer");
export const layoutLayoutDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutDiffText(value: unknown, at = "$"): LayoutDiffText {
  return layoutLayoutDiffTextGuardObject(value, `${at}`);
}
