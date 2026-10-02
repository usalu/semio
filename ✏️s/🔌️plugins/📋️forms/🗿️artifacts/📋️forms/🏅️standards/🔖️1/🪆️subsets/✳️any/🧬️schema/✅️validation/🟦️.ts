/** ✅️ Answer validation shared by preview, navigation and submission. */
import type { DslValue, FormQuestion, FormStep } from "../🧬️mutations/🟦️.ts";
import{formsValueNumber}from"../🌱️value/🟦️.ts";
export type AnswerErrorCode = "required" | "type" | "range" | "step" | "option" | "date" | "color" | "vector";
export interface FormsAnswerError { questionId: string; code: AnswerErrorCode }
const empty = (value: DslValue) => value.kind==="null"||value.kind==="text"&&value.value.trim()===""||value.kind==="array"&&value.items.length===0||value.kind==="object"&&value.members.length===0;
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
    case "text": case "longText": case "file": return value.kind==="text" ? null : "type";
    case "boolean": return value.kind==="boolean" ? null : "type";
    case "number": case "slider": {
      const number=formsValueNumber(value);if(number===undefined||!Number.isFinite(number))return"type";
      if (question.min !== undefined && number < question.min || question.max !== undefined && number > question.max) return "range";
      if (question.step !== undefined) {
        const ticks = (number - (question.min ?? 0)) / question.step;
        if (!Number.isFinite(ticks) || Math.abs(ticks - Math.round(ticks)) > 1e-9) return "step";
      }
      return null;
    }
    case "single": return value.kind!=="text" ? "type" : question.options?.some(option => option.value === value.value) ? null : "option";
    case "multi":{if(value.kind!=="array")return"type";const seen=new Set<string>();return value.items.some(item=>item.kind!=="text"||seen.has(item.value)||!question.options?.some(option=>option.value===item.value)||(seen.add(item.value),false))?"option":null;}
    case "date": return value.kind!=="text" ? "type" : validDate(value.value) ? null : "date";
    case "color": return value.kind!=="text" ? "type" : /^#[0-9a-fA-F]{6}$/.test(value.value) ? null : "color";
    case "vector": return value.kind!=="array" || value.items.length !== (question.fields?.length ?? 0) || value.items.some(item => {const number=formsValueNumber(item);return number===undefined||!Number.isFinite(number);}) ? "vector" : null;
    default: return value.kind==="object" ? null : "type";
  }
}
