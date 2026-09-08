/** 💡️ Din16798 inference schema — document outline (field/section list + entry count). */

export interface Din16798Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Din16798Inference {
  /** @derived */
  outline: Din16798Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin16798InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin16798InferenceGuardReject = (at: string, why: string): never => {
  throw new normDin16798InferenceGuardRefusal(at, why);
};

type normDin16798InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin16798InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin16798InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin16798InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin16798InferenceGuardReject(at, "value is not an object");
export const normDin16798InferenceGuardArray = (value: unknown, at: string, bounds: normDin16798InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin16798InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin16798InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin16798InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin16798InferenceGuardString = (value: unknown, at: string, bounds: normDin16798InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin16798InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin16798InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin16798InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin16798InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin16798InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin16798InferenceGuardReject(at, "value is not a boolean"));
export const normDin16798InferenceGuardNumber = (value: unknown, at: string, bounds: normDin16798InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin16798InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin16798InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin16798InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin16798InferenceGuardInteger = (value: unknown, at: string, bounds: normDin16798InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin16798InferenceGuardNumber(value, at, bounds) : normDin16798InferenceGuardReject(at, "value is not an integer");
export const normDin16798InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin16798InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin16798InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin16798InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin16798Inference(value: unknown, at = "$"): Din16798Inference {
  const row = normDin16798InferenceGuardObject(value, at);
  return {
    outline: parseDin16798Outline(row["outline"], `${at}.outline`),
  };
}

export function parseDin16798Outline(value: unknown, at = "$"): Din16798Outline {
  const row = normDin16798InferenceGuardObject(value, at);
  return {
    sectionOutline: normDin16798InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normDin16798InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normDin16798InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normDin16798InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
