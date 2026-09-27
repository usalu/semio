/** ⚖️ WG9 (ticket-local): Ajv (third-party) admits the backbone-parity fixture by its schema. Usage: bun wg9-parity-ajv.ts */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚖️parity";
const schema = JSON.parse(readFileSync(`${root}/🧬️schema/🔣️.json`, "utf8"));
const fixture = JSON.parse(readFileSync(`${root}/🧫️fixtures/🔣️.json`, "utf8"));
const validate = new Ajv({ allErrors: true, strict: false }).compile(schema);
console.log(validate(fixture) ? `ADMITTED ${fixture.scenarios.length} scenarios` : `REFUSED ${JSON.stringify(validate.errors).slice(0, 800)}`);
