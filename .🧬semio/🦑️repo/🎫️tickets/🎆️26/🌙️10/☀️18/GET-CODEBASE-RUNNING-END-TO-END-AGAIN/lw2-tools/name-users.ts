import ts from "typescript";
import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const NAMES = new Set(JSON.parse(process.env.NAMES!) as string[]);
const roots = (process.env.ROOTS ?? "🧰️framework,✏️s,🌎️hub,♻️mit-bestand,.storybook").split(",");
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated", ".git", "📤️dist"]);
const files: string[] = [];
const walk = (d: string) => { let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; } for (const e of es) { if (SKIP.has(e.name)) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(ts|tsx|mts)$/u.test(e.name) && !e.name.endsWith(".d.ts")) files.push(p); } };
for (const r of roots) walk(join(ROOT, r));
const rows: string[] = [];
for (const f of files) {
  let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
  let hit = false; for (const n of NAMES) if (t.includes(n)) { hit = true; break; } if (!hit) continue;
  const sf = ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  for (const st of sf.statements) {
    if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier)) {
      const els = ts.isImportDeclaration(st) ? (st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) ? st.importClause.namedBindings.elements : []) : (st.exportClause && ts.isNamedExports(st.exportClause) ? st.exportClause.elements : []);
      const names = els.map(e => (e.propertyName ?? e.name).text).filter(n => NAMES.has(n));
      const star = ts.isExportDeclaration(st) && !st.exportClause;
      if (names.length || star && /WasmSessionLoader/.test(st.moduleSpecifier.text)) rows.push(`${relative(ROOT, f)} | ${ts.isImportDeclaration(st) ? "import" : "export"} ${star ? "*" : names.join(",")} from ${st.moduleSpecifier.text}`);
    }
  }
}
console.log(rows.length); for (const r of rows) console.log(r.replace(/🧰️framework\/🛍️products\/💻️os\/🔨️modules\/📺️renderer\/🧑‍🎨engine\//u, "R/").slice(0, 300));
