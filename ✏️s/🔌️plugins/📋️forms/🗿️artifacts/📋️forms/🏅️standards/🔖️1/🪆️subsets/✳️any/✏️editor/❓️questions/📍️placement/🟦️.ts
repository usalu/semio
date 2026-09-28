import { parseFormsDefinition, type FormsDefinition } from "../../../🧬️schema/📝️definition/🟦️.ts";
import type { FormQuestion, FormStep, FormsMutation } from "../../../🧬️schema/🧬️mutations/🟦️.ts";

/** 🎯️ Computes the insertion index in the destination after a same-page source is removed. */
export function questionInsertIndex(definition: FormsDefinition, stepId: string, targetId: string, position: string, movingId: string | null): number {
  if (!["before", "after", "inside"].includes(position)) throw new Error("invalid-position");
  const step = definition.steps.find(step => step.id === stepId);
  if (!step) throw new Error("missing-step");
  const source = step.blocks.findIndex(question => question.id === movingId);
  let index: number;
  if (targetId.startsWith("step:")) {
    if (targetId.slice(5) !== stepId) throw new Error("missing-target");
    index = position === "before" ? 0 : step.blocks.length;
  } else {
    const target = step.blocks.findIndex(question => question.id === targetId);
    if (target < 0) throw new Error("missing-target");
    if (movingId === targetId) return target;
    index = position === "before" ? target : position === "after" ? target + 1 : step.blocks.length;
  }
  return index - (source >= 0 && source < index ? 1 : 0);
}

/** 🌱️ Creates the first page when needed and preserves an explicit existing page target. */
export function createQuestionEvent(definition: FormsDefinition, question: FormQuestion, stepId: string | null, newStepId: string): Extract<FormsMutation, { mutation: "createStep" | "createBlock" }> {
  if (definition.steps.some(step => step.blocks.some(existing => existing.id === question.id))) throw new Error("duplicate-question");
  const candidate: FormStep = { id: newStepId, title: "Inputs", blocks: [structuredClone(question)] };
  try { parseFormsDefinition({ steps: [candidate] }); } catch { throw new Error("invalid-question"); }
  const target = stepId === null ? definition.steps[0] : definition.steps.find(step => step.id === stepId);
  if (stepId !== null && !target) throw new Error("missing-step");
  return target ? { mutation: "createBlock", step_id: target.id, block: structuredClone(question), index: null } : { mutation: "createStep", step: candidate, index: null };
}
