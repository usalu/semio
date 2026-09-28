import type { DslValue, FormExpr, FormQuestion } from "../../🧬️schema/🧬️mutations/🟦️.ts";

import { parseCondition } from "../../🧬️schema/📝️definition/🟦️.ts";

/** ✏️ Returns a validated field edit and leaves the source question untouched. */
export function patchQuestion(question: FormQuestion, field: string, value: DslValue): FormQuestion {
  let next = structuredClone(question);
  const optionalText = ["description", "placeholder", "text", "unit", "schema", "src", "accept", "fixtureSlug"] as const;
  if (field === "label") {
    if (typeof value !== "string") throw new Error("invalid-value");
    next.label = value;
  } else if (field === "kind") {
    if (typeof value !== "string" || !value.trim()) throw new Error("invalid-value");
    if (value !== question.kind) {
      next = defaultQuestionForKind(value, question.id);
      next.label = question.label;
      for (const key of ["description", "required", "condition"] as const) if (question[key] !== undefined) Object.assign(next, { [key]: structuredClone(question[key]) });
    }
  } else if ((optionalText as readonly string[]).includes(field)) {
    if (value !== null && typeof value !== "string") throw new Error("invalid-value");
    const key = field as typeof optionalText[number];
    if (value === null) delete next[key]; else next[key] = value as string;
  } else if (field === "required") {
    if (typeof value !== "boolean") throw new Error("invalid-value");
    next.required = value;
  } else if (field === "min" || field === "max" || field === "step") {
    if (value === null || value === "") delete next[field];
    else if (typeof value === "number" && Number.isFinite(value)) next[field] = value;
    else throw new Error("invalid-value");
  } else if (field === "default") {
    if (value === null) delete next.default;
    else {
      if (["number", "slider"].includes(next.kind) && (typeof value !== "number" || !Number.isFinite(value))) throw new Error("invalid-value");
      if (next.kind === "boolean" && typeof value !== "boolean") throw new Error("invalid-value");
      if (["text", "longText", "date", "color", "single"].includes(next.kind) && typeof value !== "string") throw new Error("invalid-value");
      if (next.kind === "multi" && (!Array.isArray(value) || value.some(item => typeof item !== "string"))) throw new Error("invalid-value");
      next.default = structuredClone(value);
    }
  } else if (field === "condition") {
    if (value === null) delete next.condition; else next.condition = parseCondition(value);
  } else if (field === "params") {
    if (value === null) delete next.params;
    else if (typeof value === "object" && !Array.isArray(value)) next.params = structuredClone(value);
    else throw new Error("invalid-value");
  } else throw new Error("unknown-field");
  if (next.min !== undefined && next.max !== undefined && next.min > next.max || next.step !== undefined && next.step <= 0) throw new Error("invalid-range");
  return next;
}

/** 🌱️ Initializes the built-in field contracts shared with the Rust authoring commands. */
export function defaultQuestionForKind(kind: string, id: string): FormQuestion {
  const question: FormQuestion = { id, kind, label: kind };
  switch (kind) {
    case "text": return { ...question, label: "Text", placeholder: "Enter text" };
    case "longText": return { ...question, label: "Long Text", placeholder: "Enter long text" };
    case "number": case "slider": return { ...question, label: kind === "number" ? "Number" : "Slider", default: kind === "number" ? 0 : 50, min: 0, max: 100, step: 1 };
    case "boolean": return { ...question, label: "Boolean", default: false };
    case "single": case "multi": return { ...question, label: kind === "single" ? "Single Select" : "Multi Select", ...(kind === "multi" ? { default: [] } : {}), options: [{ value: "a", label: "Option A" }, { value: "b", label: "Option B" }] };
    case "note": return { ...question, label: "Note", text: "Informational note" };
    case "date": return { ...question, label: "Date" };
    case "color": return { ...question, label: "Color", default: "#336699" };
    case "image": return { ...question, label: "Image" };
    case "file": return { ...question, label: "File" };
    case "vector": return { ...question, label: "Vector", schema: "vec3", step: 0.1, fields: ["x", "y", "z"].map(key => ({ key, label: key.toUpperCase(), value: 0 })) };
    default: return question;
  }
}

/** 🔘️ Edits choices and their selected defaults as one atomic document change. */
export function patchChoice(question: FormQuestion, edit: { option: string; field: string; value: DslValue }): FormQuestion {
  const next = structuredClone(question);
  const option = next.options?.find(option => option.value === edit.option);
  if (!option) return next;
  const rename = (replacement: string | undefined): void => {
    if (next.kind === "single" && next.default === edit.option) {
      if (replacement === undefined) delete next.default; else next.default = replacement;
    }
    if (next.kind === "multi" && Array.isArray(next.default)) next.default = next.default.flatMap(value => value === edit.option ? replacement === undefined ? [] : [replacement] : [value]);
  };
  switch (edit.field) {
    case "label": if (typeof edit.value !== "string") throw new Error("invalid-value"); option.label = edit.value; break;
    case "value":
      if (typeof edit.value !== "string" || !edit.value.trim()) throw new Error("invalid-value");
      if (next.options!.some(other => other !== option && other.value === edit.value)) throw new Error("duplicate-option");
      option.value = edit.value; rename(edit.value); break;
    case "remove": next.options = next.options!.filter(option => option.value !== edit.option); rename(undefined); break;
    case "default":
      if (next.kind !== "multi" || typeof edit.value !== "boolean") throw new Error("invalid-value");
      const selected = new Set(Array.isArray(next.default) ? next.default : []);
      if (edit.value) selected.add(edit.option); else selected.delete(edit.option);
      next.default = next.options!.filter(option => selected.has(option.value)).map(option => option.value); break;
    default: throw new Error("unknown-field");
  }
  return next;
}
