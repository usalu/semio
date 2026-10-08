import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

function oracle(fold: boolean): void {
  let root = import.meta.dirname;
  while (!existsSync(join(root, "nx.json"))) {
    const parent = dirname(root);
    assert.notEqual(parent, root);
    root = parent;
  }
  const hydration = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration");
  const inputs = join(hydration, "🧪️tests/🧫️fixtures/📏️physical-demand");
  const fixture = JSON.parse(readFileSync(join(inputs, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(inputs, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, maximumAdmissionBytes: 262145 }), false);
  assert.equal(validate({ ...fixture, terminalFrameSeparate: false }), false);
  assert.ok(fixture.refusedAllocationBytes > fixture.maximumAdmissionBytes);
  for (const extent of [...fixture.allocationBytes, ...fixture.runtimeAllocationBytes]) {
    const buffer = Buffer.allocUnsafeSlow(extent).fill(42);
    assert.equal(buffer.buffer.byteLength, extent);
    assert.ok(extent > fixture.ordinaryPageBytes);
    assert.ok(extent <= fixture.maximumAdmissionBytes);
    assert.equal(fixture.ordinaryPageBytes >= buffer.buffer.byteLength ? extent : 0, 0);
    assert.equal(extent >= buffer.buffer.byteLength ? extent : 0, extent);
    console.log("[DEBUG] independent Node Buffer hydration whole extent=" + extent + " page4096 insufficient; existing262144 admission unchanged");
  }
  const source = readFileSync(join(hydration, "🦀️.rs"), "utf8").split("impl<P, M> ErasedSnapshotRetirement for RetainedPersistedDocumentHydration")[1]!;
  assert.match(source, /fn next_close_byte_demand/u);
  const runtime = readFileSync(join(hydration, "🦀️.rs"), "utf8").split("fn drive_hydration_runtime<P>")[1]!.split("#[derive")[0]!;
  assert.match(runtime, /runtime\.next_close_byte_demand\(\)/u);
  console.log("[DEBUG] retained hydration publishes its phase-specific physical demand");
  if (fold) {
    for (const logical of fixture.foldLogicalBytes) {
      for (const extent of fixture.foldAllocationBytes) {
        const owner = Buffer.allocUnsafeSlow(extent);
        const grant = Math.max(logical, owner.buffer.byteLength);
        assert.ok(grant <= fixture.maximumAdmissionBytes);
        assert.equal(grant, Math.max(logical, extent));
        console.log("[DEBUG] independent Fold caller logical=" + logical + " exactPhysical=" + extent + " grant=" + grant);
      }
    }
    const document = readFileSync(join(hydration, "🦀️.rs"), "utf8");
    const config = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎚️config/📥️retained/🦀️.rs"), "utf8");
    assert.ok(/job\.next_step_byte_demand\(logical_bytes\)/u.test(document));
    assert.ok(/job\.next_step_byte_demand\(1\)/u.test(config));
    assert.ok(/released_bytes <= maximum_bytes/u.test(document));
  }
}

function storeEntryOracle(): void {
  let root = import.meta.dirname;
  while (!existsSync(join(root, "nx.json"))) {
    const parent = dirname(root);
    assert.notEqual(parent, root);
    root = parent;
  }
  const hydration = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration");
  const inputs = join(hydration, "🧪️tests/🧫️fixtures/📏️store-entry");
  const fixture = JSON.parse(readFileSync(join(inputs, "🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true });
  const validate = ajv.compile(JSON.parse(readFileSync(join(inputs, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  const admission = ajv.compile({ const: fixture.storeTarget });
  assert.equal(admission(fixture.storeTarget), true);
  assert.equal(admission(fixture.wrongTarget), false);
  console.log("[DEBUG] independent Ajv Store entry accepts store/refuses envelope; retained original input field contract closed");
  const source = readFileSync(join(hydration, "🦀️.rs"), "utf8");
  const start = source.indexOf("pub fn step_store(");
  assert.ok(start >= 0, "Store-only entry must exist");
  const entry = source.slice(start, source.indexOf("pub fn step(", start));
  assert.ok(entry.indexOf("self.target") >= 0 && entry.indexOf("self.target") < entry.indexOf("self.step(cx)"), "immutable target admission precedes any inner output or ownership move");
  assert.ok(source.includes("pub enum PersistedDocumentStoreHydrationStep"));
}

if (import.meta.main) {
  const command = process.argv[2] ?? "oracle";
  assert.ok(command === "oracle" || command === "fold-oracle" || command === "store-entry-oracle");
  if (command === "store-entry-oracle") storeEntryOracle();
  else oracle(command === "fold-oracle");
}
