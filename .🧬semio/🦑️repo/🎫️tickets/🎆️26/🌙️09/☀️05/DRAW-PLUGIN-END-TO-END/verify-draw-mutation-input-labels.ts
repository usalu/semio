/** 🌐️ Audits the Draw mutation forms with the real renderer input resolver and independent Ajv payload validation. */
import Ajv from "ajv";
import { resolve } from "node:path";
import { mutationInputAudit } from "../../../../../../../🧰️framework/🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts";
const fixture = await Bun.file(new URL("draw-mutation-input-labels.json", import.meta.url)).json();
const schemaValidator = new Ajv({ allErrors: true, strict: false });
const schemas = await Promise.all(fixture.map(async (row: { path: string; sample: Record<string, unknown> }) =>
  ({ ...row, schema: await Bun.file(resolve(row.path)).json() })));
const byId = new Map(schemas.map(row => [row.schema.$id, row.schema]));
const findings = [];
for (const { schema, sample } of schemas) {
  const accept = schemaValidator.compile(schema);
  if (!accept(sample)) throw Error(JSON.stringify(accept.errors));
  const audit = mutationInputAudit(schema, id => byId.get(id));
  findings.push(...audit.findings.map(finding => ({ schema: schema.title, ...finding })));
}
console.log(JSON.stringify({ schemas: schemas.length, findings }));
if (findings.length) process.exitCode = 1;
