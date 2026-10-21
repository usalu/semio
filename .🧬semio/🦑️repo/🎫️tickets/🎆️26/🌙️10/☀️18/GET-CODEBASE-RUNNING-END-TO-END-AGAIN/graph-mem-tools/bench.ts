// 🧠️ Memory/time harness for the per-project inference loop of the cold graph build on a small sample, in a scratch cache (live tree read-only).
// env: N projects, MIX cargo share, CONC workers, VARIANT=live|<ops.json>, SCRATCH, KEEP=1 keeps records (warm), SERIES=1 forced-GC checkpoints, RSSMAX_MB abort bound, RETAIN=1 keeps the project values like the real loop, DEBUGEXPORT=1 exposes module-private maps.
import {readFileSync,existsSync,mkdirSync,rmSync,writeFileSync,readdirSync,statSync} from "node:fs";import {join,dirname} from "node:path";import {createHash} from "node:crypto";import {heapStats} from "bun:jsc";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),strip=(s:string)=>s.replace(/️/g,"");
const env=process.env,N=Number(env.N??"60"),MIX=Number(env.MIX??"0.5"),CONC=Number(env.CONC??"8"),VARIANT=env.VARIANT??"live",SCRATCH=env.SCRATCH??"/private/tmp/graph-mem/run",RSSMAX=Number(env.RSSMAX_MB??"3700")*1e6,WALLMAX=Number(env.WALLMAX_S??"520")*1e3;
if(!env.KEEP)rmSync(SCRATCH,{recursive:true,force:true});mkdirSync(join(SCRATCH,"np"),{recursive:true});
type Op={anchor:string;replacement:string};const opsByPath:Record<string,Op[]>=VARIANT==="live"?{}:JSON.parse(readFileSync(VARIANT,"utf8")).edits;
const newModules:Record<string,string>=VARIANT==="live"?{}:JSON.parse(readFileSync(VARIANT,"utf8")).newModules??{};
const applyOps=(text:string,ops:Op[],path:string)=>{let out=text;for(const {anchor,replacement} of ops){if(out.includes(replacement)&&!out.includes(anchor))continue;if(out.split(anchor).length!==2)throw Error("anchor drift in "+path+": "+anchor.slice(0,70));out=out.replace(anchor,()=>replacement);}return out;};
const scratchDir=(name:string)=>`(process.getBuiltinModule("fs").mkdirSync(${JSON.stringify(join(SCRATCH,name))},{recursive:true}),${JSON.stringify(join(SCRATCH,name))})`;
const DEBUG=env.DEBUGEXPORT==="1";
Bun.plugin({name:"graph-mem-patches",setup(b){
 b.onLoad({filter:/(?:inference\/[^/]*\/[^/]*|inputs\/[^/]*|observation\/[^/]*)\.mjs$/},async args=>{
 const key=Object.keys(opsByPath).find(k=>args.path.endsWith(k)),isInputs=args.path.endsWith("🏗️build/📥️inputs/🟨️.mjs");
 let t=await Bun.file(args.path).text();
 if(key)t=applyOps(t,opsByPath[key]!,key);
 for(const staged of Object.values(newModules))t=t.replace(/"(?:\.\.\/)+🧠️memory\/🟨️\.mjs"/gu,()=>JSON.stringify(staged));
 if(DEBUG&&args.path.endsWith("👁️observation/🟨️.mjs"))t+=`\nexport const __debugV1={projectImages,originalInferencePublications,internedObservations,usedInference,settledReads,sourceAnalysisImages,loadObservations,blocksByIdentity,blocksByDigest,blocksWritten,projectComputations,clear(name){({projectImages,originalInferencePublications,internedObservations,usedInference,settledReads,sourceAnalysisImages,loadObservations,blocksByIdentity,blocksByDigest,blocksWritten,projectComputations})[name].clear();}};\n`;
 if(isInputs)t=t.replace('repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures")',scratchDir("lock")).replace('repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures")',scratchDir("input"));
 return {contents:t,loader:"js"};});
}});
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const observation:any=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const {cacheInternals,createScriptInputCache}=inference;
const gitFiles=new TextDecoder().decode(Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z"],{cwd:W}).stdout).split("\0");
const ig=new Set(["🧫️fixtures","🧫️examples","🧪️fixtures","🧪️examples"].map(strip));
const excluded=(env.EXCLUDE??"").split(";").filter(Boolean);const eligible=gitFiles.filter(f=>!excluded.some(x=>f.includes(x))).filter(f=>f&&strip(f.split("/").at(-1)!)===strip("📋️project.json")&&existsSync(join(W,f))&&(s=>s[0]!==strip(".🧬semio")&&!s.slice(0,-1).some(x=>ig.has(x)))(f.split("/").map(strip))).sort();
const projectRoots=new Set(eligible.map(f=>dirname(f)));
const cargoOnly=gitFiles.filter(f=>f.endsWith("/Cargo.toml")&&!projectRoots.has(dirname(f))&&!f.split("/").map(strip).some(x=>x===strip(".🧬semio")||ig.has(x)||x==="compose"||x==="temp")&&existsSync(join(W,f))).sort();
const nCargo=Math.round(N*MIX),nJson=N-nCargo;
const pick=(list:string[],k:number)=>{const step=Math.max(1,Math.floor(list.length/Math.max(k,1)));return list.filter((_,i)=>i%step===0).slice(0,k);};
const only=env.ONLY?env.ONLY.split(";"):undefined;const sample:string[]=only?eligible.filter(f=>only.some(o=>f.includes(o))).slice(0,N):[...pick(eligible,nJson),...pick(cargoOnly,nCargo)];
if(env.INDEX!==undefined){const keep=sample[Number(env.INDEX)]!;sample.length=0;sample.push(keep);console.log("INDEX",env.INDEX,keep);}
const context={workspaceRoot:W,get invocation(){return undefined;}};
const scripts=createScriptInputCache(context);
const t0=performance.now();
const commandInputs=await cacheInternals.nativeTargetCommandInputs({},W,undefined,scripts,".");
const tCommand=performance.now()-t0;
const jobs:any[]=[];
for(const file of sample){
 const root=dirname(file)||".";
 if(file.endsWith("project.json")){jobs.push({file,root,json:JSON.parse(readFileSync(join(W,file),"utf8"))});continue;}
 try{const manifest=Bun.TOML.parse(readFileSync(join(W,file),"utf8")) as any;if(!manifest.package?.name)continue;
  const targets={...await cacheInternals.cargoTargets(root,W,commandInputs,scripts),...await cacheInternals.componentBuildTargets(root,W)};
  jobs.push({file,root,json:{name:manifest.package.name,root,projectType:"library",tags:["language:cargo","discovery:manifest"],targets}});}catch(e:any){console.error("skip",file,e.message?.slice(0,100));}
}
const owned:{name:string;root:string;cachePath:string;value:any}[]=[];const failures:string[]=[],retained:any[]=[],timings:{root:string;ms:number}[]=[];let cursor=0,peakRss=0,peakHeap=0,aborted=false;
const poll=setInterval(()=>{const m=process.memoryUsage();peakRss=Math.max(peakRss,m.rss);peakHeap=Math.max(peakHeap,heapStats().heapSize);if(m.rss>RSSMAX){aborted=true;console.error("ABORT rss",Math.round(m.rss/1e6),"MB");cursor=jobs.length;}},200);
const mb=(n:number)=>Math.round(n/1e6);
const series:any[]=[];
let firstTypes:Record<string,number>|undefined;const snap=(done:number)=>{if(env.SERIES!=="1")return;if(env.TYPES==="1"){Bun.gc(true);const t=heapStats().objectTypeCounts as Record<string,number>;if(!firstTypes)firstTypes={...t};else if(done>=Number(env.TYPESAT??"40")){const d=Object.entries(t).map(([k,v])=>[k,v-(firstTypes![k]??0)] as [string,number]).filter(([,v])=>v>0).sort((a,b)=>b[1]-a[1]).slice(0,14);console.log("TYPES delta",done,JSON.stringify(d));firstTypes={...t};}}const g0=performance.now();Bun.gc(true);const gcMs=Math.round(performance.now()-g0),h=heapStats();series.push({done,gcMs,heapMB:mb(h.heapSize),extraMB:mb(h.extraMemorySize),objects:h.objectCount,rssMB:mb(process.memoryUsage().rss)});};
const inferOne=async({root,json}:any)=>{
 const started=performance.now();
 const run=async()=>{try{return await cacheInternals.projectWithDefaults(json,root,join(W,root),W,{},new Map(),commandInputs,scripts,undefined,"hash",undefined,[]);}catch(e:any){failures.push(root+": "+String(e?.message).slice(0,90));return {failed:true};}};
 const cachePath=join(SCRATCH,"np",createHash("sha256").update(root).digest("hex").slice(0,24)+".json");
 const value=await observation.observeProjectInferenceV1({cachePath,key:["bench",root,json],owner:root,reusable:()=>true},run);
 if(env.RETAIN!=="0")retained.push(value);owned.push({name:json.name,root,cachePath,value});
 timings.push({root,ms:performance.now()-started});if(env.TRACE==="1"){const h=heapStats();console.log("TRACE",timings.length,mb(h.heapSize),mb(h.extraMemorySize),Math.round(performance.now()-started),root.slice(-60));}if(timings.length%Number(env.EVERY??"10")===0)snap(timings.length);
};
const cpu0=process.cpuUsage(),t1=performance.now();
const loop=async()=>{const workers=Array.from({length:CONC},async()=>{while(cursor<jobs.length){if(performance.now()-t1>WALLMAX){cursor=jobs.length;aborted=true;break;}await inferOne(jobs[cursor++]);}});await Promise.all(workers);return null;};
snap(0);
if(env.OUTER==="1")await observation.observeInference({cachePath:join(SCRATCH,"emoji.json"),key:["bench",Date.now()]},()=>observation.settleObservedComputationV1(loop,{}));
else await (observation.observeInferenceSnapshotV1?observation.observeInferenceSnapshotV1(loop):loop());
const cpu=process.cpuUsage(cpu0),wall=performance.now()-t1;
const phaseStats:any[]=[];let phasePeak=0;const phasePoll=setInterval(()=>{phasePeak=Math.max(phasePeak,heapStats().heapSize+heapStats().extraMemorySize);},50);
const phase=async<T>(name:string,run:()=>Promise<T>|T):Promise<T>=>{Bun.gc(true);const before=heapStats().heapSize+heapStats().extraMemorySize;phasePeak=before;const t=performance.now(),c=process.cpuUsage();const out=await run();const cpuUsed=process.cpuUsage(c);phaseStats.push({phase:name,ms:Math.round(performance.now()-t),cpuMs:Math.round((cpuUsed.user+cpuUsed.system)/1e3),beforeMB:mb(before),peakMB:mb(phasePeak),afterMB:mb(heapStats().heapSize+heapStats().extraMemorySize)});return out;};
if(env.DOWNSTREAM==="1"){
 const generations:any=await import(join(L,"🏗️build/🕸️graph/🕰️generations/🟨️.mjs")),load:any=await import(join(L,"🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs"));
 const rowsOf=(cachePath:string)=>observation.projectInferenceObservationsV1(cachePath)??[];
 const part:any=await phase("part",()=>{const all=new Map<string,any>();const owners=owned.map(o=>{const rows=rowsOf(o.cachePath);for(const row of rows)all.set(generations.rowKeyV1(row),row);return {name:o.name,root:o.root,node:o.value,nodeRows:rows};});return {owners,edgeOnly:[],edges:[],externalNodes:{},parseUnavailable:[],rows:[...all.values()],env:{digest:"e".repeat(64),rows:[]},replaceTags:[]};});
 owned.length=0;retained.length=0;
 const shaped:any=await phase("shape",()=>load.shapeComputeResultV1({workspaceRoot:W,part,stubs:{}}));
 const storeRoot=join(SCRATCH,"generations");rmSync(storeRoot,{recursive:true,force:true});
 const frontier=generations.createFrontierV1({directories:[],members:[],capturedAt:0});
 const engine=generations.createGenerationStoreV1({root:storeRoot,holds:async()=>true,resources:{facts:"facts",builder:"builder"}});
 const result:any=await phase("refresh",()=>engine.refresh({identity:"a".repeat(64),policy:"fresh",membership:async()=>({digest:"b".repeat(64),frontier,changes:[]}),compute:async()=>shaped,signal:undefined,progress:undefined}));
 let bytes=0;for(const dir of ["o","x","m"]){try{for(const f of readdirSync(join(storeRoot,dir)))bytes+=statSync(join(storeRoot,dir,f)).size;}catch{}}
 const assembled:any=await phase("assemble",()=>engine.assemble(result.current,{compare:(a:string,b:string)=>a<b?-1:a>b?1:0}));
 let indexBytes=0;try{for(const f of readdirSync(join(storeRoot,"x")))indexBytes+=statSync(join(storeRoot,"x",f)).size;}catch{}
 console.log("DOWNSTREAM "+JSON.stringify({owners:part.owners.length,distinctRows:part.rows.length,storedMB:mb(bytes),indexPackedMB:mb(indexBytes),projectsAssembled:Object.keys(assembled.projects).length,phases:phaseStats}));
}
clearInterval(phasePoll);

clearInterval(poll);
Bun.gc(true);const end=heapStats();if(env.SNAP){const snapshot:any=(Bun as any).generateHeapSnapshot("v8");writeFileSync(env.SNAP,typeof snapshot==="string"?snapshot:JSON.stringify(snapshot));}
let dbg:any;
if(DEBUG&&observation.__debugV1){dbg={};const total=()=>heapStats().heapSize+heapStats().extraMemorySize;
 const before0=(Bun.gc(true),Bun.gc(true),total());retained.length=0;owned.length=0;Bun.gc(true);Bun.gc(true);dbg.retainedValues={releasedMB:mb(before0-total())};
 if(env.SCRIPTS==="1"){dbg.scriptsKeys=Object.keys(scripts);for(const [k,v] of Object.entries(scripts)){if(v instanceof Map||v instanceof Set){const before=total();const size=v.size;v.clear();Bun.gc(true);Bun.gc(true);dbg["scripts."+k]={entries:size,releasedMB:mb(before-total())};}}}
 for(const name of ["projectImages","originalInferencePublications","usedInference","settledReads","sourceAnalysisImages","internedObservations","blocksByIdentity","blocksByDigest","blocksWritten"]){const before=total();const size=observation.__debugV1[name].size;observation.__debugV1.clear(name);Bun.gc(true);Bun.gc(true);dbg[name]={entries:size,releasedMB:mb(before-total())};}}
const ms=timings.map(t=>t.ms).sort((a,b)=>a-b);
const valuesBytes=env.SIZES==="1"?retained.reduce((n,v)=>n+JSON.stringify(v).length,0):undefined;
const np=join(SCRATCH,"np");let recordBytes=0,recordCount=0;try{for(const f of readdirSync(np)){if(f.endsWith(".json")){recordBytes+=statSync(join(np,f)).size;recordCount++;}}}catch{}
console.log(JSON.stringify({eligibleJson:eligible.length,cargoOnly:cargoOnly.length,variant:VARIANT==="live"?"live":VARIANT.split("/").slice(-2).join("/"),projects:jobs.length,done:timings.length,aborted,conc:CONC,commandInputsMs:Math.round(tCommand),wallS:+(wall/1000).toFixed(1),cpuS:+((cpu.user+cpu.system)/1e6).toFixed(1),medianMs:Math.round(ms[ms.length>>1]??0),p90Ms:Math.round(ms[Math.floor(ms.length*.9)]??0),peakRssMB:mb(peakRss),peakHeapMB:mb(peakHeap),endHeapMB:mb(end.heapSize),endExtraMB:mb(end.extraMemorySize),endRssMB:mb(process.memoryUsage().rss),recordCount,recordMB:mb(recordBytes),valuesMB:valuesBytes===undefined?undefined:mb(valuesBytes),failures:failures.length,stats:observation.sourceObservationStatsV1?.(),dbg}));
if(series.length)console.log("SERIES "+JSON.stringify(series));
if(failures.length)console.log("FAILURES",JSON.stringify(failures.slice(0,5)));
if(env.DUMPVALUES){writeFileSync(env.DUMPVALUES,JSON.stringify(Object.fromEntries(jobs.map((j,i)=>[j.root,0]))));}
process.exit(0);
