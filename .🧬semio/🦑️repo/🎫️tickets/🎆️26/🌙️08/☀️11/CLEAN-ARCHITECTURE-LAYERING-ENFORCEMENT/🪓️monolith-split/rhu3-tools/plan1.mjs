import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const include = new RegExp(process.argv[3]);
const exclude = new RegExp(process.argv[4] ?? "$^");
const per = new Map(); const files = new Map();
for (const r of rows) {
  if (!include.test(r.to) || exclude.test(r.to)) continue;
  const abs = R + r.from;
  let sf = files.get(abs);
  if (!sf) { sf = ts.createSourceFile(abs, fs.readFileSync(abs, "utf8"), ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); files.set(abs, sf); }
  const m = per.get(r.to) ?? { names: new Map(), files: new Set(), dynamic: [] }; per.set(r.to, m); m.files.add(r.from);
  const add = (n, isType) => { const prev = m.names.get(n); m.names.set(n, prev === undefined ? isType : prev && isType); };
  const visit = n => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier?.text === r.spec) {
      if (ts.isImportDeclaration(n)) { const c = n.importClause; if (c) { if (c.name) add("default", !!c.isTypeOnly); const nb = c.namedBindings; if (nb) { if (ts.isNamespaceImport(nb)) add("*", !!c.isTypeOnly); else nb.elements.forEach(e => add((e.propertyName ?? e.name).text, !!c.isTypeOnly || e.isTypeOnly)); } } else m.dynamic.push(r.from + " side-effect"); }
      else if (n.exportClause && ts.isNamedExports(n.exportClause)) n.exportClause.elements.forEach(e => add((e.propertyName ?? e.name).text, !!n.isTypeOnly || e.isTypeOnly)); else m.dynamic.push(r.from + " export*");
    }
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0]?.text === r.spec) m.dynamic.push(r.from + " dynamic-import");
    if (ts.isImportTypeNode(n) && n.argument.literal?.text === r.spec) m.dynamic.push(r.from + " import-type()");
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
const out = [...per].map(([to, m]) => ({ to, files: [...m.files], names: [...m.names].map(([name, type]) => ({ name, type })), dynamic: m.dynamic }));
fs.writeFileSync(process.argv[5] ?? "/dev/stdout", JSON.stringify(out, null, 1));
for (const o of out) console.error(o.to.replace("🧰️framework/🔨️modules/", "F/"), "files", o.files.length, "names", o.names.length, "dyn", o.dynamic.length);
