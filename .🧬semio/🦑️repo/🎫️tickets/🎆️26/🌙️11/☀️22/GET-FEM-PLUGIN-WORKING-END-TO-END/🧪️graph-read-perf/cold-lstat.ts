import {readFileSync,lstatSync,readdirSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const kind=process.argv[3]??"lstat";
const rows=[...new Set(memo.observations.filter((r:any)=>r.kind===kind).map((r:any)=>r.path))] as string[];
const n=Number(process.argv[4]??4000),offset=Number(process.argv[5]??0),conc=Number(process.argv[6]??64);
const take=(i:number)=>rows.slice(offset+i*n,offset+(i+1)*n);
const probeSync=kind==="readdir"?(p:string)=>readdirSync(p,{withFileTypes:true}):(p:string)=>lstatSync(p);
const probeAsync=kind==="readdir"?(p:string)=>readdir(p,{withFileTypes:true}):(p:string)=>lstat(p);
let t=performance.now();for(const p of take(0)){try{probeSync(p);}catch{}}
const a=performance.now()-t;
t=performance.now();
const set=take(1);let next=0;
await Promise.all(Array.from({length:conc},async()=>{while(next<set.length){const p=set[next++];try{await probeAsync(p);}catch{}}}));
const b=performance.now()-t;
t=performance.now();for(const p of take(0)){try{probeSync(p);}catch{}}
const c=performance.now()-t;
console.log(JSON.stringify({kind,n,distinct:rows.length,syncColdUs:Math.round(a*1000/n),asyncColdUs:Math.round(b*1000/n),syncWarmUs:Math.round(c*1000/n),threadpool:process.env.UV_THREADPOOL_SIZE??"default"}));
