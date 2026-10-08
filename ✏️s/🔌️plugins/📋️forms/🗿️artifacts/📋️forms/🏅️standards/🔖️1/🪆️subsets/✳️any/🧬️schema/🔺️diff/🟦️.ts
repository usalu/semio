/** 🔺️ Forms sparse durable delta: scalar slots, positional step and response row deltas (steps carry nested positional question deltas); the derived child handles are re-derived by the central applier, and omitted slots are untouched. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseFormsArtifact, type FormsArtifact } from "../🟦️.ts";
import { parseFormsDefinition } from "../📝️definition/🟦️.ts";
import { parseFormsResponse, type FormsResponse } from "../📨️response/🟦️.ts";
import type { FormQuestion, FormStep } from "../🧬️mutations/🟦️.ts";
import { applyBlockField, parseChangeBlockField, type BlockField } from "../🧬️mutations/🎛️change-block-field/🦠️mutation/🟦️.ts";

export interface FormsOptionalText { value: string | null }
export interface RowRemoval { id: string; index: number }
export interface RowInsertion<Row> { index: number; row: Row }
export interface RowRelocation { id: string; from: number; to: number }
export interface RowModification<Patch> { id: string; patch: Patch }
export interface RowDelta<Row, Patch> { removed: RowRemoval[]; inserted: RowInsertion<Row>[]; moved: RowRelocation[]; modified: RowModification<Patch>[] }
export interface FormsQuestionPatch { kind: string | null; changes: BlockField[] }
export type FormsQuestionsDelta = RowDelta<FormQuestion, FormsQuestionPatch>;
export interface FormsStepPatch { title: string | null; description: FormsOptionalText | null; blocks: FormsQuestionsDelta | null }
export type FormsStepsDelta = RowDelta<FormStep, FormsStepPatch>;
export type FormsResponsesDelta = Omit<RowDelta<FormsResponse, never>, "modified">;

export interface FormsDiff {
  /** @state artifact */ schema?: "forms.form";
  /** @state artifact */ id?: string;
  /** @state artifact */ version?: string;
  /** @state artifact */ title?: FormsOptionalText;
  /** @state artifact */ steps?: FormsStepsDelta;
  /** @state artifact */ responses?: FormsResponsesDelta;
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

const count = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > 4294967295) throw new Error(`${at}: value must be a uint32`);
  return value;
};

const positions = <Row>(fields: Record<string, unknown>, at: string, row: (value: unknown, at: string) => Row): Omit<RowDelta<Row, never>, "modified"> => ({
  removed: list(fields.removed, `${at}.removed`).map((item, position) => {
    const entry = record(item, `${at}.removed[${position}]`, ["id", "index"]);
    return { id: text(entry.id, `${at}.removed[${position}].id`), index: count(entry.index, `${at}.removed[${position}].index`) };
  }),
  inserted: list(fields.inserted, `${at}.inserted`).map((item, position) => {
    const entry = record(item, `${at}.inserted[${position}]`, ["index", "row"]);
    return { index: count(entry.index, `${at}.inserted[${position}].index`), row: row(entry.row, `${at}.inserted[${position}].row`) };
  }),
  moved: list(fields.moved, `${at}.moved`).map((item, position) => {
    const entry = record(item, `${at}.moved[${position}]`, ["id", "from", "to"]);
    return { id: text(entry.id, `${at}.moved[${position}].id`), from: count(entry.from, `${at}.moved[${position}].from`), to: count(entry.to, `${at}.moved[${position}].to`) };
  }),
});

const delta = <Row, Patch>(value: unknown, at: string, row: (value: unknown, at: string) => Row, patch: (value: unknown, at: string) => Patch): RowDelta<Row, Patch> => {
  const fields = record(value, at, ["removed", "inserted", "moved", "modified"]);
  return {
    ...positions(fields, at, row),
    modified: list(fields.modified, `${at}.modified`).map((item, position) => {
      const entry = record(item, `${at}.modified[${position}]`, ["id", "patch"]);
      return { id: text(entry.id, `${at}.modified[${position}].id`), patch: patch(entry.patch, `${at}.modified[${position}].patch`) };
    }),
  };
};

const optionalText = (value: unknown, at: string): FormsOptionalText => {
  const slot = record(value, at, ["value"]);
  return { value: slot.value == null ? null : text(slot.value, `${at}.value`) };
};

export function parseFormsQuestionPatch(value: unknown, at = "$"): FormsQuestionPatch {
  const row = record(value, at, ["kind", "changes"]);
  const changes = list(row.changes, `${at}.changes`).map((change, position) => {
    const { blockId: _blockId, ...field } = parseChangeBlockField({ mutation: "changeBlockField", blockId: "row", ...record(change, `${at}.changes[${position}]`, ["field", "value"]) }, `${at}.changes[${position}]`);
    return field as BlockField;
  });
  return { kind: row.kind == null ? null : text(row.kind, `${at}.kind`), changes };
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
  const row = record(value, at, ["title", "description", "blocks"]);
  return {
    title: row.title == null ? null : text(row.title, `${at}.title`),
    description: optional(row.description, (description) => optionalText(description, `${at}.description`)),
    blocks: optional(row.blocks, (blocks) => parseFormsQuestionsDelta(blocks, `${at}.blocks`)),
  };
}

export function parseFormsStepsDelta(value: unknown, at = "$"): FormsStepsDelta {
  return delta(value, at, stepRow, parseFormsStepPatch);
}

export function parseFormsResponsesDelta(value: unknown, at = "$"): FormsResponsesDelta {
  return positions(record(value, at, ["removed", "inserted", "moved"]), at, (response) => parseFormsResponse(response));
}

/** 🔎️ Keeps absent, set and clear operations distinct at the wire boundary. */
export function parseFormsDiff(value: unknown, at = "$"): FormsDiff {
  const row = parseSchemaRecord(value, ["schema", "id", "version", "title", "steps", "responses"], at);
  const diff: FormsDiff = {};
  if (Object.hasOwn(row, "schema")) {
    if (row.schema !== "forms.form") throw new Error(`${at}.schema: invalid Forms marker`);
    diff.schema = row.schema;
  }
  for (const field of ["id", "version"] as const) if (Object.hasOwn(row, field)) diff[field] = text(row[field], `${at}.${field}`);
  if (Object.hasOwn(row, "title")) diff.title = optionalText(row.title, `${at}.title`);
  if (Object.hasOwn(row, "steps")) diff.steps = parseFormsStepsDelta(row.steps, `${at}.steps`);
  if (Object.hasOwn(row, "responses")) diff.responses = parseFormsResponsesDelta(row.responses, `${at}.responses`);
  return diff;
}

/** 🧺️ Commits a positional row delta onto the base list: every removed or moved id is checked at its base index, inserted and moved rows take their after slots, unmoved survivors fill the rest in base order, then patches fold. */
function applyRows<Row extends { id: string }, Patch>(rows: readonly Row[], rowDelta: Omit<RowDelta<Row, Patch>, "modified"> & { modified?: RowModification<Patch>[] }, fold: (row: Row, patch: Patch) => Row): Row[] {
  const gone = new Set<number>();
  for (const { id, index } of [...rowDelta.removed, ...rowDelta.moved.map(({ id, from }) => ({ id, index: from }))]) {
    if (rows[index]?.id !== id) throw new Error(`row ${id} is not at base index ${index}`);
    if (gone.has(index)) throw new Error(`row ${id} is named twice`);
    gone.add(index);
  }
  const slots: (Row | undefined)[] = Array.from({ length: rows.length - rowDelta.removed.length + rowDelta.inserted.length }, () => undefined);
  const place = (index: number, row: Row) => {
    if (index >= slots.length) throw new Error(`after index ${index} lies past the end`);
    if (slots[index] !== undefined) throw new Error(`two rows take after index ${index}`);
    slots[index] = row;
  };
  for (const { index, row } of rowDelta.inserted) place(index, row);
  for (const { from, to } of rowDelta.moved) place(to, rows[from]!);
  const survivors = rows.filter((_, index) => !gone.has(index));
  let next: Row[] = slots.map((slot) => slot ?? survivors.shift()!);
  if (new Set(next.map((row) => row.id)).size !== next.length) throw new Error("two rows of the after list carry the same id");
  for (const { id, patch } of rowDelta.modified ?? []) {
    const target = next.find((row) => row.id === id);
    if (!target) throw new Error(`modified row ${id} does not exist`);
    next = next.map((row) => (row === target ? fold(row, patch) : row));
  }
  return next;
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
  if (responses !== undefined) next.responses = applyRows(base.responses, responses, (row: FormsResponse) => row);
  return parseFormsArtifact(next);
}
