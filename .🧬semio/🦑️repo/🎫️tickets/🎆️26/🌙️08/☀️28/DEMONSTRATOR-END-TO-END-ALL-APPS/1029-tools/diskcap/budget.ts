import {repositoryCacheBudgetV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
const b=repositoryCacheBudgetV1("/Users/ueli/Documents/semio");
for(const [k,v] of Object.entries(b.partitions))console.log(k,v.capacityBytes,v.filesystemBytes);
console.log("max",b.maximumBytes,"headroom",b.headroomBytes);
