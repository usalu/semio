// 🧾️ For each no-oracle decision that claims a mutation capability: which qualifying oracles already supply those capabilities, and where the decision lives.
import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
const mutationCaps = new Set(registry.mutationManifests.flatMap((m) => m.mutations.flatMap((x) => [x.capability, ...x.oracleRequirements.map((r) => r.capability)])));
for (const d of registry.noOracleDecisions as any[]) {
  const covered = (d.capabilities ?? []).filter((c: string) => mutationCaps.has(c));
  if (covered.length === 0) continue;
  const supply = covered.map((c: string) => `${c}: ${registry.oracles.filter((o: any) => o.capabilities.includes(c)).map((o: any) => `${o.id}[${o.kind}]`).join(",") || "-"}`);
  console.log(JSON.stringify({ id: d.id, keys: Object.keys(d), capabilities: d.capabilities, supply, source: d.manifestPath ?? d.source ?? null }));
}
