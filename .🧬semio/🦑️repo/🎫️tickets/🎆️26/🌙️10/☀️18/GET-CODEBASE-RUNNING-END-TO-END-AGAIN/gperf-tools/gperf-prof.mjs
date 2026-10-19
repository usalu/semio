import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));
const nodes=new Map(p.nodes.map(n=>[n.id,n]));
const self=new Map();
const dt=p.timeDeltas;
for(let i=0;i<p.samples.length;i++){self.set(p.samples[i],(self.get(p.samples[i])??0)+dt[i]);}
const total=[...self.values()].reduce((a,b)=>a+b,0);
console.log("total ms",total/1000);
const byFn=new Map();
for(const [id,t] of self){const n=nodes.get(id),cf=n.callFrame;const k=cf.functionName+" "+cf.url.split("/").slice(-3).join("/")+":"+cf.lineNumber;byFn.set(k,(byFn.get(k)??0)+t);}
console.log([...byFn].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,t])=>(t/1000|0)+"ms "+k).join("\n"));
// inclusive
const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const incl=new Map();
for(const [id,t] of self){const seen=new Set();let cur=id;while(cur!==undefined){const cf=nodes.get(cur).callFrame;const k=cf.functionName+" "+cf.url.split("/").slice(-3).join("/")+":"+cf.lineNumber;if(!seen.has(k)){seen.add(k);incl.set(k,(incl.get(k)??0)+t);}cur=parent.get(cur);}}
console.log("--- inclusive");
console.log([...incl].sort((a,b)=>b[1]-a[1]).slice(0,45).map(([k,t])=>(t/1000|0)+"ms "+k).join("\n"));
