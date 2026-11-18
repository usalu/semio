import json,re
from lib import *

def patch_load(t):
    t=once(t,"const isOwnerRegistryInvalidationV1=(value:unknown):value is OwnerRegistryInvalidationV1=>value===ownerRegistryInvalidationV1;\n",
"""const isOwnerRegistryInvalidationV1=(value:unknown):value is OwnerRegistryInvalidationV1=>value===ownerRegistryInvalidationV1;

/** 🕯️ The admission facts of the owners this process observed, each with the rows that observation consumed: a later attempt of the same read asks for the same owner again, and a memo whose rows still hold answers without observing it, so one edit re-observes the owners whose rows it touched and no others.
 * @see ../../🧪️tests/🕸️graph/🩹️repair-rounds */
const ownerFactsMemoV1=new Map<string,{readonly facts:unknown;readonly rows:readonly unknown[]}>();

function rememberOwnerFactsV1(key:string,entry:{readonly facts:unknown;readonly rows:readonly unknown[]}):void{
  ownerFactsMemoV1.delete(key);ownerFactsMemoV1.set(key,entry);
  while(ownerFactsMemoV1.size>GRAPH_LIMITS_V1.ownerFactsMemoEntries)ownerFactsMemoV1.delete(ownerFactsMemoV1.keys().next().value as string);
}

/** 🪄️ Answers the admission facts of one owner from the memo while every row of its observation holds, and observes the owner (recording its rows apart) otherwise; the rows of a reused answer are retained by the capture around the caller, so the result stays held to what it rests on.
 * @see ../../🧪️tests/🕸️graph/🩹️repair-rounds */
export async function observeOwnerFactsMemoizedV1(input:{readonly key:string;readonly hold:(rows:readonly unknown[])=>Promise<boolean>;readonly observation:Pick<typeof observationV1,"captureObservedSourceInputsV1"|"retainObservedRowsV1">;readonly observe:()=>Promise<unknown>}):Promise<unknown>{
  const known=ownerFactsMemoV1.get(input.key);
  if(known!==undefined&&await input.hold(known.rows)){input.observation.retainObservedRowsV1(known.rows as never);return known.facts;}
  const fresh=await input.observation.captureObservedSourceInputsV1(input.observe,undefined,false);
  rememberOwnerFactsV1(input.key,{facts:fresh.value,rows:fresh.observations});return fresh.value;
}
""","memo")
    t=once(t,"return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures},inputs);});",
"return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return await observeOwnerFactsMemoizedV1({key:scope+\"\\0\"+memoModule.implementation+\"\\0\"+owner.name+\"\\0\"+JSON.stringify([targets,inputs]),hold:rows=>buildGraphSourceFactsHoldV1(rows,invocation),observation,observe:()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures},inputs)}) as never;});","callback")
    t=once(t,"  try{captured=await observation.captureObservedSourceInputsV1(async()=>{\n","  const discoverOnce=()=>observation.captureObservedSourceInputsV1(async()=>{\n","discoverOnce head")
    i=t.index("  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});}\n  catch(error){")
    j=t.index("\n",t.index("  catch(error){",i))
    old=t[i:j]
    body=old[old.index("  catch(error){")+len("  catch(error){"):-1]
    new="  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});\n  const discoveryFailure=async(error:unknown):Promise<undefined>=>{"+body+"};\n  try{captured=await discoverOnce();}catch(error){return discoveryFailure(error);}"
    t=t[:i]+new+t[j:]
    t=once(t,"  const after=final.digest;\n  captured.observations=[...initial.observations,...captured.observations];\n  const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);});\n",
"""  let after=final.digest;
  const stale:{kind:string;path:string;fact:unknown}[]=[],collect=(row:{kind:string;path:string;fact:unknown})=>{if(stale.length<8)stale.push(row);},unchanged=()=>!selected||JSON.stringify(initial.admission)===JSON.stringify(final.admission);
  captured.observations=[...final.observations,...captured.observations];
  let holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,collect);
  for(let repair=0;!holds&&before===after&&unchanged()&&repair<GRAPH_LIMITS_V1.repairRounds;repair++){
    const repairStarted=performance.now();stale.length=0;
    let again;try{again=await discoverOnce();}catch(error){return discoveryFailure(error);}
    if(isOwnerRegistryInvalidationV1(again.value)){invalidated("owner-registry",{before});return again.value;}
    try{final=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership",{before});throw error;}
    if(isOwnerRegistryInvalidationV1(final)){invalidated("owner-registry",{before});return final;}
    after=final.digest;captured={value:again.value,observations:[...final.observations,...again.observations]};
    holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,collect);timing("graph-repair",repairStarted,{round:repair+1,holds});
  }
""","tail")
    return t

def patch_observation(t):
    return once(t,"/** 🔁️ The stored row arrays already merged into one scope:",
"""/** 🧵️ Adds rows an earlier observation consumed, and that hold now, to the capture and the settlement scope around the caller, so a result reused instead of recomputed stays held to the facts it rests on. */
export function retainObservedRowsV1(rows){retainObservationsV1(sourceCaptures.getStore(),rows);retainObservationsV1(scopes.getStore(),rows);}

/** 🔁️ The stored row arrays already merged into one scope:""","retain")

def patch_observation_d(t):
    return once(t,"export function settleObservedComputationV1<T>(","export function retainObservedRowsV1(rows:readonly unknown[]):void;\nexport function settleObservedComputationV1<T>(","retain d")

def patch_schema(t):
    t=once(t,'"snapshotReadChunkBytes": 65536\n','"snapshotReadChunkBytes": 65536,\n        "repairRounds": 4,\n        "ownerFactsMemoEntries": 16384\n',"limits")
    t=once(t,'"snapshotReadChunkBytes": "The size of one read while a retained graph snapshot is staged (64 KiB)."\n','"snapshotReadChunkBytes": "The size of one read while a retained graph snapshot is staged (64 KiB).",\n          "repairRounds": "The most times one attempt of a graph read repeats only the inference after a source it read changed, with the owner closure kept, before the attempt starts over.",\n          "ownerFactsMemoEntries": "The most owner admission observations one process keeps to answer a repeated attempt without observing the owner again."\n',"en")
    t=once(t,'"snapshotReadChunkBytes": "Die Größe eines Lesevorgangs, während ein behaltener Graph-Schnappschuss bereitgestellt wird (64 KiB)."\n','"snapshotReadChunkBytes": "Die Größe eines Lesevorgangs, während ein behaltener Graph-Schnappschuss bereitgestellt wird (64 KiB).",\n          "repairRounds": "Wie oft ein Versuch eines Graph-Lesens nur die Inferenz wiederholt, nachdem sich eine gelesene Quelle änderte, bei behaltenem Eigentümerabschluss, bevor der Versuch von vorn beginnt.",\n          "ownerFactsMemoEntries": "Wie viele Zulassungsbeobachtungen von Eigentümern ein Prozess behält, um einen wiederholten Versuch ohne neue Beobachtung des Eigentümers zu beantworten."\n',"de")
    return t


BUILD_ROOT="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"

def patch_tree_schema(t):
    t=once(t,'"codeCheckMilliseconds": 60000,\n','"codeCheckMilliseconds": 60000,\n        "staleStageMilliseconds": 1800000,\n',"stale key")
    t=once(t,'"codeRoots": ["🧰️framework/🔨️modules/🏃️process", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"],','"codeRoots": ["🧰️framework/🔨️modules/🏃️process", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching", "'+BUILD_ROOT+'"],',"roots")
    return t

def patch_tree(t):
    t=once(t,"holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor)};","holderWaitRestartMilliseconds:scaledMillisecondsV1(policy.holderWaitRestartMilliseconds,factor),staleStageMilliseconds:scaledMillisecondsV1(policy.staleStageMilliseconds,factor)};","scaled")
    t=once(t,"/** 🧬️ The first file below `codeRoots`","""/** 🕰️ Whether a stage that shows progress is stopped to start again with the new code: it has run for `staleStageMilliseconds` and the code it loaded at its start changed since, so a read that reports progress on code that is hours behind hands over (what it finished is stored and kept) and one that started a moment ago is not interrupted by every landing. A pure function.
 * @see ./🧪️tests/🕰️stale-stage */
export function readyStaleRestartV1(ageMs:number,codeChanged:boolean,policy:typeof P=P):boolean{return codeChanged&&ageMs>=policy.staleStageMilliseconds;}

/** 🧬️ The first file below `codeRoots`""","fn")
    return t

def patch_ready_schema(t):
    en='          "stage-code-changed": "Stage {stage} waited {minutes} minutes for a named holder while the code it runs changed ({file}); its whole process tree was stopped and the stage starts again at once with the new code (not counted as a failure).",\n'
    de='          "stage-code-changed": "Stufe {stage} wartete {minutes} Minuten auf einen benannten Halter, während sich der Code änderte, den sie ausführt ({file}); ihr ganzer Prozessbaum wurde beendet und die Stufe startet sofort neu mit dem neuen Code (zählt nicht als Fehler).",\n'
    t=once(t,en,en+'          "stage-code-stale": "Stage {stage} had run for at least {minutes} minutes when the code it runs changed ({file}); its whole process tree was stopped and the stage starts again at once with the new code, everything it finished stays stored (not counted as a failure).",\n',"en")
    t=once(t,de,de+'          "stage-code-stale": "Stufe {stage} lief mindestens {minutes} Minuten, als sich der Code änderte, den sie ausführt ({file}); ihr ganzer Prozessbaum wurde beendet und die Stufe startet sofort neu mit dem neuen Code, alles Fertige bleibt gespeichert (zählt nicht als Fehler).",\n',"de")
    t=once(t,'"stage-code-changed", "store-full"','"stage-code-changed", "stage-code-stale", "store-full"',"enum")
    return t

def patch_ready(t):
    t=once(t,"readyHolderRestartV1,readyHolderWaitTrackerV1,","readyHolderRestartV1,readyStaleRestartV1,readyHolderWaitTrackerV1,","import")
    t=once(t,'"stage-code-changed"|"store-full"','"stage-code-changed"|"stage-code-stale"|"store-full"',"type")
    t=once(t,"codeChanged?:string;detached:boolean","codeChanged?:string;codeStale?:boolean;detached:boolean","result")
    t=once(t,"codeFile:string|undefined,codeCheckedAt=0,","codeFile:string|undefined,codeStale=false,codeCheckedAt=0,","state")
    t=once(t,"finish(codeFile===undefined||result.detached?full:{...full,codeChanged:codeFile})","finish(codeFile===undefined||result.detached?full:{...full,codeChanged:codeFile,...codeStale?{codeStale:true}:{}})","done")
    t=once(t,'if(!killed&&waiting>=treePolicy.holderWaitRestartMilliseconds&&Date.now()-codeCheckedAt>=treePolicy.codeCheckMilliseconds){codeCheckedAt=Date.now();const file=readyCodeChangedSinceV1(input.root,inflight.startedMs,treePolicy);if(file!==null&&readyHolderRestartV1(waiting,true,treePolicy)){codeFile=file;stalled=true;kill("restart");}}',
'if(!killed&&(waiting>=treePolicy.holderWaitRestartMilliseconds||readyStaleRestartV1(Date.now()-inflight.startedMs,true,treePolicy))&&Date.now()-codeCheckedAt>=treePolicy.codeCheckMilliseconds){codeCheckedAt=Date.now();const file=readyCodeChangedSinceV1(input.root,inflight.startedMs,treePolicy),holder=readyHolderRestartV1(waiting,true,treePolicy);if(file!==null&&(holder||readyStaleRestartV1(Date.now()-inflight.startedMs,true,treePolicy))){codeFile=file;codeStale=!holder;stalled=true;kill("restart");}}',"tick")
    t=once(t,"const byCode=result.codeChanged!==undefined,minutes=Math.round((byCode?READY_TREE_POLICY_V1.holderWaitRestartMilliseconds:READY_TREE_POLICY_V1.stallStopMilliseconds)/60000)","const byCode=result.codeChanged!==undefined,stale=result.codeStale===true,minutes=Math.round((stale?READY_TREE_POLICY_V1.staleStageMilliseconds:byCode?READY_TREE_POLICY_V1.holderWaitRestartMilliseconds:READY_TREE_POLICY_V1.stallStopMilliseconds)/60000)","handler")
    t=once(t,"     if(byCode)ports.say(`stage ${row.name} waited","     if(stale)ports.say(`stage ${row.name} had run for at least ${minutes} min when its code changed (${result.codeChanged}), its process tree was stopped, it starts again at once with the new code and keeps what it finished / Stufe ${row.name} lief mindestens ${minutes} Min., als sich ihr Code änderte (${result.codeChanged}), ihr Prozessbaum wurde beendet, sie startet sofort neu mit dem neuen Code und behält alles Fertige`);\n     else if(byCode)ports.say(`stage ${row.name} waited","say")
    t=once(t,'refusal:byCode?readyRefusalV1("stage-code-changed",{stage:row.name,minutes,file:result.codeChanged!}):','refusal:stale?readyRefusalV1("stage-code-stale",{stage:row.name,minutes,file:result.codeChanged!}):byCode?readyRefusalV1("stage-code-changed",{stage:row.name,minutes,file:result.codeChanged!}):',"refusal")
    return t

READY_TEST=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🟦️.ts"
STALE_HANDLER_TEST='''
test("a stage stopped because it had run long while its code changed starts again at once: no failure, no attempt, no back-off, its own refusal and reason in both languages",async()=>{
 const w=world({store:temp("store"),outcomes:[{},{},{},{exit:null,stalled:true,codeChanged:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts",codeStale:true,tail:["Reading the project graph slice (13930 s): taken 1657 of 4455."]},{}],stopAfterSleeps:6});
 await runReadyV1(w.ports,w.controller.signal);
 expect(w.shared.calls.slice(0,6)).toEqual(["native","repo-mcp","dependencies","tools","tools","workspace"]);
 expect(w.updates.some(update=>update.state==="backoff")).toBe(false);expect(w.shared.state.failure).toBeNull();expect(w.shared.state.attempts).toBe(0);
 const stopped=w.updates.find(update=>update.refusal?.code==="stage-code-stale")!;
 expect(stopped.refusal!.en).toContain("30 minutes");expect(stopped.refusal!.en).toContain("load/");expect(stopped.refusal!.de).toContain("30 Minuten");expect(stopped.refusal!.de).toContain("neuen Code");
 expect(w.said.find(text=>text.includes("had run for at least"))).toContain("Stufe tools lief mindestens");
 const restarted=w.updates.filter(update=>update.state==="running"&&update.stage==="tools");
 expect(restarted[1]!.reason!.code).toBe("code-changed");
});
'''
def patch_ready_test(t):
    return t.rstrip("\n")+"\n"+STALE_HANDLER_TEST

def patch_tree_test(t):
    t=once(t,"expect(scaled.holderWaitRestartMilliseconds).toBe(1698000);","expect(scaled.holderWaitRestartMilliseconds).toBe(1698000);expect(scaled.staleStageMilliseconds).toBe(Math.ceil(P.staleStageMilliseconds*2.83));","scaled expect")
    t=once(t,"expect({...scaled,stallMilliseconds:0,stallStopMilliseconds:0,holderWaitRestartMilliseconds:0}).toEqual({...P,stallMilliseconds:0,stallStopMilliseconds:0,holderWaitRestartMilliseconds:0});","expect({...scaled,stallMilliseconds:0,stallStopMilliseconds:0,holderWaitRestartMilliseconds:0,staleStageMilliseconds:0}).toEqual({...P,stallMilliseconds:0,stallStopMilliseconds:0,holderWaitRestartMilliseconds:0,staleStageMilliseconds:0});","scaled equal")
    return t

TARGETS={LOAD:patch_load,GRAPH_SCHEMA:patch_schema,TREE:patch_tree,TREE_SCHEMA:patch_tree_schema,READY:patch_ready,READY_SCHEMA:patch_ready_schema,READY_TEST:patch_ready_test,TREE_TEST:patch_tree_test}
