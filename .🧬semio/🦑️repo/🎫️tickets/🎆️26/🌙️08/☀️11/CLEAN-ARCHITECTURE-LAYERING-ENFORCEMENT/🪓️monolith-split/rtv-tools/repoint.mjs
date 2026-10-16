import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const [parent, cfgPath, infoPath, ...files] = process.argv.slice(2);
const cfg = (await import(cfgPath)).default; const info = JSON.parse(readFileSync(infoPath, "utf8"));
const exportsByName = new Map(info.exports.map((e) => [e.name, e.module]));
const defs = new Map(cfg.modules.map((m) => [m.name, m]));
const norm = (p) => p.replace(/\.(ts|tsx)$/u, "");
const target = process.env.RTV_TARGET ?? parent;
const parentFile = process.env.RTV_PARENT_FILE ?? parent;
const modPath = (mod, def) => (def.isParent ? parentFile : join(dirname(parentFile), def.folder, "🟦️" + (cfg.ext ?? ".ts")));
for (const f of files) {
  const src = readFileSync(f, "utf8");
  const sf = ts.createSourceFile(f, src, ts.ScriptTarget.Latest, true);
  const edits = [];
  for (const n of sf.statements) {
    if (!(ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) || !n.moduleSpecifier || !ts.isStringLiteral(n.moduleSpecifier)) continue;
    const spec = n.moduleSpecifier.text; if (!spec.startsWith(".") || norm(resolve(dirname(f), spec)) !== norm(target)) continue;
    const clause = ts.isImportDeclaration(n) ? n.importClause : n.exportClause;
    const els = ts.isImportDeclaration(n) ? clause?.namedBindings?.elements : clause?.elements;
    if (!els) throw new Error(`${f}: unsupported form ${n.getText().slice(0, 80)}`);
    const per = new Map();
    for (const el of els) { const name = (el.propertyName ?? el.name).text; const mod = exportsByName.get(name); if (!mod) throw new Error(`${f}: ${name} is not a public name of the parent`); if (!per.has(mod)) per.set(mod, []); per.get(mod).push(el.getText(sf)); }
    const kw = ts.isImportDeclaration(n) ? "import" : "export"; const typeKw = (ts.isImportDeclaration(n) ? clause.isTypeOnly : clause && n.isTypeOnly) ? "type " : "";
    const text = [...per].map(([mod, names]) => { const abs = modPath(mod, defs.get(mod)); let rel = relative(dirname(f), abs); if (!rel.startsWith(".")) rel = "./" + rel; return `${kw} ${typeKw}{ ${names.join(", ")} } from ${JSON.stringify(rel)};`; }).join("\n");
    edits.push({ start: n.getStart(), end: n.end, text });
  }
  if (!edits.length) { console.log("no import of parent in", f); continue; }
  let out = src; for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  if (process.env.RTV_WRITE === "1") writeFileSync(f, out);
  console.log(f, edits.length, "edit(s)");
}
