import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));
const nodes=new Map(p.nodes.map(n=>[n.id,n]));const self=new Map();
const dt=p.timeDeltas;let total=0;
p.samples.forEach((id,i)=>{const d=dt[i]??0;self.set(id,(self.get(id)??0)+d);total+=d;});
const byFn=new Map();for(const [id,t] of self){const n=nodes.get(id),c=n.callFrame,k=(c.functionName||"(anon)")+" "+c.url.split("/").slice(-3).join("/")+":"+c.lineNumber;byFn.set(k,(byFn.get(k)??0)+t);}
console.log("total ms",Math.round(total/1000));
for(const [k,t] of [...byFn].sort((a,b)=>b[1]-a[1]).slice(0,25))console.log(Math.round(t/1000)+"ms",k);
