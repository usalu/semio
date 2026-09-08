import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
import ts from "typescript";
const here = dirname(fileURLToPath(import.meta.url)), require = createRequire(import.meta.url);
const source = ts.createSourceFile("root.ts", readFileSync("📜️script.ts", "utf8"), ts.ScriptTarget.Latest, true);
const declaration = source.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "NxScript")!;
const code = ts.transpileModule(declaration.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const NxScript = new Function("Script", code + "; return NxScript;")(class {});
const { groupBy, sortBy } = require("lodash");
for (const row of JSON.parse(readFileSync(join(here, "🧫️cases.json"), "utf8"))) {
  const children = groupBy(row.rows, "parent"), normalized = (value: string) => value.replaceAll("\\", "/").toLowerCase();
  const visit = (pid: number): number[] => (children[pid] ?? []).flatMap((child: any) => normalized(child.command).includes(normalized(row.daemon)) ? [] : [child.pid, ...visit(child.pid)]);
  const oracle = sortBy([...new Set(row.roots.flatMap(visit))]);
  assert.deepEqual(oracle, row.expected);
  assert.deepEqual(NxScript.ownedDescendants(row.roots, row.rows, row.daemon), oracle, row.name);
}
console.log("[DEBUG] Owned process closure: POSIX, Windows, detached tasks and shared daemon exclusion PASS");
