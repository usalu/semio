// ⚖️ c5d: replays the plugin publication source ownership law (owners exist and export their names, consumers close on owners, isolated consumers import no owner/router, required/forbidden edges, acyclic runtime graph, routers bounded) without the test runner.
import {existsSync, readFileSync} from "node:fs";
import {basename, dirname, relative, resolve} from "node:path";
const ts = (await import("/Users/ueli/.bun/install/cache/typescript@5.9.3@@@1/lib/typescript.js")).default as typeof import("typescript");
const repo = "/Users/ueli/Documents/semio";
const fx = JSON.parse(readFileSync(`${repo}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📣️plugin-publication-source-ownership/🔣️.json`, "utf8"));
let problems = 0;
const bad = (m: string) => { problems++; console.log("PROBLEM", m); };
const parse = (p: string) => ts.createSourceFile(p, readFileSync(p, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const exportedNames = (p: string) => {
  const sf = parse(p), names = new Set<string>();
  if ((sf as any).parseDiagnostics.length) bad("parse " + p);
  for (const s of sf.statements) {
    if (ts.isExportDeclaration(s) && s.exportClause && ts.isNamedExports(s.exportClause)) for (const e of s.exportClause.elements) names.add(e.name.text);
    const exp = ts.canHaveModifiers(s) && ts.getModifiers(s)?.some(m => m.kind === ts.SyntaxKind.ExportKeyword);
    if (!exp) continue;
    if ((ts.isFunctionDeclaration(s) || ts.isClassDeclaration(s) || ts.isTypeAliasDeclaration(s) || ts.isInterfaceDeclaration(s) || ts.isEnumDeclaration(s)) && s.name) names.add(s.name.text);
    if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) if (ts.isIdentifier(d.name)) names.add(d.name.text);
  }
  return names;
};
const rel = (path: string, specs: Set<string>) => (e: ts.Expression | undefined) => { if (e && ts.isStringLiteral(e) && e.text.startsWith(".")) specs.add(e.text); };
const direct = (p: string): Set<string> => { const out = new Set<string>(), sf = parse(p); const add = (e?: ts.Expression) => { if (e && ts.isStringLiteral(e) && e.text.startsWith(".")) out.add(relative(repo, resolve(dirname(p), e.text)).replaceAll("\\", "/")); }; const v = (n: ts.Node): void => { if (ts.isImportDeclaration(n)) add(n.moduleSpecifier); if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword) add(n.arguments[0]); ts.forEachChild(n, v); }; v(sf); return out; };
const runtime = (p: string): Set<string> => { const out = new Set<string>(), sf = parse(p); const add = (e?: ts.Expression) => { if (e && ts.isStringLiteral(e) && e.text.startsWith(".")) { const t = resolve(dirname(p), e.text); if (existsSync(t)) out.add(t); } }; const v = (n: ts.Node): void => { if (ts.isImportDeclaration(n)) { const c = n.importClause; const only = c?.isTypeOnly || Boolean(c && !c.name && c.namedBindings && ts.isNamedImports(c.namedBindings) && c.namedBindings.elements.every(x => x.isTypeOnly)); if (!only) add(n.moduleSpecifier); } if (ts.isExportDeclaration(n) && !n.isTypeOnly) add(n.moduleSpecifier); if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword) add(n.arguments[0]); ts.forEachChild(n, v); }; v(sf); return out; };
const owners = new Set<string>(fx.owners.map((o: any) => o.path)), routers = new Set<string>(fx.routers.map((o: any) => o.path));
for (const o of fx.owners) { const p = resolve(repo, o.path); if (!existsSync(p)) { bad("missing owner " + o.path); continue; } if (basename(p) !== "🟦️.ts") bad("leaf " + o.path); const n = exportedNames(p); for (const e of o.exports) if (!n.has(e)) bad(`${o.path}: ${e}`); }
for (const r of fx.routers) { const p = resolve(repo, r.path), s = readFileSync(p, "utf8"); if (s.trimEnd().split("\n").length > r.maximumLines) bad("router too long " + r.path); if ([...exportedNames(p)].length) bad("router exports " + r.path); }
for (const c of fx.consumers) { const m = direct(resolve(repo, c.path)); const a = [...m].filter(x => owners.has(x)).sort(), b = [...c.owners].sort(); if (JSON.stringify(a) !== JSON.stringify(b)) bad("consumer " + c.path); if ([...m].filter(x => routers.has(x)).length) bad("consumer imports router " + c.path); }
for (const c of fx.isolatedConsumers) { const p = resolve(repo, c.path); if (!existsSync(p)) { bad("missing isolated " + c.path); continue; } const m = direct(p); if ([...m].filter(x => owners.has(x) || routers.has(x)).length) bad("isolated imports owner/router " + c.path); if ([...exportedNames(p)].length) bad("isolated exports " + c.path); }
for (const e of fx.dependencyEdges.required) if (!direct(resolve(repo, e.from)).has(e.to)) bad(`required ${e.from} -> ${e.to}`);
for (const e of fx.dependencyEdges.forbidden) if (direct(resolve(repo, e.from)).has(e.to)) bad(`forbidden ${e.from} -/> ${e.to}`);
const roots = [resolve(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"), resolve(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry")];
const state = new Map<string, 1 | 2>(), stack: string[] = [], cycles: string[][] = [];
const visit = (p: string): void => { if (state.get(p) === 2) return; if (state.get(p) === 1) { cycles.push(stack.slice(stack.indexOf(p)).concat(p)); return; } state.set(p, 1); stack.push(p); for (const t of runtime(p)) if (roots.some(r => t.startsWith(r + "/"))) visit(t); stack.pop(); state.set(p, 2); };
for (const row of [...fx.owners, ...fx.routers]) visit(resolve(repo, row.path));
if (cycles.length) bad("cycles " + JSON.stringify(cycles.map(c => c.map(x => x.split("/").slice(-2).join("/")))));
console.log("owners", fx.owners.length, "consumers", fx.consumers.length, "isolated", fx.isolatedConsumers.length, "required", fx.dependencyEdges.required.length, "forbidden", fx.dependencyEdges.forbidden.length, "visited", state.size, "problems", problems);
