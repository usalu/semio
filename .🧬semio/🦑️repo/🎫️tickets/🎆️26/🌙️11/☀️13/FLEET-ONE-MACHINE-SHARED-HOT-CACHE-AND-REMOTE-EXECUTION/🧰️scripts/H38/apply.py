import os,sys,json
DRY="--dry" in sys.argv
G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
pending=[]
def flush():
    if DRY: print("dry ok",len(pending),"files");return
    for path,text in pending:
        tmp=path+".h38tmp"
        open(tmp,"w",encoding="utf8").write(text)
        os.replace(tmp,path)
    print("landed",len(pending),"files")

def chain(path,steps):
    t=open(path,encoding="utf8").read()
    for old,new,count in steps:
        n=t.count(old)
        if n!=count: sys.exit("anchor count %d (want %d) in %s: %s"%(n,count,path,old[:90]))
        t=t.replace(old,new)
    pending.append((path,t))

E=G+"🕰️generations/🟨️.mjs"
chain(E,[
("async function refresh({identity,implementation,policy=\"fresh\",only,membership,compute,prime,signal,progress,builder,requestedAt,published,wave,planned,unchanged,repair,qualify}){",
 "async function refresh({identity,implementation,policy=\"fresh\",only,membership,compute,prime,signal,progress,builder,requestedAt,published,wave,planned,unchanged,repair,qualify,ephemeral}){",1),
("const body=claim=>build({identity,implementation,membership,compute,prime,signal,progress,published,wave,hint,only:scope,observedAtMs:started,coverage,claim,qualify});",
 "const body=claim=>build({identity,implementation,membership,compute,prime,signal,progress,published,wave,hint,only:scope,observedAtMs:started,coverage,claim,qualify,ephemeral});",1),
("if(result.published||!result.built)return{...result,...resumedComputed===undefined",
 "if(result.published||!result.built||result.ephemeral)return{...result,...resumedComputed===undefined",1),
("async function buildOnce({identity,implementation,membership,compute,prime,signal,progress,hint,only,observedAtMs=now(),coverage,claim,repair,qualify},carry){",
 "async function buildOnce({identity,implementation,membership,compute,prime,signal,progress,hint,only,observedAtMs=now(),coverage,claim,repair,qualify,ephemeral=false},carry){",1),
("const resumed=!hint?.wave&&!carry.tried?await journal.resume(",
 "const resumed=!hint?.wave&&!carry.tried&&!ephemeral?await journal.resume(",1),
("await journal.wave({key:journalKey,","if(!ephemeral)await journal.wave({key:journalKey,",1),
("  for(const name of carry.returned)returned.add(name);\n  for(const name of returned)dropped.delete(name);\n",
 """  for(const name of carry.returned)returned.add(name);
  for(const name of returned)dropped.delete(name);
  if(ephemeral){
   if(cold||repair||!usable)throw fail("An overlay refreshes a sealed generation by owners","GENERATION_OVERLAY_COLD");
   const globalDigest=(await store(P.fragments,{version:1,name:"@global",value:globalPayload??null},".json.br",P.maximumFragmentBytes)).digest,{owners:_table,...envParts}=computedParts??{};
   const next={version:1,generation:usable.generation+1,parent:current.pointer.manifest,identity,implementation,env:envDigest,membership:live.digest,frontier:usable.frontier,factIndex:usable.factIndex,global:globalDigest,...(computedParts?{envParts}:{}),owners,built:{kind:"delta",base:usable.generation,dirty:[...returned].sort(compareCodePointsV1)}};
   return{current:{pointer:{...current.pointer,generation:next.generation},manifest:next},built:true,published:false,ephemeral:true,plan:found,computed,phases:{...marks,...Object.fromEntries(Object.entries(computePhases??{}).map(([name,ms])=>["compute."+name,ms]))},carried:carried?.length??0};
  }
""",1),
])

D=G+"🕰️generations/🟨️.d.mts"
chain(D,[
("export interface GenerationRefreshOptionsV1{","export interface GenerationRefreshOptionsV1{ephemeral?:boolean;",1),
("export interface GenerationRefreshResultV1{","export interface GenerationRefreshResultV1{ephemeral?:boolean;",1),
])

C=G+"🕰️generations/🔐️custody/🟨️.mjs"
chain(C,[
(" return Object.freeze({hold,idle,list:live,resource});",
 """
 /** 🔭️ Whether a live claim rebuilds the whole workspace (structural, held or queued): a narrow reader that would queue behind it refreshes its own owners privately instead (see `../🔌️load`). */
 async function structuralBusy({signal}={}){return(await active(signal)).some(claim=>claim.structural);}

 return Object.freeze({hold,idle,structuralBusy,list:live,resource});""",1),
])
CD=G+"🕰️generations/🔐️custody/🟨️.d.mts"
chain(CD,[("idle(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal}):Promise<boolean>;","idle(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal}):Promise<boolean>;structuralBusy(options?:{signal?:AbortSignal}):Promise<boolean>;",1)])

L=G+"🕰️generations/🔌️load/🟨️.mjs"
chain(L,[
("  progressIdle:\"No whole read is running on this machine.\",\n",
 "  progressIdle:\"No whole read is running on this machine.\",\n  overlay:\"graph: a whole read of this machine holds the graph custody; this command refreshed {computed} projects for itself in {seconds} s on top of the sealed generation {generation} and published nothing, so it did not wait\",\n  overlayFailed:\"graph: the private refresh beside the whole read failed ({reason}); waiting for the custody as before\",\n",1),
("  progressIdle:\"Auf dieser Maschine läuft kein ganzes Lesen.\",\n",
 "  progressIdle:\"Auf dieser Maschine läuft kein ganzes Lesen.\",\n  overlay:\"Graph: ein ganzes Lesen dieser Maschine hält die Graph-Obhut; dieser Befehl hat {computed} Projekte für sich in {seconds} s auf der versiegelten Generation {generation} erneuert und nichts veröffentlicht, er hat also nicht gewartet\",\n  overlayFailed:\"Graph: die private Erneuerung neben dem ganzen Lesen ist fehlgeschlagen ({reason}); es wird wie bisher auf die Obhut gewartet\",\n",1),
("const attempt=async(builder,store,control,expectedHead,repair,localHead)=>{","const attempt=async(builder,store,control,expectedHead,repair,localHead,overlay=false)=>{",1),
("  const restored=builder&&host.exchange?await restoreRemoteGenerationV1(","  const restored=builder&&!overlay&&host.exchange?await restoreRemoteGenerationV1(",1),
("   else await settleRestoredClosuresV1(host,engine,restored,implementation,builder,prepare);","   else if(!overlay)await settleRestoredClosuresV1(host,engine,restored,implementation,builder,prepare);",1),
("wave:writer&&host.waveDone?info=>host.waveDone(info):undefined,","wave:writer&&!overlay&&host.waveDone?info=>host.waveDone(info):undefined,",1),
("   const base={identity,implementation,policy:writer?\"fresh\":policy,","   const base={identity,implementation,ephemeral:overlay,policy:writer?\"fresh\":policy,",1),
("  if(result.served===\"current\"){host.say(","  if(result.ephemeral)host.say(\"overlay\",{generation:result.current.manifest.generation-1,computed:result.computed,seconds:Math.round((Date.now()-started)/1000)});\n  else if(result.served===\"current\"){host.say(",1),
("  if(!sealedOnly&&host.sealed){","  if(!sealedOnly&&!result.ephemeral&&host.sealed){",1),
(" carried=first.planned;\n const request=first.pending?.dirty.length",
 """ if(roots!==undefined&&pin===undefined&&!sealedOnly&&policy==="fresh"&&host.withOverlay&&host.wholeBusy&&first.pending?.dirty.length&&first.pending.structural!==true&&await Promise.resolve(host.wholeBusy()).catch(()=>false)){
  try{
   const overlaid=await host.withOverlay((store,control)=>attempt(true,store,control,undefined,undefined,undefined,true));
   if(!overlaid.needsBuild){await host.release?.();return{graph:overlaid.graph,stats:overlaid.stats};}
  }catch(error){
   host.signal?.throwIfAborted();
   host.say("overlayFailed",{reason:String(error?.message??error).replace(/\\s+/gu," ").slice(0,200)});
  }
 }
 carried=first.planned;
 const request=first.pending?.dirty.length""",1),
])
LD=G+"🕰️generations/🔌️load/🟨️.d.mts"
chain(LD,[
("\"progressIdle\"","\"progressIdle\"|\"overlay\"|\"overlayFailed\"",1),
(" withBuilder<T>(body:"," /** 🫧️ Whether a live claim of the builder custody rebuilds the whole workspace; optional. */\n wholeBusy?():Promise<boolean>|boolean;\n /** 🫧️ Runs a private, unpublished refresh of the stale owners of a narrow read beside a whole read: no custody claim, the store as a shared cohort; optional. */\n withOverlay?<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;\n withBuilder<T>(body:",1),
])

S=G+"🕰️generations/🧬️schema/🔣️.json"
chain(S,[
("          \"progressIdle\": \"No whole read is running on this machine.\",\n",
 "          \"progressIdle\": \"No whole read is running on this machine.\",\n          \"overlay\": \"graph: a whole read of this machine holds the graph custody; this command refreshed {computed} projects for itself in {seconds} s on top of the sealed generation {generation} and published nothing, so it did not wait\",\n          \"overlayFailed\": \"graph: the private refresh beside the whole read failed ({reason}); waiting for the custody as before\",\n",1),
("          \"progressIdle\": \"Auf dieser Maschine läuft kein ganzes Lesen.\",\n",
 "          \"progressIdle\": \"Auf dieser Maschine läuft kein ganzes Lesen.\",\n          \"overlay\": \"Graph: ein ganzes Lesen dieser Maschine hält die Graph-Obhut; dieser Befehl hat {computed} Projekte für sich in {seconds} s auf der versiegelten Generation {generation} erneuert und nichts veröffentlicht, er hat also nicht gewartet\",\n          \"overlayFailed\": \"Graph: die private Erneuerung neben dem ganzen Lesen ist fehlgeschlagen ({reason}); es wird wie bisher auf die Obhut gewartet\",\n",1),
])

P=G+"🤝️prefetch/🟦️.ts"
chain(P,[
("/** 🧭️ What an attached reader looks at:",
 """/** 🫧️ Whether a live claim of the builder custody rebuilds the whole workspace: the narrow reader that would queue behind it refreshes its own stale owners privately instead. Any doubt or failure answers false and the reader queues as before. */
export async function wholeBuildBusyV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl}):Promise<boolean> {
  const{root,invocation,phaseControl}=input;
  try{
    requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
    return await withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>generationCustodyV1(root,store,control).structuralBusy({signal:invocation.control.signal}),{optionalCaches:false,shared:true});
  }catch(error){if(invocation.control.signal.aborted)throw error;return false;}
}


/** 🧭️ What an attached reader looks at:""",1),
])

K=G+"🛑️custody/🟦️.ts"
chain(K,[
("/** ♻️ Invalidates current graph authority while retaining",
 """/** 🫧️ Runs a private refresh beside a whole read: the generation namespace as a shared cohort (sweeps stay out while it runs) and no claim, so it never queues behind the custody and never publishes. */
export async function withGenerationOverlayV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{checkResourceLeaseControl(control);return body(store,control);},{optionalCaches:false,shared:true});
}


/** ♻️ Invalidates current graph authority while retaining""",1),
])

H=G+"🕰️host/🟦️.ts"
chain(H,[
("builderIdleV1,builderLookV1,","builderIdleV1,builderLookV1,wholeBuildBusyV1,",1),
("import {withGenerationCustodyV1,generationFilesV1} from","import {withGenerationCustodyV1,withGenerationOverlayV1,generationFilesV1} from",1),
("    builderIdle:(request:GenerationPrefetchRequestV1)=>builderIdleV1({root,invocation,phaseControl,request}),\n",
 """    builderIdle:(request:GenerationPrefetchRequestV1)=>builderIdleV1({root,invocation,phaseControl,request}),
    wholeBusy:()=>wholeBuildBusyV1({root,invocation,phaseControl}),
    withOverlay:async<T>(build:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>Promise<T>)=>{
      let releaseCapacity:(()=>void)|undefined;
      try{
        const prepared=await profilePhaseV1("graph.host.capacity",()=>prepareRebuild());releaseCapacity=prepared.release;
        await markBuildCapacityPhaseV1("rebuild");
        return await withOwnerRegistryCustodyV1<T>(phaseControl.signal,contain=>profilePhaseV1("graph.host.overlay",()=>withGenerationOverlayV1(root,invocation,phaseControl,(store,control)=>contain(()=>build(store,control)))));
      }catch(error){if(isClosedOwnerRegistryInvalidationV1(error))await release();throw error;}
      finally{try{releaseCapacity?.();}finally{await markBuildCapacityPhaseV1("plan");}}
    },
""",1),
])
flush()
