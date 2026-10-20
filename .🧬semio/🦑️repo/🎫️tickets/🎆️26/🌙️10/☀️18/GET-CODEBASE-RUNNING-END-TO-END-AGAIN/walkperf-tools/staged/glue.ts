import {createHash} from "node:crypto";
import {lstatSync,readFileSync,readdirSync,statSync,unlinkSync} from "node:fs";
import {homedir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withArtifactRelativeDirectoryV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {readBunWorkspaceListingProjectionV1,type BunWorkspaceObservedOwnershipV1} from "../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import {javascriptDependencyReadContextV1,javascriptDependencySourceOwnerRuntimeV1,observeJavascriptDependencyBindingsReadV1,readJavascriptDependencyBindingReceiptReadV1,type JavascriptDependencyReadContextV1,type readJavascriptDependencySourceOwnerScopeReadV1} from "../../💾️storage/🟦️.ts";
import installSchema from "../../💾️storage/🧬️schema/📥️install.json" with {type:"json"};
import {captureGenerationReceiptV1,generationLocationV1,generationProtocolV1 as protocol,generationSha256V1,parseGenerationReceiptV1,verifyGenerationReceiptV1,type GenerationControlV1,type GenerationProducerFileV1,type GenerationProducerV1,type GenerationProducerVariableV1,type GenerationReceiptV1,type GenerationRuntimeV1,type GenerationVerifyResultV1} from "../🟦️.ts";

const repositoryRoot=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../../../../../.."),compareBytes=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const scopeKey=(scope:string)=>generationSha256V1(scope).slice(0,16);
const evidenceName=/^[a-f0-9]{64}\.(?:frontier|execution|closure)\.ndjson$/u;

/** 🧬️ Is a completed complete walk plus the topology it retained; only such a walk may be published. */
export type JavascriptDependencyGenerationWalkV1=Awaited<ReturnType<typeof readJavascriptDependencySourceOwnerScopeReadV1>>&Readonly<{observed?:()=>BunWorkspaceObservedOwnershipV1}>;
/** 🔍️ Is either the exact result of a complete walk, a typed mismatch, or the absence of any receipt. */
export type JavascriptDependencyGenerationVerifyV1=GenerationVerifyResultV1|Readonly<{status:"absent"}>;

/** 🧭️ Names what a receipt is sealed against: where the implementation closure lives, which entries span it and which ambient locations the installer environment could read. */
export interface JavascriptDependencyGenerationSubjectV1{readonly root:string;readonly entries:readonly string[];readonly ambient:JavascriptDependencyGenerationAmbientV1;readonly loadedAtMs:number}
/** 🧱️ Reads one runtime closure file as the capture hashed it; a symlink, a missing file or anything but an ordinary file counts as changed. */
export function javascriptDependencyGenerationImplementationFileV1(root:string=repositoryRoot):(path:string)=>Readonly<{sha256:string;bytes:number}>|undefined{
 return path=>{
  const full=join(root,path),facts=lstatSync(full,{bigint:true,throwIfNoEntry:false});
  if(!facts||!facts.isFile()||facts.size>BigInt(protocol.maximumSourceFileBytes))return undefined;
  try{const bytes=readFileSync(full);return Object.freeze({sha256:generationSha256V1(bytes),bytes:bytes.length});}catch{return undefined;}
 };
}
/** 🔏️ Captures the actual runtime import closure of every walk and fence entry (imports, literal assets, module-owned schemas) through the one reader that closes build implementations. */
export async function captureJavascriptDependencyGenerationImplementationV1(invocation:ScriptInvocation,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1(),extraFiles:readonly string[]=protocol.implementation.extraFiles){
 const {captureBuildSourceClosureV1}=await import("../../../../../🏗️build/📥️source/🔏️closure/🟦️.ts"),files=new Map<string,Readonly<{path:string;sha256:string;bytes:number}>>(),read=javascriptDependencyGenerationImplementationFileV1(subject.root);
 for(const entry of subject.entries)for(const file of (await captureBuildSourceClosureV1(subject.root,entry,invocation)).files)files.set(file.path,Object.freeze({path:file.path,sha256:file.sha256,bytes:Math.floor(file.base64.length*3/4)-(file.base64.endsWith("==")?2:file.base64.endsWith("=")?1:0)}));
 for(const extra of extraFiles){const live=read(extra);if(!live)throw Error("Ordinary implementation asset required");files.set(extra,Object.freeze({path:extra,...live}));}
 const stamped=[...files.values()].sort((left,right)=>compareBytes(left.path,right.path)).map(file=>{const facts=lstatSync(join(subject.root,file.path),{bigint:true});if(!facts.isFile())throw Error("Ordinary implementation Source file required");return Object.freeze({...file,mtimeNs:String(facts.mtimeNs),ctimeNs:String(facts.ctimeNs)});});
 return Object.freeze({entries:Object.freeze([...subject.entries]),loadedAtMs:subject.loadedAtMs,files:Object.freeze(stamped)});
}
/** ⚙️ Reads the live tool identity from the same source the provider walk fingerprints. */
export function javascriptDependencyGenerationRuntimeV1():GenerationRuntimeV1{
 return Object.freeze({bun:String(process.versions.bun),revision:String((globalThis as {Bun?:{revision?:string}}).Bun?.revision??""),node:process.versions.node,platform:process.platform,arch:process.arch,sha256:javascriptDependencySourceOwnerRuntimeV1()});
}
/** 🌐️ Names where the process finds its home and configuration directories; every field is injectable so laws can run in isolation. */
export interface JavascriptDependencyGenerationAmbientV1{readonly env:Readonly<Record<string,string|undefined>>;readonly home:string;readonly config:string}
/** 🔭️ Reads the live ambient locations. */
export const javascriptDependencyGenerationAmbientV1=():JavascriptDependencyGenerationAmbientV1=>Object.freeze({env:process.env,home:homedir(),config:process.env.XDG_CONFIG_HOME||join(homedir(),protocol.producer.ambientConfigFallback)});
/** 🎯️ Names the live subject: this checkout's closure entries and the process ambient. */
export const javascriptDependencyGenerationSubjectV1=():JavascriptDependencyGenerationSubjectV1=>Object.freeze({root:repositoryRoot,entries:protocol.implementation.entries,ambient:javascriptDependencyGenerationAmbientV1(),loadedAtMs:Date.now()-process.uptime()*1000});
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
 for(const[role,names]of Object.entries(policy.storeRoleFiles))for(const name of names)paths.add(join(storeRoot,role,name));
 const files=[...paths].map(path=>configFile(path,root,storeRoot)).sort((left,right)=>compareBytes(left.path,right.path)),ignored=new Set(policy.environmentIgnored),names=new Set(policy.environmentNames),environment:GenerationProducerVariableV1[]=[];
 for(const[name,value]of Object.entries(ambient.env)){
  const upper=name.toUpperCase();
  if(value===undefined||ignored.has(upper))continue;
  if(names.has(upper)||policy.environmentPrefixes.some(prefix=>upper.startsWith(prefix)))environment.push(Object.freeze({name,sha256:generationSha256V1(value)}));
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
const portsOf=(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,generation:GenerationControlV1,read:(name:string,maximumBytes:number)=>Uint8Array,receipt:GenerationReceiptV1,subject:JavascriptDependencyGenerationSubjectV1)=>Object.freeze({
 workspace,storeRoot:context.root,receipt,readEvidence:read,implementationEntries:subject.entries,implementationFile:javascriptDependencyGenerationImplementationFileV1(subject.root),runtime:javascriptDependencyGenerationRuntimeV1,producer:()=>observeJavascriptDependencyGenerationProducerV1(workspace,context.root,receipt.required,subject.ambient),conditionsClosed:javascriptDependencyGenerationConditionsClosedV1,
 project:(path:string)=>readBunWorkspaceListingProjectionV1(workspace,path),
 observeBindings:(_scope:string,pairs:readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[])=>observeJavascriptDependencyBindingsReadV1(context,pairs,control),
 readReady:async(path:string)=>(await readJavascriptDependencyBindingReceiptReadV1(context,path,control)).fingerprint,control:generation,
});

/** 🪙️ Proves an unchanged generation by stamps and bounded reads under the caller's shared read custody; any doubt returns a typed mismatch for the complete walk. */
export async function verifyJavascriptDependencyGenerationReceiptV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,scope:string,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<JavascriptDependencyGenerationVerifyV1>{
 const root=resolve(workspace),names=receiptNames(context.root,scope);let last:JavascriptDependencyGenerationVerifyV1={status:"absent"};
 for(const name of names){
  context.check(control);let receipt:GenerationReceiptV1;
  try{receipt=parseGenerationReceiptV1(readBounded(join(context.root,protocol.directory,name),protocol.maximumReceiptBytes));}catch{last=Object.freeze({status:"mismatch",class:"receipt-invalid",path:name});continue;}
  if(receipt.scope!==scope)continue;
  const result=await verifyGenerationReceiptV1(portsOf(root,context,control,controlOf(context,control,"shared"),(evidence,maximum)=>{if(!evidenceName.test(evidence))throw Error("Sealed generation evidence required");try{return readBounded(join(context.root,protocol.directory,evidence),maximum);}catch{throw Error("Sealed generation evidence required");}},receipt,subject));
  if(result.status==="match")return result;last=result;
 }
 return last;
}

const colon=(json:string|null):string|null=>json===null?null:(JSON.parse(json)as string[]).join(":");
/** 🗺️ Maps one completed walk onto the neutral capture input; nothing outside the walk's own results is consulted. */
export async function captureJavascriptDependencyGenerationReceiptV1(workspace:string,store:Readonly<{root:string}>,control:GenerationControlV1,walk:JavascriptDependencyGenerationWalkV1,observed:BunWorkspaceObservedOwnershipV1,startedAtMs:number,invocation:ScriptInvocation,subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()){
 const root=resolve(workspace),evidence=walk.evidence,plans=new Map(walk.projection.plans.map(row=>[row.scope,row.plan]));
 return captureGenerationReceiptV1({
  workspace:root,storeRoot:store.root,scope:evidence.scope,startedAtMs,implementation:await captureJavascriptDependencyGenerationImplementationV1(invocation,subject),runtime:javascriptDependencyGenerationRuntimeV1(),producer:observeJavascriptDependencyGenerationProducerV1(root,store.root,walk.required,subject.ambient),recipeSha256:walk.projection.recipeSha256,ownershipSha256:walk.projection.owners.sha256,required:walk.required,
  inputs:walk.projection.plans.map(row=>({scope:row.scope,sources:row.plan.sources.map(source=>({path:source.path,kind:source.kind,dev:source.dev,ino:source.ino,mode:source.mode,bytes:source.bytes}))})),
  membership:observed,
  bindings:walk.required.map(name=>({scope:name,pairs:plans.get(name)!.bindings,observation:evidence.bindings.find(([scope])=>scope===name)![1]})),
  ready:walk.receipts.map(row=>({scope:row.scope,generation:row.generation,path:join(plans.get(row.scope)!.directory,"bindings.json"),fingerprint:row.fingerprint})),
  execution:{scopes:evidence.execution.scopes,runtimeSha256:evidence.execution.runtimeSha256,observations:evidence.execution.observations.map(row=>({path:row.path,stamp:colon(row.stamp),anchor:colon(row.anchor),kind:row.kind,target:row.target,full:row.full}))},
  scopeReadSha256:walk.sha256,control,
 });
}

/** 📮️ Publishes one receipt under the caller's exclusive Store custody after proving it against the live tree; a receipt only saves time, so every failure except cancellation reports why nothing was written. */
export async function publishJavascriptDependencyGenerationReceiptV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined=walk.observed?.(),subject:JavascriptDependencyGenerationSubjectV1=javascriptDependencyGenerationSubjectV1()):Promise<"published"|"present"|"deferred"|"stale"|"failed"> {
 try{return await writeJavascriptDependencyGenerationReceiptV1(workspace,store,control,checkpoint,walk,startedAtMs,observed,subject);}
 catch(error){if(control.signal.aborted)throw error;console.error("[DATA] Store dependency generation receipt failed "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-failure",scope:walk.evidence.scope,message:String((error as Error)?.message??error).slice(0,300)}));return "failed";}
}
async function writeJavascriptDependencyGenerationReceiptV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,checkpoint:()=>void,walk:JavascriptDependencyGenerationWalkV1,startedAtMs:number,observed:BunWorkspaceObservedOwnershipV1|undefined,subject:JavascriptDependencyGenerationSubjectV1):Promise<"published"|"present"|"deferred"|"stale"> {
 if(!observed)throw Error("Retained workspace topology required");
 const context=await javascriptDependencyReadContextV1(workspace,store),scope=walk.evidence.scope,generation=controlOf(context,control,"exclusive",checkpoint);
 if((await verifyJavascriptDependencyGenerationReceiptV1(workspace,context,control,scope,subject)).status==="match")return "present";
 const captured=await captureJavascriptDependencyGenerationReceiptV1(workspace,store,generation,walk,observed,startedAtMs,context.invocation,subject);
 if(captured.status==="deferred"){console.error("[DATA] Store dependency generation receipt deferred "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-deferral",scope,reason:captured.reason,...(captured.path===undefined?{}:{path:captured.path})}));return "deferred";}
 const files=new Map(captured.files.map(file=>[file.name,file.bytes])),proof=await verifyGenerationReceiptV1(portsOf(resolve(workspace),context,control,generation,name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},captured.receipt,subject));
 if(proof.status!=="match"||proof.sha256!==walk.sha256){console.error("[DATA] Store dependency generation receipt stale "+JSON.stringify({version:1,purpose:"current-dependency-generation-receipt-stale",scope,...(proof.status==="mismatch"?{class:proof.class,...(proof.path===undefined?{}:{path:proof.path})}:{})}));return "stale";}
 const receiptName=scopeKey(scope)+"."+captured.receipt.sha256+protocol.receiptSuffix,receiptBytes=Buffer.from(JSON.stringify(captured.receipt)+"\n"),all=[...captured.files,{name:receiptName,bytes:receiptBytes}];
 await withArtifactRelativeDirectoryV1(store.root,checkpoint,async base=>{
  if(await base.entry(protocol.directory)===null){await base.withNewDirectory(protocol.directory,async child=>{await child.sync();});await base.sync();}
  await base.withDirectory(protocol.directory,async directory=>{
   for(const file of all){
    const existing=await directory.entry(file.name);
    if(existing){if(existing.kind!=="file"||existing.size!==BigInt(file.bytes.length))throw Error("Existing immutable generation file changed");continue;}
    await directory.withNewFile(file.name,async handle=>{let offset=0;while(offset<file.bytes.length){checkpoint();offset+=handle.write(file.bytes,offset,Math.min(262144,file.bytes.length-offset),offset);await control.yieldContinuation();}handle.sync();});
    await directory.sync();
   }
  });
 });
 retireJavascriptDependencyGenerationFilesV1(store.root,scope);
 return "published";
}

/** 🧹️ Removes superseded receipts of one scope and every evidence file no remaining receipt names; failure only costs disk. */
export function retireJavascriptDependencyGenerationFilesV1(storeRoot:string,scope:string):void{
 const directory=join(storeRoot,protocol.directory);
 try{
  const names=readdirSync(directory),prefix=scopeKey(scope)+".",receipts=names.filter(name=>name.endsWith(protocol.receiptSuffix)),newest=receipts.filter(name=>name.startsWith(prefix)).map(name=>({name,at:lstatSync(join(directory,name)).mtimeMs})).sort((a,b)=>b.at-a.at)[0]?.name,keep=new Set<string>();
  for(const name of receipts){
   if(name.startsWith(prefix)&&name!==newest){unlinkSync(join(directory,name));continue;}
   try{const receipt=parseGenerationReceiptV1(readFileSync(join(directory,name)));keep.add(receipt.membership.frontier.name);keep.add(receipt.execution.evidence.name);keep.add(receipt.implementation.evidence.name);}catch{}
  }
  for(const name of names)if(evidenceName.test(name)&&!keep.has(name))unlinkSync(join(directory,name));
 }catch{}
}
