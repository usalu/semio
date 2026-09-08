/** 📝️ Text representation for `norm.din18599.inference`. */
export type Din18599InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin18599InferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin18599InferenceTextGuardReject = (at: string, why: string): never => {
  throw new normDin18599InferenceTextGuardRefusal(at, why);
};

type normDin18599InferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin18599InferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin18599InferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin18599InferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin18599InferenceTextGuardReject(at, "value is not an object");
export const normDin18599InferenceTextGuardArray = (value: unknown, at: string, bounds: normDin18599InferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin18599InferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin18599InferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin18599InferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin18599InferenceTextGuardString = (value: unknown, at: string, bounds: normDin18599InferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin18599InferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin18599InferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin18599InferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin18599InferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin18599InferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin18599InferenceTextGuardReject(at, "value is not a boolean"));
export const normDin18599InferenceTextGuardNumber = (value: unknown, at: string, bounds: normDin18599InferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin18599InferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin18599InferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin18599InferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin18599InferenceTextGuardInteger = (value: unknown, at: string, bounds: normDin18599InferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin18599InferenceTextGuardNumber(value, at, bounds) : normDin18599InferenceTextGuardReject(at, "value is not an integer");
export const normDin18599InferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin18599InferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin18599InferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin18599InferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin18599InferenceText(value: unknown, at = "$"): Din18599InferenceText {
  return normDin18599InferenceTextGuardObject(value, `${at}`);
}
