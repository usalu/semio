import assert from "node:assert/strict";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json";
import fixtures from "../../🧫️fixtures/🔣️.json";
import { meshSelectionInputs } from "../../🟦️.ts";

/** ⚖️ The production input resolver and independent Ajv oracle answer shared cases. */
export function generation3dMeshSelectionSelfTests(): number {
  const validate = new Ajv({ strict: false }).compile(schema);
  let checks = 0;
  for (const row of fixtures.valid) {
    assert.equal(validate(row.payload), true, row.name);
    assert.deepEqual(meshSelectionInputs(row.ids, row.payload), row.inputs, row.name);
    assert.throws(() => meshSelectionInputs(row.ids.map(id => id.replace("#0.", "#1.")), row.payload), row.name);
    checks += 3;
  }
  for (const row of fixtures.invalid) {
    assert.equal(validate(row.payload), false, row.name);
    assert.throws(() => meshSelectionInputs(fixtures.valid[0]!.ids, row.payload), row.name);
    checks += 2;
  }
  for (const value of [Infinity, NaN]) {
    assert.throws(() => meshSelectionInputs(fixtures.valid[0]!.ids, { ...fixtures.valid[0]!.payload, width: value }));
    checks++;
  }
for (const row of fixtures.analytic) {
  assert.equal(validate(row.payload), true, row.name);
  assert.deepEqual(meshSelectionInputs(row.ids, row.payload), row.inputs, row.name);
  assert.throws(() => meshSelectionInputs(row.ids.map(id => id.split("~")[0]!), row.payload), row.name);
  assert.throws(() => meshSelectionInputs(row.ids, { ...row.payload, amount: 0, radius: 0 }), row.name);
}
console.log("[DEBUG] Selected BRep absolute inputs exactLabels=uint64 operations=filletEdges,chamferEdges,shell");
  return checks + fixtures.analytic.length * 4;
}
