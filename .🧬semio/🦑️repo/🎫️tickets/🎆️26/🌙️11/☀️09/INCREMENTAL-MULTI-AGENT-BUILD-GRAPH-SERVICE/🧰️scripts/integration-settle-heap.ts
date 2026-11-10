import{readFileSync,readdirSync,statSync,mkdtempSync,rmSync}from"node:fs";
import{join,resolve}from"node:path";
import{heapStats}from"bun:jsc";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const cl=await import(join(G,"🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs")),obs=await import(join(B,"👁️observation/🟨️.mjs")),cas=await import(join(B,"🗄️cas/🗃️store/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
let index:any=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
const limit=Number(process.argv[2]??1200),derived=index.rows.filter((r:any)=>r.kind==="derived").slice(0,limit);
const pseudoRows=new Map<string,any>();for(const row of derived)pseudoRows.set(cut.derivedOwnerNameV1(row),row);
index=undefined;
const deriver:any=await memo.openPlannerDeriverV1(process.cwd());
const hold=async(rows:any[])=>true;
const cacheRoot=mkdtempSync(join(resolve(".🧬semio/🦑️repo/⚡️cache/tests"),"settle-heap-"));
const settler=cl.createClosureSettlerV1({workspace:{workspaceRoot:process.cwd(),cacheRoot,blobs:cas.createCasStoreV1()}});
const snap=(label:string)=>{Bun.gc(true);const h=heapStats(),m=process.memoryUsage();console.log(label,JSON.stringify({rssMB:Math.round(m.rss/1048576),heapMB:Math.round(h.heapSize/1048576),extraMB:Math.round(h.extraMemorySize/1048576),objects:h.objectCount,strings:h.objectTypeCounts.string,arrays:h.objectTypeCounts.Array,Sets:h.objectTypeCounts.Set,Maps:h.objectTypeCounts.Map}));};
snap("before");
let ticks=0;const result=await settler.settle({pseudoRows,derive:deriver,hold,progress:async()=>{if(process.env.GC_EVERY&&++ticks%Number(process.env.GC_EVERY)===0)Bun.gc(true);}});
snap("after settle");
const scripts=deriver.scripts,nodes=scripts.walkNodes??new Map();
let nodeCount=0,partArrays=new Set<any>(),rowRefs=0,edges=0;
for(const[,entry]of nodes){const n=entry.node;if(!n)continue;nodeCount++;edges+=n.imports?.length??0;for(const p of[...(n.parts??[]),...(n.shared??[])]){if(!partArrays.has(p)){partArrays.add(p);rowRefs+=p.length;}}}
const rows=new Set<any>();for(const p of partArrays)for(const r of p)rows.add(r);
console.log(JSON.stringify({closures:pseudoRows.size,walkNodes:nodeCount,edges,partArrays:partArrays.size,rowRefsInParts:rowRefs,distinctRows:rows.size,answers:result.answers.size,held:result.held.size}));
// drop experiments
const keep=[...partArrays];
(scripts as any).walkNodes=new Map();snap("after dropping walkNodes");
export{keep};
rmSync(cacheRoot,{recursive:true,force:true});process.exit(0);
