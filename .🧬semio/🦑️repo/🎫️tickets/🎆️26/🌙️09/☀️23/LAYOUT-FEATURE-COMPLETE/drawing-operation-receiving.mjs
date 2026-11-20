import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";

const mutationSchema = readFileSync("✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs", "utf8");
const decoder = readFileSync("✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs", "utf8");
const fixture = JSON.parse(readFileSync(new URL("./drawing-operation-receiving.json", import.meta.url), "utf8"));
const declaration = mutationSchema.match(/pub enum DrawingMutation \{([^}]+)\}/)?.[1] ?? "";
const actual = [...declaration.matchAll(/^\s*(\w+)\(\w+\),$/gm)].map(row => row[1]);
assert.deepEqual(actual, fixture.variants);
assert.match(decoder, /original_ordinal_operation_receiving!\(DrawingMutation\)/);

const db = new Database(":memory:");
try {
  db.exec("create table operation (ordinal integer primary key, name text unique, header blob)");
  for (const [ordinal, name] of fixture.variants.entries()) {
    db.query("insert into operation values (?, ?, char(1) || char(?))").run(ordinal, name, ordinal);
    assert.equal(db.query("select hex(header) as value from operation where name = ?").get(name).value,
      Buffer.from([1, ordinal]).toString("hex").toUpperCase());
  }
  assert.equal(db.query("select count(*) as n from operation").get().n, 27);
} finally {
  db.close();
}
console.log("27 Drawing operation discriminants and retained decoder registration match the original ordinal wire");
