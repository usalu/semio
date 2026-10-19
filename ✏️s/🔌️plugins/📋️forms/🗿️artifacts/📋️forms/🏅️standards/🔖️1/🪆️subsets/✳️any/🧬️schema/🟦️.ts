/** 🧬️ Forms durable document with exact owned-child coordinates. */
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import { requireArtifactDialect, parseArtifactChild, type ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";

import { parseFormsDefinition, type FormsDefinition } from "./📝️definition/🟦️.ts";
import { parseFormsResponse, type FormsResponse } from "./📨️response/🟦️.ts";

/** 🪆️ Admits a child by its declared Semio subset, independent of the plugin that owns that dialect. */
function parseFormsChild(value: unknown, subset: "value" | "table", at: string): ArtifactChild {
  const child = parseArtifactChild(value);
  requireArtifactDialect(child.target.dialect, { artifactKind: "s.stdio.semio", standard: "v1", subset }, at);
  return child;
}

export interface FormsArtifact {
  /** 📐️ @state artifact */ schema: "forms.form";
  /** 🔑️ @state artifact */ id: string;
  /** 🔖️ @state artifact */ version: string;
  /** 📛️ @state artifact */ title?: string;
  /** 📜️ @state artifact */ definition: FormsDefinition;
  /** 📬️ @state artifact */ responses: FormsResponse[];
  /** 🏗️ @state artifact @child kind=s.stdio.semio */ structure: ArtifactChild;
  /** 📥️ @state artifact @child kind=s.stdio.semio */ results: ArtifactChild;
}

/** 🪪️ Validates the document marker, declared fields and each exact child dialect. */
export function parseFormsArtifact(value: unknown, at = "$"): FormsArtifact {
  const row = parseSchemaRecord(value, ["schema", "id", "version", "title", "definition", "responses", "structure", "results"], at);
  if (row.schema !== "forms.form") throw new Error(`${at}.schema: invalid Forms marker`);
  if (typeof row.id !== "string" || typeof row.version !== "string" || (row.title != null && typeof row.title !== "string")) throw new Error(`${at}: invalid Forms metadata`);
  if (!Array.isArray(row.responses)) throw new Error("invalid form responses");
  const responses = row.responses.map(parseFormsResponse);
  if (new Set(responses.map(response => response.id)).size !== responses.length) throw new Error("duplicate response id");
  return { definition: parseFormsDefinition(row.definition), responses, schema: row.schema, id: row.id, version: row.version, ...(row.title == null ? {} : { title: row.title as string }), structure: parseFormsChild(row.structure, "value", `${at}.structure`), results: parseFormsChild(row.results, "table", `${at}.results`) };
}
