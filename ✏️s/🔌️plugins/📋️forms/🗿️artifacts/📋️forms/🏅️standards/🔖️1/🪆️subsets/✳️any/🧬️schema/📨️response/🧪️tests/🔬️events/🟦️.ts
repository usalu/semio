import assert from "node:assert/strict";
import { applyPatch, compare } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️events.json";
import timestamps from "../../🧫️fixtures/🔣️timestamps.json";
import responseSchema from "../../🔣️.json";
import Ajv from "ajv";
import { parseFormsResponse, applyResponseEvent, inverseResponseEvent, type FormsResponseEvent } from "../../🟦️.ts";

/** 📨️ Submission, retraction and undo share fixtures and independent JSON Patch outcomes. */
export function testFormsResponses(): void {
  for (const item of fixture.cases) {
    const event = item.event as FormsResponseEvent;
    if ("error" in item) assert.throws(() => applyResponseEvent(item.before, event), { message: item.error }, item.name);
    else {
      const actual = applyResponseEvent(item.before, event);
      assert.deepEqual(actual, item.after, item.name);
      assert.deepEqual(actual, applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.name);
      assert.deepEqual(inverseResponseEvent(item.before, event).reduce(applyResponseEvent, actual), item.before, item.name);
    }
  }
  const validate = new Ajv().compile(responseSchema);
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
  const { default: Ajv } = await import("ajv");
  const validate = new Ajv().compile(responseSchema);
  for (const test of input.cases) {
    const result = prepareResponse(input.definition as import("../../../📝️definition/🟦️.ts").FormsDefinition, test.values, input.metadata);
    assert.deepEqual(result, test.expected, test.name);
    if (result.response) assert.equal(validate(result.response), true, test.name);
  }
}
