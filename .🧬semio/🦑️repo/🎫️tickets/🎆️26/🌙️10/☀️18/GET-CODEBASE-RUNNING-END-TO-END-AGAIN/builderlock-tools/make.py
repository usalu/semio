#!/usr/bin/env python3
"""👯 Builds staged batch `multiwriter` (lane builderlock) from the CURRENT files of a root: anchored edits (each anchor exactly once) + the new modules in `src/`.
usage: python3 -I make.py <root> [--out DIR]   (default out: staged/multiwriter next to this file; never writes under <root>)
exit 1 = an anchor is missing or not unique (prints which)."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
GRAPH = LIB + "/🕸️graph"
G = GRAPH + "/🕰️generations"

E = {}
E[G + "/🟨️.mjs"] = [
 ('import{derivedUpsertsV1,', 'import{mergeIntoHeadV1}from"./🔀️merge/🟨️.mjs";\nimport{derivedUpsertsV1,'),
 ('casConflicts:0,', 'casConflicts:0,merged:0,mergeConflicts:0,'),
 ('async function build({identity,implementation,membership,compute,signal,progress,hint,only,observedAtMs=now(),coverage}){',
  'async function build({identity,implementation,membership,compute,signal,progress,hint,only,observedAtMs=now(),coverage,claim}){'),
 ('names=cold?[]:[...found.dirty].sort(compareCodePointsV1);\n',
  'names=cold?[]:[...found.dirty].sort(compareCodePointsV1);\n'
  '  const want=claim&&await claim.extend({owners:[...names,...found.removed],structural:cold||found.envStale||found.added.length>0,signal});\n'
  '  if(want&&!want.held)return{current,built:false,published:false,computed:0,plan:found,needsBuild:true,pending:{generation:current?.pointer.generation??-1,roots:{},...want.request}};\n'),
 ('  const outcome=await publish({manifestDigest,expectedGeneration,verified,signal});\n'
  '  if(!outcome.published)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};\n'
  '  indexCache.set(indexStored.digest,index);\n'
  '  await retire({signal}).catch(error=>{if(signal?.aborted)throw error;});\n'
  '  return{current:{pointer:outcome.pointer,manifest:next},built:true,published:true,plan:found,computed:result.owners.length,index:{rows:index.rows.length,owners:index.owners.filter(name=>!isDerivedOwnerV1(name)).length,expandedBytes:packedIndex.expandedBytes,packedBytes:indexStored.bytes}};\n',
  '  let outcome=await publish({manifestDigest,expectedGeneration,verified,signal}),done=outcome.published?{next,index,packed:packedIndex,kept:indexStored}:undefined;\n'
  '  if(!done&&!cold){index=undefined;const merged=await mergeIntoHeadV1({read,readIndex,writeManifest,publish,patch:patchFactIndexV1,pack:value=>packFactIndexV1(value,P.maximumIndexBytes),keep:bytes=>keep(P.indexes,bytes,".facts.br"),key:rowKeyV1,stats},{base:current,mine:next,patch:{drop:[...dropped],upserts,append},verified,signal});if(merged.outcome){outcome=merged.outcome;done=merged;}}\n'
  '  if(!done)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};\n'
  '  indexCache.set(done.kept.digest,done.index);\n'
  '  if(await(claim?claim.alone({signal}).catch(()=>false):true))await retire({signal}).catch(error=>{if(signal?.aborted)throw error;});\n'
  '  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,published:true,plan:found,computed:result.owners.length,...(done.theirs?{merged:done.theirs.length}:{}),index:{rows:done.index.rows.length,owners:done.index.owners.filter(name=>!isDerivedOwnerV1(name)).length,expandedBytes:done.packed.expandedBytes,packedBytes:done.kept.bytes}};\n'),
 ('if(verdict.kind==="delta")pending={generation:', 'if(verdict.kind==="delta")pending={structural:verdict.envStale||verdict.added.length>0||verdict.removed.length>0,generation:'),
 ('const body=()=>build({identity,implementation,membership,compute,signal,progress,hint,only:scope,observedAtMs:started,coverage});',
  'const body=claim=>build({identity,implementation,membership,compute,signal,progress,hint,only:scope,observedAtMs:started,coverage,claim});'),
 ('/** 🔁️ Brings the caller to a generation that holds. `current` serves the newest generation of this identity without verification or waiting; `fresh` verifies (only the closure `only` when given), serves a verified generation without any lease and builds under the builder lease otherwise, where a holder that finds the work done by a peer returns that generation untouched. */',
  '/** 🔁️ Brings the caller to a generation that holds. `current` serves the newest generation of this identity without verification or waiting; `fresh` verifies (only the closure `only` when given), serves a verified generation without any lease and builds otherwise, inside `builder` (which hands the build its slice claim; see `./🔐️custody`) or the builder lease, where a holder that finds the work done by a peer returns that generation untouched and a build whose swap a peer won merges onto the peer\'s generation (see `./🔀️merge`). */'),
]
E[G + "/🟨️.d.mts"] = [
 ('export interface GenerationPendingV1{readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>}',
  'export interface GenerationPendingV1{readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>;readonly structural:boolean}'),
 ('builder?:<T>(body:()=>Promise<T>,pending?:GenerationPendingV1)=>Promise<T>;',
  'builder?:<T>(body:(claim?:import("./🔐️custody/🟨️.mjs").SliceClaimV1)=>Promise<T>,pending?:GenerationPendingV1)=>Promise<T>;'),
 ('export interface GenerationRefreshResultV1{current:GenerationSealedV1;served:"current"|"verified"|"covered"|"built";built:boolean;published:boolean;computed:number;plan?:GenerationPlanV1;index?:GenerationIndexSizeV1}',
  'export interface GenerationRefreshResultV1{current:GenerationSealedV1;served:"current"|"verified"|"covered"|"built";built:boolean;published:boolean;computed:number;plan?:GenerationPlanV1;index?:GenerationIndexSizeV1;needsBuild?:boolean;pending?:GenerationPendingV1;merged?:number}'),
 ('coverage?:import("./🔭️coverage/🟨️.mjs").CoverageV1}):Promise<{current:GenerationSealedV1;built:boolean;published:boolean;computed:number;plan:GenerationPlanV1;index?:GenerationIndexSizeV1}>;',
  'coverage?:import("./🔭️coverage/🟨️.mjs").CoverageV1;claim?:import("./🔐️custody/🟨️.mjs").SliceClaimV1}):Promise<{current:GenerationSealedV1;built:boolean;published:boolean;computed:number;plan:GenerationPlanV1;index?:GenerationIndexSizeV1;needsBuild?:boolean;pending?:GenerationPendingV1;merged?:number}>;'),
 ('casConflicts:number;', 'casConflicts:number;merged:number;mergeConflicts:number;'),
]
MERGED_EN = 'graph: generation {generation} published, {computed} projects refreshed in {seconds} s and merged onto the generation a peer published meanwhile ({theirs} projects of peers kept)'
MERGED_DE = 'Graph: Generation {generation} veröffentlicht, {computed} Projekte in {seconds} s erneuert und auf die Generation eines anderen Prozesses zusammengeführt ({theirs} Projekte anderer Prozesse übernommen)'
E[G + "/🧬️schema/🔣️.json"] = [
 ('     "changeChanged": "changed"\n', '     "changeChanged": "changed",\n     "merged": ' + json.dumps(MERGED_EN, ensure_ascii=False) + '\n'),
 ('     "changeChanged": "geändert"\n', '     "changeChanged": "geändert",\n     "merged": ' + json.dumps(MERGED_DE, ensure_ascii=False) + '\n'),
]
E[G + "/🔌️load/🟨️.mjs"] = [
 ('import{readPinnedGenerationV1}from"../🧷️pin/🟨️.mjs";', 'import{readPinnedGenerationV1}from"../🧷️pin/🟨️.mjs";\nimport{CUSTODY_POLICY_V1}from"../🔐️custody/🟨️.mjs";'),
 ('  changeChanged:"changed",\n', '  changeChanged:"changed",\n  merged:' + json.dumps(MERGED_EN, ensure_ascii=False) + ',\n'),
 ('  changeChanged:"geändert",\n', '  changeChanged:"geändert",\n  merged:' + json.dumps(MERGED_DE, ensure_ascii=False) + ',\n'),
 ('builder:builder?body=>body():needsBuild};', 'builder:builder?body=>body(builder===true?undefined:builder):needsBuild};'),
 ('\n  engine.dropIndexes();\n', '\n  engine.dropIndexes();if(typeof builder==="object")await builder.release();\n'),
 ('else if(result.served==="built"&&result.published){host.say("published",{generation,computed:result.computed,seconds:Math.round((Date.now()-started)/1000)});',
  'else if(result.served==="built"&&result.published){host.say(result.merged===undefined?"published":"merged",{generation,computed:result.computed,seconds:Math.round((Date.now()-started)/1000),theirs:result.merged??0});'),
 ('const request=first.pending?.dirty.length?{generation:first.pending.generation,dirty:first.pending.dirty,roots:first.pending.roots,implementation}:undefined;',
  'const request=first.pending?.dirty.length?{generation:first.pending.generation,dirty:first.pending.dirty,roots:first.pending.roots,structural:first.pending.structural,implementation}:undefined;'),
 (' let built;try{built=await host.withBuilder((store,control)=>{held=true;return attempt(true,store,control);});}finally{clearInterval(notice);}',
  ' let built,want=first.pending;try{for(let round=1;;round++){built=await host.withBuilder((store,control,claim)=>{held=true;return attempt(claim??true,store,control);},want);if(!built.needsBuild)break;held=false;want={...built.pending,structural:built.pending.structural||round>=CUSTODY_POLICY_V1.widenRounds};}}finally{clearInterval(notice);}'),
]
E[G + "/🔌️load/🟨️.d.mts"] = [
 ('withBuilder<T>(body:(store:unknown,control:unknown)=>Promise<T>):Promise<T>;',
  'withBuilder<T>(body:(store:unknown,control:unknown,claim?:import("../🔐️custody/🟨️.mjs").SliceClaimV1)=>Promise<T>,request?:import("../🟨️.mjs").GenerationPendingV1):Promise<T>;'),
 ('prefetch?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;implementation:string}):Promise<void>;',
  'prefetch?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;structural:boolean;implementation:string}):Promise<void>;'),
 ('"changeChanged",string>>>>;', '"changeChanged"|"merged",string>>>>;'),
]
E[G + "/🧩️claims/🟨️.mjs"] = [
 ('/** 💤️ Whether a command may build at once: its builder custody `builder` is free right now and no live running claim of this plan covers a wanted owner.',
  '/** 💤️ Whether a command may build at once: `builderFree()` says its builder custody would be held at once and no live running claim of this plan covers a wanted owner.'),
 (' async function idle({base,implementation,wanted,builder,signal}){\n  const probe=await lease.tryHold(builder);if(!probe)return false;probe.release();\n',
  ' async function idle({base,implementation,wanted,builderFree,signal}){\n  if(!await builderFree())return false;\n'),
 ('export function createClaimRegistryV1({root,fs,lease,now=()=>Date.now(),sleep=defaultSleep,token=', 'export function createClaimRegistryV1({root,fs,lease,now=()=>Date.now(),token='),
 ('`hold` queues for it, `wait` returns when no exclusive holder is left; a lease is released with `release()`', '`hold` queues for it, `wait({signal, timeoutMs})` answers true when no exclusive holder is left and false after its timeout; a lease is released with `release()`'),
 (' /** ⏳️ Returns when one of the leases is free or after the poll time, whichever comes first. */\n async function waitAny(resources,signal){\n  const controller=new AbortController(),onAbort=()=>controller.abort(signal.reason);\n  signal?.addEventListener("abort",onAbort,{once:true});\n  const quiet=promise=>promise.catch(error=>{if(!controller.signal.aborted)throw error;});\n  try{await Promise.race([...resources.map(name=>quiet(lease.wait(name,{signal:controller.signal}))),quiet(sleep(P.pollMilliseconds,controller.signal))]);}\n  finally{controller.abort();signal?.removeEventListener("abort",onAbort);}\n }\n',
  ' const waitAny=(resources,signal)=>waitAnyLeaseV1(lease,resources,{signal,timeoutMs:P.pollMilliseconds});\n'),
 ('/**\n * 🔗️ The lease port of the registry', '/** ⏳️ Answers true as soon as one of `resources` has no exclusive holder and false after `timeoutMs`. Every wait goes through the lease port with the caller\'s own signal and ends by its own timeout, never by a derived abort signal: the lease kernel admits only the original signal of the control it serves, and a wait that lost the race ends by itself within the timeout. */\nexport async function waitAnyLeaseV1(lease,resources,{signal,timeoutMs}){\n aborted(signal);if(!resources.length)throw fail("Leases to wait for required");\n const waits=resources.map(name=>lease.wait(name,{signal,timeoutMs}));\n for(const wait of waits)wait.catch(()=>{});\n return Promise.race(waits);\n}\n\n/**\n * 🔗️ The lease port of the registry'),
 ('`hold` queues for it, `wait` returns when no exclusive holder is left. Shared acquisitions are cohort tickets', '`hold` queues for it, `wait` answers true when no exclusive holder is left and false after its `timeoutMs`; with a control every acquisition carries the control\'s own signal, the only one the kernel admits. Shared acquisitions are cohort tickets'),
 ('signal:signal??control?.signal??new AbortController().signal', 'signal:control?control.signal:signal??new AbortController().signal'),
 ('  async wait(resource,options){(await request(resource,"shared",options?.signal)).release();},', '  async wait(resource,options){try{(await request(resource,"shared",options?.signal,options?.timeoutMs)).release();return true;}catch(error){if(options?.timeoutMs!==undefined&&/timed out/iu.test(String(error?.message)))return false;throw error;}},'),
 (' async function wait(name,{signal}={}){\n  const path=pathOf(name);\n  for(let delay=2;;delay=Math.min(delay*2,50)){aborted(signal);const held=await holderOf(path);if(!held||!alive(held.pid))return;await sleep(delay,signal);}\n }',
  ' async function wait(name,{signal,timeoutMs}={}){\n  const path=pathOf(name),end=timeoutMs===undefined?Infinity:Date.now()+timeoutMs;\n  for(let delay=2;;delay=Math.min(delay*2,50)){aborted(signal);const held=await holderOf(path);if(!held||!alive(held.pid))return true;if(Date.now()>=end)return false;await sleep(Math.min(delay,Math.max(1,end-Date.now())),signal);}\n }'),
]
E[G + "/🧩️claims/🟨️.d.mts"] = [
 ('idle(options:{base:number;implementation:string;wanted:readonly string[];builder:string;signal?:AbortSignal}):Promise<boolean>;',
  'idle(options:{base:number;implementation:string;wanted:readonly string[];builderFree:()=>Promise<boolean>;signal?:AbortSignal}):Promise<boolean>;'),
 ('wait(resource:string,options?:{signal?:AbortSignal}):Promise<void>}', 'wait(resource:string,options?:{signal?:AbortSignal;timeoutMs?:number}):Promise<boolean>}\n/** ⏳️ Answers true as soon as one of the leases has no exclusive holder, false after `timeoutMs`; every wait carries the caller\'s own signal. */\nexport function waitAnyLeaseV1(lease:ClaimLeasePortV1,resources:readonly string[],options:{signal?:AbortSignal;timeoutMs:number}):Promise<boolean>;'),
 ('now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;token?:()=>string}):ClaimRegistryV1;', 'now?:()=>number;token?:()=>string}):ClaimRegistryV1;'),
]
E[G + "/🧩️claims/🧪️tests/💤️idle/🟦️.ts"] = [
 ('const BUILDER="graph-builder:idle-law";', 'const BUILDER="graph-builder:idle-law";\nconst freeOf=(port:{tryHold(resource:string):Promise<{release():void}|undefined>})=>async()=>{const probe=await port.tryHold(BUILDER);probe?.release();return probe!==undefined;};'),
 ('wanted:fixture.wanted,builder:BUILDER})', 'wanted:fixture.wanted,builderFree:freeOf(port)})'),
 ('wanted:["a"],builder:BUILDER})', 'wanted:["a"],builderFree:freeOf(port)})'),
]
E[GRAPH + "/🟦️.ts"] = [
 ('import{CLAIMS_POLICY_V1,createClaimRegistryV1,createQueuedClaimLeasePortV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1}from"./🕰️generations/🧩️claims/🟨️.mjs";',
  'import{CLAIMS_POLICY_V1,createClaimRegistryV1,createQueuedClaimLeasePortV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1}from"./🕰️generations/🧩️claims/🟨️.mjs";\nimport{createSliceCustodyV1,renderCustodyMessageV1,type SliceClaimV1,type SliceCustodyV1}from"./🕰️generations/🔐️custody/🟨️.mjs";'),
 ('  return scope?withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run,scope===generationScopeV1?{optionalCaches:false,shared:true}:undefined):withRepositoryBuildGraphFactStoreV1(root,invocation,phaseControl,run);\n}\n',
  '  return scope?withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run):withRepositoryBuildGraphFactStoreV1(root,invocation,phaseControl,run);\n}\n\n'
  '/** 🔐️ The slice custody of the generation builders of one graph image (see `./🕰️generations/🔐️custody`): claim documents beside the generations, leases in the repository lease directory. */\n'
  'function generationCustodyV1(root:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl):SliceCustodyV1{\n'
  '  return createSliceCustodyV1({root:join(store.root,"generations"),fs:generationFilesV1(store) as never,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,directory:join(root,".cache/b/l/m/data"),control})});\n'
  '}\n\n'
  '/** 🧑‍🤝‍🧑️ Builds inside the shared cohort of the generation namespace (sweeps stay out) under a slice claim on the owners `request` writes (every owner when it is undefined or structural): refreshes of disjoint projects build at the same time, overlapping ones one after the other in ticket order. */\n'
  'async function withGenerationCustodyV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,request:{readonly dirty:readonly string[];readonly structural:boolean}|undefined,body:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>Promise<T>):Promise<T> {\n'
  '  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();\n'
  '  return withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{\n'
  '    const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,claim=await generationCustodyV1(root,store,control).hold({owners:request?.dirty??[],structural:request===undefined||request.structural,signal:control.signal,say:(key,fields)=>{process.stderr.write(renderCustodyMessageV1(key,fields,language)+"\\n");}});\n'
  '    try{requireCurrentGraphModuleV1();checkResourceLeaseControl(control);return await body(store,control,claim);}finally{await claim.release();}\n'
  '  },{optionalCaches:false,shared:true});\n'
  '}\n'),
 ('/** 🔌️ The identity of the one builder custody of the generation store: every builder of the all-workspace graph queues on it, shared readers never do. */',
  '/** 🔌️ The generation namespace of the graph image: its writer lease is taken as a shared cohort by every builder, prefetch and idle probe (sweeps take it exclusively), the slice custody decides who builds. */'),
 ('    withBuilder:async<T>(build:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>Promise<T>)=>{\n'
  '      const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>{await markBuildCapacityPhaseV1("rebuild");return build(store,control);};\n',
  '    withBuilder:async<T>(build:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>Promise<T>,request?:{readonly dirty:readonly string[];readonly structural:boolean})=>{\n'
  '      const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>{await markBuildCapacityPhaseV1("rebuild");return build(store,control,claim);};\n'),
 ('      const result=await withBuildGraphStateV1(root,invocation,phaseControl,(store,control)=>{image=store.root;return body(store,control);},generationScopeV1);\n',
  '      const result=await withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;return body(store,control,claim);});\n'),
 ('type GenerationPrefetchRequestV1={readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>;readonly implementation:string};',
  'type GenerationPrefetchRequestV1={readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>;readonly structural:boolean;readonly implementation:string};'),
 ('/** 💤️ Whether this command may build at once: the builder custody is free and no live peer claim covers a stale owner of its plan,',
  '/** 💤️ Whether this command may build at once: a slice claim on its stale owners would be held at once and no live peer claim covers a stale owner of its plan,'),
 ('builder:buildGraphPublicationWriterResourceV1(store.root,generationScopeV1),signal:invocation.control.signal});',
  'builderFree:()=>generationCustodyV1(root,store,control).idle({owners:request.dirty,structural:request.structural,signal:invocation.control.signal}),signal:invocation.control.signal});'),
]
E[GRAPH + "/🧪️tests/🔼️upgrade-before-custody/🟦️.ts"] = [
 ('custody=source.indexOf("withBuildGraphStateV1(root,invocation,phaseControl,(store,control)=>{image=store.root;return body(store,control);},generationScopeV1)",builder);',
  'custody=source.indexOf("withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;return body(store,control,claim);})",builder);'),
 ('const body=source.slice(builder,source.indexOf("generationScopeV1)",builder));',
  'const body=source.slice(builder,source.indexOf("withGenerationCustodyV1(",builder));'),
 ("expect(body).toContain('const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>{await markBuildCapacityPhaseV1(\"rebuild\");return build(store,control);};');",
  "expect(body).toContain('const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>{await markBuildCapacityPhaseV1(\"rebuild\");return build(store,control,claim);};');"),
]
E[G + "/🔌️load/🧪️tests/🚧️withheld-dependency/🔣️.json"] = [
 ('      "end": "/** 🔌️ The identity of the one builder custody",\n      "before": "configs[name]??=file",', '      "end": "/** 🔌️ The generation namespace of the graph image",\n      "before": "configs[name]??=file",'),
 ('      "end": "/** 🔌️ The identity of the one builder custody",\n      "before": "withholdDanglingTestProjectsV1({",', '      "end": "/** 🔌️ The generation namespace of the graph image",\n      "before": "withholdDanglingTestProjectsV1({",'),
]
E[GRAPH + "/🧪️tests/🕰️generation-protocol/🔣️.json"] = [
 ('   "name": "the builder queues on the one generation scope",\n   "function": "generationHostV1",\n   "text": "withBuildGraphStateV1(root,invocation,phaseControl,(store,control)=>{image=store.root;return body(store,control);},generationScopeV1)"',
  '   "name": "every builder holds a slice claim under the one generation scope",\n   "function": "generationHostV1",\n   "text": "withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;return body(store,control,claim);})"'),
]
E[GRAPH + "/🧪️tests/🕰️generation-protocol/🟦️.ts"] = [
 ('Shared readers never take a writer, the builder queues on one generation scope,', 'Shared readers never take a writer, every builder holds a slice claim under the one generation scope,'),
]

def sha(data): return hashlib.sha256(data).hexdigest()

def main():
    root = os.path.abspath(sys.argv[1])
    out = os.path.abspath(sys.argv[sys.argv.index("--out") + 1]) if "--out" in sys.argv else os.path.join(HERE, "staged", "multiwriter")
    if out.startswith(root + os.sep) and not out.startswith(os.path.join(root, ".🧬semio")): sys.exit("refusing to write inside the root")
    bad = 0; rows = {}; payload = {}
    for rel, edits in E.items():
        data = open(os.path.join(root, rel), encoding="utf8").read(); base = data
        for old, new in edits:
            n = data.count(old)
            if n != 1: print(f"ANCHOR x{n}: {rel[-60:]} :: {old[:90]!r}"); bad += 1; continue
            data = data.replace(old, new)
        payload[rel] = (base.encode("utf8"), data.encode("utf8"))
    src = os.path.join(HERE, "src")
    for dirpath, _dirs, files in os.walk(src):
        for name in files:
            full = os.path.join(dirpath, name); rel = G + "/" + os.path.relpath(full, src).replace(os.sep, "/")
            if os.path.exists(os.path.join(root, rel)): print("NEW FILE EXISTS:", rel); bad += 1
            payload[rel] = (None, open(full, "rb").read())
    if bad: sys.exit(1)
    shutil.rmtree(os.path.join(out, "files"), ignore_errors=True); shutil.rmtree(os.path.join(out, "base"), ignore_errors=True)
    os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    for at, (rel, (base, new)) in enumerate(sorted(payload.items())):
        name = f"{at:02d}-" + rel.rsplit("/", 2)[-2].encode("ascii", "ignore").decode() + "-" + sha(rel.encode())[:8]
        open(os.path.join(out, "files", name), "wb").write(new)
        if base is not None: open(os.path.join(out, "base", name), "wb").write(base)
        rows[rel] = {"file": name, "base": sha(base) if base is not None else None, "new": sha(new)}
    json.dump(rows, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    head = template.split("\n", 2)
    head[1] = '"""👯 Staged batch `multiwriter` (lane builderlock): disjoint graph refreshes build at the same time (slice custody) and a lost pointer swap merges onto the peer generation.'
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write("\n".join(head))
    shutil.copyfile(os.path.join(HERE, "verify.sh"), os.path.join(out, "verify.sh"))
    gen = payload[G + "/🟨️.mjs"][1]
    print(f"OK {len(rows)} files -> {out}; generations/🟨️.mjs {len(gen)} B (cap 65536)")
main()
