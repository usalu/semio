import { type Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseBinary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🔺️ Sparse Terrain changes own named fields and optional imported-map intent. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseGisTerrainArtifact,parseImportedMap,type ImportedMap, type GisTerrainArtifact } from "../🟦️.ts";
export type { GisTerrainArtifact } from "../🟦️.ts";

export interface GisTerrainDiff {
  /** @state artifact */ exaggeration: Binary64 | null;
  /** @state artifact */ importedMap: {value?:ImportedMap} | null;
}

/** 🪪️ Applies parent field deltas while retaining independently owned mesh identity. */
export function applyGisTerrainDiff(snapshot: GisTerrainArtifact, diff: GisTerrainDiff): GisTerrainArtifact {
  const result={...snapshot,exaggeration:diff.exaggeration??snapshot.exaggeration};
  if(diff.importedMap){if(diff.importedMap.value===undefined)delete result.importedMap;else result.importedMap=diff.importedMap.value;}return result;
}

/** 🧮️ Normalizes omitted values to native unchanged null. */
export function parseGisTerrainDiff(value: unknown, at = "$"): GisTerrainDiff {
  const row = parseSchemaRecord(value, ["exaggeration", "importedMap"], at);

  let importedMap:GisTerrainDiff["importedMap"]=null;
  if(row.importedMap!=null){const change=parseSchemaRecord(row.importedMap,["value"]);importedMap=change.value===undefined?{}:{value:parseImportedMap(change.value)};}
  return {
    exaggeration: row.exaggeration == null ? null : parseBinary64(row.exaggeration),
    importedMap,
  };
}
