import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import type { DslValue } from "../🧬️mutations/🟦️.ts";
import type { FormExpr } from "../🧬️mutations/🟦️.ts";
import type { FormsDefinition } from "../📝️definition/🟦️.ts";
import { answerError, type FormsAnswerError } from "../✅️validation/🟦️.ts";
import{parseFormsValue,formsValueEqual}from"../🌱️value/🟦️.ts";

/** 📨️ Immutable answers preserve their original field presentation. */
export interface FormsAnswer { questionId: string; label: string; kind: string; value: DslValue }
export interface FormsResponse { id: string; submittedAt: number; definitionVersion: string; answers: FormsAnswer[] }

function evaluate(expr: FormExpr, values: Record<string, DslValue>): DslValue {
 const pending:{node:FormExpr;close?:boolean}[]=[{node:expr}],active=new Set<FormExpr>(),results=new Map<FormExpr,DslValue>();
 while(pending.length){const{node,close}=pending.pop()!;if(!close){if(active.has(node))throw Error("cyclic condition");active.add(node);pending.push({node,close:true});switch(node.kind){case"eq":pending.push({node:node.right},{node:node.left});break;case"truthy":pending.push({node:node.expr});break;case"and":case"or":for(let i=node.items.length-1;i>=0;i--)pending.push({node:node.items[i]!});break;}continue;}active.delete(node);
  switch(node.kind){case"const":results.set(node,node.value);break;case"var":results.set(node,values[node.name]??{kind:"null"});break;case"eq":results.set(node,{kind:"boolean",value:formsValueEqual(results.get(node.left)!,results.get(node.right)!)});break;case"truthy":results.set(node,{kind:"boolean",value:truth(results.get(node.expr)!)});break;case"and":results.set(node,{kind:"boolean",value:node.items.every(item=>truth(results.get(item)!))});break;case"or":results.set(node,{kind:"boolean",value:node.items.some(item=>truth(results.get(item)!))});break;}
 }return results.get(expr)!;
}
const truth=(value:DslValue)=>value.kind==="boolean"&&value.value;

function equal(left:FormsResponse,right:FormsResponse):boolean{return left.id===right.id&&left.submittedAt===right.submittedAt&&left.definitionVersion===right.definitionVersion&&left.answers.length===right.answers.length&&left.answers.every((a,i)=>{const b=right.answers[i]!;return a.questionId===b.questionId&&a.label===b.label&&a.kind===b.kind&&formsValueEqual(a.value,b.value);});}

/** ✅️ Captures visible fields only after every step passes the same answer contract. */
export function prepareResponse(definition: FormsDefinition, values: Record<string, DslValue>, metadata: Omit<FormsResponse, "answers">): { response: FormsResponse | null; errors: FormsAnswerError[] } {
  const questions = definition.steps.flatMap(step => step.blocks).filter(question => !question.condition || truth(evaluate(question.condition, values)));
  const errors = questions.flatMap(question => {
    const code = answerError(question, values[question.id] ?? {kind:"null"});
    return code ? [{ questionId: question.id, code }] : [];
  });
  if (errors.length) return { response: null, errors };
  const answers = questions.filter(question => question.kind !== "note" && question.kind !== "image").map(question => ({ questionId: question.id, label: question.label, kind: question.kind, value: parseFormsValue(values[question.id] ?? {kind:"null"}) }));
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
    seen.add(answer.questionId); return{questionId:answer.questionId,label:answer.label,kind:answer.kind,value:parseFormsValue(answer.value)};
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
    const next = before.map(parseFormsResponse);
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
  return index < 0 ? [] : [{ mutation: "commitResponse", response: parseFormsResponse(before[index]), index }];
}
