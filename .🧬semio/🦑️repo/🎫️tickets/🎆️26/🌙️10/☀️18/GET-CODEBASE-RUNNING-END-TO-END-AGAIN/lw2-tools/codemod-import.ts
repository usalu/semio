import ts from "typescript";
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const FROM = JSON.parse(process.env.FROM!) as string[];
const TO = process.env.TO!;
const RENAME = JSON.parse(process.env.RENAME ?? "{}") as Record<string, string>;
const roots = (process.env.ROOTS ?? "✏️s").split(",");
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated", ".git", "📤️dist"]);
const dry = process.env.DRY === "1";
const files: string[] = [];
const walk = (d: string) => { let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; } for (const e of es) { if (SKIP.has(e.name)) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(ts|tsx|mts)$/u.test(e.name) && !e.name.endsWith(".d.ts")) files.push(p); } };
for (const r of roots) walk(join(ROOT, r));
const specOf = (n: ts.Node): string | undefined => { const m = (n as any).moduleSpecifier; return m && ts.isStringLiteral(m) ? m.text : undefined; };
let changed = 0, merged = 0; const out: string[] = [];
for (const f of files) {
  let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
  if (!FROM.some(s => t.includes(s))) continue;
  const sf = ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits: { start: number; end: number; text: string }[] = [];
  const fromImports: ts.ImportDeclaration[] = [], toImports: ts.ImportDeclaration[] = [];
  for (const st of sf.statements) {
    const s = specOf(st); if (s === undefined) continue;
    if (FROM.includes(s)) {
      if (ts.isImportDeclaration(st)) fromImports.push(st);
      else if (ts.isExportDeclaration(st)) edits.push({ start: st.moduleSpecifier!.getStart(sf) + 1, end: st.moduleSpecifier!.getEnd() - 1, text: TO });
    } else if (s === TO && ts.isImportDeclaration(st)) toImports.push(st);
  }
  const elementText = (el: ts.ImportSpecifier, typeOnlyDecl: boolean): string => {
    const prop = el.propertyName?.text, name = el.name.text;
    const p2 = prop && RENAME[prop] ? RENAME[prop] : prop, n2 = !prop && RENAME[name] ? RENAME[name] : name;
    const core = p2 ? `${p2} as ${n2}` : n2;
    return (typeOnlyDecl && !el.isTypeOnly ? "type " : el.isTypeOnly ? "type " : "") + core;
  };
  const target = toImports.find(d => d.importClause?.namedBindings && ts.isNamedImports(d.importClause.namedBindings) && !d.importClause.name);
  for (const d of fromImports) {
    const clause = d.importClause;
    const named = clause?.namedBindings;
    if (target && clause && named && ts.isNamedImports(named) && !clause.name) {
      const tclause = target.importClause!, tnamed = tclause.namedBindings as ts.NamedImports;
      const existing = tnamed.elements.map(e => elementText(e, tclause.isTypeOnly).replace(/^type /u, tclause.isTypeOnly ? "type " : "type ")).map((s, i) => tclause.isTypeOnly ? s : (tnamed.elements[i]!.isTypeOnly ? s : s.replace(/^type /u, "")));
      const incoming = named.elements.map(e => elementText(e, clause.isTypeOnly));
      const all = [...new Set([...existing, ...incoming])];
      const typeOnly = tclause.isTypeOnly && all.every(s => s.startsWith("type "));
      const rendered = typeOnly ? all.map(s => s.replace(/^type /u, "")) : all;
      edits.push({ start: tnamed.getStart(sf), end: tnamed.getEnd(), text: `{ ${rendered.join(", ")} }` });
      if (tclause.isTypeOnly && !typeOnly) edits.push({ start: tclause.getStart(sf), end: tclause.getStart(sf) + "type ".length, text: "" });
      edits.push({ start: d.getFullStart() === 0 ? d.getStart(sf) : d.getStart(sf), end: d.getEnd() + (t[d.getEnd()] === "\n" ? 1 : 0), text: "" });
      merged++;
    } else {
      edits.push({ start: d.moduleSpecifier.getStart(sf) + 1, end: d.moduleSpecifier.getEnd() - 1, text: TO });
      if (named && ts.isNamedImports(named)) for (const el of named.elements) { const orig = (el.propertyName ?? el.name).text; if (RENAME[orig]) { const id = el.propertyName ?? el.name; edits.push({ start: id.getStart(sf), end: id.getEnd(), text: RENAME[orig]! }); } }
    }
  }
  if (!edits.length) continue;
  edits.sort((a, b) => b.start - a.start);
  let u = t; for (const e of edits) u = u.slice(0, e.start) + e.text + u.slice(e.end);
  const re = ts.createSourceFile(f, u, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  if ((re as any).parseDiagnostics?.length) { console.error("PARSE ERROR after edit", relative(ROOT, f)); continue; }
  changed++; out.push(relative(ROOT, f));
  if (!dry) writeFileSync(f, u);
}
console.log((dry ? "would rewrite " : "rewrote ") + changed + " files, merged " + merged);
writeFileSync(process.env.OUT ?? "/dev/null", out.join("\n") + "\n");
