import {writeFileSync} from "node:fs";
import {planAdoptV1} from "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/adopt2/tree/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧲️adopt/🟦️.ts";
const started=Date.now();
const plan=await planAdoptV1({worktree:"/Users/ueli/Documents/semio",storeRoot:"/Users/Shared/semio-store",originId:"g1-server",groupId:"g1"},{sample:20},undefined,event=>{if((event as {stage?:string}).stage?.startsWith("adopt")&&(event as {done?:number}).done!==undefined&&(event as {done:number}).done%20000===0)console.error("[DEBUG]",JSON.stringify(event),Date.now()-started);});
writeFileSync(process.argv[2]!,JSON.stringify(plan,null,1));
console.error("[DEBUG] finished in ms",Date.now()-started);
