/** 💡️ En1994 inference schema — document outline (field/section list + entry count). */

export interface En1994Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1994Inference {
  /** @derived */
  outline: En1994Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1994InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1994InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1994InferenceGuardRefusal(at, why);
};

type normEn1994InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1994InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1994InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1994InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1994InferenceGuardReject(at, "value is not an object");
export const normEn1994InferenceGuardArray = (value: unknown, at: string, bounds: normEn1994InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1994InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1994InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1994InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1994InferenceGuardString = (value: unknown, at: string, bounds: normEn1994InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1994InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1994InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1994InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1994InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1994InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1994InferenceGuardReject(at, "value is not a boolean"));
export const normEn1994InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1994InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1994InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1994InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1994InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1994InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1994InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1994InferenceGuardNumber(value, at, bounds) : normEn1994InferenceGuardReject(at, "value is not an integer");
export const normEn1994InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1994InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1994InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1994InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1994Inference(value: unknown, at = "$"): En1994Inference {
  const row = normEn1994InferenceGuardObject(value, at);
  return {
    outline: parseEn1994Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1994Outline(value: unknown, at = "$"): En1994Outline {
  const row = normEn1994InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1994InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1994InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1994InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1994InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
