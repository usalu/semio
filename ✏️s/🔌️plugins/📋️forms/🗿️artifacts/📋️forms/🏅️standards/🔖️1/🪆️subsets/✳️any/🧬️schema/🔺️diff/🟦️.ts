/** 🔺️ Forms sparse durable delta; omitted slots are untouched. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseFormsArtifact, type FormsArtifact } from "../🟦️.ts";

import { parseFormsDefinition, type FormsDefinition } from "../📝️definition/🟦️.ts";
import { parseFormsResponse, type FormsResponse } from "../📨️response/🟦️.ts";

export interface FormsDiff {
  /** @state artifact */ schema?: "forms.form";
  /** @state artifact */ id?: string;
  /** @state artifact */ version?: string;
  /** @state artifact */ title?: string | null;
  /** @state artifact */ definition?: FormsDefinition;
  /** @state artifact */ responses?: FormsResponse[];
  /** @state artifact @child kind=s.stdio.semio */ structure?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ results?: ArtifactChild;
}

/** 🔎️ Keeps absent, set and clear operations distinct at the wire boundary. */
export function parseFormsDiff(value: unknown, at = "$"): FormsDiff {
  const row = parseSchemaRecord(value, ["schema", "id", "version", "title", "definition", "responses", "structure", "results"], at);
  const diff: FormsDiff = {};
  if (Object.hasOwn(row, "schema")) {
    if (row.schema !== "forms.form") throw new Error(`${at}.schema: invalid Forms marker`);
    diff.schema = row.schema;
  }
  for (const field of ["id", "version"] as const) if (Object.hasOwn(row, field)) {
    if (typeof row[field] !== "string") throw new Error(`${at}.${field}: expected a string`);
    diff[field] = row[field];
  }
  if (Object.hasOwn(row, "title")) {
    if (row.title !== null && typeof row.title !== "string") throw new Error(`${at}.title: expected a string or null`);
    diff.title = row.title;
  }
  if (Object.hasOwn(row, "definition")) diff.definition = parseFormsDefinition(row.definition);
  if (Object.hasOwn(row, "responses")) {
    if (!Array.isArray(row.responses)) throw new Error("invalid form responses");
    diff.responses = row.responses.map(parseFormsResponse);
    if (new Set(diff.responses.map(response => response.id)).size !== diff.responses.length) throw new Error("duplicate response id");
  }
  if (Object.hasOwn(row, "structure")) diff.structure = parseSemioChild(row.structure, "value", `${at}.structure`);
  if (Object.hasOwn(row, "results")) diff.results = parseSemioChild(row.results, "table", `${at}.results`);
  return diff;
}

/** 🩹️ Applies only the declared document edits and preserves untouched child references. */
export function applyFormsDiff(base: FormsArtifact, value: unknown): FormsArtifact {
  parseFormsArtifact(base);
  const diff = parseFormsDiff(value);
  const next = { ...base, ...diff };
  if (next.title === null) delete next.title;
  return next as FormsArtifact;
}
