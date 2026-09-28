import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { deepStrictEqual } from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import Ajv from "ajv";
import { parseLayoutDiff } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
const subset = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any";
const schema = JSON.parse(readFileSync(join(subset, "🧬️schema/🔺️diff/🔣️.json"), "utf8"));
const draft = String(schema.$schema ?? "");
const validate = (draft.includes("2020") ? new Ajv2020({ strict: false, allErrors: true }) : new Ajv({ strict: false, allErrors: true })).compile(schema);
const diffs: string[] = [];
const walk = (dir: string): void => { for (const name of readdirSync(dir)) { const path = join(dir, name); if (statSync(path).isDirectory()) walk(path); else if (name === "🔣️.json" && dir.endsWith("🔺️diff")) diffs.push(path); } };
walk(join(subset, "🧫️fixtures/🧬️mutations"));
let framePatched = 0, schemaOk = 0, parsed = 0;
const failures: string[] = [];
for (const path of diffs) {
  const value = JSON.parse(readFileSync(path, "utf8"));
  const text = JSON.stringify(value);
  if (text.includes('"frame_patched":{')) framePatched += 1;
  if (validate(value)) schemaOk += 1; else failures.push(`schema ${path}: ${JSON.stringify(validate.errors?.slice(0, 2))}`);
  try { deepStrictEqual(parseLayoutDiff(value), value); parsed += 1; } catch (error) { failures.push(`parse ${path}: ${String(error).slice(0, 200)}`); }
}
console.log(JSON.stringify({ diffs: diffs.length, framePatched, schemaOk, parsed, failures }, null, 1));
