import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import fixture from "./declaration-example-cache.json";

type Dialect = (typeof fixture.lookups)[number]["dialect"];
const surface = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface";
const source = readFileSync(`${surface}/🏗️surface-builders/🦀️.rs`, "utf8");
const helpers = source.slice(source.indexOf("fn subset_example_key("));
const fields = helpers.match(/\(dialect\.(\w+),\s*dialect\.(\w+)\.0,\s*dialect\.(\w+)\.0\)/u)?.slice(1);
assert.deepEqual(fields, ["artifact_kind", "standard", "subset"]);
assert.match(helpers, /BTreeMap::new\(\)/u);
assert.match(helpers, /if examples\.is_empty\(\)\s*\{\s*return;\s*\}/u);
assert.match(helpers, /insert\(subset_example_key\(&dialect\),\s*examples\.to_vec\(\)\)/u);
assert.match(helpers, /get\(&subset_example_key\(dialect\)\)\.cloned\(\)\.unwrap_or_default\(\)/u);

const key = (dialect: Dialect) => [dialect.artifactKind, dialect.standard, dialect.subset] as const;
const declarations = new Map<string, string[]>();
const db = new Database(":memory:");
try {
  db.exec("CREATE TABLE registry(kind TEXT, standard TEXT, subset TEXT, examples TEXT, PRIMARY KEY(kind,standard,subset)) WITHOUT ROWID");
  const upsert = db.query("INSERT OR REPLACE INTO registry VALUES (?, ?, ?, ?)");
  const lookup = db.query("SELECT examples FROM registry WHERE kind=? AND standard=? AND subset=?");
  for (const row of fixture.registrations) {
    if (!row.examples.length) continue;
    declarations.set(JSON.stringify(key(row.dialect)), row.examples);
    upsert.run(...key(row.dialect), JSON.stringify(row.examples));
  }
  for (const row of fixture.lookups) {
    const expected = lookup.get(...key(row.dialect)) as { examples: string } | null;
    assert.deepEqual(expected ? JSON.parse(expected.examples) : [], row.examples);
    assert.deepEqual(declarations.get(JSON.stringify(key(row.dialect))) ?? [], row.examples);
  }
} finally {
  db.close();
}
console.log("[DEBUG] Complete dialect keys preserve declaration order, replacement and absent examples");
