import {readFileSync,mkdtempSync,statSync} from "node:fs";
import * as fsNs from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {startSamplingProfiler,samplingProfilerStackTraces} from "bun:jsc";
const W="/Users/Shared/semio",B=process.env.BUILD_ROOT??W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const LIMIT=Number(process.env.LIMIT_S??900)*1000,STORE=process.env.STORE,READONLY=process.env.READONLY==="1",OVERLAY=process.env.OVERLAY;
if(!STORE)throw Error("STORE required");
const SI=process.env.SI_URL??B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs"),storage=await import(B+"📥️inputs/💾️storage/🟨️.mjs"),rev=await import(B+"📥️inputs/🔏️revision/🟨️.mjs");
const nodes=await import(B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"),si=await import(SI),od=await import(B+"🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs");
const pg=await import(B+"🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs"),pa=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const {catalogBuildContributionForManifestV1}=await import(W+"/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts");
const {createRowLogV1}=await import(B+"🕸️graph/📥️inference/../🕰️generations/🗃️row-log/🟨️.mjs");
const GEN=process.env.GEN??decodeURIComponent(new URL("./🗑️generated/",import.meta.url).pathname),staging=mkdtempSync(join(GEN,"rp-staging-"));
const reg=JSON.parse(readFileSync(W+"/📋️build-owners.json","utf8")),owner=reg.owners.find(o=>o.name===(process.env.OWNER??"@semio-tech/framework-os-dev"));
const ROWLOG=createRowLogV1({path:join(GEN,"rl-rp-"+process.pid+".rows")});const T={};const lap=(k,t)=>{T[k]=Math.round(performance.now()-t);console.log("stage",k,T[k],"ms rss",Math.round(process.memoryUsage().rss/1048576));};
if(process.env.PROFILE)startSamplingProfiler();const t0=performance.now();
const timer=setInterval(()=>{console.log("elapsed",Math.round((performance.now()-t0)/1000),"s rss",Math.round(process.memoryUsage().rss/1048576),JSON.stringify(si.closureEntryCountsV1()));if(performance.now()-t0>LIMIT)report();},30000);
function report(){const summary={T,counts:si.closureEntryCountsV1(),hc:Object.fromEntries(Object.entries(globalThis.__hc??{}).filter(([,v])=>typeof v==="number"))};console.log("RESULT",JSON.stringify(summary));
 if(!process.env.PROFILE)process.exit(0);const tr=samplingProfilerStackTraces();const self=new Map(),incl=new Map();let n=0;
 for(const t of tr.traces??[]){n++;const frames=t.frames??t.stackFrames??[];const seen=new Set();frames.forEach((f,i)=>{const k=(f.name||"?")+" "+(f.sourceURL??"").split("/").slice(-3).join("/")+":"+f.line;if(i===0)self.set(k,(self.get(k)??0)+1);if(!seen.has(k)){seen.add(k);incl.set(k,(incl.get(k)??0)+1);}});}
 const top=m=>[...m].sort((a,b)=>b[1]-a[1]).slice(0,40).map(([k,v])=>String(v).padStart(6)+" "+k).join("\n");
 console.log("samples",n,"\nSELF\n"+top(self)+"\nINCLUSIVE\n"+top(incl));process.exit(0);}
let port;
if(OVERLAY){const [file,add]=OVERLAY.split("|"),target=W+"/"+file,orig=readFileSync(target),extra=Buffer.from("\n"+add+"\n"),changed=Buffer.concat([orig,extra]);
 const st=fsNs.statSync(target);
 const patchStat=s=>s&&typeof s==="object"?new Proxy(s,{get:(o,k)=>k==="size"?(typeof o.size==="bigint"?o.size+BigInt(extra.length):o.size+extra.length):k==="mtimeMs"?o.mtimeMs+7777:k==="ctimeMs"?o.ctimeMs+7777:k==="ino"?o.ino:typeof o[k]==="function"?o[k].bind(o):o[k]}):s;
 const is=p=>typeof p==="string"&&p===target;
 port=new Proxy(fsNs,{get:(o,k)=>{
  if(k==="readFileSync")return (p,...a)=>is(p)?(a[0]&&(typeof a[0]==="string"||a[0].encoding)?changed.toString(a[0].encoding??a[0]):changed):o.readFileSync(p,...a);
  if(k==="statSync")return (p,...a)=>patchStat(o.statSync(p,...a));
  if(k==="lstatSync")return (p,...a)=>is(p)?patchStat(o.lstatSync(p,...a)):o.lstatSync(p,...a);
  return o[k];}});
 console.log("overlay on",target,"+",extra.length,"bytes");}
try{
 const contracts=nodes.projectGeneratorContractsV1(W);
 const clos=si.withClosurePassV1;
 const run=fn=>clos(()=>rev.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:staging,published(){}},()=>obs.observeInferenceSnapshotV1(fn))));
 const wrapped=fn=>port?obs.withObservedWorldV1(port,fn):fn();
 let t=performance.now();
 await wrapped(()=>run(async()=>{
  const providers=reg.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),catalog=await od.catalogBuildContributionsV1(W,providers);
  const files=await od.catalogNativeManifestCandidatesV1(W,reg,p=>catalogBuildContributionForManifestV1(catalog,p),{});
  lap("catalog",t);t=performance.now();
  const context={workspaceRoot:W,registry:reg,preparationControl:{advance:async()=>{}},lateClosures:true};
  const prep=await pg.playgroundPreparationTargets(files,W,owner.root,contracts,catalog,context);
  lap("playgroundPreparationTargets("+Object.keys(prep).length+")",t);t=performance.now();
  const json={...JSON.parse(readFileSync(join(W,owner.manifest),"utf8")),root:owner.root};json.targets={...json.targets,...prep};
  const env=nodes.createProjectInferenceEnvironmentV1({workspaceRoot:W,registry:reg,contracts,projectsByRoot:reg.owners.flatMap(o=>[[o.root,o.name]]),componentRoots:[],commandInputs:null,lateClosures:true,closureStore:STORE});
  si.bindClosureStoreV1(W,STORE,!READONLY);const cachePath=join(staging,"np-"+Date.now()+".json");
  const project=await obs.observeProjectInferenceV1({cachePath,key:["rp",owner.root,json],owner:owner.root,stamps:[],reusable:()=>true},()=>nodes.inferProjectBodyV1(env,{name:owner.name,root:owner.root,json,native:false,pipeline:false,selectedInputs:[]}));
  lap("inferProjectBody",t);t=performance.now();
  const resolved=await pa.resolveClosureRefsV1(project,W,env.scripts,"hash",ROWLOG);
  lap("resolveClosureRefs rows="+resolved.rows.length,t);await si.settleClosureStoresV1?.();
  return resolved;
 }));
 console.log("finished",JSON.stringify(T));
}catch(e){console.log("error",e?.stack??e);}
report();
