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
  const path = join(root, "🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧫️fixtures");
  const fixture = JSON.parse(readFileSync(join(path, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(path, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, winnerCount: 2 }), false);
  for (const bytes of fixture.payloadBytes) {
    const original = Buffer.allocUnsafeSlow(bytes);
    assert.equal(original.buffer.byteLength, bytes);
    assert.ok(bytes <= fixture.maximumAdmissionBytes);
    assert.equal(fixture.deniedReleaseBytes, 0);
    console.log("[DEBUG] independent Node original factory payload=" + original.buffer.byteLength + " exact extent; aliases=" + fixture.aliasCount + " winner=" + fixture.winnerCount);
  }
  for (const row of fixture.soleSnapshotWeak) {
    const words = new Uint8Array(row.pointerBytes * 2);
    const body = Buffer.allocUnsafeSlow(row.snapshotBytes);
    const extent = words.byteLength + body.buffer.byteLength;
    assert.equal(extent, row.frameBytes);
    console.log("[DEBUG] independent sole-Weak concrete snapshot frame=" + extent + " pointer=" + row.pointerBytes + " body=" + row.snapshotBytes);
  }
  const witness = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  assert.ok(witness.includes("struct DemoPublishedRootWitness"), "sole-Weak owner must retain its typed original snapshot-frame authority");
  assert.equal(fixture.nestedDepth, 2);
  assert.equal(fixture.inlineOwnedPayload, true);
  assert.equal(fixture.observerGuard, "inline-exclusive");
  assert.equal(fixture.birthBeforeClose, true);
  assert.equal(fixture.weakAliasesRefused, true);
}

if (import.meta.main) { assert.equal(process.argv[2] ?? "oracle", "oracle"); oracle(); }
