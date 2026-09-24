import { oracleImportsInProduction } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const hits = oracleImportsInProduction("/Users/ueli/Documents/semio");
const by: Record<string, number> = {};
for (const h of hits) by[h.oracle] = (by[h.oracle] ?? 0) + 1;
console.log(JSON.stringify(by, null, 1));
for (const h of hits) console.log(`${h.oracle} :: ${h.path}`);
