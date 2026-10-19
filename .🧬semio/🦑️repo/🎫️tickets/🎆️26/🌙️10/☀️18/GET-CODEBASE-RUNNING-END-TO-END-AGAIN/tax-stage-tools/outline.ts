// 🔭 Prints the top-level statement outline of one script. usage: bun outline.ts <file>
import {readFileSync} from "node:fs";
import ts from "typescript";
const p = process.argv[2]!, src = readFileSync(p, "utf8"), f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
for (const s of f.statements) {
  const t = s.getText().replace(/\s+/g, " ");
  const kind = ts.SyntaxKind[s.kind].replace("Declaration", "").replace("Statement", "");
  if (ts.isImportDeclaration(s)) { console.log(`I ${t.slice(0, 160)}`); continue; }
  console.log(`${kind.padEnd(10)} [${s.getEnd() - s.getStart()}] ${t.slice(0, 230)}`);
}
