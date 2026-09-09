/** 🧬️ Writer artifact schema. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface WriterArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  languageId: string;
  /** @state artifact */
  uri: string;
  /** @state artifact */
  document: ArtifactChild;
}

/** 🪪️ Validates the durable Writer document boundary. */
export function parseWriterArtifact(value: unknown, at = "$"): WriterArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: artifact must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "id", "languageId", "uri", "document"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: artifact fields do not match its schema`);
  for (const key of keys.slice(0, 4)) if (typeof row[key] !== "string") throw new Error(`${at}.${key}: value must be a string`);
  return {
    schema: row.schema as string,
    id: row.id as string,
    languageId: row.languageId as string,
    uri: row.uri as string,
    document: parseArtifactChild(row.document, `${at}.document`),
  };
}
