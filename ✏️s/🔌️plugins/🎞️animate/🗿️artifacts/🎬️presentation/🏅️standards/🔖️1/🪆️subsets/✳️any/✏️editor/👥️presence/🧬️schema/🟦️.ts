/** 🧬️ PresentationPresence — empty: tile selection/hover broadcast through the framework's typed `PresenceInteraction` ("tiles" domain, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); presentation has no other app-specific ephemeral field left to carry. Mirrors `PresentationPresence` in `../🦀️.rs`. */
export interface PresentationPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class animatePresentationPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const animatePresentationPresenceGuardReject = (at: string, why: string): never => {
  throw new animatePresentationPresenceGuardRefusal(at, why);
};

type animatePresentationPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type animatePresentationPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type animatePresentationPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const animatePresentationPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : animatePresentationPresenceGuardReject(at, "value is not an object");
export const animatePresentationPresenceGuardArray = (value: unknown, at: string, bounds: animatePresentationPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return animatePresentationPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) animatePresentationPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) animatePresentationPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const animatePresentationPresenceGuardString = (value: unknown, at: string, bounds: animatePresentationPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return animatePresentationPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) animatePresentationPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) animatePresentationPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) animatePresentationPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const animatePresentationPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : animatePresentationPresenceGuardReject(at, "value is not a boolean"));
export const animatePresentationPresenceGuardNumber = (value: unknown, at: string, bounds: animatePresentationPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return animatePresentationPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) animatePresentationPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) animatePresentationPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const animatePresentationPresenceGuardInteger = (value: unknown, at: string, bounds: animatePresentationPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? animatePresentationPresenceGuardNumber(value, at, bounds) : animatePresentationPresenceGuardReject(at, "value is not an integer");
export const animatePresentationPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : animatePresentationPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const animatePresentationPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : animatePresentationPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePresentationPresence(value: unknown, at = "$"): PresentationPresence {
  return animatePresentationPresenceGuardObject(value, `${at}`);
}
