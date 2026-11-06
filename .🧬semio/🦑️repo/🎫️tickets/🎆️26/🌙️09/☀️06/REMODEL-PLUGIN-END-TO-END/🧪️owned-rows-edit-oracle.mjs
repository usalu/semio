import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("package.json"));
const Ajv = require("ajv");
const { findIndex } = require("lodash");
const { applyPatch } = require("fast-json-patch");
const root = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🧫️owned-rows-edit";
const fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
assert.ok(validate(fixture), JSON.stringify(validate.errors));
const less = (left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)) < 0;
function partition(rows, key) {
  if (!rows.length) return 0;
  let base = 0, size = rows.length;
  while (size > 1) {
    const half = Math.floor(size / 2), mid = base + half;
    if (less(rows[mid].id, key)) base = mid;
    size -= half;
  }
  return base + Number(less(rows[base].id, key));
}
for (const test of fixture.cases) {
  let candidate = structuredClone(test.base), failure = null;
  for (const row of test.rows) {
    const key = row.entity?.id ?? row.key;
    const found = findIndex(candidate, value => value.id === key);
    if (row.row === "insert" && found >= 0) { failure = "duplicate-id"; break; }
    if (row.row !== "insert" && found < 0) { failure = "missing-target"; break; }
    const patch = row.row === "insert" ? [{ op: "add", path: "/" + partition(candidate, key), value: structuredClone(row.entity) }]
      : row.row === "replace" ? [{ op: "replace", path: "/" + found, value: structuredClone(row.entity) }]
      : row.row === "remove" ? [{ op: "remove", path: "/" + found }]
      : [{ op: "test", path: "/" + found + "/id", value: key }];
    candidate = applyPatch(candidate, patch, true).newDocument;
  }
  if (test.refused) assert.equal(failure, test.refused, test.id);
  else { assert.equal(failure, null, test.id); assert.deepEqual(candidate, test.expected, test.id); }
}
console.log(JSON.stringify({ cases: fixture.cases.length, schema: true, oracles: ["lodash.findIndex", "fast-json-patch"], rustExecution: false }));

