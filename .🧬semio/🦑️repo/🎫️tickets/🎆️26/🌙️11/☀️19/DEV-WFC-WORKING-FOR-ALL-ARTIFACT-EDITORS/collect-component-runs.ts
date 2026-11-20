import {collectComponentRunsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🟦️.ts";
const store=process.argv[2]!,dryRun=process.argv[3]!=="apply";
const result=await collectComponentRunsV1(store,{dryRun,needBytes:()=>Number.MAX_SAFE_INTEGER,report:{language:"en",write:line=>console.log(line)}});
console.log(JSON.stringify({dryRun,removed:result.removed.length,freedBytes:result.freedBytes,held:result.held,kept:result.kept}));
