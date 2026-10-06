import type { FormsAnswer, FormsResponse } from "../../../../../🧬️schema/📨️response/🟦️.ts";
import {formsValueJson} from "../../🔣️json/🟦️.ts";
export type FormsResponseExportFormat = "json" | "csv";
export const RESPONSE_COLUMNS = ["responseId", "submittedAt", "definitionVersion", "questionId", "label", "kind", "valueJson"] as const;

function responseRow(response: FormsResponse, answer: FormsAnswer): string[] {
  return [response.id, String(response.submittedAt), response.definitionVersion, answer.questionId, answer.label, answer.kind, formsValueJson(answer.value)];
}

function csvRecord(row: readonly string[]): string {
  return row.map(value => /[",\r\n]/.test(value) ? '"' + value.replaceAll('"', '""') + '"' : value).join(",") + "\r\n";
}

/** 📊️ A normalized answer grid preserves differing definitions between responses. */
export function responseRows(responses: readonly FormsResponse[]): string[][] {
  return [[...RESPONSE_COLUMNS], ...responses.flatMap(response => response.answers.map(answer => responseRow(response, answer)))];
}

/** 🧵️ Consumers may yield, report progress, or cancel between envelopes and answer records. */
export function* exportResponseChunks(responses: readonly FormsResponse[], format: FormsResponseExportFormat): Generator<string, void, unknown> {
  yield format === "csv" ? csvRecord(RESPONSE_COLUMNS) : "[";
  for (let index = 0; index < responses.length; index++) {
    const response = responses[index];
    if (format === "json") yield (index ? "," : "") + '{"id":' + JSON.stringify(response.id) + ',"submittedAt":' + response.submittedAt + ',"definitionVersion":' + JSON.stringify(response.definitionVersion) + ',"answers":[';
    for (let answer = 0; answer < response.answers.length; answer++) {
      const value=response.answers[answer]!;
      yield format === "csv" ? csvRecord(responseRow(response,value)) : (answer ? "," : "") + '{"questionId":'+JSON.stringify(value.questionId)+',"label":'+JSON.stringify(value.label)+',"kind":'+JSON.stringify(value.kind)+',"value":'+formsValueJson(value.value)+'}';
    }
    if (format === "json") yield "]}";
  }
  if (format === "json") yield "]";
}

/** 📤️ JSON retains whole submissions; RFC 4180 CSV carries typed values in valueJson. */
export function exportResponses(responses: readonly FormsResponse[], format: FormsResponseExportFormat): string {
  return [...exportResponseChunks(responses, format)].join("");
}
