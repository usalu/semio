import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8")),target=process.argv[3];
const nodes=new Map(p.nodes.map(n=>[n.id,n]));const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const self=new Map();p.samples.forEach((id,i)=>self.set(id,(self.get(id)??0)+(p.timeDeltas[i]??0)));
const agg=new Map();
for(const [id,t] of self){const n=nodes.get(id);if(n.callFrame.functionName!==target)continue;const chain=[];let x=parent.get(id);while(x&&chain.length<4){const c=nodes.get(x).callFrame;chain.push((c.functionName||"anon")+":"+c.lineNumber+"@"+decodeURIComponent(c.url.split("/").slice(-2).join("/")));x=parent.get(x);}const k=chain.join(" < ");agg.set(k,(agg.get(k)??0)+t);}
for(const [k,t] of [...agg].sort((a,b)=>b[1]-a[1]).slice(0,8))console.log(Math.round(t/1000)+"ms",k);
