// [mem2] Harness parent (copied to <mirror>/fiharness/run.ts; not a library file): farmid's production parent loop emojiProjectJsonNodes over ONLY_ROOTS inside the captured snapshot, plus phase marks (globalThis.__m2mark, defined by the instrumented loader) with RSS, true peak RSS (ru_maxrss), JSC heap and macOS physical footprint. env as farmid's run.ts plus M2_GC=1 (collect before every mark: live heap per phase), M2_STOP=<mark> (exit right after that mark).
import{readFileSync,mkdirSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{join}from"node:path";
import{execFileSync}from"node:child_process";
import{createHash}from"node:crypto";
import{heapStats}from"bun:jsc";
import*as inference from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import*as nodes from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs";
import*as admission from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import*as obs from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
import*as storage from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import*as scriptInputs from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import*as inputs from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs";
import*as memory from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs";
import*as revision from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
type Mark={label:string;t:number;rss:number;maxRss:number;heap:number;extra:number;objects:number;fp?:number;fpPeak?:number;top?:string};
export async function runHarnessV1():Promise<void>{
 const E=process.env,ROOT=E.GRAM_ROOT!,STORE=E.STORE!,t0=performance.now(),mb=(n:number)=>Math.round(n/1048576),G=globalThis as unknown as {__m2mark:(label:string)=>void;__m2:Mark[];__m2images?:{[Symbol.iterator]():Iterator<[string,Buffer]>}};
 G.__m2mark("H-start");
 rmSync(STORE,{recursive:true,force:true});mkdirSync(STORE,{recursive:true});
 await (inference as {libraryBootstrap:Promise<unknown>}).libraryBootstrap;
 G.__m2mark("H-bootstrap");
 const peaks=new Map<number,{rssMB:number;command:string;firstS:number;lastS:number}>();let treePeakMB=0;const series:{t:number;self:number;heap:number;tree:number;workers:number}[]=[];let heapPeakMB=0;
 const sample=()=>{const now=Math.round((performance.now()-t0)/100)/10,rows=execFileSync("ps",["-A","-o","pid=,ppid=,rss=,command="],{maxBuffer:1<<26}).toString().split("\n").map(line=>/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.*)$/.exec(line)).filter(Boolean).map(m=>({pid:Number(m![1]),ppid:Number(m![2]),rss:Number(m![3])*1024,command:m![4]!}));
  const kids=new Map<number,number[]>();for(const row of rows){const list=kids.get(row.ppid)??[];list.push(row.pid);kids.set(row.ppid,list);}const tree=new Set([process.pid]),stack=[process.pid];while(stack.length)for(const child of kids.get(stack.pop()!)??[])if(!tree.has(child)){tree.add(child);stack.push(child);}
  let total=0,workers=0,self=0;for(const row of rows)if(tree.has(row.pid)&&row.command.includes("bun")){total+=row.rss;if(row.pid===process.pid)self=row.rss;else workers++;const seen=peaks.get(row.pid);peaks.set(row.pid,{rssMB:Math.max(seen?.rssMB??0,mb(row.rss)),command:row.pid===process.pid?"parent":row.command.includes("graph-farm-worker")?"worker":row.command.slice(-60),firstS:seen?.firstS??now,lastS:now});}
  const h=heapStats(),heap=mb(h.heapSize+h.extraMemorySize);heapPeakMB=Math.max(heapPeakMB,heap);treePeakMB=Math.max(treePeakMB,mb(total));series.push({t:now,self:mb(self),heap,tree:mb(total),workers});};
 const timer=setInterval(sample,Number(E.SAMPLE_MS??"1000"));
 const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
 const all=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:E.LIST_ROOT??ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")&&existsSync(join(ROOT,f)));
 const files=all.filter(f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||f.endsWith("bun.lock")||f.endsWith(".patch")),ownerRoots=new Set(E.ONLY_ROOTS!.split(";").filter(Boolean)),digests:Record<string,string>={};
 G.__m2mark("H-inputs");
 const options={analyzeLockfile:true,catalogProviders:registry.catalogProviders.filter(({ownerRoot}:{ownerRoot:string})=>existsSync(join(ROOT,ownerRoot,"📋️project.json"))).map(({ownerRoot,sourcePath}:{ownerRoot:string;sourcePath:string})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots,onOwnerObservations:(name:string,rows:{kind:string;path:string;fact:{provider?:string}}[])=>{const normal=rows.map(r=>[r.kind,r.path,r.kind==="analysis"?{...r.fact,provider:String(r.fact.provider).replace(/:[0-9a-f]{64}$/,":rev"),revision:"rev"}:r.fact]);digests[name]=createHash("sha256").update(JSON.stringify(normal)).digest("hex")+":"+rows.length;if(E.ROWS_DUMP){mkdirSync(E.ROWS_DUMP,{recursive:true});writeFileSync(join(E.ROWS_DUMP,name.replace(/[^\w.@-]/g,"_")+".jsonl"),normal.map(row=>JSON.stringify(row)).join("\n")+"\n");}}};
 const context={workspaceRoot:ROOT,registry,preparationControl:{advance:async()=>{}}};
 sample();
 const result=await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>obs.observeInferenceSnapshotV1(()=>(nodes as {emojiProjectJsonNodes:Function}).emojiProjectJsonNodes(files,options,context))));
 clearInterval(timer);sample();G.__m2mark("H-end");Bun.gc(true);const end=heapStats(),liveEndMB=mb(end.heapSize+end.extraMemorySize);
 const attribution:Record<string,number>={liveEndMB:liveEndMB};
 if(E.M2_ATTR==="1"){const si=scriptInputs as unknown as {resetClosureEntriesV1:()=>void;bindClosureScriptsV1:(root:string,scripts:unknown)=>void};const live=()=>{Bun.gc(true);const h=heapStats();return mb(h.heapSize+h.extraMemorySize);};attribution.entries=(result as unknown[]).length;si.resetClosureEntriesV1();attribution.afterClosureEntriesMB=live();si.bindClosureScriptsV1(ROOT,null);attribution.afterClosureScriptsMB=live();(obs as unknown as {releaseInferenceMemoryV1:()=>void}).releaseInferenceMemoryV1();attribution.afterObservationMemosMB=live();(inputs as unknown as {releaseInputMemoryV1:()=>void}).releaseInputMemoryV1();attribution.afterInputMemosMB=live();(memory as unknown as {graphStringInternerV1:{clear:()=>void}}).graphStringInternerV1.clear();attribution.afterInternerMB=live();(result as unknown[]).length=0;attribution.afterResultMB=live();attribution.types=Object.entries(heapStats().objectTypeCounts as Record<string,number>).sort((x,y)=>y[1]-x[1]).slice(0,40) as never;}
 let imageCount=0,imageBytes=0;for(const[,bytes]of G.__m2images??[]){imageCount++;imageBytes+=bytes.length;}
 const out={seconds:Math.round((performance.now()-t0)/100)/10,entries:attribution.entries??(result as unknown[]).length,owners:Object.keys(digests).length,parentPeakMB:peaks.get(process.pid)?.rssMB,parentMaxRssMB:mb(process.resourceUsage().maxRSS*1024),parentHeapPeakMB:heapPeakMB,parentLiveEndMB:liveEndMB,treePeakMB,decodedImages:{count:imageCount,mb:Math.round(imageBytes/1048576*10)/10},attribution,marks:G.__m2,workers:[...peaks].filter(([pid])=>pid!==process.pid).map(([pid,row])=>({pid,...row})),identity:obs.inferenceRecordIdentityV1(),implementation:import.meta.semioCapturedImplementationIdentity??null,series};
 if(E.ROWS_OUT)writeFileSync(E.ROWS_OUT,JSON.stringify(digests));
 if(E.OUT)writeFileSync(E.OUT,JSON.stringify(out));
 console.log("[DEBUG] HARNESS "+JSON.stringify({...out,series:undefined,marks:undefined,workers:out.workers.length,workerPeaksMB:out.workers.map(w=>w.rssMB)}));
 process.exit(0);
}
