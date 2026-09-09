/** 🧬️ GIS terrain artifact schema — every field with its state class. */

export interface GisTerrainArtifact {
  /** @state artifact */
  exaggeration: number;
  /** @state artifact */
  importedFeaturesJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainArtifactGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainArtifactGuardRefusal(at, why);
};

type gisGisterrainArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainArtifactGuardReject(at, "value is not an object");
export const gisGisterrainArtifactGuardArray = (value: unknown, at: string, bounds: gisGisterrainArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainArtifactGuardString = (value: unknown, at: string, bounds: gisGisterrainArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainArtifactGuardReject(at, "value is not a boolean"));
export const gisGisterrainArtifactGuardNumber = (value: unknown, at: string, bounds: gisGisterrainArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainArtifactGuardInteger = (value: unknown, at: string, bounds: gisGisterrainArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainArtifactGuardNumber(value, at, bounds) : gisGisterrainArtifactGuardReject(at, "value is not an integer");
export const gisGisterrainArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainArtifact(value: unknown, at = "$"): GisTerrainArtifact {
  const row = gisGisterrainArtifactGuardObject(value, at);
  return {
    exaggeration: gisGisterrainArtifactGuardNumber(row["exaggeration"], `${at}.exaggeration`),
    importedFeaturesJson: gisGisterrainArtifactGuardString(row["importedFeaturesJson"], `${at}.importedFeaturesJson`),
  };
}
