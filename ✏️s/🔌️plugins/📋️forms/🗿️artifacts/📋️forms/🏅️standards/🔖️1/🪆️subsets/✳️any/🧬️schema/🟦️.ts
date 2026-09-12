/** 🧬️ Forms durable document with exact owned-child coordinates. */
import { parseSchemaRecord } from "../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";

export interface FormsArtifact {
  /** @state artifact */ schema: "forms.form";
  /** @state artifact */ id: string;
  /** @state artifact */ version: string;
  /** @state artifact */ title?: string;
  /** @state artifact @child kind=s.stdio.semio */ structure: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ results: ArtifactChild;
}

/** 🪪️ Validates the document marker, declared fields and each exact child dialect. */
export function parseFormsArtifact(value: unknown, at = "$"): FormsArtifact {
  const row = parseSchemaRecord(value, ["schema", "id", "version", "title", "structure", "results"], at);
  if (row.schema !== "forms.form") throw new Error(`${at}.schema: invalid Forms marker`);
  if (typeof row.id !== "string" || typeof row.version !== "string" || (row.title != null && typeof row.title !== "string")) throw new Error(`${at}: invalid Forms metadata`);
  return { schema: row.schema, id: row.id, version: row.version, ...(row.title == null ? {} : { title: row.title as string }), structure: parseSemioChild(row.structure, "value", `${at}.structure`), results: parseSemioChild(row.results, "table", `${at}.results`) };
}
