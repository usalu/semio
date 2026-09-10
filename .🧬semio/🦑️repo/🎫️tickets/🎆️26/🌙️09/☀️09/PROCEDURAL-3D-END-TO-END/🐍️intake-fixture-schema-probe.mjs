// 🔬️ Validates the extended retained-UI IntakeFixture against its own `$defs` entry with Ajv, the same
// oracle `📃️UiDocumentStore/🧪️tests/🧪️typedwire` uses, so the schema and the fixture cannot drift.
import Ajv from "ajv";
import { readFileSync } from "node:fs";
const base = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained";
const schema = JSON.parse(readFileSync(`${base}/🧬️schema/🔣️.json`, "utf8"));
const fixture = JSON.parse(readFileSync(`${base}/🧫️fixtures/📥️intake/🔣️.json`, "utf8"));
const definition = { $schema: schema.$schema, ...schema.$defs.IntakeFixture };
const validate = new Ajv({ strict: true }).compile(definition);
const ok = validate(fixture);
console.log("IntakeFixture valid:", ok, ok ? "" : JSON.stringify(validate.errors, null, 2));
console.log("budget:", JSON.stringify(fixture.budget));
console.log("laws:", fixture.laws.length);
process.exit(ok ? 0 : 1);
