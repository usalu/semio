/** 💡️ Writer inference schema — document outline (markdown headings + word/line counts). */

export interface WriterOutline {
  sectionOutline: string[];
  wordCount: number;
  lineCount: number;
}

export interface WriterInference {
  /** @derived */
  outline: WriterOutline;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class writerWriterInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const writerWriterInferenceGuardReject = (at: string, why: string): never => {
  throw new writerWriterInferenceGuardRefusal(at, why);
};

type writerWriterInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type writerWriterInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type writerWriterInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const writerWriterInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : writerWriterInferenceGuardReject(at, "value is not an object");
export const writerWriterInferenceGuardArray = (value: unknown, at: string, bounds: writerWriterInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return writerWriterInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) writerWriterInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) writerWriterInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const writerWriterInferenceGuardString = (value: unknown, at: string, bounds: writerWriterInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return writerWriterInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) writerWriterInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) writerWriterInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) writerWriterInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const writerWriterInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : writerWriterInferenceGuardReject(at, "value is not a boolean"));
export const writerWriterInferenceGuardNumber = (value: unknown, at: string, bounds: writerWriterInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return writerWriterInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) writerWriterInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) writerWriterInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const writerWriterInferenceGuardInteger = (value: unknown, at: string, bounds: writerWriterInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? writerWriterInferenceGuardNumber(value, at, bounds) : writerWriterInferenceGuardReject(at, "value is not an integer");
export const writerWriterInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : writerWriterInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const writerWriterInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : writerWriterInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWriterInference(value: unknown, at = "$"): WriterInference {
  const row = writerWriterInferenceGuardObject(value, at);
  return {
    outline: parseWriterOutline(row["outline"], `${at}.outline`),
  };
}

export function parseWriterOutline(value: unknown, at = "$"): WriterOutline {
  const row = writerWriterInferenceGuardObject(value, at);
  return {
    sectionOutline: writerWriterInferenceGuardArray(row["sectionOutline"], `${at}.sectionOutline`).map((item, index) => writerWriterInferenceGuardString(item, `${at}.sectionOutline[${index}]`)),
    wordCount: writerWriterInferenceGuardInteger(row["wordCount"], `${at}.wordCount`),
    lineCount: writerWriterInferenceGuardInteger(row["lineCount"], `${at}.lineCount`),
  };
}
