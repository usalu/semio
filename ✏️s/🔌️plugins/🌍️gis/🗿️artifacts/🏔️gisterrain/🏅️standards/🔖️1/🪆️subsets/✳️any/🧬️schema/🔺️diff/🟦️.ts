import { type Binary64 } from "@semio-tech/framework/value/ieee754";
import { parseBinary64 } from "@semio-tech/framework/value/ieee754";
/** 🔺️ Sparse Terrain changes own named fields and optional imported-map intent. */
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import { parseGisTerrainArtifact,parseImportedMap,type ImportedMap, type GisTerrainArtifact } from "../🟦️.ts";
import { parseElevationSource, type ElevationSource } from "../🟦️.ts";
export type { GisTerrainArtifact } from "../🟦️.ts";

export interface GisTerrainDiff {
  /** ✏️ @state artifact */ exaggeration: Binary64 | null;
  /** 🔄️ @state artifact */ importedMap: {value?:ImportedMap} | null;
  /** 🏔️ @state artifact */ elevationSource?: {value?: ElevationSource} | null;
}

/** 🪪️ Applies parent field deltas while retaining independently owned mesh identity. */
export function applyGisTerrainDiff(snapshot: GisTerrainArtifact, diff: GisTerrainDiff): GisTerrainArtifact {
  const result={...snapshot,exaggeration:diff.exaggeration??snapshot.exaggeration};
  if(diff.importedMap){if(diff.importedMap.value===undefined)delete result.importedMap;else result.importedMap=diff.importedMap.value;}
  if (diff.elevationSource) {
    if (diff.elevationSource.value === undefined) delete result.elevationSource;
    else result.elevationSource = diff.elevationSource.value;
  }
  return result;
}

/** 🧮️ Normalizes omitted values to native unchanged null. */
export function parseGisTerrainDiff(value: unknown, at = "$"): GisTerrainDiff {
  const row = parseSchemaRecord(value, ["exaggeration", "importedMap", "elevationSource"], at);

  let importedMap:GisTerrainDiff["importedMap"]=null;
  if(row.importedMap!=null){const change=parseSchemaRecord(row.importedMap,["value"]);importedMap=change.value===undefined?{}:{value:parseImportedMap(change.value)};}
  const diff: GisTerrainDiff = {
    exaggeration: row.exaggeration == null ? null : parseBinary64(row.exaggeration),
    importedMap,
  };
  if (row.elevationSource != null) {
    const change = parseSchemaRecord(row.elevationSource, ["value"]);
    diff.elevationSource = change.value === undefined ? {} : { value: parseElevationSource(change.value) };
  }
  return diff;
}
