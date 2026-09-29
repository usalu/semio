/** ✅️ H13: compiles a staged execution-target resolution schema with the repo's strict Ajv and validates a staged fixture.
 * usage: bun h13-ajv-eviction-check.ts <schema> <fixture> */
import { readFileSync } from "node:fs";
import { semioSchemaAjvV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";

const validate = semioSchemaAjvV1({ allErrors: true, strict: true }).compile(JSON.parse(readFileSync(process.argv[2]!, "utf8")));
const valid = validate(JSON.parse(readFileSync(process.argv[3]!, "utf8")));
console.log(valid ? "VALID" : `INVALID ${JSON.stringify(validate.errors)}`);
process.exit(valid ? 0 : 1);
