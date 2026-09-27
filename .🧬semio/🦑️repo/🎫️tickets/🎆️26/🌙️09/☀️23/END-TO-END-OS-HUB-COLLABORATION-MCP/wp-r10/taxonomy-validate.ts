#!/usr/bin/env bun
/** ✅️ R10 item 1: validates a candidate taxonomy with discovery's own `validateTaxonomy` (the rule set `loadCatalogTaxonomy`
 * applies) without touching the live file. Usage: bun taxonomy-validate.ts <candidate.json> */
import { readFileSync } from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const { validateTaxonomy } = await import(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`);
const started = performance.now();
const problems: string[] = validateTaxonomy(JSON.parse(readFileSync(process.argv[2]!, "utf8")));
console.log(JSON.stringify({ ms: Math.round(performance.now() - started), problems: problems.length }));
for (const problem of problems) console.log(`- ${problem}`);
process.exit(problems.length ? 1 : 0);
