import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));const from=+process.argv[3],to=+process.argv[4],target=process.argv[5],depth=+process.argv[6];
const nodes=new Map(p.nodes.map(n=>[n.id,n]));const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const self=new Map();let t=0;for(let i=0;i<p.samples.length;i++){t+=p.timeDeltas[i];if(t/1e6>=from&&t/1e6<to)self.set(p.samples[i],(self.get(p.samples[i])??0)+p.timeDeltas[i]);}
const name=id=>{const cf=nodes.get(id).callFrame;return (cf.functionName||"anon")+"@"+decodeURIComponent(cf.url.split("/").slice(-2).join("/")).replace(/[^\x20-\x7e]/g,"").slice(-26)+":"+cf.lineNumber};
const agg=new Map();
for(const [id,t2] of self){const ch=[];let c=id;while(c!==undefined){ch.push(c);c=parent.get(c);}const i=ch.findIndex(x=>name(x).startsWith(target));if(i<0)continue;const k=ch.slice(i+1,i+1+depth).map(name).join(" < ");agg.set(k,(agg.get(k)??0)+t2);}
console.log([...agg].sort((a,b)=>b[1]-a[1]).slice(0,6).map(([k,v])=>(v/1000|0)+"ms "+k).join("\n\n"));
