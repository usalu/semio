import { readFileSync } from "node:fs";
import { discoverTestContributions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const seen = new Map<string, number>();
const show = (k: string, v: unknown, owner: string) => { const n = (seen.get(k) ?? 0) + 1; seen.set(k, n); if (n <= 2) console.log(k, owner.split("/").slice(2, 6).join("/"), JSON.stringify(v).slice(0, 400)); };
for (const c of discoverTestContributions(root)) {
  const raw = JSON.parse(readFileSync(`${root}/${c.manifestPath}`, "utf8"));
  for (const f of raw.fixtureManifests ?? []) { for (const k of ["comparisonPipeline", "reproducibilityDiffs", "invariants"]) if (k in f) show(k, f[k], c.owner); if (f.generator?.exportEngine) show("exportEngine", f.generator.exportEngine, c.owner); }
  for (const mm of raw.mutationManifests ?? []) for (const mu of mm.mutations ?? []) { for (const k of ["carriers", "notes", "comparisonPipeline"]) if (k in mu) show(k, mu[k], c.owner); for (const r of mu.oracleRequirements ?? []) if ("oracle" in r) show("req.oracle", r, c.owner); }
  for (const p of raw.probes ?? []) if (p.packages) show("probe.packages", p.packages, c.owner);
  for (const h of raw.oracleHostPackages ?? []) if (h._comment) show("host._comment", h, c.owner);
  for (const o of raw.oracles ?? []) if (o.productionDebt && (!o.productionDebt.reachableFrom?.length || o.productionDebt.reason)) show("debt", o.productionDebt, c.owner);
}
console.log(Object.fromEntries(seen));
