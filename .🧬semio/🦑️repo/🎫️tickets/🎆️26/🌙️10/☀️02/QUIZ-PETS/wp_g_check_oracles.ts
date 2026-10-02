/** 🔮️ Ticket tool of work package G: validates the pets oracle registry against the schema of the test harness with ajv and lists its oracle ids.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_g_check_oracles.ts"
 */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../..");
const registry = JSON.parse(readFileSync(resolve(root, "🧰️framework/🛍️products/🐾️pets/🔮️oracles/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ allErrors: true, strict: false });
const valid = ajv.validate(schema, registry);
console.log(JSON.stringify({ valid, errors: ajv.errors ?? [], oracles: registry.oracles.map((oracle: { id: string }) => oracle.id) }, null, 1));
process.exit(valid ? 0 : 1);
