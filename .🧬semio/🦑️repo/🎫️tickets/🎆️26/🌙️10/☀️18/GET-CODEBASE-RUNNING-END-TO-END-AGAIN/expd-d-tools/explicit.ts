import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const apply = process.argv[2] === "apply";
const facades = process.argv.slice(3);
const parse = (file: string) => ts.createSourceFile(file, readFileSync(join(root, file), "utf8"), ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const isFile = (path: string) => { try { return statSync(join(root, path)).isFile(); } catch { return false; } };
const resolveSpec = (from: string, spec: string) => spec.startsWith(".") ? normalize(join(dirname(from), spec.split("?")[0]!)) : null;
const hasMod = (n: ts.Node, k: ts.SyntaxKind) => !!ts.getModifiers(n as ts.HasModifiers)?.some((m) => m.kind === k);
const kinds = new Map<string, Map<string, "value" | "type">>();
function exportKinds(file: string, depth = 0): Map<string, "value" | "type"> {
  if (kinds.has(file)) return kinds.get(file)!;
  const out = new Map<string, "value" | "type">(); kinds.set(file, out);
  if (depth > 10 || !isFile(file)) return out;
  const set = (name: string, kind: "value" | "type") => { if (kind === "value" || !out.has(name)) out.set(name, kind); };
  for (const st of parse(file).statements) {
    if (ts.isExportDeclaration(st)) {
      const target = st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) ? resolveSpec(file, st.moduleSpecifier.text) : null;
      const sub = target ? exportKinds(target, depth + 1) : null;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) { const original = (e.propertyName ?? e.name).text; const kind = st.isTypeOnly || e.isTypeOnly ? "type" : sub?.get(original) ?? "value"; set(e.name.text, kind); }
      else if (st.exportClause && ts.isNamespaceExport(st.exportClause)) set(st.exportClause.name.text, "value");
      else if (sub) for (const [name, kind] of sub) if (name !== "default" && !out.has(name)) out.set(name, st.isTypeOnly ? "type" : kind);
    } else if (ts.isExportAssignment(st)) set("default", "value");
    else if (hasMod(st, ts.SyntaxKind.ExportKeyword)) {
      if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) set(d.name.text, "value"); else for (const b of d.name.elements) if (ts.isBindingElement(b) && ts.isIdentifier(b.name)) set(b.name.text, "value"); }
      else if (ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st)) { if (st.name) set(st.name.text, "value"); }
      else if (ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) set(st.name.text, "type");
      else if (ts.isEnumDeclaration(st) || ts.isModuleDeclaration(st)) set((st.name as ts.Identifier).text, "value");
    }
  }
  return out;
}
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && isFile(f));
const used = new Map<string, Set<string>>(), star = new Map<string, string[]>();
for (const f of facades) { used.set(f, new Set()); star.set(f, []); }
const memberNames = (sf: ts.SourceFile, local: string): string[] | "*" => {
  const out = new Set<string>(); let escapes = false;
  const visit = (n: ts.Node) => {
    if (ts.isImportDeclaration(n)) return;
    if (ts.isPropertyAccessExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === local) { out.add(n.name.text); return; }
    if (ts.isQualifiedName(n) && ts.isIdentifier(n.left) && n.left.text === local) { out.add(n.right.text); return; }
    if (ts.isIdentifier(n) && n.text === local && !(ts.isNamespaceImport(n.parent))) { escapes = true; return; }
    ts.forEachChild(n, visit);
  };
  visit(sf);
  return escapes ? "*" : [...out];
};
for (const from of files) {
  const text = readFileSync(join(root, from), "utf8");
  if (!facades.some((f) => text.includes(f.split("/").slice(-3, -1).join("/")) || true)) continue;
  const sf = ts.createSourceFile(from, text, ts.ScriptTarget.Latest, true, from.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const note = (spec: string, names: string[] | "*") => { const to = resolveSpec(from, spec); if (to && used.has(to)) { if (names === "*") star.get(to)!.push(from); else names.forEach((n) => used.get(to)!.add(n)); } };
  const visit = (n: ts.Node) => {
    if (ts.isImportDeclaration(n) && ts.isStringLiteral(n.moduleSpecifier)) {
      const c = n.importClause; const names: string[] = []; let all: string[] | "*" = [];
      if (!c) all = "*"; else { if (c.name) names.push("default"); if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) all = memberNames(sf, c.namedBindings.name.text); else for (const e of c.namedBindings.elements) names.push((e.propertyName ?? e.name).text); } }
      note(n.moduleSpecifier.text, all === "*" ? "*" : [...names, ...all]);
    } else if (ts.isExportDeclaration(n) && n.moduleSpecifier && ts.isStringLiteral(n.moduleSpecifier)) {
      if (n.exportClause && ts.isNamedExports(n.exportClause)) note(n.moduleSpecifier.text, n.exportClause.elements.map((e) => (e.propertyName ?? e.name).text)); else note(n.moduleSpecifier.text, "*");
    } else if (ts.isCallExpression(n) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) && (n.expression.kind === ts.SyntaxKind.ImportKeyword || (ts.isIdentifier(n.expression) && n.expression.text === "require"))) {
      let p: ts.Node = n.parent; while (ts.isParenthesizedExpression(p) || ts.isAwaitExpression(p) || ts.isAsExpression(p)) p = p.parent;
      let names: string[] | "*" = "*";
      if (ts.isPropertyAccessExpression(p)) names = [p.name.text];
      else if (ts.isVariableDeclaration(p) && ts.isObjectBindingPattern(p.name) && !p.name.elements.some((e) => e.dotDotDotToken)) names = p.name.elements.map((e) => (e.propertyName && ts.isIdentifier(e.propertyName) ? e.propertyName : e.name as ts.Identifier).text);
      else if (ts.isVariableDeclaration(p) && ts.isIdentifier(p.name)) names = memberNames(sf, p.name.text);
      note(n.arguments[0].text, names);
    } else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument) && ts.isStringLiteral(n.argument.literal)) {
      const q = n.qualifier; note(n.argument.literal.text, q ? [(function f(x: ts.EntityName): string { return ts.isIdentifier(x) ? x.text : f(x.left); })(q)] : "*");
    }
    ts.forEachChild(n, visit);
  };
  visit(sf);
}
for (const f of facades) {
  const sf = parse(f);
  const specs = sf.statements.filter((s): s is ts.ExportDeclaration => ts.isExportDeclaration(s) && !!s.moduleSpecifier && !s.exportClause);
  if (specs.length !== 1) { console.log("SKIP (not a single wildcard)", f); continue; }
  const spec = (specs[0]!.moduleSpecifier as ts.StringLiteral).text, target = resolveSpec(f, spec)!;
  const have = exportKinds(target);
  const names = [...used.get(f)!].sort(), values: string[] = [], types: string[] = [], missing: string[] = [];
  for (const name of names) { const kind = have.get(name); if (!kind) missing.push(name); else (kind === "value" ? values : types).push(name); }
  console.log(f.slice(-70), "values", values.length, "types", types.length, "missing", missing, "starImporters", star.get(f)!.length, star.get(f)!.slice(0, 3));
  if (apply && !missing.length) {
    const header = sf.getFullText().slice(0, specs[0]!.getFullStart()).trim();
    const lines = [...(header ? [header] : []), ...(values.length ? [`export { ${values.join(", ")} } from "${spec}";`] : []), ...(types.length ? [`export type { ${types.join(", ")} } from "${spec}";`] : [])];
    writeFileSync(join(root, f), lines.join("\n") + "\n");
  }
}
