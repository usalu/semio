// ⏱️ Reproduces the per-project inference loop of `repo show projects` (observation scopes, shared script cache, 8 workers) on a sample, in a scratch cache.
import {readFileSync,existsSync,mkdirSync,rmSync,writeFileSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";import {join,dirname} from "node:path";import {createHash} from "node:crypto";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),strip=(s:string)=>s.replace(/️/g,"");
const [nArg="50",strideArg="0",mode="observe",scratchArg,concArg="8",variant=process.env.VARIANT??"live"]=process.argv.slice(2);
const scratch=scratchArg??join(import.meta.dir,"..","🗑️generated","show-perf-bench","run");if(!process.env.KEEP)rmSync(scratch,{recursive:true,force:true});mkdirSync(join(scratch,"np"),{recursive:true});
const variantName=process.env.VARIANT??"live";
const dump=(cmd:string)=>JSON.parse(new TextDecoder().decode(Bun.spawnSync(["python3",join(import.meta.dir,"staged","apply.py"),cmd]).stdout)) as Record<string,[string,string][]>;
const opsByPath:Record<string,[string,string][]>=variantName==="gp"?dump("dump-ops"):variantName==="showperf"?dump("dump-ops-base"):{};
const applyOps=(text:string,ops:[string,string][])=>{let out=text;for(const [anchor,replacement] of ops){if(out.includes(replacement))continue;if(out.split(anchor).length!==2)throw Error("anchor drift: "+anchor.slice(0,60));out=out.replace(anchor,replacement);}return out;};
const scratchDir=(name:string)=>`(process.getBuiltinModule("fs").mkdirSync(${JSON.stringify(join(scratch,name))},{recursive:true}),${JSON.stringify(join(scratch,name))})`;
Bun.plugin({name:"graph-perf-patches",setup(b){b.onLoad({filter:/(?:inference\/[^/]*\/[^/]*|inputs\/[^/]*|observation\/[^/]*)\.mjs$/},async args=>{
 const key=Object.keys(opsByPath).find(k=>args.path.endsWith(k)),isInputs=args.path.endsWith("🏗️build/📥️inputs/🟨️.mjs");
 const original=await Bun.file(args.path).text();
 let t=key?applyOps(original,opsByPath[key]!):original;
 if(isInputs)t=t.replace('repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-lock-closures")',scratchDir("lock")).replace('repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"build-input-closures")',scratchDir("input"));
 return {contents:t,loader:"js"};});}});
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const observation:any=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const {cacheInternals,createScriptInputCache}=inference;
const tracked=new TextDecoder().decode(Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z"],{cwd:W}).stdout).split("\0").filter(f=>f&&strip(f.split("/").at(-1)!)===strip("📋️project.json")&&existsSync(join(W,f)));
const ig=new Set(["🧫️fixtures","🧫️examples","🧪️fixtures","🧪️examples"].map(strip));
const eligible=tracked.filter(f=>{const s=f.split("/").map(strip);return s[0]!==strip(".🧬semio")&&!s.slice(0,-1).some(x=>ig.has(x));}).sort();
const projectRoots=new Set(eligible.map(f=>dirname(f)));
const cargoOnly=new TextDecoder().decode(Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z"],{cwd:W}).stdout).split("\0").filter(f=>f.endsWith("/Cargo.toml")&&!projectRoots.has(dirname(f))&&!f.split("/").map(strip).some(x=>x===strip(".🧬semio")||ig.has(x)||x==="compose"||x==="temp")&&existsSync(join(W,f))).sort();
const n=Number(nArg),mix=process.env.MIX??"0.5",nCargo=Math.round(n*Number(mix)),nJson=n-nCargo;
const pick=(list:string[],k:number)=>{const step=Math.max(1,Math.floor(list.length/Math.max(k,1)));return list.filter((_,i)=>i%step===0).slice(0,k);};
const sample=[...pick(eligible,nJson),...pick(cargoOnly,nCargo)];const stride=0;
const counters={yields:0,writerStops:0};
const context={workspaceRoot:W,get invocation(){counters.yields++;return undefined;}};
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
const failures:string[]=[];const timings:{root:string;ms:number}[]=[];let cursor=0;
const inferOne=async({root,json}:any)=>{
 const started=performance.now();
 const run=async()=>{try{return await cacheInternals.projectWithDefaults(json,root,join(W,root),W,{},new Map(),commandInputs,scripts,undefined,"hash",undefined,[]);}catch(e:any){if(String(e?.message).includes("bounded graph writer custody")){counters.writerStops++;return {partial:true};}failures.push(root+": "+String(e?.message).slice(0,90));return {failed:true};}};
 if(mode==="plain")await run();
 else await observation.observeProjectInferenceV1({cachePath:join(scratch,"np",createHash("sha256").update(root).digest("hex").slice(0,24)+".json"),key:["bench",root,json],owner:root,reusable:()=>true},run);
 timings.push({root,ms:performance.now()-started,at:performance.now(),cpu:(process.cpuUsage().user+process.cpuUsage().system)/1e3});if(timings.length%10===0)snap(timings.length);
};
const series:any[]=[];
const sizes=()=>{const keyChars=(m:Map<string,any>)=>{let n=0;for(const k of m.keys())n+=k.length;return n;};const rows=(m:Map<string,any>)=>{let n=0;for(const v of m.values())n+=(v.observations?.length??0);return n;};const vals=(m:Map<string,any>)=>{let n=0;for(const v of m.values())n+=Array.isArray(v.value)?v.value.length:0;return n;};return {closures:scripts.closures.size,closureKeyChars:keyChars(scripts.closures),closureValueStrings:vals(scripts.closures),closureRowRefs:rows(scripts.closures),files:scripts.files.size,fileRowRefs:rows(scripts.files),depKeys:scripts.commandDependencies.size,depKeyChars:keyChars(scripts.commandDependencies)};};
import {heapStats} from "bun:jsc";
let firstTypes:any;
const snap=(done:number)=>{if(process.env.SERIES!=="1")return;if(process.env.TYPES==="1"){(globalThis as any).Bun.gc(true);const t=heapStats().objectTypeCounts as Record<string,number>;if(!firstTypes){firstTypes={...t};}else if(done>=Number(process.env.TYPESAT??"50")){const d=Object.entries(t).map(([k,v])=>[k,v-(firstTypes[k]??0)] as [string,number]).filter(([,v])=>v>0).sort((a,b)=>b[1]-a[1]).slice(0,12);console.log("TYPES delta vs first",done,JSON.stringify(d));firstTypes={...t};}}const g0=performance.now();(globalThis as any).Bun.gc(true);const gcMs=Math.round(performance.now()-g0);series.push({done,gcMs,heapMB:Math.round(process.memoryUsage().heapUsed/1e6),rssMB:Math.round(process.memoryUsage().rss/1e6),...sizes()});};
const cpu0=process.cpuUsage(),t1=performance.now();
const loop=async()=>{const workers=Array.from({length:Number(concArg)},async()=>{while(cursor<jobs.length)await inferOne(jobs[cursor++]);});await Promise.all(workers);return null;};
if(mode==="plain")await loop();
else await observation.observeInference({cachePath:join(scratch,"emoji.json"),key:["bench",process.env.NOTOP?Date.now():0]},()=>observation.settleObservedComputationV1(loop,{}));
const cpu=process.cpuUsage(cpu0),wall=performance.now()-t1;
if(process.env.WINDOWS){const w=Number(process.env.WINDOWS);const rows=[];for(let i=w;i<=timings.length;i+=w){const a=timings[i-w-1]??{at:t1,cpu:cpu0.user/1e3};const b=timings[i-1]!;rows.push({upTo:i,wallMsPerProject:Math.round((b.at-(i-w>0?timings[i-w-1]!.at:t1))/w),cpuMsPerProject:Math.round((b.cpu-(i-w>0?timings[i-w-1]!.cpu:0))/w)});}console.log("WINDOWS",JSON.stringify(rows));}
const ms=timings.map(t=>t.ms).sort((a,b)=>a-b);
if(mode==="observe"){const digests:Record<string,[string,string]>={};for(const job of jobs){const file=join(scratch,"np",createHash("sha256").update(job.root).digest("hex").slice(0,24)+".json");if(!existsSync(file))continue;const text=brotliDecompressSync(readFileSync(file)).toString("utf8"),nl=text.indexOf("\n"),header=JSON.parse(text.slice(0,nl));digests[job.root]=[createHash("sha256").update(text.slice(nl+1)).digest("hex"),createHash("sha256").update([...new Set(header.observations.map((r:any)=>r.kind+"\0"+r.path))].sort().join("\n")).digest("hex")];}writeFileSync(join(scratch,"digests.json"),JSON.stringify(digests));console.log("EQUIV",JSON.stringify({persisted:Object.keys(digests).length,results:createHash("sha256").update(JSON.stringify(Object.entries(digests).sort().map(([k,v])=>[k,v[0]]))).digest("hex").slice(0,16),rowKeys:createHash("sha256").update(JSON.stringify(Object.entries(digests).sort().map(([k,v])=>[k,v[1]]))).digest("hex").slice(0,16)}));}
if(failures.length)console.log("FAILURES",JSON.stringify(failures));console.log("COUNTS",JSON.stringify({dm:(globalThis as any).__dm,dmHit:(globalThis as any).__dmHit,tssCalls:(globalThis as any).__tss,tssInputs:(globalThis as any).__tssInputs}));console.log(JSON.stringify({variant,mode,projects:jobs.length,eligible:eligible.length,stride,commandInputsMs:Math.round(tCommand),wallSeconds:+(wall/1000).toFixed(1),cpuSeconds:+((cpu.user+cpu.system)/1e6).toFixed(1),perProjectMsMedian:Math.round(ms[ms.length>>1]!),perProjectMsP90:Math.round(ms[Math.floor(ms.length*.9)]!),yieldCalls:counters.yields,writerStops:counters.writerStops,rssMB:Math.round(process.memoryUsage().rss/1e6),stats:observation.sourceObservationStatsV1?.()}));
writeFileSync(join(scratch,"timings.json"),JSON.stringify(timings));if(series.length){snap(timings.length);console.log(JSON.stringify(series.map(r=>JSON.stringify(r))).replace(/\\"/g,'"'));}
if(!process.env.NOEXIT)process.exit(0);
