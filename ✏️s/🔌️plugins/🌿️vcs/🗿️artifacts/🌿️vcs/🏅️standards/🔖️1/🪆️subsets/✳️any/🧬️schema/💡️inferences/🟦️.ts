/** 💡️ VCS inference schema — a scalar summary digest of the tags/notes free-form fields. */

export interface VcsSummary {
  tagCount: number;
  notesWordCount: number;
  hasNotes: boolean;
}

export interface VcsInference {
  /** @derived */
  summary: VcsSummary;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class vcsVcsInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const vcsVcsInferenceGuardReject = (at: string, why: string): never => {
  throw new vcsVcsInferenceGuardRefusal(at, why);
};

type vcsVcsInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type vcsVcsInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type vcsVcsInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const vcsVcsInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : vcsVcsInferenceGuardReject(at, "value is not an object");
export const vcsVcsInferenceGuardArray = (value: unknown, at: string, bounds: vcsVcsInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return vcsVcsInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) vcsVcsInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) vcsVcsInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const vcsVcsInferenceGuardString = (value: unknown, at: string, bounds: vcsVcsInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return vcsVcsInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) vcsVcsInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) vcsVcsInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) vcsVcsInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const vcsVcsInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : vcsVcsInferenceGuardReject(at, "value is not a boolean"));
export const vcsVcsInferenceGuardNumber = (value: unknown, at: string, bounds: vcsVcsInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return vcsVcsInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) vcsVcsInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) vcsVcsInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const vcsVcsInferenceGuardInteger = (value: unknown, at: string, bounds: vcsVcsInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? vcsVcsInferenceGuardNumber(value, at, bounds) : vcsVcsInferenceGuardReject(at, "value is not an integer");
export const vcsVcsInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : vcsVcsInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const vcsVcsInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : vcsVcsInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVcsInference(value: unknown, at = "$"): VcsInference {
  const row = vcsVcsInferenceGuardObject(value, at);
  return {
    summary: parseVcsSummary(row["summary"], `${at}.summary`),
  };
}

export function parseVcsSummary(value: unknown, at = "$"): VcsSummary {
  const row = vcsVcsInferenceGuardObject(value, at);
  return {
    tagCount: vcsVcsInferenceGuardInteger(row["tagCount"], `${at}.tagCount`),
    notesWordCount: vcsVcsInferenceGuardInteger(row["notesWordCount"], `${at}.notesWordCount`),
    hasNotes: vcsVcsInferenceGuardBoolean(row["hasNotes"], `${at}.hasNotes`),
  };
}
