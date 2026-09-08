/** 💡️ En1999 inference schema — document outline (field/section list + entry count). */

export interface En1999Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1999Inference {
  /** @derived */
  outline: En1999Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1999InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1999InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1999InferenceGuardRefusal(at, why);
};

type normEn1999InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1999InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1999InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1999InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1999InferenceGuardReject(at, "value is not an object");
export const normEn1999InferenceGuardArray = (value: unknown, at: string, bounds: normEn1999InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1999InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1999InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1999InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1999InferenceGuardString = (value: unknown, at: string, bounds: normEn1999InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1999InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1999InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1999InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1999InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1999InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1999InferenceGuardReject(at, "value is not a boolean"));
export const normEn1999InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1999InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1999InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1999InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1999InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1999InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1999InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1999InferenceGuardNumber(value, at, bounds) : normEn1999InferenceGuardReject(at, "value is not an integer");
export const normEn1999InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1999InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1999InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1999InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1999Inference(value: unknown, at = "$"): En1999Inference {
  const row = normEn1999InferenceGuardObject(value, at);
  return {
    outline: parseEn1999Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1999Outline(value: unknown, at = "$"): En1999Outline {
  const row = normEn1999InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1999InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1999InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1999InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1999InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
