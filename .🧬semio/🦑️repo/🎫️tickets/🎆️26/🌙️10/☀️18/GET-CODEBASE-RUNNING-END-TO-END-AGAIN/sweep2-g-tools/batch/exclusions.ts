// sweep2-g: the exclusion patterns of a candidate scheduling table, derived by the live policy code (usage: bun exclusions.ts <table.json>).
import {readFileSync} from "node:fs";
import {buildSchedulingExclusionPatternsV1, buildSchedulingV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
console.log(JSON.stringify(buildSchedulingExclusionPatternsV1(buildSchedulingV1(JSON.parse(readFileSync(process.argv[2]!, "utf8"))))));
