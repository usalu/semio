import { readFileSync } from "node:fs";
const lib = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const { validateTaxonomy } = await import(`${lib}/🔍️discovery/🟦️.ts`);
const { loadTaxonomy } = await import(`${lib}/🧹️normalization/🔣️taxonomy/🟦️.ts`);
const path = process.argv[2]!;
const problems: string[] = validateTaxonomy(JSON.parse(readFileSync(path, "utf8")));
console.log(`discovery validateTaxonomy problems=${problems.length}`);
for (const p of problems.slice(0, 40)) console.log(" -", p);
try { const t = loadTaxonomy({ repoRoot: "/Users/ueli/Documents/semio", taxonomyPath: path }); console.log("normalization loadTaxonomy ok kinds=", t.directoryKinds.length); } catch (e) { console.log("normalization loadTaxonomy FAILED", String(e).slice(0, 600)); }
