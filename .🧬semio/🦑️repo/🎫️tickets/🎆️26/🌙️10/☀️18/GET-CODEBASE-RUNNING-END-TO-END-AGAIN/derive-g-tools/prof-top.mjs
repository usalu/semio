// [derive-g] top self time and top inclusive time (by function) of a .cpuprofile. usage: bun prof-top.mjs <file> [urlFilter]
import{readFileSync}from"node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8")),F=process.argv[3]??"";
const nodes=new Map(p.nodes.map(n=>[n.id,n])),self=new Map(),dt=p.timeDeltas,total=dt.reduce((a,b)=>a+b,0);
p.samples.forEach((id,i)=>self.set(id,(self.get(id)??0)+(dt[i]??0)));
const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const key=n=>`${n.callFrame.functionName||"(anon)"} ${n.callFrame.url.split("/").slice(-3).join("/")}:${n.callFrame.lineNumber+1}`;
const bySelf=new Map(),byIncl=new Map();
for(const[id,t]of self){const n=nodes.get(id);const k=key(n);bySelf.set(k,(bySelf.get(k)??0)+t);const seen=new Set();for(let c=id;c!==undefined;c=parent.get(c)){const kk=key(nodes.get(c));if(seen.has(kk))continue;seen.add(kk);byIncl.set(kk,(byIncl.get(kk)??0)+t);}}
const top=(m,n)=>[...m].filter(([k])=>k.includes(F)).sort((a,b)=>b[1]-a[1]).slice(0,n).map(([k,t])=>(t/1000).toFixed(0).padStart(7)+" ms  "+k);
console.log("total ms",Math.round(total/1000));console.log("-- self");console.log(top(bySelf,25).join("\n"));console.log("-- inclusive");console.log(top(byIncl,45).join("\n"));
