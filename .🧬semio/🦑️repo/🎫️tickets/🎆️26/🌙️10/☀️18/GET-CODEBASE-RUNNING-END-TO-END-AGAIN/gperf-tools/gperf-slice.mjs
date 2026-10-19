import {readFileSync} from "node:fs";
const p=JSON.parse(readFileSync(process.argv[2],"utf8"));const from=+process.argv[3],to=+process.argv[4];// seconds since profile start
const nodes=new Map(p.nodes.map(n=>[n.id,n]));const parent=new Map();for(const n of p.nodes)for(const c of n.children??[])parent.set(c,n.id);
const self=new Map();let t=0;for(let i=0;i<p.samples.length;i++){t+=p.timeDeltas[i];if(t/1e6>=from&&t/1e6<to)self.set(p.samples[i],(self.get(p.samples[i])??0)+p.timeDeltas[i]);}
const name=id=>{const cf=nodes.get(id).callFrame;return (cf.functionName||"anon")+"@"+decodeURIComponent(cf.url.split("/").slice(-2).join("/")).replace(/[^\x20-\x7e]/g,"").slice(-26)+":"+cf.lineNumber};
const sf=new Map(),incl=new Map();let total=0;
for(const [id,t2] of self){total+=t2;sf.set(name(id),(sf.get(name(id))??0)+t2);const seen=new Set();let c=id;while(c!==undefined){const k=name(c);if(!seen.has(k)){seen.add(k);incl.set(k,(incl.get(k)??0)+t2);}c=parent.get(c);}}
console.log("slice ms",total/1000|0);
console.log("--self");console.log([...sf].sort((a,b)=>b[1]-a[1]).slice(0,14).map(([k,v])=>(v/1000|0)+"ms "+k).join("\n"));
console.log("--incl");console.log([...incl].sort((a,b)=>b[1]-a[1]).slice(3,40).map(([k,v])=>(v/1000|0)+"ms "+k).join("\n"));
