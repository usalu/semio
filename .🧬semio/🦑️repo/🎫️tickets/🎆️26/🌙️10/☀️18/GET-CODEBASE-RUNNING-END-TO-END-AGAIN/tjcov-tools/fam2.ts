import { toolJobOwnerFamilyFiles } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const f = process.argv[2]!;
const fam = toolJobOwnerFamilyFiles("/Users/ueli/Documents/semio", f);
console.log(fam.length); for (const x of fam) console.log(x.split("/").slice(-5).join("/"));
