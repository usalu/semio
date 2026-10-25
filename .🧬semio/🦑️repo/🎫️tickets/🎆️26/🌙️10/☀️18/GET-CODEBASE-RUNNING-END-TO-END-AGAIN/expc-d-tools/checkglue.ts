import ts from "typescript";
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const plans = process.argv.slice(2).flatMap(f => JSON.parse(readFileSync(f, "utf8")).plans as any[]);
const globs = new Set<string>(plans.map(p => p.glue));
let bad = 0;
for (const g of globs) {
  const abs = resolve(root, g);
  if (!existsSync(abs) || !/\.(ts|tsx|mts)$/u.test(g)) continue;
  const text = readFileSync(abs, "utf8");
  const sf = ts.createSourceFile(g, text, ts.ScriptTarget.Latest, true);
  const diags = (sf as any).parseDiagnostics as ts.Diagnostic[];
  const modules = new Set<string>();
  for (const st of sf.statements) if (ts.isExportDeclaration(st) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier)) modules.add(st.moduleSpecifier.text);
  const dup = new Set<string>(), seen = new Set<string>();
  for (const st of sf.statements) if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) { if (seen.has(e.name.text)) dup.add(e.name.text); seen.add(e.name.text); }
  if (diags.length || modules.size > 4 || dup.size) { bad++; console.log(g.slice(-90), "syntax", diags.length, "modules", modules.size, "dup", [...dup].join(",")); }
}
console.log(globs.size, "glue files checked,", bad, "suspicious");
