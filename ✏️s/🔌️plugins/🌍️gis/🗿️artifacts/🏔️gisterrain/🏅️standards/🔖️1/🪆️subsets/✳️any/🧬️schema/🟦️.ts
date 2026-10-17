/** 🏔️ Terrain document data and exact durable mesh child identity. */
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import { parseArtifactChild, type ArtifactChild } from "@semio-tech/framework-os/modules/store/child/schema";
export type { ArtifactChild } from "@semio-tech/framework-os/modules/store/child/schema";

import { type Binary64 } from "@semio-tech/framework/value/ieee754";
import { parseBinary64 } from "@semio-tech/framework/value/ieee754";
import {parseImportedMap,type ImportedMap} from "./🗺️imported-map/🟦️.ts";
export {parseImportedMap} from "./🗺️imported-map/🟦️.ts";
export type {ImportedMap,ImportedFeature} from "./🗺️imported-map/🟦️.ts";
export interface GisTerrainArtifact {
  /** @state artifact */ exaggeration: Binary64;
  /** @state artifact */ importedMap?: ImportedMap;
  /** @state artifact @child kind=s.stdio.semio */ mesh?: ArtifactChild;
}

/** 🪪️ Parses the document boundary independently of window or OS settings. */
export function parseGisTerrainArtifact(value: unknown, at = "$"): GisTerrainArtifact {
  const row = parseSchemaRecord(value, ["exaggeration", "importedMap", "mesh"], at);
  const exaggeration = parseBinary64(row.exaggeration);
  const document: GisTerrainArtifact = { exaggeration };
  if(row.importedMap!==undefined)document.importedMap=parseImportedMap(row.importedMap);
  if (row.mesh != null) document.mesh = parseArtifactChild(row.mesh);
  if(document.mesh)for(const value of[document.mesh.childId,document.mesh.target.artifactId,document.mesh.target.dialect.artifactKind,document.mesh.target.dialect.standard,document.mesh.target.dialect.subset])if(/[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(value))throw Error(`${at}.mesh: native UTF8 required`);
  return document;
}
