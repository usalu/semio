import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
const apply = process.argv.includes("--apply");
const ovArg = process.argv.find((a) => a.startsWith("--overrides="));
const overrides: Record<string, Record<string, string>> = ovArg ? JSON.parse(readFileSync(ovArg.slice(12), "utf8")) : {};
for (const mod of process.argv.slice(2).filter((a) => !a.startsWith("--"))) {
  const text = readFileSync(mod, "utf8");
  const sf = ts.createSourceFile(mod, text, ts.ScriptTarget.Latest, true, mod.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const imports = new Set<string>();
  const destructured = new Map<string, { obj: string; prop: string }>();
  const specials = new Map<string, string>();
  const stringConsts = new Set<string>();
  const locals = new Set<string>();
  for (const s of sf.statements) {
    if (ts.isImportDeclaration(s) && s.importClause) {
      const c = s.importClause;
      if (c.name) imports.add(c.name.text);
      if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) imports.add(c.namedBindings.name.text);
      if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) imports.add(e.name.text);
    } else if (ts.isVariableStatement(s)) {
      for (const d of s.declarationList.declarations) {
        if (ts.isObjectBindingPattern(d.name) && d.initializer && ts.isIdentifier(d.initializer)) {
          for (const el of d.name.elements) if (ts.isIdentifier(el.name)) destructured.set(el.name.text, { obj: d.initializer.text, prop: (el.propertyName && ts.isIdentifier(el.propertyName) ? el.propertyName.text : el.name.text) });
        } else if (ts.isIdentifier(d.name)) {
          const n = d.name.text;
          const init = d.initializer;
          if (n.startsWith("__") && init) {
            const m = /import\(\s*["']([^"']+)["']\s*\)/u.exec(init.getText());
            if (m) specials.set(n, m[1]!);
          } else if (init && (ts.isStringLiteral(init) || ts.isNoSubstitutionTemplateLiteral(init))) stringConsts.add(n);
          else locals.add(n);
        }
      }
    } else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s)) && s.name) locals.add(s.name.text);
  }
  let alias: ts.TypeAliasDeclaration | undefined;
  for (const s of sf.statements) if (ts.isTypeAliasDeclaration(s) && /TestDependencies$/u.test(s.name.text) && ts.isTypeLiteralNode(s.type)) alias = s;
  if (!alias) { console.log(mod, "no alias"); continue; }
  const lit = alias.type as ts.TypeLiteralNode;
  const edits: [number, number, string][] = [];
  const report: string[] = [];
  for (const m of lit.members) {
    if (!ts.isPropertySignature(m) || !m.type) continue;
    const t = m.type.getText();
    const mm = /^typeof\s+([A-Za-z0-9_$]+)$/u.exec(t);
    if (!mm) { report.push(`COMPLEX ${t}`); continue; }
    const n = mm[1]!;
    let repl: string | undefined;
    if (imports.has(n)) continue;
    if (destructured.has(n)) { const d = destructured.get(n)!; if (!imports.has(d.obj)) report.push(`DESTRUCT-SRC-LOCAL ${n} from ${d.obj}`); repl = `typeof ${d.obj}.${d.prop}`; }
    else if (specials.has(n)) { const sp = specials.get(n)!; const adj = sp.startsWith("./") ? "." + sp : sp.startsWith("../") ? "../" + sp : sp; repl = `typeof import("${adj}") | null`; }
    else if (stringConsts.has(n)) repl = "string";
    else if (overrides[mod]?.[n]) repl = overrides[mod]![n]!;
    else if (locals.has(n)) { report.push(`LOCAL ${n}`); continue; }
    else report.push(`UNKNOWN ${n}`);
    if (repl) edits.push([m.type.getStart(), m.type.getEnd(), repl]);
  }
  console.log(`${mod.split("/").slice(-3).join("/")} ${alias.name.text}: ${edits.length} rewritten; ${report.join("; ") || "no local left"}`);
  if (apply && report.length === 0) {
    let out = text;
    for (const [st, en, r] of edits.sort((a, b) => b[0] - a[0])) out = out.slice(0, st) + r + out.slice(en);
    writeFileSync(mod, out);
  }
}
