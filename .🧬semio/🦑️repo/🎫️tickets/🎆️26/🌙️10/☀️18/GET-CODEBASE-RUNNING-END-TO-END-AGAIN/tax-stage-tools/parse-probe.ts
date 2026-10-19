// 🔬 Finds the top-level statements the restricted ECMA subset parser cannot parse. usage: bun parse-probe.ts <file>
import {readFileSync} from "node:fs";
import ts from "typescript";
import {ecmaProgram} from "@semio-tech/compiler-syntax-ecma-ts";
const p = process.argv[2]!, src = readFileSync(p, "utf8"), f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
console.log("whole file parses:", ecmaProgram(src) !== null);
for (const s of f.statements) {
  const text = src.slice(s.getFullStart(), s.end);
  if (ecmaProgram(text) === null) console.log("UNPARSEABLE", ts.SyntaxKind[s.kind], s.getText().replace(/\s+/g, " ").slice(0, 200));
}
