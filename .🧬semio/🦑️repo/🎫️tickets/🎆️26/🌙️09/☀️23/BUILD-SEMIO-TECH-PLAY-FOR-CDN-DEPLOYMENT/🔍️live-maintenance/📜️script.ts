import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

function oracle(): void {
  let root = import.meta.dirname;
  while (!existsSync(join(root, "nx.json"))) {
    const parent = dirname(root);
    assert.notEqual(parent, root);
    root = parent;
  }
  const inputs = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/📏️live-physical-demand");
  const fixture = JSON.parse(readFileSync(join(inputs, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(inputs, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, maximumAdmissionBytes: 262145 }), false);
  for (const extent of fixture.allocationBytes) {
    const allocation = Buffer.allocUnsafeSlow(extent);
    assert.equal(allocation.buffer.byteLength, extent);
    assert.ok(extent > fixture.ordinaryGrantBytes && extent <= fixture.maximumAdmissionBytes);
    assert.equal(Math.max(fixture.ordinaryGrantBytes, allocation.buffer.byteLength), extent);
    console.log("[DEBUG] independent Node Buffer live whole extent=" + extent + " ordinary32768 insufficient; work1 unchanged; ceiling262144");
  }
  assert.ok(fixture.refusedAllocationBytes > fixture.maximumAdmissionBytes);
  assert.equal(fixture.refusedWorkItems, 0);
}

if (import.meta.main) {
  assert.equal(process.argv[2] ?? "oracle", "oracle");
  oracle();
}
