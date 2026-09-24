import { scanTestLayout, oracleImportsInProduction } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const mode = process.argv[2] ?? "layout";
if (mode === "layout") for (const f of await scanTestLayout(root)) console.log(`  testing/layout  ${f.path}  ${f.detail}${f.line === null ? "" : ` (line ${f.line})`}`);
else for (const h of oracleImportsInProduction(root)) console.log(`  testing/dependency  ${h.path}  Production source imports the registered oracle ${h.oracle}`);
