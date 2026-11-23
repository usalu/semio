import {readFileSync} from "node:fs";
const m=JSON.parse(readFileSync(process.argv[2],"utf8"));
const obs=m.observations as any[];
const d=new Map<string,number>(),r=new Map<string,number>();
for(const o of obs)d.set(o.kind,(d.get(o.kind)??0)+1);
for(const ow of m.owners)for(const i of ow.rows){const k=obs[i].kind;r.set(k,(r.get(k)??0)+1);}
console.log("kind distinct refs ratio");
for(const [k,n] of [...d].sort((a,b)=>b[1]-a[1]))console.log(k,n,r.get(k),((r.get(k)??0)/n).toFixed(1));
