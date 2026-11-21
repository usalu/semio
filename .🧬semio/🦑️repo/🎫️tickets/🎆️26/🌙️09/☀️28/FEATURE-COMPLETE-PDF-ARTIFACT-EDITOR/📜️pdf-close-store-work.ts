/** 🧪️ Runs the mounted OwnedStores close work receipt expression against the neutral fixture. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import Ajv from "ajv";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const corpus = join(root, "♻️lifecycle/🏪️stores");
const fixture = JSON.parse(readFileSync(join(corpus, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(corpus, "🧬️schema/🔣️.json"), "utf8"));
const source = readFileSync(join(root, "🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs"), "utf8");
const branch = source.split("let result=self.ladder_ctx().close_owned_stores_step(effect);")[1];
const expression = branch?.match(/let work=([^;]+);/)?.[1];
assert(expression);
const out = join(import.meta.dir, "🗑️generated", "pdf-close-store-work");
if (process.argv[2] === "emit") {
  writeFileSync(out + ".rs", `
#[derive(Default,PartialEq)]
struct RetainedCloneProgress { copied_items:u32, copied_bytes:u32, retained_capacity_bytes:u32, released_bytes:u32 }
struct Effect(u128);
impl Effect { fn performed_work(&self)->u128 { self.0 } }
#[unsafe(no_mangle)] pub extern "C" fn settle(performed:u32,copied_items:u32,released_bytes:u32)->u32 {
    let effect=Effect(u128::from(performed));
    let progress=RetainedCloneProgress { copied_items,released_bytes,..Default::default() };
    let work=${expression};
    work as u32
}`);
} else {
  assert(new Ajv({strict:false}).compile(schema)(fixture));
  const { instance } = await WebAssembly.instantiate(readFileSync(out + ".wasm"));
  const settle = instance.exports.settle as unknown as (performed:number, items:number, release:number) => number;
  const database = new Database(":memory:");
  const oracle = database.query("select max(?1, (?2 != 0 or ?3 != 0)) as spent");
  for (const row of fixture.closeWork) {
    assert.deepEqual(oracle.get(row.performed, row.copiedItems, row.releasedBytes), {spent: row.spent}, row.name);
    assert.equal(settle(row.performed, row.copiedItems, row.releasedBytes), row.spent, row.name);
  }
  database.close();
  process.stdout.write(`[DEBUG] Verified ${fixture.closeWork.length} close work receipts against SQLite\n`);
}
