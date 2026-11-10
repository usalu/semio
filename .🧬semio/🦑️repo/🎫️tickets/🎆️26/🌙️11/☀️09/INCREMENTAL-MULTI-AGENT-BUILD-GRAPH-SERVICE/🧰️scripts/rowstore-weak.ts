import{readFileSync,readdirSync,statSync,mkdtempSync,rmSync}from"node:fs";
import{join,resolve}from"node:path";
import{heapStats}from"bun:jsc";
const THRESH=Number(process.env.THRESH??3000);const live:{ref:WeakRef<any>,kind:string,stack:string,at:number}[]=[];const marked=new WeakSet<object>();
const patch=(proto:any,method:string,kind:string)=>{const orig=proto[method];proto[method]=function(this:any,...a:any[]){const r=orig.apply(this,a);if(this.size>=THRESH&&!marked.has(this)){marked.add(this);live.push({ref:new WeakRef(this),kind,stack:(new Error().stack??"").split("\n").slice(2,9).map(l=>l.trim().replace(/\(?file:\/\/\/?/,"(").replace(/.*🏗️build\//,"")).join(" | "),at:this.size});}return r;};};
patch(Map.prototype,"set","Map");patch(Set.prototype,"add","Set");
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const cl=await import(join(G,"🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs")),cas=await import(join(B,"🗄️cas/🗃️store/🟦️.ts"));
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
const limit=Number(process.argv[2]??50);
const cacheRoot=mkdtempSync(join(resolve(".🧬semio/🦑️repo/⚡️cache/tests"),"settle-weak-"));
const run=async()=>{
 let index:any=gen.unpackFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024);
 const derived=index.rows.filter((r:any)=>r.kind==="derived").slice(0,limit);
 const pseudoRows=new Map<string,any>();for(const row of derived)pseudoRows.set(cut.derivedOwnerNameV1(row),row);
 index=undefined;
 const deriver:any=await memo.openPlannerDeriverV1(process.cwd());
 const settler:any=cl.createClosureSettlerV1({workspace:{workspaceRoot:process.cwd(),cacheRoot,blobs:cas.createCasStoreV1()}});
 const result=await settler.settle({pseudoRows,derive:deriver,hold:async()=>true});
 return{d:new WeakRef(deriver),s:new WeakRef(deriver.scripts),st:new WeakRef(settler),g:new WeakRef(settler.graph),r:new WeakRef(result)};
};
const w=await run();
for(let i=0;i<4;i++){await new Promise(r=>setTimeout(r,30));Bun.gc(true);}
console.log("heapMB",Math.round(heapStats().heapSize/1048576),"objects",heapStats().objectCount);const out:any[]=[];for(const l of live){const o=l.ref.deref();if(o)out.push({size:o.size,kind:l.kind,stack:l.stack});}out.sort((a,b)=>b.size-a.size);for(const o of out.slice(0,40))console.log("[COLL]",o.kind,o.size,o.stack.slice(0,230));const snapH=(l:string)=>{Bun.gc(true);const h=heapStats();console.log(l,"heapMB",Math.round(h.heapSize/1048576),"extraMB",Math.round(h.extraMemorySize/1048576),"objects",h.objectCount,"rssMB",Math.round(process.memoryUsage().rss/1048576));};
const obs=await import(join(B,"👁️observation/🟨️.mjs")),mem=await import(join(B,"🧠️memory/🟨️.mjs"));
obs.releaseInferenceMemoryV1();snapH("release");mem.graphStringInternerV1.clear();snapH("interner");
console.log("alive deriver",!!w.d.deref(),"scripts",!!w.s.deref(),"settler",!!w.st.deref(),"graph",!!w.g.deref(),"result",!!w.r.deref());
rmSync(cacheRoot,{recursive:true,force:true});process.exit(0);
