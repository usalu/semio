/** 🧪️ Shared answer contracts checked independently with Ajv and ajv-formats. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import addFormats from "ajv-formats";
import fixture from "../🧫️fixtures/🔣️answers.json";
import { answerError } from "../🟦️.ts";
import type { FormQuestion, DslValue } from "../../🧬️mutations/🟦️.ts";
export function testFormsValidation(): void {
  const ajv = new Ajv({ multipleOfPrecision: 10 });
  addFormats(ajv);
  for (const test of fixture.cases) {
    const actual = answerError(test.question as FormQuestion, test.value as DslValue);
    assert.equal(actual, test.error, test.name);
    assert.equal(actual === null, ajv.compile(test.oracle)(test.value), test.name);
  }
  console.log(`[DEBUG] Forms answer validation matched ${fixture.cases.length} shared vectors and Ajv formats`);
}
