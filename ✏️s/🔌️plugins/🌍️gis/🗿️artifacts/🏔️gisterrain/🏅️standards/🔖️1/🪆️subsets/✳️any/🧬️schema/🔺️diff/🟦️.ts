/** 🔺️ Sparse Terrain changes preserve the complete replacement artifact. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseGisTerrainArtifact, type GisTerrainArtifact } from "../🟦️.ts";
export type { GisTerrainArtifact } from "../🟦️.ts";

export interface GisTerrainDiff {
  /** @state artifact */ artifact: GisTerrainArtifact | null;
  /** @state artifact */ exaggeration: number | null;
  /** @state artifact */ importedFeaturesJson: string | null;
}

/** 🪪️ Applies parent field deltas while retaining independently owned mesh identity. */
export function applyGisTerrainDiff(snapshot: GisTerrainArtifact, diff: GisTerrainDiff): GisTerrainArtifact {
  if (diff.artifact) return { ...diff.artifact };
  return { ...snapshot, exaggeration: diff.exaggeration ?? snapshot.exaggeration, importedFeaturesJson: diff.importedFeaturesJson ?? snapshot.importedFeaturesJson };
}

/** 🧮️ Normalizes omitted values to native unchanged null. */
export function parseGisTerrainDiff(value: unknown, at = "$"): GisTerrainDiff {
  const row = parseSchemaRecord(value, ["artifact", "exaggeration", "importedFeaturesJson"], at);
  if (row.exaggeration != null && (typeof row.exaggeration !== "number" || !Number.isFinite(row.exaggeration))) throw new Error(`${at}.exaggeration: finite number or null required`);
  if (row.importedFeaturesJson != null && typeof row.importedFeaturesJson !== "string") throw new Error(`${at}.importedFeaturesJson: string or null required`);
  return {
    artifact: row.artifact == null ? null : parseGisTerrainArtifact(row.artifact, `${at}.artifact`),
    exaggeration: (row.exaggeration ?? null) as number | null,
    importedFeaturesJson: (row.importedFeaturesJson ?? null) as string | null,
  };
}
