/** 🔬️ The `bootDescriptor()` half of `🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts` (lines 33-40),
 * run on its own: extract `BOOT_FIELD_CAPACITY`/`LOCATION_SEARCH_CAPACITY`/`bounded`/`bootDescriptor`
 * from the shipped wgpu boot entry by TypeScript AST, evaluate them in a bare `node:vm`, and compare
 * every fixture row. Same extraction, same vm, same fixture — this exists because the owning suite
 * (`repo` cache contracts) also packages the vscode extension and runs two bundlers, which is not
 * this lane's gate.
 *
 * Usage: cd <ticket> && bun 🔍️boot-descriptor-oracle.ts
 */
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";

const require = createRequire(import.meta.url);
const ts = require("typescript");
const workspace = resolve(import.meta.dir, "../../../../../../..");
const engine = resolve(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const entry = resolve(engine, "🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts");
const fixture = JSON.parse(readFileSync(resolve(engine, "🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json"), "utf8"));

const text = readFileSync(entry, "utf8");
const source = ts.createSourceFile(entry, text, ts.ScriptTarget.Latest, true);
const names = new Set(["BOOT_FIELD_CAPACITY", "LOCATION_SEARCH_CAPACITY", "bounded", "bootDescriptor"]);
const statements = source.statements.filter((node: any) =>
  ts.isFunctionDeclaration(node) ? names.has(node.name?.text) : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration: any) => names.has(declaration.name?.text)),
);
assert.equal(statements.length, names.size, `expected ${names.size} extracted statements, found ${statements.length}`);
const runtime = ts.transpileModule(statements.map((node: any) => node.getText(source)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText;
const consumer = `const vm = require("node:vm"); const fixture = ${JSON.stringify(fixture)}; const runtime = ${JSON.stringify(runtime + "\nJSON.stringify(bootDescriptor())")}; const rows = fixture.cases.map(row => vm.runInNewContext(runtime, {URLSearchParams, DEFAULT_HOST_VARIANT: fixture.defaultVariant, PLAYGROUND_SESSION: {variant: fixture.ambientVariants[0]}, window: {location: {search: row.search}}})).map(JSON.parse); console.log(JSON.stringify(rows));`;
const result = spawnSync("node", ["-e", consumer], { cwd: workspace, encoding: "utf8", timeout: 10000 });
assert.equal(result.status, 0, result.stderr);
assert.deepEqual(
  JSON.parse(result.stdout),
  fixture.cases.map((row: any) => row.expected),
);
console.log(`boot-descriptor oracle: ${fixture.cases.length}/${fixture.cases.length} fixture rows agree with the shipped bootDescriptor()`);
for (const row of fixture.cases) console.log(`  ${JSON.stringify(row.search)} -> ${JSON.stringify(row.expected)}`);
