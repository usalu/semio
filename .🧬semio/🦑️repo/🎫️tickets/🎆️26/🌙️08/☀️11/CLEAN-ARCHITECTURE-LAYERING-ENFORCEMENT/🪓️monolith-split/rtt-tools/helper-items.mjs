import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";
const R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const sets = [
  { helper: `${R}/🧪️tests/🧰️engine-contract-helpers/🟦️.ts`, dir: `${R}/🧪️tests`, label: "engine-contract" },
  { helper: `${R}/🧪️tests/🛠️plugin-runtime-helpers/🟦️.tsx`, dir: `${R}/🧪️tests`, label: "plugin-runtime" },
  { helper: `${R}/🧱️elements/📃️UiDocumentStore/🧪️tests/🪛️typedwire-helpers/🟦️.tsx`, dir: `${R}/🧱️elements/📃️UiDocumentStore/🧪️tests`, label: "typedwire" },
];
for (const s of sets) {
  const text = fs.readFileSync(s.helper, "utf8");
  const sf = ts.createSourceFile(s.helper, text, ts.ScriptTarget.Latest, true, s.helper.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const exports = [];
  const locals = [];
  for (const st of sf.statements) {
    if (ts.isImportDeclaration(st)) continue;
    const exp = st.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    let names = [];
    if (ts.isVariableStatement(st)) {
      const w = (b) => { if (ts.isIdentifier(b)) names.push(b.text); else for (const e of b.elements) if (ts.isBindingElement(e)) w(e.name); };
      for (const d of st.declarationList.declarations) w(d.name);
    } else if (st.name && ts.isIdentifier(st.name)) names = [st.name.text];
    else names = ["<" + ts.SyntaxKind[st.kind] + ">"];
    (exp ? exports : locals).push(...names);
  }
  const users = new Map();
  const dirs = fs.readdirSync(s.dir);
  for (const d of dirs) {
    for (const ext of ["🟦️.ts", "🟦️.tsx"]) {
      const f = path.join(s.dir, d, ext);
      if (!fs.existsSync(f) || f === s.helper) continue;
      const t = fs.readFileSync(f, "utf8");
      if (!t.includes(path.basename(path.dirname(s.helper)))) continue;
      users.set(d, t);
    }
  }
  console.log(`## ${s.label}: ${path.basename(path.dirname(s.helper))}`);
  console.log(`exports (${exports.length}): ${exports.join(", ")}`);
  console.log(`module-private (${locals.length}): ${locals.join(", ")}`);
  console.log(`importers (${users.size}): ${[...users.keys()].join(", ")}`);
}
