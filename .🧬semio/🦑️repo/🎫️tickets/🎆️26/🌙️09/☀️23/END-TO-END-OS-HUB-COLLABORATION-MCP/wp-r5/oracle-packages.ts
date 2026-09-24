import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const r = loadOracleRegistry("/Users/ueli/Documents/semio");
const pkg = process.argv[2];
for (const c of r.contributions) for (const o of c.oracles as any[]) {
  const names = [o.package, ...(o.packages ?? []).map((p: any) => p.name ?? p.package)];
  if (!pkg || names.includes(pkg)) console.log(o.id, "|", o.ecosystem, "|", names.join(","), "|", o.productionDebt ? "DEBT" : "", "|", c.owner);
}
