import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const ids = process.argv.slice(2);
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const o of registry.oracles as any[]) if (ids.includes(o.id)) console.log(JSON.stringify({ ...o, rationale: String(o.rationale ?? "").slice(0, 600) }, null, 1));
for (const c of registry.contributions as any[]) for (const o of c.oracles ?? []) if (ids.includes(o.id)) console.log("MANIFEST", c.manifestPath);
