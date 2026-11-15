#!/usr/bin/env python3
"""P11 edit set: anchored replacements on live files (each anchor must occur exactly the stated number of times) and new test files copied from P11/new. Used for the scratch mirror and, unchanged, by land.py."""
import os
LIVE="/Users/Shared/semio"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G=LIB+"/🕸️graph"
GEN=G+"/🕰️generations"
STEER=GEN+"/🧭️steering"
EDITS=[]
def rep(path,old,new,count=1):EDITS.append((path,old,new,count))

rep(STEER+"/🧬️schema/🔣️.json",'''    "Priority": {''','''    "Offered": {
      "description": "EN: What the read has offered to the fleet cache is kept in one file next to the steering files, which the start of a read does not clear: a project that a wave of the read looked up itself, or that an earlier read offered, is not offered again, and one call offers at most one wave of names, so the first wave after a start costs what any later wave costs. A name expires after the time to live. DE: Was der Lauf dem Flotten-Cache angeboten hat, steht in einer Datei neben den Steuerdateien, die der Start eines Laufs nicht löscht: ein Projekt, das eine Welle des Laufs selbst nachgeschlagen hat oder das ein früherer Lauf angeboten hat, wird nicht erneut angeboten, und ein Aufruf bietet höchstens eine Welle an Namen an, damit die erste Welle nach einem Start so viel kostet wie jede spätere. Ein Name verfällt nach der Lebensdauer.",
      "const": { "file": "offered.json", "maximumNames": 16384, "ttlMilliseconds": 21600000, "maximumPerWave": 48 }
    },
    "OfferedFile": {
      "type": "object",
      "additionalProperties": false,
      "required": ["version", "offered"],
      "properties": {
        "version": { "const": 1 },
        "offered": { "type": "object", "maxProperties": 16384, "propertyNames": { "minLength": 1, "maxLength": 512 }, "additionalProperties": { "type": "integer", "minimum": 0 } }
      }
    },
    "Priority": {''')

rep(STEER+"/🟨️.mjs",'''const variable=schema.definitions.Environment.const.directory,files=schema.definitions.Files.const;''','''const variable=schema.definitions.Environment.const.directory,files=schema.definitions.Files.const,offering=schema.definitions.Offered.const;''')
rep(STEER+"/🟨️.mjs",'''/** 🆕️ The priority projects that are fresh in this tree and not offered yet: the server's plan (pending file, written against the current priority) does not list them as dirty, so their records exist locally and only have to reach the fleet cache. Empty while the pending file does not answer the current priority. */
export function freshPriorityV1(environment,offered){
 const paths=steeringPathsV1(environment),priority=readPriorityV1(environment),pending=paths&&readPendingV1(paths.directory);
 if(priority===undefined||pending===undefined||pending.priorityAtMs<priority.atMs)return[];
 const dirty=new Set(pending.projects);
 return[...new Set(priority.projects)].filter(name=>!dirty.has(name)&&!offered.has(name));
}''','''/** 🆕️ At most `limit` priority projects (one wave unless told otherwise) that are fresh in this tree, not offered yet and accepted by `known`: the server's plan (pending file, written against the current priority) does not list them as dirty, so their records exist locally and only have to reach the fleet cache. Empty while the pending file does not answer the current priority. */
export function freshPriorityV1(environment,offered,limit=offering.maximumPerWave,known=()=>true){
 const paths=steeringPathsV1(environment),priority=readPriorityV1(environment),pending=paths&&readPendingV1(paths.directory);
 if(priority===undefined||pending===undefined||pending.priorityAtMs<priority.atMs)return[];
 const dirty=new Set(pending.projects),out=[];
 for(const name of new Set(priority.projects)){if(out.length>=limit)break;if(!dirty.has(name)&&!offered.has(name)&&known(name))out.push(name);}
 return out;
}

/** 📚️ The names an earlier call or read offered to the fleet cache and that have not outlived the time to live; nothing when no directory is named or the file is missing or damaged. */
export function readOfferedV1(environment=process.env,now=Date.now()){
 const paths=steeringPathsV1(environment),row=paths&&readJson(join(paths.directory,offering.file)),out=new Set();
 if(!row||row.version!==1||row.offered===null||typeof row.offered!=="object"||Array.isArray(row.offered))return out;
 for(const[name,at]of Object.entries(row.offered))if(Number.isSafeInteger(at)&&at>=0&&now-at<offering.ttlMilliseconds)out.add(name);
 return out;
}

/** 📝️ Notes names as offered at `now`: merges them into the file, drops the expired and, above the maximum, the oldest names, and replaces the file atomically. Never throws: a read does not depend on its notes. */
export function writeOfferedV1(environment,names,now=Date.now()){
 const paths=steeringPathsV1(environment);if(paths===undefined)return;
 try{
  const row=readJson(join(paths.directory,offering.file)),kept=row&&row.version===1&&row.offered!==null&&typeof row.offered==="object"&&!Array.isArray(row.offered)?Object.entries(row.offered).filter(([,at])=>Number.isSafeInteger(at)&&at>=0&&now-at<offering.ttlMilliseconds):[];
  const merged=new Map(kept);for(const name of names)merged.set(name,now);
  const offered=Object.fromEntries([...merged].sort((a,b)=>b[1]-a[1]).slice(0,offering.maximumNames).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0)),path=join(paths.directory,offering.file),temporary=path+"."+process.pid+".tmp";
  mkdirSync(paths.directory,{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,offered}));renameSync(temporary,path);
 }catch{}
}''')
rep(STEER+"/🟨️.d.mts",'''export function freshPriorityV1(environment:Readonly<Record<string,string|undefined>>,offered:ReadonlySet<string>):string[];''','''export function freshPriorityV1(environment:Readonly<Record<string,string|undefined>>,offered:ReadonlySet<string>,limit?:number,known?:(name:string)=>boolean):string[];
export function readOfferedV1(environment?:Readonly<Record<string,string|undefined>>,now?:number):Set<string>;
export function writeOfferedV1(environment:Readonly<Record<string,string|undefined>>,names:Iterable<string>,now?:number):void;''')

PF=G+"/🤝️prefetch/🟦️.ts"
rep(PF,'''const offeredFreshV1=new Set<string>();
''','''export type OfferedStateV1={readonly names:Set<string>;readonly loaded:Set<string>};
/** 🧺️ The names one read has offered and the directories their file was read from; a fresh state is a restarted read. */
export const createOfferedStateV1=():OfferedStateV1=>({names:new Set(),loaded:new Set()});
const processOfferedV1=createOfferedStateV1();
const loadOfferedV1=async(state:OfferedStateV1,environment:NodeJS.ProcessEnv):Promise<void>=>{
  const{steeringPathsV1,readOfferedV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs"),directory=steeringPathsV1(environment)?.directory;
  if(directory===undefined||state.loaded.has(directory))return;
  state.loaded.add(directory);for(const name of readOfferedV1(environment))state.names.add(name);
};
/** 📝️ Notes the projects a wave of this read looked up itself as offered: the lookup of a hit offers its record and an inference uploads it, so the offer pass has nothing left to do for them, now or after a restart of the read. */
export async function noteOfferedV1(names:readonly string[],environment:NodeJS.ProcessEnv=process.env,state:OfferedStateV1=processOfferedV1):Promise<void>{
  const{writeOfferedV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs");
  await loadOfferedV1(state,environment);for(const name of names)state.names.add(name);writeOfferedV1(environment,names);
}
''')
rep(PF,'''/** 📤️ Offers the records of the projects other machines asked for and that are fresh in this tree to the fleet cache: a fresh project is not inferred by the read (its record holds), so nothing would upload it; one pass over exactly these projects looks their records up (a hit offers it) and infers nothing.''','''/** 📤️ Offers the records of the projects other machines asked for and that are fresh in this tree to the fleet cache: a fresh project is not inferred by the read (its record holds), so nothing would upload it; one pass over exactly these projects looks their records up (a hit offers it) and infers nothing. A call offers at most `limit` of them (one wave of the read, at least the policy wave), never a project a wave of this read looked up itself and never one an earlier read offered (`noteOfferedV1`).''')
rep(PF,'''  const{freshPriorityV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs"),names=freshPriorityV1(input.environment??process.env,offeredFreshV1).filter(name=>input.stubs[name]!==undefined);
  if(names.length===0)return 0;
  try{await input.offer(new Set(names.map(name=>input.stubs[name]!.root)));for(const name of names)offeredFreshV1.add(name);return names.length;}''','''  const environment=input.environment??process.env,state=input.state??processOfferedV1,{freshPriorityV1,writeOfferedV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs");
  await loadOfferedV1(state,environment);
  const names=freshPriorityV1(environment,state.names,input.limit,name=>input.stubs[name]!==undefined);
  if(names.length===0)return 0;
  try{await input.offer(new Set(names.map(name=>input.stubs[name]!.root)));for(const name of names)state.names.add(name);writeOfferedV1(environment,names);return names.length;}''')

rep(PF,'offer(roots:ReadonlySet<string>):Promise<unknown>;environment?:NodeJS.ProcessEnv;say?(line:string):void}):Promise<number>{','offer(roots:ReadonlySet<string>):Promise<unknown>;environment?:NodeJS.ProcessEnv;state?:OfferedStateV1;limit?:number;say?(line:string):void}):Promise<number>{')

HOST=G+"/🕰️host/🟦️.ts"
rep(HOST,'fleetReadStaleGenerationV1,offerFreshPriorityV1,prefetchBuildGraphProjectsV1} from "../🤝️prefetch/🟦️.ts";','fleetReadStaleGenerationV1,offerFreshPriorityV1,noteOfferedV1,prefetchBuildGraphProjectsV1} from "../🤝️prefetch/🟦️.ts";')
rep(HOST,'''    release:async()=>{observationV1.releaseInferenceMemoryV1();(await import("../../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();if(!state.carry){''','''    release:async(options?:{more?:boolean})=>{observationV1.releaseInferenceMemoryV1();(await import("../../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();if(!state.carry&&options?.more!==true){''')
rep(HOST,'''requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");await offerFreshPriorityV1({stubs:spec.stubs,offer:roots=>prefetchBuildGraphProjectsV1(root,registry,invocation,control,state.files,roots),say:line=>process.stderr.write(line+"\\n")});return discoverBuildGraphPartV1(root,registry,invocation,control,state.files,{''','''requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");const offerStartedMs=performance.now();await offerFreshPriorityV1({stubs:spec.stubs,limit:Math.max(GENERATION_POLICY_V1.ownerWave,spec.owners.length),offer:roots=>prefetchBuildGraphProjectsV1(root,registry,invocation,control,state.files,roots),say:line=>process.stderr.write(line+"\\n")});const offerMs=Math.round(performance.now()-offerStartedMs);const part=await discoverBuildGraphPartV1(root,registry,invocation,control,state.files,{''')
rep(HOST,'''previousOwners:previousEnvironmentOwnersV1(spec.stubs)});},
    expand:''','''previousOwners:previousEnvironmentOwnersV1(spec.stubs)});await noteOfferedV1(spec.owners);return{...part,phases:{...part.phases,offer:offerMs}};},
    expand:''')

GENM=GEN+"/🟨️.mjs"
rep(HOST,'import {GENERATION_POLICY_V1,createFrontierV1,','import {GENERATION_POLICY_V1,carriedStagesV1,createFrontierV1,')
rep(HOST,'''if(process.env.SEMIO_GRAPH_BACKGROUND==="1")await(await import("../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts")).yieldBackgroundGraphV1({''','''if(process.env.SEMIO_GRAPH_BACKGROUND==="1")carriedStagesV1.yield=await(await import("../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts")).yieldBackgroundGraphV1({''')
rep(GENM,'''  const verifiable=verifiableRowIdsV1(index,selection.ids),''','''  const factsAt=now(),verifiable=verifiableRowIdsV1(index,selection.ids),''')
rep(GENM,'''  await doubt?.settled?.({rows:index.rows,ids:verifiable,stale});
  const{direct,suspects:reached}''','''  times.facts=now()-factsAt;await doubt?.settled?.({rows:index.rows,ids:verifiable,stale});
  const settleAt=now();const{direct,suspects:reached}''')
rep(GENM,'''  await settler.release?.();
  for(const name of direct)''','''  await settler.release?.();times.settle=now()-settleAt;
  for(const name of direct)''')
rep(GENM,'''  const index=await profilePhaseV1("graph.plan.index",()=>readIndexView(manifest.factIndex));if(!index)return cold("index");''','''  const times={},indexAt=now(),index=await profilePhaseV1("graph.plan.index",()=>readIndexView(manifest.factIndex));times.index=now()-indexAt;if(!index)return cold("index");''')
rep(GENM,'''return{kind:clean?"clean":"delta",dirty:scoped,''','''return{kind:clean?"clean":"delta",times,dirty:scoped,''')
rep(GENM,'''live=reuse?hint.live:await profilePhaseV1("graph.build.membership",()=>membership(previous?.frontier?previous:undefined,signal));''','''live=reuse?hint.live:await profilePhaseV1("graph.build.membership",async()=>{const began=now();try{return await membership(previous?.frontier?previous:undefined,signal);}finally{marks.membership=now()-began;}});''')
rep(GENM,'''  const priority=readPriorityV1();mark("plan");''','''  const priority=readPriorityV1();mark("plan");for(const[name,ms]of Object.entries(found.times??{}))marks["plan."+name]=ms;''')
rep(GENM,'''  const marks={};let markedAt=now();''','''  const marks={};if(!processStartedV1){processStartedV1=true;marks.start=Math.round(process.uptime()*1000);}Object.assign(marks,carriedStagesV1);for(const key of Object.keys(carriedStagesV1))delete carriedStagesV1[key];let markedAt=now();''')
rep(GENM,'''export{canonicalJsonV1,compareCodePointsV1,createFrontierV1,diffFrontiersV1,frontierFactHoldsV1,frontierHoldsV1,stableDirectoriesV1};
''','''export{canonicalJsonV1,compareCodePointsV1,createFrontierV1,diffFrontiersV1,frontierFactHoldsV1,frontierHoldsV1,stableDirectoriesV1};
let processStartedV1=false;
/** ⏱️ The milliseconds stages outside the build took since the last build (`start.reader` and `start.prefetch` of the first load, `yield` of the pause between two waves): their owners fill it, the next build reports it and empties it. */
export const carriedStagesV1={};
''')
rep(GENM,'''const spec={cold,owners:wave,total:offset+names.length,''','''const spec={cold,owners:wave,more:rest.length>0,total:offset+names.length,''')

rep(GENM,'''plan:{...found,kind:"delta",''','''plan:{...found,times:{},kind:"delta",''')

LOAD=GEN+"/🔌️load/🟨️.mjs"
rep(LOAD,'''import{GENERATION_POLICY_V1,canonicalJsonV1,''','''import{GENERATION_POLICY_V1,carriedStagesV1,canonicalJsonV1,''')
rep(LOAD,'''const first=await host.withShared((store,control)=>attempt(false,store,control));''','''const readerAt=Date.now(),first=await host.withShared((store,control)=>attempt(false,store,control));carriedStagesV1["start.reader"]=(carriedStagesV1["start.reader"]??0)+Date.now()-readerAt;''')
rep(LOAD,'''host.carryClosures?.(true);try{await host.prefetch(request);}finally{host.carryClosures?.(false);}''','''host.carryClosures?.(true);const prefetchAt=Date.now();try{await host.prefetch(request);}finally{host.carryClosures?.(false);carriedStagesV1["start.prefetch"]=(carriedStagesV1["start.prefetch"]??0)+Date.now()-prefetchAt;}''')
rep(LOAD,'''await host.releaseCutoff?.();const part=await host.discoverPart(''','''const cutAt=Date.now();await host.releaseCutoff?.();const cutMs=Date.now()-cutAt;const part=await host.discoverPart(''')
rep(LOAD,'''announceWithheldProjectsV1(host,part);await host.release?.();return shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});}''','''announceWithheldProjectsV1(host,part);const releaseAt=Date.now();await host.release?.({more:spec.more===true});const shaped=shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});return{...shaped,phases:{...shaped.phases,cutoff:cutMs,release:Date.now()-releaseAt}};}''')
rep(GEN+"/🟨️.d.mts",'''export function scaledOwnerWaveV1(ownerWave:number,basisPoints:unknown):number;''','''export function scaledOwnerWaveV1(ownerWave:number,basisPoints:unknown):number;\nexport const carriedStagesV1:Record<string,number>;''')
rep(GEN+"/🔌️load/🟨️.d.mts",''' release?():Promise<void>;''',''' release?(options?:{more?:boolean}):Promise<void>;''')

rep(GEN+"/🧪️tests/📈️adaptive-waves/🟦️.ts",'''  for(const info of told){
   expect(Object.keys(info.phases!).sort()).toEqual(["compute","compute.projects","compute.setup","compute.tail","fragments","pack","patch","plan","prime","publish"]);''','''  for(const[at,info]of told.entries()){
   expect(Object.keys(info.phases!).sort()).toEqual([...(at===0?["plan.facts","plan.index","plan.settle"]:[]),"compute","compute.projects","compute.setup","compute.tail","fragments","pack","patch","plan","prime","publish"].sort());''')

NEWFILES=[
 (STEER+"/🧪️tests/📤️offered/🟦️.ts","steering-offered/🟦️.ts"),
 (G+"/🧪️tests/📤️offered-once/🟦️.ts","offered-once/🟦️.ts"),
 (GEN+"/🧪️tests/⏱️stages/🟦️.ts","stages/🟦️.ts"),
 (GEN+"/🔌️load/🧪️tests/🔁️carried-release/🟦️.ts","carried-release/🟦️.ts"),
]
