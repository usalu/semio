/** 🧬️ Framework-owned empty presence facet used by every norm editor. */
export type NoPresence = Readonly<Record<string, never>>;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normNormPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normNormPresenceGuardReject = (at: string, why: string): never => {
  throw new normNormPresenceGuardRefusal(at, why);
};

type normNormPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normNormPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normNormPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normNormPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normNormPresenceGuardReject(at, "value is not an object");
export const normNormPresenceGuardArray = (value: unknown, at: string, bounds: normNormPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normNormPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normNormPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normNormPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normNormPresenceGuardString = (value: unknown, at: string, bounds: normNormPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normNormPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normNormPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normNormPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normNormPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normNormPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normNormPresenceGuardReject(at, "value is not a boolean"));
export const normNormPresenceGuardNumber = (value: unknown, at: string, bounds: normNormPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normNormPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normNormPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normNormPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normNormPresenceGuardInteger = (value: unknown, at: string, bounds: normNormPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normNormPresenceGuardNumber(value, at, bounds) : normNormPresenceGuardReject(at, "value is not an integer");
export const normNormPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normNormPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normNormPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normNormPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseNoPresence(value: unknown, at = "$"): NoPresence {
  return normNormPresenceGuardObject(value, `${at}`);
}
