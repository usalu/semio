import {acquireResourceLease} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
const root="/Users/ueli/Documents/semio",directory=root+"/.cache/b/l/m/data",resource="native-fixed-image:"+root+"/.cache/b/t",seconds=Number(process.argv[2]??"60");
let idle=0,busy=0;const started=Date.now();
while(Date.now()-started<seconds*1000){
 try{const lease=await acquireResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,timeoutMs:0,admissionPatienceMs:1000});lease.release();idle++;}
 catch(error){if(String((error as Error).message).startsWith("Resource lease timed out"))busy++;else throw error;}
 await new Promise(done=>setTimeout(done,500));
}
console.log(JSON.stringify({seconds,idle,busy,idleFraction:idle/(idle+busy)}));
