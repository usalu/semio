/** 🧪 Simulates package ownership overrides on the scanned source graph and recomputes runtime/test strongly connected components. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane, model } from "../exports-stage-tools/model.ts";
import { scc } from "./pkg-graph.ts";

export type Carve = { name: string; owners: string[] };
export const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️|\/🔮️oracles\//u.test(f);
const FW = "🧰️framework";
const G = FW + "/📦️packages/🟦️typescript";
const M = FW + "/🔨️modules";

export function build(carves: Carve[], ignoreGlue = process.env.GLUE !== "keep") {
  const policy = loadPolicy();
  const { edges, bare } = scan();
  const owners = carves.flatMap(c => c.owners.map(o => [o, c.name] as const)).sort((a, b) => b[0].length - a[0].length);
  const cache = new Map<string, string>();
  const pk = (f: string): string => {
    let v = cache.get(f); if (v !== undefined) return v;
    for (const [o, n] of owners) if (f === o || f.startsWith(o + "/")) { v = n; break; }
    if (v === undefined) { const s = ownerOfFile(policy, f); v = s ? lane(s.owner)?.name ?? s.names[0]! : "-"; }
    cache.set(f, v); return v;
  };
  const glueTargets = new Map<string, string[]>();
  for (const e of edges) (glueTargets.get(e.file) ?? glueTargets.set(e.file, []).get(e.file)!).push(e.target);
  const ownerOfKey = (pkg: string, sub: string): string | undefined => {
    const P = model.byName.get(pkg);
    if (!P) return undefined;
    const v = P.exports[sub];
    if (typeof v !== "string" || !v.startsWith("./")) return undefined;
    const file = (P.dir + "/" + v.slice(2));
    const t = glueTargets.get(file);
    if (t?.length) { const set = new Set(t.map(x => pk(x))); if (set.size === 1) return [...set][0]; }
    const o = pk(file);
    return o === "-" ? undefined : o;
  };
  const out: { from: string; to: string; file: string; test: boolean; kind: string; target: string }[] = [];
  for (const e of edges) { if (ignoreGlue && e.file.startsWith(G + "/")) continue; const a = pk(e.file), b = pk(e.target); if (a !== b && a !== "-" && b !== "-") out.push({ from: a, to: b, file: e.file, test: isTest(e.file) || isTest(e.target), kind: "rel", target: e.target }); }
  for (const b of bare) {
    if (ignoreGlue && b.file.startsWith(G + "/")) continue;
    const a = pk(b.file); if (a === "-") continue;
    let to = b.pkg;
    { const c = ownerOfKey(b.pkg, b.sub); if (c) to = c; }
    if (!model.byName.has(to) && !carves.some(c => c.name === to)) continue;
    if (a !== to) out.push({ from: a, to, file: b.file, test: isTest(b.file), kind: "bare", target: b.spec });
  }
  return out;
}
export function report(es: ReturnType<typeof build>, mode: "runtime" | "test" | "all", show = true) {
  const adj = new Map<string, Set<string>>(), nodes = new Set<string>(), cnt = new Map<string, number>();
  for (const e of es) { if (mode === "runtime" && e.test) continue; nodes.add(e.from); nodes.add(e.to); (adj.get(e.from) ?? adj.set(e.from, new Set()).get(e.from)!).add(e.to); const k = e.from + ">" + e.to; cnt.set(k, (cnt.get(k) ?? 0) + 1); }
  const comps = scc([...nodes], adj).filter(c => c.length > 1).sort((a, b) => b.length - a.length);
  console.log(mode, "packages", nodes.size, "cyclic components", comps.map(c => c.length).join(",") || "none");
  if (show) for (const c of comps) { const s = new Set(c); console.log("\nSCC", c.length); for (const a of c) for (const b of adj.get(a) ?? []) if (s.has(b)) console.log(`  ${a} -> ${b} ${cnt.get(a + ">" + b)}`); }
  return { comps, adj, cnt };
}
export const carveLeaf = (dir: string, name: string): Carve => ({ name, owners: [M + "/" + dir, G + "/" + dir] });
if (import.meta.main) {
  const plan: Carve[] = JSON.parse(readFileSync(resolve(import.meta.dir, process.argv[2] ?? "carves.json"), "utf8"));
  const es = build(plan);
  report(es, "runtime", process.argv.includes("--show"));
  report(es, "test", false);
}
