"""R43: the cold owner wave runs at the speed of the work and says where the time goes: the memory governor reserves what an owner measurably takes (a sample that only reflects shared garbage or sits at the ceiling no longer poisons the kind), the owners that are known to take minutes run after the light ones, and the owner steps and the closure settlement stages are timed in the trace."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
G=B+"🕸️graph/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))

M=B+"🧠️memory/"
add(M+"🟨️.mjs",
 'learned=new Map(Object.entries(peaks?.read()??{}).filter(([,samples])=>Array.isArray(samples)&&samples.length>0&&samples.every(bytes=>Number.isFinite(bytes)&&bytes>=0)).map(([kind,samples])=>[kind,samples.slice(-policy.peakSamples).map(bytes=>Math.min(bytes,ceilingOf(kind)))]));',
 'learned=new Map(Object.entries(peaks?.read()??{}).filter(([,samples])=>Array.isArray(samples)&&samples.length>0&&samples.every(bytes=>Number.isFinite(bytes)&&bytes>=0)).map(([kind,samples])=>[kind,samples.slice(-policy.peakSamples).map(bytes=>bytes>=ceilingOf(kind)?estimateBytes:bytes)])),soloPeaks=new Map(),lowShares=new Map();')
add(M+"🟨️.mjs",
 'peakOf=kind=>learned.has(kind)?Math.max(...learned.get(kind)):estimateBytes,',
 'peakOf=kind=>learned.has(kind)?Math.max(policy.floorOwnerMegabytes*MEGABYTE,...learned.get(kind)):estimateBytes,')
add(M+"🟨️.mjs",
 '    if(!owner.stopped){const samples=learned.get(owner.kind)??[];if(!owner.shared||samples.length===0||owner.taken>Math.max(...samples)){learned.set(owner.kind,[...samples,Math.min(owner.taken,ceilingOf(owner.kind))].slice(-policy.peakSamples));peaks?.write(Object.fromEntries(learned));}}',
 '    if(!owner.stopped){\n     const kind=owner.kind,samples=learned.get(kind)??[],taken=Math.min(owner.taken,ceilingOf(kind)),window=policy.peakSamples,keep=list=>{learned.set(kind,list.slice(-window));peaks?.write(Object.fromEntries(learned));};\n     if(!owner.shared){soloPeaks.set(kind,Math.max(soloPeaks.get(kind)??0,taken));lowShares.delete(kind);keep([...samples,taken]);}\n     else{\n      const largest=samples.length===0?0:Math.max(...samples),bounded=Math.min(taken,Math.max(soloPeaks.get(kind)??0,estimateBytes));\n      if(samples.length===0||bounded>largest){lowShares.delete(kind);keep([...samples,bounded]);}\n      else if(taken<=largest/2){const count=(lowShares.get(kind)??0)+1;if(count>=4*window){lowShares.delete(kind);keep([taken]);}else lowShares.set(kind,count);}\n      else lowShares.delete(kind);\n     }\n    }')
add(M+"🧬️schema/🔣️.json",'        "initialOwnerMegabytes": 256,\n','        "initialOwnerMegabytes": 256,\n        "floorOwnerMegabytes": 64,\n')
add(M+"🧪️tests/🔣️.json",'  "initialOwnerMegabytes": 256,\n','  "initialOwnerMegabytes": 256,\n  "floorOwnerMegabytes": 64,\n')
add(M+"🟨️.d.mts",'readonly initialOwnerMegabytes:number;','readonly initialOwnerMegabytes:number;readonly floorOwnerMegabytes:number;')

F=M+"🦶️footprint/"
add(F+"🟨️.mjs",
 'let installed,step,pulser,pulses=0,pulsedAt=0,releasedAt=0,releaseCalls=0;',
 '''/** ⏲️ Writes one line to the trace of this process (the settlement stages and the owner steps report themselves here; a trace that cannot be written costs nothing). */
export function noteTraceV1(line,trace=(slowTrace??=createFootprintTraceV1())){trace(line);}

/** ⏲️ The milliseconds of named stages summed over many calls: `time` runs and measures one piece of work, `add` takes a measurement made elsewhere, `text` is `name=Nms/calls` for every stage in the order it first ran. */
export function createStageClockV1({now=Date.now}={}){
 const sums=new Map(),calls=new Map();
 const add=(name,milliseconds)=>{sums.set(name,(sums.get(name)??0)+milliseconds);calls.set(name,(calls.get(name)??0)+1);};
 return Object.freeze({add,async time(name,work){const began=now();try{return await work();}finally{add(name,now()-began);}},totals:()=>Object.fromEntries(sums),text:()=>[...sums].map(([name,milliseconds])=>name+"="+Math.round(milliseconds)+"ms/"+calls.get(name)).join(" ")});
}

/** ⏲️ Where the wall time of an owner goes: `begin` opens an owner, `add` charges a named step to it, `finish` closes it and writes `owner <name> wall=Nms step=Nms ...` when it took `slowMilliseconds`, and every `rollupOwners` owners one `owners n=... wall.avg=... running.avg=... step=avg...` line with the average milliseconds of every step per owner and how many owners ran at once (`running` is what the caller counts at that moment). The steps are the ones the caller names; the part of the wall time no step covers is `other`. */
export function createOwnerClockV1({now=Date.now,trace=(line=>noteTraceV1(line)),slowMilliseconds=3000,rollupOwners=50}={}){
 const open=new Map();let window=[];
 const render=(row,count)=>[...row].map(([name,milliseconds])=>name+"="+Math.round(milliseconds/count)+"ms").join(" ");
 return Object.freeze({
  begin(name){open.set(name,{began:now(),steps:new Map()});},
  add(name,step,milliseconds){const row=open.get(name);if(row!==undefined)row.steps.set(step,(row.steps.get(step)??0)+milliseconds);},
  finish(name,{running=0}={}){
   const row=open.get(name);if(row===undefined)return;open.delete(name);
   const wall=now()-row.began,covered=[...row.steps.values()].reduce((sum,value)=>sum+value,0);row.steps.set("other",Math.max(0,wall-covered));
   if(wall>=slowMilliseconds)trace("owner "+name+" wall="+Math.round(wall)+"ms "+render(row.steps,1));
   window.push({wall,running,steps:row.steps});
   if(window.length>=rollupOwners){
    const sums=new Map();for(const item of window)for(const[step,milliseconds]of item.steps)sums.set(step,(sums.get(step)??0)+milliseconds);
    const count=window.length,walls=window.map(item=>item.wall).sort((a,b)=>a-b);
    trace("owners n="+count+" wall.avg="+Math.round(walls.reduce((sum,value)=>sum+value,0)/count)+"ms wall.p90="+Math.round(walls[Math.floor(count*0.9)])+"ms wall.max="+Math.round(walls[count-1])+"ms running.avg="+(window.reduce((sum,item)=>sum+item.running,0)/count).toFixed(1)+" running.max="+Math.max(...window.map(item=>item.running))+" "+render(sums,count));
    window=[];
   }
  },
 });
}

let installed,step,pulser,pulses=0,pulsedAt=0,releasedAt=0,releaseCalls=0;''')
add(F+"🟨️.d.mts",
 'export function footprintPulseV1():void;',
 '''export function footprintPulseV1():void;
export function noteTraceV1(line:string,trace?:(line:string)=>void):void;
export function createStageClockV1(options?:{now?:()=>number}):Readonly<{add:(name:string,milliseconds:number)=>void;time:<T>(name:string,work:()=>Promise<T>|T)=>Promise<T>;totals:()=>Record<string,number>;text:()=>string}>;
export function createOwnerClockV1(options?:{now?:()=>number;trace?:(line:string)=>void;slowMilliseconds?:number;rollupOwners?:number}):Readonly<{begin:(name:string)=>void;add:(name:string,step:string,milliseconds:number)=>void;finish:(name:string,state?:{running?:number})=>void}>;''')

K=G+"🧮️derived-nodes/🧩️closure-kinds/"
add(K+"🟨️.mjs",
 'import{bindProcessGateV1,footprintGateV1,releaseRuntimeMemoryV1,useEnvironmentGateV1}from"../../../🧠️memory/🦶️footprint/🟨️.mjs";',
 'import{bindProcessGateV1,createStageClockV1,footprintGateV1,noteTraceV1,releaseRuntimeMemoryV1,useEnvironmentGateV1}from"../../../🧠️memory/🦶️footprint/🟨️.mjs";')
add(K+"🟨️.mjs",
 '  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map(),gate=footprintGateV1();',
 '  const clock=createStageClockV1(),settleBegan=Date.now();\n  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map(),gate=footprintGateV1();')
add(K+"🟨️.mjs",
 '   await pool(chunk,async name=>{await ask(name);await progress?.({phase:"verification",done:Math.min(++asked,names.length),total:names.length});});',
 '   await clock.time("ask",()=>pool(chunk,async name=>{await ask(name);await progress?.({phase:"verification",done:Math.min(++asked,names.length),total:names.length});}));')
add(K+"🟨️.mjs",
 '    await stage("parts",partIds.length,async tick=>{await Promise.all(partIds.map(async id=>{await metaLoad(id);await tick();}));});',
 '    await clock.time("parts",()=>stage("parts",partIds.length,async tick=>{await Promise.all(partIds.map(async id=>{await metaLoad(id);await tick();}));}));')
add(K+"🟨️.mjs",
 '    await stage("invalidation",1,async tick=>{try{await graph.invalidate({paths:forced,...dirty?.directories?{directories:dirty.directories}:{}},{signal});}catch{signal?.throwIfAborted();}await tick();});',
 '    await clock.time("invalidate",()=>stage("invalidation",1,async tick=>{try{await graph.invalidate({paths:forced,...dirty?.directories?{directories:dirty.directories}:{}},{signal});}catch{signal?.throwIfAborted();}await tick();}));')
add(K+"🟨️.mjs",
 '    await stage("rederivation",redo.length,tick=>pool(redo,async name=>{await ask(name);await tick();}));',
 '    await clock.time("rederive",()=>stage("rederivation",redo.length,tick=>pool(redo,async name=>{await ask(name);await tick();})));')
add(K+"🟨️.mjs",
 '   for(const name of chunk){\n    const row=pseudoRows.get(name),node=nodes.get(name);',
 '   const supportAt=Date.now();\n   for(const name of chunk){\n    const row=pseudoRows.get(name),node=nodes.get(name);')
add(K+"🟨️.mjs",
 '   if(vouching!==undefined&&recorded.length)await vouching.seal(recorded);\n   await stage("flush",1,async tick=>{await graph.flush();await tick();});',
 '   clock.add("support",Date.now()-supportAt);\n   if(vouching!==undefined&&recorded.length)await clock.time("seal",()=>vouching.seal(recorded));\n   await clock.time("flush",()=>stage("flush",1,async tick=>{await graph.flush();await tick();}));')
add(K+"🟨️.mjs",
 '   const outside=[],affected=await affectedPartsV1(book,summary,async rows=>{const failing=await classify(rows,signal)?[]:rows.filter(row=>broken.has(row));outside.push(...failing);return failing;});\n   await vouching?.begin(new Set(outside.flatMap(row=>book.outside.get(rowKeyV1(row))?.parts??[])));',
 '   const outside=[],affectedAt=Date.now(),affected=await affectedPartsV1(book,summary,async rows=>{const failing=await classify(rows,signal)?[]:rows.filter(row=>broken.has(row));outside.push(...failing);return failing;});\n   await vouching?.begin(new Set(outside.flatMap(row=>book.outside.get(rowKeyV1(row))?.parts??[])));clock.add("affected",Date.now()-affectedAt);')
add(K+"🟨️.mjs",
 '  forgetParts(true);await saveManifest();releaseRuntimeMemoryV1();\n  return{held,kept,changed,answers,failed,vouched,reused:Math.max(0,names.length-failed-fresh.size)};',
 '  forgetParts(true);await clock.time("save",saveManifest);releaseRuntimeMemoryV1();\n  noteTraceV1("settle closures="+names.length+" entries="+book.entries.size+" slow="+slow.length+" vouched="+vouched+" kept="+kept.size+" failed="+failed+" summary="+(summary===undefined?"none":"paths:"+summary.paths.size+",dirs:"+summary.directories.size+",configs:"+summary.configs+",modules:"+summary.modules)+" wall="+(Date.now()-settleBegan)+"ms "+clock.text());\n  return{held,kept,changed,answers,failed,vouched,reused:Math.max(0,names.length-failed-fresh.size)};')

P=G+"📥️inference/📦️project-nodes/"
add(P+"🟨️.mjs",
 'import { useEnvironmentGateV1, noteSlowV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";',
 'import { useEnvironmentGateV1, noteSlowV1, createOwnerClockV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";')
add(P+"🟨️.mjs",
 '/** 🪙️ What one project may take on this machine',
 '''/** 🐌️ The recorded seconds from which an owner counts as slow: it runs after the owners that take seconds, so the minutes of its synchronous stretches never hold back the light owners and their workers. */
export const OWNER_SLOW_SECONDS_V1 = 120;

/** 🧭️ The order owners start in: the owners the cost table says need a solo grant (`alone`) last, before them the ones recorded as slow (`slow`), the held requests first (`rank`) within each class; the order inside a class is kept. */
export function orderOwnerJobsV1(jobs, { alone = new Set(), slow = new Set(), rank } = {}) {
  const tier = job => alone.has(job.name) ? 2 : slow.has(job.name) ? 1 : 0;
  return jobs.sort((a, b) => tier(a) - tier(b) || (rank === undefined ? 0 : rank(a.name) - rank(b.name)));
}

/** 🪙️ What one project may take on this machine''')
add(P+"🟨️.mjs",
 '    if (!Number.isFinite(grant.managedMegabytes)) return { cost, recorder, limitBytes: undefined };',
 '    const slow = names => { const owners = recorder.table().owners ?? {}; return names.filter(name => (owners[name]?.seconds ?? 0) >= OWNER_SLOW_SECONDS_V1); };\n    if (!Number.isFinite(grant.managedMegabytes)) return { cost, recorder, limitBytes: undefined, slow };')
add(P+"🟨️.mjs",
 '      recordedBytes: name =>',
 '      slow,\n      recordedBytes: name =>')
add(P+"🟨️.mjs",
 '  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal });',
 '  const collector = createErrorCollectorFromEnvV1({ workspaceRoot, signal: context.invocation?.control.signal }), clock = createOwnerClockV1();')
add(P+"🟨️.mjs",
 '    const authorityMilliseconds=performance.now()-started;',
 '    const authorityMilliseconds=performance.now()-started;clock.add(name,"authority",authorityMilliseconds);')
add(P+"🟨️.mjs",
 'prepare:farm?()=>{missed.add(name);return away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}));}:undefined},()=>{missed.add(name);return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission);}));',
 'prepare:farm?()=>{missed.add(name);const at=performance.now();return away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey})).finally(()=>clock.add(name,"farm",performance.now()-at));}:undefined},()=>{missed.add(name);const at=performance.now();return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission).finally(()=>clock.add(name,"inline",performance.now()-at));}));')
add(P+"🟨️.mjs",
 '    noteSlowV1("owner-inference "+name,inferStartedAt,10000);let rows=',
 '    clock.add(name,"observe",Date.now()-inferStartedAt);noteSlowV1("owner-inference "+name,inferStartedAt,10000);let rows=')
add(P+"🟨️.mjs",
 'finally{noteSlowV1("resolve-closure-refs "+name,startedAt,5000);}',
 'finally{clock.add(name,"refs",Date.now()-startedAt);noteSlowV1("resolve-closure-refs "+name,startedAt,5000);}')
add(P+"🟨️.mjs",
 '    options?.onOwnerObservations?.(name,rows,derived);releaseProjectInferenceRetentionV1(cachePath);',
 '    {const at=performance.now();options?.onOwnerObservations?.(name,rows,derived);clock.add(name,"observations",performance.now()-at);}releaseProjectInferenceRetentionV1(cachePath);')
add(P+"🟨️.mjs",
 'jobs.sort((a,b)=>Number(alone.has(a.name))-Number(alone.has(b.name))||(rank===undefined?0:rank(a.name)-rank(b.name)));',
 'orderOwnerJobsV1(jobs,{alone,slow:new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]),rank});')
add(P+"🟨️.mjs",
 'if(options?.skipOwner?.(job.name)===true){finished++;continue;}await governor.run(async({away,scope})=>{decision=undefined;active.set(job.name,{since:performance.now(),kind:ownerKindV1(job.json,workspaceRoot),cachePath:ownerCachePathV1(workspaceRoot,job.json.root)});await publishRunning();try{',
 'if(options?.skipOwner?.(job.name)===true){finished++;continue;}clock.begin(job.name);const requested=performance.now();await governor.run(async({away,scope})=>{clock.add(job.name,"queue",performance.now()-requested);decision=undefined;active.set(job.name,{since:performance.now(),kind:ownerKindV1(job.json,workspaceRoot),cachePath:ownerCachePathV1(workspaceRoot,job.json.root)});{const at=performance.now();await publishRunning();clock.add(job.name,"publish",performance.now()-at);}try{')
add(P+"🟨️.mjs",
 'active.delete(job.name);decision=undefined;releaseOwnerStateV1(scripts,{shrink:governor.pressure()});}},{kind:ownerKindV1(job.json,workspaceRoot),key:job.name,...solo?{limitBytes:enforcement.soloLimitBytes()}:{}});if(!again)finished++;}',
 'active.delete(job.name);decision=undefined;{const at=performance.now();releaseOwnerStateV1(scripts,{shrink:governor.pressure()});clock.add(job.name,"release",performance.now()-at);}}},{kind:ownerKindV1(job.json,workspaceRoot),key:job.name,...solo?{limitBytes:enforcement.soloLimitBytes()}:{}});clock.finish(job.name,{running:active.size});if(!again)finished++;}')
add(M+"🧪️tests/🟦️.ts",'stored:{value:Record<string,number[]>}={value:{"language:cargo":[3000*MEGABYTE],"root:language:cargo":[4600*MEGABYTE]}};','stored:{value:Record<string,number[]>}={value:{"language:cargo":[2000*MEGABYTE],"root:language:cargo":[4600*MEGABYTE]}};')
