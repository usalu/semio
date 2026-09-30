import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import type { DslValue } from "../🧬️mutations/🟦️.ts";
import type { FormExpr } from "../🧬️mutations/🟦️.ts";
import type { FormsDefinition } from "../📝️definition/🟦️.ts";
import { answerError, type FormsAnswerError } from "../✅️validation/🟦️.ts";

/** 📨️ Immutable answers preserve their original field presentation. */
export interface FormsAnswer { questionId: string; label: string; kind: string; value: DslValue }
export interface FormsResponse { id: string; submittedAt: number; definitionVersion: string; answers: FormsAnswer[] }

function evaluate(expr: FormExpr, values: Record<string, DslValue>): DslValue {
  switch (expr.kind) {
    case "const": return expr.value;
    case "var": return values[expr.name] ?? null;
    case "eq": return equal(evaluate(expr.left, values), evaluate(expr.right, values));
    case "truthy": return evaluate(expr.expr, values) === true;
    case "and": return expr.items.every(item => evaluate(item, values) === true);
    case "or": return expr.items.some(item => evaluate(item, values) === true);
  }
}

function equal(left: unknown, right: unknown): boolean {
  if (left === right) return true;
  if (typeof left !== "object" || left === null || typeof right !== "object" || right === null || Array.isArray(left) !== Array.isArray(right)) return false;
  const a = left as Record<string, unknown>, b = right as Record<string, unknown>;
  return Object.keys(a).length === Object.keys(b).length && Object.keys(a).every(key => Object.hasOwn(b, key) && equal(a[key], b[key]));
}

/** ✅️ Captures visible fields only after every step passes the same answer contract. */
export function prepareResponse(definition: FormsDefinition, values: Record<string, DslValue>, metadata: Omit<FormsResponse, "answers">): { response: FormsResponse | null; errors: FormsAnswerError[] } {
  const questions = definition.steps.flatMap(step => step.blocks).filter(question => !question.condition || evaluate(question.condition, values) === true);
  const errors = questions.flatMap(question => {
    const code = answerError(question, values[question.id] ?? null);
    return code ? [{ questionId: question.id, code }] : [];
  });
  if (errors.length) return { response: null, errors };
  const answers = questions.filter(question => question.kind !== "note" && question.kind !== "image").map(question => ({ questionId: question.id, label: question.label, kind: question.kind, value: structuredClone(values[question.id] ?? null) }));
  return { response: { ...metadata, answers }, errors };
}

/** 🪪️ Decodes a submission without accepting ambiguous answer identities. */
export function parseFormsResponse(value: unknown): FormsResponse {
  const row = parseSchemaRecord(value, ["id", "submittedAt", "definitionVersion", "answers"]);
  if (typeof row.id !== "string" || !row.id || typeof row.submittedAt !== "number" || !Number.isSafeInteger(row.submittedAt) || row.submittedAt < 0 || typeof row.definitionVersion !== "string" || !row.definitionVersion || !Array.isArray(row.answers)) throw new Error("invalid response");
  const seen = new Set<string>();
  const answers = row.answers.map(value => {
    const answer = parseSchemaRecord(value, ["questionId", "label", "kind", "value"]);
    if (typeof answer.questionId !== "string" || !answer.questionId || seen.has(answer.questionId) || typeof answer.label !== "string" || typeof answer.kind !== "string" || !("value" in answer)) throw new Error("invalid response answer");
    seen.add(answer.questionId); return structuredClone(answer) as unknown as FormsAnswer;
  });
  return { id: row.id, submittedAt: row.submittedAt, definitionVersion: row.definitionVersion, answers };
}

export type FormsResponseEvent = { mutation: "commitResponse"; response: FormsResponse; index?: number | null } | { mutation: "discardResponse"; id: string };

/** 📨️ Applies semantic submission events with idempotent retries and explicit retraction. */
export function applyResponseEvent(before: FormsResponse[], event: FormsResponseEvent): FormsResponse[] {
  if (event.mutation === "commitResponse") {
    const response = parseFormsResponse(event.response);
    const existing = before.find(item => item.id === response.id);
    if (existing) {
      if (equal(existing, response)) return before;
      throw new Error("mutation.duplicate-id");
    }
    const next = structuredClone(before);
    next.splice(Math.min(Math.max(0, event.index ?? next.length), next.length), 0, response);
    return next;
  }
  if (!before.some(item => item.id === event.id)) throw new Error("mutation.target-missing");
  return before.filter(item => item.id !== event.id);
}

/** ↩️ Returns the exact event inverse, retaining original submission order. */
export function inverseResponseEvent(before: FormsResponse[], event: FormsResponseEvent): FormsResponseEvent[] {
  if (event.mutation === "commitResponse") return before.some(item => item.id === event.response.id) ? [] : [{ mutation: "discardResponse", id: event.response.id }];
  const index = before.findIndex(item => item.id === event.id);
  return index < 0 ? [] : [{ mutation: "commitResponse", response: structuredClone(before[index]), index }];
}
