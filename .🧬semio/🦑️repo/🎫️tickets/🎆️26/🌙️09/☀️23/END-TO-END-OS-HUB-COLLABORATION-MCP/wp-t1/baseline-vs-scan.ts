/** 🔒️ Lists every oracle-linked or host package whose committed baseline classification disagrees with the live declaration scan. */
import { readFileSync } from "node:fs";
import { scanDeclaredDependencies, loadOracleRegistry, oracleLinkedPackages, externalOracleHostPackages, dependencyEcosystemOfRegistryValue } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const registry = loadOracleRegistry(root);
const baseline = JSON.parse(readFileSync(`${root}/🔒️dependencies.json`, "utf8")).entries as any[];
const scanned = scanDeclaredDependencies(root, registry) as any[];
const keys = new Map<string, string[]>();
for (const oracle of registry.oracles) for (const linked of oracleLinkedPackages(oracle)) { const k = `${dependencyEcosystemOfRegistryValue(oracle.ecosystem)}:${linked.package}`; keys.set(k, [...(keys.get(k) ?? []), `${oracle.id}${oracle.productionDebt ? "(debt)" : ""}`]); }
for (const host of externalOracleHostPackages(registry)) { const k = `${host.ecosystem}:${host.name}`; keys.set(k, [...(keys.get(k) ?? []), "host"]); }
for (const [key, oracles] of keys) {
  const [ecosystem, ...rest] = key.split(":"); const name = rest.join(":");
  const b = baseline.find((e) => e.ecosystem === ecosystem && e.name === name), s = scanned.find((e) => e.ecosystem === ecosystem && e.name === name);
  const bk = JSON.stringify(b?.kinds ?? null), sk = JSON.stringify(s?.kinds ?? null);
  if (bk !== sk || (b?.productionReachable ?? null) !== (s?.productionReachable ?? null)) console.log(`${key}  baseline=${bk}/${b?.productionReachable}  scan=${sk}/${s?.productionReachable}  oracles=${oracles.slice(0, 4).join(",")}  users=${(s?.users ?? []).slice(0, 3).join(" ")}`);
}
