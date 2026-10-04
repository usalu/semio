/** 🧬️ Writer persisted snapshot schema. */
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
  text: string;
  /** @state artifact @child kind=s.stdio.semio */
  document: ArtifactChild;
}

/** 🪪️ Validates the persisted Writer document boundary. */
export function parseWriterArtifact(value: unknown, at = "$"): WriterArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: snapshot must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["schema", "id", "languageId", "uri", "text", "document"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: snapshot fields do not match its schema`);
  for (const key of keys.slice(0, 5)) if (typeof row[key] !== "string" || /[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(row[key] as string)) throw new Error(`${at}.${key}: value must be a string`);
  const document=parseArtifactChild(row.document);
  for(const value of[document.childId,document.target.artifactId,document.target.dialect.artifactKind,document.target.dialect.standard,document.target.dialect.subset])if(/[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/.test(value))throw Error(`${at}.document: native UTF8 required`);
  return {
    schema: row.schema as string,
    id: row.id as string,
    languageId: row.languageId as string,
    uri: row.uri as string,
    text: row.text as string,
    document,
  };
}
