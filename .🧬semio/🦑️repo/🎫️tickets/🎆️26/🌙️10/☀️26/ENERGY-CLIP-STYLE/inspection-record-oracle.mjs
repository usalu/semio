import fs from "node:fs";
import assert from "node:assert/strict";
const input = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
for (const raw of input.snapshots) {
  const state = JSON.parse(raw);
  assert.deepEqual(state.old, input.cases.initial.old);
  for (const [key, value] of Object.entries(state)) {
    if (key === "old") continue;
    assert.match(key, /^writer[0-9]+$/);
    assert.equal(value.state, "running");
    assert.equal(Number.isInteger(value.sequence), true);
    assert.ok(value.sequence >= 0 && value.sequence < input.cases.updates_per_writer);
  }
}
const final = JSON.parse(input.final);
const expected = structuredClone(input.cases.initial);
for (let index = 0; index < input.cases.writers; index++) {
  expected["writer" + index] = {state: "running", sequence: input.cases.updates_per_writer - 1};
}
assert.deepEqual(final, expected);
console.log(JSON.stringify({valid: true, observations: input.snapshots.length, final}));
