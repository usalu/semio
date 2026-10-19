import { readFileSync } from "node:fs";
import { policyTestModSpans } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📋️policy/🔎️source/🟦️.ts";
const f = process.argv[2]!;
const lines = readFileSync(f, "utf8").split("\n");
console.log(policyTestModSpans(lines), lines.length);
