import {snapshotAcquireV1} from "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H95/run/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📸️snapshot/⏩️advance/🟦️.ts";
const base="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H95/realbase2",root="/Users/ueli/Documents/semio",store="/Users/Shared/semio-store";
const step=async(label:string,extra:Record<string,unknown>={})=>{const t=Date.now();const r=await snapshotAcquireV1({root,lane:"m1",mode:"root",base,build:true},{store,...extra} as never);console.log(label,JSON.stringify({wall:(Date.now()-t)/1000,via:r!.via,gen:r!.generation,files:r!.files,copied:r!.copied,rounds:r!.rounds,consistent:r!.consistent,lag:r!.lagSeconds,digest:r!.digest.slice(0,19)}));};
await step("warmA");
await step("warmB");
const t=Date.now();const r=await snapshotAcquireV1({root,lane:"m2",mode:"root",base,build:true},{store} as never);console.log("newlane",JSON.stringify({wall:(Date.now()-t)/1000,copied:r!.copied,gen:r!.generation}));
await Bun.sleep(20000);
await step("warmC");
