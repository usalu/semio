import { readFileSync } from "node:fs";
import { externalOracleHostPackages, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const baseline = JSON.parse(readFileSync(`${root}/🔒️dependencies.json`, "utf8"));
for (const host of externalOracleHostPackages(loadOracleRegistry(root))) {
  const e = baseline.entries.find((c: any) => c.ecosystem === host.ecosystem && c.name === host.name);
  console.log(host.ecosystem, host.name, JSON.stringify(e?.kinds), JSON.stringify(e?.users), JSON.stringify((host as any).users));
}
