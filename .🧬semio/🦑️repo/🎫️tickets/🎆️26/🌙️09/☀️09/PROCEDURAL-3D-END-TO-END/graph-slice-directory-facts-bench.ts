import fs,{readFileSync} from "node:fs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const obs=await import(G+"/👁️observation/🟨️.mjs");
const rows=(JSON.parse(readFileSync(process.argv[2],"utf8")).observations as {kind:string;path:string}[]).filter(r=>r.kind==="readdir").filter((_,i)=>i%8===0);
const reads={n:0};const port=new Proxy(fs,{get:(t,k)=>k==="readdirSync"?(...a:unknown[])=>{reads.n++;return(t.readdirSync as Function)(...a);}:(t as never)[k as never]});
const run=(label:string,inRound:boolean)=>{reads.n=0;const cpu=process.cpuUsage(),t=performance.now();
 const body=()=>{let h=0;for(const r of rows)h+=obs.directoryFactOfV1("readdir",r.path).length;return h;};
 obs.withObservedWorldV1(port as never,()=>inRound?obs.withObservationRoundV1(body):body());
 const c=process.cpuUsage(cpu);console.log(label.padEnd(22),"readdir calls",String(reads.n).padStart(7),"cpu ms",Math.round((c.user+c.system)/1000),"wall ms",Math.round(performance.now()-t));};
console.log("readdir rows",rows.length,"distinct",new Set(rows.map(r=>r.path)).size);
for(let i=0;i<1;i++){run("old: no shared round",false);run("new: one round",true);}
