// [farmid] Harness parent (copied to <mirror>/fiharness/run.ts; not a library file): the production parent loop emojiProjectJsonNodes over ONLY_ROOTS with the project farm exactly as production opens it, per-owner row digests (analysis revisions normalized), peak RSS of this process and of every descendant (ps), wall. env: ROWS_DUMP (dir: the normalized rows of every owner as JSON lines), GRAM_ROOT (workspace data), LIST_ROOT (the git checkout that lists the tracked files, filtered to those present under GRAM_ROOT), STORE (scratch graph store), ONLY_ROOTS, OUT, ROWS_OUT, SAMPLE_MS.
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
import*as revision from"../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
export async function runHarnessV1():Promise<void>{
 const E=process.env,ROOT=E.GRAM_ROOT!,STORE=E.STORE!,t0=performance.now(),mb=(n:number)=>Math.round(n/1048576);
 rmSync(STORE,{recursive:true,force:true});mkdirSync(STORE,{recursive:true});
 await (inference as {libraryBootstrap:Promise<unknown>}).libraryBootstrap;
 const peaks=new Map<number,{rssMB:number;command:string;firstS:number;lastS:number}>();let treePeakMB=0;const series:{t:number;self:number;heap:number;tree:number;workers:number}[]=[];let heapPeakMB=0;
 const sample=()=>{const now=Math.round((performance.now()-t0)/100)/10,rows=execFileSync("ps",["-A","-o","pid=,ppid=,rss=,command="],{maxBuffer:1<<26}).toString().split("\n").map(line=>/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.*)$/.exec(line)).filter(Boolean).map(m=>({pid:Number(m![1]),ppid:Number(m![2]),rss:Number(m![3])*1024,command:m![4]!}));
  const kids=new Map<number,number[]>();for(const row of rows){const list=kids.get(row.ppid)??[];list.push(row.pid);kids.set(row.ppid,list);}const tree=new Set([process.pid]),stack=[process.pid];while(stack.length)for(const child of kids.get(stack.pop()!)??[])if(!tree.has(child)){tree.add(child);stack.push(child);}
  let total=0,workers=0,self=0;for(const row of rows)if(tree.has(row.pid)&&row.command.includes("bun")){total+=row.rss;if(row.pid===process.pid)self=row.rss;else workers++;const seen=peaks.get(row.pid);peaks.set(row.pid,{rssMB:Math.max(seen?.rssMB??0,mb(row.rss)),command:row.pid===process.pid?"parent":row.command.includes("graph-farm-worker")?"worker":row.command.slice(-60),firstS:seen?.firstS??now,lastS:now});}
  const h=heapStats(),heap=mb(h.heapSize+h.extraMemorySize);heapPeakMB=Math.max(heapPeakMB,heap);treePeakMB=Math.max(treePeakMB,mb(total));series.push({t:now,self:mb(self),heap,tree:mb(total),workers});};
 const timer=setInterval(sample,Number(E.SAMPLE_MS??"1000"));
 const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(ROOT,"📋️build-owners.json"),"utf8")));
 const all=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:E.LIST_ROOT??ROOT,maxBuffer:1<<29}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")&&existsSync(join(ROOT,f)));
 const files=all.filter(f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||f.endsWith("bun.lock")||f.endsWith(".patch")),ownerRoots=new Set(E.ONLY_ROOTS!.split(";").filter(Boolean)),digests:Record<string,string>={};
 const options={analyzeLockfile:true,catalogProviders:registry.catalogProviders.filter(({ownerRoot}:{ownerRoot:string})=>existsSync(join(ROOT,ownerRoot,"📋️project.json"))).map(({ownerRoot,sourcePath}:{ownerRoot:string;sourcePath:string})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots,onOwnerObservations:(name:string,rows:{kind:string;path:string;fact:{provider?:string}}[])=>{const normal=rows.map(r=>[r.kind,r.path,r.kind==="analysis"?{...r.fact,provider:String(r.fact.provider).replace(/:[0-9a-f]{64}$/,":rev"),revision:"rev"}:r.fact]);digests[name]=createHash("sha256").update(JSON.stringify(normal)).digest("hex")+":"+rows.length;if(E.ROWS_DUMP){mkdirSync(E.ROWS_DUMP,{recursive:true});writeFileSync(join(E.ROWS_DUMP,name.replace(/[^\w.@-]/g,"_")+".jsonl"),normal.map(row=>JSON.stringify(row)).join("\n")+"\n");}}};
 const context={workspaceRoot:ROOT,registry,preparationControl:{advance:async()=>{}}};
 sample();
 const result=await revision.withImplementationPinV1(()=>storage.withBuildGraphStagingV1({root:STORE,published:()=>{}},()=>obs.observeInferenceSnapshotV1(()=>(nodes as {emojiProjectJsonNodes:Function}).emojiProjectJsonNodes(files,options,context))));
 clearInterval(timer);sample();Bun.gc(true);const end=heapStats(),liveEndMB=mb(end.heapSize+end.extraMemorySize);
 const out={seconds:Math.round((performance.now()-t0)/100)/10,entries:(result as unknown[]).length,owners:Object.keys(digests).length,parentPeakMB:peaks.get(process.pid)?.rssMB,parentHeapPeakMB:heapPeakMB,parentLiveEndMB:liveEndMB,treePeakMB,workers:[...peaks].filter(([pid])=>pid!==process.pid).map(([pid,row])=>({pid,...row})),identity:obs.inferenceRecordIdentityV1(),implementation:import.meta.semioCapturedImplementationIdentity??null,series};
 if(E.ROWS_OUT)writeFileSync(E.ROWS_OUT,JSON.stringify(digests));
 if(E.OUT)writeFileSync(E.OUT,JSON.stringify(out));
 console.log("[DEBUG] HARNESS "+JSON.stringify({...out,series:undefined,workers:out.workers.length,workerPeaksMB:out.workers.map(w=>w.rssMB)}));
 process.exit(0);
}
