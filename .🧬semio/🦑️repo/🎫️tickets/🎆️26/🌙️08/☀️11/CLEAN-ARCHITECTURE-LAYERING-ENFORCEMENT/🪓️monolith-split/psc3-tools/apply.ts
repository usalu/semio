/** 🧱️ Writes the planned facades, direct exports and new packages. Dry-run by default; re-reads every file right before writing.
 * Usage: bun apply.ts <plan.json> [--write] [--snapshot <out.json>]
 */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { ROOT } from "../sdkb-tools/lib.ts";
import { nfc } from "./model.ts";

const args = process.argv.slice(2);
const planPath = args[0];
const write = args.includes("--write");
const snapIdx = args.indexOf("--snapshot");
const snapshotPath = snapIdx >= 0 ? args[snapIdx + 1] : undefined;
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const snapshot: Record<string, string | null> = {};

const emojiOf = (seg: string) => seg.match(/^(?:\p{Extended_Pictographic}(?:\u{FE0F}|\u{200D}\p{Extended_Pictographic}\u{FE0F}?)*)/u)?.[0] ?? "🔌️";

interface Stmt { module: string; typeOnly: boolean; names: Map<string, string>; side: boolean }
function parseExisting(abs: string): { header: string; stmts: Stmt[]; unknown: string[] } {
  const text = readFileSync(abs, "utf8");
  const sf = ts.createSourceFile(abs, text, ts.ScriptTarget.ESNext, true);
  const stmts: Stmt[] = [], unknown: string[] = [];
  let header = "";
  const lead = text.match(/^\s*\/\*\*[\s\S]*?\*\/\s*\n/u);
  if (lead) header = lead[0].trimEnd() + "\n";
  for (const st of sf.statements) {
    if (ts.isExportDeclaration(st) && st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && st.exportClause && ts.isNamedExports(st.exportClause)) {
      const names = new Map<string, string>();
      for (const el of st.exportClause.elements) names.set(el.name.text, (el.propertyName ?? el.name).text);
      stmts.push({ module: resolve(dirname(abs), st.moduleSpecifier.text), typeOnly: st.isTypeOnly, names, side: false });
    } else if (ts.isImportDeclaration(st) && !st.importClause && ts.isStringLiteral(st.moduleSpecifier)) {
      stmts.push({ module: resolve(dirname(abs), st.moduleSpecifier.text), typeOnly: false, names: new Map(), side: true });
    } else unknown.push(st.getText(sf));
  }
  return { header, stmts, unknown };
}

function render(facadeRel: string, header: string, stmts: Stmt[], unknown: string[]): string {
  const rel = (m: string) => { const r = relative(join(ROOT, dirname(facadeRel)), m).normalize("NFC"); return r.startsWith(".") ? r : "./" + r; };
  const byModule = new Map<string, Stmt[]>();
  for (const s of stmts) { const a = byModule.get(s.module) ?? []; a.push(s); byModule.set(s.module, a); }
  const lines: string[] = [];
  for (const m of [...byModule.keys()].sort((a, b) => rel(a) < rel(b) ? -1 : 1)) {
    const group = byModule.get(m)!;
    if (group.some(g => g.side)) lines.push(`import "${rel(m)}";`);
    for (const typeOnly of [false, true]) {
      const names = new Map<string, string>();
      for (const g of group.filter(x => !x.side && x.typeOnly === typeOnly)) for (const [exp, loc] of g.names) names.set(exp, loc);
      if (!names.size) continue;
      const items = [...names].sort((a, b) => (a[0] < b[0] ? -1 : 1)).map(([exp, loc]) => (exp === loc ? exp : `${loc} as ${exp}`));
      lines.push(`export ${typeOnly ? "type " : ""}{ ${items.join(", ")} } from "${rel(m)}";`);
    }
  }
  return header + [...lines, ...unknown].join("\n") + "\n";
}

let created = 0, extended = 0, unchanged = 0, newExports = 0;
const touched: string[] = [], collisions: string[] = [], pkgChanges: string[] = [], unhandled: string[] = [];
const savedBefore = (rel: string, text: string | null) => { if (!(rel in snapshot)) snapshot[rel] = text; };

const byPkg = new Map<string, { dir: string; adds: { subpath: string; target: string }[] }>();
const note = (pkg: string, dir: string, subpath: string, target: string) => { const v = byPkg.get(pkg) ?? { dir, adds: [] }; v.adds.push({ subpath, target }); byPkg.set(pkg, v); };

for (const f of plan.facades) {
  const abs = join(ROOT, f.facade);
  const exists = existsSync(abs);
  const cur = exists ? parseExisting(abs) : { header: "", stmts: [] as Stmt[], unknown: [] as string[] };
  const before = exists ? readFileSync(abs, "utf8") : "";
  const stmts = cur.stmts.map(s => ({ ...s, names: new Map(s.names) }));
  const exportedNames = new Map<string, string>();
  for (const s of stmts) for (const n of s.names.keys()) exportedNames.set(n, s.module);
  for (const a of f.adds) {
    const mod = join(ROOT, a.from);
    const vals = new Map<string, string>(), typs = new Map<string, string>();
    for (const n of a.names) {
      if (exportedNames.has(n.name) && exportedNames.get(n.name) !== mod) { collisions.push(`${f.subpath}: ${n.name} from ${exportedNames.get(n.name)} and ${a.from}`); continue; }
      exportedNames.set(n.name, mod);
      (n.value ? vals : typs).set(n.name, n.name);
    }
    if (vals.size) stmts.push({ module: mod, typeOnly: false, names: vals, side: false });
    if (typs.size) stmts.push({ module: mod, typeOnly: true, names: typs, side: false });
    if (a.sideEffect) stmts.push({ module: mod, typeOnly: false, names: new Map(), side: true });
    for (const j of a.json) {
      const jmod = join(ROOT, j.file);
      if (exportedNames.has(j.exportName) && exportedNames.get(j.exportName) !== jmod) { collisions.push(`${f.subpath}: json ${j.exportName} duplicate`); continue; }
      exportedNames.set(j.exportName, jmod);
      stmts.push({ module: jmod, typeOnly: false, names: new Map([[j.exportName, "default"]]), side: false });
    }
  }
  let header = cur.header;
  if (!exists) {
    const segs = f.facade.split("/");
    header = `/** ${emojiOf(segs[segs.length - 2])} Public entry \`${f.pkg}${f.subpath.slice(1)}\`: the explicit surface that importers outside the module use. */\n`;
  }
  const next = render(f.facade, header, stmts, cur.unknown);
  if (next === before) unchanged++;
  else {
    if (write) { savedBefore(f.facade, exists ? before : null); mkdirSync(dirname(abs), { recursive: true }); writeFileSync(abs, next); }
    if (exists) extended++; else created++;
    touched.push(f.facade);
  }
  note(f.pkg, f.pkgDir, f.subpath, f.facade);
}
for (const d of plan.directExports) note(d.pkg, d.pkgDir, d.subpath, d.target);

for (const np of plan.newPackages ?? []) {
  const pj = join(ROOT, np.dir, "package.json");
  if (existsSync(pj)) continue;
  if (write) { savedBefore(np.dir + "/package.json", null); mkdirSync(dirname(pj), { recursive: true }); writeFileSync(pj, JSON.stringify({ name: np.name, private: true, type: "module", exports: {} }, null, 2) + "\n"); }
  pkgChanges.push(`${np.dir}/package.json (new package ${np.name})`);
}

const KEY_AFTER = ["name", "version", "description", "type", "private"];
for (const [pkg, info] of byPkg) {
  const pkgPath = join(ROOT, info.dir, "package.json");
  const text = existsSync(pkgPath) ? readFileSync(pkgPath, "utf8") : JSON.stringify({ name: pkg, private: true, type: "module", exports: {} }, null, 2) + "\n";
  const manifest = JSON.parse(text);
  const stable = JSON.stringify(manifest, null, 2) + (text.endsWith("\n") ? "\n" : "");
  if (existsSync(pkgPath) && stable !== text) { unhandled.push(`${info.dir}/package.json is not round-trip stable; exports not written for ${pkg}`); continue; }
  const raw = manifest.exports;
  const exp: Record<string, unknown> = raw === undefined ? {} : typeof raw === "string" ? { ".": raw } : { ...raw };
  const existingKeys = Object.keys(exp);
  const added: string[] = [];
  for (const a of info.adds) {
    const target = "./" + nfc(a.target).slice(nfc(info.dir).length + 1);
    if (!(a.subpath in exp)) { exp[a.subpath] = target; added.push(a.subpath); newExports++; }
    else if (exp[a.subpath] !== target) collisions.push(`exports ${pkg} ${a.subpath}: ${JSON.stringify(exp[a.subpath])} vs ${target}`);
  }
  if (!added.length) continue;
  const ordered = [...existingKeys, ...[...new Set(added)].sort((a, b) => (a === "." ? -1 : b === "." ? 1 : Buffer.from(a).compare(Buffer.from(b))))];
  const nextExports = Object.fromEntries(ordered.map(k => [k, exp[k]]));
  let nextManifest: Record<string, unknown>;
  if (raw !== undefined) nextManifest = { ...manifest, exports: nextExports };
  else {
    const entries = Object.entries(manifest).filter(([k]) => k !== "exports");
    let at = 0;
    entries.forEach(([k], i) => { if (KEY_AFTER.includes(k)) at = i + 1; });
    entries.splice(at, 0, ["exports", nextExports]);
    nextManifest = Object.fromEntries(entries);
  }
  const out = JSON.stringify(nextManifest, null, 2) + (text.endsWith("\n") ? "\n" : "");
  pkgChanges.push(`${info.dir}/package.json (${added.length} new exports)`);
  if (write) { savedBefore(info.dir + "/package.json", existsSync(pkgPath) ? text : null); mkdirSync(dirname(pkgPath), { recursive: true }); writeFileSync(pkgPath, out); }
}
if (write && snapshotPath) writeFileSync(snapshotPath, JSON.stringify(snapshot));
console.log(`${write ? "WROTE" : "DRY"}: facadesCreated=${created} facadesExtended=${extended} unchanged=${unchanged} newExports=${newExports} collisions=${collisions.length} unhandled=${unhandled.length}`);
for (const c of collisions) console.log("COLLISION", c);
for (const u of unhandled) console.log("UNHANDLED", u);
for (const p of pkgChanges) console.log("PKG", p.replace("🧰️framework/🔨️modules/", ""));
