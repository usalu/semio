/** ✅️ Answer validation shared by preview, navigation and submission. */
import type { DslValue, FormQuestion, FormStep } from "../🧬️mutations/🟦️.ts";
export type AnswerErrorCode = "required" | "type" | "range" | "step" | "option" | "date" | "color" | "vector";
export interface FormsAnswerError { questionId: string; code: AnswerErrorCode }
const object = (value: DslValue): value is { [key: string]: DslValue } => typeof value === "object" && value !== null && !Array.isArray(value);
const empty = (value: DslValue) => value === null || typeof value === "string" && value.trim() === "" || Array.isArray(value) && value.length === 0 || object(value) && Object.keys(value).length === 0;
function validDate(value: string): boolean {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return false;
  const [year, month, day] = value.split("-").map(Number);
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  return month >= 1 && month <= 12 && day >= 1 && day <= [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month - 1];
}
export function answerError(question: FormQuestion, value: DslValue): AnswerErrorCode | null {
  if (question.kind === "note" || question.kind === "image") return null;
  if (empty(value)) return question.required ? "required" : null;
  switch (question.kind) {
    case "text": case "longText": case "file": return typeof value === "string" ? null : "type";
    case "boolean": return typeof value === "boolean" ? null : "type";
    case "number": case "slider": {
      if (typeof value !== "number" || !Number.isFinite(value)) return "type";
      if (question.min !== undefined && value < question.min || question.max !== undefined && value > question.max) return "range";
      if (question.step !== undefined) {
        const ticks = (value - (question.min ?? 0)) / question.step;
        if (!Number.isFinite(ticks) || Math.abs(ticks - Math.round(ticks)) > 1e-9) return "step";
      }
      return null;
    }
    case "single": return typeof value !== "string" ? "type" : question.options?.some(option => option.value === value) ? null : "option";
    case "multi": return !Array.isArray(value) ? "type" : new Set(value).size !== value.length || value.some(item => typeof item !== "string" || !question.options?.some(option => option.value === item)) ? "option" : null;
    case "date": return typeof value !== "string" ? "type" : validDate(value) ? null : "date";
    case "color": return typeof value !== "string" ? "type" : /^#[0-9a-fA-F]{6}$/.test(value) ? null : "color";
    case "vector": return !Array.isArray(value) || value.length !== (question.fields?.length ?? 0) || value.some(item => typeof item !== "number" || !Number.isFinite(item)) ? "vector" : null;
    default: return object(value) ? null : "type";
  }
}
