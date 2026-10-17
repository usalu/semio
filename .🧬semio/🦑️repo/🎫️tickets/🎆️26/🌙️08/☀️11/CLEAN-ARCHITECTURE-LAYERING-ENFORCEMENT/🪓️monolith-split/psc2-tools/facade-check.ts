/** 🧩️ Checks that every re-export of every planned facade resolves to an existing file and that every new `exports` target exists. Usage: bun facade-check.ts <plan.json>... */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
let bad = 0, files = 0, stmts = 0;
for (const planPath of process.argv.slice(2)) {
  const plan = JSON.parse(readFileSync(planPath, "utf8"));
  for (const f of plan.facades) {
    files++;
    if (!existsSync(join(ROOT, f.facade))) { bad++; console.log("MISSING facade", f.facade); continue; }
    const text = readFileSync(join(ROOT, f.facade), "utf8");
    const sf = ts.createSourceFile(f.facade, text, ts.ScriptTarget.ESNext, true);
    for (const st of sf.statements) {
      const spec = (ts.isExportDeclaration(st) || ts.isImportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? st.moduleSpecifier.text : null;
      if (!spec || !spec.startsWith(".")) continue;
      stmts++;
      if (!existsSync(resolve(join(ROOT, dirname(f.facade)), spec))) { bad++; console.log("BROKEN", f.facade.slice(-110), "->", spec); }
    }
    const pj = JSON.parse(readFileSync(join(ROOT, f.pkgDir, "package.json"), "utf8"));
    const target = pj.exports?.[f.subpath];
    if (!target || !existsSync(join(ROOT, f.pkgDir, target))) { bad++; console.log("EXPORT", f.pkg, f.subpath, JSON.stringify(target)); }
  }
  for (const d of plan.directExports) {
    const pj = JSON.parse(readFileSync(join(ROOT, d.pkgDir, "package.json"), "utf8"));
    const target = pj.exports?.[d.subpath];
    if (!target || !existsSync(join(ROOT, d.pkgDir, target))) { bad++; console.log("EXPORT", d.pkg, d.subpath, JSON.stringify(target)); }
  }
}
console.log({ facades: files, statements: stmts, bad });
process.exit(bad ? 1 : 0);
