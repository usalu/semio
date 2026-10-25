import { readFileSync } from "node:fs";
import { runDocstringCensus } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts";
const scope = new Set(readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean));
const census = runDocstringCensus("/Users/ueli/Documents/semio", new AbortController().signal, () => {});
const inScope = census.hits.filter((h: any) => scope.has(h.path));
console.log(JSON.stringify({ files: census.files, repoWide: census.hits.length, inScope: inScope.length, disagreements: census.disagreements.length }));
for (const h of inScope.slice(0, 20)) console.log(h.rule, h.path, h.line);
