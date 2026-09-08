/** 🧬️ WiresConfig */
export interface WiresConfig {
  /** @state config */
  dragNodeId?: string;
  /** @state config */
  dragLastX: number;
  /** @state config */
  dragLastY: number;
  /** @state config */
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class reasoningWiresConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reasoningWiresConfigGuardReject = (at: string, why: string): never => {
  throw new reasoningWiresConfigGuardRefusal(at, why);
};

type reasoningWiresConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type reasoningWiresConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type reasoningWiresConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const reasoningWiresConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reasoningWiresConfigGuardReject(at, "value is not an object");
export const reasoningWiresConfigGuardArray = (value: unknown, at: string, bounds: reasoningWiresConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return reasoningWiresConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) reasoningWiresConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) reasoningWiresConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const reasoningWiresConfigGuardString = (value: unknown, at: string, bounds: reasoningWiresConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return reasoningWiresConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) reasoningWiresConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) reasoningWiresConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) reasoningWiresConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const reasoningWiresConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : reasoningWiresConfigGuardReject(at, "value is not a boolean"));
export const reasoningWiresConfigGuardNumber = (value: unknown, at: string, bounds: reasoningWiresConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return reasoningWiresConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) reasoningWiresConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) reasoningWiresConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const reasoningWiresConfigGuardInteger = (value: unknown, at: string, bounds: reasoningWiresConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? reasoningWiresConfigGuardNumber(value, at, bounds) : reasoningWiresConfigGuardReject(at, "value is not an integer");
export const reasoningWiresConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : reasoningWiresConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const reasoningWiresConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : reasoningWiresConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWiresConfig(value: unknown, at = "$"): WiresConfig {
  const row = reasoningWiresConfigGuardObject(value, at);
  return {
    dragNodeId: row["dragNodeId"] === undefined ? undefined : reasoningWiresConfigGuardString(row["dragNodeId"], `${at}.dragNodeId`),
    dragLastX: reasoningWiresConfigGuardNumber(row["dragLastX"], `${at}.dragLastX`),
    dragLastY: reasoningWiresConfigGuardNumber(row["dragLastY"], `${at}.dragLastY`),
  };
}
