import {acquireQueuedResourceLease} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {randomBytes} from "node:crypto";
const resource="native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g",directory="/Users/ueli/Documents/semio/.cache/b/l/m/data",started=Date.now();
const lease=await acquireQueuedResourceLease({directory,resource,mode:"shared",owner:randomBytes(8).toString("hex"),signal:new AbortController().signal,sharedCohort:true,timeoutMs:90000,onWait:()=>{}}).then(value=>({value,error:undefined}),error=>({value:undefined,error:String(error)}));
console.log(JSON.stringify({pid:process.pid,resource,acquiredAfterMs:Date.now()-started,error:lease.error}));
lease.value?.release();
