import {collectComponentRunsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧩️runs/🟦️.ts";
const [store,need,mode]=[process.argv[2]!,Number(process.argv[3]),process.argv[4]!];
const lines:string[]=[];
const r=await collectComponentRunsV1(store,{dryRun:mode!=="apply",needBytes:()=>need,report:{language:"en",write:l=>lines.push(l)}});
console.log(lines.join("\n"));console.log(JSON.stringify({dry:r.dryRun,total:r.total,removed:r.removed.map(x=>x.path.split("/").pop()+":"+x.bytes),freed:r.freedBytes,held:r.held,kept:r.kept}));
