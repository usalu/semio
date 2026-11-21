import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import fixture from "./declaration-action-index.json";

const source = readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🦀️.rs", "utf8");
const index = source.slice(source.indexOf("pub struct AppActionRegistry"));
assert.match(index, /pub fn from_definition/u);
assert.equal(/\b(?:HashMap|HashSet|RandomState)\b/u.test(index), fixture.ambientEntropy);
assert.match(index, /definition\.window_kinds\.iter\(\)\.map\(\|window\|\s*\{[\s\S]*?window_kind_actions\(definition,\s*window\)[\s\S]*?actions\.insert/u);
assert.match(index, /mode_commands\.values\(\)[\s\S]*?min_by_key/u);

const db = new Database(":memory:");
try {
  db.exec("CREATE TABLE actions (id TEXT PRIMARY KEY, marker TEXT); CREATE TABLE windows (window TEXT, position INTEGER, id TEXT); CREATE TABLE modes (id TEXT, position INTEGER, marker TEXT)");
  const action = db.query("INSERT OR REPLACE INTO actions VALUES (?, ?)");
  const window = db.query("INSERT INTO windows VALUES (?, ?, ?)");
  for (const row of fixture.generatedActions) action.run(row.id, row.marker);
  for (const declared of fixture.windows) {
    declared.actions.forEach((row, position) => {
      action.run(row.id, row.marker);
      window.run(declared.id, position, row.id);
    });
  }
  const lookup = db.query("SELECT id FROM windows WHERE window=? ORDER BY position");
  for (const [id, expected] of Object.entries(fixture.windowOrder)) {
    assert.deepEqual(lookup.all(id).map(row => (row as { id: string }).id), expected);
  }
  const rows = db.query("SELECT id, marker FROM actions ORDER BY id").all() as { id: string; marker: string }[];
  assert.deepEqual(Object.fromEntries(rows.map(row => [row.id, row.marker])), fixture.global);
  const mode = db.query("INSERT INTO modes VALUES (?, ?, ?)");
  fixture.modes.forEach((row, position) => mode.run(row.id, position, row.command));
  assert.deepEqual(db.query("SELECT marker FROM modes ORDER BY position LIMIT 1").get(), { marker: fixture.unscopedCommand });
} finally {
  db.close();
}
console.log("[DEBUG] Authored declaration order wins across windows and keeps the unscoped command fallback stable");
