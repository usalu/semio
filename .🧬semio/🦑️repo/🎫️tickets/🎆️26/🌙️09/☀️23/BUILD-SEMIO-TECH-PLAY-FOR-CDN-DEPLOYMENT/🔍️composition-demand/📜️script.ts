import { strict as assert } from "node:assert";
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";

let workspace = resolve(import.meta.dir);
while (!existsSync(join(workspace, "nx.json"))) {
  const parent = dirname(workspace);
  assert.notEqual(parent, workspace);
  workspace = parent;
}
const domain = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/🌳️graph/📏️demand");
const fixture = JSON.parse(readFileSync(join(domain, "🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(domain, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
for (const candidate of [{ ...fixture, expectedDemandBytes: fixture.ordinaryBytes }, { ...fixture, maximumBytes: 1048576 }, { ...fixture, finalOwnedStage: 7 }]) assert.equal(validate(candidate), false);
const identifier = Buffer.allocUnsafeSlow(fixture.identifierBytes).fill(fixture.identifierByte);
const parent = Buffer.allocUnsafeSlow(fixture.identifierBytes).fill(fixture.parentByte);
const slot = Buffer.allocUnsafeSlow(fixture.identifierBytes).fill(fixture.slotByte);
assert.equal(new Set([identifier.toString("utf8"), parent.toString("utf8"), slot.toString("utf8")]).size, 3);
for (const owner of [parent, slot]) assert.equal(owner.buffer.byteLength, fixture.expectedDemandBytes);
assert.equal(identifier.buffer.byteLength, fixture.expectedDemandBytes);
assert.equal(identifier.toString("utf8").length, fixture.identifierBytes);
for (const bytes of fixture.deniedBytes) assert.equal(bytes >= identifier.buffer.byteLength, false);
assert.ok(identifier.buffer.byteLength <= fixture.maximumBytes);
assert.equal(fixture.physicalReceiptBytes, identifier.buffer.byteLength);
console.log(`[DEBUG] Node Buffer composition oracle whole=${identifier.buffer.byteLength} ordinary=${fixture.ordinaryBytes} denied=${fixture.deniedBytes.join(",")} admission=${fixture.maximumBytes}; closed Ajv schema and three negative witnesses pass`);
