/**
 * 🔤 Names every bare workspace import asks for vs the names the resolved export entry provides (glue completeness).
 *
 *   bun import-names.ts   (writes generated/import-names.json)
 */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, ROOT } from "./lib.ts";
import { scan } from "./scan.ts";
import { entriesOf, entryFile, model, originsOf, abs, rel } from "./model.ts";

const { bare } = scan();
const files = [...new Set(bare.map(b => b.file))];
type Miss = { file: string; spec: string; pkg: string; key: string; entry: string; names: string[]; reason: string };
const misses: Miss[] = [];
const entryCache = new Map<string, string | null>();
const entryOfKey = (pkgName: string, key: string): string | null => {
  const ck = pkgName + key;
  if (entryCache.has(ck)) return entryCache.get(ck)!;
  const p = model.byName.get(pkgName);
  let out: string | null = null;
  if (p) {
    const v = (p.exports as Record<string, unknown>)[key];
    const t = v === undefined ? null : entryFile(v);
    if (t && t.startsWith("./")) out = rel(resolve(abs(p.dir), t));
  }
  entryCache.set(ck, out);
  return out;
};
let checked = 0, stat = { noKey: 0, nonTs: 0 };
for (const file of files) {
  let sf: ts.SourceFile;
  try { sf = ts.createSourceFile(file, readFileSync(resolve(ROOT, file), "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); } catch { continue; }
  const check = (spec: string, names: string[] | "*"): void => {
    const clean = spec.split("?")[0]!;
    if (!clean.startsWith("@semio-tech/")) return;
    const parts = clean.split("/");
    const pkgName = parts.slice(0, 2).join("/");
    const key = parts.length > 2 ? "./" + parts.slice(2).join("/") : ".";
    const entry = entryOfKey(pkgName, key);
    if (!entry) { stat.noKey++; return; }
    if (!/\.(ts|tsx|mts)$/u.test(entry)) { stat.nonTs++; return; }
    if (names === "*" || !names.length) return;
    const o = originsOf(entry);
    if (!o) return;
    checked++;
    const missing = names.filter(n => !o.has(n) && ![...o.keys()].some(k => k.startsWith("*bare:")));
    if (missing.length) misses.push({ file, spec, pkg: pkgName, key, entry, names: missing, reason: "entry does not export the imported names" });
  };
  const visit = (n: ts.Node): void => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause; const names: string[] = [];
      if (c?.name) names.push("default");
      if (c?.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text);
      check(n.moduleSpecifier.text, c?.namedBindings && ts.isNamespaceImport(c.namedBindings) ? "*" : names);
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      check(n.moduleSpecifier.text, n.exportClause && ts.isNamedExports(n.exportClause) ? n.exportClause.elements.map(e => (e.propertyName ?? e.name).text) : "*");
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
writeFileSync(resolve(GENERATED, "import-names.json"), JSON.stringify(misses));
console.log(`files ${files.length} checked imports ${checked} noKey ${stat.noKey} nonTs ${stat.nonTs} | missing-name imports ${misses.length} (entries ${new Set(misses.map(m => m.entry)).size}, files ${new Set(misses.map(m => m.file)).size})`);
const byEntry = new Map<string, { names: Set<string>; files: Set<string>; pkg: string; key: string }>();
for (const m of misses) { const e = byEntry.get(m.entry) ?? byEntry.set(m.entry, { names: new Set(), files: new Set(), pkg: m.pkg, key: m.key }).get(m.entry)!; m.names.forEach(n => e.names.add(n)); e.files.add(m.file); }
for (const [entry, e] of [...byEntry].slice(0, 40)) console.log(`${e.pkg}${e.key.slice(1)} (${e.files.size} files): ${[...e.names].slice(0, 6).join(",")}`);
