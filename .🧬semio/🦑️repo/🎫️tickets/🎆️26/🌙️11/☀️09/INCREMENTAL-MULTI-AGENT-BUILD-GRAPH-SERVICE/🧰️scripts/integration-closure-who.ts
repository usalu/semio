import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
const G=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const index=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const sel=gen.selectFactClassesV1(index,undefined),ver=cut.verifiableRowIdsV1(index,sel.ids);
const per=new Map();
for(const at of sel.classes){const cls=index.classes[at];const ps=cls.o.filter(o=>index.owners[o].startsWith("derived")||cut.isDerivedOwnerV1(index.owners[o])).length;if(!ps)continue;for(const id of cls.r){per.set(id,(per.get(id)??0)+ps);}}
const top=[...per].sort((a,b)=>b[1]-a[1]).slice(0,25);
for(const[id,n]of top)console.log(n,index.rows[id].kind,String(index.rows[id].path).slice(-90));
const raw=ver.filter(id=>index.rows[id].kind==="file");console.log("example stale (first file row):",index.rows[raw[0]].path.slice(-80),per.get(raw[0]));
const kinds={};for(const[id,n]of per)kinds[index.rows[id].kind]=(kinds[index.rows[id].kind]??0)+1;console.log(JSON.stringify(kinds));
const counts=raw.map(id=>per.get(id)??0).sort((a,b)=>a-b),q=p=>counts[Math.floor(p*(counts.length-1))];
console.log("file rows",counts.length,"withSuspects",counts.filter(n=>n>0).length,"p50",q(.5),"p90",q(.9),"p99",q(.99),"max",counts.at(-1));
const hot=raw.filter(id=>(per.get(id)??0)>100).map(id=>[per.get(id),index.rows[id].path.slice(-70)]).sort((a,b)=>b[0]-a[0]);console.log("hot",hot.length);for(const h of hot.slice(0,15))console.log(...h);
