/** 🧬️ GIS map artifact schema — every field with its state class. */

export interface GisMapArtifact {
  /** @state artifact */
  positions: GisMapFeature[];
  /** @state artifact */
  routes: GisMapFeature[];
  /** @state artifact */
  regions: GisMapFeature[];
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  featureSelectionJson: string;
  /** @state presence */
  layerVisibility: Record<string, boolean>;
  /** @state presence */
  layerStrokeScale: Record<string, number>;
  /** @state config */
  cameraJson: string;
  /** @state config */
  renderMode: string;
  /** @state config */
  vectorStyle: string;
  /** @state config */
  lodMode: string;
  /** @state config */
  hoverJson: string;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  selectionMode: string;
  /** @state config */
}

export interface GisMapFeature {
  id: string;
  data: Record<string, unknown>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisGismapArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisGismapArtifactGuardReject = (at: string, why: string): never => {
  throw new gisGismapArtifactGuardRefusal(at, why);
};

type gisGismapArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisGismapArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisGismapArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisGismapArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisGismapArtifactGuardReject(at, "value is not an object");
export const gisGismapArtifactGuardArray = (value: unknown, at: string, bounds: gisGismapArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisGismapArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisGismapArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisGismapArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisGismapArtifactGuardString = (value: unknown, at: string, bounds: gisGismapArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisGismapArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisGismapArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisGismapArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisGismapArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisGismapArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisGismapArtifactGuardReject(at, "value is not a boolean"));
export const gisGismapArtifactGuardNumber = (value: unknown, at: string, bounds: gisGismapArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisGismapArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisGismapArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisGismapArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisGismapArtifactGuardInteger = (value: unknown, at: string, bounds: gisGismapArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisGismapArtifactGuardNumber(value, at, bounds) : gisGismapArtifactGuardReject(at, "value is not an integer");
export const gisGismapArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisGismapArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisGismapArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisGismapArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGisMapArtifact(value: unknown, at = "$"): GisMapArtifact {
  const row = gisGismapArtifactGuardObject(value, at);
  return {
    positions: gisGismapArtifactGuardArray(row["positions"], `${at}.positions`).map((item, index) => parseGisMapFeature(item, `${at}.positions[${index}]`)),
    routes: gisGismapArtifactGuardArray(row["routes"], `${at}.routes`).map((item, index) => parseGisMapFeature(item, `${at}.routes[${index}]`)),
    regions: gisGismapArtifactGuardArray(row["regions"], `${at}.regions`).map((item, index) => parseGisMapFeature(item, `${at}.regions[${index}]`)),
    selectedIds: gisGismapArtifactGuardArray(row["selectedIds"], `${at}.selectedIds`).map((item, index) => gisGismapArtifactGuardString(item, `${at}.selectedIds[${index}]`)),
    featureSelectionJson: gisGismapArtifactGuardString(row["featureSelectionJson"], `${at}.featureSelectionJson`),
    layerVisibility: gisGismapArtifactGuardObject(row["layerVisibility"], `${at}.layerVisibility`),
    layerStrokeScale: gisGismapArtifactGuardObject(row["layerStrokeScale"], `${at}.layerStrokeScale`),
    cameraJson: gisGismapArtifactGuardString(row["cameraJson"], `${at}.cameraJson`),
    renderMode: gisGismapArtifactGuardString(row["renderMode"], `${at}.renderMode`),
    vectorStyle: gisGismapArtifactGuardString(row["vectorStyle"], `${at}.vectorStyle`),
    lodMode: gisGismapArtifactGuardString(row["lodMode"], `${at}.lodMode`),
    hoverJson: gisGismapArtifactGuardString(row["hoverJson"], `${at}.hoverJson`),
    selectionMethod: gisGismapArtifactGuardString(row["selectionMethod"], `${at}.selectionMethod`),
    selectionMode: gisGismapArtifactGuardString(row["selectionMode"], `${at}.selectionMode`),
  };
}

export function parseGisMapFeature(value: unknown, at = "$"): GisMapFeature {
  const row = gisGismapArtifactGuardObject(value, at);
  return {
    id: gisGismapArtifactGuardString(row["id"], `${at}.id`),
    data: gisGismapArtifactGuardObject(row["data"], `${at}.data`),
  };
}

export interface GisMapBoolEntry {
  readonly key: string;
  readonly value: boolean;
}

export function parseGisMapBoolEntry(value: unknown, at = "$"): GisMapBoolEntry {
  const row = gisGismapArtifactGuardObject(value, at);
  return {
    key: gisGismapArtifactGuardString(row["key"], `${at}.key`),
    value: gisGismapArtifactGuardBoolean(row["value"], `${at}.value`),
  };
}

export interface GisMapNumberEntry {
  readonly key: string;
  readonly value: number;
}

export function parseGisMapNumberEntry(value: unknown, at = "$"): GisMapNumberEntry {
  const row = gisGismapArtifactGuardObject(value, at);
  return {
    key: gisGismapArtifactGuardString(row["key"], `${at}.key`),
    value: gisGismapArtifactGuardNumber(row["value"], `${at}.value`),
  };
}
