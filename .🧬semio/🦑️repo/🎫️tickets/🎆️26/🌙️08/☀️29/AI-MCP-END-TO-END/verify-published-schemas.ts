import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { runInNewContext } from "node:vm";
import Ajv from "ajv";
import ts from "typescript";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const owner = readFileSync(join(root, "📦️packages/🦀️rust/📜️script.ts"), "utf8");
const renderer = owner.slice(owner.indexOf("type SchemaNode ="), owner.indexOf("const router ="));
const json = execFileSync(process.argv[2], ["schemas"], { encoding: "utf8", maxBuffer: 16 << 20 });
const document = JSON.parse(json);
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(document);
for (const name of Object.keys(document.$defs)) {
  if (!ajv.getSchema(document.$id + "#/$defs/" + name)) throw Error(name);
}
const typescript = runInNewContext(ts.transpile(renderer + "\nrenderSchemaTypescript(document)", {target: ts.ScriptTarget.ESNext}), { document });
for (const [name, expected] of [["🔣️.json", json], ["🟦️.ts", typescript]]) {
  const path = join(root, "🧬️schema", name);
  const matches = readFileSync(path, "utf8") === expected;
  console.log(name, matches ? "matches" : "stale");
  if (!matches && process.argv.includes("--write")) writeFileSync(path, expected);
  else if (!matches) process.exitCode = 1;
}
console.log(Object.keys(document.$defs).length, "schemas resolved");
