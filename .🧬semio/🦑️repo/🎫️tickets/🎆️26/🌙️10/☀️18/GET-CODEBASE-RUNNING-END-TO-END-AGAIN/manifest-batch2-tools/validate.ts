/** 🔍 Validates the live workspace manifest graph: members exist, names unique, no alias specs, every workspace edge resolves, external specs match bun.lock, declared cycles. Usage: bun validate.ts [--dev] */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
const root = process.cwd();
const read = (p: string): any => JSON.parse(readFileSync(resolve(root, p), "utf8"));
const roots = ["package.json", ...readdirSync(root).filter(d => existsSync(resolve(root, d, "package.json")) && (JSON.parse(readFileSync(resolve(root, d, "package.json"), "utf8")).workspaces ?? []).length).map(d => `${d}/package.json`)];
const members = new Map<string, string>();
const problems: string[] = [];
for (const r of roots) {
  const dir = r === "package.json" ? "" : r.replace(/package\.json$/u, "");
  for (const w of read(r).workspaces ?? []) {
    const rel = resolve(root, dir, w);
    const pj = resolve(rel, "package.json");
    if (!existsSync(pj)) { problems.push(`member without manifest: ${r} -> ${w}`); continue; }
    members.set(pj, rel);
  }
}
const names = new Map<string, string>();
const manifests = new Map<string, any>();
for (const pj of members.keys()) {
  const m = JSON.parse(readFileSync(pj, "utf8"));
  manifests.set(pj, m);
  if (m.name) { if (names.has(m.name) && names.get(m.name) !== pj) problems.push(`duplicate name ${m.name}: ${names.get(m.name)} and ${pj}`); names.set(m.name, pj); }
}
const locks = roots.map(r => r.replace(/package\.json$/u, "") + "bun.lock").filter(l => existsSync(resolve(root, l))).map(l => JSON.parse(readFileSync(resolve(root, l), "utf8").replace(/,(\s*[}\]])/gu, "$1")));
const lockSpecs = new Map<string, Set<string>>();
for (const lock of locks) for (const ws of Object.values<any>(lock.workspaces)) for (const f of ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"]) for (const [k, v] of Object.entries<string>(ws[f] ?? {})) (lockSpecs.get(k) ?? lockSpecs.set(k, new Set()).get(k)!).add(v);
const fields = ["dependencies", "peerDependencies", "optionalDependencies", "devDependencies"] as const;
const edges = new Map<string, Set<string>>();
const prod = new Map<string, Set<string>>();
let ext = 0;
for (const [pj, m] of manifests) {
  const from = m.name ?? pj;
  for (const f of fields) for (const [k, v] of Object.entries<string>(m[f] ?? {})) {
    if (typeof v !== "string") continue;
    if (/^(npm|jsr|github|git|file|link):/u.test(v) || v.startsWith("http")) problems.push(`alias/non-registry spec ${pj} ${f}.${k}=${v}`);
    if (v.startsWith("workspace:")) {
      if (!names.has(k)) problems.push(`unresolved workspace dep ${from} -> ${k}`);
      (edges.get(from) ?? edges.set(from, new Set()).get(from)!).add(k);
      if (f !== "devDependencies") (prod.get(from) ?? prod.set(from, new Set()).get(from)!).add(k);
    } else {
      ext++;
      if (!lockSpecs.get(k)?.has(v)) problems.push(`external spec not in bun.lock: ${from} ${f}.${k}=${v} (lock has ${[...(lockSpecs.get(k) ?? [])].join("|") || "none"})`);
    }
  }
}
const sccs = (g: Map<string, Set<string>>): string[][] => {
  let idx = 0; const st: string[] = [], on = new Set<string>(), ix = new Map<string, number>(), low = new Map<string, number>(), out: string[][] = [];
  const nodes = new Set<string>([...g.keys(), ...[...g.values()].flatMap(s => [...s])]);
  const visit = (v: string): void => {
    ix.set(v, idx); low.set(v, idx++); st.push(v); on.add(v);
    for (const w of g.get(v) ?? []) { if (!ix.has(w)) { visit(w); low.set(v, Math.min(low.get(v)!, low.get(w)!)); } else if (on.has(w)) low.set(v, Math.min(low.get(v)!, ix.get(w)!)); }
    if (low.get(v) === ix.get(v)) { const c: string[] = []; let w: string; do { w = st.pop()!; on.delete(w); c.push(w); } while (w !== v); if (c.length > 1 || g.get(v)?.has(v)) out.push(c); }
  };
  for (const n of nodes) if (!ix.has(n)) visit(n);
  return out;
};
console.log(`members ${members.size}, names ${names.size}, external edges ${ext}`);
console.log(`production cycles: ${JSON.stringify(sccs(prod))}`);
if (process.argv.includes("--dev")) console.log(`all-edge cycles: ${JSON.stringify(sccs(edges).map(c => c.length > 12 ? [`${c.length} packages`, ...c.slice(0, 12)] : c))}`);
console.log(`problems: ${problems.length}`);
for (const p of problems.slice(0, 80)) console.log("- " + p);
process.exit(problems.length ? 1 : 0);
