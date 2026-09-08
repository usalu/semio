/** 💡️ En1998 inference schema — document outline (field/section list + entry count). */

export interface En1998Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface En1998Inference {
  /** @derived */
  outline: En1998Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1998InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1998InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1998InferenceGuardRefusal(at, why);
};

type normEn1998InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1998InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1998InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1998InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1998InferenceGuardReject(at, "value is not an object");
export const normEn1998InferenceGuardArray = (value: unknown, at: string, bounds: normEn1998InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1998InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1998InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1998InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1998InferenceGuardString = (value: unknown, at: string, bounds: normEn1998InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1998InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1998InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1998InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1998InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1998InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1998InferenceGuardReject(at, "value is not a boolean"));
export const normEn1998InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1998InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1998InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1998InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1998InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1998InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1998InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1998InferenceGuardNumber(value, at, bounds) : normEn1998InferenceGuardReject(at, "value is not an integer");
export const normEn1998InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1998InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1998InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1998InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1998Inference(value: unknown, at = "$"): En1998Inference {
  const row = normEn1998InferenceGuardObject(value, at);
  return {
    outline: parseEn1998Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1998Outline(value: unknown, at = "$"): En1998Outline {
  const row = normEn1998InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1998InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1998InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1998InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1998InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
