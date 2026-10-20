"""Group 2 of the conc-deps set, executed inside mkops.py after group 1: generation-scoped maintenance and image-fullness admission.

Every anchor is cut from the text that group 1 produces, so group 2 applies on top of group 1 (live or in the same run).
"""
for _key in TARGETS:
    _body = text(_key)
    for _row in OPERATIONS:
        if _row["target"] == _key:
            if _body.count(_row["old"]) != 1:
                raise SystemExit("group 1 anchor of %s is not unique while chaining" % _row["id"])
            _body = _body.replace(_row["old"], _row["new"])
    TEXT2[_key] = _body
GROUP = 2

S = "storage"

operation(
    "g2-01-core-imports", S,
    'import {concurrentCollectionV1,concurrentEventV1,concurrentLanguageV1,concurrentLeaseResourceV1,concurrentProtocolV1,concurrentTextV1,sealConcurrentPointerV1,type ConcurrentEventV1} from',
    'import {concurrentAdmissionV1,concurrentCollectionV1,concurrentEventV1,concurrentLanguageV1,concurrentLeaseResourceV1,concurrentPinNamesV1,concurrentProtocolV1,concurrentTextV1,sealConcurrentPointerV1,type ConcurrentEventV1} from',
    'import {concurrentAdmissionV1,concurrentCollectionV1')
operation(
    "g2-02-glue-imports", S,
    'import {acquireConcurrentLeaseV1,concurrentFreeFractionV1,concurrentLeasePortsV1,readConcurrentPointerV1,',
    'import {acquireConcurrentLeaseV1,acquireConcurrentPinsV1,concurrentFreeBytesV1,concurrentFreeFractionV1,concurrentGenerationReferencedV1,concurrentLeasePortsV1,readConcurrentPointerV1,',
    'import {acquireConcurrentLeaseV1,acquireConcurrentPinsV1,')
operation(
    "g2-03-frame-type", S,
    'build:ResourceLease;gate:ResourceLease|null;stop:AbortController',
    'build:ResourceLease;gate:ResourceLease|null;pins:ResourceLease[];built:Map<string,number>;stop:AbortController',
    'gate:ResourceLease|null;pins:ResourceLease[];built:Map<string,number>;')
operation(
    "g2-04-frame-fields", S,
    'build,gate:null,stop,live:true',
    'build,gate:null,pins:[],built:new Map(),stop,live:true',
    'build,gate:null,pins:[],built:new Map(),stop,live:true')
operation(
    "g2-05-frame-release", S,
    'for(const lease of [frame.gate,frame.build])if(lease)',
    'for(const lease of [frame.gate,...frame.pins.reverse(),frame.build])if(lease)',
    'for(const lease of [frame.gate,...frame.pins.reverse(),frame.build])if(lease)')

PRODUCED_OLD = between(S, '/** 🧬️ Is what one producer phase hands the next:', 'targets:readonly string[]}>;')
operation(
    "g2-06-produced-type", S, PRODUCED_OLD,
    '/** 🧬️ Is what one producer phase hands the next: the state it reached. */\ntype JavascriptDependencyProducedV1=Readonly<{state:"current"|"ready"|"published"}>;',
    'type JavascriptDependencyProducedV1=Readonly<{state:"current"|"ready"|"published"}>;')
operation("g2-07-produced-ready", S, 'Object.freeze({state:"ready",currents:[],targets:[]})', 'Object.freeze({state:"ready"})', 'Object.freeze({state:"ready"})')
operation("g2-08-produced-current", S, 'return Object.freeze({state:"current",currents:[],targets:[]});', 'return Object.freeze({state:"current"});', 'return Object.freeze({state:"current"});')
operation(
    "g2-09-pointer-bytes-and-produced", S,
    'next=sealConcurrentPointerV1(previous,scope,generation,switched);',
    'next=sealConcurrentPointerV1(previous,scope,generation,switched,frame.built.get(scope)??previous?.bytes??0);',
    'frame.built.get(scope)??previous?.bytes??0);')
PUBLISHED_OLD = between(S, 'produced=Object.freeze({state:"published",currents:', 'flatMap(row=>row.target===null?[]:[row.target])});}')
operation("g2-10-produced-published", S, PUBLISHED_OLD, 'produced=Object.freeze({state:"published"});}', 'produced=Object.freeze({state:"published"});}')

operation(
    "g2-11-pin-before-anything", S,
    'if(phase==="build")await collectJavascriptDependencyStoreGenerationsV1(workspace,store,control,Object.freeze({state:"published",currents:projection.plans.map(row=>row.plan.generation.slice(0,20)),targets:[...expected.values()].flatMap(row=>row.target===null?[]:[row.target])}));',
    'await pinJavascriptDependencyGenerationsV1(frame,selected.map(row=>row.plan.generation));',
    'await pinJavascriptDependencyGenerationsV1(frame,selected.map(row=>row.plan.generation));')
operation(
    "g2-12-admit-before-each-install", S,
    'for(const row of fresh){await fence();if(!anchors)throw Error("Actual installer environment required");',
    'for(const row of fresh){await admitJavascriptDependencyGenerationV1(workspace,store,control,row.scope);const freeBefore=concurrentFreeBytesV1(store.root);await fence();if(!anchors)throw Error("Actual installer environment required");',
    'await admitJavascriptDependencyGenerationV1(workspace,store,control,row.scope);')
operation(
    "g2-13-measure-built-bytes", S,
    'await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);await fence();}\n if(projection.owners.scopes.some(',
    'await requireJavascriptDependencyInstalledPathsIncrementalV1(workspace,context,control,row.plan.generation,checkpoint);frame.built.set(row.scope,Math.max(0,freeBefore-concurrentFreeBytesV1(store.root)));await fence();}\n if(projection.owners.scopes.some(',
    'frame.built.set(row.scope,Math.max(0,freeBefore-concurrentFreeBytesV1(store.root)));')

operation(
    "g2-14-report-numbers", S,
    'const name=event.event==="waiting"?event.resource==="bindings"?"waitingPublish":event.resource==="writer"?"waitingWriter":"waitingBuild":event.event;await writeOwnedCommandOutputV1(process.stdout,concurrentTextV1(language,name,{seconds:Math.round((event.elapsedMs??0)/1000),scope:event.scope??"",epoch:event.epoch??0,count:event.count??0})+"\\n",control);',
    'if(event.event==="refused")return;const name=event.event==="waiting"?event.resource==="bindings"?"waitingPublish":event.resource==="writer"?"waitingWriter":"waitingBuild":event.event;await writeOwnedCommandOutputV1(process.stdout,concurrentTextV1(language,name,{seconds:Math.round((event.elapsedMs??0)/1000),scope:event.scope??"",epoch:event.epoch??0,count:event.count??0,needed:Math.ceil((event.neededBytes??0)/1048576),free:Math.floor((event.freeBytes??0)/1048576)})+"\\n",control);',
    'if(event.event==="refused")return;')

operation(
    "g2-15-orchestrator-publish", S,
    'try{published=await runJavascriptDependencyStoreProducerV1(workspace,store,control,selection,"publish");}',
    'try{await runJavascriptDependencyStoreProducerV1(workspace,store,control,selection,"publish");}',
    'try{await runJavascriptDependencyStoreProducerV1(workspace,store,control,selection,"publish");}')
operation(
    "g2-16-orchestrator-locals", S,
    'let failed=false,failure:unknown,published:JavascriptDependencyProducedV1|undefined;',
    'let failed=false,failure:unknown;',
    'frame.gate=gate;let failed=false,failure:unknown;')
operation(
    "g2-17-orchestrator-collect", S,
    'await collectJavascriptDependencyStoreGenerationsV1(workspace,store,control,published!);',
    'await collectJavascriptDependencyGenerationsV1(workspace,store,control,{});',
    'await collectJavascriptDependencyGenerationsV1(workspace,store,control,{});')

COLLECT_FULL = between(S, '/** ♻️ Retires superseded generations through a tombstone rename,', 'Store dependency generation collection failed "+JSON.stringify({version:1,purpose:"current-dependency-generation-collection-failure",message:String((error as Error)?.message??error).slice(0,300)})+"\\n",control);}\n}\n')
COLLECT_NEW = r'''/** 📌️ Pins the generations a producer is about to create, resume or switch to, so no maintenance can remove them while they are under construction; the pins end with the writer frame. */
async function pinJavascriptDependencyGenerationsV1(frame:JavascriptDependencyStoreFrameV1,generations:readonly string[]):Promise<void>{
 const names=concurrentPinNamesV1(generations).filter(name=>!frame.pins.some(lease=>lease.resource.endsWith(":"+name)));
 if(!names.length)return;
 frame.pins.push(...await acquireConcurrentPinsV1(repositoryLeaseDirectoryV1(frame.workspace),frame.workspace,names,frame.control,frame.control.signal));
}
/** 🛂️ Refuses to start one more generation the image cannot hold: it first removes every unpinned superseded generation, then admits it or fails with a bilingual diagnostic before any install begins, leaving every published generation untouched. */
async function admitJavascriptDependencyGenerationV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,scope:string):Promise<void>{
 const report=javascriptDependencyConcurrencyReportV1(control),first=concurrentAdmissionV1({scope,pointers:readConcurrentPointersV1(store.root),freeBytes:concurrentFreeBytesV1(store.root)});
 if(first.admitted)return;
 await report(concurrentEventV1({event:"reclaiming",scope,neededBytes:first.requiredBytes,freeBytes:first.freeBytes}));
 await collectJavascriptDependencyGenerationsV1(workspace,store,control,{neededBytes:first.requiredBytes});
 const second=concurrentAdmissionV1({scope,pointers:readConcurrentPointersV1(store.root),freeBytes:concurrentFreeBytesV1(store.root)});
 if(second.admitted)return;
 await report(concurrentEventV1({event:"refused",scope,neededBytes:second.requiredBytes,freeBytes:second.freeBytes}));
 const values={needed:Math.ceil(second.requiredBytes/1048576),free:Math.floor(second.freeBytes/1048576)};
 throw Error(concurrentTextV1(concurrentLanguageV1(process.env),"admissionRefused",values),{cause:Object.freeze({version:1,purpose:"dependency-admission-refusal",scope,neededBytes:second.requiredBytes,freeBytes:second.freeBytes,en:concurrentTextV1("en","admissionRefused",values),de:concurrentTextV1("de","admissionRefused",values)})});
}
/** ♻️ Retires superseded generations without the build lease or any whole-image hold: a generation goes only when it is not current, no live binding resolves into it, nobody pins it and the retention policy no longer holds it; removal goes through a tombstone rename, so a crash never leaves a half-removed directory under a generation name, and a failure only costs disk. */
async function collectJavascriptDependencyGenerationsV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,options:Readonly<{neededBytes?:number}>):Promise<void>{
 const selected=resolve(workspace),checkpoint=()=>{requireRepositoryCacheStoreV1(store,selected,"dependencies");requireRepositoryCacheStoreReadCustodyV1(store,selected,"dependencies");checkResourceLeaseControl(control);},started=performance.now(),container=join(store.root,"j"),report=javascriptDependencyConcurrencyReportV1(control),directory=repositoryLeaseDirectoryV1(selected);
 checkpoint();
 try{
  const names=lstatSync(container,{throwIfNoEntry:false})?.isDirectory()?readdirSync(container):[],tick=async(count:number)=>{checkpoint();await control.publish({resource:"dependencies:collect:"+count,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkpoint();};
  const retire=async(name:string,tombstoned:boolean):Promise<boolean>=>{const pin=await tryConcurrentPinExclusiveV1(directory,selected,name,control,control.signal);if(!pin)return false;let failure:unknown,done=false;try{if(tombstoned||!concurrentGenerationReferencedV1(container,name)){const tombstone=join(container,".x"+name);if(!tombstoned)renameSync(join(container,name),tombstone);await removeJavascriptDependencyTreeV1(tombstone,tick);done=true;}}catch(error){failure=error;}try{pin.release();}catch(error){failure??=error;}if(failure)throw failure;return done;};
  for(const name of names.filter(name=>/^\.x[a-f0-9]{20}$/u.test(name)))await retire(name.slice(2),true);
  const pointers=readConcurrentPointersV1(store.root),current=new Set(pointers.map(pointer=>pointer.generation.slice(0,20))),rows=selectObsoleteJavascriptDependencyGenerationsV1(names,[],[]).filter(name=>!current.has(name)&&!concurrentGenerationReferencedV1(container,name)).map(name=>({name,modifiedAtMs:lstatSync(join(container,name)).mtimeMs})),decision=concurrentCollectionV1({rows,pointers,nowMs:Date.now(),freeFraction:concurrentFreeFractionV1(store.root),freeBytes:concurrentFreeBytesV1(store.root),neededBytes:options.neededBytes});
  let removed=0,pinned=0;
  for(const name of decision.collect){checkpoint();if(await retire(name,false))removed++;else pinned++;}
  if(removed)await report(concurrentEventV1({event:"collected",count:removed}));
  if(decision.keep.length+pinned)await report(concurrentEventV1({event:"kept",count:decision.keep.length+pinned}));
 }catch(error){if(control.signal.aborted)throw error;await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency generation collection failed "+JSON.stringify({version:1,purpose:"current-dependency-generation-collection-failure",message:String((error as Error)?.message??error).slice(0,300)})+"\n",control);}
}
/** 🧹️ Maintains the generations of the dependencies image under the shared image alone: no build lease, no bindings gate, no whole-image hold, so a running build, its readers and every other command are never blocked by it, and a build that needs room calls the same collector. */
export function maintainJavascriptDependencyGenerationsV1(workspace:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:Readonly<{neededBytes?:number}>={}):Promise<void>{requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,phaseControl,(store,control)=>collectJavascriptDependencyGenerationsV1(workspace,store,control,options));}
'''
operation("g2-18-generation-scoped-maintenance", S, COLLECT_FULL, COLLECT_NEW, 'async function collectJavascriptDependencyGenerationsV1(')

