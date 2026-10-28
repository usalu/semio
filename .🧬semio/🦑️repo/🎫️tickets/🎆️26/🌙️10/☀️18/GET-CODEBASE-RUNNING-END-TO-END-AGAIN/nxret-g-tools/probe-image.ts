import {acquireResourceLease} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
const root="/Users/ueli/Documents/semio",directory=root+"/.cache/b/l/m/data",resource="native-fixed-image:"+root+"/.cache/b/t";
for(let attempt=0;attempt<3;attempt++){
 const started=Date.now();
 try{const lease=await acquireResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,timeoutMs:0,admissionPatienceMs:1000});lease.release();console.log("idle",Date.now()-started,"ms");}
 catch(error){console.log("busy:",String((error as Error).message).slice(0,120),Date.now()-started,"ms");}
}
