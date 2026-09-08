/** 🧪️ Compiles every framework-module `🧬️schema/🔣️.json` export with ajv draft-07. */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const Ajv = require("ajv");
const ROOT = "🧰️framework/🔨️modules";
const DIALECT = "http://json-schema.org/draft-07/schema#";

const walk = (dir, out = []) => {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (entry === "🤖️generated" || entry === "node_modules" || entry === "target") continue;
    if (statSync(path).isDirectory()) walk(path, out);
    else if (entry === "🔣️.json" && dir.endsWith("🧬️schema")) out.push(path);
  }
  return out;
};

const REGISTRY = "🧰️framework/🔨️modules/🧬️schema/🔣️.json";
const modules = walk(ROOT).filter(path => !path.includes("🧬️mutations") && path !== REGISTRY).sort();
const ajv = new Ajv({ strict: true, allErrors: true, validateSchema: true });
for (const keyword of ["x-semio-binary", "x-semio-state", "x-semio-formats"]) ajv.addKeyword({ keyword, metaSchema: {} });
const documents = new Map();
let dialect = 0, ids = 0, defs = 0;
const problems = [];

for (const path of modules) {
  let document;
  try { document = JSON.parse(readFileSync(path, "utf8")); }
  catch (error) { problems.push([path, `parse: ${error.message}`]); continue; }
  if (Array.isArray(document)) { problems.push([path, "root is an array, not a schema module"]); continue; }
  if (document.$schema !== DIALECT) { problems.push([path, `dialect ${document.$schema ?? "(none)"}`]); dialect++; }
  if (typeof document.$id !== "string" || !document.$id.startsWith("https://semio.tech/schema/")) { problems.push([path, `id ${document.$id ?? "(none)"}`]); ids++; }
  if (!document.$defs || Object.keys(document.$defs).length === 0) { problems.push([path, "no $defs exports"]); defs++; }
  documents.set(path, document);
}

for (const [path, document] of documents) {
  try { if (document.$id && !ajv.getSchema(document.$id)) ajv.addSchema(document); }
  catch (error) { problems.push([path, `addSchema: ${error.message}`]); }
}

let compiled = 0;
for (const [path, document] of documents) {
  for (const name of Object.keys(document.$defs ?? {})) {
    try {
      const validate = ajv.getSchema(`${document.$id}#/$defs/${name}`);
      if (!validate) { problems.push([path, `${name}: unresolved pointer`]); continue; }
      compiled++;
    } catch (error) { problems.push([path, `${name}: ${error.message}`]); }
  }
}

console.log(`modules=${modules.length} exports=${compiled} badDialect=${dialect} badId=${ids} noExports=${defs} problems=${problems.length}`);
for (const [path, reason] of problems) console.log(`  ${path}\n    ${reason}`);
process.exit(problems.length === 0 ? 0 : 1);
