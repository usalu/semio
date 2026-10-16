import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
const [ownersPath, needle, ...files] = process.argv.slice(2).filter((a) => a !== "--write");
const WRITE = process.argv.includes("--write");
const owners = JSON.parse(fs.readFileSync(ownersPath, "utf8"));
let total = 0;
for (const file of files) {
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  for (const s of sf.statements) {
    const isImp = ts.isImportDeclaration(s), isExp = ts.isExportDeclaration(s);
    if (!(isImp || isExp) || !s.moduleSpecifier || !ts.isStringLiteral(s.moduleSpecifier) || !s.moduleSpecifier.text.endsWith(needle)) continue;
    const named = isImp ? s.importClause?.namedBindings : s.exportClause;
    if (!named || !(ts.isNamedImports(named) || ts.isNamedExports(named))) { console.log("SKIP non-named", file); continue; }
    if (isImp && s.importClause.name) { console.log("SKIP default", file); continue; }
    const typeOnly = isImp ? s.importClause.isTypeOnly : s.isTypeOnly;
    const spec = s.moduleSpecifier.text;
    const groups = new Map();
    for (const e of named.elements) {
      const name = (e.propertyName ?? e.name).text;
      const owner = owners[name];
      if (owner === undefined) { console.log(`UNKNOWN ${name} in ${file}`); process.exit(1); }
      (groups.get(owner) ?? groups.set(owner, []).get(owner)).push(e.getText());
    }
    const out = [];
    for (const [owner, els] of groups) {
      const from = owner === "parent" ? spec : spec.replace(/🟦️\.ts$/, `${owner}/🟦️.ts`);
      const kw = isImp ? "import" : "export";
      const one = `${kw} ${typeOnly ? "type " : ""}{ ${els.join(", ")} } from "${from}";`;
      out.push(one.length <= 220 ? one : `${kw} ${typeOnly ? "type " : ""}{\n${els.map((x) => "  " + x + ",").join("\n")}\n} from "${from}";`);
    }
    edits.push([s.getStart(), s.end, out.join("\n")]);
  }
  if (!edits.length) continue;
  let t = text;
  for (const [a, b, r] of edits.sort((x, y) => y[0] - x[0])) t = t.slice(0, a) + r + t.slice(b);
  total += edits.length;
  console.log(`${edits.length} edit(s) ${file.split("/").slice(-3).join("/")}`);
  if (WRITE) fs.writeFileSync(file, t);
}
console.log("total", total);
