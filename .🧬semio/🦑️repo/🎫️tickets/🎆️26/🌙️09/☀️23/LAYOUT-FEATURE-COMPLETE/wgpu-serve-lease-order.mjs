import assert from "node:assert/strict";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import fixture from "./wgpu-serve-lease-order.json" with { type: "json" };

const order = join(process.cwd(), "🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts");
const { checkLeaseOrderV1, leaseOrderPolicyV1 } = await import(pathToFileURL(order).href);
const policy = leaseOrderPolicyV1({});
const results = {};
for (const scenario of fixture.sequences) {
  const held = new Map();
  const violations = [];
  for (const [index, step] of scenario.steps.entries()) {
    if (step.op === "release") {
      assert.ok(held.delete(step.resource));
      continue;
    }
    const request = { directory: fixture.directory, resource: step.resource, mode: step.mode, nonblocking: false };
    const result = checkLeaseOrderV1(policy, [...held.values()], request);
    if (result) violations.push({ kind: result.kind, held: result.held, requested: result.requested });
    held.set(step.resource, { directory: fixture.directory, resource: step.resource, mode: step.mode, task: 1, atMs: index + 1 });
  }
  assert.equal(held.size, 0);
  assert.deepEqual(violations, scenario.violations, scenario.id);
  results[scenario.id] = violations;
}
console.log(JSON.stringify(results));
