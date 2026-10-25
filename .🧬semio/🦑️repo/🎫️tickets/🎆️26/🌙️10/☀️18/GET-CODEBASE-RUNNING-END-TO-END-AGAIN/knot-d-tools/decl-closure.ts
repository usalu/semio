import { readFileSync } from "node:fs";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const [file, ...seeds] = process.argv.slice(2) as string[];
const text = readFileSync(file!, "utf8"), sf = ts.createSourceFile(file!, text, ts.ScriptTarget.Latest, true);
const decl = new Map<string, ts.Statement>();
const names = (st: ts.Statement): string[] => {
  if (ts.isVariableStatement(st)) return st.declarationList.declarations.flatMap((d) => ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements.flatMap((e: any) => ts.isIdentifier(e.name) ? [e.name.text] : []));
  if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) return [st.name.text];
  return [];
};
for (const st of sf.statements) for (const n of names(st)) decl.set(n, st);
const imports = new Map<string, ts.ImportDeclaration>();
for (const st of sf.statements) if (ts.isImportDeclaration(st)) { const c = st.importClause; if (c?.name) imports.set(c.name.text, st); if (c?.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) imports.set(c.namedBindings.name.text, st); else for (const e of c.namedBindings.elements) imports.set(e.name.text, st); } }
const seen = new Set<ts.Statement>(), order: string[] = [], usedImports = new Set<string>(), queue = [...seeds];
while (queue.length) {
  const n = queue.pop()!; const st = decl.get(n);
  if (!st) { if (imports.has(n)) usedImports.add(n); else console.log("UNKNOWN", n); continue; }
  if (seen.has(st)) continue; seen.add(st); order.push(names(st).join(",") + "@" + (sf.getLineAndCharacterOfPosition(st.getStart()).line + 1) + ":" + (st.getEnd() - st.getStart()));
  const visit = (x: ts.Node) => { if (ts.isIdentifier(x)) queue.push(x.text); ts.forEachChild(x, visit); }; visit(st);
}
let bytes = 0; for (const st of seen) bytes += st.getEnd() - st.getStart();
console.log("decls", seen.size, "bytes", bytes, "of", text.length); console.log(order.join("\n")); console.log("imports used:", [...usedImports].length);
