// ts2-g: the ownership walk floor without the per-operation event-loop yield (control), and with a time-sliced yield.
import {withCurrentBunWorkspaceOwnershipV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";
import {setImmediate as yieldLoop} from "node:timers/promises";
const mode=process.argv[2]??"none";
let ops=0,sliceStart=performance.now(),yields=0;
const advance=async()=>{ops++;if(mode==="every"){await yieldLoop();yields++;}else if(mode==="slice"&&performance.now()-sliceStart>=20){await yieldLoop();yields++;sliceStart=performance.now();}};
const t=performance.now(),c0=process.cpuUsage();
const n=await withCurrentBunWorkspaceOwnershipV1(process.cwd(),{kind:"scope",owner:"🎓️teaching"},{signal:new AbortController().signal,advance},async c=>c.scopes.flatMap(s=>s.packages).length);
const c=process.cpuUsage(c0);
console.log(JSON.stringify({mode,members:n,ops,yields,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((c.user+c.system)/1000)}));
