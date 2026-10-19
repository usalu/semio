import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { ROOT, TICKET, OWNER, tracked, trackedSet, trackedDirs, projectFiles, readJson, PROJECT } from "./lib.ts";

const allProjectFiles = tracked.filter((f) => f.split("/").pop() === PROJECT && !f.startsWith(".🧬semio/") && !f.split("/").some((s) => ["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples", "pkg"].includes(s.replace(/️/g, "") === s ? s : s)));
const rootsByName = new Map<string, string>();
const nameByRoot = new Map<string, string>();
const manifests = new Map<string, any>();
for (const f of allProjectFiles) {
  let j: any; try { j = JSON.parse(readFileSync(join(ROOT, f), "utf8")); } catch { continue; }
  const root = dirname(f);
  if (!j.name) continue;
  rootsByName.set(j.name, root); nameByRoot.set(root, j.name); manifests.set(j.name, { ...j, __file: f, __root: root });
}
const ownerCache = new Map<string, string | null>();
export const ownerOfPath = (p: string): string | null => {
  if (ownerCache.has(p)) return ownerCache.get(p)!;
  let d = p; let res: string | null = null;
  for (;;) { if (nameByRoot.has(d)) { res = nameByRoot.get(d)!; break; } const i = d.lastIndexOf("/"); if (i < 0) break; d = d.slice(0, i); }
  ownerCache.set(p, res); return res;
};

const pkgNames = new Map<string, string>();
for (const [name, m] of manifests) {
  const pj = join(ROOT, m.__root, "package.json");
  if (existsSync(pj)) { try { const p = JSON.parse(readFileSync(pj, "utf8")); if (p.name) pkgNames.set(p.name, name); } catch {} }
}

const TESTDIR = /(^|\/)(🧪️tests|🧪️testing|🧫️fixtures|🧫️examples|🧪️fixtures|🧪️examples|\.storybook)(\/|$)/;
const importRe = /(?:\bfrom\s*|\bimport\s*\(\s*|\brequire\s*\(\s*|\bimport\s+)(["'])([^"'\n]+)\1/g;
const tsLike = /\.(ts|tsx|mts|cts|mjs|js|jsx)$/;
const importCache = new Map<string, string[]>();
export function importsOf(file: string): string[] {
  const hit = importCache.get(file); if (hit) return hit;
  let out: string[] = [];
  try { const src = readFileSync(join(ROOT, file), "utf8"); for (const m of src.matchAll(importRe)) out.push(m[2]); } catch {}
  importCache.set(file, out); return out;
}
export function resolveRel(from: string, spec: string): string | null {
  const base = join(dirname(from), spec).replace(/\\/g, "/");
  const norm = resolve("/", base).slice(1);
  for (const cand of [norm, norm + ".ts", norm + ".tsx", norm + ".mjs", norm + ".js", norm + "/🟦️.ts", norm + "/index.ts"]) if (trackedSet.has(cand)) return cand;
  return null;
}

const deps = new Map<string, Set<string>>();
const add = (a: string, b: string | null | undefined) => { if (!b || a === b) return; (deps.get(a) ?? deps.set(a, new Set()).get(a)!).add(b); };
for (const f of tracked) {
  if (!tsLike.test(f) || TESTDIR.test(f) || f.startsWith(".🧬semio/") || f.startsWith("node_modules/")) continue;
  const owner = ownerOfPath(f); if (!owner) continue;
  for (const spec of importsOf(f)) {
    if (spec.startsWith(".")) { const r = resolveRel(f, spec); if (r) add(owner, ownerOfPath(r)); }
    else { const pkg = spec.startsWith("@") ? spec.split("/").slice(0, 2).join("/") : spec.split("/")[0]; add(owner, pkgNames.get(pkg)); }
  }
}
for (const [name, m] of manifests) {
  const pj = join(ROOT, m.__root, "package.json");
  if (existsSync(pj)) { try { const p = JSON.parse(readFileSync(pj, "utf8")); for (const k of ["dependencies", "peerDependencies", "optionalDependencies", "devDependencies"]) for (const d of Object.keys(p[k] ?? {})) add(name, pkgNames.get(d)); } catch {} }
  for (const d of m.implicitDependencies ?? []) add(name, d);
  const cargoRoot = m.metadata?.nativeRoot ?? m.__root;
  const ct = join(ROOT, cargoRoot, "Cargo.toml");
  if (existsSync(ct)) {
    const text = readFileSync(ct, "utf8");
    let section = "";
    for (const line of text.split("\n")) {
      const h = line.match(/^\s*\[([^\]]+)\]/); if (h) { section = h[1]; continue; }
      if (!/dependencies/.test(section) || /dev-/.test(section)) continue;
      const pm = line.match(/path\s*=\s*"([^"]+)"/); if (pm) { const target = resolve("/", join(cargoRoot, pm[1])).slice(1); add(name, ownerOfPath(target + "/Cargo.toml")); }
    }
  }
}
export function depClosure(name: string): Set<string> {
  const seen = new Set<string>(); const stack = [name];
  while (stack.length) { const n = stack.pop()!; for (const d of deps.get(n) ?? []) if (!seen.has(d)) { seen.add(d); stack.push(d); } }
  return seen;
}

const SCAN = /Bun\.Glob|new Glob\(|readdirSync|readdir\(|git["'`]?,\s*["'`]ls-files|ls-files|listTrackedFiles|trackedFiles|walkFiles|walk\(|globSync|\.scan\(/;

export type Result = { name: string; root: string; scripts: string[]; closureSize: number; refs: string[]; extras: string[]; scanFlags: string[]; defaultDirs: string[] };
export function analyze(file: string): Result {
  const j = manifests.get(readJson(file).name)!;
  const root = dirname(file);
  const scripts = new Set<string>();
  const own = join(root, "📜️script.ts"); if (trackedSet.has(own)) scripts.add(own);
  for (const t of Object.values<any>(j.targets ?? {})) {
    const cmd: string = t.options?.command ?? t.command ?? "";
    const cwd: string = t.options?.cwd ?? root;
    for (const m of cmd.matchAll(/(?:^|\s)(?:bun\s+)?["']?((?:\.{1,2}\/|[^\s"']*\/)?[^\s"']*\.ts)["']?/g)) {
      const raw = m[1]; for (const base of [cwd, root, ""]) { const c = resolve("/", join(base, raw)).slice(1); if (trackedSet.has(c)) { scripts.add(c); break; } }
    }
    const cm = cmd.match(/--cwd\s+(\S+)/); if (cm) { const c = resolve("/", join(t.options?.cwd ?? root, cm[1].replace(/^["']|["']$/g, ""), "📜️script.ts")).slice(1); if (trackedSet.has(c)) scripts.add(c); }
  }
  const closure = new Set<string>(); const q = [...scripts];
  while (q.length) { const f = q.pop()!; if (closure.has(f)) continue; closure.add(f); if (!tsLike.test(f)) continue; for (const s of importsOf(f)) if (s.startsWith(".")) { const r = resolveRel(f, s); if (r) q.push(r); } }
  const projFiles = [...closure].filter((f) => ownerOfPath(f) === j.name || f.startsWith(root + "/"));
  const refs = new Set<string>(); const scanFlags = new Set<string>();
  const litRe = /(["'`])((?:\.{1,2}\/)[^"'`\n$]*?)\1/g;
  for (const f of projFiles) {
    let src = ""; try { src = readFileSync(join(ROOT, f), "utf8"); } catch { continue; }
    if (SCAN.test(src)) scanFlags.add(f);
    for (const m of src.matchAll(litRe)) {
      const spec = m[2]; const abs = resolve("/", join(dirname(f), spec)).slice(1);
      if (importsOf(f).includes(spec)) continue;
      if (root === abs || root.startsWith(abs + "/")) continue;
      if (trackedSet.has(abs) || trackedDirs.has(abs)) refs.add(abs);
    }
  }
  const touchedFiles = new Set<string>(); const dirsTouched = new Set<string>();
  for (const r of refs) { if (trackedSet.has(r)) touchedFiles.add(r); else dirsTouched.add(r); }
  const q2 = [...touchedFiles];
  while (q2.length) { const f = q2.pop()!; if (!tsLike.test(f)) continue; for (const s of importsOf(f)) if (s.startsWith(".")) { const r = resolveRel(f, s); if (r && !touchedFiles.has(r)) { touchedFiles.add(r); q2.push(r); } } }
  for (const f of touchedFiles) { if (!TESTDIR.test(f) || true) { let src = ""; try { src = readFileSync(join(ROOT, f), "utf8"); } catch {} if (SCAN.test(src) && !closure.has(f)) scanFlags.add(f); } }
  const self = j.name; const cov = depClosure(self);
  const defaultDirs: string[] = [];
  const dflt: string[] = j.namedInputs?.default ?? ["{projectRoot}/**/*"];
  for (const p of dflt) { if (typeof p !== "string" || p.startsWith("!")) continue; const m = p.match(/^\{(projectRoot|workspaceRoot)\}\/(.*?)(?:\/\*\*\/\*.*|\/\*\*.*)?$/); if (!m) continue; defaultDirs.push(m[1] === "projectRoot" ? (root + (m[2] && m[2] !== "**" ? "/" + m[2] : "")) : m[2]); }
  if (!j.namedInputs?.default) defaultDirs.push(root);
  const under = (p: string, d: string) => p === d || p.startsWith(d + "/");
  const covered = (p: string) => defaultDirs.some((d) => under(p, d)) || (ownerOfPath(p) !== null && (ownerOfPath(p) === self || cov.has(ownerOfPath(p)!)));
  const extras = new Set<string>();
  const A = closure;
  for (const f of touchedFiles) { if (A.has(f) || covered(f)) continue; const o = ownerOfPath(f); const r = o ? manifests.get(o).__root : dirname(f); extras.add(r); }
  for (const d of dirsTouched) { if (covered(d)) continue; extras.add(d); }
  return { name: self, root, scripts: [...scripts], closureSize: closure.size, refs: [...refs], extras: [...extras].sort(), scanFlags: [...scanFlags], defaultDirs };
}

if (import.meta.main) {
  const out: Result[] = [];
  for (const f of projectFiles) { const j = readJson(f); if (!Object.keys(j.targets ?? {}).length) continue; out.push(analyze(f)); }
  writeFileSync(join(TICKET, "🗑️generated", "cache-fw-analysis.json"), JSON.stringify(out, null, 1));
  console.log(out.length, "projects;", out.filter((r) => r.extras.length).length, "with extras;", out.filter((r) => r.scanFlags.length).length, "with scan flags");
}
