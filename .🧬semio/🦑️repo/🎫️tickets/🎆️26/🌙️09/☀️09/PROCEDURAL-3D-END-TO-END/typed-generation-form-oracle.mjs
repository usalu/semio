import Ajv from "ajv";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📝️forms/";
const read = (file) => JSON.parse(readFileSync(base + file, "utf8"));
const fixture = read("🧪️tests/🧫️fixtures/🔣️.json");
const schema = read("🧬️schema/🔣️.json");
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const validate = ajv.getSchema(schema.$id);
assert(validate(fixture), ajv.errorsText(validate.errors));
const validateAnswer = ajv.getSchema(schema.$id + "#/$defs/VariableAnswer");
const blocks = new Map(fixture.playbook.steps.flatMap((step) => step.blocks).map((block) => [block.id, block]));
const answers = Object.fromEntries(Array.from(blocks, ([id, block]) => [id, block.fields?.map((field) => field.value) ?? block.default ?? null]));
for (const { questionId, fieldIndex, value, accepted } of fixture.answerEdits) {
  const block = blocks.get(questionId);
  const current = answers[questionId];
  const next = fieldIndex === undefined ? value : Array.isArray(current) && fieldIndex < current.length ? current.with(fieldIndex, value) : null;
  const valid = Boolean(block && validateAnswer({ schema: block.schema, value: next }));
  assert.equal(valid, accepted, questionId);
  if (valid) answers[questionId] = next;
}
assert.deepEqual(answers, fixture.expectedAnswers);
const seeds = Object.fromEntries(fixture.widgets.filter((widget) => blocks.get(widget.id)?.schema).map(({ id, name, schema }) => {
  const value = answers[id];
  const fields = Array.isArray(value) ? Object.fromEntries(["x", "y", "z"].map((axis, index) => [axis, value[index]])) : { value };
  return [id, { port: name, schema, fields }];
}));
assert.deepEqual(seeds, fixture.expectedSeeds);
