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
  const folder = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/♻️typed-source-physical");
  const fixture = JSON.parse(readFileSync(join(folder, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(folder, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, maximumAllocationBytes: 262144 }), false);
  assert.equal(validate({ ...fixture, physicalRelease: { ...fixture.physicalRelease, demandBytes: 4096 } }), false);
  assert.equal(validate({ ...fixture, physicalRelease: { ...fixture.physicalRelease, terminalFrameSeparate: false } }), false);
  const source = Buffer.allocUnsafeSlow(fixture.textBytes).fill(fixture.textByte);
  const backing = source.buffer;
  assert.equal(backing.byteLength, fixture.physicalRelease.demandBytes);
  assert.ok(backing.byteLength <= fixture.maximumAllocationBytes);
  let logical = source;
  while (logical.byteLength !== 0) {
    logical = logical.subarray(0, Math.max(0, logical.byteLength - fixture.maximumBytes));
    assert.equal(logical.buffer, backing);
    assert.equal(backing.byteLength, fixture.textBytes);
  }
  for (const caller of fixture.physicalRelease.deniedBytes) {
    const admitted = caller >= backing.byteLength ? backing.byteLength : 0;
    assert.equal(admitted, 0);
    console.log("[DEBUG] Independent Node Buffer whole-allocation oracle denied caller=" + caller + " extent=" + backing.byteLength + " admittedRelease=" + admitted);
  }
  assert.equal(fixture.physicalRelease.reportedBytes, backing.byteLength);
  console.log("[DEBUG] Independent Ajv/Node Buffer oracle logical4096 retains exact8194 backing; whole8194 release admitted under unchanged65536 ceiling; terminal frame separate");
}

if (import.meta.main) {
  assert.equal(process.argv[2] ?? "oracle", "oracle");
  oracle();
}
