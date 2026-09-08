/** 🧬️ Playground snapshot schema — artifact-lane fields only. */

export interface PlaygroundSnapshot {
  /** @state artifact */
  schema: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class demonstratorPlaygroundSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const demonstratorPlaygroundSnapshotGuardReject = (at: string, why: string): never => {
  throw new demonstratorPlaygroundSnapshotGuardRefusal(at, why);
};

type demonstratorPlaygroundSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type demonstratorPlaygroundSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type demonstratorPlaygroundSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const demonstratorPlaygroundSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : demonstratorPlaygroundSnapshotGuardReject(at, "value is not an object");
export const demonstratorPlaygroundSnapshotGuardArray = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return demonstratorPlaygroundSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) demonstratorPlaygroundSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) demonstratorPlaygroundSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const demonstratorPlaygroundSnapshotGuardString = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return demonstratorPlaygroundSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) demonstratorPlaygroundSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) demonstratorPlaygroundSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) demonstratorPlaygroundSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const demonstratorPlaygroundSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : demonstratorPlaygroundSnapshotGuardReject(at, "value is not a boolean"));
export const demonstratorPlaygroundSnapshotGuardNumber = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return demonstratorPlaygroundSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) demonstratorPlaygroundSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) demonstratorPlaygroundSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const demonstratorPlaygroundSnapshotGuardInteger = (value: unknown, at: string, bounds: demonstratorPlaygroundSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? demonstratorPlaygroundSnapshotGuardNumber(value, at, bounds) : demonstratorPlaygroundSnapshotGuardReject(at, "value is not an integer");
export const demonstratorPlaygroundSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : demonstratorPlaygroundSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const demonstratorPlaygroundSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : demonstratorPlaygroundSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlaygroundSnapshot(value: unknown, at = "$"): PlaygroundSnapshot {
  const row = demonstratorPlaygroundSnapshotGuardObject(value, at);
  return {
    schema: demonstratorPlaygroundSnapshotGuardString(row["schema"], `${at}.schema`),
  };
}
