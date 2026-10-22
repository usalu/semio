import {chmod,lstat,mkdir,readdir,readFile,rename,rmdir,unlink,writeFile} from "node:fs/promises";
import {nativeImageBackingFilenameV1,nativeImageDirectoryHostV1,nativePartitionOfV1,nativeStatfsV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {isAbsolute,join,relative,resolve,sep} from "node:path";
import ticket from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json" with {type:"json"};
import budgetPolicy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {CACHE_RUNTIME_V1,CACHE_SIZE_FILE_V1,cacheAreaBudgetBytesV1,cacheAreaDirectoryV1,cacheRootDirectoryV1,cacheTextV1,classifyCachePathV1,formatCacheBytesV1,formatCacheReasonV1,planCacheCollectionV1,readCacheSizeLedgerV1,type CacheAreaRowV1,type CacheEntryRowV1,type CacheLanguageV1,type CacheLedgerV1,type CachePartitionBudgetsV1,type CachePartitionRowV1,type CacheRemovalV1,type CacheReportV1,type CacheRootIdV1,type CacheSizeLedgerV1,type CacheSizeRecordV1,type CacheSourceV1,type CacheTargetPlanV1,type CacheTargetV1,type CacheUnitV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";

const runtime=CACHE_RUNTIME_V1,protocol=ticket.definitions.Protocol.const,mount="m",tickets=new RegExp(protocol.ticketPattern,"u");

/** 🔌️ Everything the runner needs from its host: cancellation, a clock, progress, the exclusive non-blocking lease and the liveness probes; tests replace them. */
export type CacheRunnerPortsV1=Readonly<{
 signal:AbortSignal;
 now():number;
 progress(stage:string):Promise<void>;
 exclusive(resource:string):Promise<Readonly<{release():void}>|undefined>;
 alive(pid:number):boolean;
 sqliteFree(file:string):Promise<boolean>;
 platform?:NodeJS.Platform;
 delegate?(owner:string,areaId:string,dryRun:boolean,language:CacheLanguageV1):Promise<string|undefined>;
}>;

/** 🧟️ Asks the operating system whether a process still exists; a process we may not signal still exists. */
export function cachePidAliveV1(pid:number):boolean{
 try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}
}

/** 🔐️ Proves a SQLite lease file is unheld: an exclusive transaction without waiting only succeeds when no process holds a shared or exclusive lock. */
export async function cacheSqliteFreeV1(file:string):Promise<boolean>{
 const {Database}=await import("bun:sqlite");let database:InstanceType<typeof Database>|undefined;
 try{database=new Database(file,{create:false,readwrite:true});database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");database.exec("ROLLBACK");return true;}
 catch(error){return !/locked|busy/iu.test(String((error as Error).message));}
 finally{try{database?.close();}catch{}}
}

/** 🏭️ Completes the host ports with the portable defaults. */
export function cacheRunnerPortsV1(ports:Pick<CacheRunnerPortsV1,"signal"|"progress"|"exclusive">&Partial<CacheRunnerPortsV1>):CacheRunnerPortsV1{
 return Object.freeze({now:()=>Date.now(),alive:cachePidAliveV1,sqliteFree:cacheSqliteFreeV1,...ports});
}

type Ticker=()=>Promise<void>;
function ticker(ports:CacheRunnerPortsV1,stage:()=>string):Ticker{
 let last=-Infinity;
 return async()=>{ports.signal.throwIfAborted();const now=ports.now();if(now-last>=runtime.progressIntervalMs){last=now;await ports.progress(stage());ports.signal.throwIfAborted();}};
}

/** 📏️ What one bounded walk found: allocated bytes, newest file time, entries visited, and the lease files and queues inside. */
export type CacheMeasureV1=Readonly<{present:boolean;bytes:number;recencyMs:number;entries:number;partial:boolean;sqlite:readonly string[];queues:readonly string[]}>;

/** 🚶️ Walks one tree without following links, counts hardlinks once, takes recency from file times only (directory times only when no file exists), and stops at the entry limit or on cancellation. */
export async function measureCacheTreeV1(path:string,ports:CacheRunnerPortsV1,limit:number,stage:string=""):Promise<CacheMeasureV1>{
 let first;try{first=await lstat(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{present:false,bytes:0,recencyMs:0,entries:0,partial:false,sqlite:[],queues:[]};throw error;}
 const allocated=(info:{size:number;blocks?:number}):number=>info.blocks?info.blocks*512:info.size,seen=new Set<string>(),sqlite:string[]=[],queues:string[]=[],tick=ticker(ports,()=>stage);
 if(!first.isDirectory())return{present:true,bytes:first.isSymbolicLink()?0:allocated(first),recencyMs:Math.max(first.mtimeMs,first.atimeMs),entries:1,partial:false,sqlite:path.endsWith(".sqlite")?[path]:[],queues:[]};
 let bytes=0,files=0,entries=0,partial=false;const stack=[path];let recency=0,directories=first.mtimeMs;
 while(stack.length){
  await tick();const directory=stack.pop()!;let children;
  try{children=await readdir(directory,{withFileTypes:true});}catch(error){if(["ENOENT","ENOTDIR","EACCES","EPERM"].includes((error as NodeJS.ErrnoException).code??""))continue;throw error;}
  if(directory.endsWith(".queue")&&queues.length<runtime.sqliteProbesPerUnit)queues.push(directory);
  for(const child of children){
   if(++entries>limit){partial=true;stack.length=0;break;}
   if(entries%runtime.checkEvery===0)await tick();
   const full=join(directory,child.name);
   if(child.isDirectory()){stack.push(full);try{directories=Math.max(directories,(await lstat(full)).mtimeMs);}catch{}continue;}
   let info;try{info=await lstat(full);}catch{continue;}
   if(info.isSymbolicLink())continue;
   if(info.nlink>1){const exact=Number.isSafeInteger(info.ino)?info:await lstat(full,{bigint:true}).catch(()=>info),key=exact.dev+":"+exact.ino;if(seen.has(key))continue;seen.add(key);}
   bytes+=allocated(info);files++;recency=Math.max(recency,info.mtimeMs,info.atimeMs);
   if(child.name.endsWith(".sqlite")&&sqlite.length<runtime.sqliteProbesPerUnit)sqlite.push(full);
  }
 }
 return{present:true,bytes,recencyMs:files?recency:directories,entries,partial,sqlite,queues};
}

/** 🔒️ Names what holds a unit: a SQLite lease file with a live lock, or a queue ticket whose process is alive; nothing means nobody. */
export async function probeCacheHeldV1(measure:CacheMeasureV1,ports:CacheRunnerPortsV1):Promise<string|undefined>{
 for(const file of measure.sqlite){ports.signal.throwIfAborted();if(!await ports.sqliteFree(file))return "sqlite:"+file;}
 for(const queue of measure.queues){
  ports.signal.throwIfAborted();let names:string[];try{names=await readdir(queue);}catch{continue;}
  for(const name of names)if(tickets.test(name)){const pid=Number.parseInt(name.split("-")[1]!,protocol.pidBase);if(Number.isSafeInteger(pid)&&pid>0&&ports.alive(pid))return "pid:"+pid;}
 }
 return undefined;
}

const exists=async(path:string):Promise<boolean>=>lstat(path).then(()=>true,()=>false);

async function partitionStateV1(directory:string,ports:CacheRunnerPortsV1,deep:boolean):Promise<Readonly<{state:CachePartitionRowV1["state"];usedBytes:number|null;capacityBytes:number|null;partial:boolean;walked:boolean}>>{
 let outer;try{outer=await lstat(directory);}catch{return{state:"absent",usedBytes:null,capacityBytes:null,partial:false,walked:false};}
 const inner=join(directory,mount);let child;try{child=await lstat(inner);}catch{child=undefined;}
 const platform=ports.platform??process.platform,backing=nativeImageBackingFilenameV1(platform),mounted=nativeImageDirectoryHostV1(platform)?nativePartitionOfV1(inner,platform)?.attached===true:child?.isDirectory()===true&&child.dev!==outer.dev;
 if(mounted){const row=await nativeStatfsV1(inner,{platform});return{state:"mounted",usedBytes:(row.blocks-row.bfree)*row.bsize,capacityBytes:row.blocks*row.bsize,partial:false,walked:false};}
 if(await exists(join(directory,backing)))return{state:"detached",usedBytes:null,capacityBytes:null,partial:false,walked:false};
 if(!child)return{state:"absent",usedBytes:null,capacityBytes:null,partial:false,walked:false};
 if(!deep)return{state:"directory",usedBytes:null,capacityBytes:null,partial:false,walked:false};
 const measure=await measureCacheTreeV1(inner,ports,runtime.statusEntriesPerTarget,"partition");
 return{state:measure.present?"directory":"absent",usedBytes:measure.present?measure.bytes:null,capacityBytes:null,partial:measure.partial,walked:true};
}

/** 🪪️ Reads the physical size an image was born with from its `image.json`; no such record (directory hosts, absent image) is null. */
export async function readImageEnvelopeV1(directory:string):Promise<number|null>{
 try{const value=JSON.parse(await readFile(join(directory,"image.json"),"utf8")) as {maximumPhysicalBytes?:unknown};return Number.isSafeInteger(value.maximumPhysicalBytes)?value.maximumPhysicalBytes as number:null;}catch{return null;}
}

/** 📒️ Reads the size ledger of the repository cache root; a missing or damaged file is an empty ledger. */
export async function loadCacheSizesV1(workspace:string,ledger:CacheLedgerV1):Promise<CacheSizeLedgerV1>{
 try{return readCacheSizeLedgerV1(JSON.parse(await readFile(join(cacheRootDirectoryV1(workspace,ledger,"repository"),CACHE_SIZE_FILE_V1),"utf8")));}catch{return{version:1,records:{}};}
}

/** ✍️ Merges measured sizes into the size ledger (null forgets a key) and replaces the file atomically; losing a write only costs the next status a deep walk. */
export async function recordCacheSizesV1(workspace:string,ledger:CacheLedgerV1,updates:Readonly<Record<string,CacheSizeRecordV1|null>>):Promise<boolean>{
 if(!Object.keys(updates).length)return true;
 const directory=cacheRootDirectoryV1(workspace,ledger,"repository"),file=join(directory,CACHE_SIZE_FILE_V1),records:Record<string,CacheSizeRecordV1>={...(await loadCacheSizesV1(workspace,ledger)).records};
 for(const[key,value]of Object.entries(updates))if(value===null)delete records[key];else records[key]=value;
 const newest=Object.entries(records).sort((left,right)=>right[1].atMs-left[1].atMs||(left[0]<right[0]?-1:1)).slice(0,runtime.maximumSizeRecords);
 const temporary=file+"."+process.pid+".tmp";
 try{await mkdir(directory,{recursive:true});await writeFile(temporary,JSON.stringify({version:1,records:Object.fromEntries(newest.sort((left,right)=>left[0]<right[0]?-1:1))},null,1)+"\n");await rename(temporary,file);return true;}
 catch{await unlink(temporary).catch(()=>{});return false;}
}

type Sized=Readonly<{present:boolean;usedBytes:number|null;entries:number;partial:boolean;source:CacheSourceV1;recordedAtMs:number|null}>;
const MISSING:Sized={present:false,usedBytes:null,entries:0,partial:false,source:"unknown",recordedAtMs:null};

/** 📈️ Reads the ledger's status from the disk without taking a lease and, by default, without walking any tree: partitions come from the (virtual) statfs of their mounted image, every other size from the size ledger that collections and deep status write, existence from one lstat or readdir. `deep` walks everything with progress and cancellation, then records what it measured. */
export async function reportCacheLedgerV1(workspace:string,ledger:CacheLedgerV1,ports:CacheRunnerPortsV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions,only?:ReadonlySet<string>,options:Readonly<{deep:boolean}>={deep:false}):Promise<CacheReportV1>{
 const deep=options.deep,sizes=deep?{version:1 as const,records:{}}:await loadCacheSizesV1(workspace,ledger),fresh:Record<string,CacheSizeRecordV1>={},rows:CachePartitionRowV1[]=[],areaRows:CacheAreaRowV1[]=[],states=new Map<string,Awaited<ReturnType<typeof partitionStateV1>>>(),used=new Map<string,Sized>(),partial=new Map<CacheRootIdV1,boolean>(),selected=(id:string)=>only===undefined||only.has(id),now=ports.now();
 const sized=async(key:string,paths:readonly string[],stage:string):Promise<Sized>=>{
  if(deep){
   let bytes=0,entries=0,isPartial=false,present=false;
   for(const path of paths){const measure=await measureCacheTreeV1(path,ports,runtime.statusEntriesPerTarget,stage);bytes+=measure.bytes;entries+=measure.entries;isPartial=isPartial||measure.partial;present=present||measure.present;}
   if(present)fresh[key]={usedBytes:bytes,partial:isPartial,atMs:now};
   return{present,usedBytes:present?bytes:null,entries,partial:isPartial,source:present?"walked":"unknown",recordedAtMs:null};
  }
  let present=false;for(const path of paths)if(await exists(path)){present=true;break;}
  if(!present)return MISSING;
  const record=sizes.records[key];
  return record?{present:true,usedBytes:record.usedBytes,entries:0,partial:record.partial,source:"recorded",recordedAtMs:record.atMs}:{present:true,usedBytes:null,entries:0,partial:false,source:"unknown",recordedAtMs:null};
 };
 const matching=async(root:string,pattern:string):Promise<string[]>=>{const matcher=new RegExp(pattern,"u");try{return(await readdir(root)).filter(name=>matcher.test(name)).map(name=>join(root,name));}catch{return[];}};
 for(const area of ledger.areas.filter(area=>area.kind==="partition"||area.kind==="control"&&area.managed)){
  if(!selected(area.id))continue;
  await ports.progress("status:"+area.id);ports.signal.throwIfAborted();
  const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions)!,budgetBytes=cacheAreaBudgetBytesV1(ledger,area,partitions);
  if(area.kind==="partition"){
   const state=await partitionStateV1(directory,ports,deep);states.set(area.partition!,state);
   rows.push({id:area.id,kind:"partition",directory,state:state.state,usedBytes:state.usedBytes,budgetBytes,capacityBytes:state.capacityBytes,partial:state.partial,owner:area.managed!.owner,source:state.usedBytes===null?"unknown":state.walked?"walked":"statfs",recordedAtMs:null,envelopeBytes:await readImageEnvelopeV1(directory),declaredBytes:partitions[area.partition!]!.capacityBytes,recreate:area.recreate==="derived"});
  }else{
   const size=await sized(area.id,[directory],area.id);
   rows.push({id:area.id,kind:"control",directory,state:size.present?"directory":"absent",usedBytes:size.usedBytes,budgetBytes,capacityBytes:null,partial:size.partial,owner:area.managed!.owner,source:size.source,recordedAtMs:size.recordedAtMs,envelopeBytes:null,declaredBytes:null,recreate:false});
  }
 }
 const nested=ledger.areas.filter(area=>area.kind==="nested"&&selected(area.id)).sort((left,right)=>Number("remainder" in (left.budget??{}))-Number("remainder" in (right.budget??{})));
 for(const area of nested){
  await ports.progress("status:"+area.id);ports.signal.throwIfAborted();
  const state=states.get(area.partition!)??await partitionStateV1(join(workspace,partitions[area.partition!]!.directory),ports,deep),directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions)!,budget=cacheAreaBudgetBytesV1(ledger,area,partitions),remainder=area.budget&&"remainder" in area.budget,owner=area.managed!.owner;
  let row:CacheAreaRowV1;
  if(state.state==="detached"||state.state==="absent")row={id:area.id,root:area.root,partition:area.partition!,directory,present:false,measurable:false,usedBytes:null,budgetBytes:budget,entries:0,partial:false,over:false,owner,source:"unknown",recordedAtMs:null};
  else if(remainder){
   const siblings=nested.filter(other=>other.partition===area.partition&&other.id!==area.id).map(other=>used.get(other.id)),known=siblings.every(row=>row!==undefined&&row.usedBytes!==null),total=state.usedBytes;
   if(total===null||!known)row={id:area.id,root:area.root,partition:area.partition!,directory,present:true,measurable:total!==null,usedBytes:null,budgetBytes:budget,entries:0,partial:false,over:false,owner,source:"unknown",recordedAtMs:null};
   else{
    const own=Math.max(0,total-siblings.reduce((sum,row)=>sum+row!.usedBytes!,0)),stamps=siblings.map(row=>row!.recordedAtMs).filter((value):value is number=>value!==null),recorded=stamps.length>0;
    row={id:area.id,root:area.root,partition:area.partition!,directory,present:true,measurable:true,usedBytes:own,budgetBytes:budget,entries:0,partial:state.partial||siblings.some(row=>row!.partial),over:budget!==null&&own>budget,owner,source:recorded?"recorded":state.walked?"walked":"statfs",recordedAtMs:recorded?Math.min(...stamps):null};
   }
  }else{
   const size=await sized(area.id,[directory],area.id);used.set(area.id,size);
   row={id:area.id,root:area.root,partition:area.partition!,directory,present:size.present,measurable:true,usedBytes:size.usedBytes,budgetBytes:budget,entries:size.entries,partial:size.partial,over:budget!==null&&size.usedBytes!==null&&size.usedBytes>budget,owner,source:size.source,recordedAtMs:size.recordedAtMs};
  }
  areaRows.push(row);
 }
 const totals=new Map<CacheRootIdV1,number>([["repository",0],["native",0]]),retired:CacheEntryRowV1[]=[],undeclared:CacheEntryRowV1[]=[],count=(root:CacheRootIdV1,size:Sized)=>{totals.set(root,totals.get(root)!+(size.usedBytes??0));partial.set(root,(partial.get(root)??false)||size.partial||size.usedBytes===null);};
 for(const area of ledger.areas.filter(area=>(area.kind==="tree"||area.kind==="control"&&area.collect)&&selected(area.id))){
  await ports.progress("status:"+area.id);ports.signal.throwIfAborted();
  const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions),budget=cacheAreaBudgetBytesV1(ledger,area,partitions),size=await sized(area.id,directory!==null?[directory]:await matching(cacheRootDirectoryV1(workspace,ledger,area.root),area.pattern!),area.id);
  if(size.present)count(area.root,size);
  areaRows.push({id:area.id,root:area.root,partition:null,directory,present:size.present,measurable:true,usedBytes:size.usedBytes,budgetBytes:budget,entries:size.entries,partial:size.partial,over:budget!==null&&size.usedBytes!==null&&size.usedBytes>budget,owner:area.managed?.owner??null,source:size.source,recordedAtMs:size.recordedAtMs});
 }
 for(const retirement of ledger.retirements){
  if(only&&!only.has("retired"))continue;
  await ports.progress("status:"+retirement.directory);ports.signal.throwIfAborted();
  const size=await sized("retired:"+retirement.root+":"+retirement.directory,[join(cacheRootDirectoryV1(workspace,ledger,retirement.root),...retirement.directory.split("/"))],retirement.directory);
  if(!size.present)continue;
  retired.push({root:retirement.root,path:retirement.root+":"+retirement.directory,kind:"retired",usedBytes:size.usedBytes,entries:size.entries,partial:size.partial,source:size.source,recordedAtMs:size.recordedAtMs});count(retirement.root,size);
 }
 if(!only||only.has("undeclared"))for(const root of["repository","native"] as const){
  for(const entry of await undeclaredEntriesV1(workspace,ledger,root)){
   await ports.progress("status:"+entry);ports.signal.throwIfAborted();
   const size=await sized("undeclared:"+root+":"+entry,[join(cacheRootDirectoryV1(workspace,ledger,root),...entry.split("/"))],entry);
   undeclared.push({root,path:root+":"+entry,kind:"undeclared",usedBytes:size.usedBytes,entries:size.entries,partial:size.partial,source:size.source,recordedAtMs:size.recordedAtMs});count(root,size);
  }
 }
 if(deep)await recordCacheSizesV1(workspace,ledger,fresh);
 const roots=(["repository","native"] as const).map(id=>({id,directory:ledger.roots[id].directory,usedBytes:totals.get(id)!,budgetBytes:ledger.roots[id].maximumBytes,partial:partial.get(id)??false}));
 return{version:1,workspace,generatedAtMs:now,partitions:rows,areas:areaRows,roots,retired,undeclared};
}

/** 🕵️ Lists the root-relative entries nobody declared, descending only through containers of declared homes. */
export async function undeclaredEntriesV1(workspace:string,ledger:CacheLedgerV1,root:CacheRootIdV1):Promise<readonly string[]>{
 const base=cacheRootDirectoryV1(workspace,ledger,root),found:string[]=[],pending=[""];
 while(pending.length){
  const prefix=pending.shift()!;let names:string[];try{names=await readdir(prefix?join(base,...prefix.split("/")):base);}catch(error){if(prefix&&(error as NodeJS.ErrnoException).code==="ENOTDIR"){const path=root+":"+prefix;throw Error(cacheTextV1("en","error.container",{path})+"\n"+cacheTextV1("de","error.container",{path}));}continue;}
  for(const name of names.sort()){
   const path=prefix?prefix+"/"+name:name,kind=classifyCachePathV1(ledger,root,path).kind;
   if(kind==="undeclared")found.push(path);else if(kind==="container")pending.push(path);
  }
 }
 return found;
}

async function unlinkPath(path:string):Promise<void>{
 try{await unlink(path);}
 catch(error){
  const code=(error as NodeJS.ErrnoException).code;if(code==="ENOENT")return;
  if(code!=="EPERM"&&code!=="EACCES"&&code!=="EBUSY")throw error;
  await chmod(path,0o666).catch(()=>{});await unlink(path);
 }
}

/** 🗑️ Removes one tree in cancellable batches without ever following a link; a link is unlinked, never entered. */
export async function removeCacheTreeV1(path:string,ports:CacheRunnerPortsV1,stage:string):Promise<void>{
 let info;try{info=await lstat(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
 if(!info.isDirectory()){await unlinkPath(path);return;}
 const stack:{path:string;visited:boolean}[]=[{path,visited:false}],tick=ticker(ports,()=>stage);
 while(stack.length){
  await tick();const top=stack[stack.length-1]!;
  if(top.visited){stack.pop();await rmdir(top.path).catch(error=>{if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;});continue;}
  top.visited=true;const files:string[]=[];
  for(const child of await readdir(top.path,{withFileTypes:true}).catch(error=>{if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}))child.isDirectory()?stack.push({path:join(top.path,child.name),visited:false}):files.push(join(top.path,child.name));
  for(let index=0;index<files.length;index+=runtime.removeBatch){await tick();await Promise.all(files.slice(index,index+runtime.removeBatch).map(unlinkPath));}
 }
}

/** 📤️ One line of collector output, already localized. */
export type CacheOutputV1=Readonly<{write(line:string):void;language:CacheLanguageV1}>;
/** 🧮️ What a collection did or would do. */
export type CacheCollectionV1=Readonly<{dryRun:boolean;plans:readonly CacheTargetPlanV1[];removed:readonly CacheRemovalV1[];kept:number;failed:readonly Readonly<{target:string;name:string;message:string}>[];freedBytes:number;delegated:readonly Readonly<{owner:string;text:string}>[]}>;

type Resolved=Readonly<{target:CacheTargetV1;base:string;prefix:string[];unusedAgeMs:number|null;release?:()=>void}>;

function insideBase(base:string,path:string):boolean{const distance=relative(base,path);return distance!==""&&!distance.startsWith(".."+sep)&&distance!==".."&&!isAbsolute(distance);}

async function observe(base:string,names:readonly string[],id:string,custody:"quiet"|"sqlite"|"lease",ports:CacheRunnerPortsV1):Promise<CacheUnitV1[]>{
 const units:CacheUnitV1[]=[];
 for(const name of names){
  ports.signal.throwIfAborted();await ports.progress("scan:"+id+"/"+name);
  const measure=await measureCacheTreeV1(join(base,...name.split("/")),ports,runtime.collectEntriesPerUnit,id);
  if(!measure.present)continue;
  const held=custody==="lease"?undefined:await probeCacheHeldV1(measure,ports);
  units.push({name,bytes:measure.bytes,recencyMs:measure.recencyMs,...held?{held}:{}});
 }
 return units;
}

/** 🧹️ Plans and, unless asked to only plan, performs the collection: it scans every selected target, takes the area's exclusive lease without waiting, re-checks each unit right before removing it, and never touches a held or recently used one. */
export async function collectCacheV1(workspace:string,ledger:CacheLedgerV1,request:Readonly<{dryRun:boolean;only?:ReadonlySet<string>}>,ports:CacheRunnerPortsV1,output:CacheOutputV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):Promise<CacheCollectionV1>{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(output.language,key,variables),selected=(id:string)=>request.only===undefined||request.only.has(id),resolved:Resolved[]=[],delegated:{owner:string;text:string}[]=[],notes:string[]=[],now=ports.now();
 try{
  for(const area of ledger.areas){
   if(!selected(area.id))continue;
   if(area.managed&&area.kind!=="partition"&&area.kind!=="control"){
    const answer=ports.delegate?await ports.delegate(area.managed.owner,area.id,request.dryRun,output.language):undefined;
    if(answer!==undefined){delegated.push({owner:area.managed.owner,text:answer});notes.push(text("gc.owner",{owner:area.managed.owner,text:answer}));}else notes.push(text("gc.managed",{id:area.id,owner:area.managed.owner}));
    continue;
   }
   if(!area.collect)continue;
   const directory=cacheAreaDirectoryV1(workspace,ledger,area,partitions);let names:string[]=[],base:string;
   if(directory!==null){base=directory;try{names=await readdir(directory);}catch{names=[];}}
   else{base=cacheRootDirectoryV1(workspace,ledger,area.root);const matcher=new RegExp(area.pattern!,"u");try{names=(await readdir(base)).filter(name=>matcher.test(name));}catch{names=[];}}
   const lease=area.collect.custody==="lease"?await ports.exclusive(area.collect.lease!):undefined,heldBy=area.collect.custody==="lease"&&!lease?area.collect.lease!:undefined;
   const units=heldBy?[]:await observe(base,names,area.id,area.collect.custody,ports);
   resolved.push({target:{id:area.id,kind:"area",budgetBytes:cacheAreaBudgetBytesV1(ledger,area,partitions),unusedAgeMs:area.collect.unusedAgeMs,quietMs:area.collect.quietMs,...heldBy?{heldBy}:{},units},base,prefix:[],unusedAgeMs:area.collect.unusedAgeMs,...lease?{release:()=>lease.release()}:{}});
   if(heldBy)notes.push(text("gc.held",{path:area.id,holder:heldBy}));
  }
  if(request.only===undefined||request.only.has("retired"))for(const retirement of ledger.retirements){
   const base=join(cacheRootDirectoryV1(workspace,ledger,retirement.root),...retirement.directory.split("/"));let names:string[];
   try{names=await readdir(base);}catch{continue;}
   resolved.push({target:{id:retirement.root+":"+retirement.directory,kind:"retired",budgetBytes:null,unusedAgeMs:0,quietMs:retirement.quietMs,units:await observe(base,names,retirement.directory,"quiet",ports)},base,prefix:[],unusedAgeMs:0});
  }
  if(request.only===undefined||request.only.has("undeclared"))for(const root of["repository","native"] as const){
   const base=cacheRootDirectoryV1(workspace,ledger,root),names=await undeclaredEntriesV1(workspace,ledger,root);
   if(names.length)resolved.push({target:{id:root+":undeclared",kind:"undeclared",budgetBytes:null,unusedAgeMs:ledger.roots[root].undeclaredUnusedAgeMs,quietMs:ledger.roots[root].undeclaredQuietMs,units:await observe(base,names,root+":undeclared","quiet",ports)},base,prefix:[],unusedAgeMs:ledger.roots[root].undeclaredUnusedAgeMs});
  }
  const plans=planCacheCollectionV1(resolved.map(row=>row.target),now),removed:CacheRemovalV1[]=[],failed:{target:string;name:string;message:string}[]=[],gone=new Map<string,number>(),goneNames=new Set<string>();let kept=0,freed=0;
  output.write(text("gc.start",{count:plans.length,mode:text(request.dryRun?"gc.mode.plan":"gc.mode.apply")}));
  for(const note of notes)output.write(note);
  for(const[index,plan]of plans.entries()){
   const row=resolved[index]!;ports.signal.throwIfAborted();
   for(const unit of plan.kept){kept++;output.write(unit.reason==="held"?text("gc.held",{path:plan.id+"/"+unit.name,holder:row.target.units.find(candidate=>candidate.name===unit.name)?.held??row.target.heldBy??"?"}):text("gc.quiet",{path:plan.id+"/"+unit.name,minutes:Math.round(row.target.quietMs/60000)}));}
   for(const removal of plan.removals){
    ports.signal.throwIfAborted();
    const path=resolve(row.base,...removal.name.split("/")),reason=formatCacheReasonV1(output.language,removal.reason,row.unusedAgeMs),label=plan.id+"/"+removal.name;
    if(!insideBase(resolve(row.base),path)){failed.push({target:plan.id,name:removal.name,message:text("error.refused",{path:label})});continue;}
    if(request.dryRun){removed.push(removal);freed+=removal.bytes;output.write(text("gc.would",{path:label,bytes:formatCacheBytesV1(removal.bytes),reason}));continue;}
    const again=await measureCacheTreeV1(path,ports,runtime.collectEntriesPerUnit,plan.id),unit=row.target.units.find(candidate=>candidate.name===removal.name)!;
    if(!again.present)continue;
    const held=await probeCacheHeldV1(again,ports);
    if(held||ports.now()-again.recencyMs<row.target.quietMs||again.recencyMs>unit.recencyMs){kept++;output.write(held?text("gc.held",{path:label,holder:held}):text("gc.changed",{path:label}));continue;}
    output.write(text("gc.remove",{path:label,bytes:formatCacheBytesV1(removal.bytes),reason}));
    try{await removeCacheTreeV1(path,ports,"remove:"+label);removed.push(removal);freed+=removal.bytes;gone.set(plan.id,(gone.get(plan.id)??0)+removal.bytes);goneNames.add(plan.id+"/"+removal.name);}
    catch(error){if(ports.signal.aborted)throw error;failed.push({target:plan.id,name:removal.name,message:String((error as Error).message??error)});}
   }
   if(!request.dryRun&&row.target.kind==="retired"&&plan.removals.length===row.target.units.length&&!failed.some(item=>item.target===plan.id))await rmdir(row.base).catch(()=>{});
  }
  output.write(text(request.dryRun?"gc.summary.plan":"gc.summary.apply",{count:removed.length,bytes:formatCacheBytesV1(freed),kept}));
  const updates:Record<string,CacheSizeRecordV1|null>={},at=ports.now(),record=(bytes:number)=>({usedBytes:Math.max(0,bytes),partial:false,atMs:at});
  for(const[index,plan]of plans.entries()){
   const row=resolved[index]!;if(row.target.heldBy!==undefined)continue;
   if(row.target.kind==="area")updates[row.target.id]=record(plan.totalBytes-(gone.get(plan.id)??0));
   else if(row.target.kind==="retired")updates["retired:"+row.target.id]=row.target.units.every(unit=>goneNames.has(plan.id+"/"+unit.name))?null:record(plan.totalBytes-(gone.get(plan.id)??0));
   else for(const unit of row.target.units)updates["undeclared:"+row.target.id.slice(0,row.target.id.indexOf(":"))+":"+unit.name]=goneNames.has(plan.id+"/"+unit.name)?null:record(unit.bytes);
  }
  await recordCacheSizesV1(workspace,ledger,updates);
  return Object.freeze({dryRun:request.dryRun,plans,removed,kept,failed,freedBytes:freed,delegated});
 }finally{for(const row of resolved)try{row.release?.();}catch{}}
}
