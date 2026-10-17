/** 🧮️ Prints schema-scope inventory diagnostics that mention a needle: `bun inv.mjs <needle>`. */
const R = process.env.R;
const { inventorySchemaScopes } = await import(`${R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`);
const inv = inventorySchemaScopes(R);
const needle = process.argv[2];
console.log("scopes", Object.keys(inv.catalog.scopes).length, "diagnostics", inv.diagnostics.length);
for (const d of inv.diagnostics) { const t = typeof d === "string" ? d : JSON.stringify(d); if (t.includes(needle)) console.log(t.slice(0, 600)); }
