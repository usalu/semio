/** 🧬️ GIS terrain diff schema — sparse field delta. */

export interface GisTerrainDiff {
  /** @state artifact */
  artifact?: GisTerrainArtifact;
  /** @state artifact */
  exaggeration?: number;
  /** @state artifact */
  importedFeaturesJson?: string;
  /** @state presence */
  selectedIds?: GisTerrainStringList;
  /** @state config */
  cameraJson?: string;
  /** @state config */
}

export interface GisTerrainArtifact {
  exaggeration: number;
  importedFeaturesJson: string;
  selectedIds: string[];
  cameraJson: string;
}

export interface GisTerrainStringList {
  values: string[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGisterrainDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGisterrainDiffGuardReject = (at: string, why: string): never => {
  throw new gisGisterrainDiffGuardRefusal(at, why);
};

type gisGisterrainDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGisterrainDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGisterrainDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGisterrainDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGisterrainDiffGuardReject(at, "value is not an object");
export const gisGisterrainDiffGuardArray = (value: unknown, at: string, bounds: gisGisterrainDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGisterrainDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGisterrainDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGisterrainDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGisterrainDiffGuardString = (value: unknown, at: string, bounds: gisGisterrainDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGisterrainDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGisterrainDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGisterrainDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGisterrainDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGisterrainDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGisterrainDiffGuardReject(at, "value is not a boolean"));
export const gisGisterrainDiffGuardNumber = (value: unknown, at: string, bounds: gisGisterrainDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGisterrainDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGisterrainDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGisterrainDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGisterrainDiffGuardInteger = (value: unknown, at: string, bounds: gisGisterrainDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGisterrainDiffGuardNumber(value, at, bounds) : gisGisterrainDiffGuardReject(at, "value is not an integer");
export const gisGisterrainDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGisterrainDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGisterrainDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGisterrainDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisTerrainDiff(value: unknown, at = "$"): GisTerrainDiff {
  const row = gisGisterrainDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === undefined ? undefined : gisGisterrainDiffGuardObject(row["artifact"], `${at}.artifact`),
    exaggeration: row["exaggeration"] === undefined ? undefined : gisGisterrainDiffGuardNumber(row["exaggeration"], `${at}.exaggeration`),
    importedFeaturesJson: row["importedFeaturesJson"] === undefined ? undefined : gisGisterrainDiffGuardString(row["importedFeaturesJson"], `${at}.importedFeaturesJson`),
    selectedIds: row["selectedIds"] === undefined ? undefined : parseGisTerrainStringList(row["selectedIds"], `${at}.selectedIds`),
    cameraJson: row["cameraJson"] === undefined ? undefined : gisGisterrainDiffGuardString(row["cameraJson"], `${at}.cameraJson`),
  };
}

export function parseGisTerrainStringList(value: unknown, at = "$"): GisTerrainStringList {
  const row = gisGisterrainDiffGuardObject(value, at);
  return {
    values: gisGisterrainDiffGuardArray(row["values"], `${at}.values`).map((item, index) => gisGisterrainDiffGuardString(item, `${at}.values[${index}]`)),
  };
}
