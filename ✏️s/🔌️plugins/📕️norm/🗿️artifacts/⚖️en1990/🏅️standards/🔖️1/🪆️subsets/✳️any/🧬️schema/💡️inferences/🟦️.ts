/** 💡️ En1990 inference schema — document outline and clause summary. */

export interface En1990Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
  checkCount: number;
  passCount: number;
  allPass: boolean;
  governingClause: string;
  governingUtilization: number;
}

export interface En1990Inference {
  /** @derived */
  outline: En1990Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1990InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1990InferenceGuardReject = (at: string, why: string): never => {
  throw new normEn1990InferenceGuardRefusal(at, why);
};

type normEn1990InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1990InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1990InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1990InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1990InferenceGuardReject(at, "value is not an object");
export const normEn1990InferenceGuardArray = (value: unknown, at: string, bounds: normEn1990InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1990InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1990InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1990InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1990InferenceGuardString = (value: unknown, at: string, bounds: normEn1990InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1990InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1990InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1990InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1990InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1990InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1990InferenceGuardReject(at, "value is not a boolean"));
export const normEn1990InferenceGuardNumber = (value: unknown, at: string, bounds: normEn1990InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1990InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1990InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1990InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1990InferenceGuardInteger = (value: unknown, at: string, bounds: normEn1990InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1990InferenceGuardNumber(value, at, bounds) : normEn1990InferenceGuardReject(at, "value is not an integer");
export const normEn1990InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1990InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1990InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1990InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1990Inference(value: unknown, at = "$"): En1990Inference {
  const row = normEn1990InferenceGuardObject(value, at);
  return {
    outline: parseEn1990Outline(row["outline"], `${at}.outline`),
  };
}

export function parseEn1990Outline(value: unknown, at = "$"): En1990Outline {
  const row = normEn1990InferenceGuardObject(value, at);
  return {
    sectionOutline: normEn1990InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normEn1990InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normEn1990InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normEn1990InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
    checkCount: normEn1990InferenceGuardInteger(row["checkCount"], `${at}.checkCount`),
    passCount: normEn1990InferenceGuardInteger(row["passCount"], `${at}.passCount`),
    allPass: normEn1990InferenceGuardBoolean(row["allPass"], `${at}.allPass`),
    governingClause: normEn1990InferenceGuardString(row["governingClause"], `${at}.governingClause`),
    governingUtilization: normEn1990InferenceGuardNumber(row["governingUtilization"], `${at}.governingUtilization`),
  };
}
