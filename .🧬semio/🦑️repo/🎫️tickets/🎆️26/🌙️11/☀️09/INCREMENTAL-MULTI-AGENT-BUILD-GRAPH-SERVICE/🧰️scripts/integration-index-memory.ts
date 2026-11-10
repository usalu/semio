import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
import{statSync as statSyncX}from"node:fs";
import{heapStats}from"bun:jsc";
const G=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const snap=(label:string)=>{Bun.gc(true);const h=heapStats(),m=process.memoryUsage();console.log(label,JSON.stringify({rssMB:Math.round(m.rss/1048576),heapMB:Math.round(h.heapSize/1048576),objects:h.objectCount,strings:h.objectTypeCounts.string??h.objectTypeCounts.String,Objects:h.objectTypeCounts.Object,arrays:h.objectTypeCounts.Array}));};
snap("start");
const bytes=readFileSync(join(dir,file.n));console.log("file MB",(bytes.length/1048576).toFixed(1));
const index=gen.unpackFactIndexV1(bytes,512*1024*1024);snap("index");
const kinds={};for(const r of index.rows)kinds[r.kind]=(kinds[r.kind]??0)+1;console.log(JSON.stringify(kinds),"classes",index.classes.length,"owners",index.owners.length);
let pathBytes=0,fileFactStrings=0;for(const r of index.rows){pathBytes+=typeof r.path==="string"?r.path.length:0;}console.log("pathChars MB",(pathBytes/1048576).toFixed(0));
const cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),doubtMod=await import(join(G,"🕰️generations/🔎️doubt/🟨️.mjs")),stampsMod=await import(join(G,"🕰️generations/🗂️directory-stamps/🟨️.mjs"));
const sel=gen.selectFactClassesV1(index,undefined),verifiable=cut.verifiableRowIdsV1(index,sel.ids);snap("selected");
const stampFile="/tmp/semio-ia-stamps.json",observedAt=process.env.OBSERVED_NOW?Date.now():statSyncX(join(dir,file.n)).mtimeMs-5*60_000;
const publisher=doubtMod.createEpochDoubtV1({stamps:stampsMod.createDirectoryStampsV1({file:stampFile})});
let t=performance.now();await publisher.published({rows:index.rows,observedAt});console.log("published stamps",Math.round(performance.now()-t),"ms");
const doubt=doubtMod.createEpochDoubtV1({stamps:stampsMod.createDirectoryStampsV1({file:stampFile})});
t=performance.now();const candidates=await doubtMod.selectDoubtfulRowsV1(index,verifiable,doubt,{chunk:8192});snap("doubt");console.log("doubtful",candidates.length,"of",verifiable.length,Math.round(performance.now()-t),"ms");
const byKind={};for(const id of candidates)byKind[index.rows[id].kind]=(byKind[index.rows[id].kind]??0)+1;console.log(JSON.stringify(byKind));
const{existingEntryV1}=doubtMod;const why={changedDir:0,symlink:0,other:0};const dirs=new Map();
for(const id of candidates){const row=index.rows[id];if(row.kind!=="exists"&&row.kind!=="fileExists"&&row.kind!=="directoryExists")continue;const e=existingEntryV1(row);if(!e){why.other++;continue;}
 let m=dirs.get(e.directory);if(m===undefined){try{m=statSyncX(e.directory).mtimeMs;}catch{m=-1;}dirs.set(e.directory,m);}
 if(m>observedAt-2000||m<0)why.changedDir++;else why.symlink++;}
console.log("exists-family doubtful reasons",JSON.stringify(why),"distinct dirs",dirs.size);
if(process.env.VERIFY){
 const producer=await import(join(G,"📥️inference/🟨️.mjs"));await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();snap("analyses prepared");
 const obs=await import(join(G,"../👁️observation/🟨️.mjs"));
 const hold=async(rows:any[],signal?:AbortSignal)=>{const own=rows.filter((r:any)=>r.kind!=="closure"&&r.kind!=="derived"&&!(r.kind==="field"&&r.fact?.extract!==undefined));return own.length===0?true:obs.observationsHold(own,64,new Set(),true,signal);};
 t=performance.now();const{stale}=await gen.staleFactRowsV1(index,candidates,hold,{budget:Infinity});snap("verified");console.log("stale",stale.size,Math.round(performance.now()-t),"ms");
 const split=cut.splitStaleConsumersV1(index,sel.classes,stale);console.log("direct owners",split.direct.size,"suspect closures",split.suspects.size);
 {const per=new Map<number,number>();for(const at of sel.classes){const c=index.classes[at];const ps=c.o.filter((o:number)=>cut.isDerivedOwnerV1(index.owners[o])).length;if(!ps)continue;for(const id of c.r)if(stale.has(id))per.set(id,(per.get(id)??0)+ps);}
 const top=[...per].sort((a,b)=>b[1]-a[1]).slice(0,14);for(const[id,n]of top)console.log(n,index.rows[id].kind,String(index.rows[id].path).slice(-90));
 const hist=[...per.values()].sort((a,b)=>b-a);console.log("stale rows reaching closures",hist.length,"max",hist[0],"p50",hist[hist.length>>1]);}
 const staleKinds={};for(const id of stale)staleKinds[index.rows[id].kind]=(staleKinds[index.rows[id].kind]??0)+1;console.log(JSON.stringify(staleKinds));
}
process.exit(0);
