import assert from "node:assert/strict";
import { applyPatch, compare } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔣️patches.json";
import choices from "../../🧫️fixtures/🔣️choices.json";
import choiceSchema from "../../🧬️schema/🔣️choice-edit.json";
import Ajv from "ajv/dist/2020.js";
import { patchChoice, patchQuestion } from "../../🟦️.ts";
import type { FormQuestion, DslValue } from "../../../../🧬️schema/🧬️mutations/🟦️.ts";

/** 🧪️ Validated field edits match shared vectors and independent JSON Patch application. */
export function testFormsQuestionPatches(): void {
  for (const item of fixture.cases) {
    const before = structuredClone(item.before) as FormQuestion;
    if ("error" in item) {
      assert.throws(() => patchQuestion(before, item.field, item.value as DslValue), { message: item.error }, item.name);
    } else {
      const actual = patchQuestion(before, item.field, item.value as DslValue);
      assert.deepEqual(actual, item.after, item.name);
      assert.deepEqual(actual, applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.name);
    }
    assert.deepEqual(before, item.before, "patch leaves the input untouched");
  }
  const validate = new Ajv().compile(choiceSchema);
  for (const item of choices.cases) {
    const before = structuredClone(item.before) as FormQuestion;
    const edit = { option: item.option, field: item.field, value: item.value };
    assert.equal(validate(edit), true, JSON.stringify(validate.errors));
    if ("error" in item) assert.throws(() => patchChoice(before, edit), { message: item.error }, item.name);
    else {
      const actual = patchChoice(before, edit);
      assert.deepEqual(actual, item.after, item.name);
      assert.deepEqual(actual, applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.name);
    }
    assert.deepEqual(before, item.before, "choice edit leaves source untouched");
  }
  console.log("[DEBUG] Forms question edits matched shared vectors and the JSON Patch oracle");
}
