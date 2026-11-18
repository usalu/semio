import {sliceLabPlansV1,sliceLabEnvironmentV1,readSliceLabOptionsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🟦️.ts";
import {sliceLabShimsV1,sliceLabSeedHomeV1,sliceLabHomeSeedsV1,sliceLabSeedToolsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab/🏠️home/🟦️.ts";
import {mkdirSync} from "node:fs";
const W="/tmp/slw-r159b";const o=readSliceLabOptionsV1({machines:1,memoryMegabytes:16384,cores:8,slice:["repo-mcp"],timeoutSeconds:300,language:"en"});
const p=sliceLabPlansV1(o,"159b",W,"/tmp/slw-r155probe/r")[0]!;
for(const d of [p.state,p.temporary,p.runtime,p.capacity,p.registry,p.cache,p.store,p.state+"/f",p.state+"/b",p.state+"/b/c",p.home])mkdirSync(d,{recursive:true});
sliceLabShimsV1(p.home,W+"/m1/downloads.log");
console.log(JSON.stringify(sliceLabSeedHomeV1(sliceLabHomeSeedsV1(sliceLabSeedToolsV1([],"workspace"),process.env.HOME!,p.home))));
console.error(JSON.stringify(sliceLabEnvironmentV1(p,undefined)));
