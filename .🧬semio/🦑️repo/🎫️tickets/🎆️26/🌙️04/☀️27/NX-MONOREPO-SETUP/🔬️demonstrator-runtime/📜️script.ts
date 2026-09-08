import { testDemonstratorRuntime } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import ts from "typescript";
await testDemonstratorRuntime(process.cwd());
const source = ts.createSourceFile("📜️script.ts", readFileSync("📜️script.ts", "utf8"), ts.ScriptTarget.Latest, true);
const declaration = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation")!;
const code = ts.transpileModule(declaration.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const resolve = new Function("process", `${code}; return resolveNxInvocation;`)({ env: {} });
const fixture = JSON.parse(readFileSync("♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️invocations.json", "utf8"));
for (const row of fixture.valid) { const result = resolve(row.input); assert.deepEqual(result.args, row.args); assert.deepEqual(result.env, row.env); }
for (const row of fixture.invalid) assert.throws(() => resolve(row));
console.log("[DEBUG] Demonstrator preparation routes preserve the selected native profile and reject nested compiler arguments PASS");
