import { readFileSync } from "node:fs";
import { parseFeature } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
const files = readFileSync(process.argv[2], "utf8").trim().split("\n").filter(Boolean);
for (const f of files) {
  try { const p = parseFeature(readFileSync(f, "utf8")); console.log(`${p.scenarios.length}\t${f}`); } catch (e) { console.log(`ERR\t${f}`); }
}
