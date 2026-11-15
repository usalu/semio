import {readFileSync,mkdtempSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {startSamplingProfiler,samplingProfilerStackTraces} from "bun:jsc";
const W="/Users/Shared/semio",B=W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const LIMIT=Number(process.env.LIMIT_S??240)*1000;
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs"),storage=await import(B+"📥️inputs/💾️storage/🟨️.mjs"),rev=await import(B+"📥️inputs/🔏️revision/🟨️.mjs");
const nodes=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"),si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),od=await import(B+"🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs");
const pg=await import(B+"🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs"),pa=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const {catalogBuildContributionForManifestV1}=await import(W+"/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts");
const {createRowLogV1}=await import(B+"🕸️graph/📥️inference/../🕰️generations/🗃️row-log/🟨️.mjs");const clos=si.withClosurePassV1;const staging=mkdtempSync(join(tmpdir(),"prof-staging-")),store=process.env.STORE??mkdtempSync(join(tmpdir(),"prof-ce-"));
const reg=JSON.parse(readFileSync(W+"/📋️build-owners.json","utf8")),owner=reg.owners.find(o=>o.name==="@semio-tech/framework-os-dev");
const ROWLOG=createRowLogV1({path:join(tmpdir(),"rl-prof-"+process.pid+".rows")});const T={};const lap=(k,t)=>{T[k]=Math.round(performance.now()-t);console.log("[DEBUG] stage",k,T[k],"ms rss",Math.round(process.memoryUsage().rss/1048576));};
if(process.env.PROFILE)startSamplingProfiler();const t0=performance.now();
const timer=setInterval(()=>{console.log("[DEBUG] elapsed",Math.round((performance.now()-t0)/1000),"s rss",Math.round(process.memoryUsage().rss/1048576));if(performance.now()-t0>LIMIT)report();},30000);
function report(){if(!process.env.PROFILE){console.log("[DEBUG] done",JSON.stringify(T));process.exit(0);}const tr=samplingProfilerStackTraces();const self=new Map(),incl=new Map();let n=0;
 for(const t of tr.traces??[]){n++;const frames=t.frames??t.stackFrames??[];const seen=new Set();frames.forEach((f,i)=>{const k=(f.name||"?")+" "+(f.sourceURL??"").split("/").slice(-3).join("/")+":"+f.line;if(i===0)self.set(k,(self.get(k)??0)+1);if(!seen.has(k)){seen.add(k);incl.set(k,(incl.get(k)??0)+1);}});}
 const top=m=>[...m].sort((a,b)=>b[1]-a[1]).slice(0,45).map(([k,v])=>String(v).padStart(6)+" "+k).join("\n");
 console.log("[DEBUG] samples",n,JSON.stringify(T),"\nSELF\n"+top(self)+"\nINCLUSIVE\n"+top(incl));process.exit(0);}
try{
 const contracts=nodes.projectGeneratorContractsV1(W);
 const run=fn=>clos(()=>rev.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published(){}},()=>obs.observeInferenceSnapshotV1(fn))));
 let t=performance.now();
 const out=await run(async()=>{
  const providers=reg.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),catalog=await od.catalogBuildContributionsV1(W,providers);
  const files=await od.catalogNativeManifestCandidatesV1(W,reg,p=>catalogBuildContributionForManifestV1(catalog,p),{});
  lap("catalog",t);t=performance.now();
  const context={workspaceRoot:W,registry:reg,preparationControl:{advance:async()=>{}},lateClosures:true};
  const prep=await pg.playgroundPreparationTargets(files,W,owner.root,contracts,catalog,context);
  lap("playgroundPreparationTargets("+Object.keys(prep).length+")",t);t=performance.now();
  const json={...JSON.parse(readFileSync(join(W,owner.manifest),"utf8")),root:owner.root};json.targets={...json.targets,...prep};
  const env=nodes.createProjectInferenceEnvironmentV1({workspaceRoot:W,registry:reg,contracts,projectsByRoot:reg.owners.flatMap(o=>[[o.root,o.name]]),componentRoots:[],commandInputs:null,lateClosures:true,closureStore:store});
  si.bindClosureStoreV1(W,store,true);const cachePath=join(staging,"np-"+Date.now()+".json");const project=await obs.observeProjectInferenceV1({cachePath,key:["prof",owner.root,json],owner:owner.root,stamps:[],reusable:()=>true},()=>nodes.inferProjectBodyV1(env,{name:owner.name,root:owner.root,json,native:false,pipeline:false,selectedInputs:[]}));console.log("[DEBUG] rows of record",(obs.projectInferenceObservationsV1?.(cachePath)??[]).length);
  lap("inferProjectBody",t);t=performance.now();
  const resolved=await pa.resolveClosureRefsV1(project,W,env.scripts,"hash",ROWLOG);
  lap("resolveClosureRefs rows="+resolved.rows.length,t);await si.settleClosureStoresV1?.();console.log("[DEBUG] counts",JSON.stringify(si.closureEntryCountsV1()),JSON.stringify(globalThis.__hc??{}).slice(0,600));
  return resolved;
 });
 console.log("[DEBUG] finished",JSON.stringify(T));
}catch(e){console.log("[DEBUG] error",e?.stack??e);}
report();
