/** 🔁 Source-level workspace package graph (relative cross-package edges, bare workspace imports, declared workspace deps) and its strongly connected components. */
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { loadPackages } from "../exports-stage-tools/lib.ts";
import { lane } from "../exports-stage-tools/model.ts";

const policy = loadPolicy();
const pk = loadPackages();
const own = new Map<string, string | null>();
export const pkgOf = (f: string): string | null => {
  if (own.has(f)) return own.get(f)!;
  const s = ownerOfFile(policy, f);
  const r = s ? (lane(s.owner)?.name ?? s.names[0]!) : null;
  own.set(f, r);
  return r;
};
export type E = { from: string; to: string; kind: "rel" | "bare" | "decl"; file?: string; target?: string; spec?: string };
export function edges(): E[] {
  const { edges: rel, bare } = scan();
  const out: E[] = [];
  for (const e of rel) { const a = pkgOf(e.file), b = pkgOf(e.target); if (a && b && a !== b) out.push({ from: a, to: b, kind: "rel", file: e.file, target: e.target, spec: e.spec }); }
  for (const e of bare) { const a = pkgOf(e.file); if (a && e.pkg !== a && pk.byName.has(e.pkg)) out.push({ from: a, to: e.pkg, kind: "bare", file: e.file, spec: e.spec }); }
  for (const p of pk.all) for (const f of ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] as const) for (const [k, v] of Object.entries<string>(p.json[f] ?? {})) if (p.name && k !== p.name && pk.byName.has(k)) out.push({ from: p.name, to: k, kind: "decl" });
  return out;
}
export function scc(nodes: string[], adj: Map<string, Set<string>>): string[][] {
  let idx = 0; const st: string[] = [], on = new Set<string>(), I = new Map<string, number>(), L = new Map<string, number>(), res: string[][] = [];
  const go = (v: string): void => {
    I.set(v, idx); L.set(v, idx); idx++; st.push(v); on.add(v);
    for (const w of adj.get(v) ?? []) { if (!I.has(w)) { go(w); L.set(v, Math.min(L.get(v)!, L.get(w)!)); } else if (on.has(w)) L.set(v, Math.min(L.get(v)!, I.get(w)!)); }
    if (L.get(v) === I.get(v)) { const c: string[] = []; let w: string; do { w = st.pop()!; on.delete(w); c.push(w); } while (w !== v); res.push(c); }
  };
  for (const n of nodes) if (!I.has(n)) go(n);
  return res;
}
if (import.meta.main) {
  const es = edges();
  const adj = new Map<string, Set<string>>();
  const nodes = new Set<string>();
  const kindsBy = new Map<string, Map<string, number>>();
  for (const e of es) { nodes.add(e.from); nodes.add(e.to); (adj.get(e.from) ?? adj.set(e.from, new Set()).get(e.from)!).add(e.to); const k = e.from + "\t" + e.to; const m = kindsBy.get(k) ?? kindsBy.set(k, new Map()).get(k)!; m.set(e.kind, (m.get(e.kind) ?? 0) + 1); }
  const comps = scc([...nodes], adj).filter(c => c.length > 1).sort((a, b) => b.length - a.length);
  console.log("packages", nodes.size, "edges", es.length, "cyclic components", comps.length);
  for (const c of comps) {
    const set = new Set(c);
    console.log("\nSCC size", c.length);
    const rows: string[] = [];
    for (const a of c) for (const b of adj.get(a) ?? []) if (set.has(b)) { const m = kindsBy.get(a + "\t" + b)!; rows.push(`  ${a} -> ${b}  ${[...m].map(([k, n]) => k + ":" + n).join(" ")}`); }
    console.log(rows.join("\n"));
  }
  writeFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/graph.json"), JSON.stringify({ edges: es }));
}
