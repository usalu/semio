import{readFileSync}from"node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));const nodes=new Map(p.nodes.map(n=>[n.id,n]));
const self=new Map();const dt=p.timeDeltas;const total=dt.reduce((a,b)=>a+b,0);
p.samples.forEach((id,i)=>self.set(id,(self.get(id)||0)+dt[i]));
const byFn=new Map();for(const[id,t]of self){const n=nodes.get(id);const k=n.callFrame.functionName+" "+n.callFrame.url.split("/").slice(-3).join("/").slice(-60)+":"+n.callFrame.lineNumber;byFn.set(k,(byFn.get(k)||0)+t);}
console.log("total ms",total/1000);
for(const[k,t]of[...byFn].sort((a,b)=>b[1]-a[1]).slice(0,35))console.log((t/1000).toFixed(0).padStart(7),k);
// inclusive by function name
const parent=new Map();for(const n of p.nodes)for(const c of n.children||[])parent.set(c,n.id);
const incl=new Map();for(const[id,t]of self){const seen=new Set();let cur=id;while(cur!==undefined){const n=nodes.get(cur);const k=n.callFrame.functionName+":"+n.callFrame.lineNumber;if(!seen.has(k)){seen.add(k);incl.set(k,(incl.get(k)||0)+t);}cur=parent.get(cur);}}
console.log("--- inclusive");for(const[k,t]of[...incl].sort((a,b)=>b[1]-a[1]).slice(0,45))console.log((t/1000).toFixed(0).padStart(7),k);
