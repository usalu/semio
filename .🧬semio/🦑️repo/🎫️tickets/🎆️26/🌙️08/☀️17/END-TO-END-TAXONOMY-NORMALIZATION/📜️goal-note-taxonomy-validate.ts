const repoRoot = "/Users/ueli/Documents/semio";
const { join } = await import("node:path");
const { validateTaxonomy } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts"));
const { readFileSync } = await import("node:fs");
const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const problems = validateTaxonomy(taxonomy);
console.log("problems:", problems.length);
for (const p of problems) console.log(" -", p);
