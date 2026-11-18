import{existsSync,readFileSync,realpathSync,mkdtempSync,rmSync}from"node:fs";import{createHash}from"node:crypto";import{tmpdir}from"node:os";import{join}from"node:path";import{deserialize}from"node:v8";import{brotliDecompressSync}from"node:zlib";import*as JSC from"bun:jsc";
const R="/Users/ueli/Documents/semio",B=(process.env.R107_CODE??R)+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const[dir,countArg,concArg,skipArg]=process.argv.slice(2),count=Number(countArg||5),conc=Number(concArg||1),skip=Number(skipArg||0),profile=process.env.R107_PROFILE==="1";
if(process.env.R107_PATCH){const{apply,all}=await import(import.meta.dir+"/patches.mjs");const names=process.env.R107_PATCH.split(",");const{readFileSync:rf}=await import("node:fs");(globalThis as any).Bun.plugin({name:"r107",setup(b:any){b.onLoad({filter:/🟨️\.mjs$/},(args:any)=>{const text=rf(args.path,"utf8"),out=apply(args.path,text,all.filter((_:any,i:number)=>names.includes(String(i))));return{contents:out,loader:"js"};});}});}
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const pn:any=await import(PN+"🟨️.mjs");const{projectStampPathsV1}=await import(B+"🕸️graph/📥️inference/🔖️stamps/🟨️.mjs") as any;
const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const{resolveClosureRefsV1}=await import(B+"🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs") as any;
const OB:any=await import(B+"👁️observation/🟨️.mjs");const{inferenceInventoryReferencesHoldV1}=await import(B+"👁️observation/📥️reuse/🟨️.mjs") as any;
const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;
const head=JSON.parse(readFileSync(dir+"/head.json","utf8")),units=JSON.parse(readFileSync(dir+"/units.json","utf8")),session=deserialize(readFileSync(dir+"/session.bin"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
const env=pn.createProjectInferenceEnvironmentV1({...session,workspaceRoot:R,closureStore:store});
const recordOfKey=(key:string)=>{try{const text=brotliDecompressSync(readFileSync(join(dataDir,"np",key+".json"))).toString("utf8");return JSON.parse(text.slice(text.indexOf("\n")+1));}catch{return undefined;}};
const stored=(r:any)=>existsSync(store+"/e/"+createHash("sha256").update(R+"\0"+SI.closureRecipeKeyV1(r)).digest("hex")+".json.br");
const all=process.env.R107_ALL==="1",jobs:any[]=[];let skipped=0;
for(const u of units){const v=recordOfKey(ownerRecordKeyV1(u.json.root));if(!v)continue;const rec=Object.values(v.closureRecipes??{}) as any[];if(!all&&(!rec.length||!rec.every(stored)))continue;if(skipped<skip){skipped++;continue;}jobs.push(pn.ownerJobV1(env,head.globals??head,u));if(jobs.length>=count)break;}
const takeAway=Object.assign(()=>{throw Object.assign(new Error("miss"),{code:"BUILD_TAKE_MISS"});},{take:true});
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};
const per:any[]=[],outcome:Record<string,number>={};
if(profile)(JSC as any).startSamplingProfiler();
const clock0=OB.observationLookupClockV1();
const steps:Record<string,number>={read:0,settle:0,refs:0},cpus:Record<string,number>={read:0,settle:0,refs:0};
const timed=async(k:string,f:()=>any)=>{const t=performance.now(),c=cpu();try{return await f();}finally{steps[k]+=performance.now()-t;cpus[k]+=cpu()-c;}};
const look=async(job:any)=>{
 const t=performance.now(),c0=cpu();let ok="";
 try{
  await SI.runClosureTakeV1(async()=>{
   const value=await timed("read",()=>recordOfKey(job.cacheKey));
   if(!value)throw Object.assign(new Error("none"),{code:"NONE"});
   const paths=Object.keys(value.closureRecipes??{}).map(k=>({kind:"closure",path:JSON.stringify([R,k,["digest"]])}));
   await timed("settle",()=>OB.settleClosureRowsV1(paths,value));
   await timed("refs",()=>resolveClosureRefsV1(value,R,env.scripts,"hash",undefined));
  });ok="present";
 }catch(e:any){ok="err:"+String(e?.code??e?.message).slice(0,40);}
 outcome[ok]=(outcome[ok]||0)+1;
 SI.releaseAfterTakeLookV1(env.scripts,{shrink:false});
 per.push({ok,cpuMs:Math.round(cpu()-c0),wallMs:Math.round(performance.now()-t)});
};
const t0=performance.now(),c00=cpu();
await SI.withClosurePassV1(()=>OB.observeInferenceSnapshotV1(async()=>{let next=0;await Promise.all(Array.from({length:conc},async()=>{while(next<jobs.length){await look(jobs[next++]);}}));}));
const sec=(performance.now()-t0)/1000,cpuTotal=cpu()-c00,clock1=OB.observationLookupClockV1();
const lookup=Object.fromEntries(Object.keys(clock1).map(k=>[k,clock1[k]-clock0[k]]));
console.log("PER",JSON.stringify(per));console.log("STEPS",JSON.stringify({wallMs:Object.fromEntries(Object.entries(steps).map(([k,v])=>[k,Math.round(v)])),cpuMs:Object.fromEntries(Object.entries(cpus).map(([k,v])=>[k,Math.round(v)]))}));
console.log("RESULT",JSON.stringify({owners:jobs.length,conc,seconds:Math.round(sec*100)/100,cpuMsPerOwner:Math.round(cpuTotal/Math.max(1,jobs.length)),ownersPerSecond:Math.round(jobs.length/sec*100)/100,outcome,layers:SI.layerVerdictCountsV1?.(),closure:SI.closureEntryCountsV1?.()}));
if(profile){const tr=(JSC as any).samplingProfilerStackTraces();const self=new Map<string,number>(),incl=new Map<string,number>();let n=0;for(const t of tr.traces){n++;const fr=t.frames??[];const names=fr.map((f:any)=>(f.name||"?")+"@"+String(f.sourceURL||"").split("/").slice(-2).join("/")+":"+f.lineNumber);if(names[0])self.set(names[0],(self.get(names[0])||0)+1);for(const x of new Set(names))incl.set(x,(incl.get(x)||0)+1);}
 const top=(m:Map<string,number>)=>[...m].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,v])=>v+" "+k);console.log("SAMPLES",n);console.log("SELF\n"+top(self).join("\n"));console.log("INCL\n"+top(incl).join("\n"));}
process.exit(0);
