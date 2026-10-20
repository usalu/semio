import{createHash}from"node:crypto";
import{lstatSync,readdirSync,readFileSync}from"node:fs";
import{join,sep}from"node:path";
import ticketSchema from"../../../../../../../🔒️leases/🧬️schema/📜️ticket.json" with{type:"json"};
import schema from"../🧬️schema/🔣️.json" with{type:"json"};
import type{NativeBootFactV1,NativeBootObservationV1}from"../📋️plan/🟦️.ts";
const protocol=Object.freeze({...schema.definitions.Protocol.const}),ticket=ticketSchema.definitions.Protocol.const,sha=new RegExp(schema.definitions.Sha256.pattern,"u"),maximumRecordBytes=8192;

/** 🎯️ Names one directory tree to inspect; paths are canonical absolute and never leave their root. */
export type NativeBootTargetV1=
 |Readonly<{kind:"publication";scope:string;root:string;writerFree:()=>boolean|Promise<boolean>}>
 |Readonly<{kind:"lease-catalog";scope:string;directory:string}>
 |Readonly<{kind:"server";scope:string;root:string;paths:readonly string[]}>
 |Readonly<{kind:"swap";scope:string;directory:string;intent:string;writerFree:()=>boolean|Promise<boolean>}>
 |Readonly<{kind:"record";scope:string;final:string;temporary:string}>
 |Readonly<{kind:"images";scope:string;workspace:string;roots:readonly string[];now:()=>number;roster:()=>Promise<readonly Readonly<{imagePath:string}>[]>;lease:(imagePath:string)=>Promise<"free"|"held"|"unavailable">}>;

/** 🕰️ Fixes the boot identity the facts are judged against. */
export type NativeBootClockV1=Readonly<{bootSession:string;bootEpochMs:number}>;

const absent=(error:unknown)=>(error as NodeJS.ErrnoException).code==="ENOENT";
function stat(path:string){try{return lstatSync(path,{bigint:true});}catch(error){if(absent(error))return undefined;throw error;}}
function names(path:string):string[]{try{return readdirSync(path);}catch(error){if(absent(error))return[];throw error;}}
function readBounded(path:string):Buffer|"unsafe"|undefined{
 const row=stat(path);if(!row)return undefined;
 if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1n||row.size>BigInt(maximumRecordBytes)||(Number(row.mode)&0o177)!==0)return"unsafe";
 return readFileSync(path);
}
function jsonState(bytes:Buffer|"unsafe"|undefined,shape:(value:Record<string,unknown>)=>boolean):"absent"|"torn"|"valid"|"unsafe"{
 if(bytes===undefined)return"absent";if(bytes==="unsafe")return"unsafe";if(!bytes.length)return"torn";
 let value:unknown;try{value=JSON.parse(bytes.toString("utf8"));}catch{return"torn";}
 return value!==null&&typeof value==="object"&&!Array.isArray(value)&&shape(value as Record<string,unknown>)?"valid":"unsafe";
}
const digest=(bytes:Buffer)=>createHash("sha256").update(bytes).digest("hex");

async function publication(target:Extract<NativeBootTargetV1,{kind:"publication"}>):Promise<NativeBootFactV1>{
 const writerFree=await target.writerFree(),base=join(target.root,protocol.publication.directory),row=stat(base);
 const closed={family:"publication-residue" as const,scope:target.scope,writerFree,unknownEntries:0,intentReceiptSha256:null,image:{state:"absent" as const,sha256:null}};
 if(!row)return{...closed,directory:"absent",intent:"absent",receipt:"absent"};
 if(!row.isDirectory()||row.isSymbolicLink()||(Number(row.mode)&0o777)!==protocol.publication.directoryMode)return{...closed,directory:"unsafe",intent:"absent",receipt:"absent"};
 const listed=names(base),known=protocol.publication.files as readonly string[];
 const intentBytes=readBounded(join(base,known[0]!)),receiptBytes=readBounded(join(base,known[1]!)),imageBytes=readBounded(join(target.root,"image.json"));
 const intent=jsonState(intentBytes,value=>typeof value.receiptSha256==="string"&&sha.test(value.receiptSha256)),receipt=jsonState(receiptBytes,()=>true);
 const image=imageBytes===undefined?{state:"absent" as const,sha256:null}:imageBytes==="unsafe"||jsonState(imageBytes,()=>true)!=="valid"?{state:"invalid" as const,sha256:null}:{state:"valid" as const,sha256:digest(imageBytes)};
 return{...closed,directory:"ordinary",intent,receipt,unknownEntries:listed.filter(name=>!known.includes(name)).length,intentReceiptSha256:intent==="valid"?JSON.parse((intentBytes as Buffer).toString("utf8")).receiptSha256:null,image};
}

function decodeTicket(name:string):Readonly<{parsed:boolean;arrivedAtMs:number|null;pid:number|null}>{
 if(!new RegExp(ticket.ticketPattern,"u").test(name))return{parsed:false,arrivedAtMs:null,pid:null};
 const parts=name.split("-"),arrivedAtMs=Number.parseInt(parts[0]!,ticket.arrivalBase),pid=Number.parseInt(parts[1]!,ticket.pidBase);
 const canonical=arrivedAtMs.toString(ticket.arrivalBase).padStart(ticket.arrivalDigits,"0")===parts[0]&&pid.toString(ticket.pidBase).padStart(ticket.pidDigits,"0")===parts[1];
 return canonical&&pid>=1&&Number.isSafeInteger(arrivedAtMs)?{parsed:true,arrivedAtMs,pid}:{parsed:false,arrivedAtMs:null,pid:null};
}
function leaseCatalog(target:Extract<NativeBootTargetV1,{kind:"lease-catalog"}>,clock:NativeBootClockV1):readonly NativeBootFactV1[]{
 const facts:NativeBootFactV1[]=[];
 for(const queue of names(target.directory).filter(name=>/^[a-f0-9]{64}\.queue$/u.test(name)).sort()){
  const row=stat(join(target.directory,queue));if(!row?.isDirectory()||row.isSymbolicLink())continue;
  for(const name of names(join(target.directory,queue)).sort())facts.push({family:"lease-ticket",scope:target.scope,queue,name,...decodeTicket(name)});
 }
 const claims=join(target.directory,protocol.claimsDirectory),databases=names(claims).filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name));
 const previous=databases.filter(name=>{const row=stat(join(claims,name));return row?.isFile()&&Number(row.mtimeMs)+protocol.toleranceMilliseconds<=clock.bootEpochMs;}).length;
 facts.push({family:"lease-claims",scope:target.scope,claimDatabases:databases.length,previousBootClaimDatabases:previous});
 return facts;
}
function server(target:Extract<NativeBootTargetV1,{kind:"server"}>):NativeBootFactV1{
 const files:{path:string;modifiedAtMs:number}[]=[];
 for(const path of target.paths){const row=stat(join(target.root,path));if(row?.isFile()&&!row.isSymbolicLink())files.push({path,modifiedAtMs:Math.floor(Number(row.mtimeMs))});}
 return{family:"tool-server",scope:target.scope,files};
}
async function swap(target:Extract<NativeBootTargetV1,{kind:"swap"}>):Promise<NativeBootFactV1>{
 const row=stat(join(target.directory,target.intent));return{family:"directory-swap",scope:target.scope,writerFree:await target.writerFree(),intent:row?"present":"absent"};
}
function record(target:Extract<NativeBootTargetV1,{kind:"record"}>):NativeBootFactV1{
 const a=stat(target.final),b=stat(target.temporary),side=(row:ReturnType<typeof stat>)=>({state:row?"present" as const:"absent" as const,links:row?Number(row.nlink):0});
 return{family:"persistent-record",scope:target.scope,final:side(a),temporary:side(b),pair:a&&b?(a.dev===b.dev&&a.ino===b.ino?"same-inode":"different-inode"):"not-applicable"};
}

const within=(path:string,root:string)=>path===root||path.startsWith(root.endsWith(sep)?root:root+sep);
async function images(target:Extract<NativeBootTargetV1,{kind:"images"}>):Promise<readonly NativeBootFactV1[]>{
 const rows=await target.roster(),limits=protocol.leakedImages;if(rows.length>limits.maximumImages)throw Error("Bounded kernel image roster required");
 const facts:NativeBootFactV1[]=[];
 for(const{imagePath}of rows){
  const key=createHash("sha256").update(imagePath).digest("hex"),row=stat(imagePath),present=!!row&&row.isFile()&&!row.isSymbolicLink(),location=within(imagePath,target.workspace)?"workspace":target.roots.some(root=>within(imagePath,root))?"scratch":"other";
  if(present&&location!=="scratch")continue;
  const idle=present&&target.now()-Number(row!.mtimeMs)>=limits.idleMilliseconds;
  facts.push({family:"leaked-image",scope:target.scope,key,backing:present?"present":"absent",location,idle,lease:present&&location==="scratch"&&idle?await target.lease(imagePath):"unavailable"});
 }
 return facts;
}

/** 👁️ Reads the closed facts of the given targets; it only lstat/readdir/reads and mutates nothing. */
export async function observeNativeBootTargetsV1(clock:NativeBootClockV1,targets:readonly NativeBootTargetV1[]):Promise<NativeBootObservationV1>{
 const facts:NativeBootFactV1[]=[];
 for(const target of targets){
  if(target.kind==="publication")facts.push(await publication(target));
  else if(target.kind==="lease-catalog")facts.push(...leaseCatalog(target,clock));
  else if(target.kind==="server")facts.push(server(target));
  else if(target.kind==="swap")facts.push(await swap(target));
  else if(target.kind==="images")facts.push(...await images(target));
  else facts.push(record(target));
 }
 return Object.freeze({version:1 as const,bootSession:clock.bootSession,bootEpochMs:clock.bootEpochMs,facts:Object.freeze(facts)});
}
