import{createHash}from"node:crypto";
import{constants,readFileSync}from"node:fs";
import{lstat,mkdir,open,readFile,readdir,rename,rm,writeFile}from"node:fs/promises";
import{homedir}from"node:os";
import{dirname,join,resolve}from"node:path";
import{runtimeKeyV1}from"../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import authorizerSchema from"../🧬️schema/🔣️.json"with{type:"json"};

/** 👁️ One filesystem fact a capture depended on beyond the bytes of its files: a path that was absent, the entries of a data directory it listed, or a package it resolved. */
export type NxAuthorizerObservationV1=
 |Readonly<{kind:"absent";path:string}>
 |Readonly<{kind:"directory";path:string;entries:readonly(readonly[string,"d"|"f"])[]}>
 |Readonly<{kind:"package";specifier:string;from:string;candidate:string|null;mark:string|null}>;

/** 🧊️ Everything a later process needs to prove that a finished capture is still the capture of the present tree. */
export interface NxAuthorizerMemoV1{
 readonly version:1;
 readonly installed:boolean;
 readonly implementation:string;
 readonly root:string;
 readonly entry:string;
 readonly directories:readonly Readonly<{path:string;dev:string;ino:string}>[];
 readonly observations:readonly NxAuthorizerObservationV1[];
 readonly manifest:Readonly<{version:1;mode:"release";entry:string;resolutions:readonly Readonly<{specifier:string;path:string}>[];sources:readonly Readonly<{path:string;sha256:string}>[]}>;
 readonly marks:Readonly<Record<string,string>>;
 readonly imports:Readonly<Record<string,Readonly<Record<string,string>>>>;
}

/** 🎛️ The environment variable that switches the source memo: `on` (default) or `off`; anything else is refused in both languages. */
export const NX_AUTHORIZER_MEMO_VARIABLE_V1="SEMIO_REPO_SOURCE_MEMO";
const MEMO_POLICY=authorizerSchema.$defs.MemoPolicy.const,MAXIMUM_FILE_BYTES=BigInt(MEMO_POLICY.maximumFileBytes),CONCURRENCY=MEMO_POLICY.concurrency;
/** 🪪️ The files, relative to this module, whose bytes decide what a capture contains. */
export const NX_AUTHORIZER_MEMO_IMPLEMENTATION_FILES_V1=Object.freeze(["../🟦️.ts","../🧬️schema/🔣️.json","../🧭️sites/🟦️.ts","../🧭️sites/🧬️schema/🔣️.json","../📦️workspace/🟦️.ts","./🟦️.ts"] as const);
let implementation:string|undefined;

/** 🧬️ Identifies the code that decides what a capture contains - the capture, its import-site reader, its workspace resolver, this memo - and the runtime that scans imports, so a memo is never served by an implementation other than the one that wrote it. */
export function nxAuthorizerMemoImplementationV1():string{
 if(implementation===undefined){
  const digest=createHash("sha256").update(runtimeKeyV1()+"\0");
  for(const name of NX_AUTHORIZER_MEMO_IMPLEMENTATION_FILES_V1)digest.update(name+"\0").update(readFileSync(new URL(name,import.meta.url)));
  implementation=digest.digest("hex");
 }
 return implementation;
}

/** 🔎️ Reads the memo switch; an unknown value is a refusal, never a silent default. */
export function readNxAuthorizerMemoSwitchV1(environment:Readonly<Record<string,string|undefined>>=process.env):boolean{
 const raw=environment[NX_AUTHORIZER_MEMO_VARIABLE_V1]??"on",value=raw.toLowerCase();
 if(value==="on")return true;
 if(value==="off")return false;
 throw Error(`Expected ${NX_AUTHORIZER_MEMO_VARIABLE_V1}=on|off, got "${raw}" / Erwartet ${NX_AUTHORIZER_MEMO_VARIABLE_V1}=on|off, erhalten "${raw}"`);
}

/** 🏠️ The per-user cache directory of this host, outside every checkout and every managed partition so that nested processes with another TMPDIR find the same memo. */
export function nxAuthorizerMemoBaseV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 if(platform==="darwin")return join(home,"Library","Caches","semio-repo");
 if(platform==="win32")return join(environment.LOCALAPPDATA||join(home,"AppData","Local"),"semio-repo","cache");
 return join(environment.XDG_CACHE_HOME||join(home,".cache"),"semio-repo");
}

/** 🗂️ The memo file of one workspace and one capture entry. */
export function nxAuthorizerMemoFileV1(base:string,root:string,entry:string,installed:boolean):string{
 const digest=(value:string)=>createHash("sha256").update(value).digest("hex").slice(0,20);
 return join(base,"source-memo",digest(resolve(root)),digest(entry+(installed?"\0installed":""))+".json");
}

const stamp=(row:{dev:bigint;ino:bigint;size:bigint;nlink:bigint;mtimeNs:bigint;ctimeNs:bigint})=>[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(":");
const digestOf=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");

async function pool<T>(items:readonly T[],work:(item:T)=>Promise<boolean>,signal:AbortSignal):Promise<boolean>{
 let next=0,ok=true;
 const lane=async()=>{while(ok&&next<items.length){signal.throwIfAborted();const item=items[next++]!;if(!await work(item))ok=false;}};
 await Promise.all(Array.from({length:Math.min(CONCURRENCY,items.length)},lane));
 signal.throwIfAborted();
 return ok;
}

/** 📖️ Reads one regular, single-link file once without following a link and answers its bytes only when they are the remembered bytes and stayed unchanged while they were read. */
async function readRememberedV1(absolute:string,sha256:string):Promise<Uint8Array|undefined>{
 let handle;
 try{handle=await open(absolute,constants.O_RDONLY|(constants.O_NOFOLLOW??0));}catch{return undefined;}
 try{
  const before=await handle.stat({bigint:true});
  if(!before.isFile()||before.nlink!==1n||before.size>MAXIMUM_FILE_BYTES)return undefined;
  const bytes=Buffer.alloc(Number(before.size));
  for(let offset=0;offset<bytes.length;){const row=await handle.read(bytes,offset,bytes.length-offset,offset);if(!row.bytesRead)return undefined;offset+=row.bytesRead;}
  const tail=await handle.read(Buffer.alloc(1),0,1,bytes.length),after=await handle.stat({bigint:true}),named=await lstat(absolute,{bigint:true});
  if(tail.bytesRead||named.isSymbolicLink()||stamp(before)!==stamp(after)||stamp(before)!==stamp(named))return undefined;
  return digestOf(bytes)===sha256?bytes:undefined;
 }catch{return undefined;}finally{try{await handle.close();}catch{}}
}

async function absentV1(absolute:string):Promise<boolean>{
 try{await lstat(absolute);return false;}catch(error){const code=(error as NodeJS.ErrnoException).code;return code==="ENOENT"||code==="ENOTDIR";}
}

async function listedV1(absolute:string,entries:readonly(readonly[string,"d"|"f"])[]):Promise<boolean>{
 try{
  const facts=await lstat(absolute);
  if(!facts.isDirectory()||facts.isSymbolicLink())return false;
  const rows=await readdir(absolute,{withFileTypes:true}),seen:(readonly[string,"d"|"f"])[]=[];
  for(const row of rows){if(row.isSymbolicLink())return false;if(row.isDirectory())seen.push([row.name,"d"]);else if(row.isFile())seen.push([row.name,"f"]);else return false;}
  seen.sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])));
  return seen.length===entries.length&&seen.every((row,index)=>row[0]===entries[index]![0]&&row[1]===entries[index]![1]);
 }catch{return false;}
}

async function directoryHoldsV1(row:Readonly<{path:string;dev:string;ino:string}>,root:string):Promise<boolean>{
 try{const facts=await lstat(join(root,row.path||"."),{bigint:true});return facts.isDirectory()&&!facts.isSymbolicLink()&&String(facts.dev)===row.dev&&String(facts.ino)===row.ino;}catch{return false;}
}

/** 🔁️ Proves the memo against the present tree: every remembered file reads back with its remembered digest, every ancestor directory is the same directory, every remembered absence and listing is unchanged; answers the file bytes, or nothing when anything differs. Package observations are proven by the caller, which owns resolution. */
export async function replayNxAuthorizerMemoV1(root:string,memo:NxAuthorizerMemoV1,signal:AbortSignal):Promise<ReadonlyMap<string,Uint8Array>|undefined>{
 const base=resolve(root),contents=new Map<string,Uint8Array>();
 if(memo.version!==1||memo.root!==base||memo.implementation!==nxAuthorizerMemoImplementationV1())return undefined;
 const directories=await pool(memo.directories,row=>directoryHoldsV1(row,base),signal);
 if(!directories)return undefined;
 const observed=await pool(memo.observations.filter(row=>row.kind!=="package"),async row=>row.kind==="absent"?absentV1(join(base,row.path)):row.kind==="directory"?listedV1(join(base,row.path),row.entries):true,signal);
 if(!observed)return undefined;
 const files=await pool(memo.manifest.sources,async row=>{const bytes=await readRememberedV1(join(base,row.path),row.sha256);if(!bytes)return false;contents.set(row.path,bytes);return true;},signal);
 return files?contents:undefined;
}

/** 📚️ Lists the distinct directories a set of captured files lives in, from the root down, with their identity at this moment. */
export async function nxAuthorizerMemoDirectoriesV1(root:string,paths:readonly string[]):Promise<readonly Readonly<{path:string;dev:string;ino:string}>[]|undefined>{
 const names=new Set<string>([""]);
 for(const path of paths){let current=dirname(path);while(current!=="."&&!names.has(current)){names.add(current);current=dirname(current);}}
 const rows:{path:string;dev:string;ino:string}[]=[];
 try{
  for(const path of[...names].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)))){
   const facts=await lstat(join(root,path||"."),{bigint:true});
   if(!facts.isDirectory()||facts.isSymbolicLink())return undefined;
   rows.push({path,dev:String(facts.dev),ino:String(facts.ino)});
  }
 }catch{return undefined;}
 return rows;
}

/** 📥️ Loads a memo file; a missing, torn or foreign file is no memo. */
export async function readNxAuthorizerMemoV1(file:string):Promise<NxAuthorizerMemoV1|undefined>{
 try{
  const value=JSON.parse(await readFile(file,"utf8")) as Partial<NxAuthorizerMemoV1>;
  return value&&value.version===1&&typeof value.root==="string"&&typeof value.entry==="string"&&Array.isArray(value.directories)&&Array.isArray(value.observations)&&value.manifest&&Array.isArray(value.manifest.sources)&&value.marks&&value.imports?value as NxAuthorizerMemoV1:undefined;
 }catch{return undefined;}
}

/** 📤️ Stores a memo atomically with owner-only permissions; a failure to store costs only the next capture and is never an error. */
export async function writeNxAuthorizerMemoV1(file:string,memo:NxAuthorizerMemoV1):Promise<void>{
 const temporary=file+"."+process.pid+"."+Date.now()+".tmp";
 try{
  await mkdir(dirname(file),{recursive:true,mode:0o700});
  await writeFile(temporary,JSON.stringify(memo),{mode:0o600});
  await rename(temporary,file);
 }catch{try{await rm(temporary,{force:true});}catch{}}
}
