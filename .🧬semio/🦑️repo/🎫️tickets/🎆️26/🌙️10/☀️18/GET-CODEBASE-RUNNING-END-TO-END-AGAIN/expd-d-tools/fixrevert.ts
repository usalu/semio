import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const dir = process.argv[2]!;
const apply = process.argv[3] === "apply";
const barrels = new Set<string>((JSON.parse(readFileSync(join(dir, "barrels.json"), "utf8")) as any[]).map((b) => b.glue));
const isFile = (p: string) => { try { return statSync(join(root, p)).isFile(); } catch { return false; } };
const parse = (file: string, text = readFileSync(join(root, file), "utf8")) => ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
const hasMod = (n: ts.Node, k: ts.SyntaxKind) => !!ts.getModifiers(n as ts.HasModifiers)?.some((m) => m.kind === k);
const memo = new Map<string, Set<string>>();
function names(file: string, depth = 0): Set<string> {
  if (memo.has(file)) return memo.get(file)!;
  const out = new Set<string>(); memo.set(file, out);
  if (depth > 10 || !isFile(file)) return out;
  for (const st of parse(file).statements) {
    if (ts.isExportDeclaration(st)) {
      const t = st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text.startsWith(".") ? normalize(join(dirname(file), st.moduleSpecifier.text)) : null;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) out.add(e.name.text);
      else if (st.exportClause && ts.isNamespaceExport(st.exportClause)) out.add(st.exportClause.name.text);
      else if (t) for (const n of names(t, depth + 1)) out.add(n);
    } else if (ts.isExportAssignment(st)) out.add("default");
    else if (hasMod(st, ts.SyntaxKind.ExportKeyword)) {
      if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) out.add(d.name.text); else for (const b of d.name.elements) if (ts.isBindingElement(b) && ts.isIdentifier(b.name)) out.add(b.name.text); }
      else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st)) && st.name) out.add(st.name.text);
      else if (ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) out.add(st.name.text);
      else if (ts.isEnumDeclaration(st) || ts.isModuleDeclaration(st)) out.add((st.name as ts.Identifier).text);
    }
  }
  return out;
}
const perFile = new Map<string, Map<string, Set<string>>>();
for (const n of ["edges0.plan-applied.json", "e1.plan-applied.json"]) {
  const d = JSON.parse(readFileSync(join(dir, n), "utf8"));
  for (const e of d.edits) {
    if (!e.after) continue; const glue = normalize(join(dirname(e.file), e.after)); if (!barrels.has(glue)) continue;
    const m = perFile.get(e.file) ?? new Map(); const set = m.get(e.after) ?? new Set(); set.add(e.before); m.set(e.after, set); perFile.set(e.file, m);
  }
}
for (const [file, byAfter] of perFile) for (const [after, befores] of byAfter) {
  if (befores.size < 2) continue;
  const candidates = [...befores].map((b) => ({ spec: b, target: normalize(join(dirname(file), b)) }));
  const text = readFileSync(join(root, file), "utf8"), sf = parse(file, text);
  const edits: { start: number; end: number; text: string }[] = [];
  const consider = (lit: ts.StringLiteral, wanted: string[] | "*") => {
    if (!candidates.some((c) => c.spec === lit.text)) return;
    if (wanted === "*") { console.log("STAR", file.slice(-60), lit.text.slice(-40)); return; }
    const fit = candidates.filter((c) => wanted.every((n) => names(c.target).has(n)));
    if (fit.length !== 1) { console.log("AMBIGUOUS", file.slice(-60), wanted.slice(0, 4), fit.length); return; }
    if (fit[0]!.spec !== lit.text) edits.push({ start: lit.getStart(sf) + 1, end: lit.getEnd() - 1, text: fit[0]!.spec });
  };
  const visit = (n: ts.Node) => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause, w: string[] = []; let star = !c;
      if (c) { if (c.name) w.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) star = true; else for (const e of c.namedBindings.elements) w.push((e.propertyName ?? e.name).text); } }
      consider(n.moduleSpecifier, star ? "*" : w);
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteral(n.arguments[0]) && n.expression.kind === ts.SyntaxKind.ImportKeyword) {
      let p: ts.Node = n.parent; while (ts.isParenthesizedExpression(p) || ts.isAwaitExpression(p)) p = p.parent;
      if (ts.isVariableDeclaration(p) && ts.isObjectBindingPattern(p.name)) consider(n.arguments[0], p.name.elements.map((e) => (e.propertyName && ts.isIdentifier(e.propertyName) ? e.propertyName : (e.name as ts.Identifier)).text));
      else if (ts.isPropertyAccessExpression(p)) consider(n.arguments[0], [p.name.text]);
      else consider(n.arguments[0], "*");
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  console.log(apply ? "FIX" : "DRY", file.slice(-70), edits.length);
  if (apply && edits.length) { let out = text; for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end); writeFileSync(join(root, file), out); }
}
