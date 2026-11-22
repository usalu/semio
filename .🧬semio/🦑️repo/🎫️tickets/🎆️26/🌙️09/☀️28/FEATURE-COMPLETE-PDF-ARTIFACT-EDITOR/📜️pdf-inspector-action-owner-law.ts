/** 🧪 Verifies authored inspector controls use the same declaration their mounted caller owns. */
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import {join} from "node:path";
import laws from "./pdf-inspector-action-ownership.json";
import schema from "./pdf-inspector-action-ownership.schema.json";

const check = new Ajv({strict:true}).compile(schema);
assert(check(laws),JSON.stringify(check.errors));
const root = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page";
const live = await Bun.file(root+"/🪟️windows/🦀️.rs").text();
const stage = Bun.file(join(import.meta.dir,"🗑️generated/pdf-inspector-owner/windows.rs"));
const source = process.argv.includes("--staged") ? await stage.text() : live;
const control = await Bun.file(root+"/🔎️inspector/🦀️.rs").text();
const roster = source.match(/const INSPECTOR_ACTION_IDS: &\[&str\] = &\[([\s\S]+?)\];/u)?.[1] ?? "";
const declared = [...roster.matchAll(/"([^"]+)"/gu)].map(item=>item[1]);
const page = [...source.matchAll(/\baction\(\s*"([^"]+)"/gu)].map(item=>item[1]);
for (const id of laws.active) {
  assert((control + source).includes(JSON.stringify(id)), "Control no longer names "+id);
}
const database = new Database(":memory:");
database.exec("create table source (id text primary key, surface text); create table declared (id text primary key)");
for (const id of new Set(page)) database.query("insert into source values (?, 'pdf.page')").run(id);
for (const id of declared) database.query("insert into declared values (?)").run(id);
const admitted = database.query("select d.id from declared d join source s on s.id=d.id order by d.id").all() as {id:string}[];
assert.deepEqual(admitted.map(row=>row.id),laws.active.slice().sort());
assert(laws.other.every(id=>!declared.includes(id)));
assert(laws.app.every(id=>!declared.includes(id)));
assert.equal(declared.length,laws.active.length);
assert(source.includes("action.clone().in_palette(false)"));
database.close();
console.log("[DEBUG] 21 rendered inspector actions use shared window authority, five other actions stay scoped, and image pickers remain app-owned.");
