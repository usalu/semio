/** 🏔️ Terrain document data and exact durable mesh child identity. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

import {parseBinary64,type Binary64} from "../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export interface GisTerrainArtifact {
  /** @state artifact */ exaggeration: Binary64;
  /** @state artifact */ importedFeaturesJson: string;
  /** @state artifact @child kind=s.stdio.semio */ mesh?: ArtifactChild;
}

/** 🪪️ Parses the document boundary independently of window or OS settings. */
export function parseGisTerrainArtifact(value: unknown, at = "$"): GisTerrainArtifact {
  const row = parseSchemaRecord(value, ["exaggeration", "importedFeaturesJson", "mesh"], at);
  const exaggeration = parseBinary64(row.exaggeration);
  if (typeof row.importedFeaturesJson !== "string") throw new Error(`${at}.importedFeaturesJson: string required`);
  const document: GisTerrainArtifact = { exaggeration, importedFeaturesJson: row.importedFeaturesJson };
  if (row.mesh != null) document.mesh = parseArtifactChild(row.mesh);
  return document;
}
