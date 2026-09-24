import { scanDeclaredDependencies, loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const names = new Set(process.argv.slice(2));
for (const entry of scanDeclaredDependencies(root, loadOracleRegistry(root)) as any[]) if (names.has(entry.name)) console.log(JSON.stringify({ ecosystem: entry.ecosystem, name: entry.name, kinds: entry.kinds, productionReachable: entry.productionReachable, users: entry.users }));
