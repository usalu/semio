/** 🎛️ `change-block-field` payload — mirrors Rust `ChangeBlockField` + `BlockField` (`../🦀️.rs`): the absolute set of ONE
 * field of one question, on the wire `{mutation: "changeBlockField", blockId, field, value}` (`value: null` clears an
 * optional field). Schema: `../🧬️schema/🔣️.json`; design §17.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING. */
import type { DslValue, FormExpr, FormQuestion, FormQuestionOption, FormVectorField } from "../../🟦️.ts";

export const BLOCK_TEXT_FIELDS = ["description", "placeholder", "text", "unit", "schema", "src", "accept", "fixtureSlug"] as const;
export const BLOCK_NUMBER_FIELDS = ["min", "max", "step"] as const;
export const BLOCK_FIELDS = ["label", ...BLOCK_TEXT_FIELDS, "required", ...BLOCK_NUMBER_FIELDS, "default", "params", "condition", "options", "fields"] as const;

export type BlockFieldName = (typeof BLOCK_FIELDS)[number];
export type BlockField =
  | { field: "label"; value: string }
  | { field: (typeof BLOCK_TEXT_FIELDS)[number]; value: string | null }
  | { field: "required"; value: boolean | null }
  | { field: (typeof BLOCK_NUMBER_FIELDS)[number]; value: number | null }
  | { field: "default"; value: DslValue }
  | { field: "params"; value: { [key: string]: DslValue } | null }
  | { field: "condition"; value: FormExpr | null }
  | { field: "options"; value: FormQuestionOption[] | null }
  | { field: "fields"; value: FormVectorField[] | null };
export type ChangeBlockField = { blockId: string } & BlockField;

/** 🚨️ A refusal the leaf raises against a question: Error `target-missing`, Fatal `invariant` / `duplicate-id`, or the
 * Warning `no-op` of a value the question already holds. */
export interface BlockFieldDiagnostic {
  readonly code: "mutation.target-missing" | "mutation.no-op" | "mutation.invariant" | "mutation.duplicate-id";
  readonly path?: string[];
}

const isObject = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
const fail = (at: string, message: string): never => {
  throw new TypeError(`${at}: ${message}`);
};

/** 🧮️ Reads one condition expression exactly as `📝️definition/🔣️.json#/$defs/Expression` admits it. */
export function parseFormExpr(value: unknown, at = "$"): FormExpr {
  if (!isObject(value)) return fail(at, "an expression is an object");
  const keys = Object.keys(value).sort().join(",");
  const exact = (expected: string) => (keys === expected ? undefined : fail(at, `a ${String(value.kind)} expression carries exactly ${expected}`));
  switch (value.kind) {
    case "const":
      exact("kind,value");
      return { kind: "const", value: value.value as DslValue };
    case "var":
      exact("kind,name");
      return typeof value.name === "string" ? { kind: "var", name: value.name } : fail(`${at}.name`, "is a string");
    case "eq":
      exact("kind,left,right");
      return { kind: "eq", left: parseFormExpr(value.left, `${at}.left`), right: parseFormExpr(value.right, `${at}.right`) };
    case "truthy":
      exact("expr,kind");
      return { kind: "truthy", expr: parseFormExpr(value.expr, `${at}.expr`) };
    case "and":
    case "or": {
      exact("items,kind");
      const items = Array.isArray(value.items) ? value.items : fail(`${at}.items`, "is an array");
      return { kind: value.kind, items: items.map((item, index) => parseFormExpr(item, `${at}.items[${index}]`)) };
    }
    default:
      return fail(`${at}.kind`, "is const, var, eq, truthy, and or or");
  }
}

/** 📥️ Reads a `change-block-field` wire payload exactly as the leaf schema admits it. */
export function parseChangeBlockField(value: unknown, at = "$"): ChangeBlockField {
  if (!isObject(value)) return fail(at, "a change-block-field is an object");
  const { mutation, blockId, field, value: fieldValue, ...rest } = value;
  if (Object.keys(rest).length > 0) fail(at, `unknown member ${Object.keys(rest)[0]}`);
  if (mutation !== "changeBlockField") fail(`${at}.mutation`, "is changeBlockField");
  if (typeof blockId !== "string" || blockId === "") return fail(`${at}.blockId`, "is a non-empty question id");
  if (!("value" in value)) fail(`${at}.value`, "is required");
  const where = `${at}.value`;
  const nullable = <T>(read: (item: unknown) => T) => (fieldValue === null ? null : read(fieldValue));
  const text = (item: unknown): string => (typeof item === "string" ? item : fail(where, "is a string"));
  const number = (item: unknown): number => (typeof item === "number" && Number.isFinite(item) ? item : fail(where, "is a finite number"));
  const list = <T>(item: unknown, read: (entry: Record<string, unknown>, index: number) => T): T[] => (Array.isArray(item) ? item.map((entry, index) => (isObject(entry) ? read(entry, index) : fail(`${where}[${index}]`, "is an object"))) : fail(where, "is an array"));
  switch (field) {
    case "label":
      return { blockId, field, value: text(fieldValue) };
    case "description":
    case "placeholder":
    case "text":
    case "unit":
    case "schema":
    case "src":
    case "accept":
    case "fixtureSlug":
      return { blockId, field, value: nullable(text) };
    case "required":
      return { blockId, field, value: nullable((item) => (typeof item === "boolean" ? item : fail(where, "is a boolean"))) };
    case "min":
    case "max":
      return { blockId, field, value: nullable(number) };
    case "step":
      return { blockId, field, value: nullable((item) => (number(item) > 0 ? (item as number) : fail(where, "is greater than zero"))) };
    case "default":
      return { blockId, field, value: fieldValue as DslValue };
    case "params":
      return { blockId, field, value: nullable((item) => (isObject(item) ? (item as { [key: string]: DslValue }) : fail(where, "is an object"))) };
    case "condition":
      return { blockId, field, value: nullable((item) => parseFormExpr(item, where)) };
    case "options":
      return { blockId, field, value: nullable((item) => list(item, (entry, index) => (typeof entry.value === "string" && entry.value !== "" && typeof entry.label === "string" && Object.keys(entry).length === 2 ? { value: entry.value, label: entry.label } : fail(`${where}[${index}]`, "is {value, label}")))) };
    case "fields":
      return {
        blockId,
        field,
        value: nullable((item) =>
          list(item, (entry, index) => {
            const { key, label, value: component, ...extra } = entry;
            if (typeof key !== "string" || key === "" || Object.keys(extra).length > 0 || (label !== undefined && typeof label !== "string") || (component !== undefined && (typeof component !== "number" || !Number.isFinite(component)))) return fail(`${where}[${index}]`, "is {key, label?, value?}");
            return { key, ...(label === undefined ? {} : { label: label as string }), ...(component === undefined ? {} : { value: component as number }) };
          }),
        ),
      };
    default:
      return fail(`${at}.field`, `is one of ${BLOCK_FIELDS.join(", ")}`);
  }
}

/** 🔎️ The same field as `question` holds it now — what an undo restores (`null` for an absent optional field). */
export function readBlockField(question: FormQuestion, field: BlockFieldName): BlockField {
  const held = (question as unknown as Record<string, unknown>)[field];
  return { field, value: (held === undefined ? (field === "label" ? "" : null) : held) as never } as BlockField;
}

/** ✏️ `question` with `change`'s field set; `null` removes an optional field like the Rust projection skips `None`. */
export function applyBlockField(question: FormQuestion, change: BlockField): FormQuestion {
  const next = { ...question } as unknown as Record<string, unknown>;
  if (change.value === null) delete next[change.field];
  else next[change.field] = change.value;
  return next as unknown as FormQuestion;
}

const sameValue = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);

/** 🛡️ The Fatal refusal of `next` (the question with `change` set), mirroring the Rust diff's invariants. */
export function blockFieldRefusal(next: FormQuestion, change: BlockField): BlockFieldDiagnostic["code"] | undefined {
  const unique = (ids: string[]): BlockFieldDiagnostic["code"] | undefined => (ids.some((id) => id === "") ? "mutation.invariant" : new Set(ids).size !== ids.length ? "mutation.duplicate-id" : undefined);
  switch (change.field) {
    case "min":
    case "max":
      return next.min !== undefined && next.max !== undefined && next.min > next.max ? "mutation.invariant" : undefined;
    case "step":
      return change.value !== null && !(change.value > 0) ? "mutation.invariant" : undefined;
    case "default": {
      const value = change.value;
      if (value === null) return undefined;
      const fits = ["number", "slider"].includes(next.kind) ? typeof value === "number" && Number.isFinite(value) : next.kind === "boolean" ? typeof value === "boolean" : ["text", "longText", "date", "color", "single"].includes(next.kind) ? typeof value === "string" : next.kind === "multi" ? Array.isArray(value) && value.every((item) => typeof item === "string") : true;
      return fits ? undefined : "mutation.invariant";
    }
    case "options":
      return change.value === null ? undefined : unique(change.value.map((option) => option.value));
    case "fields":
      return change.value === null ? undefined : unique(change.value.map((entry) => entry.key));
    default:
      return undefined;
  }
}

/** 🚦️ What the leaf answers against `questions` (every question of every step): its diagnostic, or `undefined` when it
 * applies — the TS twin of the Rust diff's outcome classes. */
export function diagnoseChangeBlockField(questions: readonly FormQuestion[], change: ChangeBlockField): BlockFieldDiagnostic | undefined {
  const question = questions.find((candidate) => candidate.id === change.blockId);
  if (question === undefined) return { code: "mutation.target-missing", path: [change.blockId] };
  const { blockId: _, ...field } = change;
  const next = applyBlockField(question, field as BlockField);
  if (sameValue(next, question)) return { code: "mutation.no-op" };
  const refused = blockFieldRefusal(next, field as BlockField);
  return refused === undefined ? undefined : { code: refused, path: [change.blockId] };
}
