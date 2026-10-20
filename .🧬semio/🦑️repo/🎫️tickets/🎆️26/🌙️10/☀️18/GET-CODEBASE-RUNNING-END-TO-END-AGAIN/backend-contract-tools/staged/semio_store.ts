import {constants} from "node:fs";
import {copyFile,link,lstat,mkdir,open,readFile,readdir,readlink,rename,rm,stat,symlink,utimes,writeFile,chmod} from "node:fs/promises";
import {randomUUID} from "node:crypto";
import {createReadStream} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join,relative,resolve,sep,isAbsolute} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {SEMIO_POLICY_V1 as policy,semioOutputsDigestV1,type SemioEntryV1,type SemioFileV1} from "../🟦️.ts";

const slash=(value:string)=>value.split(sep).join("/"),wait=(milliseconds:number,signal?:AbortSignal)=>new Promise<void>((accept,refuse)=>{const timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);accept();},milliseconds),abort=()=>{clearTimeout(timer);refuse(signal!.reason??Error("Cancelled"));};if(signal?.aborted)abort();else signal?.addEventListener("abort",abort,{once:true});});

/** 🔏️ Hashes a file by streaming it. */
export function semioFileDigestV1(path:string):Promise<string>{
 return new Promise((accept,refuse)=>{const hash=createHash("sha256"),stream=createReadStream(path);stream.on("data",chunk=>hash.update(chunk));stream.once("error",refuse);stream.once("end",()=>accept(hash.digest("hex")));});
}

/** 🧭️ Keeps a workspace-relative path inside the workspace. */
export function semioInsideV1(workspace:string,path:string):string{
 const full=resolve(workspace,path),inner=relative(resolve(workspace),full);
 if(inner===""||inner.startsWith("..")||isAbsolute(inner))throw Error(`A workspace-relative path is required: ${path}`);
 return full;
}

/** 🗄️ The local content-addressed action cache: output blobs by digest, entries by action key, locks per key. Every write is a temporary file renamed into place, so a reader never sees a partial entry. */
export class SemioStoreV1{
 constructor(readonly directory:string){}
 blob(sha:string):string{return join(this.directory,"cas",sha.slice(0,2),sha);}
 entry(key:string):string{return join(this.directory,"ac",key.slice(0,2),key+".json");}
 private async place(target:string,write:(temporary:string)=>Promise<void>):Promise<void>{
  const temporary=join(this.directory,"tmp",randomUUID());
  await mkdir(dirname(temporary),{recursive:true});
  try{await write(temporary);await mkdir(dirname(target),{recursive:true});await rename(temporary,target);}finally{await rm(temporary,{force:true});}
 }
 /** 📥️ Stores a blob under its digest unless it is already there. */
 async putBlob(sha:string,bytes:Uint8Array):Promise<void>{
  try{await stat(this.blob(sha));return;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  await this.place(this.blob(sha),temporary=>writeFile(temporary,bytes));
 }
 /** 📤️ Reads a blob and checks its digest. */
 async readBlob(sha:string):Promise<Uint8Array>{
  const bytes=await readFile(this.blob(sha));
  if(createHash("sha256").update(bytes).digest("hex")!==sha)throw Error(`Corrupt cache blob ${sha}`);
  return bytes;
 }
 /** 🔎️ Reads the entry of a key when it exists, is well formed and all its blobs are present; otherwise it is a miss. */
 async lookup(key:string):Promise<SemioEntryV1|undefined>{
  let text:string;
  try{text=await readFile(this.entry(key),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
  let entry:SemioEntryV1;
  try{entry=JSON.parse(text);}catch{return undefined;}
  if(validateJsonSchemaSubset(schema.definitions.Entry,entry,schema).length||entry.key!==key||semioOutputsDigestV1(entry.files)!==entry.digest)return undefined;
  for(const file of entry.files)if(file.kind==="file"){try{await stat(this.blob(file.sha256!));}catch{return undefined;}}
  return entry;
 }
 /** 📝️ Publishes an entry atomically; a concurrent publisher of the same key writes identical content. */
 async publish(entry:SemioEntryV1):Promise<void>{await this.place(this.entry(entry.key),temporary=>writeFile(temporary,JSON.stringify(entry)));}
 /** 📸️ Reads the outputs a task declared from the workspace; with `copy` the file contents also enter the blob store. A declared output that was not produced is returned in `missing`. */
 async capture(workspace:string,task:string,key:string,patterns:readonly string[],copy:boolean,signal?:AbortSignal):Promise<{entry:SemioEntryV1;missing:string[]}>{
  const roots=new Set<string>(),missing:string[]=[],files:SemioFileV1[]=[];
  for(const pattern of patterns){
   const path=slash(relative(workspace,resolve(workspace,pattern)));
   semioInsideV1(workspace,path);
   if(/[?*{[]/u.test(path)){for await(const match of new Bun.Glob(path).scan({cwd:workspace,onlyFiles:false,dot:true}))roots.add(slash(match));}
   else try{await lstat(join(workspace,path));roots.add(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;missing.push(path);}
  }
  const visit=async(path:string):Promise<void>=>{
   if(signal?.aborted)throw signal.reason??Error("Cancelled");
   if(files.length>policy.maximumOutputFiles)throw Error("Bounded output file count required");
   const source=join(workspace,path),facts=await lstat(source),mode=facts.mode&0o777;
   if(facts.isSymbolicLink())files.push({path,kind:"symlink",mode,link:await readlink(source)});
   else if(facts.isDirectory()){files.push({path,kind:"directory",mode});for(const child of (await readdir(source)).sort())await visit(path+"/"+child);}
   else{const sha=await semioFileDigestV1(source);files.push({path,kind:"file",mode,sha256:sha,size:facts.size});if(copy)try{await stat(this.blob(sha));}catch{await this.place(this.blob(sha),temporary=>copyFile(source,temporary,constants.COPYFILE_FICLONE));}}
  };
  const sorted=[...roots].sort();
  for(const path of sorted.filter(path=>!sorted.some(parent=>parent!==path&&path.startsWith(parent+"/"))))await visit(path);
  files.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
  return {entry:{version:1,key,task,files,digest:semioOutputsDigestV1(files)},missing};
 }
 /** ♻️ Replaces the workspace's outputs with an entry's: a matching tree is left alone, anything else is removed and written again from the blobs. */
 async restore(workspace:string,entry:SemioEntryV1,signal?:AbortSignal):Promise<void>{
  const roots=entry.files.map(file=>file.path).filter((path,_at,all)=>!all.some(parent=>parent!==path&&path.startsWith(parent+"/")));
  if(await this.matches(workspace,entry))return;
  for(const root of roots){if(signal?.aborted)throw signal.reason??Error("Cancelled");await rm(semioInsideV1(workspace,root),{recursive:true,force:true});}
  for(const file of entry.files){
   if(signal?.aborted)throw signal.reason??Error("Cancelled");
   const target=semioInsideV1(workspace,file.path);
   await mkdir(dirname(target),{recursive:true});
   if(file.kind==="directory")await mkdir(target,{recursive:true});
   else if(file.kind==="symlink")await symlink(file.link!,target);
   else{await copyFile(this.blob(file.sha256!),target,constants.COPYFILE_FICLONE);await chmod(target,file.mode);}
  }
  for(const file of [...entry.files].filter(file=>file.kind==="directory").sort((left,right)=>right.path.length-left.path.length))await chmod(semioInsideV1(workspace,file.path),file.mode);
 }
 private async matches(workspace:string,entry:SemioEntryV1):Promise<boolean>{
  try{return (await this.capture(workspace,entry.task,entry.key,entry.files.filter((file,_at,all)=>!all.some(parent=>parent!==file&&file.path.startsWith(parent.path+"/"))).map(file=>file.path),false)).entry.digest===entry.digest;}catch{return false;}
 }
 /** 🚦️ Runs `body` as the single owner of an action key across processes. A waiter polls until the owner releases the lock or the entry appears, then reports `owner: false` so the caller restores instead of executing. A lock whose owner died or stopped refreshing is broken. */
 async withFlight<T>(key:string,signal:AbortSignal,body:(owner:boolean)=>Promise<T>):Promise<T>{
  const lock=join(this.directory,"locks",key+".lock");
  await mkdir(dirname(lock),{recursive:true});
  for(;;){
   if(signal.aborted)throw signal.reason??Error("Cancelled");
   try{
    const handle=await open(lock,"wx");
    await handle.writeFile(JSON.stringify({pid:process.pid,at:Date.now()}));await handle.close();
    const beat=setInterval(()=>{const now=new Date();utimes(lock,now,now).catch(()=>{});},policy.lockHeartbeatMilliseconds);
    try{const published=await this.lookup(key)!==undefined;return await body(!published);}finally{clearInterval(beat);await rm(lock,{force:true});}
   }catch(error){
    if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   }
   if(await this.lookup(key))return body(false);
   await this.breakStale(lock);
   await wait(policy.lockPollMilliseconds,signal);
  }
 }
 private async breakStale(lock:string):Promise<void>{
  try{
   const facts=await stat(lock);let dead=false;
   try{const owner=JSON.parse(await readFile(lock,"utf8")) as {pid:number};try{process.kill(owner.pid,0);}catch(error){dead=(error as NodeJS.ErrnoException).code==="ESRCH";}}catch{dead=false;}
   if(dead||Date.now()-facts.mtimeMs>policy.lockStaleMilliseconds)await rm(lock,{force:true});
  }catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 }
 /** 📊️ Counts the stored entries and the bytes of the store. */
 async status():Promise<{entries:number;bytes:number}>{
  let entries=0,bytes=0;
  const walk=async(directory:string,count:boolean):Promise<void>=>{
   let names:import("node:fs").Dirent[];
   try{names=await readdir(directory,{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
   for(const name of names){const path=join(directory,name.name);if(name.isDirectory())await walk(path,count);else{bytes+=(await stat(path)).size;if(count)entries++;}}
  };
  await walk(join(this.directory,"ac"),true);await walk(join(this.directory,"cas"),false);
  return {entries,bytes};
 }
 /** 🧹️ Removes every entry, blob and stale temporary file. */
 async clear():Promise<void>{for(const name of ["ac","cas","tmp"])await rm(join(this.directory,name),{recursive:true,force:true});}
}
