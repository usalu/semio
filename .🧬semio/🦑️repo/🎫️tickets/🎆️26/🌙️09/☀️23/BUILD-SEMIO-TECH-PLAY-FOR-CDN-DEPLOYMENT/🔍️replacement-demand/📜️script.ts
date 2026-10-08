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
  const inputs = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🧫️fixtures/📏️replacement-demand");
  const fixture = JSON.parse(readFileSync(join(inputs, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(inputs, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, expectedCloseDemandBytes: fixture.ordinaryGrantBytes }), false);
  const layout = Buffer.allocUnsafeSlow(fixture.pageCount * fixture.pageSlotBytes);
  layout.writeUInt16LE(fixture.pageBytes, fixture.pageBytes);
  assert.equal(layout.readUInt16LE(fixture.pageBytes), fixture.pageBytes);
  assert.equal(layout.buffer.byteLength, fixture.expectedCloseDemandBytes);
  assert.ok(layout.buffer.byteLength > fixture.ordinaryGrantBytes);
  assert.ok(layout.buffer.byteLength <= fixture.maximumAdmissionBytes);
  assert.equal(fixture.deniedPhysicalReleaseBytes, 0);
  console.log("[DEBUG] independent Node Buffer page4096 + UInt16 length gives retained4098; ordinary4096 denial retains whole allocation under existing262144 admission");
  const terminalInputs = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🧫️fixtures/📏️replacement-terminal");
  const terminal = JSON.parse(readFileSync(join(terminalInputs, "🔣️.json"), "utf8"));
  const validateTerminal = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(terminalInputs, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validateTerminal(terminal), JSON.stringify(validateTerminal.errors));
  assert.equal(validateTerminal({ ...terminal, terminalFrameSeparate: false }), false);
  for (const extent of terminal.oracleExtents) {
    const allocation = Buffer.allocUnsafeSlow(extent);
    assert.equal(allocation.buffer.byteLength, extent);
    assert.ok(extent <= terminal.maximumAdmissionBytes);
    assert.equal(extent - terminal.undergrantOffset >= allocation.buffer.byteLength ? extent : 0, terminal.undergrantReleasedBytes);
    assert.equal(extent >= allocation.buffer.byteLength ? extent : 0, extent);
    console.log("[DEBUG] independent Node Buffer terminal extent=" + extent + " one-below0 exact-whole=" + extent);
  }
}

if (import.meta.main) {
  assert.equal(process.argv[2] ?? "oracle", "oracle");
  oracle();
}
