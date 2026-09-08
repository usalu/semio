/** 💡️ En1992 inference schema — document outline (field/section list + entry count). */

export interface En1992Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1992Inference {
  /** @derived */
  outline: En1992Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1992InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1992InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1992InferenceGuardRefusal(at, why);
};

type normEn1992InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1992InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1992InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1992InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1992InferenceGuardReject(at, "value is not an object");
export const normEn1992InferenceGuardArray = (value: unknown, at: string, bounds: normEn1992InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1992InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1992InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1992InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1992InferenceGuardString = (value: unknown, at: string, bounds: normEn1992InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1992InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1992InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1992InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1992InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1992InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1992InferenceGuardReject(at, "value is not a boolean"));
export const normEn1992InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1992InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1992InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1992InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1992InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1992InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1992InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1992InferenceGuardNumber(value, at, bounds) : normEn1992InferenceGuardReject(at, "value is not an integer");
export const normEn1992InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1992InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1992InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1992InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1992Inference(value: unknown, at = "$"): En1992Inference {
  const row = normEn1992InferenceGuardObject(value, at);
  return {
    outline: parseEn1992Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1992Outline(value: unknown, at = "$"): En1992Outline {
  const row = normEn1992InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1992InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1992InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1992InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1992InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
