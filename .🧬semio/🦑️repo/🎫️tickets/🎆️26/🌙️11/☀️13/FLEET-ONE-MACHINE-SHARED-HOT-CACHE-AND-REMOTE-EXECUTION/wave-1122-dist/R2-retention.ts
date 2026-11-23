import {retireStaleUnitsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts";
const [root,workspace,mode]=[process.argv[2]!,process.argv[3]!,(process.argv[4]??"dry-run") as "dry-run"|"enforce"];
const report=await retireStaleUnitsV1(root,{workspace,mode,graceMilliseconds:0});
console.log(JSON.stringify(report,null,1));
