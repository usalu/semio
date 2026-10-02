import{parseFormsJsonResponse,formsResponseJson,parseFormsJsonDefinition,parseFormsJsonValue}from"../../../🌱️value/🔣️json/🟦️.ts";
import assert from "node:assert/strict";
import { applyPatch, compare } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️events.json";
import timestamps from "../../🧫️fixtures/🔣️timestamps.json";
import responseSchema from "../../🔣️.json";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { parseFormsResponse, applyResponseEvent, inverseResponseEvent, type FormsResponseEvent } from "../../🟦️.ts";

/** 📨️ Submission, retraction and undo share fixtures and independent JSON Patch outcomes. */
export function testFormsResponses(): void {
  for (const item of fixture.cases) {
    const raw=item.event;const event:FormsResponseEvent=raw.mutation==="commitResponse"?{...raw,response:parseFormsJsonResponse(raw.response)}as FormsResponseEvent:raw as FormsResponseEvent;const before=item.before.map(parseFormsJsonResponse);
    if ("error" in item) assert.throws(() => applyResponseEvent(before, event), { message: item.error }, item.name);
    else {
      const actual = applyResponseEvent(before, event);
      assert.deepEqual(actual.map(formsResponseJson), item.after, item.name);
      assert.deepEqual(actual.map(formsResponseJson), applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.name);
      assert.deepEqual(inverseResponseEvent(before, event).reduce(applyResponseEvent, actual).map(formsResponseJson), item.before, item.name);
    }
  }
  const validate = semioSchemaAjvV1().compile(responseSchema);
  for (const item of timestamps.cases) {
    const response = { id: "response-a", submittedAt: item.submittedAt, definitionVersion: "revision-a", answers: [] };
    assert.equal(validate(response), item.valid, item.name);
    if (item.valid) assert.deepEqual(parseFormsResponse(response), response, item.name);
    else assert.throws(() => parseFormsResponse(response), item.name);
  }
}

/** ✅️ Submission validates every step and snapshots only visible answer fields. */
export async function testFormsSubmission(): Promise<void> {
  const { prepareResponse } = await import("../../🟦️.ts");
  const { default: input } = await import("../../🧫️fixtures/🔣️submission.json");
  const { default: responseSchema } = await import("../../🔣️.json");
  const validate = semioSchemaAjvV1().compile(responseSchema);
  for (const test of input.cases) {
    const result = prepareResponse(parseFormsJsonDefinition(input.definition), Object.fromEntries(Object.entries(test.values).map(([key,value])=>[key,parseFormsJsonValue(value)])), input.metadata);
    assert.deepEqual({...result,response:result.response?formsResponseJson(result.response):null}, test.expected, test.name);
    if (result.response) assert.equal(validate(formsResponseJson(result.response)), true, test.name);
  }
}
