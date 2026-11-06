import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";

const require = createRequire(resolve("package.json"));
const Ajv = require("ajv");
const { applyPatch } = require("fast-json-patch");
const root = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🧫️retained-params";
const fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
assert.ok(validate(fixture), JSON.stringify(validate.errors));
assert.equal(new Set(fixture.cases.map(row => row.field)).size, 8);
for (const row of fixture.cases) {
  const before = { [row.field]: { [row.member]: row.before }, untouched: { witness: "original" } };
  const authored = { [row.member]: row.after };
  const forward = [{ op: "replace", path: "/" + row.field, value: authored }];
  const inverse = [{ op: "replace", path: "/" + row.field, value: before[row.field] }];
  const expected = { ...before, [row.field]: authored };
  const after = applyPatch(structuredClone(before), forward, true).newDocument;
  assert.deepEqual(after, expected);
  assert.deepEqual(applyPatch(structuredClone(after), inverse, true).newDocument, before);
  assert.deepEqual(before[row.field], { [row.member]: row.before });
}
console.log(JSON.stringify({ cases: fixture.cases.length, schema: true, oracle: "fast-json-patch", forward: true, inverse: true, rustExecution: false }));

