import{readFileSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{heapStats}from"bun:jsc";
const ROOT="/home/deck/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const out=process.argv[2]??"rss-profile.json",stop=process.argv[3]??"all",snapshotAt=process.env.RSS_SNAPSHOT;
const mb=(n:number)=>Math.round(n/1048576);
const status=()=>Object.fromEntries(readFileSync("/proc/self/status","utf8").split("\n").filter(l=>/^(VmRSS|VmHWM|RssAnon|RssFile|VmSwap):/.test(l)).map(l=>[l.split(":")[0],Math.round(parseInt(l.split(/\s+/)[1])/1024)]));
const rows:any[]=[];let peak=0,snapped=false;
const sampler=setInterval(()=>{const s=status();peak=Math.max(peak,s.VmRSS);if(process.env.RSS_SNAPSHOT_AT_MB&&!snapped&&mb(heapStats().heapSize)>=Number(process.env.RSS_SNAPSHOT_AT_MB)){snapped=true;writeFileSync(out+".mid.heap.json",JSON.stringify(Bun.generateHeapSnapshot()));console.error("SNAPSHOT_TAKEN "+JSON.stringify(status()));}
  if(process.env.RSS_TICK){const h=heapStats();const top=h.heapSize>500*1048576?Object.entries(h.objectTypeCounts).sort((x:any,y:any)=>y[1]-x[1]).slice(0,7).map(([k,v])=>k+"="+v).join(","):"";console.error("TICK "+JSON.stringify({rss:s.VmRSS,heap:mb(h.heapSize),extra:mb(h.extraMemorySize),objs:h.objectCount,top}));}},Number(process.env.RSS_TICK??250));
async function mark(phase:string,extra:any={}){
  const before=status();Bun.gc(true);await new Promise(r=>setTimeout(r,1500));Bun.gc(true);await new Promise(r=>setTimeout(r,1500));const h=heapStats(),m=process.memoryUsage(),after=status();
  const types=Object.entries(h.objectTypeCounts).sort((a:any,b:any)=>b[1]-a[1]).slice(0,8).map(([k,v])=>k+"="+v);
  const row={phase,peakSoFarMb:peak,rssBeforeGcMb:before.VmRSS,rssMb:after.VmRSS,hwmMb:after.VmHWM,anonMb:after.RssAnon,swapMb:after.VmSwap,heapMb:mb(h.heapSize),heapCapacityMb:mb(h.heapCapacity),extraMb:mb(h.extraMemorySize),objects:h.objectCount,jscExternalMb:mb(m.external),arrayBuffersMb:mb(m.arrayBuffers),top:types,...extra};
  rows.push(row);console.error(JSON.stringify(row));
  if(snapshotAt===phase){const s=Bun.generateHeapSnapshot("v8");writeFileSync(out+"."+phase+".heapsnapshot",typeof s==="string"?s:JSON.stringify(s));}
}
const t0=performance.now(),write=process.stderr.write.bind(process.stderr);(process.stderr as any).write=(chunk:any,...rest:any[])=>write(typeof chunk==="string"?chunk.split("\n").map((l:string,i:number,a:string[])=>l===""&&i===a.length-1?l:"[t="+Math.round((performance.now()-t0)/1000)+"] "+l).join("\n"):chunk,...rest);
await mark("start");
const{openPackedFactIndexV1}=await import(B+"🕸️graph/🕰️generations/🟨️.mjs");
const factFile=process.env.RSS_FACTS;
let index:any;
if(factFile){index=openPackedFactIndexV1(readFileSync(factFile),1<<30);let n=0;for(let i=0;i<index.rows.length;i+=1)if(index.rows[i]?.kind)n++;await mark("facts-index-walked",{rows:n});}
const{workspaceBuildFilesV1,buildGraphMembersV1}=await import(B+"🕸️graph/📁️workspace/🟦️.ts");
const{readBuildOwnerRegistryV1}=await import(B+"🕸️graph/🚪️admission/🟦️.ts");
const owner=await import(B+"🕸️graph/📥️inference/🟨️.mjs");
const tests=await import(B+"../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs");
const policies=await import(B+"🕸️graph/📥️inference/🛡️policy/🟨️.mjs");
await owner.libraryBootstrap;owner.prepareSourceAnalysesV1();
await mark("modules-loaded");
const root=ROOT,registry=readBuildOwnerRegistryV1(JSON.parse(owner.readBuildOwnerRegistrySourceV1(root)));
const files=buildGraphMembersV1(await workspaceBuildFilesV1(root));
const limit=Number(process.env.RSS_LIMIT??0);
await mark("files-walked",{files:files.length});
const sampleRoots=limit?new Set(files.filter((f:string)=>f.endsWith("📋️project.json")).sort().filter((_:any,i:number,all:any[])=>i%Math.ceil(all.length/limit)===Number(process.env.RSS_OFFSET??0)).map((f:string)=>f.slice(0,f.lastIndexOf("/")))):undefined;
const options:any={analyzeLockfile:true,...(process.env.RSS_CONC?{projectConcurrency:Number(process.env.RSS_CONC)}:{}),...(sampleRoots?{ownerRoots:sampleRoots}:{}),catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath}:any)=>({ownerRoot,sourcePath}))};
const{createRequire}=await import("node:module");
const manager=createRequire(B+"x.mjs")(B+"../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript");
const lock=manager.lockFileNames;
const primary=await owner.discoverBuildProjectsV1(files.filter((f:string)=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||lock.includes(f)||f.endsWith(".patch")),options,{workspaceRoot:root,registry});
await mark("inference-nodes",{contributions:primary.length});
if(process.env.RSS_SNAPSHOT_FILE)writeFileSync(process.env.RSS_SNAPSHOT_FILE,JSON.stringify(Bun.generateHeapSnapshot()));
if(stop==="nodes"){clearInterval(sampler);writeFileSync(out,JSON.stringify({peakMb:peak,rows},null,1));process.exit(0);}
const contributions=[...primary,...await tests.discoverTestBuildProjectsV1(files.filter((f:string)=>f.endsWith(".feature")),{},{workspaceRoot:root})];
const policyOwners=Object.assign({},...contributions.map(([,c]:any)=>c.projects??{}));
contributions.push(...await policies.default.createNodesV2[1](files.filter((f:string)=>f.endsWith("📜️script.ts")),{},{workspaceRoot:root,projects:policyOwners}));
const projects:Record<string,any>={},configs:Record<string,string>={};
for(const[file,c]of contributions as any[])for(const[name,p]of Object.entries(c.projects??{}) as any){projects[name]={...projects[name],...p,name,targets:{...projects[name]?.targets,...p.targets}};configs[name]??=file;}
await mark("contributions-merged",{projects:Object.keys(projects).length});
const{createProjectFileIndexV1}=await import(B+"🕸️graph/🧭️file-owner/🟨️.mjs").catch(()=>({}));
const projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(n=>[n,[]]));
if(createProjectFileIndexV1){const ownerOf=createProjectFileIndexV1(projects);for(const f of files){const o=ownerOf(f);if(o!==undefined)projectFileMap[o]?.push({file:f});}}
const context:any={workspaceRoot:root,projects,fileMap:{projectFileMap},selection:{kind:"all"},registry,completeSourceImports:false};
const testEdges:any=await tests.discoverTestBuildDependenciesV1({},context);
const edges=[...await owner.discoverBuildDependenciesV1(options,context),...testEdges];
await mark("inference-edges",{edges:edges.length});
const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(projects).map(([n,data])=>[n,{name:n,type:"lib",data}]))},options,context);
await mark("inference-metadata",{metadata:Object.keys(metadata).length});
const text=JSON.stringify({projects,edges,metadata});
await mark("plan-serialised",{planMb:mb(text.length)});
clearInterval(sampler);
writeFileSync(out,JSON.stringify({peakMb:peak,rows},null,1));
console.error("PEAK_RSS_MB",peak);
process.exit(0);
