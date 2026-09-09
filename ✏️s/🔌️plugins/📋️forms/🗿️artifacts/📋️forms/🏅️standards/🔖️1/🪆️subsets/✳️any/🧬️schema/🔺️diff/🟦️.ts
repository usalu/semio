/** 🔺️ Forms sparse document delta references OS Store child identities. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";

export interface FormsDiff {
  /** @state artifact */
  schema?: string | null;
  /** @state artifact */
  id?: string | null;
  /** @state artifact */
  version?: string | null;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  structure?: ArtifactChild | null;
  /** @state artifact */
  results?: ArtifactChild | null;
}

/** 🔺️ Decodes the native sparse slots with exact owner and child validation. */
export function parseFormsDiff(value: unknown, at = "$"): FormsDiff {
  parseDslValue(value);
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected a Forms diff`);
  const row = value as Record<string, unknown>;
  const strings = ["schema", "id", "version", "title"];
  if (Object.keys(row).some((key) => ![...strings, "structure", "results"].includes(key)) || strings.some((key) => row[key] != null && typeof row[key] !== "string")) throw new Error(`${at}: invalid Forms diff fields`);
  return { schema: row.schema as string ?? null, id: row.id as string ?? null, version: row.version as string ?? null, title: row.title as string ?? null, structure: row.structure == null ? null : parseArtifactChild(row.structure), results: row.results == null ? null : parseArtifactChild(row.results) };
}
