import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const filter = new RegExp(process.argv[3] ?? ".");
const prodOnly = process.argv[4] !== "all";
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p);
const seen = new Set();
const cache = new Map();
for (const r of rows) {
  if (!filter.test(r.to) || r.to.startsWith(".cache/")) continue;
  if (prodOnly && isTest(r.from)) continue;
  const k = r.from + "|" + r.spec; if (seen.has(k)) continue; seen.add(k);
  const abs = "/Users/ueli/Documents/semio/" + r.from;
  let sf = cache.get(abs);
  if (!sf) { try { sf = ts.createSourceFile(abs, fs.readFileSync(abs, "utf8"), ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS); } catch { continue; } cache.set(abs, sf); }
  const names = [];
  const visit = n => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier && n.moduleSpecifier.text === r.spec) {
      if (ts.isImportDeclaration(n)) {
        const c = n.importClause; const tl = c?.isTypeOnly ? "type " : "";
        if (!c) names.push("<side-effect>");
        else { if (c.name) names.push(tl + "default:" + c.name.text); const nb = c.namedBindings; if (nb) { if (ts.isNamespaceImport(nb)) names.push(tl + "*:" + nb.name.text); else nb.elements.forEach(e => names.push((tl || (e.isTypeOnly ? "type " : "")) + (e.propertyName ? e.propertyName.text : e.name.text))); } }
      } else names.push(n.exportClause ? "export{" + (n.exportClause.elements ?? []).map(e => (e.propertyName ?? e.name).text).join(",") + "}" : "export*");
    }
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0]?.text === r.spec) names.push("<dynamic import>");
    if (ts.isImportTypeNode(n) && n.argument.literal?.text === r.spec) names.push("<import type()>");
    ts.forEachChild(n, visit);
  };
  visit(sf);
  console.log(r.from.replace(/^.*?engine\//u, "E/") + "\n   -> " + r.to.replace("🧰️framework/🛍️products/💻️os/", "OS/").replace("🧰️framework/", "F/") + "\n      " + names.join(", "));
}
