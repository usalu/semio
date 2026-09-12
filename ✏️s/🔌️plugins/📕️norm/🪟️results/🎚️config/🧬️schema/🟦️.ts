/** 🧬️ NormResultsWindowConfig */
export interface NormResultsWindowConfig {
  /** @state windowConfig */
  selectedCheckIndex?: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normNormResultsWindowConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normNormResultsWindowConfigGuardReject = (at: string, why: string): never => {
  throw new normNormResultsWindowConfigGuardRefusal(at, why);
};

type normNormResultsWindowConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normNormResultsWindowConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normNormResultsWindowConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normNormResultsWindowConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normNormResultsWindowConfigGuardReject(at, "value is not an object");
export const normNormResultsWindowConfigGuardArray = (value: unknown, at: string, bounds: normNormResultsWindowConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normNormResultsWindowConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normNormResultsWindowConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normNormResultsWindowConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normNormResultsWindowConfigGuardString = (value: unknown, at: string, bounds: normNormResultsWindowConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normNormResultsWindowConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normNormResultsWindowConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normNormResultsWindowConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normNormResultsWindowConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normNormResultsWindowConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normNormResultsWindowConfigGuardReject(at, "value is not a boolean"));
export const normNormResultsWindowConfigGuardNumber = (value: unknown, at: string, bounds: normNormResultsWindowConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normNormResultsWindowConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normNormResultsWindowConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normNormResultsWindowConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normNormResultsWindowConfigGuardInteger = (value: unknown, at: string, bounds: normNormResultsWindowConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normNormResultsWindowConfigGuardNumber(value, at, bounds) : normNormResultsWindowConfigGuardReject(at, "value is not an integer");
export const normNormResultsWindowConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normNormResultsWindowConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normNormResultsWindowConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normNormResultsWindowConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNormResultsWindowConfig(value: unknown, at = "$"): NormResultsWindowConfig {
  const row = normNormResultsWindowConfigGuardObject(value, at);
  for (const key of Object.keys(row)) if (key !== "selectedCheckIndex") normNormResultsWindowConfigGuardReject(`${at}.${key}`, "field is not declared");
  return {
    selectedCheckIndex: row["selectedCheckIndex"] === undefined ? undefined : normNormResultsWindowConfigGuardInteger(row["selectedCheckIndex"], `${at}.selectedCheckIndex`, {"minimum": 0,"maximum": 4294967295}),
  };
}
