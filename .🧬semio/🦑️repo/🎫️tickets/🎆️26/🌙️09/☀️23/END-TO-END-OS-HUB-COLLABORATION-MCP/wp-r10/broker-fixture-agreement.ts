#!/usr/bin/env bun
/** 🧾️ R10 probe: the session-broker fixture cases (incl. the admin capability file) under Ajv (draft-07, the hub schema) and the TypeScript parsers agree case by case — the same comparison the os-hub `local-bootstrap-launch` law makes. */
import Ajv from "/Users/ueli/Documents/semio/node_modules/ajv/dist/ajv.js";
import { readFileSync } from "node:fs";
import { parseLocalAdminCapabilityV1, parseLocalSessionBrokerRecordV1, parseLocalSessionRequestV1, parseLocalSessionV1 } from "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
const root = "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap";
const schema = JSON.parse(readFileSync(`${root}/🧬️schema/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: false, allErrors: true });
ajv.addSchema(schema);
const cases = JSON.parse(readFileSync(`${root}/🧫️fixtures/🎫️session-broker-v1/🔣️.json`, "utf8")).cases as { id: string; def: string; value: unknown; valid: boolean }[];
const parsers: Record<string, (value: unknown) => unknown> = { LocalSessionBrokerRecordV1: parseLocalSessionBrokerRecordV1, LocalSessionRequestV1: parseLocalSessionRequestV1, LocalSessionV1: parseLocalSessionV1, LocalAdminCapabilityV1: parseLocalAdminCapabilityV1 };
let disagreements = 0;
for (const row of cases) {
  const validate = ajv.getSchema(`${schema.$id}#/$defs/${row.def}`)!;
  const byAjv = validate(row.value) === true;
  let byParser = true;
  try { parsers[row.def]!(row.value); } catch { byParser = false; }
  if (byAjv !== row.valid || byParser !== row.valid) { disagreements += 1; console.log(`DISAGREE ${row.id}: fixture=${row.valid} ajv=${byAjv} parser=${byParser}`); }
}
console.log(JSON.stringify({ cases: cases.length, admin: cases.filter((row) => row.def === "LocalAdminCapabilityV1").length, disagreements }));
process.exitCode = disagreements ? 1 : 0;
