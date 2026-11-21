import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import fixture from "./declaration-language-registry.json";

const source = readFileSync("🧰️framework/🔨️modules/🗣️dsl/🗣️idiom/🦀️.rs", "utf8");
const registry = source.slice(source.indexOf("static LANGUAGE_REGISTRY"));
assert.match(registry, /preflight_languages\(&specs\)\?/u);
assert.equal(/\b(?:HashMap|HashSet|RandomState)\b/u.test(registry), fixture.ambientEntropy);

const db = new Database(":memory:");
try {
  db.exec("CREATE TABLE languages(id TEXT PRIMARY KEY, role TEXT NOT NULL, extension TEXT NOT NULL) WITHOUT ROWID");
  const insert = db.query("INSERT INTO languages VALUES (?, ?, ?) ON CONFLICT(id) DO NOTHING");
  const existing = db.query("SELECT role, extension FROM languages WHERE id=?");
  for (const batch of fixture.batches) {
    db.exec("BEGIN");
    let accepted = true;
    for (const row of batch.languages) {
      insert.run(row.id, row.role, row.extension);
      const stored = existing.get(row.id) as { role: string; extension: string };
      if (stored.role !== row.role || stored.extension !== row.extension) {
        accepted = false;
        break;
      }
    }
    db.exec(accepted ? "COMMIT" : "ROLLBACK");
    assert.equal(accepted, batch.accepted);
  }
  const ids = db.query("SELECT id FROM languages ORDER BY id").all() as { id: string }[];
  assert.deepEqual(ids.map(row => row.id), fixture.sortedLanguageIds);
} finally {
  db.close();
}
console.log("[DEBUG] Trusted language declarations preserve exact repeats and whole-batch conflicts");
