/** 🔭 Which names does the framework root barrel take from the ui tree, and which files import them from the root. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";
import { scan } from "../exports-stage-tools/scan.ts";
import { originsOf, resolveRel, parseFile } from "../exports-stage-tools/model.ts";
import { ROOT } from "./lib.ts";
const BARREL = "🧰️framework/📦️packages/🟦️typescript/🟦️.ts";
const sf = parseFile(BARREL);
const uiNames = new Map<string, string>();
for (const st of sf.statements) {
  if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) continue;
  const target = resolveRel(BARREL, st.moduleSpecifier.text);
  if (!target || !target.startsWith("🧰️framework/🔨️modules/🖱️ui/")) continue;
  if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) uiNames.set(e.name.text, target);
  else { const o = originsOf(target); for (const k of o?.keys() ?? []) if (!k.startsWith("*bare:") && k !== "default") uiNames.set(k, target); }
}
console.log("ui names in the barrel", uiNames.size);
const { bare } = scan();
const files = [...new Set(bare.filter(b => b.pkg === "@semio-tech/framework" && b.sub === ".").map(b => b.file))];
console.log("files importing the framework root", files.length);
let hit = 0; const perFile: [string, string[]][] = [];
for (const f of files) {
  let text: string; try { text = readFileSync(resolve(ROOT, f), "utf8"); } catch { continue; }
  const s2 = ts.createSourceFile(f, text, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const used: string[] = [];
  for (const st of s2.statements) {
    if (ts.isImportDeclaration(st) && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text === "@semio-tech/framework" && st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings))
      for (const e of st.importClause.namedBindings.elements) { const n = (e.propertyName ?? e.name).text; if (uiNames.has(n)) used.push(n); }
    if (ts.isExportDeclaration(st) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text === "@semio-tech/framework" && st.exportClause && ts.isNamedExports(st.exportClause))
      for (const e of st.exportClause.elements) { const n = (e.propertyName ?? e.name).text; if (uiNames.has(n)) used.push(n); }
  }
  if (used.length) { hit++; perFile.push([f, used]); }
}
console.log("files that import ui names from the root barrel", hit);
const freq = new Map<string, number>(); for (const [, u] of perFile) for (const n of u) freq.set(n, (freq.get(n) ?? 0) + 1);
console.log([...freq].sort((a, b) => b[1] - a[1]).slice(0, 25).map(([n, c]) => `${c} ${n}`).join("\n"));
const star = files.filter(f => { try { return /import\s*\*\s*as\s+\w+\s+from\s*["']@semio-tech\/framework["']/.test(readFileSync(resolve(ROOT, f), "utf8")); } catch { return false; } });
console.log("namespace importers of root", star.length);
