import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const R = "/Users/ueli/Documents/semio/";
const schema = JSON.parse(readFileSync(R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json", "utf8"));
const census = JSON.parse(readFileSync(import.meta.dir + "/../🗑️generated/cache-rest-census.json", "utf8"));
const OWNERS = new Set(["🌎️hub", "🎓️teaching", "🏢️semio-tech", "♻️mit-bestand", "👴️leutwiler", "root"]);
const files = [...new Set<string>(census.projects.filter((p: any) => OWNERS.has(p.owner)).map((p: any) => p.file))];
const ajv = new Ajv({ allErrors: true, strict: false });
const validate = ajv.compile(schema);
let bad = 0;
for (const file of files) {
  const json = JSON.parse(readFileSync(R + file, "utf8"));
  const own = validateJsonSchemaSubset(schema, json);
  const third = validate(json);
  if (own.length || !third) { bad++; console.log("INVALID", file, own.slice(0, 3), validate.errors?.slice(0, 3)); }
}
console.log(JSON.stringify({ files: files.length, invalid: bad }));
