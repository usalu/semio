import{readFileSync,readdirSync,statSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{join,resolve}from"node:path";
import{heapStats}from"bun:jsc";
var THRESH=Number(process.env.THRESH??3000);const live:{ref:WeakRef<any>,kind:string,stack:string,at:number}[]=[];const marked=new WeakSet<object>();
const patch=(proto:any,method:string,kind:string)=>{const orig=proto[method];proto[method]=function(this:any,...a:any[]){const r=orig.apply(this,a);if(this.size>=THRESH&&!marked.has(this)){marked.add(this);live.push({ref:new WeakRef(this),kind,stack:(new Error().stack??"").split("\n").slice(2,9).map(l=>l.trim().replace(/\(?file:\/\/\/?/,"(").replace(/.*🏗️build\//,"")).join(" | "),at:this.size});}return r;};};
if(process.env.ROWSTORE_COLL){patch(Map.prototype,"set","Map");patch(Set.prototype,"add","Set");}
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs")),cut=await import(join(G,"🕰️generations/✂️cutoff/🟨️.mjs")),memo=await import(join(B,"🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"));
const cl=await import(join(G,"🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs")),cas=await import(join(B,"🗄️cas/🗃️store/🟦️.ts"));
const limit=Number(process.argv[2]??1000),label=process.argv[3]??"run",outDir=process.argv[4];
const tick=()=>new Promise<void>(r=>setTimeout(r,40));
const snap=async(l:string)=>{for(let i=0;i<3;i++){await tick();Bun.gc(true);}const h=heapStats(),m=process.memoryUsage();const row={label:l,rssMB:Math.round(m.rss/1048576),heapMB:Math.round(h.heapSize/1048576),extraMB:Math.round(h.extraMemorySize/1048576),objects:h.objectCount,strings:h.objectTypeCounts.string};console.log("[MEASURE]",JSON.stringify(row));return row;};
const rows:any[]=[];const weak:Record<string,WeakRef<any>>={};
const pseudoRows=new Map<string,any>((JSON.parse(readFileSync(join(outDir??".","pseudo.json"),"utf8")) as any[]).slice(0,limit));
rows.push(await snap("start (derived questions only, no index)"));
const cacheRoot=mkdtempSync(join(resolve(".🧬semio/🦑️repo/⚡️cache/tests"),"settle-measure-"));
const settle=async()=>{
 const deriver:any=await memo.openPlannerDeriverV1(process.cwd());
 const settler:any=cl.createClosureSettlerV1({workspace:{workspaceRoot:process.cwd(),cacheRoot,blobs:cas.createCasStoreV1()},...process.env.ROWSTORE_CONCURRENCY?{limits:{...cl.CLOSURE_SETTLE_V1,deriveConcurrency:Number(process.env.ROWSTORE_CONCURRENCY)}}:{}});
theSettler=settler;lastScripts=deriver.scripts;const out=await settler.settle({pseudoRows,derive:deriver,hold:async()=>true,progress:async(e:any)=>{lastProgress=e;if(e.phase==='flush'&&e.done===1)armed=true;if(process.env.ROWSTORE_CHUNKS&&armed&&e.phase==='verification'){armed=false;chunk++;Bun.gc(true);const h=heapStats();if(process.env.ROWSTORE_REACH){const rs=lastScripts?.reachState;console.log('[REACH]',chunk,JSON.stringify({sets:rs?.sets.stats(),reaches:rs?.reaches.size,graph:rs?.graph.size,nodes:rs?.nodes.size,reverse:rs?.reverse.size,walkNodes:lastScripts?.walkNodes?.size,files:lastScripts?.files?.size}));}console.log('[CHUNK]',chunk,'heapMB',Math.round(h.heapSize/1048576),'extraMB',Math.round(h.extraMemorySize/1048576),'objects',h.objectCount,'graph',JSON.stringify(theSettler?.stats?.()));if(process.env.ROWSTORE_TYPES){const now=h.objectTypeCounts as any;if(typesBefore){const d=Object.entries(now).map(([k,v]:any)=>[k,v-(typesBefore[k]??0)]).filter(([,v]:any)=>Math.abs(v)>20000).sort((a:any,b:any)=>b[1]-a[1]);console.log('[TYPES]',chunk,JSON.stringify(d));}typesBefore={...now};}if(process.env.ROWSTORE_COLL&&(chunk===2||chunk===6))dump();}},...process.env.ROWSTORE_ANSWERS?{answers:true}:{}});weak.deriver=new WeakRef(deriver);weak.scripts=new WeakRef(deriver.scripts);weak.settler=new WeakRef(settler);weak.result=new WeakRef(out);return out;
};
let dumped=false;let lastProgress:any=null;const trace:any[]=[];const dump=()=>{dumped=true;const out:any[]=[];for(const l of live){const o=l.ref.deref();if(o)out.push({size:o.size,kind:l.kind,stack:l.stack});}out.sort((a,b)=>b.size-a.size);for(const o of out.slice(0,45))console.log('[COLL]',o.kind,o.size,o.stack.slice(0,260));};
const peak={rssMB:0,heapMB:0,extraMB:0};const sampler=setInterval(()=>{const m=process.memoryUsage(),h=heapStats();if(process.env.ROWSTORE_COLL&&!dumped&&h.heapSize>Number(process.env.ROWSTORE_COLL)*1048576)dump();peak.rssMB=Math.max(peak.rssMB,Math.round(m.rss/1048576));trace.push([Math.round(performance.now()/1000),Math.round(h.heapSize/1048576),Math.round(m.rss/1048576),lastProgress?.phase,lastProgress?.done,lastProgress?.total]);peak.heapMB=Math.max(peak.heapMB,Math.round(h.heapSize/1048576));peak.extraMB=Math.max(peak.extraMB,Math.round(h.extraMemorySize/1048576));},100);
const openView=async()=>{
 const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
 const view:any=gen.openPackedFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024,"fact-index:measure");
 const n=view.rows.length;for(let at=0;at<n;at+=4096){const end=Math.min(n,at+4096);for(let id=at;id<end;id++)view.rows[id].path.length;await tick();}
 for(let id=0;id<view.classes.length;id++)view.classes[id].r.length;
 return view;
};
let lastScripts:any;let chunk=0;let armed=false;let typesBefore:any;let theSettler:any;
const stage=async()=>{
 const view:any=process.env.ROWSTORE_VIEW?await openView():undefined;
 if(view)rows.push(await snap("fact index view open and every row read (refresh verification)"));
 const result:any=await settle();
 rows.push(await snap("settled, result held (owner stage start)"));
 return{view:view?.rows.length,held:result.held.size,answers:result.answers.size,changed:result.changed.size,kept:result.kept.size,failed:result.failed,reused:result.reused};
};
const counts=await stage();
clearInterval(sampler);console.log("[MEASURE] peak during settle",JSON.stringify(peak));if(process.env.ROWSTORE_TRACE){let lastT=-99;for(const r of trace)if(r[0]-lastT>=4){lastT=r[0];console.log("[TRACE]",JSON.stringify(r));}}rows.push(await snap("settled, result dropped"));
if(typeof (Bun as any).shrink==="function"){(Bun as any).shrink();rows.push(await snap("after Bun.shrink()"));}
const obs=await import(join(B,"👁️observation/🟨️.mjs")),mem=await import(join(B,"🧠️memory/🟨️.mjs"));
console.log("[MEASURE] caches",JSON.stringify(obs.inferenceCacheSizesV1()),"interner",JSON.stringify(mem.graphStringInternerV1.stats()));
const bounded=await import(join(B,"🧊️bounded/🟨️.mjs"));console.log("[MEASURE] bounded",JSON.stringify(bounded.boundedCacheStatsV1().filter((c:any)=>c.entries>0).map((c:any)=>[c.name,c.entries,Math.round(c.bytes/1048576)])));
obs.releaseInferenceMemoryV1();mem.graphStringInternerV1.clear();
rows.push(await snap("all caches released (floor)"));
console.log("[MEASURE] counts",JSON.stringify(counts));
if(outDir)writeFileSync(join(outDir,`measure-${label}-${limit}.json`),JSON.stringify({limit,...counts,peak,rows},null,1));
rmSync(cacheRoot,{recursive:true,force:true});process.exit(0);
