import {readFileSync} from "node:fs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const obs=await import(G+"/👁️observation/🟨️.mjs");
const producer=await import(G+"/🕸️graph/📥️inference/🟨️.mjs");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const all=memo.observations as unknown[];
console.log("owners",memo.owners.length,"rows",all.length);
const limit=Number(process.argv[3]??200);const t0=performance.now();let ok=0,stale=0,rowsDone=0;const slow:[number,number,string][]=[];
await obs.settleObservedComputationV1(async()=>{
 for(const owner of memo.owners.slice(0,limit)){
  const rows=owner.rows.map((i:number)=>all[i]);const t=performance.now();
  const holds=await obs.observationsHoldWithinRoundV1(rows,64);const ms=performance.now()-t;
  holds?ok++:stale++;rowsDone+=rows.length;slow.push([Math.round(ms),rows.length,owner.suffix.split("\0")[0]]);
 }
 return 1;
});
console.log("owners",limit,"ok",ok,"stale",stale,"rows",rowsDone,Math.round(performance.now()-t0)+"ms");
console.log(slow.sort((a,b)=>b[0]-a[0]).slice(0,8));
