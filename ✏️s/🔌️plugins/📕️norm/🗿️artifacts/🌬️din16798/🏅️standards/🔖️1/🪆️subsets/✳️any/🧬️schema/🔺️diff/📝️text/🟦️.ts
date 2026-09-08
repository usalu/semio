/** 📝️ Text representation for `norm.din16798.diff`. */
export type Din16798DiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin16798DiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin16798DiffTextGuardReject = (at: string, why: string): never => {
  throw new normDin16798DiffTextGuardRefusal(at, why);
};

type normDin16798DiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin16798DiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin16798DiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin16798DiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin16798DiffTextGuardReject(at, "value is not an object");
export const normDin16798DiffTextGuardArray = (value: unknown, at: string, bounds: normDin16798DiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin16798DiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin16798DiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin16798DiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin16798DiffTextGuardString = (value: unknown, at: string, bounds: normDin16798DiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin16798DiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin16798DiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin16798DiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin16798DiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin16798DiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin16798DiffTextGuardReject(at, "value is not a boolean"));
export const normDin16798DiffTextGuardNumber = (value: unknown, at: string, bounds: normDin16798DiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin16798DiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin16798DiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin16798DiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin16798DiffTextGuardInteger = (value: unknown, at: string, bounds: normDin16798DiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin16798DiffTextGuardNumber(value, at, bounds) : normDin16798DiffTextGuardReject(at, "value is not an integer");
export const normDin16798DiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin16798DiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin16798DiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin16798DiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin16798DiffText(value: unknown, at = "$"): Din16798DiffText {
  return normDin16798DiffTextGuardObject(value, `${at}`);
}
