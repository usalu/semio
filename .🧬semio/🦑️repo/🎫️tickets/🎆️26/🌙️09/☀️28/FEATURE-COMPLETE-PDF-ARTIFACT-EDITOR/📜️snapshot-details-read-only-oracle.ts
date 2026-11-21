/** 🔍 Verify the neutral details write policy with Ajv and the staged PDF schema annotations. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import Ajv from "ajv";

const filename = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const fixture = filename(join(import.meta.dirname, "🔣️snapshot-details-read-only.json"));
const validate = new Ajv({strict: true}).compile(fixture.schema);
const writer = new Ajv({strict: true}).compile({
  ...fixture.schema,
  properties: {
    ...fixture.schema.properties,
    declaredVersion: {const: fixture.snapshot.declaredVersion},
    roles: {const: fixture.snapshot.roles},
    retained: {const: fixture.snapshot.retained},
  },
});
for (const row of fixture.writes) {
  const changed = structuredClone(fixture.snapshot);
  const parent = row.segments.slice(0, -1).reduce((target, key) => target[key], changed);
  parent[row.segments.at(-1)] = row.value;
  assert(validate(changed), JSON.stringify(validate.errors));
  assert.equal(writer(changed), row.allowed, row.segments);
}
const changes = filename(join(import.meta.dirname, "🗑️generated", "snapshot-read-only", "stage.json"));
const [source, receipt] = Object.entries(changes).find(([path]) => path.endsWith("📸️snapshot/🔣️.json")) as [string, {staged: string}];
const before = filename(resolve(source)), after = filename(receipt.staged);
for (const key of ["schema", "declaredVersion", "admittedStreamRoles"]) {
  assert(after.properties[key].readOnly);
  delete after.properties[key].readOnly;
}
for (const key of ["properties", "admittedStreamRoles"]) {
  const label = after.properties[key]["x-semio-ui"].label;
  assert(label.en.length && label.de.length && label.en !== label.de);
  delete after.properties[key]["x-semio-ui"];
}
assert.deepEqual(after, before);
console.log("[DEBUG] Ajv verified four read/write cases and exact annotation-only PDF schema changes");
