import { surveyUnmanagedTests } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const rows = surveyUnmanagedTests("/Users/ueli/Documents/semio");
const by = new Map<string, string[]>();
for (const r of rows) { const a = r.area; if (!by.has(a)) by.set(a, []); by.get(a)!.push(r.path); }
for (const [a, ps] of by) { console.log(`== ${a}: ${ps.length}`); for (const p of ps) console.log("   " + p); }
