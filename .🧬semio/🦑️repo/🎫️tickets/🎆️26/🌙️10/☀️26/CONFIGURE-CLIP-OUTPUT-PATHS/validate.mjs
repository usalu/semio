import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020.js";
import ts from "typescript";

let workspace = dirname(fileURLToPath(import.meta.url));
while (!existsSync(join(workspace, "🧰️framework"))) workspace = dirname(workspace);
const authoring = join(workspace, "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring");
const folder = join(authoring, "🎬️clip/📁️output");
const schema = JSON.parse(readFileSync(join(folder, "🧬️schema/🔣️.schema.json"), "utf8"));
const cases = JSON.parse(readFileSync(join(folder, "🧪️tests/🔣️.cases.json"), "utf8"));
const validate = new Ajv({ strict: true }).compile(schema);
for (const entry of cases.roots) {
  if (!validate(entry.environment)) throw Error(JSON.stringify(validate.errors));
}
const file = ts.createSourceFile("script.ts", readFileSync(join(authoring, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
if (file.parseDiagnostics.length) throw Error(JSON.stringify(file.parseDiagnostics));
console.log("AJV: portable environment cases valid; TypeScript launcher syntax valid.");
