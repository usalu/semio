/** 🧩 Module-level (first directory below 🔨️modules) dependency graph of files owned by one package. */
import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane } from "../exports-stage-tools/model.ts";
import { scc } from "./pkg-graph.ts";
const pkgName = process.argv[2] ?? "@semio-tech/framework";
const policy = loadPolicy();
const { edges, bare } = scan();
const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
const mode = process.argv[3] ?? "runtime";
const own = new Map<string, string>();
const pk = (f: string): string => { let v = own.get(f); if (v === undefined) { const o = ownerOfFile(policy, f); v = o ? lane(o.owner)?.name ?? o.names[0]! : "-"; own.set(f, v); } return v; };
const mod = (f: string): string => {
  const m = /^(🧰️framework\/(?:🔨️modules|🛍️products\/[^/]+\/🔨️modules)\/[^/]+)/u.exec(f);
  if (m) return m[1]!.replace("🧰️framework/", "");
  const g = /^🧰️framework\/📦️packages\/🟦️typescript\/([^/]+)/u.exec(f);
  if (g) return "glue:" + g[1];
  return f.split("/").slice(0, 3).join("/");
};
const adj = new Map<string, Set<string>>(), nodes = new Set<string>(), cnt = new Map<string, number>();
for (const e of edges) {
  if (pk(e.file) !== pkgName || pk(e.target) !== pkgName) continue;
  if (mode === "runtime" && (isTest(e.file) || isTest(e.target))) continue;
  const a = mod(e.file), b = mod(e.target); if (a === b) continue;
  nodes.add(a); nodes.add(b); (adj.get(a) ?? adj.set(a, new Set()).get(a)!).add(b); cnt.set(a + ">" + b, (cnt.get(a + ">" + b) ?? 0) + 1);
}
const comps = scc([...nodes], adj).filter(c => c.length > 1).sort((a, b) => b.length - a.length);
console.log(pkgName, mode, "modules", nodes.size, "cyclic", comps.map(c => c.length).join(","));
for (const c of comps.slice(0, 3)) { console.log("SCC", c.length, c.join(" ")); }
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
writeFileSync(resolve(import.meta.dir, "../🗑️generated/cycle-break-x/modgraph.json"), JSON.stringify({ adj: [...adj].map(([k, v]) => [k, [...v]]), cnt: [...cnt] }));
