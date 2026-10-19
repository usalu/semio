import ts from "typescript";
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { parse } from "./lib.ts";

const cache = new Map<string, { values: Set<string>; types: Set<string>; declared: Set<string> }>();
const resolveFile = (from: string, spec: string): string | undefined => {
  const base = resolve(dirname(from), spec.split("?")[0]!);
  for (const c of [base, base + ".ts", base + ".tsx", base.replace(/\.js$/u, ".ts"), base + "/🟦️.ts"]) if (existsSync(c) && !c.endsWith("/")) { try { if (readFileSync(c, "utf8") !== undefined) return c; } catch { /* directory */ } }
  return undefined;
};

/** Names a module exports (values and types), following explicit and star re-exports. */
export function exportsOf(abs: string, seen = new Set<string>()): { values: Set<string>; types: Set<string>; declared: Set<string> } {
  const hit = cache.get(abs);
  if (hit) return hit;
  const values = new Set<string>(), types = new Set<string>(), declared = new Set<string>();
  if (seen.has(abs) || !existsSync(abs)) return { values, types, declared };
  seen.add(abs);
  if (abs.endsWith(".json")) { values.add("default"); return { values, types, declared }; }
  let text: string;
  try { text = readFileSync(abs, "utf8"); } catch { return { values, types, declared }; }
  const sf = parse(abs, text);
  const has = (n: ts.Node, k: ts.SyntaxKind): boolean => !!ts.getModifiers(n as ts.HasModifiers)?.some(m => m.kind === k);
  for (const st of sf.statements) {
    if (ts.isExportDeclaration(st)) {
      const typeOnly = st.isTypeOnly;
      if (st.exportClause && ts.isNamedExports(st.exportClause)) for (const e of st.exportClause.elements) { (typeOnly || e.isTypeOnly ? types : values).add(e.name.text); if (!st.moduleSpecifier) declared.add(e.name.text); }
      else if (st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier)) {
        const target = resolveFile(abs, st.moduleSpecifier.text);
        if (target) { const sub = exportsOf(target, seen); sub.values.forEach(v => v !== "default" && values.add(v)); sub.types.forEach(v => types.add(v)); }
        if (st.exportClause && ts.isNamespaceExport(st.exportClause)) values.add(st.exportClause.name.text);
      }
    } else if (ts.isExportAssignment(st)) values.add("default");
    else if (has(st, ts.SyntaxKind.ExportKeyword)) {
      const dflt = has(st, ts.SyntaxKind.DefaultKeyword);
      if (ts.isVariableStatement(st)) { for (const d of st.declarationList.declarations) if (ts.isIdentifier(d.name)) { values.add(d.name.text); declared.add(d.name.text); } else for (const b of d.name.elements) if (ts.isBindingElement(b) && ts.isIdentifier(b.name)) { values.add(b.name.text); declared.add(b.name.text); } }
      else if (ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isEnumDeclaration(st) || ts.isModuleDeclaration(st)) { if (dflt) values.add("default"); else if (st.name) { values.add((st.name as ts.Identifier).text); declared.add((st.name as ts.Identifier).text); } }
      else if (ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st)) { types.add(st.name.text); declared.add(st.name.text); }
    }
  }
  const out = { values, types, declared };
  cache.set(abs, out);
  return out;
}
export const allNames = (abs: string): Set<string> => { const e = exportsOf(abs); return new Set([...e.values, ...e.types]); };
