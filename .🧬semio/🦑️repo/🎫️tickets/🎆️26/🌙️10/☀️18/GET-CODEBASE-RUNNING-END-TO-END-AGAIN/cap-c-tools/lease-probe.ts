import {acquireCompilerCapacityLeaseV1,phaseCompilerAdmissionControlV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
const ctl=new AbortController();
const control=phaseCompilerAdmissionControlV1(ctl.signal,60000,()=>undefined,()=>new Promise(r=>setImmediate(r)));
try{const l=await acquireCompilerCapacityLeaseV1({directory:process.argv[2]!,control,environment:{},request:{kind:"available",minimumParallelism:1,maximumParallelism:2}});console.log("granted",l.parallelism);l.release();}catch(e){console.log("ERR",String(e).slice(0,200));}
