/** 🧱️ Writes the planned facades and the explicit `exports` entries of the owning packages. Dry-run by default.
 * Usage: bun apply.ts <plan.json> [--write] [--owners value,validator,schema]
 */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { ROOT } from "../sdkb-tools/lib.ts";
import { nfc, PKG_NAME, type OwnerKey } from "./model.ts";

const args = process.argv.slice(2);
const planPath = args[0];
const write = args.includes("--write");
const ownersArg = args.indexOf("--owners");
const wanted = new Set<string>(ownersArg >= 0 ? args[ownersArg + 1].split(",") : ["value", "validator", "schema"]);
const plan = JSON.parse(readFileSync(planPath, "utf8"));

const emojiOf = (seg: string) => seg.match(/^(?:\p{Extended_Pictographic}(?:\u{FE0F}|\u{200D}\p{Extended_Pictographic}\u{FE0F}?)*)/u)?.[0] ?? "🔌️";
const relImport = (facade: string, target: string) => { const r = relative(join(ROOT, dirname(facade)), join(ROOT, target)).normalize("NFC"); return r.startsWith(".") ? r : "./" + r; };

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
const touched: string[] = [], collisions: string[] = [];
const byOwner = new Map<string, any[]>();
for (const f of plan.facades) {
  if (!wanted.has(f.owner)) continue;
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
    const emoji = emojiOf(segs[segs.length - 2]);
    header = `/** ${emoji} Public entry \`${PKG_NAME[f.owner as OwnerKey]}${f.subpath.slice(1)}\`: the explicit surface that importers outside the module use. */\n`;
  }
  const next = render(f.facade, header, stmts, cur.unknown);
  if (next === before) { unchanged++; }
  else {
    if (write) { mkdirSync(dirname(abs), { recursive: true }); writeFileSync(abs, next); }
    if (exists) extended++; else created++;
    touched.push(f.facade);
  }
  const l = byOwner.get(f.owner) ?? []; l.push(f); byOwner.set(f.owner, l);
}

const pkgChanges: string[] = [];
for (const [owner, facades] of byOwner) {
  const dir = plan.owners[owner].dir;
  const pkgPath = join(ROOT, dir, "package.json");
  let pkg: any;
  if (existsSync(pkgPath)) pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
  else if (owner === "schema") pkg = { name: PKG_NAME.schema, private: true, type: "module", exports: {} };
  else throw new Error("missing package.json " + pkgPath);
  const text = existsSync(pkgPath) ? readFileSync(pkgPath, "utf8") : "";
  const exp: Record<string, string> = { ...(pkg.exports ?? {}) };
  const existingKeys = Object.keys(exp);
  for (const f of facades) {
    const target = "./" + f.facade.slice(dir.length + 1);
    if (!(f.subpath in exp)) { exp[f.subpath] = target; newExports++; }
    else if (exp[f.subpath] !== target) collisions.push(`exports ${owner} ${f.subpath}: ${exp[f.subpath]} vs ${target}`);
  }
  const newKeys = Object.keys(exp).filter(k => !existingKeys.includes(k)).sort((a, b) => (a === "." ? -1 : b === "." ? 1 : a < b ? -1 : 1));
  pkg.exports = Object.fromEntries([...existingKeys.map(k => [k, exp[k]]), ...newKeys.map(k => [k, exp[k]])]);
  const out = JSON.stringify(pkg, null, 2) + "\n";
  if (out !== text) { pkgChanges.push(`${dir}/package.json (${newKeys.length} new exports)`); if (write) { mkdirSync(dirname(pkgPath), { recursive: true }); writeFileSync(pkgPath, out); } }
}
console.log(`${write ? "WROTE" : "DRY"}: facadesCreated=${created} facadesExtended=${extended} unchanged=${unchanged} newExports=${newExports} collisions=${collisions.length}`);
for (const c of collisions) console.log("COLLISION", c);
for (const p of pkgChanges) console.log("PKG", p.replace("🧰️framework/🔨️modules/", ""));
writeFileSync(join(dirname(planPath), "psc-apply-touched.json"), JSON.stringify({ facades: touched, packages: pkgChanges }, null, 1));
