/** 🧬️ GIS map diff schema — sparse field delta. */

export interface GisMapDiff {
  /** @state artifact */
  artifact?: GisMapArtifact;
  /** @state artifact */
  positions?: GisMapFeaturesDelta;
  /** @state artifact */
  routes?: GisMapFeaturesDelta;
  /** @state artifact */
  regions?: GisMapFeaturesDelta;
}

export interface GisMapArtifact {
  positions: GisMapFeature[];
  routes: GisMapFeature[];
  regions: GisMapFeature[];
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}

export interface GisMapStringList { values: string[]; }
export interface GisMapBoolMapDelta { entries: Record<string, boolean | null>; }
export interface GisMapNumberMapDelta { entries: Record<string, number | null>; }
export interface GisMapFeaturesDelta {
  added: GisMapFeature[];
  removed: string[];
  patched: GisMapFeaturePatchEntry[];
  reordered?: string[];
}
export interface GisMapFeaturePatchEntry {
  id: string;
  patch: GisMapFeaturePatch;
}
export interface GisMapFeaturePatch {
  data?: Record<string, unknown>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGismapDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGismapDiffGuardReject = (at: string, why: string): never => {
  throw new gisGismapDiffGuardRefusal(at, why);
};

type gisGismapDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGismapDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGismapDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGismapDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGismapDiffGuardReject(at, "value is not an object");
export const gisGismapDiffGuardArray = (value: unknown, at: string, bounds: gisGismapDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGismapDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGismapDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGismapDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGismapDiffGuardString = (value: unknown, at: string, bounds: gisGismapDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGismapDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGismapDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGismapDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGismapDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGismapDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGismapDiffGuardReject(at, "value is not a boolean"));
export const gisGismapDiffGuardNumber = (value: unknown, at: string, bounds: gisGismapDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGismapDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGismapDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGismapDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGismapDiffGuardInteger = (value: unknown, at: string, bounds: gisGismapDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGismapDiffGuardNumber(value, at, bounds) : gisGismapDiffGuardReject(at, "value is not an integer");
export const gisGismapDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGismapDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGismapDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGismapDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisMapDiff(value: unknown, at = "$"): GisMapDiff {
  const row = gisGismapDiffGuardObject(value, at);
  return {
    artifact: row["artifact"] === undefined ? undefined : gisGismapDiffGuardObject(row["artifact"], `${at}.artifact`),
    positions: row["positions"] === undefined ? undefined : parseGisMapFeaturesDelta(row["positions"], `${at}.positions`),
    routes: row["routes"] === undefined ? undefined : parseGisMapFeaturesDelta(row["routes"], `${at}.routes`),
    regions: row["regions"] === undefined ? undefined : parseGisMapFeaturesDelta(row["regions"], `${at}.regions`),
  };
}

export function parseGisMapStringList(value: unknown, at = "$"): GisMapStringList {
  const row = gisGismapDiffGuardObject(value, at);
  return {
    values: gisGismapDiffGuardArray(row["values"], `${at}.values`).map((item, index) => gisGismapDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseGisMapBoolMapDelta(value: unknown, at = "$"): GisMapBoolMapDelta {
  const row = gisGismapDiffGuardObject(value, at);
  return {
    entries: gisGismapDiffGuardObject(row["entries"], `${at}.entries`),
  };
}

export function parseGisMapNumberMapDelta(value: unknown, at = "$"): GisMapNumberMapDelta {
  const row = gisGismapDiffGuardObject(value, at);
  return {
    entries: gisGismapDiffGuardObject(row["entries"], `${at}.entries`),
  };
}

export function parseGisMapFeaturePatchEntry(value: unknown, at = "$"): GisMapFeaturePatchEntry {
  const row = gisGismapDiffGuardObject(value, at);
  return {
    id: gisGismapDiffGuardString(row["id"], `${at}.id`),
    patch: parseGisMapFeaturePatch(row["patch"], `${at}.patch`),
  };
}

export function parseGisMapFeaturePatch(value: unknown, at = "$"): GisMapFeaturePatch {
  const row = gisGismapDiffGuardObject(value, at);
  return {
    data: row["data"] === undefined ? undefined : gisGismapDiffGuardObject(row["data"], `${at}.data`),
  };
}
