/** 🧬️ Wires snapshot schema — every field with its state class. */

export type DslValue = Record<string, unknown>;

export interface WiresSnapshot {
  /** @state artifact */
  wiresFixture: DslValue;
  /** @state artifact */
  boardFixture: DslValue;
}

export interface WiresStringList {
  values: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class reasoningWiresSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reasoningWiresSnapshotGuardReject = (at: string, why: string): never => {
  throw new reasoningWiresSnapshotGuardRefusal(at, why);
};

type reasoningWiresSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type reasoningWiresSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type reasoningWiresSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const reasoningWiresSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : reasoningWiresSnapshotGuardReject(at, "value is not an object");
export const reasoningWiresSnapshotGuardArray = (value: unknown, at: string, bounds: reasoningWiresSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return reasoningWiresSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) reasoningWiresSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) reasoningWiresSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const reasoningWiresSnapshotGuardString = (value: unknown, at: string, bounds: reasoningWiresSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return reasoningWiresSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) reasoningWiresSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) reasoningWiresSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) reasoningWiresSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const reasoningWiresSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : reasoningWiresSnapshotGuardReject(at, "value is not a boolean"));
export const reasoningWiresSnapshotGuardNumber = (value: unknown, at: string, bounds: reasoningWiresSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return reasoningWiresSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) reasoningWiresSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) reasoningWiresSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const reasoningWiresSnapshotGuardInteger = (value: unknown, at: string, bounds: reasoningWiresSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? reasoningWiresSnapshotGuardNumber(value, at, bounds) : reasoningWiresSnapshotGuardReject(at, "value is not an integer");
export const reasoningWiresSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : reasoningWiresSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const reasoningWiresSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : reasoningWiresSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWiresSnapshot(value: unknown, at = "$"): WiresSnapshot {
  const row = reasoningWiresSnapshotGuardObject(value, at);
  return {
    wiresFixture: reasoningWiresSnapshotGuardObject(row["wiresFixture"], `${at}.wiresFixture`),
    boardFixture: reasoningWiresSnapshotGuardObject(row["boardFixture"], `${at}.boardFixture`),
  };
}

export function parseWiresStringList(value: unknown, at = "$"): WiresStringList {
  const row = reasoningWiresSnapshotGuardObject(value, at);
  return {
    values: reasoningWiresSnapshotGuardArray(row["values"], `${at}.values`).map((item, index) => reasoningWiresSnapshotGuardString(item, `${at}.values[${index}]`)),
  };
}
