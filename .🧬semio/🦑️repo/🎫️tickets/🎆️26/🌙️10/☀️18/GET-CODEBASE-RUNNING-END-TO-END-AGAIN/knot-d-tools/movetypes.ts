import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, relative } from "node:path";
import { spawnSync } from "node:child_process";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
const root = "/Users/ueli/Documents/semio";
const [mode, source, dest, ...names] = process.argv.slice(2) as string[];
const apply = mode === "apply";
const moved = new Set(names);
const rel = (from: string, to: string) => { let r = relative(dirname(join(root, from)), join(root, to)).split("\\").join("/"); if (!r.startsWith(".")) r = "./" + r; return r; };
const text = readFileSync(join(root, source!), "utf8"), sf = ts.createSourceFile(source!, text, ts.ScriptTarget.Latest, true);
const topDecl = new Map<string, ts.Statement>();
for (const st of sf.statements) {
  if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) { if (ts.isIdentifier(d.name)) topDecl.set(d.name.text, st); }
  else if ((ts.isFunctionDeclaration(st) || ts.isClassDeclaration(st) || ts.isInterfaceDeclaration(st) || ts.isTypeAliasDeclaration(st) || ts.isEnumDeclaration(st)) && st.name) topDecl.set(st.name.text, st);
}
const imports = new Map<string, ts.ImportDeclaration>();
for (const st of sf.statements) if (ts.isImportDeclaration(st)) { const c = st.importClause; if (c?.name) imports.set(c.name.text, st); if (c?.namedBindings) { if (ts.isNamespaceImport(c.namedBindings)) imports.set(c.namedBindings.name.text, st); else for (const e of c.namedBindings.elements) imports.set(e.name.text, st); } }
const decls = names.map((n) => { const d = topDecl.get(n); if (!d || !(ts.isInterfaceDeclaration(d) || ts.isTypeAliasDeclaration(d))) throw Error("not a type declaration: " + n); return d; });
const refs = new Set<string>(); for (const d of decls) { const visit = (n: ts.Node) => { if (ts.isIdentifier(n)) refs.add(n.text); ts.forEachChild(n, visit); }; visit(d); }
const needImports = new Map<ts.ImportDeclaration, string[]>();
const blocked: string[] = [];
for (const r of refs) { if (moved.has(r)) continue; const im = imports.get(r); if (im) { needImports.set(im, [...(needImports.get(im) ?? []), r]); continue; } if (topDecl.has(r)) blocked.push(r); }
if (blocked.length) { console.log("BLOCKED by top-level declarations:", blocked); process.exit(1); }
const importLines: string[] = [];
for (const [im, used] of needImports) {
  const spec = (im.moduleSpecifier as ts.StringLiteral).text; const target = spec.startsWith(".") ? rel(dest!, normalize(join(dirname(source!), spec))) : spec;
  const c = im.importClause!; const named: string[] = [];
  if (c.namedBindings && ts.isNamedImports(c.namedBindings)) for (const e of c.namedBindings.elements) if (used.includes(e.name.text)) named.push(e.getText(sf));
  const def = c.name && used.includes(c.name.text) ? c.name.text : null;
  importLines.push(`import type ${def ? def + (named.length ? "," : "") : ""}${named.length ? "{ " + named.join(", ") + " }" : ""} from "${target}";`);
}
const declText = decls.map((d) => text.slice(d.getFullStart(), d.getEnd()).replace(/^\s*\n/u, "").trim());
const body = [...importLines, "", ...declText].join("\n") + "\n";
console.log("moved", names.length, "types, imports", importLines.length, "bytes", body.length);
const files = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28, encoding: "utf8" }).stdout.split("\0").filter((f) => /\.(?:[cm]?[jt]sx?|d\.mts)$/u.test(f) && !f.includes("/node_modules/") && !f.startsWith(".🧬semio/") && existsSync(join(root, f)));
const edits = new Map<string, { start: number; end: number; text: string }[]>();
const usedInSource = new Set<string>();
const sourceVisit = (n: ts.Node) => { if (ts.isIdentifier(n) && moved.has(n.text) && !decls.some((d) => (d as ts.InterfaceDeclaration).name === n)) { let p: ts.Node | undefined = n; let inside = false; while (p) { if (decls.includes(p as ts.Statement)) { inside = true; break; } p = p.parent; } if (!inside) usedInSource.add(n.text); } ts.forEachChild(n, sourceVisit); };
sourceVisit(sf);
for (const from of files) {
  if (from === source) continue;
  const t = readFileSync(join(root, from), "utf8"); if (!names.some((n) => t.includes(n))) continue;
  const f = ts.createSourceFile(from, t, ts.ScriptTarget.Latest, true);
  for (const st of f.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || !st.moduleSpecifier.text.startsWith(".") || normalize(join(dirname(from), st.moduleSpecifier.text)) !== source) continue;
    const elements = ts.isImportDeclaration(st) ? (st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) ? st.importClause.namedBindings.elements : null) : (st.exportClause && ts.isNamedExports(st.exportClause) ? st.exportClause.elements : null);
    if (!elements) continue;
    const mv = elements.filter((e) => moved.has((e.propertyName ?? e.name).text)); if (!mv.length) continue;
    const keep = elements.filter((e) => !mv.includes(e));
    const quote = t[st.moduleSpecifier.getStart(f)]!;
    const newSpec = rel(from, dest!);
    let replacement: string;
    if (ts.isImportDeclaration(st)) {
      const c = st.importClause!; const typeOnly = c.isTypeOnly;
      const stmt = (els: readonly ts.ImportSpecifier[], spec: string, forceType: boolean) => `import ${typeOnly || forceType ? "type " : ""}{ ${els.map((e) => e.getText(f)).join(", ")} } from ${quote}${spec}${quote};`;
      const parts: string[] = [];
      if (keep.length || c.name) parts.push(`import ${typeOnly ? "type " : ""}${c.name ? c.name.text + (keep.length ? ", " : "") : ""}${keep.length ? "{ " + keep.map((e) => e.getText(f)).join(", ") + " }" : ""} from ${t.slice(st.moduleSpecifier.getStart(f), st.moduleSpecifier.getEnd())};`);
      parts.push(stmt(mv as ts.ImportSpecifier[], newSpec, true));
      replacement = parts.join(" ");
    } else {
      const parts: string[] = [];
      if (keep.length) parts.push(`export ${st.isTypeOnly ? "type " : ""}{ ${keep.map((e) => e.getText(f)).join(", ")} } from ${t.slice(st.moduleSpecifier.getStart(f), st.moduleSpecifier.getEnd())};`);
      parts.push(`export type { ${mv.map((e) => e.getText(f)).join(", ")} } from ${quote}${newSpec}${quote};`);
      replacement = parts.join(" ");
    }
    (edits.get(from) ?? edits.set(from, []).get(from)!).push({ start: st.getStart(f), end: st.getEnd(), text: replacement });
  }
}
console.log("importers", edits.size, "usedInSource", [...usedInSource]);
if (apply) {
  for (const [from, list] of edits) { let t = readFileSync(join(root, from), "utf8"); for (const e of list.sort((a, b) => b.start - a.start)) t = t.slice(0, e.start) + e.text + t.slice(e.end); writeFileSync(join(root, from), t); }
  mkdirSync(dirname(join(root, dest!)), { recursive: true }); writeFileSync(join(root, dest!), body);
  let out = text;
  const cuts = decls.map((d) => ({ start: d.getFullStart(), end: d.getEnd() })).sort((a, b) => b.start - a.start);
  for (const c of cuts) out = out.slice(0, c.start) + out.slice(c.end);
  if (usedInSource.size) { const first = out.indexOf("\n", out.search(/\S/u)); const line = `import type { ${[...usedInSource].sort().join(", ")} } from "${rel(source!, dest!)}";`; out = line + "\n" + out; }
  writeFileSync(join(root, source!), out);
}
