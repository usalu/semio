import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, join } from "node:path";

type Cfg = { module: string; apply?: boolean };
const modules = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const apply = process.argv.includes("--apply");
const kind = (f: string) => (f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const relSpec = (fromDir: string, to: string): string => { let r = relative(fromDir, to); if (!r.startsWith(".")) r = "./" + r; return r; };

for (const mod of modules) {
  const text = readFileSync(mod, "utf8");
  const sf = ts.createSourceFile(mod, text, ts.ScriptTarget.Latest, true, kind(mod));
  const bindings = new Map<string, { spec: string; imported: string; typeOnly: boolean; form: "named" | "default" | "ns" }>();
  const locals = new Set<string>();
  for (const s of sf.statements) {
    if (ts.isImportDeclaration(s) && ts.isStringLiteral(s.moduleSpecifier)) {
      const c = s.importClause; if (!c) continue;
      const spec = s.moduleSpecifier.text;
      if (c.name) bindings.set(c.name.text, { spec, imported: "default", typeOnly: c.isTypeOnly, form: "default" });
      if (c.namedBindings && ts.isNamespaceImport(c.namedBindings)) bindings.set(c.namedBindings.name.text, { spec, imported: "*", typeOnly: c.isTypeOnly, form: "ns" });
      if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) bindings.set(e.name.text, { spec, imported: (e.propertyName ?? e.name).text, typeOnly: c.isTypeOnly || e.isTypeOnly, form: "named" });
    } else {
      if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) { if (ts.isIdentifier(d.name)) locals.add(d.name.text); else d.name.elements.forEach((el) => ts.isBindingElement(el) && ts.isIdentifier(el.name) && locals.add(el.name.text)); }
      else if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isEnumDeclaration(s)) && s.name) locals.add(s.name.text);
    }
  }
  let depsType: ts.TypeAliasDeclaration | undefined;
  for (const s of sf.statements) if (ts.isTypeAliasDeclaration(s) && /TestDependencies$/u.test(s.name.text) && ts.isTypeLiteralNode(s.type)) depsType = s;
  if (!depsType) { console.log(mod, "no TestDependencies type"); continue; }
  const lit = depsType.type as ts.TypeLiteralNode;
  console.log(`== ${mod.split("/").slice(-3).join("/")}  ${depsType.name.text}`);
  const rows: string[] = [];
  const special: string[] = [], local: string[] = [], imp: string[] = [];
  for (const m of lit.members) {
    if (!ts.isPropertySignature(m) || !m.type) { rows.push("?? " + m.getText()); continue; }
    const name = (m.name as ts.Identifier).text;
    const tt = m.type.getText();
    const typeofName = /^typeof\s+([A-Za-z0-9_$]+)$/u.exec(tt)?.[1];
    if (!typeofName) { rows.push(`COMPLEX ${name}: ${tt}`); continue; }
    if (typeofName !== name) rows.push(`ALIAS ${name}: typeof ${typeofName}`);
    if (bindings.has(typeofName)) imp.push(typeofName);
    else if (/^__/u.test(typeofName)) special.push(typeofName);
    else if (locals.has(typeofName)) local.push(typeofName);
    else rows.push(`UNKNOWN ${typeofName}`);
  }
  console.log(` imports=${imp.length} special=${special.join(",")} local=${local.join(",")}`);
  for (const r of rows) console.log("  ", r);
}
