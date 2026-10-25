import{readFileSync}from"node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));const nodes=new Map(p.nodes.map(n=>[n.id,n]));
const self=new Map();const dt=p.timeDeltas;let total=0;
p.samples.forEach((id,i)=>{const d=dt[i]??0;total+=d;self.set(id,(self.get(id)??0)+d);});
const byFn=new Map();for(const[id,t]of self){const n=nodes.get(id),k=n.callFrame.functionName+" "+n.callFrame.url.split("/").slice(-3).join("/")+":"+n.callFrame.lineNumber;byFn.set(k,(byFn.get(k)??0)+t);}
const rows=[...byFn].sort((a,b)=>b[1]-a[1]).slice(0,+process.argv[3]||25);
console.log("total ms",(total/1000).toFixed(0));for(const[k,t]of rows)console.log((t/1000).toFixed(0).padStart(7),k.slice(0,140));
