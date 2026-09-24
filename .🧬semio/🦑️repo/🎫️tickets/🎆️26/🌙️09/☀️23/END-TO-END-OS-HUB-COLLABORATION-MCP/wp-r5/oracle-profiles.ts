import { readFileSync } from "node:fs";
import { discoverTestCases, loadOracleRegistry, parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const r = loadOracleRegistry(root);
const missing = r.oracles.filter((o: any) => !Array.isArray(o.comparisonProfiles)).map((o: any) => o.id);
const used = new Map<string, Set<string>>();
for (const c of discoverTestCases(root)) {
  const f = parseFeature(readFileSync(`${root}/${c.featurePath}`, "utf8")) as any;
  const oracles: string[] = f.oracles ?? (f.oracle ? [f.oracle] : []);
  for (const o of oracles) { if (!used.has(o)) used.set(o, new Set()); if (f.comparison) used.get(o)!.add(f.comparison); }
}
for (const id of missing) console.log(id, JSON.stringify([...(used.get(id) ?? [])]));
console.log("--- via fixtures / requirements");
for (const id of missing) {
  const viaFixtures = new Set(r.fixtureManifests.filter((f: any) => f.generator?.oracle === id).map((f: any) => f.comparisonProfile));
  const owner = r.contributions.find((c) => c.oracles.some((o: any) => o.id === id))!;
  console.log(id, JSON.stringify([...viaFixtures]), "| owner profiles", JSON.stringify(owner.comparisonProfiles.map((p) => p.id)), "| pipelines", JSON.stringify((owner as any).comparisonPipelines?.map((p: any) => p.id)));
}
