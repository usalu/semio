import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export function testFem3dNumericalCloseOwners(): void {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  for (const row of corpus.cases) {
    assert.deepEqual(applyPatch(row.before, row.patch as Operation[], true, false).newDocument, row.expected);
    assert.equal(row.expected.complete, false);
    assert.equal(row.expected.lane, row.lane + 1);
  }
  console.log("[DEBUG] Four absent numerical child lanes advance while retaining later close owners");
}

