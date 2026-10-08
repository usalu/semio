/** 🔺️ Map document deltas reuse the canonical artifact and feature records. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseGisMapArtifact, parseGisMapFeatures, type GisMapArtifact, type GisMapFeature } from "../🟦️.ts";
import { parseGisMapFeaturePatch, type GisMapFeaturePatch } from "../📍️feature/🟦️.ts";
export type { GisMapArtifact, GisMapFeature } from "../🟦️.ts";
export type { GisMapFeaturePatch } from "../📍️feature/🟦️.ts";
export interface GisMapDiff {
  /** @state artifact */ positions: GisMapFeaturesDelta | null;
  /** @state artifact */ routes: GisMapFeaturesDelta | null;
  /** @state artifact */ regions: GisMapFeaturesDelta | null;
}
export interface GisMapFeatureRemoval { id: string; index: number; }
export interface GisMapFeatureInsertion { index: number; row: GisMapFeature; }
export interface GisMapFeatureRelocation { id: string; from: number; to: number; }
/** Positional delta: `removed` rows carry their base index, `inserted` rows their after index, `moved` rows both; no order list. */
export interface GisMapFeaturesDelta { removed: GisMapFeatureRemoval[]; inserted: GisMapFeatureInsertion[]; moved: GisMapFeatureRelocation[]; modified: GisMapFeatureModification[]; }
export interface GisMapFeatureModification { id: string; patch: GisMapFeaturePatch; }

function index(value: unknown, at: string): number {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0) throw new Error(`${at}: non-negative integer required`);
  return value;
}

function rows<T>(value: unknown, at: string, parse: (item: unknown, at: string) => T): T[] {
  if (value === undefined) return [];
  if (!Array.isArray(value)) throw new Error(`${at}: array required`);
  return value.map((item, position) => parse(item, `${at}[${position}]`));
}

/** 🩹️ Parses a patch by its exact target identity. */
export function parseGisMapFeatureModification(value: unknown, at = "$"): GisMapFeatureModification {
  const row = parseSchemaRecord(value, ["id", "patch"], at);
  if (typeof row.id !== "string") throw new Error(`${at}.id: string required`);
  return { id: row.id, patch: parseGisMapFeaturePatch(row.patch, `${at}.patch`) };
}

/** 📚️ Uses native empty collection defaults for absent delta components. */
export function parseGisMapFeaturesDelta(value: unknown, at = "$"): GisMapFeaturesDelta {
  const row = parseSchemaRecord(value, ["removed", "inserted", "moved", "modified"], at);
  return {
    removed: rows(row.removed, `${at}.removed`, (item, where) => {
      const entry = parseSchemaRecord(item, ["id", "index"], where);
      if (typeof entry.id !== "string") throw new Error(`${where}.id: string required`);
      return { id: entry.id, index: index(entry.index, `${where}.index`) };
    }),
    inserted: rows(row.inserted, `${at}.inserted`, (item, where) => {
      const entry = parseSchemaRecord(item, ["index", "row"], where);
      return { index: index(entry.index, `${where}.index`), row: parseGisMapFeatures([entry.row], `${where}.row`)[0] };
    }),
    moved: rows(row.moved, `${at}.moved`, (item, where) => {
      const entry = parseSchemaRecord(item, ["id", "from", "to"], where);
      if (typeof entry.id !== "string") throw new Error(`${where}.id: string required`);
      return { id: entry.id, from: index(entry.from, `${where}.from`), to: index(entry.to, `${where}.to`) };
    }),
    modified: rows(row.modified, `${at}.modified`, parseGisMapFeatureModification),
  };
}

/** 🧮️ Resolves absent delta fields to native unchanged null. */
export function parseGisMapDiff(value: unknown, at = "$"): GisMapDiff {
  const row = parseSchemaRecord(value, ["positions", "routes", "regions"], at);
  return {
    positions: row.positions == null ? null : parseGisMapFeaturesDelta(row.positions, `${at}.positions`),
    routes: row.routes == null ? null : parseGisMapFeaturesDelta(row.routes, `${at}.routes`),
    regions: row.regions == null ? null : parseGisMapFeaturesDelta(row.regions, `${at}.regions`),
  };
}
