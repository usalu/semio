import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const o of registry.oracles as any[]) {
  const pk = JSON.stringify([o.package, o.packages, o.library]);
  if (/serde|node:crypto|"ajv"|typescript|crypto/.test(pk)) console.log(o.id, o.ecosystem, pk, o.hostPath, o.manifestPath ?? "", JSON.stringify(o.productionDebt ?? null).slice(0, 200));
}
