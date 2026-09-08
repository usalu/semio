/** 💡️ Din18599 inference schema — document outline (field/section list + entry count). */

export interface Din18599Outline {
  sectionOutline: string[];
  fieldCount: number;
  entryCount: number;
}

export interface Din18599Inference {
  /** @derived */
  outline: Din18599Outline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin18599InferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin18599InferenceGuardReject = (at: string, why: string): never => {
  throw new normDin18599InferenceGuardRefusal(at, why);
};

type normDin18599InferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin18599InferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin18599InferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin18599InferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin18599InferenceGuardReject(at, "value is not an object");
export const normDin18599InferenceGuardArray = (value: unknown, at: string, bounds: normDin18599InferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin18599InferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin18599InferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin18599InferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin18599InferenceGuardString = (value: unknown, at: string, bounds: normDin18599InferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin18599InferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin18599InferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin18599InferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin18599InferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin18599InferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin18599InferenceGuardReject(at, "value is not a boolean"));
export const normDin18599InferenceGuardNumber = (value: unknown, at: string, bounds: normDin18599InferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin18599InferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin18599InferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin18599InferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin18599InferenceGuardInteger = (value: unknown, at: string, bounds: normDin18599InferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin18599InferenceGuardNumber(value, at, bounds) : normDin18599InferenceGuardReject(at, "value is not an integer");
export const normDin18599InferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin18599InferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin18599InferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin18599InferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin18599Inference(value: unknown, at = "$"): Din18599Inference {
  const row = normDin18599InferenceGuardObject(value, at);
  return {
    outline: parseDin18599Outline(row["outline"], `${at}.outline`),
  };
}

export function parseDin18599Outline(value: unknown, at = "$"): Din18599Outline {
  const row = normDin18599InferenceGuardObject(value, at);
  return {
    sectionOutline: normDin18599InferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => normDin18599InferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    fieldCount: normDin18599InferenceGuardInteger(row["fieldCount"], `${at}.fieldCount`),
    entryCount: normDin18599InferenceGuardInteger(row["entryCount"], `${at}.entryCount`),
  };
}
