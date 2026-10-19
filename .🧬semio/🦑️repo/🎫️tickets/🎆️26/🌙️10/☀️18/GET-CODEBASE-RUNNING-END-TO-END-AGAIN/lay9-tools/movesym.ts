import ts from "typescript";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, resolve, relative } from "node:path";
import { execSync } from "node:child_process";
const ROOT = "/Users/ueli/Documents/semio/";
type Move = { from: string; to: string; names: string[] };
const plan: Move[] = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const dry = process.argv[3] === "dry";
const names0 = [...new Set(plan.flatMap((m) => m.names))];
const list = execSync(`/usr/bin/grep -rlE '${names0.join("|")}' --include='*.ts' --include='*.tsx' --exclude-dir=node_modules --exclude-dir=target --exclude-dir=.cache --exclude-dir=dist --exclude-dir=pkg ${process.env.SCOPE ?? "🧰️framework/🛍️products/💻️os"} 2>/dev/null || true`, { cwd: ROOT, maxBuffer: 1 << 28 }).toString().split("\n").filter(Boolean);
const exts = ["", ".ts", ".tsx", "/🟦️.ts", "/🟦️.tsx"];
const resolveSpec = (from: string, spec: string): string | null => {
  if (!spec.startsWith(".")) return null;
  const base = resolve(dirname(ROOT + from), spec);
  for (const e of exts) { const c = base + e; if (existsSync(c) && !c.endsWith("/")) { try { if (require("node:fs").statSync(c).isFile()) return c.slice(ROOT.length); } catch {} } }
  return null;
};
const rel = (fromFile: string, toFile: string) => { let r = relative(dirname(ROOT + fromFile), ROOT + toFile); if (!r.startsWith(".")) r = "./" + r; return r; };
const allNames = new Set(plan.flatMap((m) => m.names));
let touched = 0;
for (const f of list) {
  const text = readFileSync(ROOT + f, "utf8");
  if (![...allNames].some((n) => text.includes(n))) continue;
  const sf = ts.createSourceFile(f, text, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits: { start: number; end: number; text: string }[] = [];
  const decls = sf.statements.filter((s): s is ts.ImportDeclaration | ts.ExportDeclaration => (ts.isImportDeclaration(s) || ts.isExportDeclaration(s)) && !!s.moduleSpecifier && ts.isStringLiteral(s.moduleSpecifier));
  const addTo = new Map<string, { type: string[]; value: string[]; anchor: ts.Node }>();
  const consumed = new Set<ts.Node>();
  for (const mv of plan) {
    for (const d of decls) {
      const spec = (d.moduleSpecifier as ts.StringLiteral).text;
      if (resolveSpec(f, spec) !== mv.from) continue;
      if (f === mv.from) continue;
      const clause = ts.isImportDeclaration(d) ? d.importClause?.namedBindings : d.exportClause;
      if (!clause || !(ts.isNamedImports(clause) || ts.isNamedExports(clause))) continue;
      const els = clause.elements as ts.NodeArray<ts.ImportSpecifier | ts.ExportSpecifier>;
      const moving = els.filter((e) => mv.names.includes((e.propertyName ?? e.name).text));
      if (!moving.length) continue;
      const declType = ts.isImportDeclaration(d) ? !!d.importClause?.isTypeOnly : d.isTypeOnly;
      const key = (ts.isExportDeclaration(d) ? "E:" : "I:") + mv.to;
      const slot = addTo.get(key) ?? { type: [], value: [], anchor: d };
      for (const e of moving) { const t = e.getText(sf).replace(/^type\s+/, ""); (declType || e.isTypeOnly ? slot.type : slot.value).push(t); }
      addTo.set(key, slot);
      const rest = els.filter((e) => !moving.includes(e));
      const hasDefault = ts.isImportDeclaration(d) && !!d.importClause?.name;
      if (!rest.length && !hasDefault) { let end = d.getEnd(); if (text[end] === "\n") end++; edits.push({ start: d.getStart(sf), end, text: "" }); }
      else edits.push({ start: clause.getStart(sf), end: clause.getEnd(), text: "{ " + rest.map((e) => e.getText(sf)).join(", ") + " }" });
    }
  }
  if (!edits.length) continue;
  const inserts: { pos: number; text: string }[] = [];
  for (const [key, slot] of addTo) {
    const to = key.slice(2); const isExport = key[0] === "E";
    if (to === f) continue;
    const specText = rel(f, to);
    const parts = [...slot.value, ...slot.type.map((t) => "type " + t)];
    const kw = isExport ? "export" : "import";
    const existing = decls.find((d) => ts.isImportDeclaration(d) === !isExport && resolveSpec(f, (d.moduleSpecifier as ts.StringLiteral).text) === to && (ts.isImportDeclaration(d) ? !d.importClause?.isTypeOnly && d.importClause?.namedBindings && ts.isNamedImports(d.importClause.namedBindings) : false));
    if (existing && ts.isImportDeclaration(existing)) {
      const nb = existing.importClause!.namedBindings as ts.NamedImports;
      edits.push({ start: nb.getStart(sf), end: nb.getEnd(), text: "{ " + [...nb.elements.map((e) => e.getText(sf)), ...parts].join(", ") + " }" });
    } else inserts.push({ pos: slot.anchor.getEnd(), text: `\n${kw} { ${parts.join(", ")} } from "${specText}";` });
  }
  let out = text;
  const all = [...edits, ...inserts.map((i) => ({ start: i.pos, end: i.pos, text: i.text }))].sort((a, b) => b.start - a.start || b.end - a.end);
  for (const e of all) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  touched++;
  console.log(dry ? "would edit" : "edit", f);
  if (!dry) writeFileSync(ROOT + f, out);
}
console.log("files", touched);
