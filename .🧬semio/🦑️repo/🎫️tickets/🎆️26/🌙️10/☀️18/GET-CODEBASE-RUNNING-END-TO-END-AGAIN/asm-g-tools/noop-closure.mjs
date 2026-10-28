// [asm-g] graph-free profile of the NO-OP assembly of a selection from a scratch copy of the sealed generation: closure, verification (real stat/digest holds on the live tree), fragments streamed, phase wall + RSS.
// usage: bun noop-closure.mjs <gen-dir> <project> <target>   env: POLICY=fresh|current  RSS_CAP_MB=1500  NOWAIT_STAMP=1
import{readFileSync}from"node:fs";import{join}from"node:path";import{lstat,readFile}from"node:fs/promises";
const ROOT="/Users/ueli/Documents/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",D=process.argv[2],PROJECT=process.argv[3],TARGET=process.argv[4]??"test",E=process.env;
process.chdir(ROOT);
const mb=n=>Math.round(n/1048576),t0=performance.now();let peak=0;const marks=[];
const sample=()=>{const r=process.memoryUsage().rss;if(r>peak)peak=r;if(r>Number(E.RSS_CAP_MB??"2500")*1048576){console.error("[DEBUG] ABORT rss "+mb(r));process.exit(3);}};
const timer=setInterval(sample,100);
import{heapStats}from"bun:jsc";
const mark=(name,extra={})=>{if(E.LIVE==="1"){Bun.gc(true);const h=heapStats();extra={...extra,liveMB:mb(h.heapSize+h.extraMemorySize)};}sample();const row={name,t:Math.round(performance.now()-t0),rss:mb(process.memoryUsage().rss),peak:mb(peak),...extra};marks.push(row);console.error("[DEBUG] "+JSON.stringify(row));};
const gen=await import(B+"🕸️graph/🕰️generations/🟨️.mjs"),load=await import(B+"🕸️graph/🕰️generations/🔌️load/🟨️.mjs"),envm=await import(B+"🕸️graph/🕰️generations/🌍️environment/🟨️.mjs");
const revision=await import(B+"📥️inputs/🔏️revision/🟨️.mjs"),obs=await import(B+"👁️observation/🟨️.mjs"),admission=await import(B+"🕸️graph/🚪️admission/🟦️.ts");
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
mark("modules");
const read=async p=>{try{return await readFile(join(ROOT,p),"utf8");}catch(e){if(["ENOENT","ENOTDIR"].includes(e.code))return undefined;throw e;}};
const statFile=async p=>{try{const s=await lstat(join(ROOT,p));return s.isFile()?{size:s.size,mtimeMs:s.mtimeMs}:undefined;}catch(e){if(["ENOENT","ENOTDIR"].includes(e.code))return undefined;throw e;}};
const statEntry=async p=>{try{const s=await lstat(join(ROOT,p));return{directory:s.isDirectory(),file:s.isFile(),mtimeMs:s.mtimeMs,ino:s.ino,mode:s.mode};}catch(e){if(["ENOENT","ENOTDIR"].includes(e.code))return undefined;throw e;}};
const staleSeen=new Map(),state={stable:new Set()};let holdCalls=0,holdRows=0,holdMs=0;
const holds=async(rows,signal)=>{const t=performance.now();holdCalls++;holdRows+=rows.length;try{if(E.HOLD_ALL==="1")return true;
 const plain=[],own=[];for(const row of rows){if(row.kind==="closure")continue;(row.kind==="field"||row.kind==="digest"?own:plain).push(row);}
 for(let at=0;at<own.length;at+=64)if(!(await Promise.all(own.slice(at,at+64).map(r=>envm.environmentRowHoldsV1(r,{read,stat:statFile})))).every(Boolean))return false;
 if(!plain.length)return true;return obs.observationsHold(plain,64,state.stable,true,signal,row=>{if(staleSeen.size<400)staleSeen.set(row.kind+' '+row.path,row);});}finally{holdMs+=performance.now()-t;}};
let membMs=0,membMode="";
const membership=async(previous,signal)=>{const t=performance.now();try{
 if(previous&&await gen.frontierHoldsV1(previous.frontier,statEntry,{signal})){membMode="stat-hold";return{digest:previous.manifest.membership,frontier:previous.frontier,changes:[]};}
 membMode="walk";throw Error("[asm-g] frontier moved: the harness does not walk (tree changed since the generation); pick STALE_OK=1 to treat as unchanged");}finally{membMs+=performance.now()-t;}};
const engine=gen.createGenerationStoreV1({root:D,fs:gen.createNodeFilePortV1(),holds,derive:undefined,resources:{facts:"asm-g"}});
const current=await engine.read();mark("pointer+manifest",{generation:current.pointer.generation,owners:Object.keys(current.manifest.owners).length});
if(E.EAGER==="1")await engine.readIndex(current.manifest.factIndex);
const roots=[PROJECT],selection={kind:"tasks",tasks:[{project:PROJECT,target:TARGET}]};
const closure=async cur=>{const global=await engine.readGlobal(cur.manifest);return load.sliceClosureV1({owners:Object.fromEntries(Object.entries(cur.manifest.owners).map(([n,r])=>[n,r.root])),external:Object.keys(global.externalNodes??{}),fragment:n=>engine.readFragment(cur.manifest.owners[n]).then(f=>E.SLIM==="0"?f:load.sliceViewV1(f)),selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))});};
let names=await closure(current);mark("closure(slice)",{projects:names.size,fragmentReads:engine.stats.fragmentReads});
const membershipSafe=E.STALE_OK==="1"?async prev=>({digest:prev.manifest.membership,frontier:prev.frontier,changes:[]}):membership;
const result=await revision.withImplementationPinV1(()=>engine.refreshClosure({roots,closure,identity:current.manifest.identity,implementation:current.manifest.implementation,policy:E.POLICY??"fresh",membership:membershipSafe,compute:async()=>{throw Error("[asm-g] would compute")},builder:async(_b,pending)=>({built:false,published:false,needsBuild:true,computed:0,pending}),progress:undefined,requestedAt:E.NOWAIT_STAMP==="1"?undefined:Date.now()}));
if(result.pending)console.error("[DEBUG] pending dirty "+result.pending.dirty.length+" "+result.pending.dirty.slice(0,12).join(" "));mark("refreshClosure",{served:result.served,needsBuild:!!result.needsBuild,checked:result.plan?.checked,stale:result.plan?.stale,dirty:result.plan?.dirty?.size,holdCalls,holdRows,holdMs:Math.round(holdMs),membMs:Math.round(membMs),membMode,indexLoads:engine.stats.indexLoads});
engine.dropIndexes();Bun.gc(true);mark("afterGC");
if(!result.needsBuild){
 const stream=engine.assembleStream(result.current,{names:result.names&&[...result.names],compare:(a,b)=>a.localeCompare(b)});let n=0,bytes=0;
 for await(const[,node]of stream.nodes()){n++;bytes+=JSON.stringify(node).length;}
 const fin=await stream.finish();mark("assembleStream",{projects:n,nodeJsonMB:mb(bytes),deps:fin.dependencies.length,external:Object.keys(fin.externalNodes).length,fragmentReads:engine.stats.fragmentReads});
}
{const kinds={};for(const row of staleSeen.values())kinds[row.kind]=(kinds[row.kind]??0)+1;console.error("[DEBUG] stale rows seen "+staleSeen.size+" "+JSON.stringify(kinds));for(const k of [...staleSeen.keys()].slice(0,25))console.error("[DEBUG]   "+k.slice(0,200));}
clearInterval(timer);const u=process.cpuUsage();console.log(JSON.stringify({cpuMs:Math.round((u.user+u.system)/1000),wallMs:Math.round(performance.now()-t0),peakRssMB:mb(peak),marks}));
