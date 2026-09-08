/** 🧬️ WiresPresence */
export interface WiresPresence {
  /** @state presence */
  dragNodeId?: string;
  /** @state presence */
  dragLastX: number;
  /** @state presence */
  dragLastY: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class reasoningWiresPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reasoningWiresPresenceGuardReject = (at: string, why: string): never => {
  throw new reasoningWiresPresenceGuardRefusal(at, why);
};

type reasoningWiresPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type reasoningWiresPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type reasoningWiresPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const reasoningWiresPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reasoningWiresPresenceGuardReject(at, "value is not an object");
export const reasoningWiresPresenceGuardArray = (value: unknown, at: string, bounds: reasoningWiresPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return reasoningWiresPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) reasoningWiresPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) reasoningWiresPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const reasoningWiresPresenceGuardString = (value: unknown, at: string, bounds: reasoningWiresPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return reasoningWiresPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) reasoningWiresPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) reasoningWiresPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) reasoningWiresPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const reasoningWiresPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : reasoningWiresPresenceGuardReject(at, "value is not a boolean"));
export const reasoningWiresPresenceGuardNumber = (value: unknown, at: string, bounds: reasoningWiresPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return reasoningWiresPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) reasoningWiresPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) reasoningWiresPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const reasoningWiresPresenceGuardInteger = (value: unknown, at: string, bounds: reasoningWiresPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? reasoningWiresPresenceGuardNumber(value, at, bounds) : reasoningWiresPresenceGuardReject(at, "value is not an integer");
export const reasoningWiresPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : reasoningWiresPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const reasoningWiresPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : reasoningWiresPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWiresPresence(value: unknown, at = "$"): WiresPresence {
  const row = reasoningWiresPresenceGuardObject(value, at);
  return {
    dragNodeId: row["dragNodeId"] === undefined ? undefined : reasoningWiresPresenceGuardString(row["dragNodeId"], `${at}.dragNodeId`),
    dragLastX: reasoningWiresPresenceGuardNumber(row["dragLastX"], `${at}.dragLastX`),
    dragLastY: reasoningWiresPresenceGuardNumber(row["dragLastY"], `${at}.dragLastY`),
  };
}
