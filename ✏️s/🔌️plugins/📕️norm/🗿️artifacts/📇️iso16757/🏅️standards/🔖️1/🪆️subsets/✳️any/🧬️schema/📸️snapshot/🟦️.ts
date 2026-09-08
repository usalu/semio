/** 🧬️ Iso16757 snapshot schema — artifact-lane fields only. */

export interface Iso16757Snapshot {
  /** @state artifact */
  catalogue: string;
  /** @state artifact */
  dictionary: string;
  /** @state artifact */
  geometry: string;
  /** @state artifact */
  selection: string;
  /** @state artifact */
  partNumberRule: string;
  /** @state artifact */
  partNumberInputs: Record<string, string>;
  /** @state artifact */
  scriptLimits: string;
  /** @state artifact */
  exchangeProcess: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normIso16757SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normIso16757SnapshotGuardReject = (at: string, why: string): never => {
  throw new normIso16757SnapshotGuardRefusal(at, why);
};

type normIso16757SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normIso16757SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normIso16757SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normIso16757SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normIso16757SnapshotGuardReject(at, "value is not an object");
export const normIso16757SnapshotGuardArray = (value: unknown, at: string, bounds: normIso16757SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normIso16757SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normIso16757SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normIso16757SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normIso16757SnapshotGuardString = (value: unknown, at: string, bounds: normIso16757SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normIso16757SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normIso16757SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normIso16757SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normIso16757SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normIso16757SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normIso16757SnapshotGuardReject(at, "value is not a boolean"));
export const normIso16757SnapshotGuardNumber = (value: unknown, at: string, bounds: normIso16757SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normIso16757SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normIso16757SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normIso16757SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normIso16757SnapshotGuardInteger = (value: unknown, at: string, bounds: normIso16757SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normIso16757SnapshotGuardNumber(value, at, bounds) : normIso16757SnapshotGuardReject(at, "value is not an integer");
export const normIso16757SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normIso16757SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normIso16757SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normIso16757SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIso16757Snapshot(value: unknown, at = "$"): Iso16757Snapshot {
  const row = normIso16757SnapshotGuardObject(value, at);
  return {
    catalogue: normIso16757SnapshotGuardObject(row["catalogue"], `${at}.catalogue`),
    dictionary: normIso16757SnapshotGuardObject(row["dictionary"], `${at}.dictionary`),
    geometry: normIso16757SnapshotGuardObject(row["geometry"], `${at}.geometry`),
    selection: normIso16757SnapshotGuardObject(row["selection"], `${at}.selection`),
    partNumberRule: normIso16757SnapshotGuardObject(row["partNumberRule"], `${at}.partNumberRule`),
    partNumberInputs: normIso16757SnapshotGuardObject(row["partNumberInputs"], `${at}.partNumberInputs`),
    scriptLimits: normIso16757SnapshotGuardObject(row["scriptLimits"], `${at}.scriptLimits`),
    exchangeProcess: normIso16757SnapshotGuardObject(row["exchangeProcess"], `${at}.exchangeProcess`),
  };
}
