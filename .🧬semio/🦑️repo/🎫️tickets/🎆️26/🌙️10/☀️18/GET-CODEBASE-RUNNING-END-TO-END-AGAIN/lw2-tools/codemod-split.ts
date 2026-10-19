import ts from "typescript";
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const FROM = new RegExp(process.env.FROM!, "u");
const NAMES = new Set(JSON.parse(process.env.NAMES!) as string[]);
const TO = process.env.TO!;
const roots = (process.env.ROOTS ?? "🧰️framework,✏️s,🌎️hub,♻️mit-bestand,.storybook").split(",");
const dry = process.env.DRY === "1";
const SKIP = new Set(["node_modules", "dist", "target", "pkg", ".cache", "🗑️generated", ".git", "📤️dist"]);
const files: string[] = [];
const walk = (d: string) => { let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; } for (const e of es) { if (SKIP.has(e.name)) continue; const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.(ts|tsx|mts)$/u.test(e.name) && !e.name.endsWith(".d.ts")) files.push(p); } };
for (const r of roots) walk(join(ROOT, r));
const elText = (e: ts.ImportSpecifier | ts.ExportSpecifier, forceType: boolean) => ((forceType || e.isTypeOnly) ? "type " : "") + (e.propertyName ? `${e.propertyName.text} as ${e.name.text}` : e.name.text);
let changed = 0; const out: string[] = [];
for (const f of files) {
  let t: string; try { t = readFileSync(f, "utf8"); } catch { continue; }
  let any = false; for (const n of NAMES) if (t.includes(n)) { any = true; break; } if (!any) continue;
  const sf = ts.createSourceFile(f, t, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits: { start: number; end: number; text: string }[] = [];
  const moved: string[] = [];
  let existingTo: ts.ImportDeclaration | undefined;
  for (const st of sf.statements) if (ts.isImportDeclaration(st) && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text === TO && st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings) && !st.importClause.name && !st.importClause.isTypeOnly) existingTo = st;
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || !FROM.test(st.moduleSpecifier.text)) continue;
    const isImp = ts.isImportDeclaration(st);
    const nb = isImp ? st.importClause?.namedBindings : st.exportClause;
    if (!nb || !(ts.isNamedImports(nb) || ts.isNamedExports(nb))) continue;
    const els = nb.elements as readonly (ts.ImportSpecifier | ts.ExportSpecifier)[];
    const hit = els.filter(e => NAMES.has((e.propertyName ?? e.name).text));
    if (!hit.length) continue;
    const declType = isImp ? st.importClause!.isTypeOnly : (st as ts.ExportDeclaration).isTypeOnly;
    for (const e of hit) moved.push(elText(e, declType));
    const keep = els.filter(e => !hit.includes(e));
    if (keep.length === 0 && !(isImp && st.importClause?.name)) edits.push({ start: st.getStart(sf), end: st.getEnd() + (t[st.getEnd()] === "\n" ? 1 : 0), text: "" });
    else edits.push({ start: nb.getStart(sf), end: nb.getEnd(), text: `{ ${keep.map(e => elText(e, false)).join(", ")} }` });
    if (!isImp) { /* re-exports are dropped, never re-pointed */ }
  }
  if (!moved.length) continue;
  const uniq = [...new Set(moved)];
  if (existingTo) {
    const nb = existingTo.importClause!.namedBindings as ts.NamedImports;
    const have = new Set(nb.elements.map(e => elText(e, false)));
    edits.push({ start: nb.getStart(sf), end: nb.getEnd(), text: `{ ${[...nb.elements.map(e => elText(e, false)), ...uniq.filter(u => !have.has(u))].join(", ")} }` });
  } else {
    const last = [...sf.statements].filter(ts.isImportDeclaration).pop();
    const at = last ? last.getEnd() : 0;
    edits.push({ start: at, end: at, text: `${last ? "\n" : ""}import { ${uniq.join(", ")} } from "${TO}";${last ? "" : "\n"}` });
  }
  edits.sort((a, b) => b.start - a.start);
  let u = t; for (const e of edits) u = u.slice(0, e.start) + e.text + u.slice(e.end);
  const re = ts.createSourceFile(f, u, ts.ScriptTarget.Latest, true, f.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  if ((re as any).parseDiagnostics?.length) { console.error("PARSE ERROR after edit", relative(ROOT, f)); continue; }
  changed++; out.push(relative(ROOT, f));
  if (!dry) writeFileSync(f, u);
}
console.log((dry ? "would rewrite " : "rewrote ") + changed + " files");
for (const o of out) console.log("  " + o.replace(/🧰️framework\/🛍️products\/💻️os\/🔨️modules\/📺️renderer\/🧑‍🎨engine\//u, "R/"));
