import {collectTemporariesV1,temporariesHoldersV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🗑️temporaries/🟦️.ts";
import {lanePathsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts";
const root=process.argv[2]!,lines:string[]=[];
const result=await collectTemporariesV1(root,{dryRun:true,report:{language:"en",write:line=>lines.push(line)}});
console.log(JSON.stringify(result),"\n"+lines.join("\n"));
