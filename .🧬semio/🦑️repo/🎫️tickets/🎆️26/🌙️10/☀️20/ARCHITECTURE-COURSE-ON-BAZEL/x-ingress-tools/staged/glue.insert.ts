const relativeWriteCeilingBytes=65536,replayEntries:readonly string[]=Object.freeze(["in-process-replay"]),replayImplementation:Awaited<ReturnType<typeof captureJavascriptDependencyGenerationImplementationV1>>=Object.freeze({entries:replayEntries,loadedAtMs:0,files:Object.freeze([])}),sharedRetireGraceMilliseconds=600000;
/** 📝️ Writes immutable evidence through the bounded relative-create API in chunks within its write ceiling and removes a partially written file instead of leaving it behind. */
export async function writeJavascriptDependencyGenerationFilesV1(storeRoot:string,files:readonly Readonly<{name:string;bytes:Uint8Array}>[],checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<void>{
 await withArtifactRelativeDirectoryV1(storeRoot,checkpoint,async base=>{
  if(await base.entry(protocol.directory)===null){await base.withNewDirectory(protocol.directory,async child=>{await child.sync();});await base.sync();}
  await base.withDirectory(protocol.directory,async directory=>{
   for(const file of files){
    const existing=await directory.entry(file.name);
    if(existing){if(existing.kind!=="file"||existing.size!==BigInt(file.bytes.length))throw Error("Existing immutable generation file changed");continue;}
    try{await directory.withNewFile(file.name,async handle=>{let offset=0;while(offset<file.bytes.length){checkpoint();offset+=handle.write(file.bytes,offset,Math.min(relativeWriteCeilingBytes,file.bytes.length-offset),offset);await yieldContinuation();}handle.sync();});}
    catch(error){try{unlinkSync(join(storeRoot,protocol.directory,file.name));}catch{}throw error;}
    await directory.sync();
   }
  });
 });
}
/** 🧫️ Replays a completed walk's own recorded observations against the live tree under the caller's custody; nothing is persisted, so the implementation closure and its settle rule do not apply. */
export async function replayJavascriptDependencyGenerationWalkV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined=walk.observed?.(),subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<GenerationReplayResultV1>{
 if(!observed)throw Error("Retained workspace topology required");
 const generation=controlOf(context,control,"shared",checkpoint),replaySubject:JavascriptDependencyGenerationSubjectV1=Object.freeze({...subject,entries:replayEntries}),captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,{root:context.root},generation,walk,observed,startedAtMs,context.invocation,replaySubject,replayImplementation);
 if(captured.status==="deferred")return Object.freeze({status:"deferred",reason:captured.reason,...(captured.path===undefined?{}:{path:captured.path})});
 const files=new Map(captured.files.map(file=>[file.name,file.bytes])),proof=await verifyGenerationReceiptV1(portsOf(resolve(workspace),context,control,generation,name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},captured.receipt,replaySubject));
 if(proof.status!=="match")return Object.freeze({status:"mismatch",class:proof.class,...(proof.path===undefined?{}:{path:proof.path})});
 return proof.sha256===walk.sha256?Object.freeze({status:"match",sha256:proof.sha256}):Object.freeze({status:"mismatch",class:"seal"});
}
/** 🤝️ Publishes one receipt under the caller's SHARED Store custody, serialised only against other receipt writers, and reports busy instead of waiting for anybody. */
export async function publishJavascriptDependencyGenerationReceiptSharedV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number):Promise<GenerationPublishOutcomeV1>{
 const root=resolve(workspace);
 return runGenerationWriterV1({acquire:()=>acquireResourceLease({directory:repositoryLeaseDirectoryV1(root),resource:"javascript-dependency-generation-writer:"+root,mode:"exclusive",signal:control.signal,control,timeoutMs:0}),aborted:()=>control.signal.aborted,body:async()=>{await store.recheck();checkpoint();const outcome=await publishJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,walk,startedAtMs,walk.observed?.(),javascriptDependencyGenerationSubjectV1(),sharedRetireGraceMilliseconds);await store.recheck();return outcome;}});
}
/** 📣️ Reports the publication outcomes the receipt writer itself stays silent about. */
export const reportJavascriptDependencyGenerationPublishV1=(scope:string)=>(outcome:GenerationPublishOutcomeV1|"error",detail?:string):void=>{if(outcome==="busy"||outcome==="error")console.error("[DATA] Store dependency generation receipt "+outcome+" "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-"+outcome,scope,...(detail===undefined?{}:{message:detail})}));};
export {confirmGenerationWalkV1,publishGenerationBestEffortV1};

