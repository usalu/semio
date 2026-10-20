// 📈 Self and inclusive time of a .cpuprofile restricted to a time window (seconds from start): prof-window.mjs FILE FROM TO [TOP]
import {readFileSync} from "node:fs";
const [file,from=0,to=1e9,top=40]=process.argv.slice(2);const p=JSON.parse(readFileSync(file,"utf8"));
const nodes=new Map(p.nodes.map(n=>[n.id,n])),parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const key=id=>{const cf=nodes.get(id).callFrame;return (cf.functionName||"anon")+" "+decodeURIComponent(cf.url.split("/").slice(-3).join("/")).replace(/[^\x20-\x7e]/g,"").slice(-34)+":"+cf.lineNumber;};
const self=new Map();let t=0,total=0;for(let i=0;i<p.samples.length;i++){t+=p.timeDeltas[i];if(t/1e6>=+from&&t/1e6<+to){self.set(p.samples[i],(self.get(p.samples[i])??0)+p.timeDeltas[i]);total+=p.timeDeltas[i];}}
const incl=new Map(),selfBy=new Map();
for(const [id,v] of self){const seen=new Set();let cur=id;const k0=key(id);selfBy.set(k0,(selfBy.get(k0)??0)+v);while(cur!==undefined){const k=key(cur);if(!seen.has(k)){seen.add(k);incl.set(k,(incl.get(k)??0)+v);}cur=parent.get(cur);}}
console.log("window total ms",Math.round(total/1000));
console.log("--- self");console.log([...selfBy].sort((a,b)=>b[1]-a[1]).slice(0,+top).map(([k,v])=>String(Math.round(v/1000)).padStart(7)+"ms "+k).join("\n"));
console.log("--- inclusive");console.log([...incl].filter(([k])=>!/typescript\.js/.test(k)).sort((a,b)=>b[1]-a[1]).slice(0,+top).map(([k,v])=>String(Math.round(v/1000)).padStart(7)+"ms "+k).join("\n"));
