const fs = require("node:fs");
const assert = require("node:assert/strict");
const Ajv = require("ajv");
const root = "🌎️hub/🧩️compositions/🖍️draw";
const guest = root + "/🧪️tests/🌐️guest";
const read = path => JSON.parse(fs.readFileSync(path, "utf8"));
const contract = read(guest + "/scenarios.json");
const validate = new Ajv().compile(read(guest + "/schema.json"));
assert.ok(validate(contract), JSON.stringify(validate.errors));
const apps = read(root + "/🧩️component/🔣️.json").manifest.apps;
assert.deepEqual(contract.scenarios.map(x => x.actor).sort(), ["editor", "viewer"]);
for (const scenario of contract.scenarios) {
  assert.ok(apps.some(app => app.id === scenario.appId && app.role === scenario.actor));
  console.log(scenario.appId + ": " + scenario.expectedStatus);
}
const duplicate = structuredClone(contract);
duplicate.scenarios[1] = structuredClone(duplicate.scenarios[0]);
assert.equal(validate(duplicate), false, "duplicate editor must fail");
const mismatch = structuredClone(contract);
mismatch.scenarios[1].appId = mismatch.scenarios[0].appId;
assert.equal(validate(mismatch), false, "viewer with editor coordinate must fail");
console.log("duplicate and mismatched surfaces rejected");
