import {repositoryCacheBudgetV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {mkdtempSync,mkdirSync} from "node:fs";
const real=repositoryCacheBudgetV1("/Users/ueli/Documents/semio");
console.log("real",real.partitions.graph.capacityBytes,real.maximumBytes,real.headroomBytes);
const fresh=mkdtempSync("/private/tmp/claude-501/gb-");mkdirSync(fresh+"/.cache/b",{recursive:true});
const f=repositoryCacheBudgetV1(fresh);console.log("fresh",f.partitions.graph.capacityBytes,f.maximumBytes,f.headroomBytes);
