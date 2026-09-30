/** 🧳️ AJV validates the actual BREP guest retirement vector and its geometry request. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

export function brepExtensionRetirementOracle(directory: string): number {
  const fixtures = resolve(directory, "../../🧫️fixtures/🚪️retirement");
  const fixture = JSON.parse(readFileSync(resolve(fixtures, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(fixtures, "📐️schema.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const input = JSON.parse(fixture.evaluate.inputJson);
  for (const axis of ["width", "depth", "height"]) assert.deepEqual(input[axis], { $schema: "number", value: 1 });
  assert.equal(validate({ ...fixture, tessellate: { ...fixture.tessellate, budget: 0 } }), false);
  assert.equal(validate({ ...fixture, grant: [0,65536] }), false);
  return 6;
}
