import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const require = createRequire(resolve("package.json"));
const Ajv = require("ajv");
const Decimal = require("decimal.js");
const root = "🧰️framework/🔨️modules/🧵️job/🏃️work/🎟️grant";
const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ allErrors: true, strict: true });
const validateGrant = ajv.compile(schema);
const validateDemand = ajv.compile(schema.$defs.demand);
const validateReceipt = ajv.compile(schema.$defs.receipt);
const { RetainedWorkBudget } = await import(pathToFileURL(resolve(root, "🟦️.ts")).href);
const axes = [["maximumCopyBytes", "copyBytes", "copiedBytes"], ["maximumCapacityBytes", "capacityBytes", "retainedCapacityBytes"], ["maximumReleaseBytes", "releaseBytes", "releasedBytes"], ["maximumDepth", "depth", null]];
const enter = (grant, demand) => new Decimal(grant.maximumItems).gt(0) && axes.every(([g, d]) => new Decimal(grant[g]).gte(demand[d]));
for (const row of fixture.cases) {
  assert.ok(validateGrant(row.grant), JSON.stringify(validateGrant.errors));
  assert.ok(validateDemand(row.demand), JSON.stringify(validateDemand.errors));
  const budget = new RetainedWorkBudget(row.grant);
  assert.equal(enter(row.grant, row.demand), row.accepted, row.name);
  assert.equal(budget.canEnter(row.demand), row.accepted, row.name);
  assert.deepEqual(budget.remaining(), row.grant);
}
const sequence = fixture.sequence;
assert.ok(validateReceipt(sequence.receipt), JSON.stringify(validateReceipt.errors));
const budget = new RetainedWorkBudget(sequence.grant);
let reference = structuredClone(sequence.grant);
for (const expected of sequence.accepted) {
  assert.equal(enter(reference, sequence.demand), expected);
  assert.equal(budget.canEnter(sequence.demand), expected);
  if (expected) {
    budget.charge(sequence.receipt);
    reference.maximumItems = new Decimal(reference.maximumItems).minus(sequence.receipt.copiedItems).toNumber();
    for (const [grant, , receipt] of axes) if (receipt) reference[grant] = new Decimal(reference[grant]).minus(sequence.receipt[receipt]).toNumber();
    assert.deepEqual(budget.remaining(), reference);
  }
}
const refused = budget.remaining();
assert.throws(() => budget.charge(sequence.receipt));
assert.deepEqual(budget.remaining(), refused);
for (const row of fixture.contexts) {
  const demand = { copyBytes: 17, capacityBytes: 17, releaseBytes: 0, depth: 1 };
  assert.equal(enter(row.grant, demand), row.accepted);
  assert.equal(new RetainedWorkBudget(row.grant).canEnter(demand), row.accepted);
}
console.log(JSON.stringify({ cases: fixture.cases.length, sequenceTurns: sequence.accepted.length, contexts: fixture.contexts.length, oracle: "decimal.js", typescript: true, rustExecution: false }));

