import{lstat as statEntry,readdir as listEntries}from"node:fs/promises";
import{lstatSync,readdirSync}from"node:fs";
import{join,resolve}from"node:path";
import{nativeDirectoryProtocolV1}from"../📋️contract/🟦️.ts";

const protocol=nativeDirectoryProtocolV1();
/** 📊️ What one measured tree holds: allocated blocks, summed file sizes and entries visited. */
export type NativeDirectoryUsageRowV1=Readonly<{allocatedBytes:number;logicalBytes:number;entries:number}>;
type EntryStatV1=Readonly<{isFile():boolean;isDirectory():boolean;nlink:bigint;blocks:bigint;size:bigint;dev:bigint;ino:bigint}>;
/** 🔌️ The two filesystem calls of the walk, replaceable so races and failures are testable on any platform. */
export type NativeDirectoryWalkPortV1=Readonly<{list(path:string):Promise<readonly string[]>;stat(path:string):Promise<EntryStatV1>}>;
const systemWalk:NativeDirectoryWalkPortV1=Object.freeze({list:path=>listEntries(path),stat:path=>statEntry(path,{bigint:true})});
const vanished=(error:unknown)=>{const code=(error as NodeJS.ErrnoException)?.code;return code==="ENOENT"||code==="ENOTDIR";};

/** 📐️ The entries a tree may hold: what the envelope can store at one entry per allocation unit, times a slack factor, never below the floor, so a legitimate full partition is never refused while a runaway producer still is. */
export function nativeDirectoryEntryBoundV1(maximumPhysicalBytes:number):number{
 if(!Number.isSafeInteger(maximumPhysicalBytes)||maximumPhysicalBytes<1)throw Error("Positive physical envelope required");
 const row=protocol.usage;return Math.max(protocol.maximumEntries,Math.ceil(maximumPhysicalBytes/row.blockBytes)*row.entryFactor);
}

/** 🧮️ Measures physical bytes below a root without blocking the event loop: bounded concurrent lstat and readdir, links not followed, a hard link counted once, a vanished entry skipped because it no longer occupies space; the signal stops the walk between batches. */
export async function measureNativeDirectoryUsageAsyncV1(root:string,options:Readonly<{maximumEntries?:number;signal?:AbortSignal;port?:NativeDirectoryWalkPortV1}>={}):Promise<NativeDirectoryUsageRowV1>{
 const maximumEntries=options.maximumEntries??protocol.maximumEntries,signal=options.signal,port=options.port??systemWalk,row=protocol.usage;
 if(!Number.isSafeInteger(maximumEntries)||maximumEntries<1)throw Error("Positive entry bound required");
 const seen=new Set<string>(),stack:{path:string;depth:number}[]=[],waiting:(()=>void)[]=[];
 let allocatedBytes=0,logicalBytes=0,entries=0,active=0,failure:unknown;
 const wake=()=>{for(const resume of waiting.splice(0))resume();};
 const take=(path:string,depth:number,stat:EntryStatV1)=>{
  if(++entries>maximumEntries)throw Error("Directory usage entry bound exceeded");
  if(stat.isFile()&&stat.nlink>1n){const key=stat.dev+":"+stat.ino;if(seen.has(key))return;seen.add(key);}
  allocatedBytes+=Number(stat.blocks)*512;if(stat.isFile())logicalBytes+=Number(stat.size);
  if(stat.isDirectory()){if(depth>=protocol.maximumDepth)throw Error("Directory usage depth bound exceeded");stack.push({path,depth});wake();}
 };
 const next=async():Promise<{path:string;depth:number}|undefined>=>{
  for(;;){
   if(failure!==undefined)return undefined;
   const item=stack.pop();if(item){active++;return item;}
   if(active===0)return undefined;
   await new Promise<void>(resume=>waiting.push(resume));
  }
 };
 take(resolve(root),0,await port.stat(resolve(root)));
 const worker=async()=>{
  for(let item=await next();item;item=await next()){
   try{
    signal?.throwIfAborted();
    let names:readonly string[];try{names=await port.list(item.path);}catch(error){if(vanished(error))continue;throw error;}
    for(let index=0;index<names.length&&failure===undefined;index+=row.batch){
     signal?.throwIfAborted();
     const results=await Promise.all(names.slice(index,index+row.batch).map(async name=>{const path=join(item.path,name);try{return{path,stat:await port.stat(path)};}catch(error){if(vanished(error))return undefined;throw error;}}));
     for(const result of results)if(result)take(result.path,item.depth+1,result.stat);
    }
   }catch(error){failure??=error;}finally{active--;wake();}
  }
 };
 await Promise.all(Array.from({length:row.concurrency},worker));
 if(failure!==undefined)throw failure;
 if(!Number.isSafeInteger(allocatedBytes))throw Error("Directory usage overflow");
 return Object.freeze({allocatedBytes,logicalBytes,entries});
}

/** 🧮️ Measures physical bytes below a root synchronously; reserved for callers that cannot await. */
export function measureNativeDirectoryUsageV1(root:string,maximumEntries:number=protocol.maximumEntries):NativeDirectoryUsageRowV1{
 if(!Number.isSafeInteger(maximumEntries)||maximumEntries<1)throw Error("Positive entry bound required");
 const seen=new Set<string>(),stack:{path:string;depth:number}[]=[{path:resolve(root),depth:0}];let allocatedBytes=0,logicalBytes=0,entries=0;
 for(let item=stack.pop();item;item=stack.pop()){
  let row;try{row=lstatSync(item.path,{bigint:true});}catch(error){if(item.depth>0&&vanished(error))continue;throw error;}
  if(++entries>maximumEntries)throw Error("Directory usage entry bound exceeded");
  if(row.isFile()&&row.nlink>1n){const key=row.dev+":"+row.ino;if(seen.has(key))continue;seen.add(key);}
  allocatedBytes+=Number(row.blocks)*512;logicalBytes+=row.isFile()?Number(row.size):0;
  if(row.isDirectory()){if(item.depth>=protocol.maximumDepth)throw Error("Directory usage depth bound exceeded");let names:string[];try{names=readdirSync(item.path);}catch(error){if(vanished(error))continue;throw error;}for(const name of names)stack.push({path:join(item.path,name),depth:item.depth+1});}
 }
 if(!Number.isSafeInteger(allocatedBytes))throw Error("Directory usage overflow");return Object.freeze({allocatedBytes,logicalBytes,entries});
}

/** 🧾️ The shared measured usage of one tree: a measurement that started after the caller arrived, a cached one no older than a bound, the last known number and a way to record one. */
export type NativeDirectoryUsageV1=Readonly<{
 fresh(signal?:AbortSignal):Promise<number>;
 recent(maximumAgeMilliseconds:number,signal?:AbortSignal):Promise<number>;
 peek():Readonly<{bytes:number;atMs:number}>|undefined;
 record(bytes:number,atMs?:number):void;
}>;
type Slot={readonly controller:AbortController;readonly promise:Promise<number>;subscribers:number;settled:boolean;start():void;cancel(reason:unknown):void};

/** 🛫️ Builds one usage cell. Measurements never overlap: a caller that arrives while one runs waits for it and for exactly one trailing run shared by every such caller, so any number of simultaneous commits cost at most two walks and every answer reflects a walk that began after the caller's own write. A walk whose last waiter leaves is cancelled. */
export function createNativeDirectoryUsageV1(input:Readonly<{root:string;maximumEntries:number;measure?:(root:string,signal:AbortSignal)=>Promise<number>;now?:()=>number}>):NativeDirectoryUsageV1{
 const now=input.now??Date.now,measure=input.measure??((root:string,signal:AbortSignal)=>measureNativeDirectoryUsageAsyncV1(root,{maximumEntries:input.maximumEntries,signal}).then(row=>row.allocatedBytes));
 let known:{bytes:number;atMs:number}|undefined,running:Slot|undefined,queued:Slot|undefined;
 const record=(bytes:number,atMs:number=now())=>{if(!known||atMs>=known.atMs)known={bytes,atMs};};
 const handoff=(done:Slot)=>{
  if(running!==done)return;running=undefined;const following=queued;queued=undefined;
  if(!following)return;
  if(following.subscribers>0){running=following;following.start();}else following.cancel(Error("Directory usage measurement abandoned"));
 };
 const slot=():Slot=>{
  const controller=new AbortController();let started=false,resolveRun!:(bytes:number)=>void,rejectRun!:(error:unknown)=>void;
  const promise=new Promise<number>((done,fail)=>{resolveRun=done;rejectRun=fail;});promise.catch(()=>undefined);
  const self:Slot={controller,promise,subscribers:0,settled:false,
   start(){if(started)return;started=true;const atMs=now();new Promise<number>(begin=>begin(measure(input.root,controller.signal))).then(bytes=>{record(bytes,atMs);resolveRun(bytes);},rejectRun).finally(()=>{self.settled=true;handoff(self);});},
   cancel(reason){if(self.settled)return;if(started)controller.abort(reason);else{started=true;self.settled=true;rejectRun(reason);}}};
  return self;
 };
 const join=(target:Slot,signal?:AbortSignal):Promise<number>=>{
  target.subscribers++;let left=false;
  const leave=()=>{if(left)return false;left=true;target.subscribers--;return true;};
  return new Promise<number>((resolveCall,rejectCall)=>{
   const abort=()=>{if(leave()&&target.subscribers<=0)target.cancel(signal!.reason);rejectCall(signal!.reason);};
   if(signal?.aborted){abort();return;}
   signal?.addEventListener("abort",abort,{once:true});
   target.promise.then(value=>{signal?.removeEventListener("abort",abort);if(leave())resolveCall(value);},error=>{signal?.removeEventListener("abort",abort);if(leave())rejectCall(error);});
  });
 };
 const fresh=(signal?:AbortSignal):Promise<number>=>{
  if(!running){running=slot();running.start();return join(running,signal);}
  if(queued?.settled)queued=undefined;
  queued??=slot();return join(queued,signal);
 };
 return Object.freeze({fresh,recent:(maximumAgeMilliseconds:number,signal?:AbortSignal)=>known&&now()-known.atMs<=maximumAgeMilliseconds?Promise.resolve(known.bytes):fresh(signal),peek:()=>known?Object.freeze({...known}):undefined,record});
}

const registry=new Map<string,NativeDirectoryUsageV1>();
/** 🗂️ Returns the process-wide usage cell of a partition root, so the capacity meter and every space query share one measurement and one cache. */
export function nativeDirectoryUsageV1(root:string,maximumPhysicalBytes:number):NativeDirectoryUsageV1{
 const key=resolve(root);let cell=registry.get(key);
 if(!cell){cell=createNativeDirectoryUsageV1({root:key,maximumEntries:nativeDirectoryEntryBoundV1(maximumPhysicalBytes)});registry.set(key,cell);}
 return cell;
}
