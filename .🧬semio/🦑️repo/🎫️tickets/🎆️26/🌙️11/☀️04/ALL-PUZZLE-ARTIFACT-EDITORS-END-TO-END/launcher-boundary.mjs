import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import ts from "typescript";

const setup = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts";
const source = ts.createSourceFile(setup, readFileSync(setup, "utf8"), ts.ScriptTarget.Latest, true);
const imports = source.statements.filter(ts.isImportDeclaration).map((statement) => ({ module: statement.moduleSpecifier.text, names: statement.importClause?.namedBindings?.elements?.map((element) => element.name.text) ?? [] }));
const staging = imports.find((entry) => entry.names.includes("ensureMcpBinary"));
assert.ok(staging, "setup must import the MCP staging operation");
assert.match(staging.module, /🌉️mcp\/📦️packages\/🟦️typescript\/📦️binary\/🟦️\.ts$/u);
const entry = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📦️binary/🟦️.ts";
const publicSource = ts.createSourceFile(entry, readFileSync(entry, "utf8"), ts.ScriptTarget.Latest, true);
const exports = publicSource.statements.filter(ts.isExportDeclaration);
assert.equal(exports.length, 1);
assert.equal(exports[0].moduleSpecifier.text, "../../../📦️binary/🟦️.ts");
assert.equal(exports[0].exportClause, undefined);
console.log("MCP staging boundary is narrow");
