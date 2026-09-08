/** 💡️ En1996 inference schema — document outline (field/section list + entry count). */

export interface En1996Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1996Inference {
  /** @derived */
  outline: En1996Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1996InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1996InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1996InferenceGuardRefusal(at, why);
};

type normEn1996InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1996InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1996InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1996InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1996InferenceGuardReject(at, "value is not an object");
export const normEn1996InferenceGuardArray = (value: unknown, at: string, bounds: normEn1996InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1996InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1996InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1996InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1996InferenceGuardString = (value: unknown, at: string, bounds: normEn1996InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1996InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1996InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1996InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1996InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1996InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1996InferenceGuardReject(at, "value is not a boolean"));
export const normEn1996InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1996InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1996InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1996InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1996InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1996InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1996InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1996InferenceGuardNumber(value, at, bounds) : normEn1996InferenceGuardReject(at, "value is not an integer");
export const normEn1996InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1996InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1996InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1996InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1996Inference(value: unknown, at = "$"): En1996Inference {
  const row = normEn1996InferenceGuardObject(value, at);
  return {
    outline: parseEn1996Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1996Outline(value: unknown, at = "$"): En1996Outline {
  const row = normEn1996InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1996InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1996InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1996InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1996InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
