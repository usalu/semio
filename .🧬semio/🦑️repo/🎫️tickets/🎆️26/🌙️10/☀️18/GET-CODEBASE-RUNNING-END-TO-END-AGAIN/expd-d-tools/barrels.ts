import { existsSync, readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const dir = process.argv[2]!;
const glueModules = (glue: string): Set<string> => {
  const out = new Set<string>(); if (!existsSync(join(root, glue))) return out;
  const sf = ts.createSourceFile(glue, readFileSync(join(root, glue), "utf8"), ts.ScriptTarget.Latest, true);
  for (const st of sf.statements) { if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) return new Set(); if (st.moduleSpecifier.text.startsWith(".")) out.add(normalize(join(dirname(glue), st.moduleSpecifier.text))); }
  return out;
};
const rows: any[] = [];
const seen = new Set<string>();
for (const n of ["edges0.plan-applied.json", "e1.plan-applied.json"]) {
  const d = JSON.parse(readFileSync(join(dir, n), "utf8"));
  for (const p of d.plans) { const key = p.glue; if (seen.has(key)) continue; seen.add(key); const mods = glueModules(p.glue); rows.push({ glue: p.glue, pkg: p.pkg, sub: p.sub, reuse: p.reuse, extend: !!p.extend, modules: [...mods] }); }
}
const barrels = rows.filter((r) => r.modules.length > 1);
console.log("glue", rows.length, "barrels", barrels.length);
for (const b of barrels) console.log(b.reuse ? "R" : b.extend ? "E" : "N", b.pkg.replace("@semio-tech/", ""), b.sub, b.modules.length);
require("fs").writeFileSync(join(dir, "barrels.json"), JSON.stringify(barrels, null, 1));
