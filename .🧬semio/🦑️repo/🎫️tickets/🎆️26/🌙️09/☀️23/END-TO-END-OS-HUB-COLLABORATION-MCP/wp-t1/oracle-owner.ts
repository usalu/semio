import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const pkgs = new Set(process.argv.slice(2));
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const c of registry.contributions) for (const o of c.oracles) if ([o.package, ...(o.packages ?? []).map((p: any) => p.package)].some((p) => pkgs.has(p) || [...pkgs].some((q) => p.includes(q)))) console.log(o.id, "|", o.package, "|", c.owner, "|", c.manifestPath, "| entry" in o ? "" : "", JSON.stringify(Object.keys(o)));
