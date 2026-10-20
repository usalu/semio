/** ✅ Applies the delta to a scratch mirror of every touched file and checks it: apply guards, export targets, specifier resolution, imported names, dead glue, declared production cycles. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync } from "node:fs";
import { dirname, resolve } from "node:path";
import ts from "typescript";
import { applyPlan, type PlanJson } from "./apply-plan.ts";
import { GENERATED, ROOT, model, posix } from "./lib.ts";
import { originsOf } from "../exports-stage-tools/model.ts";
import { scan } from "../exports-stage-tools/scan.ts";
import { scc } from "./pkg-graph.ts";

const delta = JSON.parse(readFileSync(resolve(import.meta.dir, "../cycle-break-delta.json"), "utf8")) as PlanJson & { flags: unknown[] };
const mirror = resolve(GENERATED, "mirror");
rmSync(mirror, { recursive: true, force: true });
const touched = new Set<string>([...delta.manifestOps.map(o => o.packageJson), ...delta.edits.map(e => e.file), ...delta.fileOps.map(o => o.path)]);
let copied = 0;
for (const f of touched) { const src = resolve(ROOT, f); if (!existsSync(src)) continue; const dst = resolve(mirror, f); mkdirSync(dirname(dst), { recursive: true }); copyFileSync(src, dst); copied++; }
const problems: string[] = [];
const r = applyPlan(delta, mirror, { write: true });
for (const p of r.problems) problems.push("apply: " + p);

type Pkg = { name: string; dir: string; exports: Record<string, string>; deps: string[]; dev: string[] };
const pkgs = new Map<string, Pkg>();
for (const p of model.all) if (p.name) pkgs.set(p.name, { name: p.name, dir: p.dir, exports: p.exports as Record<string, string>, deps: Object.keys(p.deps), dev: Object.keys(p.dev) });
for (const pj of new Set(delta.manifestOps.map(o => o.packageJson))) {
  const f = resolve(mirror, pj); if (!existsSync(f)) continue;
  const j = JSON.parse(readFileSync(f, "utf8")); if (!j.name) continue;
  pkgs.set(j.name, { name: j.name, dir: posix(dirname(pj)) === "." ? "." : posix(dirname(pj)), exports: j.exports ?? {}, deps: Object.keys(j.dependencies ?? {}), dev: Object.keys(j.devDependencies ?? {}) });
}
for (const nf of delta.newFiles) if (nf.path.endsWith("package.json")) { const j = JSON.parse(nf.content); pkgs.set(j.name, { name: j.name, dir: posix(dirname(nf.path)), exports: j.exports ?? {}, deps: Object.keys(j.dependencies ?? {}), dev: Object.keys(j.devDependencies ?? {}) }); }
const moves = new Map(delta.fileOps.filter(o => o.op === "move").map(o => [o.path, o.to!]));
const created = new Set(delta.newFiles.map(f => f.path));
const deleted = new Set(delta.fileOps.filter(o => o.op === "delete").map(o => o.path));
const appended = new Set(delta.edits.filter(e => e.mode === "append").map(e => e.file));
const fileExists = (p: string): boolean => (existsSync(resolve(ROOT, p)) && !deleted.has(p)) || created.has(p) || [...moves.values()].includes(p);

const splitSpec = (spec: string): { name: string; key: string } | null => { const m = /^(@[^/]+\/[^/]+|[^@./][^/]*)(\/.*)?$/u.exec(spec.split("?")[0]!); return m ? { name: m[1]!, key: m[2] ? "." + m[2] : "." } : null; };
let checked = 0, nameChecks = 0;
for (const e of delta.edits) {
  if (e.mode !== "specifier" || !e.to.startsWith("@")) continue;
  const sp = splitSpec(e.to); if (!sp) continue;
  const pkg = pkgs.get(sp.name);
  if (!pkg) { problems.push(`unknown package in ${e.file}: ${e.to}`); continue; }
  const value = pkg.exports[sp.key];
  if (typeof value !== "string") { problems.push(`no export ${sp.key} in ${sp.name} (${e.file})`); continue; }
  const target = posix(resolve("/", pkg.dir, value)).slice(1);
  if (!fileExists(target)) { problems.push(`export ${sp.name}${sp.key.slice(1)} -> ${target} does not exist (${e.file})`); continue; }
  checked++;
  if (!/\.(ts|tsx|mts)$/u.test(target) || appended.has(target) || created.has(target) || !existsSync(resolve(ROOT, target))) continue;
  const text = existsSync(resolve(mirror, e.file)) ? readFileSync(resolve(mirror, e.file), "utf8") : "";
  const sf = ts.createSourceFile(e.file, text, ts.ScriptTarget.Latest, true, e.file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const origins = originsOf(target);
  if (!origins) continue;
  for (const st of sf.statements) {
    if (!(ts.isImportDeclaration(st) || ts.isExportDeclaration(st)) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier) || st.moduleSpecifier.text !== e.to) continue;
    const names: string[] = [];
    if (ts.isImportDeclaration(st) && st.importClause?.namedBindings && ts.isNamedImports(st.importClause.namedBindings)) for (const x of st.importClause.namedBindings.elements) names.push((x.propertyName ?? x.name).text);
    if (ts.isExportDeclaration(st) && st.exportClause && ts.isNamedExports(st.exportClause)) for (const x of st.exportClause.elements) names.push((x.propertyName ?? x.name).text);
    const star = [...origins.keys()].some(k => k.startsWith("*bare:"));
    for (const n of names) { nameChecks++; if (!origins.has(n) && !star) problems.push(`${e.file}: ${n} is not exported by ${target} (via ${e.to})`); }
  }
}
// dead glue
const delSet = new Set([...deleted]);
const editedSpecs = new Map<string, Set<string>>();
for (const e of delta.edits) (editedSpecs.get(e.file) ?? editedSpecs.set(e.file, new Set()).get(e.file)!).add(e.from);
for (const e of scan().edges) if (delSet.has(e.target) && !delSet.has(e.file) && !(editedSpecs.get(e.file)?.has(e.spec))) problems.push(`dead glue still imported: ${e.file} -> ${e.target}`);
// declared production cycles
const adj = new Map<string, Set<string>>();
for (const p of pkgs.values()) adj.set(p.name, new Set(p.deps.filter(d => pkgs.has(d))));
for (const o of delta.manifestOps) { if (o.field === "dependencies") { const pk = pkgs.get(o.package); if (pk && o.op === "add") (adj.get(pk.name) ?? adj.set(pk.name, new Set()).get(pk.name)!).add(o.key); } }
for (const o of delta.manifestOps) if (o.field === "dependencies" && o.op === "remove") adj.get(o.package)?.delete(o.key);
const comps = scc([...adj.keys()], adj).filter(c => c.length > 1);
console.log(`mirror: ${copied} files; specifiers checked ${checked}; imported names checked ${nameChecks}`);
console.log(`declared production dependency components after the delta: ${comps.map(c => c.length).join(",") || "none"}`);
for (const c of comps.slice(0, 6)) console.log("  cyclic:", c.slice(0, 14).join(" "), c.length > 14 ? "…" : "");
console.log(`problems: ${problems.length}`);
for (const p of problems.slice(0, 60)) console.log(" - " + p);

if (process.argv.includes("--dag")) {
  const names = [...adj.keys()];
  const comp = new Map<string, number>(); scc(names, adj).forEach((c, i) => c.forEach(n => comp.set(n, i)));
  const cadj = new Map<number, Set<number>>();
  for (const [a, bs] of adj) for (const b of bs) { const x = comp.get(a)!, y = comp.get(b)!; if (x !== y) (cadj.get(x) ?? cadj.set(x, new Set()).get(x)!).add(y); }
  const rank = new Map<number, number>();
  const rk = (c: number): number => { const have = rank.get(c); if (have !== undefined) return have; rank.set(c, 0); let r = 0; for (const d of cadj.get(c) ?? []) r = Math.max(r, rk(d) + 1); rank.set(c, r); return r; };
  const by = new Map<number, string[]>();
  for (const n of names) (by.get(rk(comp.get(n)!)) ?? by.set(rk(comp.get(n)!), []).get(rk(comp.get(n)!))!).push(n);
  const focus = new Set(JSON.parse(readFileSync(resolve(import.meta.dir, "carves.json"), "utf8")).map((c: { name: string }) => c.name));
  for (const [r, ns] of [...by].sort((a, b) => a[0] - b[0])) console.log(`rank ${r}: ${ns.length} packages; focus: ${ns.filter(n => focus.has(n)).join(" ") || "-"}`);
}
