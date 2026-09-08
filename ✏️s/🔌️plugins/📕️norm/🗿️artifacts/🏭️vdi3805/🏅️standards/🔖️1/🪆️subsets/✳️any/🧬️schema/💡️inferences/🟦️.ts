/** 💡️ Vdi3805 inference schema — document outline (field/section list + entry count). */

export interface Vdi3805Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Vdi3805Inference {
  /** @derived */
  outline: Vdi3805Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normVdi3805InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normVdi3805InferenceGuardReject = (at: string, why: string): never => {
  throw new normVdi3805InferenceGuardRefusal(at, why);
};

type normVdi3805InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normVdi3805InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normVdi3805InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normVdi3805InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normVdi3805InferenceGuardReject(at, "value is not an object");
export const normVdi3805InferenceGuardArray = (value: unknown, at: string, bounds: normVdi3805InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normVdi3805InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normVdi3805InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normVdi3805InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normVdi3805InferenceGuardString = (value: unknown, at: string, bounds: normVdi3805InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normVdi3805InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normVdi3805InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normVdi3805InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normVdi3805InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normVdi3805InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normVdi3805InferenceGuardReject(at, "value is not a boolean"));
export const normVdi3805InferenceGuardNumber = (value: unknown, at: string, bounds: normVdi3805InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normVdi3805InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normVdi3805InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normVdi3805InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normVdi3805InferenceGuardInteger = (value: unknown, at: string, bounds: normVdi3805InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normVdi3805InferenceGuardNumber(value, at, bounds) : normVdi3805InferenceGuardReject(at, "value is not an integer");
export const normVdi3805InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normVdi3805InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normVdi3805InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normVdi3805InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVdi3805Inference(value: unknown, at = "$"): Vdi3805Inference {
  const row = normVdi3805InferenceGuardObject(value, at);
  return {
    outline: parseVdi3805Outline(row["outline"], `${at}.outline`),
  };
}

export function parseVdi3805Outline(value: unknown, at = "$"): Vdi3805Outline {
  const row = normVdi3805InferenceGuardObject(value, at);
  return {
    sectionOutline: normVdi3805InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normVdi3805InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normVdi3805InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normVdi3805InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
