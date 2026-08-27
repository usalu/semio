import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, relative } from "node:path";
const repoRoot = process.cwd();
const manifests: string[] = [];
const walk = (p: string, d: number): void => { if (d > 12) return; let es; try { es = readdirSync(p, { withFileTypes: true }); } catch { return; }
  for (const e of es) { if (["node_modules","target",".git","temp"].includes(e.name)) continue; const f = join(p, e.name);
    if (e.isDirectory()) walk(f, d + 1); else if (e.name.endsWith("component.json") && f.includes("oracle")) manifests.push(f); } };
walk(repoRoot, 0);
const BUNDLE = [["🦀️component.rs","🦀️.rs"],["🦠️mutation/🔣️component.json","🦠️mutation/🔣️.json"],["📸️snapshot/⬅️before/🔣️component.json","📸️snapshot/⬅️before/🔣️.json"],["📸️snapshot/➡️after/🔣️component.json","📸️snapshot/➡️after/🔣️.json"],["🔺️diff/🔣️component.json","🔺️diff/🔣️.json","🔺️diff/🚫️component.absent","🔺️diff/🚫️.absent"],["🎯️outcome/🔣️component.json","🎯️outcome/🔣️.json"]];
let kinds = 0, kindsWithVector = 0, kindsWithScenario = 0, kindsWithCompleteBundle = 0;
const gaps: any[] = [];
for (const m of manifests) {
  let parsed: any; try { parsed = JSON.parse(readFileSync(m, "utf8")); } catch { continue; }
  if (!Array.isArray(parsed.mutationCatalogs)) continue;
  const owner = relative(repoRoot, m).split("/").slice(0, -2).join("/");
  for (const c of parsed.mutationCatalogs) {
    const srcRoot = join(repoRoot, owner, "🧬️schema", "🧬️mutations");
    const projRoot = join(repoRoot, owner, "🧪️tests", "🦠️mutations", c.standardDirectoryName ?? "", c.subsetDirectoryName ?? "");
    for (const k of (c.kinds ?? [])) {
      kinds++;
      const v = (c.vectors ?? []).find((x: any) => x.mutationId === k);
      if (!v) { gaps.push({ owner, catalog: c.id, kind: k, reason: "no-vector" }); continue; }
      kindsWithVector++;
      const scen = (v.scenarios ?? []);
      if (scen.length === 0) { gaps.push({ owner, catalog: c.id, kind: k, reason: "no-scenario" }); continue; }
      kindsWithScenario++;
      let complete = false;
      for (const s of scen) {
        const a = join(srcRoot, v.sourceMutationDirectoryName, "🧪️tests", s.id);
        const b = join(projRoot, v.mutationDirectoryName, s.directoryName);
        for (const root of [a, b]) if (existsSync(root) && BUNDLE.every((alts) => alts.some((f) => existsSync(join(root, f))))) complete = true;
      }
      if (complete) kindsWithCompleteBundle++; else gaps.push({ owner, catalog: c.id, kind: k, reason: "incomplete-bundle", dir: v.sourceMutationDirectoryName, scen: scen.map((s:any)=>s.id) });
    }
  }
}
console.log(JSON.stringify({ kinds, kindsWithVector, kindsWithScenario, kindsWithCompleteBundle, gaps: gaps.length }, null, 2));
const byReason: any = {}; for (const g of gaps) byReason[g.reason] = (byReason[g.reason] ?? 0) + 1;
console.log(byReason);
require("fs").writeFileSync(process.argv[2], JSON.stringify(gaps, null, 1));
