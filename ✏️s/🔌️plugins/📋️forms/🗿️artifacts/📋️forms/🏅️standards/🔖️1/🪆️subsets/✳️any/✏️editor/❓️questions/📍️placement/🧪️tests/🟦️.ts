import {parseFormsJsonDefinition,parseFormsJsonQuestion,formsDefinitionJson,formsQuestionJson} from "../../../../🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
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
    const input={...item.input,definition:parseFormsJsonDefinition(item.input.definition),question:parseFormsJsonQuestion(item.input.question)};
    if ("error" in item) assert.throws(() => createQuestionEvent(input.definition, input.question, input.stepId, input.newStepId), { message: item.error });
    else {
      const event = createQuestionEvent(input.definition, input.question, input.stepId, input.newStepId);
      assert.deepEqual(event.mutation==="createStep"?{...event,step:(formsDefinitionJson({steps:[event.step]})as {steps:unknown[]}).steps[0]}:{...event,block:formsQuestionJson(event.block)}, item.event, item.name);
      const actual: FormsDefinition = parseFormsJsonDefinition(item.input.definition);
      if (event.mutation === "createStep") actual.steps.push(event.step);
      else actual.steps.find(step => step.id === event.stepId)!.blocks.push(event.block);
      assert.deepEqual(formsDefinitionJson(actual), applyPatch(structuredClone(item.input.definition), item.patch as Operation[]).newDocument, item.name);
    }
    assert.deepEqual({...input,definition:formsDefinitionJson(input.definition),question:formsQuestionJson(input.question)}, item.input);
  }
  const validateDrop = ajv.compile(dropSchema);
  for (const item of drops.cases) {
    assert.equal(validateDrop(item.input), true, JSON.stringify(validateDrop.errors));
    const { stepId, targetId, position, movingId } = item.input;
    if ("error" in item) assert.throws(() => questionInsertIndex(parseFormsJsonDefinition(drops.definition), stepId, targetId, position, movingId), { message: item.error });
    else {
      const index = questionInsertIndex(parseFormsJsonDefinition(drops.definition), stepId, targetId, position, movingId);
      assert.equal(index, item.index, item.name);
      const before = drops.definition.steps.find(step => step.id === stepId)!.blocks.map(question => question.id);
      const actual = before.filter(id => id !== movingId);
      actual.splice(index, 0, movingId ?? "new");
      assert.deepEqual(actual, item.order, item.name);
      assert.deepEqual(actual, applyPatch([...before], item.patch as Operation[]).newDocument, item.name);
    }
  }
}
