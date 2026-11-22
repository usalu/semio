import {readRemoteExecutionFleetFactsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚢️status/🟦️.ts";
const t0=Date.now();
const f=await readRemoteExecutionFleetFactsV1({root:process.cwd(),environment:process.env as Record<string,string|undefined>,timeoutMilliseconds:20000},async(...a)=>{const r=await fetch(...a);if(String(a[0]).endsWith("/v1/fleet/load")){const c=r.clone();const j=await c.json() as any;console.log(JSON.stringify(j.machines.map((m:any)=>[m.origin,m.worker])));}return r;});
console.log("ms",Date.now()-t0,f?.machines.length);
