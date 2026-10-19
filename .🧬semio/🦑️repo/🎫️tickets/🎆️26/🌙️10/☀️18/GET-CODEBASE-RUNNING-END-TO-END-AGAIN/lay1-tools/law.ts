import { policyBannedNameStemBreaches } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
const rows = policyBannedNameStemBreaches(process.cwd());
for (const r of rows) console.log(r.scope);
console.log(rows.length);
