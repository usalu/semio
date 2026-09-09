/** 🔺️ Map document deltas reuse the canonical artifact and feature records. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseGisMapArtifact, parseGisMapFeatures, type GisMapArtifact, type GisMapFeature } from "../🟦️.ts";
import { parseGisMapFeaturePatch, type GisMapFeaturePatch } from "../📍️feature/🟦️.ts";
export type { GisMapArtifact, GisMapFeature } from "../🟦️.ts";
export type { GisMapFeaturePatch } from "../📍️feature/🟦️.ts";
export interface GisMapDiff {
  /** @state artifact */ artifact: GisMapArtifact | null;
  /** @state artifact */ positions: GisMapFeaturesDelta | null;
  /** @state artifact */ routes: GisMapFeaturesDelta | null;
  /** @state artifact */ regions: GisMapFeaturesDelta | null;
}
export interface GisMapFeaturesDelta { added: GisMapFeature[]; removed: string[]; patched: GisMapFeaturePatchEntry[]; reordered: string[] | null; }
export interface GisMapFeaturePatchEntry { id: string; patch: GisMapFeaturePatch; }

function strings(value: unknown, at: string): string[] {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string")) throw new Error(`${at}: string array required`);
  return value as string[];
}

/** 🩹️ Parses a patch by its exact target identity. */
export function parseGisMapFeaturePatchEntry(value: unknown, at = "$"): GisMapFeaturePatchEntry {
  const row = parseSchemaRecord(value, ["id", "patch"], at);
  if (typeof row.id !== "string") throw new Error(`${at}.id: string required`);
  return { id: row.id, patch: parseGisMapFeaturePatch(row.patch, `${at}.patch`) };
}

/** 📚️ Uses native empty collection defaults for absent delta components. */
export function parseGisMapFeaturesDelta(value: unknown, at = "$"): GisMapFeaturesDelta {
  const row = parseSchemaRecord(value, ["added", "removed", "patched", "reordered"], at);
  if (row.patched !== undefined && !Array.isArray(row.patched)) throw new Error(`${at}.patched: array required`);
  return {
    added: parseGisMapFeatures(row.added === undefined ? [] : row.added, `${at}.added`),
    removed: strings(row.removed === undefined ? [] : row.removed, `${at}.removed`),
    patched: ((row.patched ?? []) as unknown[]).map((item, index) => parseGisMapFeaturePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row.reordered == null ? null : strings(row.reordered, `${at}.reordered`),
  };
}

/** 🧮️ Resolves absent delta fields to native unchanged null. */
export function parseGisMapDiff(value: unknown, at = "$"): GisMapDiff {
  const row = parseSchemaRecord(value, ["artifact", "positions", "routes", "regions"], at);
  return {
    artifact: row.artifact == null ? null : parseGisMapArtifact(row.artifact, `${at}.artifact`),
    positions: row.positions == null ? null : parseGisMapFeaturesDelta(row.positions, `${at}.positions`),
    routes: row.routes == null ? null : parseGisMapFeaturesDelta(row.routes, `${at}.routes`),
    regions: row.regions == null ? null : parseGisMapFeaturesDelta(row.regions, `${at}.regions`),
  };
}
