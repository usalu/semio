/** 🧬️ Forms document fields reference their canonical composed child owners. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDslValue } from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

export interface FormsArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  structure: ArtifactChild;
  /** @state artifact */
  results: ArtifactChild;
}

/** 🪪️ Validates exact document fields and normalizes an absent native optional title. */
export function parseFormsArtifact(value: unknown, at = "$"): FormsArtifact {
  parseDslValue(value);
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a Forms document`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "id", "version", "title", "structure", "results"];
  if (Object.keys(row).some((key) => !keys.includes(key)) || ["schema", "id", "version"].some((key) => typeof row[key] !== "string") || (row.title != null && typeof row.title !== "string")) throw new Error(`${at}: invalid Forms document fields`);
  return { schema: row.schema as string, id: row.id as string, version: row.version as string, ...(row.title == null ? {} : { title: row.title as string }), structure: parseArtifactChild(row.structure), results: parseArtifactChild(row.results) };
}
