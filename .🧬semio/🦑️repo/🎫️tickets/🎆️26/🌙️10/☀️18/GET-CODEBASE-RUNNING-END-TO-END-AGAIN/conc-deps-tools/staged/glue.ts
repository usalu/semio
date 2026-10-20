import {randomBytes} from "node:crypto";
import {closeSync,fsyncSync,lstatSync,mkdirSync,openSync,readdirSync,readFileSync,renameSync,rmSync,statfsSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
import {acquireResourceLease,type ResourceLease,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {concurrentEventV1,concurrentLeaseResourceV1,concurrentPinNamesV1,concurrentPinResourceV1,concurrentPointerNameV1,concurrentProtocolV1 as protocol,concurrentWaitStepV1,encodeConcurrentPointerV1,parseConcurrentPointerV1,type ConcurrentEventV1,type ConcurrentLeaseKindV1,type ConcurrentPointerV1} from "../🟦️.ts";

/** 🔌️ Are the effects of a politely waiting acquisition, replaceable by a law that must neither sleep nor touch a lease. */
export type ConcurrentLeasePortsV1=Readonly<{
 acquire:(resource:string,mode:"shared"|"exclusive")=>Promise<ResourceLease|null>;
 ready?:()=>Promise<boolean>;
 sleep:(milliseconds:number,signal:AbortSignal)=>Promise<void>;
 now:()=>number;
 report:(event:ConcurrentEventV1)=>void|Promise<void>;
 signal:AbortSignal;
}>;

/** 🧭️ Builds the production ports over real leases: a busy lease is a refusal after zero wait, never a queue ticket that could stall a newcomer. */
export function concurrentLeasePortsV1(directory:string,control:ResourceLeaseControl|undefined,signal:AbortSignal,ready:(()=>Promise<boolean>)|undefined,report:(event:ConcurrentEventV1)=>void|Promise<void>):ConcurrentLeasePortsV1{
 return Object.freeze({
  acquire:async(resource,mode)=>{
   try{return await acquireResourceLease({directory,resource,mode,signal,control,timeoutMs:0});}
   catch(error){if(error instanceof Error&&error.message.startsWith("Resource lease timed out:"))return null;throw error;}
  },
  ready,
  sleep:async(milliseconds,abort)=>{try{await delay(milliseconds,undefined,{signal:abort});}catch(error){abort.throwIfAborted();throw error;}},
  now:()=>Date.now(),
  report,
  signal,
 });
}

/** 🤝️ Acquires one workspace lease without ever queueing: it probes every round, asks `ready` every few seconds whether another process already did the work (then returns null: coalesced) and reports progress until the declared maximum. */
export async function acquireConcurrentLeaseV1(kind:ConcurrentLeaseKindV1,workspace:string,mode:"shared"|"exclusive",ports:ConcurrentLeasePortsV1,maximumMilliseconds:number):Promise<ResourceLease|null>{
 const resource=concurrentLeaseResourceV1(kind,workspace),started=ports.now();
 let lastRecheck=started,lastProgress=started;
 for(;;){
  ports.signal.throwIfAborted();
  const lease=await ports.acquire(resource,mode);
  if(lease)return lease;
  const now=ports.now(),step=concurrentWaitStepV1({elapsedMs:now-started,sinceRecheckMs:now-lastRecheck,sinceProgressMs:now-lastProgress,canRecheck:ports.ready!==undefined,maximumMs:maximumMilliseconds});
  if(step.expired)throw Error("Dependency "+kind+" lease wait exceeded its declared maximum");
  if(step.recheck){lastRecheck=now;if(await ports.ready!()){await ports.report(concurrentEventV1({event:"coalesced",resource:kind,elapsedMs:now-started}));return null;}}
  if(step.progress){lastProgress=now;await ports.report(concurrentEventV1({event:"waiting",resource:kind,elapsedMs:now-started}));}
  await ports.sleep(protocol.wait.probeMilliseconds,ports.signal);
 }
}

/** 📌️ Holds shared pins on the given generations while `body` consumes them; a collector can never remove a pinned generation. */
export async function withConcurrentPinsV1<T>(directory:string,workspace:string,generations:readonly string[],control:ResourceLeaseControl|undefined,signal:AbortSignal,body:()=>Promise<T>):Promise<T>{
 const held:ResourceLease[]=[];
 let failed=false,failure:unknown,value:T|undefined;
 try{
  for(const name of concurrentPinNamesV1(generations))held.push(await acquireResourceLease({directory,resource:concurrentPinResourceV1(workspace,name),mode:"shared",signal,control,timeoutMs:protocol.wait.maximumWriterMilliseconds}));
  value=await body();
 }catch(error){failed=true;failure=error;}
 const releases:unknown[]=[];
 for(const lease of held.reverse())try{lease.release();}catch(error){releases.push(error);}
 if(failed)throw failure;
 if(releases.length)throw releases[0];
 return value as T;
}

/** 🧹️ Takes the exclusive pin of one superseded generation without waiting; null means a reader still pins it. */
export async function tryConcurrentPinExclusiveV1(directory:string,workspace:string,name:string,control:ResourceLeaseControl|undefined,signal:AbortSignal):Promise<ResourceLease|null>{
 try{return await acquireResourceLease({directory,resource:concurrentPinResourceV1(workspace,name),mode:"exclusive",signal,control,timeoutMs:0});}
 catch(error){if(error instanceof Error&&error.message.startsWith("Resource lease timed out:"))return null;throw error;}
}

const pointerDirectory=(storeRoot:string)=>join(storeRoot,protocol.pointer.directory);
/** 📖️ Reads every sealed pointer of the store; a torn or foreign file cannot exist because publication replaces whole files, so an invalid one is skipped, never trusted. */
export function readConcurrentPointersV1(storeRoot:string):readonly ConcurrentPointerV1[]{
 let names:string[];
 try{names=readdirSync(pointerDirectory(storeRoot)).filter(name=>name.endsWith(protocol.pointer.suffix)).sort();}catch{return Object.freeze([]);}
 const rows:ConcurrentPointerV1[]=[];
 for(const name of names)try{
  const path=join(pointerDirectory(storeRoot),name),facts=lstatSync(path);
  if(!facts.isFile()||facts.size>protocol.pointer.maximumBytes)continue;
  const pointer=parseConcurrentPointerV1(readFileSync(path));
  if(concurrentPointerNameV1(pointer.scope)===name)rows.push(pointer);
 }catch{}
 return Object.freeze(rows);
}
/** 📖️ Reads one scope's pointer, or null while the scope was never published. */
export function readConcurrentPointerV1(storeRoot:string,scope:string):ConcurrentPointerV1|null{
 return readConcurrentPointersV1(storeRoot).find(pointer=>pointer.scope===scope)??null;
}
/** 🔀️ Replaces one scope's pointer atomically: the bytes become visible under the final name only after they are durable, so a concurrent reader sees the complete old or the complete new pointer and never a mixture. */
export function writeConcurrentPointerV1(storeRoot:string,pointer:ConcurrentPointerV1):void{
 const directory=pointerDirectory(storeRoot),name=concurrentPointerNameV1(pointer.scope),temporary=join(directory,name+"."+process.pid+"."+randomBytes(6).toString("hex")+".tmp");
 mkdirSync(directory,{recursive:true});
 const facts=lstatSync(directory);
 if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("Physical pointer directory required");
 try{
  writeFileSync(temporary,encodeConcurrentPointerV1(pointer),{flag:"wx",mode:0o600});
  const file=openSync(temporary,"r+");
  try{fsyncSync(file);}finally{closeSync(file);}
  renameSync(temporary,join(directory,name));
  if(process.platform!=="win32"){const parent=openSync(directory,"r");try{fsyncSync(parent);}finally{closeSync(parent);}}
 }finally{rmSync(temporary,{force:true});}
}
/** 🌡️ Reports the free fraction of the filesystem that holds the generations, or 1 when the platform cannot tell. */
export function concurrentFreeFractionV1(path:string):number{
 try{const facts=statfsSync(path);return facts.blocks>0?Number(facts.bavail)/Number(facts.blocks):1;}catch{return 1;}
}
