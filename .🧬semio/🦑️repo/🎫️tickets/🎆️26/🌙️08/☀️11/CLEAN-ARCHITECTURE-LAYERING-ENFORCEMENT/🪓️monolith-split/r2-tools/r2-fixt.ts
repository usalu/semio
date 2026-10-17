import { readFileSync } from "node:fs";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { fixturesToRuntime } = await import(`${L}/🧪️test-closure/🟦️.ts`);
const f = process.argv[2]!;
const r = fixturesToRuntime(readFileSync(f, "utf8"), f, "/Users/ueli/Documents/semio");
console.log(r === null ? "NULL" : "converted " + r.length);
