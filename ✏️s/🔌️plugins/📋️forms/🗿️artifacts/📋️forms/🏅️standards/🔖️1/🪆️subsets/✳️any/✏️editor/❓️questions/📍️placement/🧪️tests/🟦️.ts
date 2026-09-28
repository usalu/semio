import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import vectors from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import definitionSchema from "../../../../🧬️schema/📝️definition/🔣️.json";
import { createQuestionEvent, questionInsertIndex } from "../🟦️.ts";
import drops from "../🧫️fixtures/🔣️drop.json";
import dropSchema from "../🧬️schema/🔣️drop.json";
import type { FormsDefinition } from "../../../../🧬️schema/📝️definition/🟦️.ts";

/** 📍️ Placement remains recoverable after the last page is removed and matches JSON Patch. */
export function testFormsQuestionPlacement(): void {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(definitionSchema);
  const validate = ajv.compile(schema);
  for (const item of vectors.cases) {
    assert.equal(validate(item.input), true, JSON.stringify(validate.errors));
    const input = structuredClone(item.input);
    if ("error" in item) assert.throws(() => createQuestionEvent(input.definition, input.question, input.stepId, input.newStepId), { message: item.error });
    else {
      const event = createQuestionEvent(input.definition, input.question, input.stepId, input.newStepId);
      assert.deepEqual(event, item.event, item.name);
      const actual: FormsDefinition = structuredClone(input.definition);
      if (event.mutation === "createStep") actual.steps.push(event.step);
      else actual.steps.find(step => step.id === event.step_id)!.blocks.push(event.block);
      assert.deepEqual(actual, applyPatch(structuredClone(input.definition), item.patch as Operation[]).newDocument, item.name);
    }
    assert.deepEqual(input, item.input);
  }
  const validateDrop = ajv.compile(dropSchema);
  for (const item of drops.cases) {
    assert.equal(validateDrop(item.input), true, JSON.stringify(validateDrop.errors));
    const { stepId, targetId, position, movingId } = item.input;
    if ("error" in item) assert.throws(() => questionInsertIndex(drops.definition, stepId, targetId, position, movingId), { message: item.error });
    else {
      const index = questionInsertIndex(drops.definition, stepId, targetId, position, movingId);
      assert.equal(index, item.index, item.name);
      const before = drops.definition.steps.find(step => step.id === stepId)!.blocks.map(question => question.id);
      const actual = before.filter(id => id !== movingId);
      actual.splice(index, 0, movingId ?? "new");
      assert.deepEqual(actual, item.order, item.name);
      assert.deepEqual(actual, applyPatch([...before], item.patch as Operation[]).newDocument, item.name);
    }
  }
  console.log("[DEBUG] Forms question creation and drag ordering matched shared events and JSON Patch");
}
