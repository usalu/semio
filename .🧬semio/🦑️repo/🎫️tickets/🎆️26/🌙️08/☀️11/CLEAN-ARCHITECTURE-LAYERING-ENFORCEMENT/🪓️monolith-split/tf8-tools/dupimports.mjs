import { readFileSync } from "node:fs";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
for (const f of process.argv.slice(2)) {
  const sf = ts.createSourceFile(f, readFileSync(f, "utf8"), ts.ScriptTarget.Latest, true);
  const seen = new Map();
  for (const n of sf.statements) if (ts.isImportDeclaration(n)) { const k = n.moduleSpecifier.text + (n.importClause?.isTypeOnly ? "#t" : ""); seen.set(k, (seen.get(k) ?? 0) + 1); }
  for (const [k, c] of seen) if (c > 1) console.log("DUP", c, k.slice(-90), f.slice(-80));
}
