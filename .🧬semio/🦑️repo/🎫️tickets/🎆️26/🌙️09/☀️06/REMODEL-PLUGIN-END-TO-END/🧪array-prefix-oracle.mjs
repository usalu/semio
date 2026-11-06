import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Database } from "bun:sqlite";

const root = process.cwd();
const require = createRequire(resolve(root, "package.json"));
const Ajv = require("ajv/dist/2020").default;
const base = resolve(root, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🧪️tests/🧫️fixtures");
const rows = JSON.parse(readFileSync(resolve(base, "staged.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(base, "staged.schema.json"), "utf8"));
const validate = new Ajv({ strict: true }).compile(schema);
if (!validate(rows)) throw new Error(JSON.stringify(validate.errors));
const db = new Database(":memory:");
const query = db.query(`WITH input AS (SELECT ? AS n, ? AS i, ? AS w, ? AS word, ? AS body, ? AS fuel),
  amount AS (SELECT *, min(n-i, 1024, CASE WHEN w=0 THEN 1024 ELSE max(body-word,0)/w END, max(fuel-6,0)/(w+2)) AS count FROM input)
  SELECT count, CASE WHEN count=0 THEN 0 ELSE w*count+word END AS copy, CASE WHEN count=0 THEN 0 ELSE 6+count*(w+2) END AS spent FROM amount`);
const names = new Set();
for (const row of rows) {
  if (names.has(row.name) || row.index > row.length) throw new Error(`Invalid case ${row.name}`);
  names.add(row.name);
  const actual = query.get(row.length, row.index, row.width, row.wordBytes, row.body, row.work);
  for (const key of ["count", "copy", "spent"]) if (actual[key] !== row[key]) throw new Error(`${row.name}: ${key} expected ${row[key]}, got ${actual[key]}`);
}
db.close();
console.log(`Validated ${rows.length} Array prefix vectors with Ajv and SQLite.`);
