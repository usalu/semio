/** 🏔️ Terrain document data and exact durable mesh child identity. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface GisTerrainArtifact {
  /** @state artifact */ exaggeration: number;
  /** @state artifact */ importedFeaturesJson: string;
  /** @state artifact @child kind=s.stdio.semio */ mesh?: ArtifactChild;
}

/** 🪪️ Parses the document boundary independently of window or OS settings. */
export function parseGisTerrainArtifact(value: unknown, at = "$"): GisTerrainArtifact {
  const row = parseSchemaRecord(value, ["exaggeration", "importedFeaturesJson", "mesh"], at);
  if (typeof row.exaggeration !== "number" || !Number.isFinite(row.exaggeration)) throw new Error(`${at}.exaggeration: finite number required`);
  if (typeof row.importedFeaturesJson !== "string") throw new Error(`${at}.importedFeaturesJson: string required`);
  const document: GisTerrainArtifact = { exaggeration: row.exaggeration, importedFeaturesJson: row.importedFeaturesJson };
  if (row.mesh != null) document.mesh = parseArtifactChild(row.mesh);
  return document;
}
