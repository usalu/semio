// usage: node repoint.mjs <fromFile> <toFile> <importers.txt> <name,name,...> [APPLY]
// For every import/export declaration that takes one of the names from <fromFile>, splits those names onto a declaration from <toFile>.
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
const REPO = "/Users/ueli/Documents/semio";
const [fromRel, toRel, listPath, namesCsv, mode] = process.argv.slice(2);
const APPLY = mode === "APPLY";
const from = path.resolve(REPO, fromRel), to = path.resolve(REPO, toRel);
const names = new Set(namesCsv.split(","));
const relFrom = (base, target) => { let r = path.relative(base, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const changed = [];
for (const rel of fs.readFileSync(listPath, "utf8").split("\n").filter(Boolean)) {
  const file = path.resolve(REPO, rel);
  if (file === from || file === to || !fs.existsSync(file)) continue;
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) continue;
    if (!st.moduleSpecifier.text.startsWith(".") || path.resolve(path.dirname(file), st.moduleSpecifier.text) !== from) continue;
    const clause = ts.isImportDeclaration(st) ? st.importClause?.namedBindings : st.exportClause;
    if (!clause || !(ts.isNamedImports(clause) || ts.isNamedExports(clause))) continue;
    const moving = clause.elements.filter((e) => names.has((e.propertyName ?? e.name).text));
    if (!moving.length) continue;
    const staying = clause.elements.filter((e) => !moving.includes(e));
    const q = st.moduleSpecifier.getText(sf)[0];
    const head = ts.isImportDeclaration(st) ? "import " + (st.importClause.isTypeOnly ? "type " : "") : "export " + (st.isTypeOnly ? "type " : "");
    const mk = (els, abs) => `${head}{ ${els.map((e) => e.getText(sf)).join(", ")} } from ${q}${relFrom(path.dirname(file), abs)}${q};`;
    let rep = mk(moving, to);
    if (staying.length) rep = mk(staying, from) + "\n" + rep;
    edits.push({ start: st.getStart(sf), end: st.getEnd(), text: rep });
  }
  if (!edits.length) continue;
  let r = text;
  for (const e of edits.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  changed.push(rel);
  if (APPLY) fs.writeFileSync(file, r);
}
console.log(changed.length + " importer(s)", changed.map((c) => c.split("/").slice(-2).join("/")).join(", "));
