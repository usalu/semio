import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const SDK = "🧰️framework/📦️packages/🟦️typescript";
const [edgesFile, orgFile, sdkFile, includeRe, excludeRe, mode] = process.argv.slice(2);
const write = mode === "--write";
const rows = fs.readFileSync(edgesFile, "utf8").trim().split("\n").map(l => JSON.parse(l));
const org = JSON.parse(fs.readFileSync(orgFile, "utf8"));
const sdk = JSON.parse(fs.readFileSync(sdkFile, "utf8"));
const include = new RegExp(includeRe), exclude = new RegExp(excludeRe || "$^");
const facades = sdk.map(([sub, f]) => ({ sub, file: f, spec: sub === "." ? "@semio-tech/framework" : "@semio-tech/framework" + sub.slice(1) }));
const rank = f => (f.sub === "." ? 2 : 0) + (/^\.\/(process|time-travel\/testing)/.test(f.sub) ? 1 : 0);
const byFile = new Map();
for (const r of rows) { if (!include.test(r.to) || exclude.test(r.to)) continue; const a = byFile.get(r.from) ?? []; a.push(r); byFile.set(r.from, a); }
const report = { changed: 0, files: 0, skipped: [] };
function facadesFor(to, name) { const o = (org[to] ?? {})[name]; if (o === undefined) return []; return facades.filter(f => (org[f.file] ?? {})[name] === o); }
for (const [from, edges] of byFile) {
  const abs = R + from; const text = fs.readFileSync(abs, "utf8");
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.Latest, true, abs.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const specTo = new Map(edges.map(e => [e.spec, e.to]));
  const edits = [];
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || (ts.isExportDeclaration(st) && st.moduleSpecifier))) continue;
    const spec = st.moduleSpecifier.text; const to = specTo.get(spec); if (!to) continue;
    const isImport = ts.isImportDeclaration(st);
    let declType = false, items = [];
    if (isImport) {
      const c = st.importClause; if (!c || c.name || !c.namedBindings || ts.isNamespaceImport(c.namedBindings)) { report.skipped.push(`${from}: ${spec} (default/namespace/side-effect import)`); continue; }
      declType = !!c.isTypeOnly; items = c.namedBindings.elements.map(e => ({ local: e.name.text, imported: (e.propertyName ?? e.name).text, type: !!e.isTypeOnly }));
    } else {
      if (!st.exportClause || !ts.isNamedExports(st.exportClause)) { report.skipped.push(`${from}: ${spec} (export *)`); continue; }
      declType = !!st.isTypeOnly; items = st.exportClause.elements.map(e => ({ local: e.name.text, imported: (e.propertyName ?? e.name).text, type: !!e.isTypeOnly }));
    }
    if (!items.length) { report.skipped.push(`${from}: ${spec} (empty)`); continue; }
    const cand = items.map(i => facadesFor(to, i.imported));
    const rest = items.filter((i, k) => cand[k].length === 0);
    if (rest.length && !process.env.PARTIAL) { report.skipped.push(`${from}: ${spec} (no facade for ${rest.map(i => i.imported).join(",")})`); continue; }
    if (rest.length === items.length) { report.skipped.push(`${from}: ${spec} (nothing resolvable)`); continue; }
    const itemsOk = items.filter((i, k) => cand[k].length > 0), candOk = cand.filter(c => c.length > 0);
    let groups = new Map();
    const full = facades.filter(f => candOk.every(c => c.includes(f))).sort((a, b) => rank(a) - rank(b));
    if (full.length) groups.set(full[0], itemsOk);
    else { const counts = new Map(); candOk.forEach(c => c.forEach(f => counts.set(f, (counts.get(f) ?? 0) + 1))); for (let k = 0; k < itemsOk.length; k++) { const best = [...candOk[k]].sort((a, b) => (counts.get(b) - counts.get(a)) || (rank(a) - rank(b)))[0]; const g = groups.get(best) ?? []; g.push(itemsOk[k]); groups.set(best, g); } }
    if (rest.length) { groups.set({ spec: JSON.stringify(spec).slice(1, -1), sub: "keep" }, rest); report.partial = (report.partial ?? 0) + 1; }
    const multiline = text.slice(st.getStart(), st.end).includes("\n");
    const indentMatch = multiline ? /\n([ \t]+)\S/.exec(text.slice(st.getStart(), st.end)) : null; const indent = indentMatch ? indentMatch[1] : "  ";
    const out = [];
    for (const [f, its] of groups) {
      const parts = its.map(i => (i.type && !declType ? "type " : "") + (i.imported === i.local ? i.imported : `${i.imported} as ${i.local}`));
      const head = isImport ? `import ${declType ? "type " : ""}` : `export ${declType ? "type " : ""}`;
      const body = multiline ? `{\n${parts.map(p => indent + p).join(",\n")}\n}` : `{ ${parts.join(", ")} }`;
      out.push(`${head}${body} from "${f.spec}";`);
    }
    edits.push({ start: st.getStart(), end: st.end, text: out.join("\n") });
    if (process.env.PREVIEW) console.log("   - " + text.slice(st.getStart(), st.end).replace(/\n\s*/g, " ").slice(0, 200) + "\n   + " + out.join("\n     ").replace(/\n\s+/g, " ").slice(0, 300));
  }
  if (!edits.length) continue;
  let next = text; for (const e of edits.sort((a, b) => b.start - a.start)) next = next.slice(0, e.start) + e.text + next.slice(e.end);
  report.changed += edits.length; report.files++;
  console.log(`${write ? "WRITE" : "DRY"} ${from.replace(/^.*?engine\//u, "E/")}: ${edits.length} edit(s)`);
  if (write) { const cur = fs.readFileSync(abs, "utf8"); if (cur !== text) { console.log("  CHANGED UNDER US, skipped:", from); continue; } fs.writeFileSync(abs, next); }
}
console.log(JSON.stringify({ files: report.files, edits: report.changed, skipped: report.skipped.length }));
for (const s of report.skipped) console.log("SKIP", s);
