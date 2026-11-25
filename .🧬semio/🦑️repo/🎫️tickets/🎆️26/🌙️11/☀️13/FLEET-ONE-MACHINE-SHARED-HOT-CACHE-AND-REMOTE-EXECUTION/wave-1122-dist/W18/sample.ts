/** 📸️ W18: appends one sample of every fleet machine's unit counters (compiled, hits, skipped by reason) from the scheduler's live load table to `W18/🗑️generated/samples.jsonl`; read-only. */
import {appendFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
const root="/Users/ueli/Documents/semio",lib=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚢️status/🟦️.ts";
const {readRemoteExecutionFleetFactsV1}=await import(lib);
const facts=await readRemoteExecutionFleetFactsV1({root,environment:process.env,timeoutMilliseconds:20000});
const out=join(import.meta.dir,"🗑️generated");mkdirSync(out,{recursive:true});
const row={atMs:Date.now(),cache:facts?.cache,machines:(facts?.machines??[]).map((m:any)=>({origin:m.origin,ageSeconds:m.ageSeconds,worker:m.worker,code:m.code,hit:m.cacheHit,miss:m.cacheMiss,skipped:m.cacheSkipped,reasons:m.cacheSkipReasons}))};
appendFileSync(join(out,"samples.jsonl"),JSON.stringify(row)+"\n");
console.log(JSON.stringify(row,null,0).slice(0,6000));
