import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const [edgesPath, out, prefix = "🧰️framework/🛍️products/🦑️repo/"] = process.argv.slice(2) as [string, string, string?];
const { edges } = JSON.parse(readFileSync(edgesPath, "utf8"));
const pairs = new Map<string, { from: string; to: string; rules: Set<string> }>();
for (const e of edges) if (e.from.startsWith(prefix) && e.rule.startsWith("no-cross-package-relative-")) {
  const key = e.from + "\0" + e.to; const row = pairs.get(key) ?? { from: e.from, to: e.to, rules: new Set() };
  row.rules.add(e.rule.slice("no-cross-package-relative-".length)); pairs.set(key, row);
}
const byFrom = new Map<string, Set<string>>();
for (const p of pairs.values()) { if (!byFrom.has(p.from)) byFrom.set(p.from, new Set()); byFrom.get(p.from)!.add(p.to); }
const refs: unknown[] = [], unmatched: unknown[] = [];
for (const [from, targets] of byFrom) {
  const text = readFileSync(join(root, from), "utf8");
  const kind = from.endsWith(".tsx") ? ts.ScriptKind.TSX : /\.[cm]?js$/u.test(from) ? ts.ScriptKind.JS : ts.ScriptKind.TS;
  const sf = ts.createSourceFile(from, text, ts.ScriptTarget.Latest, true, kind);
  const found = new Set<string>();
  const resolve = (spec: string) => spec.startsWith(".") ? normalize(join(dirname(from), spec)) : null;
  const note = (node: ts.Node, spec: ts.StringLiteralLike, form: string, names: unknown) => {
    const to = resolve(spec.text);
    if (to !== null && targets.has(to)) { found.add(to); refs.push({ from, to, form, spec: spec.text, start: spec.getStart(sf), end: spec.getEnd(), names, line: sf.getLineAndCharacterOfPosition(spec.getStart(sf)).line + 1, text: node.getText(sf).slice(0, 400) }); }
  };
  const visit = (node: ts.Node) => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) {
      const c = node.importClause; const names: { imported: string; local: string; type: boolean }[] = []; let def: string | null = null, ns: string | null = null;
      if (c) { if (c.name) def = c.name.text; if (c.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) ns = c.namedBindings.name.text; else for (const el of c.namedBindings.elements) names.push({ imported: (el.propertyName ?? el.name).text, local: el.name.text, type: c.isTypeOnly || el.isTypeOnly }); } }
      note(node, node.moduleSpecifier, "import", { names, def, ns, sideEffect: !c, typeOnly: c?.isTypeOnly ?? false });
    } else if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) {
      const names: unknown[] = []; const clause = node.exportClause;
      if (clause && ts.isNamedExports(clause)) for (const el of clause.elements) names.push({ imported: (el.propertyName ?? el.name).text, local: el.name.text, type: node.isTypeOnly || el.isTypeOnly });
      note(node, node.moduleSpecifier, "export", { names, star: !clause, ns: clause && ts.isNamespaceExport(clause) ? clause.name.text : null });
    } else if (ts.isCallExpression(node) && node.arguments.length >= 1 && ts.isStringLiteralLike(node.arguments[0]!)) {
      const callee = node.expression;
      const isDyn = callee.kind === ts.SyntaxKind.ImportKeyword, isReq = ts.isIdentifier(callee) && callee.text === "require", isMock = ts.isPropertyAccessExpression(callee) && ["mock", "doMock"].includes(callee.name.text);
      if (isDyn || isReq || isMock) note(node.parent ?? node, node.arguments[0] as ts.StringLiteralLike, isDyn ? "dynamic" : isReq ? "require" : "mock", {});
    } else if (ts.isImportEqualsDeclaration(node) && ts.isExternalModuleReference(node.moduleReference) && ts.isStringLiteralLike(node.moduleReference.expression)) note(node, node.moduleReference.expression, "equals", {});
    ts.forEachChild(node, visit);
  };
  visit(sf);
  for (const t of targets) if (!found.has(t)) unmatched.push({ from, to: t });
}
writeFileSync(out, JSON.stringify({ pairs: [...pairs.values()].map((p) => ({ ...p, rules: [...p.rules] })), refs, unmatched }, null, 1));
console.log("pairs", pairs.size, "refs", refs.length, "unmatched", unmatched.length);
