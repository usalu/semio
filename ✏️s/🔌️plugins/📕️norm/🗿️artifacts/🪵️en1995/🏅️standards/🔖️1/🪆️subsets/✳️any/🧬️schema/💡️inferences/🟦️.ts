/** 💡️ En1995 inference schema — document outline (field/section list + entry count). */

export interface En1995Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1995Inference {
  /** @derived */
  outline: En1995Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1995InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1995InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1995InferenceGuardRefusal(at, why);
};

type normEn1995InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1995InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1995InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1995InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1995InferenceGuardReject(at, "value is not an object");
export const normEn1995InferenceGuardArray = (value: unknown, at: string, bounds: normEn1995InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1995InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1995InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1995InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1995InferenceGuardString = (value: unknown, at: string, bounds: normEn1995InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1995InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1995InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1995InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1995InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1995InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1995InferenceGuardReject(at, "value is not a boolean"));
export const normEn1995InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1995InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1995InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1995InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1995InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1995InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1995InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1995InferenceGuardNumber(value, at, bounds) : normEn1995InferenceGuardReject(at, "value is not an integer");
export const normEn1995InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1995InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1995InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1995InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1995Inference(value: unknown, at = "$"): En1995Inference {
  const row = normEn1995InferenceGuardObject(value, at);
  return {
    outline: parseEn1995Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1995Outline(value: unknown, at = "$"): En1995Outline {
  const row = normEn1995InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1995InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1995InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1995InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1995InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
