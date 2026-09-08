/** 🧬️ GIS terrain snapshot schema — artifact-lane fields only. */

export interface GisTerrainSnapshot {
  /** @state artifact */
  exaggeration: number;
  /** @state artifact */
  importedFeaturesJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainSnapshotGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainSnapshotGuardRefusal(at, why);
};

type gisGisterrainSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainSnapshotGuardReject(at, "value is not an object");
export const gisGisterrainSnapshotGuardArray = (value: unknown, at: string, bounds: gisGisterrainSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainSnapshotGuardString = (value: unknown, at: string, bounds: gisGisterrainSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainSnapshotGuardReject(at, "value is not a boolean"));
export const gisGisterrainSnapshotGuardNumber = (value: unknown, at: string, bounds: gisGisterrainSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainSnapshotGuardInteger = (value: unknown, at: string, bounds: gisGisterrainSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainSnapshotGuardNumber(value, at, bounds) : gisGisterrainSnapshotGuardReject(at, "value is not an integer");
export const gisGisterrainSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainSnapshot(value: unknown, at = "$"): GisTerrainSnapshot {
  const row = gisGisterrainSnapshotGuardObject(value, at);
  return {
    exaggeration: gisGisterrainSnapshotGuardNumber(row["exaggeration"], `${at}.exaggeration`),
    importedFeaturesJson: gisGisterrainSnapshotGuardString(row["importedFeaturesJson"], `${at}.importedFeaturesJson`),
  };
}
