/** 💡️ Din4108 inference schema — document outline (field/section list + entry count). */

export interface Din4108Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Din4108Inference {
  /** @derived */
  outline: Din4108Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin4108InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin4108InferenceGuardReject = (at: string, why: string): never => {
  throw new normDin4108InferenceGuardRefusal(at, why);
};

type normDin4108InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin4108InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin4108InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin4108InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin4108InferenceGuardReject(at, "value is not an object");
export const normDin4108InferenceGuardArray = (value: unknown, at: string, bounds: normDin4108InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin4108InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin4108InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin4108InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin4108InferenceGuardString = (value: unknown, at: string, bounds: normDin4108InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin4108InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin4108InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin4108InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin4108InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin4108InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin4108InferenceGuardReject(at, "value is not a boolean"));
export const normDin4108InferenceGuardNumber = (value: unknown, at: string, bounds: normDin4108InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin4108InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin4108InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin4108InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin4108InferenceGuardInteger = (value: unknown, at: string, bounds: normDin4108InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin4108InferenceGuardNumber(value, at, bounds) : normDin4108InferenceGuardReject(at, "value is not an integer");
export const normDin4108InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin4108InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin4108InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin4108InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin4108Inference(value: unknown, at = "$"): Din4108Inference {
  const row = normDin4108InferenceGuardObject(value, at);
  return {
    outline: parseDin4108Outline(row["outline"], `${at}.outline`),
  };
}

export function parseDin4108Outline(value: unknown, at = "$"): Din4108Outline {
  const row = normDin4108InferenceGuardObject(value, at);
  return {
    sectionOutline: normDin4108InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normDin4108InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normDin4108InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normDin4108InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
