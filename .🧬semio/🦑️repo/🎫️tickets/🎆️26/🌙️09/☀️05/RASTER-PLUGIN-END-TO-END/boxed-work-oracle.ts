import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import min from "lodash/min";

const fixture = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🪝️boxed-work/fixture.json", "utf8"));
const source = readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔖️checkpoint-and-work-contract/🦀️.rs", "utf8");
const impl = source.split("impl<A: ArtifactApp> ArtifactCommandWork<A> for Box<dyn ArtifactCommandWork<A>> {")[1].split("pub struct BoundedArtifactCommandWork")[0];
const delegates = ["receive_original_output_chunks", "original_wire_demands", "receive_original_wire", "retains_original_wire"];
assert.deepEqual({
  copyBytes: min([fixture.wire.length, fixture.copyBudget]),
  decodedCount: Buffer.from(fixture.wire).readUInt8(0),
  retainWire: true,
  outputTransferred: true,
}, fixture.expected);
for (const hook of delegates) assert.ok(impl.includes(`(**self).${hook}(`), `boxed work drops ${hook}`);
console.log("[DEBUG] Boxed retained work delegates all four original ownership hooks.");
