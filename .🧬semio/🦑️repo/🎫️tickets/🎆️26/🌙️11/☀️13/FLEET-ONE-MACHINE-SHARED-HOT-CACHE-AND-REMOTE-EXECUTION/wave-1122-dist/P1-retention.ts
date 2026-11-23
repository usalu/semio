import {retireStaleUnitsV1,scanUnitDeclarationsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts";
const root=process.argv[2]!,mode=(process.argv[3]??"dry-run") as "dry-run";
const rows=await scanUnitDeclarationsV1(root);
console.log("declared entries:",rows.length);
const report=await retireStaleUnitsV1(root,{workspace:"/Users/ueli/Documents/semio",mode,graceMilliseconds:0});
console.log(JSON.stringify(report,null,1));
