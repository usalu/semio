/** 🧪️ Exercises both Pack receiver limits against the retained cursor on WASM32. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📥️pack";
const pack = "🧰️framework/🔨️modules/🎒️pack";
const source = readFileSync(join(root, "🦀️.rs"), "utf8");
const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🔣️.json"), "utf8"));
const defaults = readFileSync("🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs", "utf8");
const cursor = readFileSync(join(pack, "🌱️value/🛬️decode/📜️record-body-cursor/🦀️.rs"), "utf8");
const retained = readFileSync(join(pack, "🌱️intrinsic/🎮️retained/🦀️.rs"), "utf8");
const ceiling = defaults.match(/impl Default for PackLimits[\s\S]*?max_total_alloc: ([\d *]+) \}/)?.[1];
const helper = source.match(/^fn addressable_allocation_limit[^\n]+/m)?.[0];
const logicalClamp = retained.match(/let maximum=(maximum\.min\(.*?limits\.max_total_alloc[^;]+);/)?.[1];
const physicalRefusal = cursor.match(/if (maximum_allocation_bytes == 0 \|\| [^{]+)\{ return Err\(PackRefusal::LimitExceeded \{ kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body physical credits"/)?.[1];
assert(ceiling && helper && logicalClamp && physicalRefusal);
const expressions = [...source.matchAll(/let maximum=([^;]+);let d=RetirementDemand/g)].map((match) => match[1]);
assert.equal(expressions.length, 2);
const methods = expressions.map((expression, index) =>
  `#[unsafe(no_mangle)] pub extern "C" fn receive_${index}(bytes:u64)->i64 {
    let limits=Limits { max_total_alloc:bytes };
    let maximum=${expression};
    let maximum_allocation_bytes=${logicalClamp};
    if ${physicalRefusal} { return -1; }
    maximum_allocation_bytes as i64
  }`);
const out = join(import.meta.dir, "🗑️generated", "pdf-pack-wasm32-limit");
if (process.argv[2] === "emit") {
  writeFileSync(out + ".rs", [
    "struct Limits { max_total_alloc:u64 }", helper,
    `#[unsafe(no_mangle)] pub extern "C" fn default_limit()->u64 { ${ceiling} }`,
    ...methods,
  ].join("\n"));
} else {
  const { instance } = await WebAssembly.instantiate(readFileSync(out + ".wasm"));
  const guest = instance.exports as unknown as Record<string, (value?: bigint) => bigint>;
  const database = new Database(":memory:");
  const oracle = database.query("select min(cast(?1 as integer), (1 << 31) - 1) as maximum, cast(?1 as integer) > 0 as admitted");
  assert(fixture.allocationLimits.some((row: {bytes:string}) => BigInt(row.bytes) === guest.default_limit()));
  for (const { bytes, wasm32, admitted } of fixture.allocationLimits) {
    assert.deepEqual(oracle.get(bytes), { maximum: wasm32, admitted: Number(admitted) });
    for (let index = 0; index < expressions.length; index++) {
      assert.equal(guest["receive_" + index](BigInt(bytes)), admitted ? BigInt(wasm32) : -1n, `receiver ${index}, requested ${bytes} bytes`);
    }
  }
  database.close();
  process.stdout.write(`[DEBUG] Verified ${fixture.allocationLimits.length} limits through both WASM32 Pack paths and the retained cursor against SQLite\n`);
}
