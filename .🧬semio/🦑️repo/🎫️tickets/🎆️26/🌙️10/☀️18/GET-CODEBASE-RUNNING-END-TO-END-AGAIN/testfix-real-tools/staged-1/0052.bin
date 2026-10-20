/** 🚏️ The retained mutation dispatch corpus validates against its closed schema and its leaf count agrees with an independent SQLite projection. */
import { strict as assert } from "node:assert";
import { Database } from "bun:sqlite";
import { test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("retained mutation dispatch leaves match the closed schema and the SQLite leaf count", () => {
  assert.equal(new Ajv2020({ strict: true }).compile(schema)(corpus), true);
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE dispatch_leaves(kind TEXT PRIMARY KEY)");
    corpus.variants.forEach(kind=>db.query("INSERT INTO dispatch_leaves VALUES(?)").run(kind));
    assert.equal((db.query("SELECT COUNT(*) AS leaves FROM dispatch_leaves").get() as {leaves:number}).leaves,corpus.requiredLeafCount);
  } finally { db.close(); }
});
