import{createHash}from"node:crypto";
import{existsSync,lstatSync,mkdirSync,realpathSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{captureOwnedCommandV1}from"../../../../../../../🎛️owned-execution/🟦️.ts";
import{acquireResourceLease,checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../🔒️leases/🟦️.ts";
import{observeSharedNativeImageInventoryV1}from"../../../../🍎️image/📡️roster/🟦️.ts";
import cleanupSchema from"../../../../🧹️cleanup/🎛️command/🧬️schema/🔣️.json" with{type:"json"};
import storageSchema from"../../../../🧬️schema/🔣️.json" with{type:"json"};
import{healNativeBootV1,nativeBootLedgerPathV1,recoverNativeBootV1,type NativeBootProgressV1,type NativeBootResultV1}from"../🟦️.ts";
import{createNativeBootFsPortV1,createNativeBootLeaseEffectsV1}from"../🧯️act/🟦️.ts";
import{readNativeBootClockV1}from"../🕰️clock/🟦️.ts";
import type{NativeBootClockV1,NativeBootTargetV1}from"../📡️observe/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
const protocol=schema.definitions.Protocol.const,leaks=protocol.leakedImages,cleanup=cleanupSchema.definitions.Protocol.const,image=storageSchema.definitions.ImageProtocol.const;

/** 🖼️ Names one image of the kernel roster by its backing path and its whole-disk device. */
export type NativeBootRosterImageV1=Readonly<{imagePath:string;device:string|null}>;

/** 🔌️ Supplies the two host reads recovery needs; production binds them to the issued control, tests to plain process spawns. */
export type NativeBootHostV1=Readonly<{capture(program:string,args:readonly string[],cwd:string,limits:Readonly<{maximumBytes:number;maximumLines:number}>):Promise<string>;clock():Promise<NativeBootClockV1>}>;

/** 🔗️ Binds the host reads to one issued resource control. */
export function nativeBootHostV1(control:ResourceLeaseControl,leaseDirectory:string):NativeBootHostV1{return Object.freeze({capture:(program,args,cwd,output)=>captureOwnedCommandV1(program,args,cwd,{control,output}),clock:()=>readNativeBootClockV1(control,leaseDirectory)});}

/** 🏷️ Places every recovery ledger beside, never inside, the partition tree: `<cache>/b/c` leases give `<cache>/boot-recovery`. */
export function nativeBootLedgerDirectoryV1(leaseDirectory:string):string{return join(dirname(dirname(leaseDirectory)),"boot-recovery");}

const publisher=(control:ResourceLeaseControl,onProgress?:(event:NativeBootProgressV1)=>void)=>{
 const started=Date.now();
 return(event:NativeBootProgressV1):void=>{onProgress?.(event);void Promise.resolve(control.publish({resource:`boot-recovery:${event.stage}:${event.done}/${event.total}${event.target?":"+event.target.slice(-48):""}`,mode:"exclusive",elapsedMs:Date.now()-started})).catch(()=>{});};
};

/** 🚧️ Tells whether a partition directory still holds publication metadata of a publisher that ended; a live publisher holds the exclusive image lease, so a reader holding the shared lease sees only abandoned metadata. */
export function nativePartitionPublicationPendingV1(directory:string):boolean{return existsSync(join(directory,protocol.publication.directory));}

/** 🩺️ Settles abandoned publication residue of one partition under the exclusive image lease the caller already holds; residue that is not provably abandoned is refused with its reason. */
export async function settleNativePartitionPublicationV1(input:Readonly<{directory:string;leaseDirectory:string;control:ResourceLeaseControl;host?:NativeBootHostV1;onProgress?:(event:NativeBootProgressV1)=>void}>):Promise<void>{
 const{directory,leaseDirectory,control}=input,clock=await(input.host??nativeBootHostV1(control,leaseDirectory)).clock(),ledgerPath=nativeBootLedgerPathV1(nativeBootLedgerDirectoryV1(leaseDirectory),directory);
 const targets:readonly NativeBootTargetV1[]=[{kind:"publication",scope:"partition",root:directory,writerFree:()=>true}];
 const port=createNativeBootFsPortV1({clock,targets,effects:createNativeBootLeaseEffectsV1({directories:new Map(),control})});
 const result=await recoverNativeBootV1({port,ledgerPath,now:Date.now,signal:control.signal,onProgress:publisher(control,input.onProgress)});
 if(result.refused.length)throw Error(`Boot recovery retained publication residue of ${directory}: ${result.refused.map(row=>row.reason).join(", ")}`);
}

/** 🧹️ Retires dead-boot queue tickets, their claims and stale build-server files once per boot; it needs no partition authority because every effect carries its own kernel proof. */
export async function healNativeToolStateBootV1(input:Readonly<{leaseDirectory:string;control:ResourceLeaseControl;host?:NativeBootHostV1;servers:readonly Readonly<{scope:string;root:string;paths:readonly string[]}>[];onProgress?:(event:NativeBootProgressV1)=>void}>):Promise<"healthy"|NativeBootResultV1>{
 const{leaseDirectory,control}=input,clock=await(input.host??nativeBootHostV1(control,leaseDirectory)).clock(),ledgerPath=nativeBootLedgerPathV1(nativeBootLedgerDirectoryV1(leaseDirectory),leaseDirectory);
 const targets:NativeBootTargetV1[]=[{kind:"lease-catalog",scope:"leases",directory:leaseDirectory},...input.servers.map(row=>({kind:"server" as const,...row}))];
 const port=createNativeBootFsPortV1({clock,targets,effects:createNativeBootLeaseEffectsV1({directories:new Map([["leases",leaseDirectory]]),control})});
 return healNativeBootV1({ledgerPath,bootSession:clock.bootSession,residue:()=>false,recover:()=>recoverNativeBootV1({port,ledgerPath,now:Date.now,signal:control.signal,onProgress:publisher(control,input.onProgress)})});
}

/** 🛰️ Reads the kernel image roster through the shared per-process roster; one attach or detach by any process never yields a torn view. */
export function nativeBootImageRosterV1(control:ResourceLeaseControl,host:NativeBootHostV1,cwd:string):()=>Promise<readonly NativeBootRosterImageV1[]>{
 const policy=Object.freeze({attempts:image.informationAttempts,backoffMilliseconds:image.informationBackoffMilliseconds,signal:control.signal});
 return()=>observeSharedNativeImageInventoryV1(()=>host.capture("/usr/bin/hdiutil",["info","-plist"],cwd,{maximumBytes:cleanup.maximumOutputBytes,maximumLines:cleanup.maximumOutputLines}),information=>{
  const rows=information.images;if(!Array.isArray(rows)||rows.length>cleanup.maximumObservedImages)throw Error("Complete bounded Native kernel inventory required");
  return Object.freeze(rows.flatMap((row:unknown)=>{
   const entry=row as Record<string,unknown>|null;if(!entry||typeof entry!=="object"||Array.isArray(entry)||typeof entry["image-path"]!=="string")return[];
   const entities=Array.isArray(entry["system-entities"])?entry["system-entities"]as Record<string,unknown>[]:[],device=entities.map(item=>item?.["dev-entry"]).find((value):value is string=>typeof value==="string"&&/^\/dev\/disk[0-9]+$/u.test(value))??null;
   return[Object.freeze({imagePath:entry["image-path"],device})];
  }));
 },policy);
}

const partition=new RegExp(leaks.partitionPattern,"u");
function leaseOf(imagePath:string):Readonly<{directory:string;resource:string}>|undefined{
 const match=partition.exec(imagePath);if(!match)return undefined;
 const directory=join(match[1]!,".cache","b",leaks.leaseSibling);return existsSync(directory)?{directory,resource:leaks.leaseResourcePrefix+dirname(imagePath)}:undefined;
}
const timedOut=(error:unknown)=>String(error).includes("Resource lease timed out");

/** 🔑️ Proves by an exclusive kernel claim that no process holds the image's lease; the claim is released at once. */
async function probeLease(control:ResourceLeaseControl,imagePath:string):Promise<"free"|"held"|"unavailable">{
 const lease=leaseOf(imagePath);if(!lease)return"unavailable";
 try{(await acquireResourceLease({...lease,mode:"exclusive",signal:control.signal,control,timeoutMs:0})).release();return"free";}
 catch(error){checkResourceLeaseControl(control);return timedOut(error)?"held":"unavailable";}
}

/** 🧲️ Detaches one roster image after re-proving its abandonment under the image lease; a busy volume is left for the next run. */
function detacher(control:ResourceLeaseControl,host:NativeBootHostV1,roster:()=>Promise<readonly NativeBootRosterImageV1[]>,cwd:string):(scope:string,key:string)=>Promise<"done"|"skipped">{
 return async(_scope,key)=>{
  const find=async()=>(await roster()).find(row=>createHash("sha256").update(row.imagePath).digest("hex")===key);
  const row=await find();if(!row?.device)return"skipped";
  const held=leaseOf(row.imagePath);let lease;
  if(held&&existsSync(row.imagePath)){try{lease=await acquireResourceLease({...held,mode:"exclusive",signal:control.signal,control,timeoutMs:0});}catch(error){checkResourceLeaseControl(control);if(timedOut(error))return"skipped";throw error;}}
  try{
   try{await host.capture("/usr/bin/hdiutil",[...leaks.detach.arguments,row.device],cwd,{maximumBytes:leaks.detach.maximumBytes,maximumLines:64});}
   catch(error){checkResourceLeaseControl(control);}
   return await find()?"skipped":"done";
  }finally{lease?.release();}
 };
}

const canonical=(path:string)=>{try{return realpathSync(path);}catch{return path;}};
const processed=new Map<string,Promise<void>>();

/** 🖼️ Sweeps abandoned scratch images and images of removed backing files off the kernel roster; at most one sweep per interval and cache runs, because a clean observation costs a roster read. */
export async function healNativeLeakedImagesV1(input:Readonly<{leaseDirectory:string;control:ResourceLeaseControl;host:NativeBootHostV1;roster:()=>Promise<readonly NativeBootRosterImageV1[]>;onProgress?:(event:NativeBootProgressV1)=>void}>):Promise<"swept"|"recent">{
 const{leaseDirectory,control,host,roster}=input,ledgerPath=nativeBootLedgerPathV1(nativeBootLedgerDirectoryV1(leaseDirectory),leaseDirectory+"#images"),stamp=ledgerPath+".stamp";
 try{if(Date.now()-lstatSync(stamp).mtimeMs<leaks.sweepMilliseconds)return"recent";}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 const workspace=dirname(dirname(dirname(leaseDirectory))),clock=await host.clock();
 const roots=[...new Set([...leaks.scratchRoots,...leaks.environmentRoots.map(name=>process.env[name]).filter((value):value is string=>!!value)].flatMap(root=>[root,canonical(root)]))];
 const targets:readonly NativeBootTargetV1[]=[{kind:"images",scope:"images",workspace,roots,now:Date.now,roster,lease:path=>probeLease(control,path)}];
 const port=createNativeBootFsPortV1({clock,targets,effects:createNativeBootLeaseEffectsV1({directories:new Map(),control,detach:detacher(control,host,roster,workspace)})});
 await recoverNativeBootV1({port,ledgerPath,now:Date.now,signal:control.signal,onProgress:publisher(control,input.onProgress)});
 mkdirSync(dirname(stamp),{recursive:true,mode:0o700});writeFileSync(stamp,"",{mode:0o600});return"swept";
}

/** 🛡️ Recovers once per process what a reboot, crash or killed run left in the lease catalog and the kernel image roster; failures are reported through progress and never block the caller, cancellation always propagates. */
export function healNativeProcessBootV1(input:Readonly<{leaseDirectory:string;control:ResourceLeaseControl;host?:NativeBootHostV1;roster?:()=>Promise<readonly NativeBootRosterImageV1[]>;servers?:readonly Readonly<{scope:string;root:string;paths:readonly string[]}>[];onProgress?:(event:NativeBootProgressV1)=>void}>):Promise<void>{
 const{leaseDirectory,control}=input,existing=processed.get(leaseDirectory);if(existing)return existing;
 const work=(async()=>{
  try{
   checkResourceLeaseControl(control);
   const host=input.host??nativeBootHostV1(control,leaseDirectory);
   await healNativeToolStateBootV1({leaseDirectory,control,host,servers:input.servers??[],...(input.onProgress?{onProgress:input.onProgress}:{})});
   const roster=input.roster??(process.platform==="darwin"?nativeBootImageRosterV1(control,host,dirname(dirname(dirname(leaseDirectory)))):undefined);
   if(roster)await healNativeLeakedImagesV1({leaseDirectory,control,host,roster,...(input.onProgress?{onProgress:input.onProgress}:{})});
  }catch(error){
   if(control.signal.aborted)throw error;
   void Promise.resolve(control.publish({resource:"boot-recovery:failed:"+String((error as Error)?.message??error).slice(0,64),mode:"exclusive",elapsedMs:0})).catch(()=>{});
  }
 })();
 processed.set(leaseDirectory,work);work.catch(()=>processed.delete(leaseDirectory));return work;
}
