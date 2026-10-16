// usage: node reimport.mjs <map.json> <importers.txt> [APPLY]
// Re-points import/export declarations of the moved names from the split file to the child that owns each name.
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import path from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const [mapPath, listPath, mode] = process.argv.slice(2);
const APPLY = mode === "APPLY";
const map = JSON.parse(fs.readFileSync(mapPath, "utf8"));
const oldFile = path.resolve(REPO, map.file);
const oldNoExt = oldFile.replace(/\.tsx?$/, "");
const importers = fs.readFileSync(listPath, "utf8").split("\n").map((l) => l.trim()).filter(Boolean);
const relFrom = (base, target) => { let r = path.relative(base, target).split(path.sep).join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const allFiles = [...new Set([...Object.values(map.names), ...Object.values(map.children)])].map((p) => path.resolve(REPO, p));
const parentExports = new Set();
const resolves = (importerFile, spec) => { if (!spec.startsWith(".")) return false; const abs = path.resolve(path.dirname(importerFile), spec); return abs === oldFile || abs === oldNoExt; };

const summary = [];
const manual = [];
for (const rel of importers) {
  const file = path.resolve(REPO, rel);
  if (file === oldFile) continue;
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  const visitDyn = (n) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && resolves(file, n.arguments[0].text)) manual.push(`${rel}: dynamic import at line ${sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1}`);
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && resolves(file, n.argument.literal.text)) manual.push(`${rel}: import() type at line ${sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1}`);
    ts.forEachChild(n, visitDyn);
  };
  visitDyn(sf);
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) continue;
    if (!resolves(file, st.moduleSpecifier.text)) continue;
    const specText = st.moduleSpecifier.text;
    const ext = specText.endsWith(".ts") ? ".ts" : specText.endsWith(".tsx") ? ".tsx" : "";
    const q = st.moduleSpecifier.getText(sf)[0];
    const target = (abs) => { let r = relFrom(path.dirname(file), abs); if (!ext) r = r.replace(/\.tsx?$/, ""); return r; };
    if (ts.isExportDeclaration(st) && !st.exportClause) {
      const head = st.isTypeOnly ? "export type * from" : "export * from";
      const files = [...allFiles].sort();
      const parentFirst = [oldFile, ...files.filter((f) => f !== oldFile)];
      edits.push({ start: st.getStart(sf), end: st.getEnd(), text: parentFirst.map((f) => `${head} ${q}${target(f)}${q};`).join("\n") });
      continue;
    }
    if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamespaceExport(st.exportClause)) { manual.push(`${rel}: export * as`); continue; }
    const clause = ts.isImportDeclaration(st) ? st.importClause?.namedBindings : st.exportClause;
    if (ts.isImportDeclaration(st) && (!st.importClause || st.importClause.name || (clause && ts.isNamespaceImport(clause)))) { manual.push(`${rel}: default/namespace/side-effect import`); continue; }
    if (!clause || !(ts.isNamedImports(clause) || ts.isNamedExports(clause))) { manual.push(`${rel}: unsupported form`); continue; }
    const groups = new Map();
    for (const el of clause.elements) {
      const orig = (el.propertyName ?? el.name).text;
      const dest = map.names[orig];
      if (!dest) { manual.push(`${rel}: name ${orig} is not declared in the split file`); groups.clear(); break; }
      const abs = path.resolve(REPO, dest);
      if (!groups.has(abs)) groups.set(abs, []);
      groups.get(abs).push(el.getText(sf));
    }
    if (!groups.size) continue;
    const typeOnly = ts.isImportDeclaration(st) ? st.importClause.isTypeOnly : st.isTypeOnly;
    const head = ts.isImportDeclaration(st) ? `import ${typeOnly ? "type " : ""}` : `export ${typeOnly ? "type " : ""}`;
    const order = [...groups.keys()].sort((a, b) => (a === oldFile ? -1 : b === oldFile ? 1 : a.localeCompare(b)));
    const out = order.map((abs) => `${head}{ ${groups.get(abs).join(", ")} } from ${q}${target(abs)}${q};`).join("\n");
    edits.push({ start: st.getStart(sf), end: st.getEnd(), text: out });
  }
  if (!edits.length) continue;
  let r = text;
  for (const e of edits.sort((a, b) => b.start - a.start)) r = r.slice(0, e.start) + e.text + r.slice(e.end);
  summary.push(`${rel}\t${edits.length} declaration(s)`);
  if (APPLY) fs.writeFileSync(file, r);
}
console.log(summary.join("\n"));
console.log(`importers changed: ${summary.length}`);
if (manual.length) console.log("MANUAL:\n  " + [...new Set(manual)].join("\n  "));
void parentExports;
