import {createHash} from "node:crypto";
import {lstatSync,readFileSync,readdirSync,statSync,unlinkSync} from "node:fs";
import {homedir} from "node:os";
import {join,resolve} from "node:path";
import {acquireResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withArtifactRelativeDirectoryV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import type {VouchV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryLeaseDirectoryV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {readBunWorkspaceListingProjectionV1,type BunWorkspaceObservedOwnershipV1} from "../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import { javascriptDependencyReadContextV1, javascriptDependencySourceOwnerRuntimeV1, observeJavascriptDependencyBindingsReadV1, readJavascriptDependencyBindingReceiptReadV1, type JavascriptDependencyReadContextV1 } from "../../💾️storage/📥️read-context/🟦️.ts";
import {javascriptInstallSchemaV1 as installSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";
import {concurrentProtocolV1} from "../../🚦️concurrent/🟦️.ts";
import {requireCurrentFormatStampsV1,staleImplementationOfV1} from "../../../../../🏗️build/🧷️format-stamps/🟨️.mjs";
import {captureGenerationReceiptV1,confirmGenerationWalkV1,describeGenerationMismatchV1,generationLocationV1,generationProtocolV1 as protocol,generationSha256V1,guardGenerationReplayV1,generationWalkOriginV1,parseGenerationReceiptV1,publishGenerationBestEffortV1,runGenerationWriterV1,verifyGenerationReceiptV1,type GenerationControlV1,type GenerationPublishOutcomeV1,type GenerationReplayResultV1,type GenerationProducerFileV1,type GenerationProducerV1,type GenerationProducerVariableV1,type GenerationReceiptV1,type GenerationRuntimeV1,type GenerationVerifyResultV1} from "../🟦️.ts";
import { runtimeIdentityV1 } from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";

const compareBytes=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const scopeKey=(scope:string)=>generationSha256V1(scope).slice(0,16);
const evidenceName=/^[a-f0-9]{64}\.(?:frontier|execution)\.ndjson$/u,evidenceSweep=/^[a-f0-9]{64}\.[a-z]+\.ndjson$/u;

/** 🧬️ Is a completed complete walk plus the topology it retained; only such a walk may be published. The receipt reads the walk through this consumer-owned shape, so it never imports the Source-owner reader that produces it. */
export type JavascriptDependencyGenerationWalkV1=Readonly<{
 required:readonly string[];sha256:string;
 projection:Readonly<{recipeSha256:string;owners:Readonly<{sha256:string}>;plans:readonly Readonly<{scope:string;plan:Readonly<{directory:string;bindings:readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[];sources:readonly Readonly<{path:string;kind:"file"|"directory"|"missing";bytes:Buffer;dev:string;ino:string;mode:number}>[]}>}>[]}>;
 receipts:readonly Readonly<{scope:string;generation:string;fingerprint:string}>[];
 evidence:Readonly<{scope:string;bindings:readonly (readonly [string,string])[];execution:Readonly<{scopes:readonly Readonly<{scope:string;sha256:string}>[];runtimeSha256:string;observations:readonly Readonly<{path:string;stamp:string|null;anchor:string|null;kind:string;target:string|null;full:boolean}>[]}>}>;
 observed?:()=>BunWorkspaceObservedOwnershipV1;
}>;
/** 🔍️ Is either the exact result of a complete walk, a typed mismatch, or the absence of any receipt. */
export type JavascriptDependencyGenerationVerifyV1=GenerationVerifyResultV1|Readonly<{status:"absent"}>;

/** 🧭️ Names what a receipt is sealed against besides the tree: the ambient locations the installer environment could read. */
export interface JavascriptDependencyGenerationSubjectV1{readonly ambient:JavascriptDependencyGenerationAmbientV1}
/** ⚙️ Reads the live tool identity from the same source the provider walk fingerprints. */
export function javascriptDependencyGenerationRuntimeV1():GenerationRuntimeV1{
 return Object.freeze({manager:String(installSchema.$defs.ManagerProtocol.const.runtime),revision:String(runtimeIdentityV1().revision),node:process.versions.node,platform:process.platform,arch:process.arch,sha256:javascriptDependencySourceOwnerRuntimeV1()});
}
/** 🌐️ Names where the process finds its home and configuration directories; every field is injectable so laws can run in isolation. */
export interface JavascriptDependencyGenerationAmbientV1{readonly env:Readonly<Record<string,string|undefined>>;readonly home:string;readonly config:string}
/** 🔭️ Reads the live ambient locations. */
export const javascriptDependencyGenerationAmbientV1=():JavascriptDependencyGenerationAmbientV1=>Object.freeze({env:process.env,home:homedir(),config:process.env.XDG_CONFIG_HOME||join(homedir(),protocol.producer.ambientConfigFallback)});
/** 🎯️ Names the live subject: the process ambient. */
export const javascriptDependencyGenerationSubjectV1=():JavascriptDependencyGenerationSubjectV1=>Object.freeze({ambient:javascriptDependencyGenerationAmbientV1()});
const configFile=(path:string,workspace:string,storeRoot:string):GenerationProducerFileV1=>{
 const location=generationLocationV1(path,workspace,storeRoot),other=Object.freeze({path:location,kind:"other"as const,bytes:0,sha256:null});
 try{
  const facts=statSync(path,{throwIfNoEntry:false});
  if(!facts)return Object.freeze({path:location,kind:"missing",bytes:0,sha256:null});
  if(!facts.isFile()||facts.size>protocol.producer.maximumFileBytes)return other;
  const bytes=readFileSync(path);return Object.freeze({path:location,kind:"file",bytes:bytes.length,sha256:generationSha256V1(bytes)});
 }catch(error){return (error as NodeJS.ErrnoException).code==="ENOTDIR"?Object.freeze({path:location,kind:"missing",bytes:0,sha256:null}):other;}
};
/** 🧪️ Observes every producer-side input outside the Source bytes: installer configuration at each level an installer could read (scope, ambient, Store-fixed) and the ambient variables that select registries, caches or transport. */
export function observeJavascriptDependencyGenerationProducerV1(workspace:string,storeRoot:string,scopes:readonly string[],ambient:JavascriptDependencyGenerationAmbientV1=javascriptDependencyGenerationAmbientV1()):GenerationProducerV1{
 const root=resolve(workspace),policy=protocol.producer,paths=new Set<string>();
 for(const scope of scopes)for(const name of policy.scopeFiles)paths.add(join(resolve(root,scope),name));
 for(const name of policy.ambientHomeFiles)paths.add(join(ambient.home,name));
 for(const directory of new Set([ambient.config,join(ambient.home,policy.ambientConfigFallback)]))for(const name of policy.ambientConfigFiles)paths.add(join(directory,name));
 for(const[role,names]of Object.entries(policy.storeRoleFiles)as [string,string[]][])for(const name of names)paths.add(join(storeRoot,role,name));
 const files=[...paths].map(path=>configFile(path,root,storeRoot)).sort((left,right)=>compareBytes(left.path,right.path)),ignored=new Set(policy.environmentIgnored),names=new Set(policy.environmentNames),environment:GenerationProducerVariableV1[]=[];
 for(const[name,value]of Object.entries(ambient.env)){
  const upper=name.toUpperCase();
  if(value===undefined||ignored.has(upper))continue;
  if(names.has(upper)||policy.environmentPrefixes.some((prefix:string)=>upper.startsWith(prefix)))environment.push(Object.freeze({name,sha256:generationSha256V1(value)}));
 }
 environment.sort((left,right)=>compareBytes(left.name,right.name));
 return Object.freeze({files:Object.freeze(files),environment:Object.freeze(environment)});
}
/** 🚪️ Repeats the provider walk's refusal of ambient resolution conditions. */
export const javascriptDependencyGenerationConditionsClosedV1=():boolean=>!(process.env.NODE_OPTIONS||process.env.BUN_OPTIONS||process.execArgv.some(value=>/^--(?:conditions|preserve-symlinks)(?:=|$)/u.test(value)));

const controlOf=(context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,mode:"shared"|"exclusive",check:()=>void=()=>{}):GenerationControlV1=>{
 const started=performance.now(),verify=()=>{context.check(control);check();};
 return Object.freeze({signal:control.signal,check:verify,tick:async(work:number)=>{verify();await control.publish({resource:"dependencies:generation-verify:"+work,mode,elapsedMs:performance.now()-started});await control.yieldContinuation();verify();}});
};
const readBounded=(path:string,maximum:number):Buffer=>{const facts=lstatSync(path,{bigint:true});if(!facts.isFile()||facts.size>BigInt(maximum))throw Error("Sealed generation evidence required");return readFileSync(path);};
const receiptNames=(storeRoot:string,scope:string):string[]=>{
 const prefix=scopeKey(scope)+".";
 try{return readdirSync(join(storeRoot,protocol.directory)).filter(name=>name.startsWith(prefix)&&name.endsWith(protocol.receiptSuffix)).sort();}catch{return[];}
};
const portsOf=(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,generation:GenerationControlV1,read:(name:string,maximumBytes:number)=>Uint8Array,receipt:GenerationReceiptV1,subject:JavascriptDependencyGenerationSubjectV1,recheck:boolean,vouched?:(paths:readonly string[])=>Promise<ReadonlySet<string>>)=>Object.freeze({
 workspace,storeRoot:context.root,receipt,recheck,...(vouched?{vouched}:{}),readEvidence:read,runtime:javascriptDependencyGenerationRuntimeV1,producer:()=>observeJavascriptDependencyGenerationProducerV1(workspace,context.root,receipt.required,subject.ambient),conditionsClosed:javascriptDependencyGenerationConditionsClosedV1,
 project:(path:string)=>readBunWorkspaceListingProjectionV1(workspace,path),
 observeBindings:(_scope:string,pairs:readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[])=>observeJavascriptDependencyBindingsReadV1(context,pairs,control),
 readReady:async(path:string)=>(await readJavascriptDependencyBindingReceiptReadV1(context,path,control)).fingerprint,control:generation,
});

/** 🪙️ Proves an unchanged generation by stamps and bounded reads under the caller's shared read custody; any doubt returns a typed mismatch for the complete walk. A `recheck` is the second look of a command at a receipt it has just verified completely: it replays every tier but the provider observations, which only an installer holding the store exclusively can move while this command holds it shared. */
export async function verifyJavascriptDependencyGenerationReceiptV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,scope:string,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1(),options:Readonly<{recheck?:boolean;vouch?:(receiptSha256:string)=>Promise<VouchV1|undefined>}>={}):Promise<JavascriptDependencyGenerationVerifyV1>{
 const root=resolve(workspace),names=receiptNames(context.root,scope);let last:JavascriptDependencyGenerationVerifyV1={status:"absent"};
 for(const name of names){
  context.check(control);let receipt:GenerationReceiptV1;
  try{receipt=parseGenerationReceiptV1(readBounded(join(context.root,protocol.directory,name),protocol.maximumReceiptBytes));}catch{last=Object.freeze({status:"mismatch",class:"receipt-invalid",path:name});continue;}
  if(receipt.scope!==scope)continue;
  const vouch=options.vouch?await options.vouch(receipt.sha256):undefined,vouched=vouch?(paths:readonly string[])=>options.recheck===true?vouch.settled(paths,control.signal):Promise.resolve(vouch.skippable(paths)):undefined;
  const result=await verifyGenerationReceiptV1(portsOf(root,context,control,controlOf(context,control,"shared"),(evidence,maximum)=>{if(!evidenceName.test(evidence))throw Error("Sealed generation evidence required");try{return readBounded(join(context.root,protocol.directory,evidence),maximum);}catch{throw Error("Sealed generation evidence required");}},receipt,subject,options.recheck===true,vouched));
  if(result.status==="match")return result;last=result;
 }
 return last;
}

const colon=(json:string|null):string|null=>json===null?null:(JSON.parse(json)as string[]).join(":");
/** 🗺️ Maps one completed walk onto the neutral capture input; nothing outside the walk's own results is consulted. */
export async function captureJavascriptDependencyGenerationReceiptV1(workspace:string,store:Readonly<{root:string}>,control:GenerationControlV1,walk:JavascriptDependencyGenerationWalkV1,observed:BunWorkspaceObservedOwnershipV1,startedAtMs:number,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()){
 const root=resolve(workspace),evidence=walk.evidence,plans=new Map(walk.projection.plans.map(row=>[row.scope,row.plan]));
 return captureGenerationReceiptV1({
  workspace:root,storeRoot:store.root,scope:evidence.scope,startedAtMs,runtime:javascriptDependencyGenerationRuntimeV1(),producer:observeJavascriptDependencyGenerationProducerV1(root,store.root,walk.required,subject.ambient),recipeSha256:walk.projection.recipeSha256,ownershipSha256:walk.projection.owners.sha256,required:walk.required,
  inputs:walk.projection.plans.map(row=>({scope:row.scope,sources:row.plan.sources.map(source=>({path:source.path,kind:source.kind,dev:source.dev,ino:source.ino,mode:source.mode,bytes:source.bytes}))})),
  membership:observed,
  bindings:walk.required.map(name=>({scope:name,pairs:plans.get(name)!.bindings,observation:evidence.bindings.find(([scope])=>scope===name)![1]})),
  ready:walk.receipts.map(row=>({scope:row.scope,generation:row.generation,path:join(plans.get(row.scope)!.directory,"bindings.json"),fingerprint:row.fingerprint})),
  execution:{scopes:evidence.execution.scopes,runtimeSha256:evidence.execution.runtimeSha256,observations:evidence.execution.observations.map(row=>({path:row.path,stamp:colon(row.stamp),anchor:colon(row.anchor),kind:row.kind,target:row.target,full:row.full}))},
  scopeReadSha256:walk.sha256,control,
 });
}

const relativeWriteCeilingBytes=65536,sharedRetireGraceMilliseconds=600000;
/** 📝️ Writes immutable evidence through the bounded relative-create API in chunks within its write ceiling and removes a partially written file instead of leaving it behind. */
export async function writeJavascriptDependencyGenerationFilesV1(storeRoot:string,files:readonly Readonly<{name:string;bytes:Uint8Array}>[],checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<void>{
 const state:{unfinished?:string}={};
 try{
  await withArtifactRelativeDirectoryV1(storeRoot,checkpoint,async base=>{
   if(await base.entry(protocol.directory)===null){await base.withNewDirectory(protocol.directory,async child=>{await child.sync();});await base.sync();}
   await base.withDirectory(protocol.directory,async directory=>{
    for(const file of files){
     const existing=await directory.entry(file.name);
     if(existing){if(existing.kind!=="file"||existing.size!==BigInt(file.bytes.length))throw Error("Existing immutable generation file changed");continue;}
     state.unfinished=file.name;
     await directory.withNewFile(file.name,async handle=>{let offset=0;while(offset<file.bytes.length){checkpoint();offset+=handle.write(file.bytes,offset,Math.min(relativeWriteCeilingBytes,file.bytes.length-offset),offset);await yieldContinuation();}handle.sync();});
     state.unfinished=undefined;
     await directory.sync();
    }
   });
  });
 }catch(error){
  if(state.unfinished!==undefined)await removeUnfinishedGenerationLeafV1(storeRoot,state.unfinished).catch(()=>{});
  throw error;
 }
}
/** 🧽️ Removes the one leaf a failed write created through a fresh descriptor scope, because the failed scope retains its created leaf and refuses further children. */
async function removeUnfinishedGenerationLeafV1(storeRoot:string,name:string):Promise<void>{
 await withArtifactRelativeDirectoryV1(storeRoot,()=>{},base=>base.withDirectory(protocol.directory,async directory=>{const leaf=await directory.entry(name);if(leaf&&leaf.kind==="file")await directory.remove(name,leaf);}));
}

/** 🧫️ Replays a completed walk's own recorded observations against the live tree under the caller's custody; nothing is persisted, so only the provider settle rule applies. */
async function replayJavascriptDependencyGenerationWalkUnguardedV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined=walk.observed?.(),subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<GenerationReplayResultV1>{
 if(!observed)throw Error("Retained workspace topology required");
 const generation=controlOf(context,control,"shared",checkpoint),captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,{root:context.root},generation,walk,observed,startedAtMs,subject);
 if(captured.status==="deferred")return Object.freeze({status:"deferred",reason:captured.reason,...(captured.path===undefined?{}:{path:captured.path})});
 const files=new Map(captured.files.map(file=>[file.name,file.bytes])),proof=await verifyGenerationReceiptV1(portsOf(resolve(workspace),context,control,generation,name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},captured.receipt,subject,false));
 if(proof.status!=="match")return Object.freeze({status:"mismatch",class:proof.class,...(proof.path===undefined?{}:{path:proof.path})});
 return proof.sha256===walk.sha256?Object.freeze({status:"match",sha256:proof.sha256}):Object.freeze({status:"mismatch",class:"seal"});
}
/** 🛡️ Replays a completed walk and contains a replay that cannot capture, so the complete walk decides instead of the command failing. */
export function replayJavascriptDependencyGenerationWalkV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined=walk.observed?.(),subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<GenerationReplayResultV1>{
 return guardGenerationReplayV1({run:()=>replayJavascriptDependencyGenerationWalkUnguardedV1(workspace,context,control,checkpoint,walk,startedAtMs,observed,subject),aborted:()=>control.signal.aborted});
}
/** 🤝️ Publishes one receipt under the caller's SHARED Store custody, serialised only against other receipt writers, and reports busy instead of waiting for anybody. */
export async function publishJavascriptDependencyGenerationReceiptSharedV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number):Promise<GenerationPublishOutcomeV1>{
 const root=resolve(workspace);
 return runGenerationWriterV1({acquire:()=>acquireResourceLease({directory:repositoryLeaseDirectoryV1(root),resource:"javascript-dependency-generation-writer:"+root,mode:"exclusive",signal:control.signal,control,timeoutMs:0}),aborted:()=>control.signal.aborted,body:async()=>{await store.recheck();checkpoint();const outcome=await publishJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,walk,startedAtMs,walk.observed?.(),javascriptDependencyGenerationSubjectV1(),sharedRetireGraceMilliseconds);await store.recheck();return outcome;}});
}
/** 🚨️ Reports exactly which input made a receipt stale: language-neutral DATA on stderr and one line in the declared language. */
export function reportJavascriptDependencyGenerationMismatchV1(scope:string,mismatch:Readonly<{class:string;path?:string;detail?:string}>,language:string=resolveLanguageV1(concurrentProtocolV1.languages,[],process.env).language):void{
 if(mismatch.class==="protocol-changed")requireCurrentFormatStampsV1(["dependency.walk"],"dependency generation receipt","refuse");
 console.error("[DATA] Store dependency generation receipt stale "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-stale",scope,class:mismatch.class,...(mismatch.path===undefined?{}:{path:mismatch.path}),...(mismatch.detail===undefined?{}:{detail:mismatch.detail})}));
 console.error(describeGenerationMismatchV1(language,{class:"stale"},{scope,reason:describeGenerationMismatchV1(language,mismatch)}));
}
/** 📣️ Reports the publication outcomes the receipt writer itself stays silent about. */
export const reportJavascriptDependencyGenerationPublishV1=(scope:string)=>(outcome:GenerationPublishOutcomeV1|"error",detail?:string):void=>{if(outcome==="busy"||outcome==="error")console.error("[DATA] Store dependency generation receipt "+outcome+" "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-"+outcome,scope,...(detail===undefined?{}:{message:detail})}));};
export {confirmGenerationWalkV1,generationWalkOriginV1,publishGenerationBestEffortV1};

/** 📮️ Publishes one receipt under the caller's exclusive Store custody after proving it against the live tree; a receipt only saves time, so every failure except cancellation reports why nothing was written. */
export async function publishJavascriptDependencyGenerationReceiptV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined=walk.observed?.(),subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1(),graceMilliseconds:number=0):Promise<"published"|"present"|"deferred"|"stale"|"failed"> {
 try{return await writeJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,walk,startedAtMs,observed,subject,graceMilliseconds);}
 catch(error){if(control.signal.aborted||staleImplementationOfV1(error))throw error;console.error("[DATA] Store dependency generation receipt failed "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-failure",scope:walk.evidence.scope,message:String((error as Error)?.message??error).slice(0,300)}));return "failed";}
}
async function writeJavascriptDependencyGenerationReceiptV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined,subject:JavascriptDependencyGenerationSubjectV1,graceMilliseconds:number):Promise<"published"|"present"|"deferred"|"stale"> {
 if(!observed)throw Error("Retained workspace topology required");
 const context=await javascriptDependencyReadContextV1(workspace,store),scope=walk.evidence.scope,generation=controlOf(context,control,"exclusive",checkpoint);
 if((await verifyJavascriptDependencyGenerationReceiptV1(workspace,context,control,scope,subject)).status==="match")return "present";
 const captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,store,generation,walk,observed,startedAtMs,subject);
 if(captured.status==="deferred"){console.error("[DATA] Store dependency generation receipt deferred "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-deferral",scope,reason:captured.reason,...(captured.path===undefined?{}:{path:captured.path})}));return "deferred";}
 const files=new Map(captured.files.map(file=>[file.name,file.bytes])),proof=await verifyGenerationReceiptV1(portsOf(resolve(workspace),context,control,generation,name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},captured.receipt,subject,false));
 if(proof.status!=="match"||proof.sha256!==walk.sha256){reportJavascriptDependencyGenerationMismatchV1(scope,proof.status==="mismatch"?proof:{class:"seal"});return "stale";}
 const receiptName=scopeKey(scope)+"."+captured.receipt.sha256+protocol.receiptSuffix,receiptBytes=Buffer.from(JSON.stringify(captured.receipt)+"\n"),all=[...captured.files,{name:receiptName,bytes:receiptBytes}];
 await writeJavascriptDependencyGenerationFilesV1(store.root,all,checkpoint,()=>control.yieldContinuation());
 retireJavascriptDependencyGenerationFilesV1(store.root,scope,graceMilliseconds);
 return "published";
}

/** 🧹️ Removes superseded receipts of one scope and every evidence file no remaining receipt names; failure only costs disk. */
export function retireJavascriptDependencyGenerationFilesV1(storeRoot:string,scope:string,graceMilliseconds:number=0):void{
 const directory=join(storeRoot,protocol.directory),aged=(name:string)=>graceMilliseconds<=0||Date.now()-lstatSync(join(directory,name)).mtimeMs>=graceMilliseconds;
 try{
  const names=readdirSync(directory),prefix=scopeKey(scope)+".",receipts=names.filter(name=>name.endsWith(protocol.receiptSuffix)),newest=receipts.filter(name=>name.startsWith(prefix)).map(name=>({name,at:lstatSync(join(directory,name)).mtimeMs})).sort((a,b)=>b.at-a.at)[0]?.name,keep=new Set<string>();
  for(const name of receipts){
   if(name.startsWith(prefix)&&name!==newest&&aged(name)){unlinkSync(join(directory,name));continue;}
   try{const receipt=parseGenerationReceiptV1(readFileSync(join(directory,name)));keep.add(receipt.membership.frontier.name);keep.add(receipt.execution.evidence.name);}catch{}
  }
  for(const name of names)if(evidenceSweep.test(name)&&!keep.has(name)&&aged(name))unlinkSync(join(directory,name));
 }catch{}
}
