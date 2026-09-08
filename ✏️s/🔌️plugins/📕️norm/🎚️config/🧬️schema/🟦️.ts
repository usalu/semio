/** 🧬️ NormConfig */
export interface NormConfig {
  /** @state config */
  selectedCheckIndex?: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normNormConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normNormConfigGuardReject = (at: string, why: string): never => {
  throw new normNormConfigGuardRefusal(at, why);
};

type normNormConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normNormConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normNormConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normNormConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normNormConfigGuardReject(at, "value is not an object");
export const normNormConfigGuardArray = (value: unknown, at: string, bounds: normNormConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normNormConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normNormConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normNormConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normNormConfigGuardString = (value: unknown, at: string, bounds: normNormConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normNormConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normNormConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normNormConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normNormConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normNormConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normNormConfigGuardReject(at, "value is not a boolean"));
export const normNormConfigGuardNumber = (value: unknown, at: string, bounds: normNormConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normNormConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normNormConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normNormConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normNormConfigGuardInteger = (value: unknown, at: string, bounds: normNormConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normNormConfigGuardNumber(value, at, bounds) : normNormConfigGuardReject(at, "value is not an integer");
export const normNormConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normNormConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normNormConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normNormConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNormConfig(value: unknown, at = "$"): NormConfig {
  const row = normNormConfigGuardObject(value, at);
  return {
    selectedCheckIndex: row["selectedCheckIndex"] === undefined ? undefined : normNormConfigGuardInteger(row["selectedCheckIndex"], `${at}.selectedCheckIndex`, {"minimum": 0}),
  };
}
