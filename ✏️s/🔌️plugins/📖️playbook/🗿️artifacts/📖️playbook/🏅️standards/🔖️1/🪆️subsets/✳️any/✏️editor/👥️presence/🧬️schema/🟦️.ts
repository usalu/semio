/** 🧬️ PlaybookPresence — empty: peer selection on the block-list builder now broadcasts via the framework's typed `PresencePeer.interaction` ("blocks" domain, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); playbook has no other shareable live state. Mirrors `PlaybookPresence` in `../🦀️.rs`. */
export interface PlaybookPresence {}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class playbookPlaybookPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const playbookPlaybookPresenceGuardReject = (at: string, why: string): never => {
  throw new playbookPlaybookPresenceGuardRefusal(at, why);
};

type playbookPlaybookPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type playbookPlaybookPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type playbookPlaybookPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const playbookPlaybookPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : playbookPlaybookPresenceGuardReject(at, "value is not an object");
export const playbookPlaybookPresenceGuardArray = (value: unknown, at: string, bounds: playbookPlaybookPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return playbookPlaybookPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) playbookPlaybookPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) playbookPlaybookPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const playbookPlaybookPresenceGuardString = (value: unknown, at: string, bounds: playbookPlaybookPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return playbookPlaybookPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) playbookPlaybookPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) playbookPlaybookPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) playbookPlaybookPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const playbookPlaybookPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : playbookPlaybookPresenceGuardReject(at, "value is not a boolean"));
export const playbookPlaybookPresenceGuardNumber = (value: unknown, at: string, bounds: playbookPlaybookPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return playbookPlaybookPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) playbookPlaybookPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) playbookPlaybookPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const playbookPlaybookPresenceGuardInteger = (value: unknown, at: string, bounds: playbookPlaybookPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? playbookPlaybookPresenceGuardNumber(value, at, bounds) : playbookPlaybookPresenceGuardReject(at, "value is not an integer");
export const playbookPlaybookPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : playbookPlaybookPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const playbookPlaybookPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : playbookPlaybookPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaybookPresence(value: unknown, at = "$"): PlaybookPresence {
  return playbookPlaybookPresenceGuardObject(value, `${at}`);
}
