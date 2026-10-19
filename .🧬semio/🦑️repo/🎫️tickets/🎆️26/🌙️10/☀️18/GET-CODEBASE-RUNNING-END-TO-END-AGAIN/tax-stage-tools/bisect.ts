// 🔬 Finds which top-level statements make the command-router validator refuse a script (statement removed alone, then statement kept alone with the routing imports). usage: bun bisect.ts <file>
import {readFileSync} from "node:fs";
import ts from "typescript";
import {accepts} from "./lib.ts";
const p = process.argv[2]!, src = readFileSync(p, "utf8"), f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
console.log("accepted:", accepts(src));
const st = [...f.statements];
for (let i = 0; i < st.length; i++) {
  if (ts.isImportDeclaration(st[i]!)) continue;
  const without = st.filter((_, j) => j !== i).map((s) => src.slice(s.getFullStart(), s.end)).join("");
  if (accepts(without)) console.log(`REMOVING fixes: [${i}] ${ts.SyntaxKind[st[i]!.kind]} ${st[i]!.getText().replace(/\s+/g, " ").slice(0, 160)}`);
}
