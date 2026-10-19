// 🔬 Prints a structural signature for each script in a list file. usage: bun shape.ts <list.txt> <out.json>
import {readFileSync, writeFileSync} from "node:fs";
import ts from "typescript";
const paths = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const rows: any[] = [];
for (const p of paths) {
  const src = readFileSync(p, "utf8");
  const f = ts.createSourceFile(p, src, ts.ScriptTarget.Latest, true);
  const kinds = new Map<string, number>();
  let exp = 0, cls = 0, fn = 0, decl = 0;
  for (const s of f.statements) {
    const k = ts.SyntaxKind[s.kind]; kinds.set(k, (kinds.get(k) ?? 0) + 1);
    if (ts.canHaveModifiers(s) && ts.getModifiers(s)?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword)) exp++;
    if (ts.isExportDeclaration(s) || ts.isExportAssignment(s)) exp++;
    if (ts.isClassDeclaration(s)) cls++;
    if (ts.isFunctionDeclaration(s)) fn++;
  }
  const lines = src.split("\n").length;
  rows.push({p, lines, kinds: Object.fromEntries(kinds), exp, cls, fn, bundle: /BundleScript|ScriptRouter/.test(src), main: /import\.meta\.main/.test(src), argv: /process\.argv/.test(src), diag: f.parseDiagnostics?.length ?? 0});
}
writeFileSync(process.argv[3]!, JSON.stringify(rows, null, 1));
const sig = new Map<string, number>();
for (const r of rows) { const k = [r.bundle ? "router-ish" : "plain", r.exp ? "exports" : "noexp", r.main ? "main" : "nomain", r.argv ? "argv" : "noargv"].join(" "); sig.set(k, (sig.get(k) ?? 0) + 1); }
console.log([...sig].sort((a, b) => b[1] - a[1]));
