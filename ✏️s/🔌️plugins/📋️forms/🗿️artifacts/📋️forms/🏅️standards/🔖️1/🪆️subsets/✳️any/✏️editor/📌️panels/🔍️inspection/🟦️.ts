import type { FormStep } from "../../../🧬️schema/🧬️mutations/🟦️.ts";
import type { FormsInspection } from "./🧬️schema/🟦️.ts";

/** 🎛️ Ordered property vocabulary for built-in and contributed questions. */
export function questionFields(kind: string): string[] {
  const fields = ["label", "kind", "description", "required"];
  switch (kind) {
    case "text": case "longText": fields.push("placeholder", "default"); break;
    case "number": case "slider": fields.push("default", "min", "max", "step", "unit"); break;
    case "boolean": case "date": case "color": fields.push("default"); break;
    case "single": case "multi": fields.push("default", "options"); break;
    case "vector": fields.push("schema", "step", "fields"); break;
    case "note": fields.push("text"); break;
    case "image": fields.push("src"); break;
    case "file": fields.push("accept"); break;
    default: fields.push("exampleId", "params"); break;
  }
  return [...fields, "condition"];
}

/** 🧭️ Resolves stale, transitive and multiple selections in document order. */
export function inspectionModel(steps: readonly FormStep[], selected: readonly string[]): FormsInspection {
  const ids = new Set(selected);
  const step = steps.find(item => ids.has(`step:${item.id}`));
  if (step) return { scope: "step", ids: [step.id], fields: ["title", "description"] };
  const questions = steps.flatMap(item => item.blocks).filter(item => ids.has(item.id));
  if (!questions.length) return { scope: "form", ids: [], fields: ["title"] };
  return {
    scope: "question",
    ids: questions.map(item => item.id),
    fields: questionFields(questions[0].kind).filter(field => questions.every(item => questionFields(item.kind).includes(field))),
  };
}
