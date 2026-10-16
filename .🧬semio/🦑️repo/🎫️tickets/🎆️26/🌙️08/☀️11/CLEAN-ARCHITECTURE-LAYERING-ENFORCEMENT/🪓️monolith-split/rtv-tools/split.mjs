import { load } from "./graph.mjs";
import { mkdirSync, writeFileSync, existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";

const [parent, cfgPath, outRootArg] = process.argv.slice(2);
const cfg = (await import(cfgPath)).default;
const parentFile = process.env.RTV_PARENT_FILE ?? parent;
const parentDir = process.env.RTV_PARENT_DIR ?? dirname(parentFile);
const outRoot = outRootArg ?? parentDir;
const ext = cfg.ext ?? ".ts";
const { tops, text, lines, sf, regions } = load(parent);
tops.forEach((t, i) => { t.index = i; });
const modByRegion = new Map(), modByName = new Map(), modDefs = new Map();
for (const m of cfg.modules) { modDefs.set(m.name, m); for (const r of m.regions ?? []) modByRegion.set(r, m.name); for (const n of m.names ?? []) modByName.set(n, m.name); }
const assign = new Map(); const problems = [];
for (const t of tops) {
  if (t.isImport) continue;
  if (t.names.some((n) => (cfg.drop ?? []).includes(n))) continue;
  const byName = t.names.map((n) => modByName.get(n)).find(Boolean);
  const mod = byName ?? modByRegion.get(t.region);
  if (!mod) { problems.push(`unassigned ${t.names[0] ?? t.kind}@${t.region}:${t.startLine}`); continue; }
  assign.set(t, mod);
}
if (problems.length) { console.error(problems.join("\n")); process.exit(1); }
const modules = cfg.modules.map((m) => m.name);
const folderOf = (name) => modDefs.get(name).folder;
const modFile = (name) => (modDefs.get(name).isParent ? parentFile : join(outRoot, folderOf(name), "🟦️" + ext));
const specFor = (fromMod, toMod) => { const rel = relative(dirname(modFile(fromMod)), modFile(toMod)); return rel.startsWith(".") ? rel : "./" + rel; };

const needExport = new Set();
const childImports = new Map();
for (const [t, tm] of assign) for (const [name, ref] of t.refs) {
  const dm = assign.get(ref.top);
  if (!dm || dm === tm) continue;
  if (!ref.top.exported) needExport.add(ref.top);
  const key = tm; if (!childImports.has(key)) childImports.set(key, new Map());
  const per = childImports.get(key); if (!per.has(dm)) per.set(dm, new Map());
  const names = per.get(dm); const prev = names.get(name);
  names.set(name, { typeOnly: (prev?.typeOnly ?? true) && ref.typeOnly, isType: ref.top.isType });
}
const importDecls = tops.filter((t) => t.isImport);
const rewriteSpec = (spec, toMod) => { if (!spec.startsWith(".")) return spec; const abs = resolve(parentDir, spec); const rel = relative(dirname(modFile(toMod)), abs); return rel.startsWith(".") ? rel : "./" + rel; };
const externalImportsFor = (mod) => {
  const used = new Map();
  for (const [t, tm] of assign) if (tm === mod) for (const [binding, use] of t.importUses) { const p = used.get(binding); used.set(binding, { typeOnly: (p?.typeOnly ?? true) && use.typeOnly }); }
  const out = [];
  for (const imp of importDecls) {
    const node = imp.st; const clause = node.importClause; if (!clause) continue;
    const spec = node.moduleSpecifier.text;
    const parts = []; let def = null, ns = null;
    if (clause.name && used.has(clause)) def = clause.name.text;
    if (clause.namedBindings) {
      if (ts.isNamespaceImport(clause.namedBindings)) { if (used.has(clause.namedBindings)) ns = clause.namedBindings.name.text; }
      else for (const el of clause.namedBindings.elements) if (used.has(el)) parts.push(el.getText(sf));
    }
    if (!def && !ns && parts.length === 0) continue;
    const typeKw = clause.isTypeOnly ? "type " : "";
    const attrs = node.attributes ? " " + node.attributes.getText(sf) : "";
    const target = JSON.stringify(rewriteSpec(spec, mod));
    if (ns) out.push(`import ${typeKw}* as ${ns} from ${target}${attrs};`);
    else out.push(`import ${typeKw}${[def, parts.length ? `{ ${parts.join(", ")} }` : null].filter(Boolean).join(", ")} from ${target}${attrs};`);
  }
  return out;
};
const adaptersRegion = regions.find((r) => r.name === "🔌️Adapters");
const emitStatement = (t) => {
  const startOffset = t.spanPos; const endOffset = t.st.end;
  let body = text.slice(startOffset, endOffset);
  if (needExport.has(t) && !t.exported) { const at = t.st.getStart() - startOffset; body = body.slice(0, at) + "export " + body.slice(at); }
  return body;
};
const files = new Map();
const stats = { modules: {} };
for (const mod of modules) {
  const def = modDefs.get(mod);
  const stmts = [...assign].filter(([, m]) => m === mod).map(([t]) => t).sort((a, b) => a.startLine - b.startLine);
  if (stmts.length === 0) { console.error("empty module", mod); process.exit(1); }
  const out = [];
  if (def.isParent) {
    const hs = regions.find((r) => /Header/u.test(r.name));
    if (hs) { out.push(...lines.slice(hs.start - 1, hs.end)); out.push(``); }
  } else {
    out.push(`/** ${def.emoji} ${def.title}`);
    out.push(` *`);
    for (const l of def.doc) out.push(` * ${l}`);
    out.push(` */`);
    out.push(``);
  }
  const ext1 = externalImportsFor(mod);
  const kids = [];
  const per = childImports.get(mod);
  if (per) for (const target of modules) if (per.has(target)) {
    const names = [...per.get(target)].sort((a, b) => a[0].localeCompare(b[0])).map(([n, v]) => (v.typeOnly || v.isType ? "type " : "") + n);
    kids.push(`import { ${names.join(", ")} } from ${JSON.stringify(specFor(mod, target))};`);
  }
  if (ext1.length + kids.length) {
    if (adaptersRegion) out.push(adaptersRegion.startText);
    out.push(...ext1, ...kids);
    if (adaptersRegion) out.push(adaptersRegion.endText);
    out.push(``);
  }
  if (def.extra) { out.push(def.extra.trimEnd(), ``); }
  const stack = [];
  let prev = null;
  const same = (a, b) => a.start === b.start;
  for (const t of stmts) {
    const path = t.path;
    let common = 0; while (common < stack.length && common < path.length && same(stack[common], path[common])) common++;
    const closing = stack.length > common;
    while (stack.length > common) { const r = stack.pop(); out.push(r.endText); }
    const opening = path.length > common;
    if (prev) { if (closing || opening) out.push(``); else { const gap = Math.max(0, t.startLine - prev.endLine - 1); for (let i = 0; i < (prev.index + 1 === t.index ? gap : 1); i++) out.push(``); } }
    for (let i = common; i < path.length; i++) { out.push(path[i].startText); stack.push(path[i]); }
    out.push(emitStatement(t));
    prev = t;
  }
  while (stack.length) out.push(stack.pop().endText);
  out.push(``);
  const content = out.join("\n");
  files.set(modFile(mod), content);
  stats.modules[mod] = { lines: content.split("\n").length - 1, statements: stmts.length };
}
if (process.env.RTV_WRITE === "1") {
  for (const [path, content] of files) { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); }
  console.log("written", files.size);
}
const exportMap = new Map();
for (const [t, m] of assign) if (t.exported) for (const n of t.names) exportMap.set(n, { module: m, isType: t.isType });
const info = { exports: [...exportMap].map(([name, v]) => ({ name, ...v })), needExport: [...needExport].map((t) => ({ name: t.names[0], module: assign.get(t), line: t.codeLine })), stats, order: modules };
writeFileSync(process.env.RTV_INFO ?? "/dev/null", JSON.stringify(info, null, 1));
console.log(JSON.stringify(stats.modules));
console.log("needExport", needExport.size, [...needExport].map((t) => t.names[0]).join(" "));
console.log("exports", exportMap.size);
