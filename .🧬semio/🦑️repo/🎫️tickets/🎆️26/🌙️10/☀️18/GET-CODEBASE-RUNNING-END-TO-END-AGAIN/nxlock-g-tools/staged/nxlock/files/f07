import{refuseCachingV1}from"../../../📖️texts/🟦️.ts";
import{createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,otherLeaseHoldersV1,withResourceLeases,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {createHash,randomBytes} from "node:crypto";
import {connect} from "node:net";
import {resolve,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readScriptProcessEnvelope,createScriptProcessEnvelope,withScriptProcessEnvelope,withReceivedImplementationPinV1,type ScriptProcessEnvelope,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {readNxDaemonBirthV1,createNxDaemonLifetimeV1,createNxDaemonReadinessV1,type NxDaemonBirthV1} from "../🧬️lifetime/🟦️.ts";
import {withNxToolingUseV1,nxToolingUseStorageV1,type NxToolingUseWitnessV1} from "../../../🚀️bootstrap/🛠️tools/📥️use/🟦️.ts";
import {withRepositoryToolingStoreV1,type RepositoryToolingStoreV1} from "../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {repositoryCachePartitionRootV1,repositoryLeaseDirectoryV1} from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {readPhysicalCargoSourceV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import {withRepositoryNxStorageV1,nxDaemonNamespaceResourceV1} from "../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import {nxGraphNamespaceResourceV1} from "../../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import servicePolicy from "../🧬️lifetime/🧬️schema/📋️policy.json" with {type:"json"};
import lifetimeSchema from "../🧬️lifetime/🧬️schema/🔣️.json" with {type:"json"};
import witnessSchema from "../../../🚀️bootstrap/🛠️tools/📥️use/🧬️schema/📥️witness.json" with {type:"json"};

import {acquireNxDaemonSlotV1,nxDaemonIdleV1,nxDaemonLimitsV1,nxDaemonRetireV1,nxDaemonYieldRequestedV1,nxDaemonYieldV1,removeNxDaemonResidentV1,writeNxDaemonResidentV1} from "../🚏️admission/🟦️.ts";
export const NX_DAEMON_SERVICE_BIRTH_ENV_V1="SEMIO_NX_DAEMON_SERVICE_BIRTH";
/** 📨️ Closed handoff data is authenticated through the existing live parent endpoint before service issuance. */
export type NxDaemonServiceHandoffV1=Readonly<{birth:NxDaemonBirthV1;witness:NxToolingUseWitnessV1}>;
/** 🔎️ Parses only the exact parent descriptor; parsing alone grants no persistent authority. */
export function readNxDaemonServiceHandoffV1(wire:string|undefined):NxDaemonServiceHandoffV1{
 if(typeof wire!=="string"||Buffer.byteLength(wire)>1048576)throw Error("Bounded original daemon handoff required");const row:unknown=JSON.parse(wire);
 if(!row||typeof row!=="object"||Array.isArray(row)||Object.keys(row).sort().join(",")!=="birth,witness")throw Error("Closed original daemon handoff required");const value=row as {birth:unknown;witness:unknown},birth=readNxDaemonBirthV1(value.birth);
 if(validateJsonSchemaSubset(witnessSchema,value.witness).length)throw Error("Original live parent toolkit witness required");const witness=value.witness as NxToolingUseWitnessV1,original=readScriptProcessEnvelope(witness.original,Date.now());
 if(birth.parentWitness!==witness.nonce||birth.toolingIdentity!==witness.identity||birth.toolkitRoot!==witness.toolkitRoot||birth.parentEnvelopeDigest!==createHash("sha256").update(JSON.stringify(original)).digest("hex")||resolve(readNativeCallerAuthorityV1(original.capabilities).repositoryRoot)!==birth.repositoryRoot)throw Error("Exact original daemon handoff binding required");
 return Object.freeze({birth,witness:Object.freeze({...witness,original})});
}
/** 🔐️ A fresh response from that exact Original endpoint is required while its startup deadline remains live. */
async function authenticatePhysicalWitnessV1(root:string,witness:NxToolingUseWitnessV1,store:RepositoryToolingStoreV1,control:ResourceLeaseControl,token?:string):Promise<string>{
 requireIssuedResourceControlRefinementV1(store.control,control);checkResourceLeaseControl(control);requireNativeStorageReservationV1(store.storage,store.control);if(store.root!==repositoryCachePartitionRootV1(root,"tools")||witness.toolkitRoot!==join(store.root,"nx",witness.identity))throw Error("Original tools-backed daemon parent required");await store.recheck();checkResourceLeaseControl(control);const witnessPath=join(store.root,"use",witness.nonce+".json"),read=()=>readPhysicalCargoSourceV1(witnessPath,{expectedBytes:null,version:1,root:store.root,maximumBytes:1048576,remainingFiles:1},control),expected=JSON.stringify(witness),before=await read();if(!before||Buffer.from(before).toString("utf8")!==expected)throw Error("Original physical daemon parent witness required");
 const original=readScriptProcessEnvelope(witness.original,Date.now()),remaining=Math.min(10000,control.remainingMilliseconds()??Infinity,original.deadlineEpochMilliseconds===null?Infinity:original.deadlineEpochMilliseconds-Date.now());if(remaining<=0)throw Error("Original daemon parent deadline exhausted");checkResourceLeaseControl(control);
 const response=await new Promise<string>((accept,reject)=>{
  const socket=connect(witness.endpoint),chunks:Buffer[]=[];let bytes=0,done=false;const abort=()=>finish(control.signal.reason instanceof Error?control.signal.reason:Error("Original daemon handshake interrupted")),finish=(error?:Error)=>{if(done)return;done=true;control.signal.removeEventListener("abort",abort);socket.destroy();error?reject(error):accept(Buffer.concat(chunks).toString("utf8"));};
  control.signal.addEventListener("abort",abort,{once:true});if(control.signal.aborted){abort();return;}socket.setTimeout(remaining,()=>finish(Error("Original daemon live parent handshake exhausted")));socket.once("error",finish);socket.once("connect",()=>socket.write(witness.nonce+(token?" "+token:"")+"\n"));socket.on("data",chunk=>{bytes+=chunk.length;if(bytes>1048576)finish(Error("Original daemon parent handshake overflow"));else chunks.push(chunk);});socket.once("end",()=>finish());socket.once("close",()=>{if(!done)finish(Error("Original daemon parent handshake closed"));});
 });
 checkResourceLeaseControl(control);const after=await read();checkResourceLeaseControl(control);if(!after||Buffer.from(after).toString("utf8")!==expected)throw Error("Original daemon parent witness changed");await store.recheck();checkResourceLeaseControl(control);readScriptProcessEnvelope(original,Date.now());return response;
}

/** 🛎️ Service birth requires the same physical Original parent and its fresh authenticated response. */
export async function authenticateNxDaemonServiceHandoffV1(handoff:NxDaemonServiceHandoffV1,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<ScriptProcessEnvelope>{
 const response=await authenticatePhysicalWitnessV1(handoff.birth.repositoryRoot,handoff.witness,store,control);if(response!==JSON.stringify(handoff.witness))throw Error("Original daemon parent witness is not live");return readScriptProcessEnvelope(handoff.witness.original,Date.now());
}

/** 🚦️ Only the acquired root service controls resource settlement; later request envelopes cannot replace it. */
export interface NxDaemonServiceV1{
 readonly birth:NxDaemonBirthV1;
 readonly witness:NxToolingUseWitnessV1;
 server(shutdown:(reason:string)=>Promise<void>):void;
 ready():Promise<void>;
 waitForTerminal():Promise<void>;
 serverClosed():void;
 resourcesStopped():Promise<void>;
}
type OwnedDaemonV1={live:boolean;environment:Readonly<Record<string,string|undefined>>;readers:Map<string,{live:boolean;threadId:number}>;session:NxDaemonServiceV1;request<T>(body:()=>Promise<T>):Promise<T>;shutdown(reason:string):void;worker(id:string):()=>void;recheck():Promise<void>};
let ownedDaemon:OwnedDaemonV1|undefined;

/** 🌱️ Issues the explicit cache-only service policy after authenticating the actual finite startup owner. */
export async function withNxDaemonServiceV1<T>(environment:Readonly<Record<string,string|undefined>>,body:(service:NxDaemonServiceV1)=>Promise<T>):Promise<T>{
 if(ownedDaemon||typeof process.send!=="function")throw Error("One original IPC-owned daemon service required");const handoff=readNxDaemonServiceHandoffV1(environment[NX_DAEMON_SERVICE_BIRTH_ENV_V1]),{birth}=handoff;
 await withScriptProcessEnvelope(handoff.witness.original,parent=>withRepositoryToolingStoreV1(birth.repositoryRoot,parent,createIssuedResourceControlV1(parent,"cache:entry",0),(store,storePhase)=>authenticateNxDaemonServiceHandoffV1(handoff,store,storePhase)));
 if(validateJsonSchemaSubset(lifetimeSchema.$defs.ServicePolicy,servicePolicy).length)throw Error("Authored original daemon service policy required");
 const policy={version:1 as const,owner:servicePolicy.owner,maximumElapsedMilliseconds:servicePolicy.maximumElapsedMilliseconds},envelope=createScriptProcessEnvelope(policy,{version:1,kind:"nx-daemon-service",repositoryRoot:birth.repositoryRoot,workspaceRoot:birth.workspaceRoot,toolingIdentity:birth.toolingIdentity,generation:birth.generation,directory:birth.directory,watcherIdentity:birth.watcherIdentity,graphIdentity:birth.graphIdentity,implementationIdentity:birth.implementationIdentity},Date.now()),lifetime=createNxDaemonLifetimeV1(birth);
 const limits=nxDaemonLimitsV1(process.env),kind=birth.watcherIdentity===null?"finite" as const:"watch" as const,slot=await acquireNxDaemonSlotV1(repositoryLeaseDirectoryV1(birth.repositoryRoot),birth.repositoryRoot,kind,limits[kind],new AbortController().signal);
 let release!:()=>void;const released=new Promise<void>(accept=>{release=accept;});
 try{return await withScriptProcessEnvelope(envelope,original=>withRepositoryToolingStoreV1(birth.repositoryRoot,original,createIssuedResourceControlV1(original,"cache:entry",0),async(store,storePhase)=>{
  lifetime.record({type:"acquire",resource:"leases"});lifetime.record({type:"acquire",resource:"tools"});
  return withRepositoryNxStorageV1(birth.repositoryRoot,original,storePhase,async(bind,nxPhase)=>{
   let reader:((token:string)=>Promise<object|undefined>)|undefined;return withNxToolingUseV1(nxToolingUseStorageV1(birth.repositoryRoot),{identity:birth.toolingIdentity,toolkitRoot:birth.toolkitRoot,invocation:original,reader:token=>reader?.(token)??Promise.resolve(undefined)},nxPhase,async(witness,usePhase)=>{
    const namespace=await bind(birth.toolingIdentity,birth.graphIdentity,birth.implementationIdentity,birth.workspaceRoot,birth.watcherIdentity??undefined);if(namespace.environment.NX_WORKSPACE_DATA_DIRECTORY!==birth.directory)throw Error("Original canonical daemon finite namespace required");lifetime.record({type:"acquire",resource:"rust"});
    return withResourceLeases({directory:repositoryLeaseDirectoryV1(birth.repositoryRoot),resources:[{resource:nxDaemonNamespaceResourceV1(birth.directory),mode:"exclusive"}],control:usePhase,signal:usePhase.signal},async()=>{await authenticateNxDaemonServiceHandoffV1(handoff,store,usePhase);lifetime.record({type:"acquire",resource:"sdk"});lifetime.record({type:"acquired"});let terminal!:()=>void,shutdown:((reason:string)=>Promise<void>)|undefined,closing:Promise<void>|undefined,stopped=false,watchdog:ReturnType<typeof setTimeout>|undefined;
    const activity={open:0,last:Date.now()};const readiness=createNxDaemonReadinessV1();
    const settled=new Promise<void>(accept=>{terminal=accept;}),recheck=async()=>{await store.recheck();await bind(birth.toolingIdentity,birth.graphIdentity,birth.implementationIdentity,birth.workspaceRoot,birth.watcherIdentity??undefined);},finish=()=>{const state=lifetime.snapshot();if(stopped&&state.phase==="draining"&&!state.server&&!state.workers.length&&!state.requests.length){lifetime.record({type:"terminal"});terminal();}},drain=(reason:string)=>{readiness.refuse(Error("Original daemon drained before readiness"));if(lifetime.snapshot().phase==="terminal")return;lifetime.record({type:"drain"});watchdog??=setTimeout(()=>{process.stderr.write(`Original daemon drain exceeded ${limits.drainMilliseconds}ms (${reason}); exiting\n`);process.exit(70);},limits.drainMilliseconds);watchdog.unref();if(shutdown&&!closing)closing=Promise.resolve().then(()=>shutdown!(reason));if(closing)void closing.catch(error=>{});else{stopped=true;finish();}};
    const session:NxDaemonServiceV1=Object.freeze({birth,witness,server(stop:(reason:string)=>Promise<void>){if(shutdown)throw Error("One original daemon server owner required");shutdown=stop;lifetime.record({type:"server-open"});},async ready(){await recheck();lifetime.record({type:"ready"});readiness.ready();},waitForTerminal:()=>settled,serverClosed(){lifetime.record({type:"server-close"});finish();},async resourcesStopped(){stopped=true;finish();await released;}});
    const context:OwnedDaemonV1={live:true,readers:new Map(),environment:Object.freeze({...namespace.environment,NX_WORKSPACE_ROOT_PATH:birth.workspaceRoot,SEMIO_NX_TOOLING_ROOT:birth.toolkitRoot,SEMIO_NX_TOOLING_IDENTITY:birth.toolingIdentity,SEMIO_NX_WATCHER_IDENTITY:birth.watcherIdentity??undefined,SEMIO_BUILD_GRAPH_DIGEST:birth.graphIdentity??undefined,SEMIO_NX_IMPLEMENTATION_IDENTITY:birth.implementationIdentity,NX_ISOLATE_PLUGINS:"false"}),session,recheck,shutdown:drain,async request(run){if(!context.live)throw refuseCachingV1("cancelled","daemon.root-released");await readiness.admit();if(!context.live)throw refuseCachingV1("cancelled","daemon.root-released");const id=randomBytes(16).toString("hex");lifetime.record({type:"request-open",id});activity.open++;activity.last=Date.now();try{return await run();}finally{activity.open--;activity.last=Date.now();lifetime.record({type:"request-close",id});finish();}},worker(id){if(!context.live)throw refuseCachingV1("cancelled","daemon.root-released");lifetime.record({type:"worker-open",id});let live=true;return()=>{if(!live)return;live=false;lifetime.record({type:"worker-close",id});finish();};}};
    reader=async token=>{const row=context.readers.get(token);if(!context.live||!row?.live||row.threadId<1)return;await recheck();if(!context.live||!row.live)return;return {version:1,kind:"daemon-worker",repositoryRoot:birth.repositoryRoot,workspaceRoot:birth.workspaceRoot,toolingIdentity:birth.toolingIdentity,generation:birth.generation,directory:birth.directory,watcherIdentity:birth.watcherIdentity,graphIdentity:birth.graphIdentity,implementationIdentity:birth.implementationIdentity,token,processId:process.pid,threadId:row.threadId};};ownedDaemon=context;for(const[key,value]of Object.entries(context.environment)){if(value===undefined)delete process.env[key];else process.env[key]=value;}const cancel=()=>drain("Original daemon service interrupted");original.control.signal.addEventListener("abort",cancel,{once:true});
    const attachedClients=()=>otherLeaseHoldersV1(repositoryLeaseDirectoryV1(birth.repositoryRoot),nxGraphNamespaceResourceV1(birth.directory)).length;
    const leaseDirectory=repositoryLeaseDirectoryV1(birth.repositoryRoot),publishResident=()=>{const rssBytes=process.memoryUsage.rss();void writeNxDaemonResidentV1(leaseDirectory,{version:1,pid:process.pid,kind,rssBytes,atMs:Date.now()}).catch(()=>undefined);return rssBytes;};
    const idleTimer=setInterval(()=>{const residentBytes=publishResident();if(nxDaemonIdleV1({kind,openRequests:activity.open+attachedClients(),lastActivity:activity.last,now:Date.now(),idleMilliseconds:limits.idleMilliseconds}))drain(`Idle for ${limits.idleMilliseconds}ms`);else if(nxDaemonRetireV1({kind,openRequests:activity.open+attachedClients(),residentBytes,limitBytes:limits.residentBytes}))drain(`Resident ${Math.round(residentBytes/1048576)} MB over the budget of ${Math.round(limits.residentBytes/1048576)} MB`);},limits.checkMilliseconds);idleTimer.unref();publishResident();
    const yieldTimer=setInterval(()=>{if(kind!=="finite"||activity.open>0||attachedClients()>0||lifetime.snapshot().phase!=="ready")return;void nxDaemonYieldRequestedV1(repositoryLeaseDirectoryV1(birth.repositoryRoot),birth.directory,original.control.signal).then(requested=>{if(nxDaemonYieldV1({kind,openRequests:activity.open,requested}))drain("Yielded the namespace to an eviction");},()=>undefined);},limits.yieldCheckMilliseconds);yieldTimer.unref();
    try{original.control.signal.throwIfAborted();process.send!({type:"SEMIO_NX_DAEMON_ACQUIRED",generation:birth.generation,toolingIdentity:birth.toolingIdentity});let result:T;try{result=await body(session);}catch(error){drain(String(error));await settled;throw error;}await settled;return result;}
    finally{clearInterval(idleTimer);clearInterval(yieldTimer);clearTimeout(watchdog);void removeNxDaemonResidentV1(leaseDirectory,process.pid).catch(()=>undefined);context.live=false;ownedDaemon=undefined;original.control.signal.removeEventListener("abort",cancel);}
    });
   });
  });
 }));}finally{release();slot.release();}
}

/** 📥️ Request work borrows the already acquired service root independently of mutable request authority and completes with the implementation the daemon loaded: a Source a peer edits meanwhile is left to the next request. */
export function withNxDaemonRequestV1<T>(body:()=>Promise<T>):Promise<T>{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");return ownedDaemon.request(()=>withReceivedImplementationPinV1(body));}
/** 🛑️ Signals the original server owner and retains its root loans through actual resource settlement. */
export function requestNxDaemonShutdownV1(reason:string):void{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");ownedDaemon.shutdown(reason);}
/** 🧹️ Shutdown completion follows actual server and worker/request settlement, then awaits physical scope release. */
export async function completeNxDaemonShutdownV1():Promise<void>{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");await ownedDaemon.session.resourcesStopped();}
/** 🔒️ Registers a real worker before its construction and keeps root custody until that owned thread exits. */
export function retainNxDaemonWorkerV1(id:string):()=>void{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");return ownedDaemon.worker(id);}

/** 🧷️ The finite client issues service birth only while its actual images and fresh exact SDK witness are retained. */
export function withNxDaemonServiceBirthV1<T>(original:ScriptInvocation,phaseControl:ResourceLeaseControl,directory:string,toolingIdentity:string,environment:Readonly<Record<string,string|undefined>>,body:(environment:Readonly<Record<string,string|undefined>>,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlInvocationV1(phaseControl,original);checkResourceLeaseControl(phaseControl);if(environment.NX_ISOLATE_PLUGINS!==undefined&&environment.NX_ISOLATE_PLUGINS!=="false")throw Error("Original daemon service requires owned in-process plugins");const root=resolve(readNativeCallerAuthorityV1(original.capabilities).repositoryRoot),toolkitRoot=environment.SEMIO_NX_TOOLING_ROOT;if(!toolkitRoot)throw Error("Original loaded daemon toolkit root required");
 return withRepositoryToolingStoreV1(root,original,phaseControl,(_store,storePhase)=>withRepositoryNxStorageV1(root,original,storePhase,async(bind,nxPhase)=>{
  const workspaceRoot=resolve(environment.NX_WORKSPACE_ROOT_PATH??process.cwd()),watcherIdentity=environment.SEMIO_NX_WATCHER_IDENTITY??null,graphIdentity=environment.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity=environment.SEMIO_NX_IMPLEMENTATION_IDENTITY!;
  return withNxToolingUseV1(nxToolingUseStorageV1(root),{identity:toolingIdentity,toolkitRoot,invocation:original},nxPhase,async(witness,usePhase)=>{
   const namespace=await bind(toolingIdentity,graphIdentity,implementationIdentity,workspaceRoot,watcherIdentity??undefined);if(directory!==namespace.environment.NX_WORKSPACE_DATA_DIRECTORY)throw Error("Original canonical daemon birth namespace required");
   const birth=readNxDaemonBirthV1({version:1,repositoryRoot:root,workspaceRoot,watcherIdentity,graphIdentity,implementationIdentity,toolkitRoot:resolve(toolkitRoot),toolingIdentity,directory,generation:randomBytes(16).toString("hex"),parentWitness:witness.nonce,parentEnvelopeDigest:createHash("sha256").update(JSON.stringify(witness.original)).digest("hex")});
   return body(Object.freeze({...environment,...namespace.environment,[NX_DAEMON_SERVICE_BIRTH_ENV_V1]:JSON.stringify({birth,witness})}),usePhase);
  });
 }));
}
/** 🚪️ Actual shutdown begins draining before server close; its existing owner still performs every close operation. */
export function beginNxDaemonShutdownV1():void{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");ownedDaemon.shutdown("Original daemon shutdown");}
/** 🪟️ Records the real server close completion without releasing worker or request loans. */
export function markNxDaemonServerClosedV1():void{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");ownedDaemon.session.serverClosed();}

/** 🏗️ Binds the actual server resource and its existing shutdown owner after the service acquired all root loans. */
export function registerNxDaemonServerV1(shutdown:(reason:string)=>Promise<void>):void{if(!ownedDaemon?.live)throw refuseCachingV1("missing","daemon.service-required");ownedDaemon.session.server(shutdown);}
/** 📌️ Client environment reflection cannot relocate any live service-owned writer or SDK reader. */
export function nxDaemonPinnedEnvironmentV1():Readonly<Record<string,string|undefined>>|undefined{return ownedDaemon?.live?ownedDaemon.environment:undefined;}

/** 👷️ One actual service-owned journal thread receives only its closed finite deployment and live reader token. */
export type NxDaemonJournalDeploymentV1=Readonly<{version:1;kind:"filesystem-journal";repositoryRoot:string;root:string;directory:string;generation:string;socketPath:string;toolingIdentity:string;graphIdentity:string|null;implementationIdentity:string;serviceWitness:NxToolingUseWitnessV1;readerToken:string}>;
/** 🧵️ Native thread construction stays behind its original service owner without exporting a third-party runtime type. */
export interface NxDaemonWorkerPortV1{readonly threadId:number;once(event:"exit",receive:(status:number)=>void):unknown;}
/** 🪡️ Registers before construction and releases only when that exact owned thread actually exits. */
export function createNxDaemonJournalWorkerV1<T extends NxDaemonWorkerPortV1>(socketPath:string,create:(deployment:NxDaemonJournalDeploymentV1)=>T):T{
 const current=ownedDaemon;if(!current?.live)throw Error("Original acquired daemon worker owner required");const endpoint=join(current.environment.NX_SOCKET_DIR!,"j.sock"),expected=process.platform==="win32"?"\\\\.\\pipe\\nx\\"+endpoint:endpoint;if(socketPath!==expected)throw Error("Original funded daemon journal socket required");const {birth,witness}=current.session,id=randomBytes(16).toString("hex"),token=randomBytes(16).toString("hex"),release=current.worker(id),row={live:true,threadId:0};current.readers.set(token,row);
 const close=()=>{if(!row.live)return;row.live=false;current.readers.delete(token);release();};
 try{const worker=create(Object.freeze({version:1,kind:"filesystem-journal",repositoryRoot:birth.repositoryRoot,root:birth.workspaceRoot,directory:join(birth.directory,"semio-journal"),generation:id,socketPath,toolingIdentity:birth.toolingIdentity,graphIdentity:birth.graphIdentity,implementationIdentity:birth.implementationIdentity,serviceWitness:witness,readerToken:token}));row.threadId=worker.threadId;worker.once("exit",close);return worker;}catch(error){close();throw error;}
}
/** 🪪️ A real same-process registered thread authenticates the service loan before any Native load or writer opens. */
export async function withNxDaemonJournalReaderV1<T>(deployment:NxDaemonJournalDeploymentV1,threadId:number,body:(original:ScriptInvocation)=>Promise<T>):Promise<T>{
 const witness=deployment.serviceWitness;if(validateJsonSchemaSubset(witnessSchema,witness).length||validateJsonSchemaSubset(lifetimeSchema.$defs.ServicePolicy,witness.original.policy).length||validateJsonSchemaSubset(lifetimeSchema.$defs.ServiceCapabilities,witness.original.capabilities).length||witness.processId!==process.pid||!Number.isSafeInteger(threadId)||threadId<1)throw Error("Original same-process daemon worker service required");
 const authority=witness.original.capabilities as {repositoryRoot:string;workspaceRoot:string;toolingIdentity:string;generation:string;directory:string;watcherIdentity:string|null;graphIdentity:string|null;implementationIdentity:string};if(authority.repositoryRoot!==deployment.repositoryRoot||authority.workspaceRoot!==deployment.root||authority.toolingIdentity!==deployment.toolingIdentity||authority.graphIdentity!==deployment.graphIdentity||authority.implementationIdentity!==deployment.implementationIdentity||deployment.directory!==join(authority.directory,"semio-journal")||!/^[a-f0-9]{32}$/u.test(deployment.readerToken))throw Error("Original canonical daemon worker loan required");
 return withScriptProcessEnvelope(witness.original,original=>withRepositoryToolingStoreV1(deployment.repositoryRoot,original,createIssuedResourceControlV1(original,"cache:entry",0),async(store,storePhase)=>{
  const response=JSON.parse(await authenticatePhysicalWitnessV1(deployment.repositoryRoot,witness,store,storePhase,deployment.readerToken));if(validateJsonSchemaSubset(lifetimeSchema.$defs.WorkerLoan,response).length||Object.entries({version:1,kind:"daemon-worker",repositoryRoot:authority.repositoryRoot,workspaceRoot:authority.workspaceRoot,toolingIdentity:authority.toolingIdentity,generation:authority.generation,directory:authority.directory,watcherIdentity:authority.watcherIdentity,graphIdentity:authority.graphIdentity,implementationIdentity:authority.implementationIdentity,token:deployment.readerToken,processId:process.pid,threadId}).some(([key,value])=>response[key]!==value))throw Error("Original live daemon worker reader refused");
  return withRepositoryNxStorageV1(deployment.repositoryRoot,original,storePhase,(bind,nxPhase)=>withNxToolingUseV1(nxToolingUseStorageV1(deployment.repositoryRoot),{identity:deployment.toolingIdentity,toolkitRoot:witness.toolkitRoot,invocation:original},nxPhase,async()=>{const namespace=await bind(deployment.toolingIdentity,deployment.graphIdentity,deployment.implementationIdentity,deployment.root,authority.watcherIdentity??undefined);if(namespace.environment.NX_WORKSPACE_DATA_DIRECTORY!==authority.directory)throw Error("Original finite daemon worker namespace required");const endpoint=join(namespace.environment.NX_SOCKET_DIR!,"j.sock"),expected=process.platform==="win32"?"\\\\.\\pipe\\nx\\"+endpoint:endpoint;if(deployment.socketPath!==expected)throw Error("Original funded daemon journal socket required");return body(original);}));
 }));
}
