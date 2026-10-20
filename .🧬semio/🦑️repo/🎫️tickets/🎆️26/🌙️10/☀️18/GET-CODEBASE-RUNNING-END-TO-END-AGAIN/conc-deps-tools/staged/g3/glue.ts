import {randomBytes} from "node:crypto";
import {closeSync,fsyncSync,lstatSync,mkdirSync,openSync,readdirSync,readFileSync,readlinkSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {nativeStatfsV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {dirname,join,resolve,sep} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
import {acquireResourceLease,type ResourceLease,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {concurrentEventV1,concurrentLanguageV1,concurrentLeaseResourceV1,concurrentPinNamesV1,concurrentPinResourceV1,concurrentPointerNameV1,concurrentProtocolV1 as protocol,concurrentStaleTemporariesV1,concurrentTextV1,concurrentWaitStepV1,encodeConcurrentPointerV1,parseConcurrentPointerV1,type ConcurrentEventV1,type ConcurrentLeaseKindV1,type ConcurrentPointerV1} from "../🟦️.ts";

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

/** 📌️ Acquires shared pins on the given generations and returns them; the caller releases them, in reverse order, when its use of the generations ends. */
export async function acquireConcurrentPinsV1(directory:string,workspace:string,generations:readonly string[],control:ResourceLeaseControl|undefined,signal:AbortSignal):Promise<ResourceLease[]>{
 const held:ResourceLease[]=[];
 try{for(const name of concurrentPinNamesV1(generations))held.push(await acquireResourceLease({directory,resource:concurrentPinResourceV1(workspace,name),mode:"shared",signal,control,timeoutMs:protocol.wait.maximumWriterMilliseconds}));}
 catch(error){for(const lease of held.reverse())try{lease.release();}catch{}throw error;}
 return held;
}
/** 📌️ Holds shared pins on the given generations while `body` consumes them; a collector can never remove a pinned generation. */
export async function withConcurrentPinsV1<T>(directory:string,workspace:string,generations:readonly string[],control:ResourceLeaseControl|undefined,signal:AbortSignal,body:()=>Promise<T>):Promise<T>{
 const held=await acquireConcurrentPinsV1(directory,workspace,generations,control,signal);
 let failed=false,failure:unknown,value:T|undefined;
 try{value=await body();}catch(error){failed=true;failure=error;}
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
/** 🌡️ Reports the free fraction of the volume that holds the generations, measured now: the mounted image on macOS, the declared envelope minus the measured tree on a directory partition; an unreadable volume is refused with a bilingual diagnostic instead of being assumed to be empty or unlimited. */
export async function concurrentFreeFractionV1(path:string,read:ConcurrentVolumeReaderV1=readConcurrentVolumeV1):Promise<number>{
 const facts=await measureConcurrentVolumeV1(path,read);return facts.blocks>0?facts.bavail/facts.blocks:1;
}
/** 🔌️ Reads the volume that holds the generations; replaceable so an unreadable volume is testable on any host. */
export type ConcurrentVolumeReaderV1=(path:string)=>Promise<Readonly<{blocks:number;bavail:number;bsize:number}>>;
const readConcurrentVolumeV1:ConcurrentVolumeReaderV1=path=>nativeStatfsV1(path,{fresh:true});
async function measureConcurrentVolumeV1(path:string,read:ConcurrentVolumeReaderV1){
 try{return await read(path);}catch(error){throw Error(concurrentTextV1(concurrentLanguageV1(process.env),"freeSpaceUnreadable",{path,reason:(error as Error).message}),{cause:error});}
}
/** 🌡️ Reports the bytes a writer can still use on that volume, measured now, so a before-and-after difference is the size of what an install built; an unreadable volume is refused, never reported as unlimited. */
export async function concurrentFreeBytesV1(path:string,read:ConcurrentVolumeReaderV1=readConcurrentVolumeV1):Promise<number>{
 const facts=await measureConcurrentVolumeV1(path,read);return facts.bavail*facts.bsize;
}
/** 🔗️ Tells whether any binding of a generation's ready receipt currently resolves into it; a ready receipt that cannot be read counts as referenced, only an absent one (a generation under construction) does not. */
export function concurrentGenerationReferencedV1(container:string,name:string):boolean{
 const path=join(container,name,"bindings.json"),prefix=join(container,name)+sep;
 let bindings:unknown;
 try{
  const facts=lstatSync(path,{throwIfNoEntry:false});
  if(!facts)return false;
  if(!facts.isFile()||facts.size>4194304)return true;
  bindings=(JSON.parse(readFileSync(path,"utf8"))as{bindings?:unknown}).bindings;
 }catch{return true;}
 if(!Array.isArray(bindings))return true;
 for(const row of bindings){
  const source=(row as {sourceDirectory?:unknown}|null)?.sourceDirectory;
  if(typeof source!=="string")continue;
  try{const facts=lstatSync(source);if(facts.isSymbolicLink()&&(resolve(dirname(source),readlinkSync(source))+sep).startsWith(prefix))return true;}catch{}
 }
 return false;
}
/** 🧽️ Removes the stale scratch directories of the image, one by one with the caller's cancellation check, and reports how many went; a directory that cannot be removed stays and only costs disk. */
export function sweepConcurrentTemporariesV1(storeRoot:string,nowMs:number,pressure:boolean,check:()=>void):number{
 const directory=join(storeRoot,protocol.temporaries.directory),rows:{name:string;modifiedAtMs:number}[]=[];
 let names:string[];
 try{names=readdirSync(directory);}catch{return 0;}
 for(const name of names)try{const facts=lstatSync(join(directory,name));if(facts.isDirectory()&&!facts.isSymbolicLink())rows.push({name,modifiedAtMs:facts.mtimeMs});}catch{}
 let removed=0;
 for(const name of concurrentStaleTemporariesV1(rows,nowMs,pressure)){check();try{rmSync(join(directory,name),{recursive:true,force:true});removed++;}catch{}}
 return removed;
}
