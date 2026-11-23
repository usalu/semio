import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const root="C:\\semio\\🧰️framework\\🛍️products\\🦑️repo\\🔨️modules\\📚️library\\🏗️build\\";
const observation=await import(pathToFileURL(root+"👁️observation\\🟨️.mjs").href);
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const rows=memo.observations as {kind:string;path:string}[];
const per=new Map<string,{n:number;ms:number;stale:number;max:number}>();
const t0=performance.now();
await observation.observationsHoldWithinRoundV1([],64,undefined);
for(const row of rows){
 const t=performance.now();
 const ok=await observation.observationsHold([row],1,new Set(),new Set());
 const ms=performance.now()-t;
 const e=per.get(row.kind)??{n:0,ms:0,stale:0,max:0};e.n++;e.ms+=ms;if(!ok)e.stale++;e.max=Math.max(e.max,ms);per.set(row.kind,e);
}
let total=0;
for(const [k,e] of [...per].sort((a,b)=>b[1].ms-a[1].ms)){total+=e.ms;console.log(k.padEnd(20),String(e.n).padStart(7),"ms",String(Math.round(e.ms)).padStart(8),"us/row",String(Math.round(e.ms*1000/e.n)).padStart(6),"stale",e.stale,"max ms",Math.round(e.max));}
console.log("sum ms",Math.round(total),"wall ms",Math.round(performance.now()-t0));
