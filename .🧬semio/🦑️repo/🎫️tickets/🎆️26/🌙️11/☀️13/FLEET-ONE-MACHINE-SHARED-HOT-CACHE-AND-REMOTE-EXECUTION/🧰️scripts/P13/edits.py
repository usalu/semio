#!/usr/bin/env python3
"""P13 edit set: anchored replacements on live files (each anchor must occur exactly the stated number of times) and new files copied from P13/new. Used for the scratch overlay and, unchanged, by land.py."""
import os,json
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
G=LIB+"/🕸️graph"
GEN=G+"/🕰️generations"
EXC=GEN+"/🌐️remote-exchange"
UP=GEN+"/📡️wave-upload"
LOAD=GEN+"/🔌️load/🟨️.mjs"
HOST=G+"/🕰️host/🟦️.ts"
SIDE=LIB+"/🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts"
EDITS=[]
def rep(path,old,new,count=1):EDITS.append((path,old,new,count))

X=EXC+"/🟦️.ts"
XS=EXC+"/🧬️schema/🔣️.json"

rep(XS,'''        "restoreMarginMilliseconds": 60000
''','''        "restoreMarginMilliseconds": 60000,
        "unmeasuredStale": 500,
        "staleOwnersPerMinute": 1,
        "ownersPerChangedPath": 0.25,
        "shadowMarginOwners": 50,
        "servingTotalMegabytes": 24000,
        "rankCandidates": 3
''')
rep(XS,'''        "signature",
        "platform",
        "publishedMs"
      ],''','''        "signature",
        "platform",
        "publishedMs",
        "origin",
        "stateMs",
        "implementation"
      ],''')
rep(XS,'''        "publishedMs": {
          "type": "integer",
          "minimum": 0
        },
''','''        "publishedMs": {
          "type": "integer",
          "minimum": 0
        },
        "origin": {
          "type": "string",
          "pattern": "^[a-z0-9][a-z0-9-]{0,39}$",
          "description": "The machine of the group that wrote this head."
        },
        "stateMs": {
          "type": "integer",
          "minimum": 0,
          "description": "When the producer last changed its generation store: the time of the tree state the generation describes, never the time of the upload, so a head uploaded later by a machine that holds an older state does not outrank a newer state."
        },
        "implementation": {
          "type": "string",
          "pattern": "^[a-f0-9]{64}$",
          "description": "The digest of the code that built the generation: a reader with other code finds every owner of it stale."
        },
        "stale": {
          "type": "integer",
          "minimum": 0,
          "maximum": 65536,
          "description": "How many owners the producer knew stale in this generation when it was written (0 for a complete read); absent when the producer did not know, which readers count as the policy's unmeasuredStale."
        },
''')

rep(X,'''import {join} from "node:path";
import {validateJsonSchemaSubset}''','''import {totalmem} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset}''')
rep(X,'''platform:string;publishedMs:number;snapshot?:string}>;''','''platform:string;publishedMs:number;origin:string;stateMs:number;implementation:string;stale?:number;snapshot?:string}>;''')
rep(X,'''/** 🏷️ The inline description of one uploaded generation: what a client compares before it downloads anything. `manifest` names the neutral manifest; `signature` is the root independent state; `publishedMs` orders uploads of different machines.''','''/** 🏷️ The inline description of one uploaded generation: what a client compares before it downloads anything. `manifest` names the neutral manifest; `signature` is the root independent state; `stateMs` is the time of the tree state it describes and `stale` the owners its producer knew stale in it, which together rank the heads of different machines (`publishedMs` only dates the upload).''')
rep(X,'''install(generation:number,snapshot:{instance:string;bytes:Uint8Array}):Promise<void>}>;''','''install(generation:number,snapshot:{instance:string;bytes:Uint8Array}):Promise<void>;distance?(snapshot:{instance:string;bytes:Uint8Array}):Promise<number|undefined>}>;''')
rep(X,'''status:"restored"|"uploaded"|"local"|"absent"|"foreign"|"empty"|"current"|"remote-newer"|"incomplete"|"oversized"|"failed";generation?:number;owners?:number;documents?:number;bytes?:number;reason?:string}>;''','''status:"restored"|"uploaded"|"local"|"absent"|"foreign"|"empty"|"current"|"remote-better"|"incomplete"|"oversized"|"failed";generation?:number;owners?:number;documents?:number;sent?:number;bytes?:number;reason?:string}>;''')
rep(X,'''restore(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal}):Promise<GenerationExchangeOutcomeV1>;''','''restore(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string}):Promise<GenerationExchangeOutcomeV1>;''')
rep(X,'''ahead(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal}):Promise<boolean>;
 publish(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;ifAbsent?:boolean}):Promise<GenerationExchangeOutcomeV1>;''','''ahead(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string}):Promise<boolean>;
 /** ⬆️ Offers the newest sealed generation unless the cache holds a head that is at least as good; `stale` is the number of owners this producer knows stale in it (0 after a complete read, absent when unknown). */
 publish(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;ifAbsent?:boolean;stale?:number}):Promise<GenerationExchangeOutcomeV1>;''')
rep(X,'''signal?:AbortSignal;now?:()=>number;cursors?:GenerationCursorsV1;keys?:readonly string[]}>;''','''signal?:AbortSignal;now?:()=>number;cursors?:GenerationCursorsV1;keys?:readonly string[];origin?:string;implementation?:string;stale?:number;serving?:boolean}>;''')

rep(X,'''/** 🪪️ The origin of this machine in the replica group''','''type HeadV1=Readonly<{note:GenerationNoteV1|undefined;entries:readonly {path:string;hash:string;size:number}[];from:string}>;
type RankedV1=HeadV1&Readonly<{note:GenerationNoteV1}>;

/** 🏁️ How many owners a reader would find stale in a head at `nowMs`: the owners its producer knew stale (all of them when the reader's code differs from the code that built it, the policy's guess when the producer did not know), plus the owners the tree moved under since: counted from the paths that changed between the head's snapshot and the reader's tree when the reader measured them, else estimated from the age of the state. Lower is better. */
export function generationHeadScoreV1(note:GenerationNoteV1,nowMs:number,reader:Readonly<{implementation?:string;changedPaths?:number}>={}):number{
 const known=reader.implementation!==undefined&&reader.implementation!==note.implementation?note.owners:Math.min(note.owners,note.stale??policy.unmeasuredStale);
 return known+(reader.changedPaths!==undefined?reader.changedPaths*policy.ownersPerChangedPath:Math.max(0,nowMs-note.stateMs)/60000*policy.staleOwnersPerMinute);
}

/** 🥇️ Orders heads best first: the fewest owners a reader would find stale, then the newest tree state, then the origin name; the time of an upload never counts, so an old state that reached the cache later does not shadow a newer one. */
export function rankGenerationHeadsV1<T extends Readonly<{note:GenerationNoteV1}>>(heads:readonly T[],nowMs:number,reader:Readonly<{implementation?:string;changed?:ReadonlyMap<T,number>}>={}):T[]{
 const score=new Map(heads.map(head=>[head,generationHeadScoreV1(head.note,nowMs,{implementation:reader.implementation,changedPaths:reader.changed?.get(head)})] as const));
 return[...heads].sort((a,b)=>score.get(a)!-score.get(b)!||b.note.stateMs-a.note.stateMs||(a.note.origin<b.note.origin?-1:a.note.origin>b.note.origin?1:0));
}

/** 🪪️ The origin of this machine in the replica group''')

old_remote=open("/Users/Shared/semio/"+X,encoding="utf8").read()
s=old_remote.index("async function remoteNote(")
e=old_remote.index("async function localState(")
rep(X,old_remote[s:e],'''async function readHead(location:Location):Promise<HeadV1|undefined>{
 const bytes=await location.content.readActionResult(location.instance,location.key,location.signal);if(!bytes)return undefined;
 const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),files=(result.output_files??[]) as {path:string;digest:{hash:string;size_bytes:number|string}}[];
 return{note:noteOf(result.stdout_raw as Uint8Array|undefined),entries:files.map(file=>({path:file.path,hash:file.digest.hash,size:Number(file.digest.size_bytes)})),from:location.key};
}

async function remoteNote(location:Location,reader:Readonly<{implementation?:string;nearest?:boolean}>={}):Promise<HeadV1|undefined>{
 const keys=location.keys??[location.key],settled=await Promise.allSettled(keys.map(key=>readHead({...location,key,keys:undefined})));
 if(settled.every(row=>row.status==="rejected"))throw(settled[0] as PromiseRejectedResult).reason;
 const found=settled.flatMap(row=>row.status==="fulfilled"&&row.value!==undefined?[row.value]:[]),valid=found.filter((head):head is RankedV1=>head.note!==undefined);
 if(valid.length===0)return found[0];
 const now=(location.now??Date.now)(),ranked=rankGenerationHeadsV1(valid,now,{implementation:reader.implementation});
 if(!reader.nearest||ranked.length<2||!location.cursors?.distance)return ranked[0];
 const candidates=ranked.slice(0,policy.rankCandidates),changed=new Map<RankedV1,number>();
 for(const head of candidates){
  const path=head.note.snapshot===undefined?undefined:`s/${head.note.snapshot}.snapshot.br`,entry=head.entries.find(row=>row.path===path);
  if(!entry||!snapshotDocument.test(entry.path)||!(entry.size<=policy.maximumSnapshotBytes))return ranked[0];
  const data=await location.content.read(location.instance,{hash:entry.hash,size:entry.size},location.signal).catch(()=>undefined);
  if(!data||sha(data)!==entry.hash)return ranked[0];
  const distance=await location.cursors.distance({instance:head.note.snapshot!,bytes:data}).catch(()=>undefined);
  if(distance===undefined)return ranked[0];
  changed.set(head,distance);
 }
 return rankGenerationHeadsV1(candidates,now,{implementation:reader.implementation,changed})[0];
}

/** 🛡️ Whether a store keeps its own generation instead of taking the head `note` of `from`: it already holds that state (same signature from its own head, or a head that is not newer than its pointer by the margin), or it holds a generation of the reader's own code and the head was built by other code, which would leave every owner stale. */
function keepsLocalV1(local:NonNullable<Awaited<ReturnType<typeof localState>>>,note:GenerationNoteV1,from:string,location:Location&Readonly<{marginMilliseconds?:number}>):boolean{
 if(generationSignatureV1(local.manifest)===note.signature&&from===location.key||note.stateMs<=local.modifiedMs+(location.marginMilliseconds??policy.restoreMarginMilliseconds))return true;
 return location.implementation!==undefined&&note.implementation!==location.implementation&&local.manifest.implementation===location.implementation;
}

/** 🆔️ The registry identity of the generation a store holds, for a process that has no registry of its own (the agent that uploads the waves of a read it does not run). */
export async function localGenerationIdentityV1(root:string,files:Files):Promise<string>{
 const local=await localState(root,files);if(!local)throw Error("no sealed generation in "+root);
 return local.manifest.identity;
}

''')

rep(X,'''export async function publishGenerationV1(location:Location&Readonly<{ifAbsent?:boolean}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,key,root,files,signal}=location,local=await localState(root,files);if(!local)return{status:"empty"};
 const signature=generationSignatureV1(local.manifest),remote=await remoteNote(location).catch(()=>undefined);
 if(remote?.note?.signature===signature&&(local.restored||remote.note.publishedMs>=local.modifiedMs))return{status:"current",generation:local.generation,owners:Object.keys(local.manifest.owners).length};
 if(remote&&(location.ifAbsent||remote.note&&remote.note.publishedMs>=local.modifiedMs))return{status:"remote-newer",generation:local.generation,owners:Object.keys(local.manifest.owners).length};''','''export async function publishGenerationV1(location:Location&Readonly<{ifAbsent?:boolean}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,key,root,files,signal}=location,local=await localState(root,files);if(!local)return{status:"empty"};
 const signature=generationSignatureV1(local.manifest),remote=await remoteNote(location,{implementation:local.manifest.implementation}).catch(()=>undefined),owners=Object.keys(local.manifest.owners).length;
 if(remote?.note?.signature===signature&&(local.restored||remote.note.stateMs>=Math.floor(local.modifiedMs)))return{status:"current",generation:local.generation,owners};
 const mine=Math.min(owners,location.stale??policy.unmeasuredStale)+(location.serving===false?policy.shadowMarginOwners:0);
 if(remote&&(location.ifAbsent||remote.note!==undefined&&remote.from!==location.key&&generationHeadScoreV1(remote.note,(location.now??Date.now)(),{implementation:local.manifest.implementation})<=mine))return{status:"remote-better",generation:local.generation,owners};''')
rep(X,''' const manifestName=moved.manifestPath.slice(P.manifests.length+1,-5);let bytesSent=0;
 await profilePhaseV1("graph.upload.write",()=>each(rows.filter(row=>missing.has(row.hash)),async row=>{const data=row.bytes??sending.get(row.hash)!;await content.write(instance,{hash:row.hash,size:row.size},data,signal);bytesSent+=row.size;}));''',''' const manifestName=moved.manifestPath.slice(P.manifests.length+1,-5);let bytesSent=0,sent=0;
 await profilePhaseV1("graph.upload.write",()=>each(rows.filter(row=>missing.has(row.hash)),async row=>{const data=row.bytes??sending.get(row.hash)!;await content.write(instance,{hash:row.hash,size:row.size},data,signal);bytesSent+=row.size;sent++;}));''')
rep(X,'''platform:location.platform,publishedMs:(location.now??Date.now)(),...shipped?{snapshot:shipped.instance}:{}};''','''platform:location.platform,publishedMs:(location.now??Date.now)(),origin:location.origin??"local",stateMs:Math.floor(local.modifiedMs),implementation:local.manifest.implementation,...location.stale===undefined?{}:{stale:Math.min(owners,Math.max(0,Math.floor(location.stale)))},...shipped?{snapshot:shipped.instance}:{}};''')
rep(X,'''return{status:"uploaded",generation:note.generation,owners:note.owners,documents:rows.length,bytes:bytesSent};''','''return{status:"uploaded",generation:note.generation,owners:note.owners,documents:rows.length,sent,bytes:bytesSent};''')
rep(X,'''unless the cache already holds this state (same signature) or a newer upload of another machine. Documents go first''','''unless the cache already holds this state (same signature) or a head of another machine that is at least as good (it leaves its readers the same number of stale owners or fewer; a machine under the serving memory size must beat it by the shadow margin). Only the documents the cache misses are sent, so an interrupted upload resumes from what arrived. Documents go first''')
rep(X,'''export async function remoteGenerationAheadV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number}>):Promise<boolean>{
 const local=await localState(location.root,location.files),remote=await remoteNote(location);if(!remote)return false;''','''export async function remoteGenerationAheadV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number}>):Promise<boolean>{
 const local=await localState(location.root,location.files),remote=await remoteNote(location,{implementation:location.implementation});if(!remote)return false;''')
rep(X,'''return!(local&&(generationSignatureV1(local.manifest)===note.signature&&remote.from===location.key||note.publishedMs<=local.modifiedMs+(location.marginMilliseconds??policy.restoreMarginMilliseconds)));
}''','''return!(local&&keepsLocalV1(local,note,remote.from,location));
}''')
rep(X,'''const{content,instance,root,files,signal}=location,local=await localState(root,files),remote=await remoteNote(location);if(!remote)return{status:"absent"};''','''const{content,instance,root,files,signal}=location,local=await localState(root,files),remote=await remoteNote(location,{implementation:location.implementation,nearest:true});if(!remote)return{status:"absent"};''')
rep(X,'''if(local&&(generationSignatureV1(local.manifest)===note.signature&&remote.from===location.key||note.publishedMs<=local.modifiedMs+(location.marginMilliseconds??policy.restoreMarginMilliseconds)))return{status:"local"};''','''if(local&&keepsLocalV1(local,note,remote.from,location))return{status:"local"};''')

rep(X,'''origins?:Readonly<{self:string;all:readonly string[]}>}>):GenerationExchangeV1|undefined{''','''origins?:Readonly<{self:string;all:readonly string[]}>;serving?:boolean}>):GenerationExchangeV1|undefined{''')
rep(X,'''const origins=input.origins??fleetOriginsV1(input.workspace),heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin))});''','''const origins=input.origins??fleetOriginsV1(input.workspace),serving=input.serving??totalmem()/1048576>=policy.servingTotalMegabytes,heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin)),origin:origins.self});''')
rep(X,'''  restore:({root,fs,signal})=>guard(async()=>{const identity=await input.identity();return restoreGenerationV1({content:make(false),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,cursors:input.cursors});}),
  ahead:async({root,fs,signal})=>{if(down)return false;try{const identity=await input.identity();return await remoteGenerationAheadV1({content:make(true),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal});}catch{return false;}},
 publish:({root,fs,signal,ifAbsent})=>guard(async()=>publishGenerationV1({content:make(ifAbsent===true),instance:contract.instance,...heads(await input.identity()),root,workspace:input.workspace,platform,files:fs,signal,ifAbsent,cursors:input.cursors})),''','''  restore:({root,fs,signal,implementation})=>guard(async()=>{const identity=await input.identity();return restoreGenerationV1({content:make(false),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors});}),
  ahead:async({root,fs,signal,implementation})=>{if(down)return false;try{const identity=await input.identity();return await remoteGenerationAheadV1({content:make(true),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors});}catch{return false;}},
 publish:({root,fs,signal,ifAbsent,stale})=>guard(async()=>publishGenerationV1({content:make(ifAbsent===true),instance:contract.instance,...heads(await input.identity()),root,workspace:input.workspace,platform,files:fs,signal,ifAbsent,stale,serving,cursors:input.cursors})),''')

rep(LOAD,'''async function restoreRemoteGenerationV1(host,options){
 const started=Date.now(),outcome=await host.exchange.restore({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal});''','''async function restoreRemoteGenerationV1(host,options,implementation){
 const started=Date.now(),outcome=await host.exchange.restore({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,implementation});''')
rep(LOAD,'''  const options=host.storeOptions(store,control),started=Date.now(),outcome=await host.exchange.publish({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,ifAbsent});''','''  const options=host.storeOptions(store,control),started=Date.now(),outcome=await host.exchange.publish({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,ifAbsent,stale:0});''')
rep(LOAD,'''host.exchange.ahead({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal})''','''host.exchange.ahead({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,implementation})''')
rep(LOAD,'''if(builder&&host.exchange)await restoreRemoteGenerationV1(host,options);''','''if(builder&&host.exchange)await restoreRemoteGenerationV1(host,options,implementation);''')

rep(HOST,'''    install:async(generation,snapshot)=>{await journalPort.manifest.store.putBytes(snapshot.instance,snapshot.bytes);cursorStore.set("g"+generation,{instance:snapshot.instance,seq:0});}''','''    install:async(generation,snapshot)=>{await journalPort.manifest.store.putBytes(snapshot.instance,snapshot.bytes);cursorStore.set("g"+generation,{instance:snapshot.instance,seq:0});},
    distance:async snapshot=>{await journalPort.manifest.store.putBytes(snapshot.instance,snapshot.bytes);const since=await journalPort.manifest.since({instance:snapshot.instance,seq:0});return since.complete?since.rows.length:undefined;}''')

rep(SIDE,'''import {createGraphReadWatchV1,graphReadEndTextV1,runWantedServerV1,wantedServerPolicyV1,type WantedInferV1,type WantedServerPolicyV1} from "../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";''','''import {createGraphReadWatchV1,graphReadEndTextV1,runWantedServerV1,wantedServerPolicyV1,type WantedInferV1,type WantedServerPolicyV1} from "../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import {createGenerationUploaderV1,localGenerationUploadSourceV1,WAVE_UPLOAD_POLICY_V1} from "../../../🕸️graph/🕰️generations/📡️wave-upload/🟦️.ts";
import {journalCursorStoreV1} from "../../../🏡️resident/🕰️graph-hooks/📖️direct/🟦️.ts";
import {snapshotStoreV1} from "../../../🪪️graph-state/📍️cursor/🟦️.ts";''')
rep(SIDE,'''steerDirectory?:string;costPath?:string;policy?:WantedServerPolicyV1;now?:()=>number;say?:(line:string)=>void}>):Promise<WantedInferV1>{''','''steerDirectory?:string;costPath?:string;policy?:WantedServerPolicyV1;now?:()=>number;say?:(line:string)=>void;onWave?:(row:GraphWaveRowV1)=>void}>):Promise<WantedInferV1>{''')
rep(SIDE,'''watch=createGraphReadWatchV1(policy,now,row=>{input.say?.(renderGraphWaveTextV1(row,input.language));''','''watch=createGraphReadWatchV1(policy,now,row=>{try{input.onWave?.(row);}catch{}input.say?.(renderGraphWaveTextV1(row,input.language));''')
rep(SIDE,'''import {GRAPH_WAVE_POLICY_V1,renderGraphWaveTextV1} from "../../../🕸️graph/📈️wave-row/🟨️.mjs";''','''import {GRAPH_WAVE_POLICY_V1,renderGraphWaveTextV1} from "../../../🕸️graph/📈️wave-row/🟨️.mjs";
import type {GraphWaveRowV1} from "../../../🕸️graph/📈️wave-row/🟨️.mjs";''')
rep(SIDE,'''  return inferGraphReadV1({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language,steerDirectory,costPath,say:context.write});
 };
 await runWantedServerV1({''','''  return inferGraphReadV1({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language,steerDirectory,costPath,say:context.write,onWave:row=>uploader?.wave(row)});
 };
 const serving=(await readRemoteExecutionHostViewV1(environment)).host.totalMegabytes>=policy.serveTotalMegabytes;
 let uploader:ReturnType<typeof createGenerationUploaderV1>|undefined,tickTimer:ReturnType<typeof setInterval>|undefined;
 if(serving){
  const{repositoryCacheBudgetV1}=await import("../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts"),generations=join(repositoryCacheBudgetV1(context.root).partitions.graph.directory,"m","generations"),snapshots=snapshotStoreV1(context.root),cursorStore=journalCursorStoreV1(context.root,environment);
  uploader=createGenerationUploaderV1({language,say:context.write,statePath:join(steerDirectory,WAVE_UPLOAD_POLICY_V1.stateFile),
   source:localGenerationUploadSourceV1({workspace:context.root,root:generations,environment,cursors:{snapshotFor:async generation=>{const entry=cursorStore.get("g"+generation);if(!entry||entry.carry)return undefined;const bytes=await snapshots.bytes(entry.instance);return bytes?{instance:entry.instance,bytes}:undefined;},install:async()=>undefined}}),
   async may(){const host=(await readRemoteExecutionHostViewV1(environment)).host;return host.pressure==="normal"&&host.availableMegabytes>=WAVE_UPLOAD_POLICY_V1.minimumFreeMegabytes;}});
  tickTimer=setInterval(()=>uploader?.tick(),policy.roundMilliseconds);tickTimer.unref?.();
 }
 try{await runWantedServerV1({''')
rep(SIDE,'''  async quietMilliseconds(){try{return Math.max(0,Date.now()-statSync(join(context.root,".cache","f","state.wal")).mtimeMs);}catch{return 0;}}});
}''','''  async quietMilliseconds(){try{return Math.max(0,Date.now()-statSync(join(context.root,".cache","f","state.wal")).mtimeMs);}catch{return 0;}}});}
 finally{if(tickTimer!==undefined)clearInterval(tickTimer);uploader?.stop();await uploader?.idle();}
}''')

MSG=GEN+"/🧬️schema/🔣️.json"
EN_OLD='restored from the remote cache, {documents} documents ({megabytes} MB) downloaded in {seconds} s'
DE_OLD='aus dem Remote-Cache wiederhergestellt, {documents} Dokumente ({megabytes} MB) in {seconds} s geladen'
for path in (LOAD,MSG):
    rep(path,EN_OLD+'"',EN_OLD+', from a state {age} min old with {stale} projects stale"')
    rep(path,DE_OLD+'"',DE_OLD+', von einem Stand von vor {age} Min. mit {stale} veralteten Projekten"')
rep(LOAD,'''if(outcome.status==="restored")host.say("restored",{generation:outcome.generation,owners:outcome.owners,documents:outcome.documents,megabytes:Math.round(outcome.bytes/1048576),seconds:Math.round((Date.now()-started)/1000)});''','''if(outcome.status==="restored")host.say("restored",{generation:outcome.generation,owners:outcome.owners,documents:outcome.documents,megabytes:Math.round(outcome.bytes/1048576),seconds:Math.round((Date.now()-started)/1000),age:Math.round((outcome.ageSeconds??0)/60),stale:outcome.stale??"?"});''')
rep(X,'''generation?:number;owners?:number;documents?:number;sent?:number;bytes?:number;reason?:string}>;''','''generation?:number;owners?:number;documents?:number;sent?:number;bytes?:number;ageSeconds?:number;stale?:number;reason?:string}>;''')
rep(X,'''return{status:"restored",generation:manifest.generation,owners:note.owners,documents:moved.documents.length,bytes};''','''return{status:"restored",generation:manifest.generation,owners:note.owners,documents:moved.documents.length,bytes,ageSeconds:Math.max(0,Math.round(((location.now??Date.now)()-note.stateMs)/1000)),...note.stale===undefined?{}:{stale:note.stale}};''')

NEWFILES=[
 (UP+"/🟦️.ts","wave-upload/module.ts"),
 (UP+"/🧬️schema/🔣️.json","wave-upload/schema.json"),
 (UP+"/🧪️tests/🟦️.ts","wave-upload/tests/uploader.ts"),
 (EXC+"/🧪️tests/\U0001F3C1️head-rank/🟦️.ts","head-rank.ts"),
 (EXC+"/🧪️tests/\U0001F4E1️wave-delta/🟦️.ts","wave-delta.ts"),
 (LIB+"/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/\U0001F4E1️wave-upload/🟦️.ts","sidecar-wave-upload.ts"),
]
