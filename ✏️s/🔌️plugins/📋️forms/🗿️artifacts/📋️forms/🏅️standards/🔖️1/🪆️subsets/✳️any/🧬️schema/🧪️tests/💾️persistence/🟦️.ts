import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/💾️persistence/🔣️.json";
import definition from "../../📝️definition/🔣️.json";
import response from "../../📨️response/🔣️.json";
import { parseFormsArtifact } from "../../🟦️.ts";
import blank from "../../🧫️fixtures/🌱️blank/🔣️.json";
import { blankFormsDefinition } from "../../📝️definition/🟦️.ts";

/** 💾️ Durable definition and answer rows survive a fresh JSON decode and schema oracle. */
export function testFormsPersistence(): void {
  const ajv = new Ajv({ strict: false });
  assert.equal(ajv.compile(definition)(fixture.definition), true);
  assert.equal(ajv.compile(definition)(blank), true);
  assert.deepEqual(blankFormsDefinition(), blank);
  for (const item of fixture.responses) assert.equal(ajv.compile(response)(item), true);
  const decoded = parseFormsArtifact(JSON.parse(JSON.stringify(fixture)));
  assert.deepEqual(decoded, fixture);
  assert.equal(decoded.definition.steps[0].blocks[0].required, true);
  assert.equal(decoded.responses[0].answers[0].value, "Ada");
  console.log("[DEBUG] Forms definition and responses survived fresh JSON decoding and Ajv");
}
