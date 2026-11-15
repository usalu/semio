#!/usr/bin/env python3
"""P9 edit set: anchored replacements on live files (each anchor must occur exactly once) and new files copied from P9/new.
Used for the scratch mirror (tests) and, unchanged, by land.py."""
import os,sys,hashlib,json
LIVE="/Users/Shared/semio"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G=LIB+"/🕸️graph"
GEN=G+"/🕰️generations"
NEW=os.path.dirname(os.path.abspath(__file__))+"/new"

EDITS=[]
def rep(path,old,new,count=1):EDITS.append((path,old,new,count))

rep(GEN+"/🧬️schema/🔣️.json",'        "ownerWave": 48,\n','        "ownerWave": 48,\n        "ownerWaveMaximum": 960,\n        "ownerWaveGrowth": 4,\n        "ownerWaveBudgetMilliseconds": 300000,\n')
rep(GEN+"/🧬️schema/🔣️.json",'          "ownerWave": "The most stale owners one build re-infers before it publishes a generation: a build cancelled after a wave keeps every finished wave, and the memory of one build is bounded by a wave instead of by the closure.",\n',
 '          "ownerWave": "The most stale owners one build re-infers before it publishes a generation: a build cancelled after a wave keeps every finished wave, and the memory of one build is bounded by a wave instead of by the closure.",\n'
 '          "ownerWaveMaximum": "The most owners one wave may hold after waves that inferred nothing: a wave of projects whose stored records all hold costs a fixed time of the whole graph, so such waves grow up to this many owners.",\n'
 '          "ownerWaveGrowth": "By how many times at most the next wave grows after a wave that inferred nothing.",\n'
 '          "ownerWaveBudgetMilliseconds": "The time one wave that inferred nothing should take: the next wave is as many owners as fit this time at the speed of the last one, within the growth and the maximum; a wave that inferred many projects returns to the base wave.",\n')
rep(GEN+"/🧬️schema/🔣️.json",'          "ownerWave": "Wie viele veraltete Projekte ein Build höchstens neu ableitet, bevor er eine Generation veröffentlicht: ein nach einer Welle abgebrochener Build behält jede fertige Welle, und der Speicher eines Builds ist durch eine Welle statt durch die Hülle begrenzt.",\n',
 '          "ownerWave": "Wie viele veraltete Projekte ein Build höchstens neu ableitet, bevor er eine Generation veröffentlicht: ein nach einer Welle abgebrochener Build behält jede fertige Welle, und der Speicher eines Builds ist durch eine Welle statt durch die Hülle begrenzt.",\n'
 '          "ownerWaveMaximum": "Wie viele Projekte eine Welle nach Wellen ohne Ableitung höchstens umfasst: eine Welle aus Projekten, deren gespeicherte Einträge alle gelten, kostet eine feste Zeit des ganzen Graphen, darum wachsen solche Wellen bis zu dieser Zahl.",\n'
 '          "ownerWaveGrowth": "Um das Wievielfache die nächste Welle nach einer Welle ohne Ableitung höchstens wächst.",\n'
 '          "ownerWaveBudgetMilliseconds": "Wie lange eine Welle ohne Ableitung dauern soll: die nächste Welle umfasst so viele Projekte, wie in dieser Zeit mit der Geschwindigkeit der letzten passen, innerhalb von Wachstum und Höchstzahl; eine Welle mit vielen abgeleiteten Projekten kehrt zur Grundwelle zurück.",\n')

rep(GEN+"/🟨️.mjs",'/** 🔌️ Admits bounded consumer external roots without changing their sealed package identity. */',
'''/** 📈️ The owners of the wave after one that took `elapsedMilliseconds` for `size` owners and inferred `inferred` of them: a wave that inferred nothing costs a fixed time of the whole graph, so the next one grows toward the time budget at the speed of the last, at most by the growth factor and never beyond the maximum; a wave that inferred a quarter at most holds its size and shrinks only when it was over the budget; a wave that said nothing about its inference or inferred more than a quarter returns to the base wave, whose publication is the progress the fleet waits on. Both bounds follow the share the host could grant (`basisPoints`).
 * @see ./🧬️schema/🔣️.json */
export function nextOwnerWaveV1({size,elapsedMilliseconds,inferred,basisPoints}){
 const base=scaledOwnerWaveV1(P.ownerWave,basisPoints);
 if(!Number.isSafeInteger(inferred)||inferred<0||inferred*4>size)return base;
 const maximum=Math.max(base,scaledOwnerWaveV1(P.ownerWaveMaximum,basisPoints)),fitting=Math.floor(size*P.ownerWaveBudgetMilliseconds/Math.max(1,elapsedMilliseconds));
 return Math.max(base,Math.min(maximum,inferred===0?size*P.ownerWaveGrowth:size,fitting));
}

/** 🔌️ Admits bounded consumer external roots without changing their sealed package identity. */''')
rep(GEN+"/🟨️.mjs",'  await profilePhaseV1("graph.build.prime",async()=>prime?.(signal));\n',
 '  const marks={};let markedAt=now();const mark=name=>{const at=now();marks[name]=(marks[name]??0)+at-markedAt;markedAt=at;};\n  await profilePhaseV1("graph.build.prime",async()=>prime?.(signal));mark("prime");\n')
rep(GEN+"/🟨️.mjs",'  const priority=readPriorityV1();\n','  const priority=readPriorityV1();mark("plan");\n')
rep(GEN+"/🟨️.mjs",'wave=names.slice(0,scaledOwnerWaveV1(P.ownerWave,process.env.SEMIO_SLICE_WAVE_BASIS_POINTS)),rest=names.slice(wave.length);',
 'wave=names.slice(0,hint?.wave?.size??scaledOwnerWaveV1(P.ownerWave,process.env.SEMIO_SLICE_WAVE_BASIS_POINTS)),rest=names.slice(wave.length),waveStartedMs=now();')
rep(GEN+"/🟨️.mjs",'    const result=await profilePhaseV1("graph.build.compute",()=>compute(spec,signal));own(result);\n','    const result=await profilePhaseV1("graph.build.compute",()=>compute(spec,signal));own(result);mark("compute");\n')
rep(GEN+"/🟨️.mjs",'    await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));\n','    await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,job=>job(),signal));mark("fragments");\n')
rep(GEN+"/🟨️.mjs",'  const packedIndex=await profilePhaseV1("graph.build.pack"','  mark("patch");\n  const packedIndex=await profilePhaseV1("graph.build.pack"')
rep(GEN+"/🟨️.mjs",'  const globalStored=await store(P.fragments,','  mark("pack");\n  const globalStored=await store(P.fragments,')
rep(GEN+"/🟨️.mjs",'  if(!done&&!cold){index=undefined;const merged=await mergeIntoHeadV1(','  mark("publish");\n  if(!done&&!cold){index=undefined;const merged=await mergeIntoHeadV1(')
rep(GEN+"/🟨️.mjs",'wave:{done:offset+wave.length},plan:','wave:{done:offset+wave.length,size:nextOwnerWaveV1({size:wave.length,elapsedMilliseconds:now()-waveStartedMs,inferred:result.inferred,basisPoints:process.env.SEMIO_SLICE_WAVE_BASIS_POINTS})},plan:')
rep(GEN+"/🟨️.mjs",'  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,published:true,plan:found,computed:result.owners.length,','  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,published:true,plan:found,computed:result.owners.length,phases:{...marks,...Object.fromEntries(Object.entries(result.phases??{}).map(([name,ms])=>["compute."+name,ms]))},')
rep(GEN+"/🟨️.mjs",'await request.published?.(last.current.manifest.generation,{done:total,total:total+(last.deferred?.plan?.dirty?.size??0)});','await request.published?.(last.current.manifest.generation,{done:total,total:total+(last.deferred?.plan?.dirty?.size??0),...(last.phases?{phases:last.phases}:{})});')

rep(GEN+"/🟨️.d.mts",'export function scaledOwnerWaveV1(ownerWave:number,basisPoints:unknown):number;','export function scaledOwnerWaveV1(ownerWave:number,basisPoints:unknown):number;\nexport function nextOwnerWaveV1(input:{size:number;elapsedMilliseconds:number;inferred?:number;basisPoints?:unknown}):number;')
rep(GEN+"/🟨️.d.mts",'derived?:import("./✂️cutoff/🟨️.mjs").DerivedFactV1[];dispose?:()=>void|Promise<void>}','derived?:import("./✂️cutoff/🟨️.mjs").DerivedFactV1[];inferred?:number;phases?:Record<string,number>;dispose?:()=>void|Promise<void>}')
rep(GEN+"/🔌️load/🟨️.mjs",'...(part.dispose?{dispose:part.dispose}:{})};\n}','...(Number.isSafeInteger(part.inferred)?{inferred:part.inferred}:{}),...(part.phases?{phases:part.phases}:{}),...(part.dispose?{dispose:part.dispose}:{})};\n}')
rep(GEN+"/🔌️load/🟨️.d.mts",'export interface GenerationPartV1{','export interface GenerationPartV1{inferred?:number;phases?:Record<string,number>;')

rep(G+"/📋️model/🟦️.ts",'export interface BuildGraphPartV1 {owners:','export interface BuildGraphPartV1 {inferred?:number;phases?:Record<string,number>;owners:')

DISC=G+"/🧭️discovery/🟦️.ts"
rep(DISC,'snapshot:true,...(part.cold?{}:{ownerRoots:part.roots}),','snapshot:true,onInferred:(count:number)=>{inferred=count;},onPhases:(times:Record<string,number>)=>{Object.assign(phases,times);},...(part.cold?{}:{ownerRoots:part.roots}),')
rep(DISC,'  const rowLog=createRowLogV1<BuildGraphRowV1>({path:join(rowLogDirectoryV1(root),"rl-"+process.pid+"-"+Date.now().toString(36)+".rows")}),edgeReads=','  let inferred:number|undefined;const phases:Record<string,number>={};\n  const rowLog=createRowLogV1<BuildGraphRowV1>({path:join(rowLogDirectoryV1(root),"rl-"+process.pid+"-"+Date.now().toString(36)+".rows")}),edgeReads=')
rep(DISC,'    if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");\n    const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),tags:Record<string,string>={},configs:Record<string,string>={},primaryNames=new Set<string>(),testNames=new Set<string>();',
 '    const tailStartedMs=performance.now();\n    if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");\n    const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),tags:Record<string,string>={},configs:Record<string,string>={},primaryNames=new Set<string>(),testNames=new Set<string>();')
rep(DISC,'      edgeOnly:[...edgeNames.map(','      ...(inferred===undefined?{}:{inferred}),phases:{...phases,tail:Math.round(performance.now()-tailStartedMs)},\n      edgeOnly:[...edgeNames.map(')

PN=G+"/📥️inference/📦️project-nodes/🟨️.mjs"
rep(PN,'  const { workspaceRoot } = context, unavailable = new Map(), wantedRoots = options?.ownerRoots, wanted = root => !wantedRoots || wantedRoots.has(root);\n','  const { workspaceRoot } = context, unavailable = new Map(), wantedRoots = options?.ownerRoots, wanted = root => !wantedRoots || wantedRoots.has(root), enteredMs = performance.now();\n')
rep(PN,'let projectIndex=0,cursor=0,stopped=false;\n','let projectIndex=0,cursor=0,stopped=false;const missed=new Set();\n')
rep(PN,'prepare:farm?()=>away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey})):undefined},()=>inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission)));',
 'prepare:farm?()=>{missed.add(name);return away(()=>farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey}));}:undefined},()=>{missed.add(name);return inferProjectBodyV1(projectEnvironment,{name,root,json,native,pipeline,nativeRoot,selectedTargets,selectedInputs},options?.ownerAdmission);}));')
rep(PN,'  const workers=Array.from({length:limit},loop);\n','  const loopStartedMs=performance.now();\n  const workers=Array.from({length:limit},loop);\n')
rep(PN,'  graphWaveMeterV1.parent(parentCollector.finish());\n','  options?.onPhases?.({setup:Math.round(loopStartedMs-enteredMs),projects:Math.round(performance.now()-loopStartedMs)});options?.onInferred?.(missed.size);\n  graphWaveMeterV1.parent(parentCollector.finish());\n')

WR=G+"/📈️wave-row"
rep(WR+"/🧬️schema/🔣️.json",'        "maximumPeakProjects": 4,\n','        "maximumPeakProjects": 4,\n        "maximumPhases": 24,\n')
rep(WR+"/🧬️schema/🔣️.json",'        "collectMilliseconds": { "type": "integer", "minimum": 0 },\n','        "collectMilliseconds": { "type": "integer", "minimum": 0 },\n        "phases": { "type": "object", "maxProperties": 24, "propertyNames": { "minLength": 1, "maxLength": 64 }, "additionalProperties": { "type": "integer", "minimum": 0 } },\n')
rep(WR+"/🧬️schema/🔣️.json",'          "peakAt": " at {projects}",\n','          "peakAt": " at {projects}",\n          "phases": ", phases: {list}",\n')
rep(WR+"/🧬️schema/🔣️.json",'          "peakAt": " bei {projects}",\n','          "peakAt": " bei {projects}",\n          "phases": ", Phasen: {list}",\n')
rep(WR+"/🟨️.mjs",'const ROW_KEYS=schema.definitions.Row.required,','const ROW_KEYS=schema.definitions.Row.required,')
rep(WR+"/🟨️.mjs",' let wave=fresh(),lastAt=now(),total=0,done=0;',' let wave=fresh(),lastAt=now(),total=0,done=0,phases={};')
rep(WR+"/🟨️.mjs",'  progress(info){if(natural(info?.total))total=info.total;if(natural(info?.done))done=info.done;},','  progress(info){if(natural(info?.total))total=info.total;if(natural(info?.done))done=info.done;if(info?.phases!==null&&typeof info?.phases==="object")phases=Object.fromEntries(Object.entries(info.phases).filter(([name,ms])=>name!==""&&name.length<=64&&natural(Math.round(ms))).slice(0,P.maximumPhases).map(([name,ms])=>[name,Math.round(ms)]));},')
rep(WR+"/🟨️.mjs",'declined:wave.declined}};\n   wave=fresh();lastAt=at;return row;','declined:wave.declined},...(Object.keys(phases).length?{phases}:{})};\n   wave=fresh();lastAt=at;phases={};return row;')
rep(WR+"/🟨️.mjs",' if(!same(row,ROW_KEYS)||row.version!==1',' const phased=row?.phases;if(phased!==undefined&&(phased===null||typeof phased!=="object"||Array.isArray(phased)||Object.keys(phased).length>P.maximumPhases||Object.values(phased).some(value=>!natural(value))))return undefined;\n if(!same(Object.fromEntries(Object.entries(row??{}).filter(([key])=>key!=="phases")),ROW_KEYS)||row.version!==1')
rep(WR+"/🟨️.mjs",'workers:row.farm.workers});\n}','workers:row.farm.workers})+(row.phases&&Object.keys(row.phases).length?fill(texts.phases,{list:Object.entries(row.phases).map(([name,ms])=>name+" "+Math.round(ms/1000)+" s").join(", ")}):"");\n}')

rep(GEN+"/🧪️tests/📈️wave-progress/🟦️.ts",'expect(told).toEqual([[before+1,{done:wave,total:COUNT}],[before+2,{done:wave*2,total:COUNT}],[before+3,{done:COUNT,total:COUNT}]]);','expect(told.map(([generation,info])=>[generation,{done:(info as {done:number}).done,total:(info as {total:number}).total}])).toEqual([[before+1,{done:wave,total:COUNT}],[before+2,{done:wave*2,total:COUNT}],[before+3,{done:COUNT,total:COUNT}]]);')

rep(G+"/📥️inference/📦️project-nodes/🧪️tests/🚪️away/🟦️.ts",'expect(source).toContain("prepare:farm?()=>away(()=>farm.prepare(");','expect(source).toContain("prepare:farm?()=>{missed.add(name);return away(()=>farm.prepare(");')

NEWFILES=[
 (GEN+"/🧪️tests/📈️adaptive-waves/🟦️.ts","adaptive-waves/🟦️.ts"),
 (GEN+"/🧪️tests/📈️adaptive-waves/🔣️.json","adaptive-waves/🔣️.json"),
 (GEN+"/🧪️tests/📈️adaptive-waves/🧬️schema/🔣️.json","adaptive-waves/🧬️schema/🔣️.json"),
 (G+"/📈️wave-row/🧪️tests/⏱️phases/🟦️.ts","wave-phases/🟦️.ts"),
 (G+"/📥️inference/📦️project-nodes/🧪️tests/📈️inferred/🟦️.ts","inferred/🟦️.ts"),
]
