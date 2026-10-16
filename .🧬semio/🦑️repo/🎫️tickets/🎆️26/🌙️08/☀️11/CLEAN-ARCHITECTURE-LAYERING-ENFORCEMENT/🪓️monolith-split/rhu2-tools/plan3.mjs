import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const [edgesFile, originsFile, outFile] = process.argv.slice(2);
const rows = fs.readFileSync(edgesFile, "utf8").trim().split("\n").filter(Boolean).map(l => JSON.parse(l));
const org = JSON.parse(fs.readFileSync(originsFile, "utf8"));
const files = new Map(); const plan = new Map(); const unknown = [];
for (const r of rows) {
  const abs = R + r.from;
  let sf = files.get(abs);
  if (!sf) { sf = ts.createSourceFile(abs, fs.readFileSync(abs, "utf8"), ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); files.set(abs, sf); }
  const add = (n, isType) => {
    if (n === "default" || n === "*") return;
    const decl = (org[r.to] ?? {})[n]; if (decl === undefined) { unknown.push(`${r.from.slice(-50)} ${n}`); return; }
    const df = decl.split("#")[0];
    const e = plan.get(df) ?? { to: df, files: new Set(), names: new Map(), dynamic: [] }; plan.set(df, e); e.files.add(r.from);
    const prev = e.names.get(n); e.names.set(n, prev === undefined ? isType : prev && isType);
  };
  const visit = n => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier?.text === r.spec) {
      if (ts.isImportDeclaration(n)) { const c = n.importClause; if (c) { const nb = c.namedBindings; if (nb && ts.isNamedImports(nb)) nb.elements.forEach(e => add((e.propertyName ?? e.name).text, !!c.isTypeOnly || e.isTypeOnly)); } }
      else if (n.exportClause && ts.isNamedExports(n.exportClause)) n.exportClause.elements.forEach(e => add((e.propertyName ?? e.name).text, !!n.isTypeOnly || e.isTypeOnly));
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
const out = [...plan.values()].map(e => ({ to: e.to, files: [...e.files], names: [...e.names].map(([name, type]) => ({ name, type })), dynamic: [] }));
fs.writeFileSync(outFile, JSON.stringify(out, null, 1));
for (const o of out) console.error(o.to.replace("🧰️framework/🛍️products/💻️os/", "OS/").replace("🧰️framework/🔨️modules/", "F/"), "files", o.files.length, "names", o.names.length);
console.error("unknown", unknown.length, unknown.slice(0, 5));
