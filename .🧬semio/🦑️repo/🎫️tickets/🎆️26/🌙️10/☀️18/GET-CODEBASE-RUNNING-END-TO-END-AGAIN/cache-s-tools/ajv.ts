import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, manifests } from "./lib.ts";
const schema = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const validate = new Ajv({ allErrors: true, strict: false }).compile(schema);
let bad = 0, n = 0;
for (const f of manifests()) { n++; if (!validate(JSON.parse(readFileSync(join(ROOT, f), "utf8")))) { bad++; console.log(f, validate.errors?.slice(0, 2)); } }
console.log({ n, bad });
