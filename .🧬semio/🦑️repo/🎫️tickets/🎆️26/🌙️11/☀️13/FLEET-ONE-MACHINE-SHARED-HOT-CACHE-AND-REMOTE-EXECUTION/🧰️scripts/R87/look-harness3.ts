import{existsSync}from"node:fs";import{createHash}from"node:crypto";import{mkdtempSync,readFileSync,realpathSync,rmSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";import{deserialize}from"node:v8";import{heapStats}from"bun:jsc";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const dir=process.argv[2],count=Number(process.argv[3]),conc=Number(process.argv[4]||12),skip=Number(process.argv[5]||0),every=Number(process.argv[6]||10);
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn=await import(PN+"🟨️.mjs");const{ownerJobV1,projectRecordKeyV1,projectRecordReusableV1}=pn as any;const{projectStampPathsV1}=await import(B+"🕸️graph/📥️inference/🔖️stamps/🟨️.mjs") as any;
const SI=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const{resolveClosureRefsV1}=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs");
const OB=await import(B+"👁️observation/🟨️.mjs");
const{withBuildGraphStagingV1}=await import(B+"📥️inputs/💾️storage/🟨️.mjs");
const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),units=JSON.parse(readFileSync(dir+"/units.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const base=mkdtempSync(join(realpathSync(tmpdir()),"semio-r80-"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
const{brotliDecompressSync}=await import("node:zlib");const{join:j}=await import("node:path");
const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;
const recordOfKey=(key:string)=>{try{const text=brotliDecompressSync(readFileSync(j(dataDir,"np",key+".json"))).toString("utf8");return JSON.parse(text.slice(text.indexOf("\n")+1));}catch{return undefined;}};
const recordOf=(job:any)=>recordOfKey(job.cacheKey);
const stored=(r:any)=>existsSync(store+"/e/"+createHash("sha256").update(R+"\0"+(SI as any).closureRecipeKeyV1(r)).digest("hex")+".json.br");
const jobs:any[]=[];let skipped=0;
for(const u of units){const v=recordOfKey(ownerRecordKeyV1(u.json.root));if(!v)continue;const rec=Object.values(v.closureRecipes??{}) as any[];if(!rec.length||!rec.every(stored))continue;if(skipped<skip){skipped++;continue;}jobs.push(ownerJobV1(env,head.globals??head,u));if(jobs.length>=count)break;}
console.log("eligible jobs",jobs.length);if(process.env.OKIDX){const keep=new Set(JSON.parse(readFileSync(process.env.OKIDX,"utf8")));const sel=jobs.filter((_:any,i:number)=>keep.has(i));jobs.length=0;jobs.push(...sel);console.log("selected",jobs.length);}
const mb=(n:number)=>Math.round(n/1048576*10)/10;
const sizes=()=>{const o:any={};const c=(OB as any).inferenceCacheSizesV1?.();Object.assign(o,{...c,rowStoreRows:c?.rowStore?.rows,rowStoreChars:c?.rowStore?.characters,rowStore:undefined});const s=(OB as any).sourceObservationStatsV1?.();Object.assign(o,s);const sc:any=env.scripts;for(const k of Object.keys(sc)){const v=sc[k];if(v instanceof Map||v instanceof Set)o["scripts."+k]=v.size;else if(Array.isArray(v))o["scripts."+k]=v.length;}try{Object.assign(o,{layerCounts:JSON.stringify((SI as any).layerVerdictCountsV1()),closure:JSON.stringify((SI as any).closureEntryCountsV1())});}catch{}return o;};
const outcome:Record<string,number>={};let done=0;const t0=performance.now();const rows:any[]=[];
const sample=()=>{Bun.gc(true);const h=heapStats(),m=process.memoryUsage();const row={owners:done,sec:Math.round((performance.now()-t0)/100)/10,heapUsedMB:mb(m.heapUsed),externalMB:mb(m.external),rssMB:mb(m.rss),jscHeapMB:mb(h.heapSize),objs:h.objectCount,...sizes()};rows.push(row);console.log(JSON.stringify(row));};
const stg=join(base,"stg");
const takeAway=Object.assign(()=>{throw Object.assign(Error("miss"),{code:"BUILD_TAKE_MISS"});},{take:true});
const{observeProjectInferenceV1}=OB as any;
const PNm:any=pn;
const cpuNow=()=>{const u=process.cpuUsage();return (u.user+u.system)/1000;};
const per:any[]=[];
import*as JSC from"bun:jsc";
const{addSampleTracesV1,createSampleBookV1,summarizeSampleBookV1}=await import("./sc/🧠️memory/🔬️sampler/🟨️.mjs").catch(()=>({} as any)) as any;
let profiling=false;const book=createSampleBookV1();
const look=async(job:any)=>{
 if(!profiling&&per.length>=Number(process.env.WARM||40)){profiling=true;(JSC as any).startSamplingProfiler();}
 const t=performance.now(),c0=cpuNow();let c1=0,c2=0,ok=false,err="";
 try{
  const value=recordOf(job);if(!value){outcome.miss=(outcome.miss||0)+1;return;}
  const paths=Object.keys(value.closureRecipes??{}).map(k=>({kind:"closure",path:JSON.stringify([R,k,["digest"]])}));
  await (OB as any).settleClosureRowsV1(paths,value);c1=cpuNow();
  const resolved=await resolveClosureRefsV1(value,R,env.scripts,"hash",undefined);c2=cpuNow();ok=true;outcome.refs=(outcome.refs||0)+1;
 }catch(e:any){err=String(e?.message).slice(0,50);const k="err:"+err;outcome[k]=(outcome[k]||0)+1;}
 finally{SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});per.push({ok,err,recipes:Object.keys(recordOf(job)?.closureRecipes??{}).length,settleCpu:c1?c1-c0:null,refsCpu:c2?c2-c1:null,totalCpu:cpuNow()-c0,wall:performance.now()-t});}
};
const clk={settle:0,refs:0,total:0};
sample();
await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(async()=>{
 let next=0;await Promise.all(Array.from({length:conc},async()=>{while(next<jobs.length){const job=jobs[next++];await look(job);done++;if(done%every===0)sample();}}));
}));
const sec=(performance.now()-t0)/1000;
sample();
if(profiling){const tr=(JSC as any).samplingProfilerStackTraces();addSampleTracesV1(book,tr.traces,{...(await import("./sc/🧠️memory/🔬️sampler/🟨️.mjs") as any).SLICE_SAMPLER_POLICY_V1,top:40,maximumFrames:200});console.log("PROFILE",JSON.stringify(summarizeSampleBookV1(book,{top:40})));}console.log("PER",JSON.stringify(per.map(r=>({...r,wall:Math.round(r.wall),settleCpu:r.settleCpu&&Math.round(r.settleCpu),refsCpu:r.refsCpu&&Math.round(r.refsCpu),totalCpu:Math.round(r.totalCpu)}))));console.log("CLK",JSON.stringify(clk));console.log("CLOCK",JSON.stringify((OB as any).observationLookupClockV1()),JSON.stringify((SI as any).layerVerdictCountsV1?.()));console.log("RESULT",JSON.stringify({owners:jobs.length,conc,seconds:Math.round(sec*10)/10,ownersPerSecond:Math.round(jobs.length/sec*100)/100,outcome,heapGrowthMBPerOwner:Math.round((rows.at(-1).heapUsedMB-rows[0].heapUsedMB)/jobs.length*100)/100,peakRssMB:Math.max(...rows.map(r=>r.rssMB))}));
rmSync(base,{recursive:true,force:true});process.exit(0);
