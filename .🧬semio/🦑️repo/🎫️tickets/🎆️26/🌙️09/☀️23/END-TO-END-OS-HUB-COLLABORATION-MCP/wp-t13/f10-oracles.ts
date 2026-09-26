import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const registry = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const id of process.argv.slice(2)) console.log(JSON.stringify(registry.oracles.find((o) => o.id === id), null, 1));
const ts = registry.oracles.filter((o) => o.ecosystem === "javascript");
console.log(`javascript oracles: ${ts.length}`);
for (const o of ts) console.log(o.id, (o as any).hostPath ?? "-", (o as any).packages ?? "");
