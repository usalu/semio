// [gram-g] graph-free harness: the production parent loop (emojiProjectJsonNodes) over the tree in ONE process, scratch fact store (staging scope), rss/heap sampling.
// env: LIMIT=N projects (default all), STORE=scratch dir, RSS_CAP_MB=abort bound, SAMPLE_S=5, OUT=json result path, SNAP=heap snapshot path at LIMIT, SEMIO_GRAPH_WORKERS=0|N
import{readFileSync,mkdirSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{join,dirname}from"node:path";
import{heapStats}from"bun:jsc";
const ROOT=process.env.GRAM_ROOT??"/Users/ueli/Documents/semio",L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",E=process.env;
const LIMIT=Number(E.LIMIT??"0"),STORE=E.STORE??"/private/tmp/gram-g/store",CAP=Number(E.RSS_CAP_MB??"6000")*1048576,SAMPLE=Number(E.SAMPLE_S??"5")*1000,OUT=E.OUT;
if(!E.KEEP)rmSync(STORE,{recursive:true,force:true});mkdirSync(STORE,{recursive:true});
const mb=n=>Math.round(n/1048576);
const t0=performance.now(),series=[];let peakRss=0,count=0,aborted=false,lastCount=0;
const probe=tag=>{const m=process.memoryUsage(),h=heapStats();peakRss=Math.max(peakRss,m.rss);const row={t:Math.round((performance.now()-t0)/1000),tag,n:count,rss:mb(m.rss),heap:mb(h.heapSize),extra:mb(h.extraMemorySize),objs:h.objectCount};series.push(row);return row;};
const timer=setInterval(()=>{const r=probe("tick");console.error("[DEBUG] "+JSON.stringify(r));if(E.WALL_S&&r.t>Number(E.WALL_S)){console.error("[DEBUG] WALL limit");aborted=true;finish();}if(r.rss*1048576>CAP){aborted=true;console.error("[DEBUG] ABORT rss "+r.rss+" MB");finish();}},SAMPLE);
let finished=false;
const finish=()=>{if(finished)return;finished=true;clearInterval(timer);probe("end");const out={limit:LIMIT,projects:count,aborted,seconds:Math.round((performance.now()-t0)/100)/10,peakRssMB:mb(peakRss),cpu:process.cpuUsage(),series};if(OUT)writeFileSync(OUT,JSON.stringify(out));console.log(JSON.stringify({...out,series:undefined,cpu:undefined,lastSeries:series.slice(-3)}));process.exit(aborted?3:0);};

const PROBES=[
 ["const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });","p:enter"],
 ["configFiles=owners.files;","p:owners"],
 ['advanceScriptInvocation(context.invocation,"build:commands","complete");',"p:commands"],
 ["const playgroundCatalog = collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions);","p:catalog"],
 ["const declaredRoots = new Set([...rootsByName.values()]);","p:configs"],
 ["const projects = results.flatMap(([, result]) => Object.values(result.projects));","p:cargo"],
 ['await phases?.begin("inference",jobs.length);',"p:jobs"],
 ["if (collector) await collector.finish({ projects: jobs.length });","p:inferred"],
];
const START_ANCHOR="const started=performance.now();";
if(E.MARKS==="1")Bun.plugin({name:"gram-marks",setup(b){b.onLoad({filter:/graph\/[^/]*\.ts$/},async args=>{if(!args.path.endsWith("🕸️graph/🟦️.ts"))return undefined;let t=await Bun.file(args.path).text();
 const start=t.indexOf("export async function discoverBuildGraphPartV1("),end=t.indexOf("/** 🧺️ Keeps exactly the discovered files",start);let part=t.slice(start,end);
 const mark=(anchor,label)=>{const i=part.indexOf(anchor);if(i<0||part.indexOf(anchor,i+1)>=0){console.error("[DEBUG] mark anchor problem "+label);return;}part=part.slice(0,i)+'globalThis.__gramMark?.("'+label+'");'+part.slice(i);};
 mark("const projects:Record<string,any>={},externalNodes","part:primary");mark("const tagged=stubs.filter(([,stub])=>stub.tag)","part:env");mark("const policyOwners=Object.assign(","part:tests");mark("merge(testContributions as [string,any][],false);","part:policies");mark("const edges=[...await owner.discoverBuildDependenciesV1(","part:testedges");mark("const metadata=await owner.createMetadata(","part:edges");mark("return {...captured.value,rows:","part:captured");mark("if(!retained)await discoveredForTests(()=>withholdDangling","m:withhold");mark("const refresh=edgeRefreshOwnersV1(","m:refresh");mark("const ownerOf=createProjectFileIndexV1(contextProjects)","m:ownerOf");mark("for(const file of files){const found=ownerOf(file)","m:fileMap");mark("const graphContext={workspaceRoot:root","m:graphContext");mark("const testEdges:unknown=retained","m:testEdges");
 return {contents:t.slice(0,start)+part+t.slice(end),loader:"ts"};});}});
globalThis.__gramMark=label=>{if(E.PROBE_GC==="1")Bun.gc(true);const r=probe(label);console.error("[DEBUG] MARK "+JSON.stringify(r));};
if(E.FIXGOV==="1")Bun.plugin({name:"gram-fixgov",setup(b){b.onLoad({filter:/memory\/[^/]*\.mjs$/},async args=>{if(!args.path.endsWith("🧠️memory/🟨️.mjs"))return undefined;let t=await Bun.file(args.path).text();const x='signal?.addEventListener("abort",()=>fail(signal.reason),{once:true});';if(!t.includes(x))throw Error("governor anchor missing");return {contents:t.replace(x,'if(signal){const handler=()=>fail(signal.reason);signal.addEventListener("abort",handler,{once:true});(globalThis.__gramDisposers??=[]).push(()=>signal.removeEventListener("abort",handler));}'),loader:"js"};});}});
Bun.plugin({name:"gram-harness",setup(b){b.onLoad({filter:/inputs\/[^/]*storage\/[^/]*\.mjs$/},async args=>{if(!args.path.endsWith("📥️inputs/💾️storage/🟨️.mjs"))return undefined;let t=await Bun.file(args.path).text();const x="export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return withGraphFactWriterV1(workspace,invocation,phaseControl,body);}";if(!t.includes(x))throw Error("storage anchor missing");return {contents:t.replace(x,"export async function withRepositoryBuildGraphFactStoreV1(workspace,invocation,phaseControl,body){return body({root:'/dev/null'},phaseControl);}"),loader:"js"};});b.onLoad({filter:/observation\/[^/]*reuse\/[^/]*\.mjs$/},async args=>{if(!args.path.endsWith("👁️observation/📥️reuse/🟨️.mjs"))return undefined;let t=await Bun.file(args.path).text();const a="export async function inferenceInventoryReferencesHoldV1(root,value,control){";if(!t.includes(a))throw Error("reuse anchor missing");return {contents:t.replace(a,a+"return true;"),loader:"js"};});}});
if(E.PROBES!=="0")Bun.plugin({name:"gram-probes",setup(b){b.onLoad({filter:/observation\/[^/]*\.mjs$/},async args=>{if(!args.path.endsWith("👁️observation/🟨️.mjs"))return undefined;let t=await Bun.file(args.path).text();return {contents:t+"\nglobalThis.__gramObs={release:releaseInferenceMemoryV1,maps:{usedInference,originalInferencePublications,loadObservations,projectComputations,projectImages,sourceAnalyses,sourceAnalysisImages,internedObservations,settledReads,verifiedBlocks,blocksByIdentity,registryOwnersV1,recordStores,recordStoreHandles,inferenceStamps},clear(name){const m=this.maps[name];if(!m)throw Error('unknown map '+name);const n=m.size;m.clear();console.error('[DEBUG] TRIAL cleared '+name+' entries '+n);}};\n",loader:"js"};});
b.onLoad({filter:/native-manifests\/[^/]*\.mjs$/},async args=>{if(!args.path.endsWith("🦀️native-manifests/🟨️.mjs"))return undefined;let t=await Bun.file(args.path).text();return {contents:t+"\nglobalThis.__gramShared=NATIVE_DIRECTORY_CLOSURES;\n",loader:"js"};});b.onLoad({filter:/project-nodes\/[^/]*\.mjs$/},async args=>{
 if(!args.path.endsWith("📦️project-nodes/🟨️.mjs"))return undefined;
 let t=await Bun.file(args.path).text();
 for(const [anchor,label] of PROBES){const at=t.indexOf(anchor);if(at<0){console.error("[DEBUG] probe anchor missing "+label);continue;}const end=at+anchor.length;t=t.slice(0,end)+'\n;globalThis.__gramProbe?.("'+label+'",{get owners(){try{return owners}catch{}},get results(){try{return results}catch{}},get scripts(){try{return scripts}catch{}},get projectEnvironment(){try{return projectEnvironment}catch{}},get jobs(){try{return jobs}catch{}},get nodeRowsHolder(){return undefined}});\n'+t.slice(end);}
 {const at=t.indexOf(START_ANCHOR);if(at>=0){const end=at+START_ANCHOR.length;t=t.slice(0,end)+'globalThis.__gramStart?.(json.name);'+t.slice(end);}}
 return {contents:t,loader:"js"};});}});
let startCount=0;globalThis.__gramStart=name=>{if(E.SNAP_N&&++startCount===Number(E.SNAP_N)){Bun.gc(true);Bun.gc(true);probe("snapN");const sn=Bun.generateHeapSnapshot("v8");writeFileSync(STORE+".snapN.json",typeof sn==="string"?sn:JSON.stringify(sn));console.error("[DEBUG] snapshotN written "+startCount);finish();}if(E.PER_START==="1")console.error("[DEBUG] S "+Math.round(performance.now()-t0)/1000+" rss "+mb(process.memoryUsage().rss)+" "+name);};
const live=()=>{Bun.gc(true);Bun.gc(true);const h=heapStats();return h.heapSize+h.extraMemorySize;};
globalThis.__gramProbe=(label,locals)=>{const owners=locals?.owners;if(E.TRIAL&&(label===(E.TRIAL_AT??"p:owners"))&&owners){const steps=E.TRIAL.split(",");let prev=live();console.error("[DEBUG] TRIAL base MB "+mb(prev));for(const step of steps){if(step==="cache")owners.nativeCache.clear();else if(step==="closures")owners.nativeClosures.clear();else if(step==="shared")globalThis.__gramShared.delete(owners.nativeClosures);else if(step==="files")owners.files=[];else if(step==="obsclear")globalThis.__gramObs.release();else if(step==="results")locals.results.length=0;else if(step==="interner")globalThis.__gramInterner.clear();else if(step==="closureEntries")globalThis.__gramClosure.reset();else if(step==="scripts"){for(const [k,v] of Object.entries(locals.scripts)){if(v instanceof Map||v instanceof Set){const n=v.size;v.clear();console.error("[DEBUG] TRIAL scripts."+k+" entries "+n);}}}else if(step.startsWith("obs:"))globalThis.__gramObs.clear(step.slice(4));else if(step==="caches"){const rows=(globalThis.__gramCaches??[]).map(c=>({c,size:c.ref?.size??-1})).filter(r=>r.size>0).sort((a,b)=>b.size-a.size);console.error("[DEBUG] CACHES "+rows.length+" non-empty of "+(globalThis.__gramCaches??[]).length);const seenRef=new Set();let before=live();for(const {c,size} of rows.slice(0,Number(E.CACHE_TOP??"40"))){if(seenRef.has(c.ref))continue;seenRef.add(c.ref);c.ref.clear();const now=live();console.error("[DEBUG] CACHE "+c.file.slice(-60)+" "+c.name+" entries "+size+" released MB "+mb(before-now));before=now;}}else if(step==="stats"){const c=[...owners.nativeClosures.values()],sh=globalThis.__gramShared.get(owners.nativeClosures),vals=c.reduce((n,e)=>n+(Array.isArray(e.value)?e.value.length:0),0),obs=c.reduce((n,e)=>n+e.observations.length,0),uniq=new Set(c.flatMap(e=>e.observations));const kinds={};for(const r of uniq)kinds[r.kind]=(kinds[r.kind]??0)+1;console.error("[DEBUG] STATS closures "+c.length+" valueRefs "+vals+" observationRefs "+obs+" distinctRows "+uniq.size+" kinds "+JSON.stringify(kinds)+" shared "+(sh?.size??0)+" sharedMembers "+[...(sh?.values()??[])].reduce((n,e)=>n+e.value.size,0)+" sharedObs "+[...(sh?.values()??[])].reduce((n,e)=>n+e.observations.length,0)+" cache "+owners.nativeCache.size);}if(step==="exit"){globalThis.__gramExit=true;continue;}const now=live();console.error("[DEBUG] TRIAL "+step+" released MB "+mb(prev-now)+" now "+mb(now));prev=now;}}
if(E.PROBE_GC==="1")Bun.gc(true);const r=probe(label);console.error("[DEBUG] PROBE "+JSON.stringify(r));if((E.SNAP_AT??"").split(",").includes(label)){const s=Bun.generateHeapSnapshot("v8");writeFileSync(STORE+".snap-"+label.replace(":","-")+".json",typeof s==="string"?s:JSON.stringify(s));console.error("[DEBUG] snapshot "+label);}if(globalThis.__gramExit)finish();};

if(E.CACHES==="1")Bun.plugin({name:"gram-caches",setup(b){b.onLoad({filter:/🏗️build.*\.mjs$/},async args=>{
 const t=await Bun.file(args.path).text(),loader=args.path.endsWith(".ts")?"ts":"js",names=new Set();
 if(/\/(🧪️tests|🧬️schema)\//.test(args.path)||args.path.includes("/node_modules/"))return {contents:t,loader};
 for(const m of t.matchAll(/^(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^=\n]+)?=\s*new\s+(Map|Set)\b/gmu))names.add(m[1]);
 for(const m of t.matchAll(/^(?:export\s+)?(?:const|let|var)\s+[A-Za-z_$][\w$]*\s*=\s*new\s+(?:Map|Set)\([^;\n]*\)\s*,\s*([A-Za-z_$][\w$]*)\s*=\s*new\s+(Map|Set)\b/gmu))names.add(m[1]);
 if(!names.size)return {contents:t,loader};
 const file=args.path.split("🏗️build/")[1];
 const add="\n;(globalThis.__gramCaches??=[]);"+[...names].map(n=>"__gramCaches.push({file:"+JSON.stringify(file)+",name:"+JSON.stringify(n)+",get ref(){try{return "+n+"}catch{return undefined}}});").join("")+"\n";
 return {contents:t+add,loader};});}});
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const nodes=await import(L+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs");
{const mem=await import(L+"🧠️memory/🟨️.mjs"),si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");globalThis.__gramInterner=mem.graphStringInternerV1;globalThis.__gramClosure={reset:si.resetClosureEntriesV1};}
const admission=await import(L+"🕸️graph/🚪️admission/🟦️.ts");
const obs=await import(L+"👁️observation/🟨️.mjs");
const storage=await import(L+"📥️inputs/💾️storage/🟨️.mjs");
const revision=await import(L+"📥️inputs/🔏️revision/🟨️.mjs");
const {execFileSync}=await import("node:child_process");
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
const all=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")&&existsSync(join(ROOT,f)));
const keep=f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||f.endsWith("bun.lock")||f.endsWith(".patch");
let files=all.filter(keep);
let ownerRoots;if(E.ONLY_ROOTS){ownerRoots=new Set(E.ONLY_ROOTS.split(";").filter(Boolean));}else if(LIMIT>0){const skip=(E.EXCLUDE??"").split(";").filter(Boolean),cfg=files.filter(f=>(f.endsWith("📋️project.json")||f.endsWith("Cargo.toml"))&&!skip.some(x=>x.startsWith("=")?f===x.slice(1):f.includes(x))),step=Math.max(1,Math.floor(cfg.length/LIMIT));ownerRoots=new Set(cfg.filter((_,i)=>i%step===0).slice(0,LIMIT).map(f=>f.includes("/")?f.slice(0,f.lastIndexOf("/")):"."));}
console.error("[DEBUG] files "+files.length+" of "+all.length+" tracked; rss "+mb(process.memoryUsage().rss));
probe("loaded");
const options={analyzeLockfile:true,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,...(ownerRoots?{ownerRoots}:{}),...(E.CONC?{projectConcurrency:Number(E.CONC)}:{}),onOwnerObservations:(name,rows)=>{count++;if(E.PER_PROJECT==="1"&&count%Number(E.PER_EVERY??"1")===0){const before=process.memoryUsage().rss;if(E.PER_PROJECT_GC!=="0")Bun.gc(true);const h=heapStats(),now=Math.round(performance.now()-t0)/1000;console.error("[DEBUG] P "+JSON.stringify({n:count,t:now,rss:mb(before),live:mb(h.heapSize+h.extraMemorySize),rows:rows.length,name:name.slice(-50)}));}}};
const context={workspaceRoot:ROOT,registry,preparationControl:{advance:async()=>{}}};
let result;
if(E.PART==="1"){
 const graph=await import(L+"🕸️graph/🟦️.ts"),testing=await import(ROOT+"/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"),routing=await import(ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const members=graph.buildGraphMembersV1(await graph.workspaceBuildFilesV1(ROOT));
 console.error("[DEBUG] members "+members.length);probe("members");
 if(E.PREFETCH==="1"){
  const inputs=await import(L+"📥️inputs/🟨️.mjs"),si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),mem=await import(L+"🧠️memory/🟨️.mjs");
  const roots=[...new Set(members.filter(f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")).map(f=>f.includes("/")?f.slice(0,f.lastIndexOf("/")):"."))].filter(r=>r!=="."&&!r.startsWith("✏️s/🧑‍💻dev")||false).sort();
  const chunk=Number(E.CHUNK??"128"),count=Number(E.CHUNKS??"8");
  await testing.withBuildTestInvocationV1(async original=>{const control=routing.createIssuedResourceControlV1(original,"gram:prefetch",0);
   for(let k=0;k<count;k++){
    const set=new Set(roots.slice(k*chunk,(k+1)*chunk));const t=performance.now();
    await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>graph.prefetchBuildGraphProjectsV1(ROOT,registry,original,control,members,set)));
    const before=process.memoryUsage().rss;
    for(const dispose of globalThis.__gramDisposers?.splice(0)??[])dispose();
    obs.releaseInferenceMemoryV1();inputs.releaseInputMemoryV1();si.resetClosureEntriesV1();mem.graphStringInternerV1.clear();Bun.gc(true);Bun.gc(true);
    if(E.SNAP_CHUNK&&Number(E.SNAP_CHUNK)===k){const sn=Bun.generateHeapSnapshot("v8");writeFileSync(STORE+".chunk"+k+".json",typeof sn==="string"?sn:JSON.stringify(sn));console.error("[DEBUG] chunk snapshot "+k);finish();}if(E.SHRINK==="1"&&typeof Bun.shrink==="function"){const r0=process.memoryUsage().rss;Bun.shrink();console.error("[DEBUG] shrink rss "+mb(r0)+" -> "+mb(process.memoryUsage().rss));}const h=heapStats(),m=process.memoryUsage();console.error("[DEBUG] CHUNK "+JSON.stringify({k,seconds:Math.round((performance.now()-t)/100)/10,rssBefore:mb(before),rssAfter:mb(m.rss),liveAfterRelease:mb(h.heapSize+h.extraMemorySize),objs:h.objectCount}));
   }});
  finish();
 }
 const part=await testing.withBuildTestInvocationV1(async original=>{const control=routing.createIssuedResourceControlV1(original,"gram:part",0);
  return revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>graph.discoverBuildGraphPartV1(ROOT,registry,original,control,members,{cold:true,roots:new Set(),stubs:{},previousEnv:null})));});
 console.error("[DEBUG] part owners "+part.owners.length+" rows "+part.rows.length+" edges "+part.edges.length);probe("part");
 if(E.PART_AFTER_GC==="1"){Bun.gc(true);probe("partGC");}
 const load=await import(L+"🕸️graph/🕰️generations/🔌️load/🟨️.mjs");
 const shaped=load.shapeComputeResultV1({workspaceRoot:ROOT,part,stubs:{}});probe("shaped");console.error("[DEBUG] shaped owners "+shaped.owners.length);
 result=[];
 globalThis.__gramShaped=shaped;
}else
result=await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>obs.observeInferenceSnapshotV1(()=>nodes.emojiProjectJsonNodes(files,options,context))));
probe("returned");
if(E.SNAP){const s=Bun.generateHeapSnapshot("v8");writeFileSync(E.SNAP,typeof s==="string"?s:JSON.stringify(s));}
Bun.gc(true);probe("afterGC");
console.error("[DEBUG] result entries "+result.length);
finish();
