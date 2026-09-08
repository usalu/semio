/** 🧬️ Vdi3805 snapshot schema — artifact-lane fields only. */

export interface Vdi3805Snapshot {
  /** @state artifact */
  manufacturerFile: string;
  /** @state artifact */
  catalog: string;
  /** @state artifact */
  editionProfile: Record<string, string>;
  /** @state artifact */
  correctionAsOf: string;
  /** @state artifact */
  strictMode: boolean;
  /** @state artifact */
  index: string;
  /** @state artifact */
  geometry: Record<string, string>;
  /** @state artifact */
  curves: Record<string, string>;
  /** @state artifact */
  limits: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normVdi3805SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normVdi3805SnapshotGuardReject = (at: string, why: string): never => {
  throw new normVdi3805SnapshotGuardRefusal(at, why);
};

type normVdi3805SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normVdi3805SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normVdi3805SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normVdi3805SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normVdi3805SnapshotGuardReject(at, "value is not an object");
export const normVdi3805SnapshotGuardArray = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normVdi3805SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normVdi3805SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normVdi3805SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normVdi3805SnapshotGuardString = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normVdi3805SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normVdi3805SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normVdi3805SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normVdi3805SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normVdi3805SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normVdi3805SnapshotGuardReject(at, "value is not a boolean"));
export const normVdi3805SnapshotGuardNumber = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normVdi3805SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normVdi3805SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normVdi3805SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normVdi3805SnapshotGuardInteger = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normVdi3805SnapshotGuardNumber(value, at, bounds) : normVdi3805SnapshotGuardReject(at, "value is not an integer");
export const normVdi3805SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normVdi3805SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normVdi3805SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normVdi3805SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVdi3805Snapshot(value: unknown, at = "$"): Vdi3805Snapshot {
  const row = normVdi3805SnapshotGuardObject(value, at);
  return {
    manufacturerFile: normVdi3805SnapshotGuardString(row["manufacturerFile"], `${at}.manufacturerFile`),
    catalog: normVdi3805SnapshotGuardString(row["catalog"], `${at}.catalog`),
    editionProfile: normVdi3805SnapshotGuardObject(row["editionProfile"], `${at}.editionProfile`),
    correctionAsOf: normVdi3805SnapshotGuardString(row["correctionAsOf"], `${at}.correctionAsOf`),
    strictMode: normVdi3805SnapshotGuardBoolean(row["strictMode"], `${at}.strictMode`),
    index: normVdi3805SnapshotGuardString(row["index"], `${at}.index`),
    geometry: normVdi3805SnapshotGuardObject(row["geometry"], `${at}.geometry`),
    curves: normVdi3805SnapshotGuardObject(row["curves"], `${at}.curves`),
    limits: normVdi3805SnapshotGuardString(row["limits"], `${at}.limits`),
  };
}
