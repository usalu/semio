/** 🗺️ Complete map document with independently persisted child envelopes. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseGisMapFeature, type GisMapFeature } from "./📍️feature/🟦️.ts";
export { parseGisMapFeature } from "./📍️feature/🟦️.ts";
export type { GisMapFeature } from "./📍️feature/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface GisMapArtifact {
  /** @state artifact */ positions: GisMapFeature[];
  /** @state artifact */ routes: GisMapFeature[];
  /** @state artifact */ regions: GisMapFeature[];
  /** @state artifact @child kind=s.stdio.semio */ drawing: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ image?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ value: ArtifactChild;
}

/** 📚️ Parses a domain feature collection through its single feature schema. */
export function parseGisMapFeatures(value: unknown, at: string): GisMapFeature[] {
  if (!Array.isArray(value)) throw new Error(`${at}: feature array required`);
  return value.map((item, index) => parseGisMapFeature(item, `${at}[${index}]`));
}

/** 🪪️ Preserves each explicit child identity and rejects foreign UI state. */
export function parseGisMapArtifact(value: unknown, at = "$"): GisMapArtifact {
  const row = parseSchemaRecord(value, ["positions", "routes", "regions", "drawing", "image", "value"], at);
  const document: GisMapArtifact = {
    positions: parseGisMapFeatures(row.positions, `${at}.positions`),
    routes: parseGisMapFeatures(row.routes, `${at}.routes`),
    regions: parseGisMapFeatures(row.regions, `${at}.regions`),
    drawing: parseArtifactChild(row.drawing),
    value: parseArtifactChild(row.value),
  };
  if (row.image != null) document.image = parseArtifactChild(row.image);
  return document;
}
