import fs from "node:fs";
import path from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";

const [mode, oldFileArg, newFilesArg, importersTsv, ...rest] = process.argv.slice(2);
const oldFile = path.resolve(oldFileArg);
const newFiles = fs.readFileSync(newFilesArg, "utf8").split("\n").filter(Boolean).map((f) => path.resolve(f));
const skip = new Set(rest.filter((r) => r.startsWith("--skip=")).map((r) => path.resolve(r.slice(7))));
const R = "/Users/ueli/Documents/semio";

const exportsOf = (file) => {
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TSX);
  const out = new Map();
  const bind = (b, o) => { if (ts.isIdentifier(b)) o.push(b.text); else for (const el of b.elements) if (!ts.isOmittedExpression(el)) bind(el.name, o); };
  for (const st of sf.statements) {
    const mods = ts.canHaveModifiers(st) ? ts.getModifiers(st) : undefined;
    const exp = mods?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    if (exp) {
      const names = [];
      if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) bind(d.name, names);
      else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isEnumDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) && st.name) names.push(st.name.text);
      const isType = ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st);
      for (const n of names) out.set(n, isType ? "type" : "value");
    }
    if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause)) for (const el of st.exportClause.elements) out.set(el.name.text, el.isTypeOnly || st.isTypeOnly ? "type" : "value");
  }
  return out;
};
const home = new Map();
for (const f of newFiles) for (const n of exportsOf(f).keys()) { if (home.has(n) && home.get(n) !== f) throw new Error(`name exported twice: ${n}`); home.set(n, f); }
const mapArg = rest.find((r) => r.startsWith("--map="));
if (mapArg) for (const [n, f] of Object.entries(JSON.parse(fs.readFileSync(mapArg.slice(6), "utf8")))) { if (home.has(n) && home.get(n) !== f) throw new Error(`mapped name already exported: ${n}`); home.set(n, f); }

const rel = (fromDir, target) => { let r = path.relative(fromDir, target); if (!r.startsWith(".")) r = "./" + r; return r; };
const files = [...new Set(fs.readFileSync(importersTsv, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t")[0].replace(/:\d+$/, "")))].map((f) => path.join(R, f));
const report = [];
let changedFiles = 0;
for (const file of files) {
  if (skip.has(file) || newFiles.includes(file)) continue;
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = [];
  const resolves = (spec) => { const r = path.resolve(path.dirname(file), spec.replace(/\?.*$/, "")); return r === oldFile || r === oldFile.replace(/\.(tsx?)$/, ""); };
  const fmt = (items, kw, typeOnly, spec) => {
    const body = items.join(", ");
    const one = `${kw}${typeOnly ? " type" : ""} { ${body} } from ${JSON.stringify(spec)};`;
    return one.length <= 190 ? one : `${kw}${typeOnly ? " type" : ""} {\n${items.map((i) => "  " + i).join(",\n")}\n} from ${JSON.stringify(spec)};`;
  };
  for (const st of sf.statements) {
    if ((ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && resolves(st.moduleSpecifier.text)) {
      const isImport = ts.isImportDeclaration(st);
      const clause = isImport ? st.importClause : st.exportClause;
      if (isImport && !st.importClause) { report.push(`BARE ${file}`); continue; }
      let elements;
      let stmtType;
      if (isImport) {
        if (st.importClause.name || (st.importClause.namedBindings && !ts.isNamedImports(st.importClause.namedBindings))) { report.push(`NONNAMED ${file}`); continue; }
        elements = st.importClause.namedBindings.elements;
        stmtType = st.importClause.isTypeOnly;
      } else {
        if (!clause || !ts.isNamedExports(clause)) { report.push(`EXPORTSTAR ${file}`); continue; }
        elements = clause.elements;
        stmtType = st.isTypeOnly;
      }
      const groups = new Map();
      for (const el of elements) {
        const orig = (el.propertyName ?? el.name).text;
        const target = home.get(orig);
        if (!target) { report.push(`NOHOME ${orig} in ${file}`); groups.clear(); break; }
        const item = `${el.isTypeOnly && !stmtType ? "type " : ""}${el.propertyName ? `${el.propertyName.text} as ${el.name.text}` : el.name.text}`;
        if (!groups.has(target)) groups.set(target, []);
        groups.get(target).push(item);
      }
      if (!groups.size) continue;
      const kw = isImport ? "import" : "export";
      const lines = [];
      for (const [target, items] of groups) {
        const spec = rel(path.dirname(file), target);
        lines.push(fmt(items, kw, stmtType, spec));
      }
      edits.push({ start: st.getStart(sf), end: st.end, text: lines.join("\n") });
      report.push(`OK ${path.relative(R, file)} -> ${[...groups.keys()].map((g) => path.basename(path.dirname(g))).join(",")}`);
    }
  }
  const visit = (n) => {
    if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && n.arguments.length === 1 && ts.isStringLiteral(n.arguments[0]) && resolves(n.arguments[0].text)) {
      const p = n.parent;
      let names = null;
      if (ts.isAwaitExpression(p) && ts.isVariableDeclaration(p.parent) && ts.isObjectBindingPattern(p.parent.name)) names = p.parent.name.elements.map((e) => (e.propertyName ?? e.name).text);
      else if (ts.isPropertyAccessExpression(p) || ts.isLiteralTypeNode(p) || ts.isImportTypeNode(p)) names = null;
      if (names) {
        const targets = new Set(names.map((x) => home.get(x)));
        if (targets.size === 1 && !targets.has(undefined)) { const t = [...targets][0]; edits.push({ start: n.arguments[0].getStart(sf), end: n.arguments[0].end, text: JSON.stringify(rel(path.dirname(file), t)) }); report.push(`DYN-OK ${path.relative(R, file)} -> ${path.basename(path.dirname(t))}`); }
        else report.push(`DYN-MANUAL ${path.relative(R, file)} ${names.join(",")}`);
      } else report.push(`DYN-MANUAL ${path.relative(R, file)}`);
    }
    if (ts.isImportTypeNode(n) && n.isTypeOf && !n.qualifier && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && resolves(n.argument.literal.text) && ts.isTypeReferenceNode(n.parent) && n.parent.typeName.getText(sf) === "Pick" && n.parent.typeArguments?.[0] === n) {
      const keys = n.parent.typeArguments[1];
      const lits = ts.isLiteralTypeNode(keys) ? [keys.literal] : ts.isUnionTypeNode(keys) ? keys.types.map((x) => (ts.isLiteralTypeNode(x) ? x.literal : null)) : [null];
      if (lits.every((l) => l && ts.isStringLiteral(l))) {
        const groups = new Map();
        let ok = true;
        for (const l of lits) { const t = home.get(l.text); if (!t) { ok = false; break; } if (!groups.has(t)) groups.set(t, []); groups.get(t).push(l.text); }
        if (ok) {
          const parts = [...groups].map(([t, names]) => `Pick<typeof import(${JSON.stringify(rel(path.dirname(file), t))}), ${names.map((x) => JSON.stringify(x)).join(" | ")}>`);
          edits.push({ start: n.parent.getStart(sf), end: n.parent.end, text: parts.join(" & ") });
          report.push(`PICK-OK ${path.relative(R, file)} -> ${[...groups.keys()].map((g) => path.basename(path.dirname(g))).join(",")}`);
          return;
        }
      }
      report.push(`PICK-MANUAL ${path.relative(R, file)}`);
      return;
    }
    if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal) && resolves(n.argument.literal.text)) {
      const q = n.qualifier && ts.isIdentifier(n.qualifier) ? n.qualifier.text : null;
      const t = q ? home.get(q) : null;
      if (t) { edits.push({ start: n.argument.literal.getStart(sf), end: n.argument.literal.end, text: JSON.stringify(rel(path.dirname(file), t)) }); report.push(`TYPEQ-OK ${path.relative(R, file)} ${q} -> ${path.basename(path.dirname(t))}`); }
      else report.push(`TYPEQ-MANUAL ${path.relative(R, file)}`);
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  if (!edits.length) continue;
  edits.sort((x, y) => y.start - x.start);
  let out = text;
  for (const e of edits) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  if (mode === "write") fs.writeFileSync(file, out);
  changedFiles++;
}
console.log(report.sort().join("\n"));
console.log("files changed", changedFiles, mode);
