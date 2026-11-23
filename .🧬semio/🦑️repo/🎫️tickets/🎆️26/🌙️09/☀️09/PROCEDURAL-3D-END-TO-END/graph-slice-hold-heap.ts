import {readFileSync} from "node:fs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const obs=await import(G+"/👁️observation/🟨️.mjs"),producer=await import(G+"/🕸️graph/📥️inference/🟨️.mjs");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const [path,passes,keep]=[process.argv[2],Number(process.argv[3]??1),process.argv[4]??"1"];
let memo:any=JSON.parse(readFileSync(path,"utf8"));
const all=memo.observations as unknown[];
const entries=new Map<string,{facts:unknown;rows:unknown[]}>();for(const o of memo.owners)entries.set(o.suffix,{facts:o.facts,rows:o.rows.map((i:number)=>all[i])});
const owners=[...entries.keys()];if(keep==="0"){memo=null;}
const mb=(gc:boolean)=>{if(gc)Bun.gc(true);const m=process.memoryUsage();return Math.round(m.heapUsed/1048576)+"/"+Math.round(m.rss/1048576)};
console.log("seeded",entries.size,"live",mb(true),"raw",mb(false));
const t0=performance.now();let n=0,stale=0;
const out=await obs.captureObservedSourceInputsV1(()=>obs.settleObservedComputationV1(async()=>{
 for(let pass=0;pass<passes;pass++)for(const suffix of owners){
  const e=entries.get(suffix)!,holds=await obs.observationsHoldWithinRoundV1(e.rows,64);holds||stale++;obs.retainObservedRowsV1(e.rows);n++;
  if(n%100===0)console.log(n,"stale",stale,"raw",mb(false),"live",mb(true),Math.round((performance.now()-t0)/1000)+"s");
 }
 return 1;
}),undefined,false);
console.log("end",n,"live",mb(true),"captured",out.observations.length);
