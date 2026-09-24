import { selectCases } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🔍️discovery/🎛️selection/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
for (const c of selectCases(root, process.argv.slice(2))) console.log(c.case, c.owner);
