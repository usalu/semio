import { readFileSync } from "node:fs";
import { dirname, resolve, join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const S = process.argv[2];
const FAMILY = new Set((process.argv[3] ?? "semio-framework-2d,semio-framework-2d-compute").split(","));
const tomls = readFileSync(S + "/tomls.txt", "utf8").split("\n").filter(Boolean);
const locks = readFileSync(S + "/locks.txt", "utf8").split("\n").filter(Boolean).filter((p) => !p.startsWith(".🧬semio"));
type M = { path: string; name: string; doc: any; dir: string };
const manifests: M[] = [];
const byName = new Map<string, M[]>();
const wsRoots = new Map<string, any>();
for (const rel of tomls) {
  let doc: any;
  try { doc = Bun.TOML.parse(readFileSync(join(root, rel), "utf8")); } catch { continue; }
  const dir = dirname(join(root, rel));
  if (doc.workspace) wsRoots.set(dir, doc);
  if (doc.package?.name) { const m = { path: rel, name: doc.package.name, doc, dir }; manifests.push(m); if (!byName.has(m.name)) byName.set(m.name, []); byName.get(m.name)!.push(m); }
}
function wsOf(m: M): any {
  const w = m.doc.package?.workspace;
  if (typeof w === "string") return wsRoots.get(resolve(m.dir, w));
  let d = m.dir;
  while (d.length > root.length - 1) { if (wsRoots.has(d)) return wsRoots.get(d); d = dirname(d); }
}
function realDeps(m: M): Set<string> {
  const out = new Set<string>();
  const ws = wsOf(m);
  const tables: any[] = [];
  for (const k of ["dependencies", "dev-dependencies", "build-dependencies"]) { if (m.doc[k]) tables.push(m.doc[k]); }
  for (const t of Object.values<any>(m.doc.target ?? {})) for (const k of ["dependencies", "dev-dependencies", "build-dependencies"]) if (t[k]) tables.push(t[k]);
  for (const t of tables) for (const [k, v] of Object.entries<any>(t)) {
    let name = k;
    if (v && typeof v === "object") {
      if (v.package) name = v.package;
      else if (v.workspace === true) { const wd = ws?.workspace?.dependencies?.[k]; if (wd && typeof wd === "object" && wd.package) name = wd.package; }
    }
    out.add(name);
  }
  return out;
}
const report: string[] = [];
let ok = 0, bad = 0;
const perLock: Record<string, { addCompute: string[]; dropFacade: string[]; addFacade: string[]; dropCompute: string[]; noManifest: string[]; computeBlock: boolean; facadeBlock: boolean }> = {};
for (const lock of locks) {
  const text = readFileSync(join(root, lock), "utf8");
  const blocks = text.split("\n[[package]]\n").slice(1);
  const lockDir = dirname(join(root, lock));
  const r = { addCompute: [] as string[], dropFacade: [] as string[], addFacade: [] as string[], dropCompute: [] as string[], noManifest: [] as string[], computeBlock: false, facadeBlock: false };
  for (const b of blocks) {
    const name = /^name = "([^"]+)"/m.exec(b)?.[1]!;
    if (name === "semio-framework-2d-compute") r.computeBlock = true;
    if (name === "semio-framework-2d") r.facadeBlock = true;
    if (/^source = /m.test(b)) continue;
    const lockDeps = new Set([...b.matchAll(/^ "([^" ]+)(?: [^"]*)?",?$/gm)].map((x) => x[1]));
    const all = byName.get(name) ?? [];
    const near = all.filter((m) => m.dir.startsWith(lockDir + "/") || m.dir === lockDir);
    const cands = all.length === 1 ? all : near;
    if (cands.length !== 1) { if (lockDeps.has("semio-framework-2d") || lockDeps.has("semio-framework-2d-compute") || cands.length > 1) r.noManifest.push(name + ":" + cands.length); continue; }
    const md = realDeps(cands[0]);
    for (const f of FAMILY) {
      if (md.has(f) && !lockDeps.has(f)) (f.endsWith("compute") ? r.addCompute : r.addFacade).push(name);
      if (!md.has(f) && lockDeps.has(f)) (f.endsWith("compute") ? r.dropCompute : r.dropFacade).push(name);
    }
  }
  perLock[lock] = r;
  const dirty = r.addCompute.length + r.dropFacade.length + r.addFacade.length + r.dropCompute.length + r.noManifest.length > 0 || (r.addCompute.length > 0 && !r.computeBlock);
  if (dirty) bad++; else ok++;
}
console.log(JSON.stringify({ locks: locks.length, consistent: ok, inconsistent: bad }));
await Bun.write(S + "/lock-report.json", JSON.stringify(perLock, null, 1));
