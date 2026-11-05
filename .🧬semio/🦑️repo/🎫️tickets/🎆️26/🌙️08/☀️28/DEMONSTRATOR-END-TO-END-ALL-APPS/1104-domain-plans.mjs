import { readFileSync } from "node:fs";
import Ajv from "ajv";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/✏️host-authoring/🧭️domain-plans/🧪️laws/";
const schema = JSON.parse(readFileSync(root + "📐️schema.json", "utf8"));
const cases = JSON.parse(readFileSync(root + "📁️cases.json", "utf8"));
const validate = new Ajv({allErrors:true}).compile(schema);
if (!validate(cases)) throw new Error(JSON.stringify(validate.errors));
console.log("Ajv validated all " + cases.length + " neutral domain authoring cases, including reorganize.");
