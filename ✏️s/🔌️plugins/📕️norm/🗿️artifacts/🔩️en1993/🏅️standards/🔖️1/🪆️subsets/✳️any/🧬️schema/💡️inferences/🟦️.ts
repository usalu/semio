/** 💡️ En1993 inference schema — document outline (field/section list + entry count). */

export interface En1993Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1993Inference {
  /** @derived */
  outline: En1993Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1993InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1993InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1993InferenceGuardRefusal(at, why);
};

type normEn1993InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1993InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1993InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1993InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1993InferenceGuardReject(at, "value is not an object");
export const normEn1993InferenceGuardArray = (value: unknown, at: string, bounds: normEn1993InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1993InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1993InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1993InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1993InferenceGuardString = (value: unknown, at: string, bounds: normEn1993InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1993InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1993InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1993InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1993InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1993InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1993InferenceGuardReject(at, "value is not a boolean"));
export const normEn1993InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1993InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1993InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1993InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1993InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1993InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1993InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1993InferenceGuardNumber(value, at, bounds) : normEn1993InferenceGuardReject(at, "value is not an integer");
export const normEn1993InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1993InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1993InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1993InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1993Inference(value: unknown, at = "$"): En1993Inference {
  const row = normEn1993InferenceGuardObject(value, at);
  return {
    outline: parseEn1993Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1993Outline(value: unknown, at = "$"): En1993Outline {
  const row = normEn1993InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1993InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1993InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1993InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1993InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
