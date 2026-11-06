import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(resolve("package.json"));
const Ajv = require("ajv");
const { transform } = require("lodash");
const root = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔎️key";
const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
assert.ok(validate(fixture), JSON.stringify(validate.errors));
for (const row of fixture.cases) {
  const actual = transform(row.keys, (indices, key, index) => { if (key === row.needle) indices.push(index); return indices.length < 2; }, []);
  assert.deepEqual(actual, row.expectedIndices, row.id);
}
console.log(JSON.stringify({ cases: fixture.cases.length, schema: true, oracle: "lodash.transform", maximumResults: 2, rustExecution: false }));

