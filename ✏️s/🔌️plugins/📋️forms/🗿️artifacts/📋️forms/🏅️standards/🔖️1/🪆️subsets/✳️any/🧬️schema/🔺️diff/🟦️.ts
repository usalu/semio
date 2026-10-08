/** 🔺️ Forms sparse durable delta: scalar slots, id-keyed step and response row deltas (steps carry nested id-keyed question deltas) and the owned-child handles; omitted slots are untouched. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseSemioChild, type ArtifactChild } from "../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🟦️.ts";
import { parseFormsArtifact, type FormsArtifact } from "../🟦️.ts";
import { parseFormsDefinition } from "../📝️definition/🟦️.ts";
import { parseFormsResponse, type FormsResponse } from "../📨️response/🟦️.ts";
import type { FormQuestion, FormStep } from "../🧬️mutations/🟦️.ts";
import { applyBlockField, parseChangeBlockField, type BlockField } from "../🧬️mutations/🎛️change-block-field/🦠️mutation/🟦️.ts";

export interface FormsOptionalText { value: string | null }
export interface RowDelta<Row, Patch> { added: Row[]; removed: string[]; patched: Patch[]; reordered: string[] | null }
export interface FormsQuestionPatch { id: string; kind: string | null; changes: BlockField[] }
export type FormsQuestionsDelta = RowDelta<FormQuestion, FormsQuestionPatch>;
export interface FormsStepPatch { id: string; title: string | null; description: FormsOptionalText | null; blocks: FormsQuestionsDelta | null }
export interface FormsResponsePatch { id: string }
export type FormsStepsDelta = RowDelta<FormStep, FormsStepPatch>;
export type FormsResponsesDelta = RowDelta<FormsResponse, FormsResponsePatch>;

export interface FormsDiff {
  /** @state artifact */ schema?: "forms.form";
  /** @state artifact */ id?: string;
  /** @state artifact */ version?: string;
  /** @state artifact */ title?: FormsOptionalText;
  /** @state artifact */ steps?: FormsStepsDelta;
  /** @state artifact */ responses?: FormsResponsesDelta;
  /** @state artifact @child kind=s.stdio.semio */ structure?: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ results?: ArtifactChild;
}

const record = (value: unknown, at: string, keys: readonly string[]): Record<string, unknown> => parseSchemaRecord(value, keys, at) as Record<string, unknown>;
const text = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new Error(`${at}: expected a string`);
  return value;
};
const list = (value: unknown, at: string): unknown[] => {
  if (!Array.isArray(value)) throw new Error(`${at}: expected an array`);
  return value;
};
const optional = <T>(value: unknown, parse: (value: unknown) => T): T | null => (value == null ? null : parse(value));

const delta = <Row, Patch>(value: unknown, at: string, row: (value: unknown, at: string) => Row, patch: (value: unknown, at: string) => Patch): RowDelta<Row, Patch> => {
  const fields = record(value, at, ["added", "removed", "patched", "reordered"]);
  return {
    added: list(fields.added, `${at}.added`).map((item, position) => row(item, `${at}.added[${position}]`)),
    removed: list(fields.removed, `${at}.removed`).map((id, position) => text(id, `${at}.removed[${position}]`)),
    patched: list(fields.patched, `${at}.patched`).map((item, position) => patch(item, `${at}.patched[${position}]`)),
    reordered: optional(fields.reordered, (order) => list(order, `${at}.reordered`).map((id, position) => text(id, `${at}.reordered[${position}]`))),
  };
};

const optionalText = (value: unknown, at: string): FormsOptionalText => {
  const slot = record(value, at, ["value"]);
  return { value: slot.value == null ? null : text(slot.value, `${at}.value`) };
};

export function parseFormsQuestionPatch(value: unknown, at = "$"): FormsQuestionPatch {
  const row = record(value, at, ["id", "kind", "changes"]);
  const id = text(row.id, `${at}.id`);
  const changes = list(row.changes, `${at}.changes`).map((change, position) => {
    const { blockId: _blockId, ...field } = parseChangeBlockField({ mutation: "changeBlockField", blockId: id, ...record(change, `${at}.changes[${position}]`, ["field", "value"]) }, `${at}.changes[${position}]`);
    return field as BlockField;
  });
  return { id, kind: row.kind == null ? null : text(row.kind, `${at}.kind`), changes };
}

const questionRow = (value: unknown, at: string): FormQuestion => parseFormsDefinition({ steps: [{ id: "row", title: at, blocks: [value] }] }).steps[0]!.blocks[0]!;
const stepRow = (value: unknown, at: string): FormStep => {
  const [step] = parseFormsDefinition({ steps: [value] }).steps;
  if (!step) throw new Error(`${at}: a step row is required`);
  return step;
};

export function parseFormsQuestionsDelta(value: unknown, at = "$"): FormsQuestionsDelta {
  return delta(value, at, questionRow, parseFormsQuestionPatch);
}

export function parseFormsStepPatch(value: unknown, at = "$"): FormsStepPatch {
  const row = record(value, at, ["id", "title", "description", "blocks"]);
  return {
    id: text(row.id, `${at}.id`),
    title: row.title == null ? null : text(row.title, `${at}.title`),
    description: optional(row.description, (description) => optionalText(description, `${at}.description`)),
    blocks: optional(row.blocks, (blocks) => parseFormsQuestionsDelta(blocks, `${at}.blocks`)),
  };
}

export function parseFormsStepsDelta(value: unknown, at = "$"): FormsStepsDelta {
  return delta(value, at, stepRow, parseFormsStepPatch);
}

export function parseFormsResponsesDelta(value: unknown, at = "$"): FormsResponsesDelta {
  return delta(value, at, (response) => parseFormsResponse(response), (patch, patchAt) => ({ id: text(record(patch, patchAt, ["id"]).id, `${patchAt}.id`) }));
}

/** 🔎️ Keeps absent, set and clear operations distinct at the wire boundary. */
export function parseFormsDiff(value: unknown, at = "$"): FormsDiff {
  const row = parseSchemaRecord(value, ["schema", "id", "version", "title", "steps", "responses", "structure", "results"], at);
  const diff: FormsDiff = {};
  if (Object.hasOwn(row, "schema")) {
    if (row.schema !== "forms.form") throw new Error(`${at}.schema: invalid Forms marker`);
    diff.schema = row.schema;
  }
  for (const field of ["id", "version"] as const) if (Object.hasOwn(row, field)) diff[field] = text(row[field], `${at}.${field}`);
  if (Object.hasOwn(row, "title")) diff.title = optionalText(row.title, `${at}.title`);
  if (Object.hasOwn(row, "steps")) diff.steps = parseFormsStepsDelta(row.steps, `${at}.steps`);
  if (Object.hasOwn(row, "responses")) diff.responses = parseFormsResponsesDelta(row.responses, `${at}.responses`);
  if (Object.hasOwn(row, "structure")) diff.structure = parseSemioChild(row.structure, "value", `${at}.structure`);
  if (Object.hasOwn(row, "results")) diff.results = parseSemioChild(row.results, "table", `${at}.results`);
  return diff;
}

/** 🧺️ Applies an id-keyed row delta: removed rows leave, added rows append, patched rows fold, then the optional complete order is imposed. */
function applyRows<Row extends { id: string }, Patch extends { id: string }>(rows: readonly Row[], rowDelta: RowDelta<Row, Patch>, fold: (row: Row, patch: Patch) => Row): Row[] {
  for (const id of rowDelta.removed) if (!rows.some((row) => row.id === id)) throw new Error(`removed row ${id} does not exist`);
  let next = rows.filter((row) => !rowDelta.removed.includes(row.id));
  for (const row of rowDelta.added) {
    if (next.some((existing) => existing.id === row.id)) throw new Error(`added row ${row.id} already exists`);
    next = [...next, row];
  }
  for (const patch of rowDelta.patched) {
    const target = next.find((row) => row.id === patch.id);
    if (!target) throw new Error(`patched row ${patch.id} does not exist`);
    next = next.map((row) => (row === target ? fold(row, patch) : row));
  }
  if (rowDelta.reordered === null) return next;
  const order = rowDelta.reordered;
  if (order.length !== next.length || new Set(order).size !== order.length || next.some((row) => !order.includes(row.id))) throw new Error("reorder must be a complete unique permutation");
  return order.map((id) => next.find((row) => row.id === id)!);
}

const foldQuestion = (question: FormQuestion, patch: FormsQuestionPatch): FormQuestion => patch.changes.reduce<FormQuestion>((current, change) => applyBlockField(current, change), patch.kind === null ? question : { ...question, kind: patch.kind });
const foldStep = (step: FormStep, patch: FormsStepPatch): FormStep => {
  const { description: _description, ...rest } = step;
  const description = patch.description === null ? step.description : (patch.description.value ?? undefined);
  return { ...rest, ...(description === undefined ? {} : { description }), title: patch.title ?? step.title, blocks: patch.blocks === null ? step.blocks : applyRows(step.blocks, patch.blocks, foldQuestion) };
};

/** 🩹️ Applies only the declared document edits and preserves untouched child references. */
export function applyFormsDiff(base: FormsArtifact, value: unknown): FormsArtifact {
  parseFormsArtifact(base);
  const diff = parseFormsDiff(value);
  const { steps, responses, title, ...fields } = diff;
  const next: FormsArtifact = { ...base, ...fields };
  if (title !== undefined) {
    if (title.value === null) delete next.title;
    else next.title = title.value;
  }
  if (steps !== undefined) next.definition = { ...base.definition, steps: applyRows(base.definition.steps, steps, foldStep) };
  if (responses !== undefined) next.responses = applyRows(base.responses, responses, (row) => row);
  return parseFormsArtifact(next);
}
