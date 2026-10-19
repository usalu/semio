import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles } from "./lib.ts";
import Ajv from "/Users/ueli/Documents/semio/node_modules/ajv/dist/ajv.js";
const { validateJsonSchemaSubset } = await import(join(ROOT, "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts"));
const schema = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const ajv = new Ajv({ allErrors: true, strict: false });
const check = ajv.compile(schema);
let own = 0, third = 0, n = 0;
for (const f of projectFiles) {
  const j = JSON.parse(readFileSync(join(ROOT, f), "utf8"));
  if (!Object.keys(j.targets ?? {}).length) continue;
  n++;
  const e1 = validateJsonSchemaSubset(schema, j);
  if (e1.length) { own++; console.log("repo validator:", f.slice(-70), JSON.stringify(e1).slice(0, 300)); }
  if (!check(j)) { third++; console.log("ajv:", f.slice(-70), JSON.stringify(check.errors).slice(0, 300)); }
}
console.log({ manifests: n, repoValidatorFailures: own, ajvFailures: third });
