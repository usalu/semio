import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import Ajv2020 from "ajv/dist/2020.js";

const base = new URL("./🗑️generated/natural-pdf-admission/", import.meta.url);
const read = (name) => readFileSync(new URL(name, base), "utf8");
const schema = JSON.parse(read("event-law/schema.json"));
const cases = JSON.parse(read("event-law/cases.json"));
const validate = new Ajv2020({ strict: true }).compile(schema);
const sql = new DatabaseSync(":memory:");
sql.exec("create table event (variant text primary key, value json)");
for (const { variant, input } of cases.cases) {
  assert.equal(validate(input), true, JSON.stringify(validate.errors));
  sql.prepare("insert into event values (?, ?)").run(variant, JSON.stringify(input));
}
assert.equal(sql.prepare("select count(variant) as count from event").get().count, 26);
for (const row of cases.invalid) assert.equal(validate(row), false, JSON.stringify(row));

const statement = sql.prepare(`
  select json_extract(p.value, '$.naturalSourcePrepare.extent') as prepared,
         json_extract(c.value, '$.naturalSourceControl.loan.extent') as loaned,
         json_extract(c.value, '$.naturalSourceControl.extent') as sent,
         json_extract(c.value, '$.naturalSourceControl.kind') as kind
  from event as p, event as c
  where p.variant = 'naturalSourcePrepare' and c.variant = 'naturalSourceControl'
`);
assert.deepEqual({ ...statement.get() }, { prepared: 9437877, loaned: 9437877, sent: 9437877, kind: 241 });

const open = structuredClone(cases.cases[0].input);
const { naturalSourcePrepare: prepare } = cases.cases.find(({ variant }) => variant === "naturalSourcePrepare").input;
open.instanceOpen.purpose = { naturalSource: { declaration: prepare.declaration, extent: prepare.extent } };
assert.equal(validate(open), true, JSON.stringify(validate.errors));
open.instanceOpen.purpose.naturalSource.extent = 0;
assert.equal(validate(open), false);
delete open.instanceOpen.purpose;
assert.equal(validate(open), false);
sql.close();
console.log("[DEBUG] Staged canonical Event shape passed all 26 cases and natural launch/loan authority checks.");
