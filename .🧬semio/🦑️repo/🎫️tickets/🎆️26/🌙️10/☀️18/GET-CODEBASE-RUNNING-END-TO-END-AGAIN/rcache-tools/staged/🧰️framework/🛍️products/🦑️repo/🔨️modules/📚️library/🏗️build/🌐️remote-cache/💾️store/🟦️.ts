import {createHash,randomUUID} from "node:crypto";
import {link,mkdir,open,readdir,rename,rm,stat,unlink,utimes} from "node:fs/promises";
import {join} from "node:path";

/** 🗂️ Names the three keyspaces one store partitions: Bazel action results, Bazel content blobs and Nx task archives. */
export type RemoteCacheKindV1="ac"|"cas"|"nx";

/** 📮️ Reports why one write ended: stored, replaced an older value, found an equal or protected value, or was refused. */
export type RemoteCachePutOutcomeV1="created"|"replaced"|"exists"|"digest-mismatch"|"length-mismatch"|"too-large"|"capacity"|"aborted";

/** 📡️ Progress and housekeeping events of one store, published to the caller without any logging dependency. */
export type RemoteCacheStoreEventV1=Readonly<{stage:"scan";entries:number;bytes:number}>|Readonly<{stage:"evict";entries:number;bytes:number}>|Readonly<{stage:"ready";entries:number;bytes:number}>;

/** 🧱️ Authors the finite bounds of one store: total bytes, bytes per entry and the idle age after which an entry is dropped (0 keeps entries until space is needed). */
export type RemoteCacheStoreOptionsV1=Readonly<{directory:string;maximumBytes:number;maximumEntryBytes:number;maximumAgeMilliseconds:number;signal?:AbortSignal;onEvent?:(event:RemoteCacheStoreEventV1)=>void}>;

/** 📏️ Describes the current fill of one store. */
export type RemoteCacheStoreUsageV1=Readonly<{entries:number;bytes:number;reservedBytes:number;maximumBytes:number;maximumEntryBytes:number}>;

/** 📤️ One stored value ready to be streamed. */
export type RemoteCacheEntryV1=Readonly<{path:string;size:number}>;

/** 🔑️ Names how a write treats an existing value: `replace` overwrites, `keep` refuses to overwrite and `content` is idempotent because the key is the content digest. */
export type RemoteCachePutModeV1="replace"|"keep"|"content";

const NAMESPACE=/^[a-z0-9][a-z0-9._-]{0,63}$/u,KEYS:Readonly<Record<RemoteCacheKindV1,RegExp>>={ac:/^[0-9a-f]{64}$/u,cas:/^[0-9a-f]{64}$/u,nx:/^[a-z0-9_-]{1,128}$/u},LOW_WATER_NUMERATOR=9,LOW_WATER_DENOMINATOR=10;

/** 🔎️ Admits only namespaces and keys that map to exactly one portable file name. */
export function requireRemoteCacheAddressV1(namespace:string,kind:RemoteCacheKindV1,key:string):void{
 if(!NAMESPACE.test(namespace))throw Error("Lowercase remote cache namespace required");
 if(!KEYS[kind]?.test(key))throw Error(`Remote cache key for ${kind} required`);
}

/** 🏛️ Stores remote cache values as files in a bounded directory with atomic writes, content verification and least-recently-used eviction. */
export class RemoteCacheStoreV1{
 private readonly index=new Map<string,{size:number;accessed:number}>();
 private bytes=0;
 private reserved=0;
 private clock=0;
 private collecting:Promise<void>=Promise.resolve();
 private constructor(private readonly options:RemoteCacheStoreOptionsV1){}

 /** 🚪️ Opens the directory, drops leftover partial writes and indexes the stored values, publishing progress and stopping when cancelled. */
 static async open(options:RemoteCacheStoreOptionsV1):Promise<RemoteCacheStoreV1>{
  if(![options.maximumBytes,options.maximumEntryBytes].every(value=>Number.isSafeInteger(value)&&value>0)||options.maximumEntryBytes>options.maximumBytes)throw Error("Finite remote cache bounds required");
  const store=new RemoteCacheStoreV1(options);
  await mkdir(store.root,{recursive:true});await rm(store.temporary,{recursive:true,force:true});await mkdir(store.temporary,{recursive:true});
  await store.scan();await store.collect();options.onEvent?.({stage:"ready",entries:store.index.size,bytes:store.bytes});return store;
 }

 private get root():string{return join(this.options.directory,"v1");}
 private get temporary():string{return join(this.root,".tmp");}
 private relative(namespace:string,kind:RemoteCacheKindV1,key:string):string{return join(namespace,kind,key.slice(0,2),key);}
 private checkpoint():void{this.options.signal?.throwIfAborted();}
 private tick():number{return this.clock=Math.max(Date.now(),this.clock+1);}

 private async scan():Promise<void>{
  let visited=0;
  const walk=async(directory:string,depth:number,prefix:string[]):Promise<void>=>{
   this.checkpoint();
   for(const row of await readdir(directory,{withFileTypes:true})){
    if(depth===0&&row.name===".tmp")continue;
    if(depth<3){if(row.isDirectory())await walk(join(directory,row.name),depth+1,[...prefix,row.name]);continue;}
    if(!row.isFile())continue;
    const info=await stat(join(directory,row.name)),relative=join(...prefix,row.name);
    this.index.set(relative,{size:info.size,accessed:info.mtimeMs});this.clock=Math.max(this.clock,info.mtimeMs);this.bytes+=info.size;
    if(++visited%256===0)this.options.onEvent?.({stage:"scan",entries:this.index.size,bytes:this.bytes});
   }
  };
  await walk(this.root,0,[]);
  this.options.onEvent?.({stage:"scan",entries:this.index.size,bytes:this.bytes});
 }

 /** 📊️ Reports the current fill. */
 usage():RemoteCacheStoreUsageV1{return Object.freeze({entries:this.index.size,bytes:this.bytes,reservedBytes:this.reserved,maximumBytes:this.options.maximumBytes,maximumEntryBytes:this.options.maximumEntryBytes});}

 /** 🔍️ Looks one value up and marks it recently used. */
 read(namespace:string,kind:RemoteCacheKindV1,key:string):RemoteCacheEntryV1|undefined{
  requireRemoteCacheAddressV1(namespace,kind,key);
  const relative=this.relative(namespace,kind,key),row=this.index.get(relative);if(!row)return undefined;
  const now=this.tick(),path=join(this.root,relative);row.accessed=now;void utimes(path,now/1000,now/1000).catch(()=>{});
  return Object.freeze({path,size:row.size});
 }

 /** 🧪️ Tells whether a value exists without touching it. */
 has(namespace:string,kind:RemoteCacheKindV1,key:string):boolean{requireRemoteCacheAddressV1(namespace,kind,key);return this.index.has(this.relative(namespace,kind,key));}

 /** 🧹️ Drops values older than the authored idle age, then least-recently-used values until the fill is at most nine tenths of the bound; calls queue behind each other. */
 collect(need:number=0):Promise<void>{
  const run=async()=>{
   const maximum=this.options.maximumBytes,age=this.options.maximumAgeMilliseconds,now=Date.now();let removedEntries=0,removedBytes=0;
   const drop=async(relative:string,row:{size:number}):Promise<void>=>{
    try{await unlink(join(this.root,relative));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")return;}
    this.index.delete(relative);this.bytes-=row.size;removedEntries++;removedBytes+=row.size;
   };
   if(age>0)for(const[relative,row]of[...this.index])if(now-row.accessed>age)await drop(relative,row);
   const target=Math.floor(maximum*LOW_WATER_NUMERATOR/LOW_WATER_DENOMINATOR)-need;
   if(this.bytes+this.reserved>maximum-need){
    const order=[...this.index].sort((left,right)=>left[1].accessed-right[1].accessed);
    for(const[relative,row]of order){if(this.bytes+this.reserved<=target)break;this.checkpoint();await drop(relative,row);}
   }
   if(removedEntries)this.options.onEvent?.({stage:"evict",entries:removedEntries,bytes:removedBytes});
  };
  this.collecting=this.collecting.then(run,run);return this.collecting;
 }

 /** ✍️ Streams one value into the store: the body is bounded, hashed when `verify` is set, written to a temporary file and then published atomically; nothing partial is ever visible. */
 async put(namespace:string,kind:RemoteCacheKindV1,key:string,body:AsyncIterable<Uint8Array>|null,input:Readonly<{mode:RemoteCachePutModeV1;verify?:boolean;declaredLength?:number|undefined}>):Promise<RemoteCachePutOutcomeV1>{
  requireRemoteCacheAddressV1(namespace,kind,key);
  const relative=this.relative(namespace,kind,key),declared=input.declaredLength;
  if(input.mode!=="replace"&&this.index.has(relative))return "exists";
  if(declared!==undefined&&declared>this.options.maximumEntryBytes)return "too-large";
  const temporary=join(this.temporary,randomUUID()),hash=input.verify?createHash("sha256"):undefined;let held=0,written=0,handle;
  const reserve=async(extra:number):Promise<boolean>=>{
   if(this.bytes+this.reserved+extra>this.options.maximumBytes){await this.collect(extra);if(this.bytes+this.reserved+extra>this.options.maximumBytes)return false;}
   this.reserved+=extra;held+=extra;return true;
  };
  try{
   if(declared!==undefined&&!await reserve(declared))return "capacity";
   handle=await open(temporary,"wx");
   if(body){
    let outcome:RemoteCachePutOutcomeV1|undefined;
    try{
     for await(const chunk of body){
      this.checkpoint();written+=chunk.byteLength;
      if(written>this.options.maximumEntryBytes){outcome="too-large";break;}
      if(written>held&&!await reserve(written-held)){outcome="capacity";break;}
      hash?.update(chunk);await handle.write(chunk);
     }
    }catch(error){if((error as NodeJS.ErrnoException).syscall!==undefined)throw error;outcome="aborted";}
    if(outcome)return outcome;
   }
   await handle.close();handle=undefined;
   if(declared!==undefined&&written!==declared)return "length-mismatch";
   if(hash&&hash.digest("hex")!==key)return "digest-mismatch";
   await mkdir(join(this.root,namespace,kind,key.slice(0,2)),{recursive:true});
   const final=join(this.root,relative),previous=this.index.get(relative);
   if(input.mode==="keep"){try{await link(temporary,final);}catch(error){if((error as NodeJS.ErrnoException).code==="EEXIST")return "exists";throw error;}}
   else await rename(temporary,final);
   if(previous)this.bytes-=previous.size;this.bytes+=written;this.index.set(relative,{size:written,accessed:this.tick()});
   return previous?"replaced":"created";
  }finally{
   this.reserved-=held;await handle?.close().catch(()=>{});await unlink(temporary).catch(()=>{});
   if(this.bytes+this.reserved>this.options.maximumBytes)void this.collect().catch(()=>{});
  }
 }

 /** 🔚️ Removes the partial-write area; stored values stay for the next start. */
 async close():Promise<void>{await this.collecting.catch(()=>{});await rm(this.temporary,{recursive:true,force:true});}
}
