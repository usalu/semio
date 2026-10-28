/** 🧬 Validate the calm cursor contract using the existing AJV oracle. */
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const read = file => JSON.parse(fs.readFileSync(path.join(__dirname, file), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(read("cursor-motion.schema.json"));
const model = read("cursor-motion.json");
assert.equal(validate(model), true, JSON.stringify(validate.errors));
const cases = read("cursor-motion.cases.json");
for (const test of cases.invalidModels) {
  const candidate = structuredClone(model);
  const parent = test.path.slice(0, -1).reduce((value, key) => value[key], candidate);
  const key = test.path.at(-1);
  if (test.remove) delete parent[key];
  else parent[key] = test.value;
  assert.equal(validate(candidate), false, test.id);
}
console.log("Calm cursor schema accepted; " + cases.invalidModels.length + " invalid mutations rejected.");
