import {appendFile,lstat,mkdir,readdir,readFile,rename,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {CasPolicyV1} from "../🧬️schema/🟦️.ts";

const HOLDER=new RegExp(schema.definitions.PinFile.properties.holder.pattern,"u"),DIGEST=new RegExp(schema.definitions.Digest.pattern,"u");

/** 📌️ Directory of the pin files of the store; one file per holder, rewritten atomically. */
export const casPinDirectoryV1=(root:string):string=>join(root,"v1","p");

const alive=(pid:number):boolean=>{if(pid<=0)return true;try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 📌️ Pins `digests` for `holder` until `nowMs + ttlMs` (capped by the policy's lease maximum); a pin or lease protects objects from eviction, and a holder that died loses it at once. Re-pinning with the same holder replaces its earlier set (renewal). */
export async function pinCasV1(root:string,policy:Pick<CasPolicyV1,"leaseMaxMs">,holder:string,digests:readonly string[],ttlMs:number,nowMs:number,pid:number=process.pid):Promise<void>{
 if(!HOLDER.test(holder))throw Error("Valid pin holder required / Gültiger Pin-Halter erforderlich");
 for(const digest of digests)if(!DIGEST.test(digest))throw Error("Valid digest required / Gültiger Digest erforderlich");
 const directory=casPinDirectoryV1(root);await mkdir(directory,{recursive:true});
 const body=JSON.stringify({version:1,holder,pid,expiresAtMs:nowMs+Math.min(Math.max(ttlMs,0),policy.leaseMaxMs),digests:[...new Set(digests)].sort()}),temporary=join(directory,`.${holder}.${process.pid}.${Math.random().toString(36).slice(2)}`);
 await writeFile(temporary,body);await rename(temporary,join(directory,holder+".json"));
}

/** 🔓️ Releases every pin of a holder. */
export async function releaseCasPinsV1(root:string,holder:string):Promise<void>{
 if(!HOLDER.test(holder))throw Error("Valid pin holder required / Gültiger Pin-Halter erforderlich");
 await unlink(join(casPinDirectoryV1(root),holder+".json")).catch(()=>undefined);
}

/** 🔎️ Reads the digests that are pinned at `nowMs` by a live holder with an unexpired lease; expired or orphaned pin files are removed. */
export async function readCasPinsV1(root:string,nowMs:number):Promise<Map<string,string>>{
 const pinned=new Map<string,string>(),directory=casPinDirectoryV1(root);let names:string[];
 try{names=await readdir(directory);}catch{return pinned;}
 for(const name of names){
  if(!name.endsWith(".json")||name.startsWith("."))continue;
  const file=join(directory,name);let pin:{holder?:string;pid?:number;expiresAtMs?:number;digests?:unknown};
  try{pin=JSON.parse(await readFile(file,"utf8"));}catch{continue;}
  if(!(Number(pin.expiresAtMs)>nowMs)||!alive(Number(pin.pid))){await unlink(file).catch(()=>undefined);continue;}
  if(Array.isArray(pin.digests))for(const digest of pin.digests)if(typeof digest==="string"&&DIGEST.test(digest)&&!pinned.has(digest))pinned.set(digest,String(pin.holder));
 }
 return pinned;
}

type HeldCasPinsV1={digests:Set<string>;lastWrite:Promise<void>;queued:boolean;leaseMaxMs:number};
const holds=new Map<string,HeldCasPinsV1>();

function publishHeldCasPinsV1(root:string,holder:string,hold:HeldCasPinsV1,leaseMaxMs:number):Promise<void>{
 hold.leaseMaxMs=leaseMaxMs;
 if(hold.queued)return hold.lastWrite;
 hold.queued=true;
 hold.lastWrite=hold.lastWrite.then(()=>{
  hold.queued=false;
  const {digests,leaseMaxMs}=hold;
  return pinCasV1(root,{leaseMaxMs},holder,[...digests],leaseMaxMs,Date.now()).catch(()=>undefined);
 });
 return hold.lastWrite;
}

/** 🪢️ Holds digests for the life of this process, dropping the least recently named above `maximum`. Updates and renewals coalesce behind the current write and publish the latest set in order. A failed write keeps the prior pin. */
export async function holdCasPinsV1(root:string,policy:Pick<CasPolicyV1,"leaseMaxMs">,holder:string,digests:Iterable<string>,maximum:number=65536):Promise<void>{
 const key=root+"\0"+holder;
 let hold=holds.get(key);
 if(!hold){
  const created:HeldCasPinsV1={digests:new Set(),lastWrite:Promise.resolve(),queued:false,leaseMaxMs:policy.leaseMaxMs};
  const renew=setInterval(()=>{void publishHeldCasPinsV1(root,holder,created,policy.leaseMaxMs);},Math.max(1,Math.floor(policy.leaseMaxMs/3)));
  renew.unref?.();holds.set(key,hold=created);
 }
 const set=hold.digests;
 for(const digest of digests){set.delete(digest);set.add(digest);}
 while(set.size>maximum)set.delete(set.values().next().value!);
 await publishHeldCasPinsV1(root,holder,hold,policy.leaseMaxMs);
}

/** ⏳️ The request header with which a client marks a blob as an input of one action (value `input`): the cache server leases it instead of treating it like a published output. */
export const CAS_LEASE_HEADER_V1="x-semio-lease";

/** ⏳️ Directory of the transient lease segments of the store: one append-only file per writer process, a line `{d,u}` per digest and lease expiry. */
export const casTransientDirectoryV1=(root:string):string=>join(root,"v1","x");

/** ⏳️ Leases blobs that are inputs of one action (not outputs): they are protected until `nowMs + ttlMs` (the action runs, plus a grace) and are the first the collector takes after that, unless something read them later. A new lease for a digest replaces the earlier one (renewal). Never pins: outputs are pinned, inputs only lease. */
export async function leaseCasTransientV1(root:string,policy:Pick<CasPolicyV1,"transientLeaseMs">,writer:string,digests:readonly string[],nowMs:number,ttlMs:number=policy.transientLeaseMs):Promise<void>{
 if(!HOLDER.test(writer))throw Error("Valid lease writer required / Gültiger Lease-Halter erforderlich");
 for(const digest of digests)if(!DIGEST.test(digest))throw Error("Valid digest required / Gültiger Digest erforderlich");
 if(digests.length===0)return;
 const directory=casTransientDirectoryV1(root),until=nowMs+Math.min(Math.max(ttlMs,0),policy.transientLeaseMs*4);
 await mkdir(directory,{recursive:true});
 await appendFile(join(directory,writer+".jsonl"),[...new Set(digests)].map(d=>JSON.stringify({d,u:until})+"\n").join(""));
}

/** ⏳️ Reads the leases: the latest expiry per digest over every segment; unreadable lines are skipped. */
export async function readCasTransientsV1(root:string):Promise<Map<string,number>>{
 const leases=new Map<string,number>(),directory=casTransientDirectoryV1(root);let names:string[];
 try{names=await readdir(directory);}catch{return leases;}
 for(const name of names){
  if(!name.endsWith(".jsonl"))continue;
  let text:string;try{text=await readFile(join(directory,name),"utf8");}catch{continue;}
  for(const line of text.split("\n")){
   if(line==="")continue;
   try{const row=JSON.parse(line) as {d?:unknown;u?:unknown};if(typeof row.d==="string"&&DIGEST.test(row.d)&&typeof row.u==="number")leases.set(row.d,Math.max(leases.get(row.d)??0,row.u));}catch{}
  }
 }
 return leases;
}

/** 🧹️ Drops what no longer matters from segments nobody appended to for longer than the lease: lines of blobs that are gone (`exists` false) or whose lease ended and which were taken. A segment that ends up empty is removed. */
export async function compactCasTransientsV1(root:string,policy:Pick<CasPolicyV1,"transientLeaseMs">,nowMs:number,exists:(digest:string)=>Promise<boolean>):Promise<number>{
 const directory=casTransientDirectoryV1(root);let names:string[],removed=0;
 try{names=await readdir(directory);}catch{return 0;}
 for(const name of names){
  if(!name.endsWith(".jsonl"))continue;
  const path=join(directory,name);let info;try{info=await lstat(path);}catch{continue;}
  if(nowMs-info.mtimeMs<policy.transientLeaseMs*4)continue;
  let text:string;try{text=await readFile(path,"utf8");}catch{continue;}
  const kept:string[]=[];
  for(const line of text.split("\n")){
   if(line==="")continue;
   try{const row=JSON.parse(line) as {d:string};if(await exists(row.d))kept.push(line);else removed++;}catch{removed++;}
  }
  if(kept.length===0){await unlink(path).catch(()=>undefined);continue;}
  if(kept.length===text.split("\n").filter(Boolean).length)continue;
  const temporary=join(directory,`.${name}.${process.pid}`);
  try{await writeFile(temporary,kept.join("\n")+"\n");await rename(temporary,path);}catch{await unlink(temporary).catch(()=>undefined);}
 }
 return removed;
}

/** ⏳️ A cheap stamp of the lease segments (names, sizes, modification times) that changes whenever a lease was appended, so a long collection reads the segments again only then. */
export async function stampCasTransientsV1(root:string):Promise<string>{
 const directory=casTransientDirectoryV1(root);let names:string[];
 try{names=(await readdir(directory)).sort();}catch{return "";}
 const parts:string[]=[];
 for(const name of names)try{const info=await lstat(join(directory,name));parts.push(`${name}:${info.size}:${info.mtimeMs}`);}catch{}
 return parts.join("|");
}
