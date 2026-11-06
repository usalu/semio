import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("package.json"));
const Ajv = require("ajv");
const { cloneDeep } = require("lodash");
const root = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🧫️owned-rows-buffer";
const fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
assert.ok(validate(fixture), JSON.stringify(validate.errors));
for (const row of fixture.cases) {
  assert.deepEqual(cloneDeep(row.input), row.expected, row.id);
  const moved = [];
  const pending = structuredClone(row.input);
  while (pending.length) moved.push(pending.pop());
  moved.reverse();
  assert.deepEqual(moved, row.expected, row.id);
  assert.ok(Number.isSafeInteger(row.input.length + row.additional));
}
console.log(JSON.stringify({ cases: fixture.cases.length, schema: true, oracle: "lodash.cloneDeep", order: true, rustExecution: false }));

