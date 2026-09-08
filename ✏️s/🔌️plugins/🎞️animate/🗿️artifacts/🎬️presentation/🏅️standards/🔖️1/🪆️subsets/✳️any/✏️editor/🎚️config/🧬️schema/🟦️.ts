/** 🧬️ PresentationConfig */
export interface PresentationConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class animatePresentationConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const animatePresentationConfigGuardReject = (at: string, why: string): never => {
  throw new animatePresentationConfigGuardRefusal(at, why);
};

type animatePresentationConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type animatePresentationConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type animatePresentationConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const animatePresentationConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : animatePresentationConfigGuardReject(at, "value is not an object");
export const animatePresentationConfigGuardArray = (value: unknown, at: string, bounds: animatePresentationConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return animatePresentationConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) animatePresentationConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) animatePresentationConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const animatePresentationConfigGuardString = (value: unknown, at: string, bounds: animatePresentationConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return animatePresentationConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) animatePresentationConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) animatePresentationConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) animatePresentationConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const animatePresentationConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : animatePresentationConfigGuardReject(at, "value is not a boolean"));
export const animatePresentationConfigGuardNumber = (value: unknown, at: string, bounds: animatePresentationConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return animatePresentationConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) animatePresentationConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) animatePresentationConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const animatePresentationConfigGuardInteger = (value: unknown, at: string, bounds: animatePresentationConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? animatePresentationConfigGuardNumber(value, at, bounds) : animatePresentationConfigGuardReject(at, "value is not an integer");
export const animatePresentationConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : animatePresentationConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const animatePresentationConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : animatePresentationConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePresentationConfig(value: unknown, at = "$"): PresentationConfig {
  const row = animatePresentationConfigGuardObject(value, at);
  return {
    engagementInput: animatePresentationConfigGuardString(row["engagementInput"], `${at}.engagementInput`),
  };
}
