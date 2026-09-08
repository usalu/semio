/** 📝️ Text representation for `layout.layout.inference`. */
export type LayoutInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class layoutLayoutInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const layoutLayoutInferenceTextGuardReject = (at: string, why: string): never => {
  throw new layoutLayoutInferenceTextGuardRefusal(at, why);
};

type layoutLayoutInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type layoutLayoutInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type layoutLayoutInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const layoutLayoutInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : layoutLayoutInferenceTextGuardReject(at, "value is not an object");
export const layoutLayoutInferenceTextGuardArray = (value: unknown, at: string, bounds: layoutLayoutInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return layoutLayoutInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) layoutLayoutInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) layoutLayoutInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const layoutLayoutInferenceTextGuardString = (value: unknown, at: string, bounds: layoutLayoutInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return layoutLayoutInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) layoutLayoutInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) layoutLayoutInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) layoutLayoutInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const layoutLayoutInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : layoutLayoutInferenceTextGuardReject(at, "value is not a boolean"));
export const layoutLayoutInferenceTextGuardNumber = (value: unknown, at: string, bounds: layoutLayoutInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return layoutLayoutInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) layoutLayoutInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) layoutLayoutInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const layoutLayoutInferenceTextGuardInteger = (value: unknown, at: string, bounds: layoutLayoutInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? layoutLayoutInferenceTextGuardNumber(value, at, bounds) : layoutLayoutInferenceTextGuardReject(at, "value is not an integer");
export const layoutLayoutInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : layoutLayoutInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const layoutLayoutInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : layoutLayoutInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLayoutInferenceText(value: unknown, at = "$"): LayoutInferenceText {
  return layoutLayoutInferenceTextGuardObject(value, `${at}`);
}
