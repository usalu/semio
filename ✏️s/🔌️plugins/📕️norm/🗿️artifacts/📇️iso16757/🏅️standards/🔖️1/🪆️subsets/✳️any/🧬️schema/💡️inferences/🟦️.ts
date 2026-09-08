/** 💡️ Iso16757 inference schema — document outline (field/section list + entry count). */

export interface Iso16757Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Iso16757Inference {
  /** @derived */
  outline: Iso16757Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normIso16757InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normIso16757InferenceGuardReject = (at: string, why: string): never => {
  throw new normIso16757InferenceGuardRefusal(at, why);
};

type normIso16757InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normIso16757InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normIso16757InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normIso16757InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normIso16757InferenceGuardReject(at, "value is not an object");
export const normIso16757InferenceGuardArray = (value: unknown, at: string, bounds: normIso16757InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normIso16757InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normIso16757InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normIso16757InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normIso16757InferenceGuardString = (value: unknown, at: string, bounds: normIso16757InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normIso16757InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normIso16757InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normIso16757InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normIso16757InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normIso16757InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normIso16757InferenceGuardReject(at, "value is not a boolean"));
export const normIso16757InferenceGuardNumber = (value: unknown, at: string, bounds: normIso16757InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normIso16757InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normIso16757InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normIso16757InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normIso16757InferenceGuardInteger = (value: unknown, at: string, bounds: normIso16757InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normIso16757InferenceGuardNumber(value, at, bounds) : normIso16757InferenceGuardReject(at, "value is not an integer");
export const normIso16757InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normIso16757InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normIso16757InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normIso16757InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIso16757Inference(value: unknown, at = "$"): Iso16757Inference {
  const row = normIso16757InferenceGuardObject(value, at);
  return {
    outline: parseIso16757Outline(row["outline"], `${at}.outline`),
  };
}

export function parseIso16757Outline(value: unknown, at = "$"): Iso16757Outline {
  const row = normIso16757InferenceGuardObject(value, at);
  return {
    sectionOutline: normIso16757InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normIso16757InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normIso16757InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normIso16757InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
