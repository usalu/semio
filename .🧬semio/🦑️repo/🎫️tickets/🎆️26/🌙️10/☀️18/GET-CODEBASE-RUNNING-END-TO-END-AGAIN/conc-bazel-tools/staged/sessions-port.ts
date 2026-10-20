import {existsSync} from "node:fs";
import {chmod,lstat,mkdir,readdir,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {randomBytes} from "node:crypto";
import {join} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {BAZEL_SESSION_POLICY_V1 as policy,bazelSlotIdsV1,planBazelSessionGarbageV1,requireBazelKeyV1,requireBazelSlotV1,type BazelGarbageV1,type BazelInventoryV1,type BazelServerPlanV1,type BazelSlotPortV1,type BazelSlotStateV1} from "../🟦️.ts";

const slotPattern=new RegExp(policy.names.slotPattern),keyPattern=new RegExp(policy.names.keyPattern),missing=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""),busy=(error:unknown)=>error instanceof Error&&error.message.startsWith("Resource lease timed out: ");
export type BazelSlotDirectoriesV1=Readonly<{leases:string;server:string;workspaces:string}>;

/** 🔌️ Maps the session port onto repository resource leases (one exclusive lease per slot, one fair queued lease for the picker) and one remembered-state file per slot beside its output base; a state is valid only while the slot's output base exists, so a reset, a cache clear or an unmounted partition never leaves a stale affinity behind and nothing is ever written below an unmounted partition; the lease directory is the checkout's own, so sessions of different checkouts never meet. */
export function bazelSlotPortV1(directories:BazelSlotDirectoriesV1,options:Readonly<{control?:ResourceLeaseControl;write?:(line:string)=>void}>={}):BazelSlotPortV1{
 const control=options.control?{control:options.control}:{},file=(slot:string)=>join(directories.server,requireBazelSlotV1(slot)+policy.names.stateSuffix);
 return Object.freeze({
  async claim(slot,signal){try{const lease=await acquireResourceLease({directory:directories.leases,resource:policy.leases.slotPrefix+requireBazelSlotV1(slot),mode:"exclusive",signal,timeoutMs:0,...control});return()=>lease.release();}catch(error){if(busy(error))return undefined;throw error;}},
  async gate(signal){const lease=await acquireQueuedResourceLease({directory:directories.leases,resource:policy.leases.pickerResource,mode:"exclusive",owner:randomBytes(12).toString("base64url"),signal,...control});return()=>lease.release();},
  async read(slot){
   if(!existsSync(join(directories.server,requireBazelSlotV1(slot))))return undefined;
   try{const value=JSON.parse(await readFile(file(slot),"utf8"));return validateJsonSchemaSubset(schema.definitions.SlotState,value,schema).length||value.slot!==slot?undefined:value as BazelSlotStateV1;}
   catch(error){if(missing(error)||error instanceof SyntaxError)return undefined;throw error;}
  },
  async write(slot,state){if(!existsSync(join(directories.server,requireBazelSlotV1(slot))))return;const target=file(slot),temporary=`${target}.${process.pid}.tmp`;await writeFile(temporary,JSON.stringify(state)+"\n");await rename(temporary,target);},
  sleep:(milliseconds,signal)=>delay(milliseconds,undefined,{signal}).then(()=>undefined),
  now:()=>Date.now(),
  progress:line=>{(options.write??console.log)(line);},
 });
}

const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
/** 🛑️ Stops the persistent server of an output base the way Bazel itself notices a shutdown request, by removing its pid file, then waits for the process to end; it needs neither the Bazel binary nor a workspace and starts no JVM. An output base without a server is already stopped. */
export async function stopBazelServerV1(outputBase:string,signal:AbortSignal,rules:{timeoutMilliseconds:number;pollMilliseconds:number}=policy.stop):Promise<void>{
 const file=join(outputBase,policy.stop.pidFile);let pid:number;
 try{pid=Number((await readFile(file,"utf8")).trim());await rm(file,{force:true});}catch(error){if(missing(error))return;throw error;}
 if(!Number.isSafeInteger(pid)||pid<1)return;
 const deadline=Date.now()+rules.timeoutMilliseconds;
 while(alive(pid)){signal.throwIfAborted();if(Date.now()>=deadline)throw Error("A Bazel server did not stop after its pid file was removed");await delay(rules.pollMilliseconds,undefined,{signal});}
}

/** 🗑️ Removes a tree whose Bazel output directories may be read-only: a refused removal makes the tree writable and retries once. */
export async function removeBazelTreeV1(path:string):Promise<void>{
 try{await rm(path,{recursive:true,force:true});}
 catch(error){if(!["EACCES","EPERM","ENOTEMPTY"].includes((error as NodeJS.ErrnoException).code??""))throw error;await writable(path);await rm(path,{recursive:true,force:true});}
}
async function writable(path:string):Promise<void>{
 const facts=await lstat(path).catch(error=>{if(missing(error))return undefined;throw error;});if(facts===undefined||facts.isSymbolicLink())return;
 if(facts.isDirectory()){await chmod(path,0o700);for(const name of await readdir(path))await writable(join(path,name));}else await chmod(path,0o600);
}

/** 🔎️ Lists what exists on disk below the server and workspace directories: per slot its output base, remembered state and generated packages; every other directory is foreign (the hashed output bases and packages of the time before slots). */
export async function readBazelInventoryV1(directories:BazelSlotDirectoriesV1,port:BazelSlotPortV1):Promise<BazelInventoryV1>{
 const listing=async(path:string)=>readdir(path,{withFileTypes:true}).catch(error=>{if(missing(error))return [];throw error;}),slots=new Map<string,{outputBase:boolean;packages:{key:string;modifiedAt:number}[]}>(),foreign:string[]=[],row=(slot:string)=>{const value=slots.get(slot)??{outputBase:false,packages:[]};slots.set(slot,value);return value;};
 for(const entry of await listing(directories.server))if(entry.isDirectory()){if(slotPattern.test(entry.name))row(entry.name).outputBase=true;else foreign.push("server/"+entry.name);}else{const match=entry.name.endsWith(policy.names.stateSuffix)?entry.name.slice(0,-policy.names.stateSuffix.length):undefined;if(match!==undefined&&slotPattern.test(match))row(match);}
 for(const entry of await listing(directories.workspaces))if(entry.isDirectory()){
  if(!slotPattern.test(entry.name)){foreign.push("workspaces/"+entry.name);continue;}
  const value=row(entry.name);for(const child of await listing(join(directories.workspaces,entry.name)))if(child.isDirectory()&&keyPattern.test(child.name))value.packages.push({key:child.name,modifiedAt:Math.floor((await stat(join(directories.workspaces,entry.name,child.name))).mtimeMs)});
 }
 const rows=[];for(const [slot,value] of [...slots].sort((left,right)=>left[0]<right[0]?-1:1)){const state=await port.read(slot);rows.push({slot,outputBase:value.outputBase,...(state?{state}:{}),packages:value.packages});}
 const inventory={slots:rows,foreign:foreign.sort()};if(validateJsonSchemaSubset(schema.definitions.Inventory,inventory,schema).length)throw Error("Closed Bazel session inventory required");return inventory;
}

/** 🧹️ Collects session garbage: for every slot that has something to collect it takes the slot's claim (a slot in use is skipped, except the one the caller already holds), stops its server before removing the output base, removes old packages and the remembered state of a retired slot, and removes foreign entries after stopping their servers. Returns what was removed. */
export async function collectBazelSessionsV1(directories:BazelSlotDirectoriesV1,port:BazelSlotPortV1,plan:Pick<BazelServerPlanV1,"slots">,signal:AbortSignal,stop:(outputBase:string)=>Promise<void>,options:Readonly<{dryRun?:boolean;holding?:string;progress?:(line:string)=>void}>={}):Promise<BazelGarbageV1>{
 const inventory=await readBazelInventoryV1(directories,port),garbage=planBazelSessionGarbageV1(inventory,plan,port.now());if(options.dryRun)return garbage;
 const removedPackages:BazelGarbageV1["packages"][number][]=[],removedBases:BazelGarbageV1["outputBases"][number][]=[],removedForeign:string[]=[],slots=[...new Set([...garbage.packages.map(row=>row.slot),...garbage.outputBases.map(row=>row.slot)])];
 for(const slot of slots){
  const claimed=slot===options.holding?()=>undefined:await port.claim(slot,signal);if(claimed===undefined)continue;
  try{
   for(const row of garbage.outputBases.filter(value=>value.slot===slot)){const base=join(directories.server,slot);await stop(base);await removeBazelTreeV1(base);await rm(join(directories.server,slot+policy.names.stateSuffix),{force:true});await rm(base+".events.json",{force:true});removedBases.push(row);options.progress?.(`[bazel] removed the output base of slot ${slot} (${row.reason})`);}
   for(const row of garbage.packages.filter(value=>value.slot===slot)){await removeBazelTreeV1(join(directories.workspaces,slot,requireBazelKeyV1(row.key)));removedPackages.push(row);}
  }finally{claimed();}
 }
 for(const name of garbage.foreign){
  signal.throwIfAborted();const [area,entry]=[name.slice(0,name.indexOf("/")),name.slice(name.indexOf("/")+1)],path=join(area==="server"?directories.server:directories.workspaces,entry);
  if(area==="server")await stop(path).catch(()=>undefined);await removeBazelTreeV1(path);if(area==="server")await rm(path+".events.json",{force:true});removedForeign.push(name);options.progress?.(`[bazel] removed ${name}`);
 }
 return Object.freeze({packages:removedPackages,outputBases:removedBases,foreign:removedForeign});
}
