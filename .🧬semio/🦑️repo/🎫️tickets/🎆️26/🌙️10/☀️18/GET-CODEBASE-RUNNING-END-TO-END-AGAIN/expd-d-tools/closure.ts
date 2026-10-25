import { readFileSync } from "node:fs";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const [file, ...names] = process.argv.slice(2) as string[];
const text = readFileSync(file!, "utf8"), sf = ts.createSourceFile(file!, text, ts.ScriptTarget.Latest, true);
type Decl = { names: string[]; node: ts.Statement; start: number; end: number; exported: boolean };
const decls: Decl[] = []; const imports = new Map<string, ts.ImportDeclaration>();
for (const st of sf.statements) {
  const exported = !!ts.getModifiers(st as ts.HasModifiers)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
  if (ts.isImportDeclaration(st)) { const c = st.importClause; if (c?.name) imports.set(c.name.text, st); if (c?.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) imports.set(c.namedBindings.name.text, st); else for (const e of c.namedBindings.elements) imports.set(e.name.text, st); } continue; }
  if (ts.isVariableStatement(st)) decls.push({ names: st.declarationList.declarations.flatMap((d) => ts.isIdentifier(d.name) ? [d.name.text] : d.name.elements.filter((b) => ts.isBindingElement(b) && ts.isIdentifier(b.name)).map((b) => ((b as ts.BindingElement).name as ts.Identifier).text)), node: st, start: st.getFullStart(), end: st.getEnd(), exported });
  else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st)) && st.name) decls.push({ names: [st.name.text], node: st, start: st.getFullStart(), end: st.getEnd(), exported });
  else if ((ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) ) decls.push({ names: [st.name.text], node: st, start: st.getFullStart(), end: st.getEnd(), exported });
  else decls.push({ names: [], node: st, start: st.getFullStart(), end: st.getEnd(), exported });
}
const byName = new Map<string, Decl>(); for (const d of decls) for (const n of d.names) byName.set(n, d);
const refs = (d: Decl): Set<string> => { const out = new Set<string>(); const visit = (n: ts.Node) => { if (ts.isIdentifier(n)) out.add(n.text); ts.forEachChild(n, visit); }; visit(d.node); return out; };
const need = new Set<Decl>(), stack = names.map((n) => byName.get(n)).filter(Boolean) as Decl[];
const missing = names.filter((n) => !byName.has(n)); if (missing.length) console.log("MISSING", missing);
while (stack.length) { const d = stack.pop()!; if (need.has(d)) continue; need.add(d); for (const r of refs(d)) { const dd = byName.get(r); if (dd && !need.has(dd)) stack.push(dd); } }
const usedImports = new Set<ts.ImportDeclaration>(); for (const d of need) for (const r of refs(d)) { const im = imports.get(r); if (im) usedImports.add(im); }
console.log("closure decls", need.size, "of", decls.length, "bytes", [...need].reduce((s, d) => s + d.end - d.start, 0), "imports used", usedImports.size);
for (const d of [...need].sort((a, b) => a.start - b.start)) console.log("  ", d.exported ? "export" : "local ", d.names.join(",") || "(stmt)", d.end - d.start);
