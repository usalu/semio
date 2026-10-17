/** 🔬️ Compares the live schema inventory with the tracked catalog and prints differing scopes: `R=<root> bun catalogdiff.mjs`. */
import { readFileSync } from "node:fs";
const R = process.env.R;
const L = `${R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const { inventorySchemaScopes } = await import(`${L}/🔍️discovery/🟦️.ts`);
const inv = inventorySchemaScopes(R);
const file = JSON.parse(readFileSync(`${L}/🔣️schema-catalog.json`, "utf8")).scopes;
const live = inv.catalog.scopes;
const ids = new Set([...Object.keys(file), ...Object.keys(live)]);
let n = 0;
for (const id of [...ids].sort()) {
  const a = JSON.stringify(file[id]), b = JSON.stringify(live[id]);
  if (a !== b) { n++; console.log("DIFF", id, file[id]?.path ?? live[id]?.path); }
}
console.log("scopes", Object.keys(live).length, "differing", n, "diagnostics", inv.diagnostics.length);
