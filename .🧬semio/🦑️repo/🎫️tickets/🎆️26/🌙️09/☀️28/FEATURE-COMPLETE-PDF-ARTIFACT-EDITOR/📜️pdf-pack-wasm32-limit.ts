/** 🧪️ Runs the actual typed and intrinsic Pack admission limit expressions in WASM32. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📥️pack";
const source = readFileSync(join(root, "🦀️.rs"), "utf8");
const fixture = JSON.parse(readFileSync(join(root, "🧪️tests/🔣️.json"), "utf8"));
const defaults = readFileSync("🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs", "utf8");
const ceiling = defaults.match(/impl Default for PackLimits[\s\S]*?max_total_alloc: ([\d *]+) \}/)?.[1];
assert(ceiling);
const expressions = [...source.matchAll(/let maximum=([^;]+);let d=RetirementDemand/g)].map((match) => match[1]);
assert.equal(expressions.length, 2);
const helper = source.match(/^fn addressable_allocation_limit[^\n]+/m)?.[0] ?? "";
const methods = expressions.map((expression, index) =>
  `#[unsafe(no_mangle)] pub extern "C" fn receive_${index}(bytes:u64)->i64 {
    fn limit(limits:Limits)->Result<i64,()> { let maximum=${expression}; Ok(maximum as i64) }
    limit(Limits { max_total_alloc:bytes }).unwrap_or(-1)
  }`);
const out = join(import.meta.dir, "🗑️generated", "pdf-pack-wasm32-limit");
if (process.argv[2] === "emit") {
  writeFileSync(out + ".rs", [
    "struct Limits { max_total_alloc:u64 }",
    "fn invalid() {}",
    helper,
    `#[unsafe(no_mangle)] pub extern "C" fn default_limit()->u64 { ${ceiling} }`,
    ...methods,
  ].join("\n"));
} else {
  const bytes = readFileSync(out + ".wasm");
  const { instance } = await WebAssembly.instantiate(bytes);
  const guest = instance.exports as unknown as Record<string, (value?: bigint) => bigint>;
  const database = new Database(":memory:");
  const oracle = database.query("select min(?1, ?2) as maximum");
  assert(fixture.allocationLimits.some((row: {bytes:number}) => BigInt(row.bytes) === guest.default_limit()));
  for (const { bytes: limit, wasm32 } of fixture.allocationLimits) {
    assert.deepEqual(oracle.get(limit, 2 ** 32 - 1), { maximum: wasm32 });
    for (let index = 0; index < expressions.length; index++) {
      assert.equal(guest["receive_" + index](BigInt(limit)), BigInt(wasm32), `receiver ${index}, requested ${limit} bytes`);
    }
  }
  database.close();
  process.stdout.write(`[DEBUG] Verified ${fixture.allocationLimits.length} limits in both WASM32 Pack admission paths against SQLite\n`);
}
