import {spawn,spawnSync} from "node:child_process";
import {createHash,randomUUID} from "node:crypto";
import {accessSync,constants,createReadStream,createWriteStream,statSync} from "node:fs";
import {chmod,copyFile,lstat,mkdir,readdir,readlink,realpath,rename,rm,stat,symlink,utimes,writeFile} from "node:fs/promises";
import {hostname,platform as osPlatform} from "node:os";
import {delimiter,dirname,isAbsolute,join,normalize,sep} from "node:path";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1,type ProtobufMessageV1} from "../📡️wire/📦️protobuf/🟦️.ts";
import {httpRemoteExecutionContentV1,RemoteExecutionContentErrorV1,type RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {selectRemoteExecutionSandboxV1,type RemoteExecutionSandboxStrategyV1} from "../🔒️sandbox/🟦️.ts";
import {readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,remoteExecutionDigestV1,requireRemoteExecutionShapeV1,type RemoteExecutionDigestV1,type RemoteExecutionHeartbeatAnswerV1,type RemoteExecutionPollAnswerV1,type RemoteExecutionPropertyV1,type RemoteExecutionSandboxV1,type RemoteExecutionTaskResultV1,type RemoteExecutionTaskStatusV1,type RemoteExecutionTaskV1,type RemoteExecutionWorkerRegisteredV1} from "../🟦️.ts";
import { runtimeSleepV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";
import {runtimeIdentityV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

const EMPTY_HASH="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/** 📣️ Something a worker did, for logs and progress. */
export type RemoteExecutionWorkerEventV1=Readonly<{stage:"registered"|"reconnecting"|"started"|"finished"|"cancelled"|"stopped"|"sandbox";detail:string}>;

/** ⚙️ Everything a worker needs: where the scheduler is, who it is, what it offers and where it works. */
export type RemoteExecutionWorkerOptionsV1=Readonly<{
 scheduler:string;token?:string|undefined;cacheToken?:string|undefined;name?:string;concurrency:number;directory:string;sandbox:"auto"|RemoteExecutionSandboxV1;materialize:"clone"|"copy";maximumBlobBytes:number;inlineOutputBytes:number;
 properties?:readonly RemoteExecutionPropertyV1[];platform?:string;signal:AbortSignal;onEvent?:(event:RemoteExecutionWorkerEventV1)=>void;content?:RemoteExecutionContentV1;keepWork?:boolean;
}>;

/** 🎬️ What executing one action needs besides the store. */
export type RemoteExecutionExecutionV1=Readonly<{
 instance:string;action:RemoteExecutionDigestV1;timeoutSeconds:number;content:RemoteExecutionContentV1;directory:string;work?:string;workerName:string;sandbox:RemoteExecutionSandboxStrategyV1;materialize:"clone"|"copy";inlineOutputBytes:number;signal:AbortSignal;keepWork?:boolean;blobs?:RemoteExecutionBlobCacheV1;
}>;

/** 🏁️ How executing one action ended. */
export type RemoteExecutionExecutionResultV1=Readonly<{outcome:"completed"|"failed"|"cancelled";status:RemoteExecutionTaskStatusV1;actionResult:Uint8Array|null}>;

class MissingBlob extends Error{constructor(readonly digest:RemoteExecutionDigestV1){super(`Missing blob ${digest.hash}/${digest.size}`);}}

const sha256=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const digestOf=(data:Uint8Array):RemoteExecutionDigestV1=>({hash:sha256(data),size:data.length});
const wire=(digest:RemoteExecutionDigestV1)=>({hash:digest.hash,size_bytes:digest.size});
const unwire=(digest:{hash:string;size_bytes:number}|undefined):RemoteExecutionDigestV1=>{
 if(!digest)throw Error("Digest required");
 return remoteExecutionDigestV1(digest.hash,digest.size_bytes);
};
const timestamp=(milliseconds:number)=>({seconds:Math.floor(milliseconds/1000),nanos:(milliseconds%1000)*1000000});

async function limited<T>(items:readonly T[],limit:number,work:(item:T)=>Promise<void>):Promise<void>{
 let next=0,failure:unknown;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{
  while(failure===undefined){const index=next++;if(index>=items.length)return;try{await work(items[index]!);}catch(error){failure??=error;}}
 }));
 if(failure!==undefined)throw failure;
}

/** 💽️ The worker's local, bounded, content-addressed blob cache: every blob is fetched once, verified against its digest and written atomically; least recently used blobs are evicted beyond the bound. */
export class RemoteExecutionBlobCacheV1{
 private readonly pending=new Map<string,Promise<string>>();
 constructor(readonly directory:string,private readonly content:RemoteExecutionContentV1,private readonly maximumBytes:number){}

 private location(hash:string):string{return join(this.directory,hash.slice(0,2),hash);}

 async path(instance:string,digest:RemoteExecutionDigestV1,signal?:AbortSignal):Promise<string>{
  const file=this.location(digest.hash),known=await stat(file).catch(()=>undefined);
  if(known&&known.size===digest.size){const now=new Date();await utimes(file,now,now).catch(()=>{});return file;}
  const key=instance+"|"+digest.hash;let running=this.pending.get(key);
  if(!running){running=this.fetch(instance,digest,file,signal).finally(()=>{this.pending.delete(key);});this.pending.set(key,running);}
  return running;
 }

 private async fetch(instance:string,digest:RemoteExecutionDigestV1,file:string,signal?:AbortSignal):Promise<string>{
  await mkdir(dirname(file),{recursive:true});
  const temporary=`${file}.${process.pid}.${randomUUID()}.tmp`;
  if(digest.size===0){await writeFile(temporary,"");await rename(temporary,file);return file;}
  const source=await this.content.stream(instance,digest,signal);
  if(!source)throw new MissingBlob(digest);
  const hash=createHash("sha256"),handle=createWriteStream(temporary),endStream=():Promise<void>=>new Promise((resolve,reject)=>{handle.once("error",reject);handle.end(()=>resolve());});let size=0;
  try{
   for await(const chunk of source){hash.update(chunk);size+=chunk.length;if(!handle.write(chunk))await new Promise<void>(resolve=>handle.once("drain",resolve));}
   await endStream();
   if(size!==digest.size||hash.digest("hex")!==digest.hash)throw new RemoteExecutionContentErrorV1(15,`Blob ${digest.hash} does not match its digest`);
   await rename(temporary,file);return file;
  }catch(error){try{await endStream();}catch{}await rm(temporary,{force:true});throw error;}
 }

 /** 🧹️ Removes the least recently used blobs until the cache fits its bound. */
 async prune(dryRun=false):Promise<number>{
  const rows:{path:string;size:number;time:number}[]=[];let total=0;
  for(const shard of await readdir(this.directory).catch(()=>[] as string[])){
   for(const name of await readdir(join(this.directory,shard)).catch(()=>[] as string[])){
    const path=join(this.directory,shard,name),info=await stat(path).catch(()=>undefined);
    if(!info)continue;
    if(name.endsWith(".tmp")){if(Date.now()-info.mtimeMs>3600000)await rm(path,{force:true});continue;}
    rows.push({path,size:info.size,time:info.mtimeMs});total+=info.size;
   }
  }
  rows.sort((left,right)=>left.time-right.time);let removed=0;
  if(total<=this.maximumBytes)return 0;
  for(const row of rows){if(total<=this.maximumBytes*0.9)break;if(!dryRun)await rm(row.path,{force:true});total-=row.size;removed++;}
  return removed;
 }
}

function safeName(name:string):string{
 if(name===""||name==="."||name===".."||name.includes("/")||name.includes("\\")||name.includes("\0"))throw new RemoteExecutionContentErrorV1(3,`Valid path segment required: ${JSON.stringify(name)}`);
 return name;
}

function safeRelative(path:string):string{
 const clean=normalize(path.replace(/\/+$/u,""));
 if(clean===""||clean==="."||isAbsolute(clean)||clean===".."||clean.startsWith(".."+sep))throw new RemoteExecutionContentErrorV1(3,`Relative path inside the input root required: ${path}`);
 return clean;
}

async function placeFile(source:string,target:string,executable:boolean,mode:"clone"|"copy"):Promise<void>{
 await copyFile(source,target,mode==="clone"?constants.COPYFILE_FICLONE:0);
 await chmod(target,executable?0o755:0o644);
}

function killTree(pid:number|undefined):void{
 if(pid===undefined)return;
 try{
  if(process.platform==="win32")spawnSync("taskkill",["/pid",String(pid),"/T","/F"],{stdio:"ignore"});
  else process.kill(-pid,"SIGKILL");
 }catch{}
}

function resolveExecutable(first:string,env:Readonly<Record<string,string>>,root:string,cwd:string):string|undefined{
 const extensions=process.platform==="win32"?["",...(env.PATHEXT??".EXE;.CMD;.BAT").split(";")]:[""];
 const usable=(path:string):boolean=>{for(const extension of extensions){try{if(statSync(path+extension).isFile()){accessSync(path+extension,constants.X_OK);return true;}}catch{}}return false;};
 if(first.includes("/")||first.includes("\\")){const full=isAbsolute(first)?first:join(root,first);return usable(full)?full:undefined;}
 for(const directory of (env.PATH??"").split(delimiter)){if(directory==="")continue;const full=join(isAbsolute(directory)?directory:join(cwd,directory),first);if(usable(full))return full;}
 return undefined;
}

type Collected={exitCode:number;stdout:Uint8Array;stderr:Uint8Array;timedOut:boolean;cancelled:boolean};
const LIMIT=64*1048576;

function execute(argv:readonly string[],env:Readonly<Record<string,string>>,cwd:string,timeoutMilliseconds:number,signal:AbortSignal):Promise<Collected>{
 return new Promise(resolve=>{
  const out:Buffer[]=[],err:Buffer[]=[];let outSize=0,errSize=0,timedOut=false,cancelled=false,settled=false;
  const finish=(exitCode:number,extra?:string):void=>{
   if(settled)return;settled=true;clearTimeout(timer);signal.removeEventListener("abort",abort);
   if(extra)err.push(Buffer.from(extra));
   resolve({exitCode,stdout:Buffer.concat(out),stderr:Buffer.concat(err),timedOut,cancelled});
  };
  let child;
  try{child=spawn(argv[0]!,argv.slice(1),{cwd,env:{...env},stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true});}
  catch(error){finish(127,`Cannot start ${argv[0]}: ${(error as Error).message}\n`);return;}
  const abort=():void=>{cancelled=true;killTree(child.pid);},timer=setTimeout(()=>{timedOut=true;killTree(child.pid);},timeoutMilliseconds);
  signal.addEventListener("abort",abort,{once:true});
  child.stdout.on("data",(chunk:Buffer)=>{if(outSize<LIMIT){out.push(chunk);outSize+=chunk.length;}});
  child.stderr.on("data",(chunk:Buffer)=>{if(errSize<LIMIT){err.push(chunk);errSize+=chunk.length;}});
  child.on("error",error=>{finish(127,`Cannot start ${argv[0]}: ${error.message}\n`);});
  child.on("close",(code,received)=>{killTree(child.pid);finish(code??(received?128+((({SIGHUP:1,SIGINT:2,SIGQUIT:3,SIGABRT:6,SIGKILL:9,SIGSEGV:11,SIGPIPE:13,SIGTERM:15}) as Record<string,number>)[received]??0):1));});
  if(signal.aborted)abort();
 });
}

type Directory=ProtobufMessageV1;

/** 🧰️ Executes one action the way the protocol describes: fetch the action, its command and its input tree, build a private input root, run the command in the sandbox with exactly the declared environment, collect the declared outputs into the store and answer an ActionResult. Failures of the infrastructure are statuses, failures of the action are exit codes. */
export async function executeRemoteExecutionActionV1(run:RemoteExecutionExecutionV1):Promise<RemoteExecutionExecutionResultV1>{
 const started=Date.now(),task=randomUUID(),work=join(run.work??join(run.directory,"work"),task),root=join(work,"root"),blobs=run.blobs??new RemoteExecutionBlobCacheV1(join(run.directory,"blobs"),run.content,Number.MAX_SAFE_INTEGER);
 const fail=(code:number,message:string,missing?:RemoteExecutionDigestV1[]):RemoteExecutionExecutionResultV1=>({outcome:"failed",status:{code,message,...(missing?{missing}:{})},actionResult:null});
 const directories=new Map<string,Directory>();
 const loadDirectory=async(digest:RemoteExecutionDigestV1):Promise<Directory>=>{
  const known=directories.get(digest.hash);if(known)return known;
  const bytes=digest.size===0?new Uint8Array(0):await run.content.read(run.instance,digest,run.signal);
  if(!bytes)throw new MissingBlob(digest);
  if(sha256(bytes)!==digest.hash)throw new RemoteExecutionContentErrorV1(15,`Directory ${digest.hash} does not match its digest`);
  const directory=decodeProtobufV1(REAPI_V1.directory,bytes);directories.set(digest.hash,directory);return directory;
 };
 const materialize=async(digest:RemoteExecutionDigestV1,target:string,depth:number):Promise<void>=>{
  if(depth>256)throw new RemoteExecutionContentErrorV1(3,"Input tree too deep");
  const directory=await loadDirectory(digest);
  await mkdir(target,{recursive:true});
  const seen=new Set<string>();
  for(const name of [...directory.files,...directory.directories,...directory.symlinks].map((node:{name:string})=>safeName(node.name))){if(seen.has(name))throw new RemoteExecutionContentErrorV1(3,`Duplicate name in input tree: ${name}`);seen.add(name);}
  await limited(directory.files as {name:string;digest:{hash:string;size_bytes:number};is_executable:boolean}[],16,async node=>{
   const source=await blobs.path(run.instance,unwire(node.digest),run.signal);
   await placeFile(source,join(target,node.name),node.is_executable,run.materialize);
  });
  for(const node of directory.symlinks as {name:string;target:string}[])await symlink(node.target,join(target,node.name));
  await limited(directory.directories as {name:string;digest:{hash:string;size_bytes:number}}[],8,node=>materialize(unwire(node.digest),join(target,node.name),depth+1));
 };
 try{
  const actionBytes=await run.content.read(run.instance,run.action,run.signal);
  if(!actionBytes)return fail(9,`Missing blob ${run.action.hash}/${run.action.size}`,[run.action]);
  const action=decodeProtobufV1(REAPI_V1.action,actionBytes),commandDigest=unwire(action.command_digest),commandBytes=await run.content.read(run.instance,commandDigest,run.signal);
  if(!commandBytes)return fail(9,`Missing blob ${commandDigest.hash}/${commandDigest.size}`,[commandDigest]);
  const command=decodeProtobufV1(REAPI_V1.command,commandBytes) as {arguments:string[];environment_variables:{name:string;value:string}[];output_files:string[];output_directories:string[];output_paths:string[];working_directory:string};
  if(command.arguments.length===0)return fail(3,"Command without arguments");
  const rootDigest=action.input_root_digest?unwire(action.input_root_digest):{hash:EMPTY_HASH,size:0},fetchStart=Date.now();
  await materialize(rootDigest,root,0);
  const cwdRelative=command.working_directory===""?"":safeRelative(command.working_directory),cwd=cwdRelative===""?root:join(root,cwdRelative),legacy=command.output_paths.length===0,declared=legacy?[...command.output_files,...command.output_directories]:command.output_paths;
  await mkdir(cwd,{recursive:true});
  for(const path of declared){const clean=safeRelative(path),directory=dirname(join(cwd,clean));await mkdir(directory,{recursive:true});}
  for(const path of legacy?command.output_directories:[])await mkdir(join(cwd,safeRelative(path)),{recursive:true});
  const env:Record<string,string>={};
  for(const variable of command.environment_variables)env[variable.name]=variable.value;
  const scratch=join(root,".tmp");await mkdir(scratch,{recursive:true});
  if(env.TMPDIR===undefined&&process.platform!=="win32")env.TMPDIR=scratch;
  const first=command.arguments[0]!,executable=resolveExecutable(first,env,root,cwd),executionStart=Date.now();
  const collected:Collected=executable===undefined?{exitCode:127,stdout:new Uint8Array(0),stderr:new TextEncoder().encode(`Executable not found: ${first}\n`),timedOut:false,cancelled:false}:await(async()=>{
   const wrapped=run.sandbox.wrap({argv:[executable,...command.arguments.slice(1)],env,cwd,root,network:false});
   return execute(wrapped.argv,wrapped.env,wrapped.cwd,run.timeoutSeconds*1000,run.signal);
  })(),executionEnd=Date.now();
  if(collected.cancelled||run.signal.aborted)return {outcome:"cancelled",status:{code:1,message:"Cancelled"},actionResult:null};
  if(collected.timedOut)return fail(4,`Action exceeded its timeout of ${run.timeoutSeconds}s`);
  const uploadStart=Date.now(),uploads=new Map<string,{digest:RemoteExecutionDigestV1;source:Uint8Array|string}>();
  const stage=(digest:RemoteExecutionDigestV1,source:Uint8Array|string)=>{if(digest.size>0&&!uploads.has(digest.hash))uploads.set(digest.hash,{digest,source});};
  const hashFile=(path:string):Promise<string>=>new Promise((resolve,reject)=>{const hash=createHash("sha256");createReadStream(path).on("data",chunk=>hash.update(chunk)).on("error",reject).on("end",()=>{resolve(hash.digest("hex"));});});
  const fileNode=async(path:string,name:string)=>{const info=await stat(path),digest={hash:await hashFile(path),size:info.size};stage(digest,path);return {name,digest:wire(digest),is_executable:(info.mode&0o111)!==0};};
  const walk=async(path:string,collect:Directory[]):Promise<RemoteExecutionDigestV1>=>{
   const files:ProtobufMessageV1[]=[],children:ProtobufMessageV1[]=[],links:ProtobufMessageV1[]=[],entries=(await readdir(path)).sort();
   for(const name of entries){
    const full=join(path,name),info=await lstat(full);
    if(info.isSymbolicLink())links.push({name,target:await readlink(full)});
    else if(info.isDirectory())children.push({name,digest:wire(await walk(full,collect))});
    else if(info.isFile())files.push(await fileNode(full,name));
   }
   const message={files,directories:children,symlinks:links},bytes=encodeProtobufV1(REAPI_V1.directory,message),digest=digestOf(bytes);collect.push(message);stage(digest,bytes);return digest;
  };
  const outputFiles:ProtobufMessageV1[]=[],outputDirectories:ProtobufMessageV1[]=[],outputSymlinks:ProtobufMessageV1[]=[],fileSymlinks:ProtobufMessageV1[]=[],directorySymlinks:ProtobufMessageV1[]=[];
  const realRoot=await realpath(root);
  for(const path of declared){
   const clean=safeRelative(path),full=join(cwd,clean),parent=await realpath(dirname(full)).catch(()=>undefined);
   if(parent!==undefined&&parent!==realRoot&&!parent.startsWith(realRoot+sep))return fail(3,`Output path leaves the input root: ${path}`);
   const info=await lstat(full).catch(()=>undefined);
   if(!info)continue;
   if(info.isSymbolicLink()){
    const row={path:clean,target:await readlink(full)};outputSymlinks.push(row);
    if(legacy)(command.output_directories.includes(path)?directorySymlinks:fileSymlinks).push(row);
   }else if(info.isDirectory()){
    const collect:Directory[]=[],rootDirectory=await walk(full,collect),rootMessage=collect[collect.length-1]!,tree={root:rootMessage,children:collect.slice(0,-1)},treeBytes=encodeProtobufV1(REAPI_V1.tree,tree),treeDigest=digestOf(treeBytes);
    stage(treeDigest,treeBytes);outputDirectories.push({path:clean,tree_digest:wire(treeDigest),root_directory_digest:wire(rootDirectory)});
   }else if(info.isFile())outputFiles.push({path:clean,...(await fileNode(full,clean).then(({digest,is_executable})=>({digest,is_executable})))});
  }
  const stdoutDigest=digestOf(collected.stdout),stderrDigest=digestOf(collected.stderr);
  const inlineOut=collected.stdout.length<=run.inlineOutputBytes,inlineErr=collected.stderr.length<=run.inlineOutputBytes;
  if(!inlineOut)stage(stdoutDigest,collected.stdout);
  if(!inlineErr)stage(stderrDigest,collected.stderr);
  const rows=[...uploads.values()],absent=new Set((await run.content.missing(run.instance,rows.map(row=>row.digest),run.signal)).map(digest=>digest.hash));
  await limited(rows.filter(row=>absent.has(row.digest.hash)),8,async row=>{
   if(typeof row.source==="string"){
    const stream=createReadStream(row.source);
    await run.content.write(run.instance,row.digest,(async function*(){for await(const chunk of stream as unknown as AsyncIterable<Uint8Array>)yield chunk;})(),run.signal);
   }else await run.content.write(run.instance,row.digest,row.source,run.signal);
  });
  const end=Date.now(),result=encodeProtobufV1(REAPI_V1.actionResult,{
   output_files:outputFiles.sort((left,right)=>left.path<right.path?-1:1),output_directories:outputDirectories,output_symlinks:outputSymlinks,...(legacy?{output_file_symlinks:fileSymlinks,output_directory_symlinks:directorySymlinks}:{}),
   exit_code:collected.exitCode,...(inlineOut?{stdout_raw:collected.stdout}:{stdout_digest:wire(stdoutDigest)}),...(inlineErr?{stderr_raw:collected.stderr}:{stderr_digest:wire(stderrDigest)}),
   execution_metadata:{worker:run.workerName,worker_start_timestamp:timestamp(started),input_fetch_start_timestamp:timestamp(fetchStart),input_fetch_completed_timestamp:timestamp(executionStart),execution_start_timestamp:timestamp(executionStart),execution_completed_timestamp:timestamp(executionEnd),output_upload_start_timestamp:timestamp(uploadStart),output_upload_completed_timestamp:timestamp(end),worker_completed_timestamp:timestamp(end)},
  });
  return {outcome:"completed",status:{code:0,message:""},actionResult:result};
 }catch(error){
  if(run.signal.aborted)return {outcome:"cancelled",status:{code:1,message:"Cancelled"},actionResult:null};
  if(error instanceof MissingBlob)return fail(9,error.message,[error.digest]);
  if(error instanceof RemoteExecutionContentErrorV1)return fail(error.code,error.message);
  return fail(14,`Executing the action failed: ${error instanceof Error?error.message:String(error)}`);
 }finally{
  if(run.keepWork!==true)await rm(work,{recursive:true,force:true,maxRetries:3}).catch(()=>{});
 }
}

/** 🧾️ Collects the workers' home for `repo cache gc`: trims the shared blob cache to its bound and removes private input roots that no running worker can still own (idle for a day); answers one language-neutral line for the ledger report. */
export async function collectRemoteExecutionForLedgerV1(workspace:string,dryRun:boolean,signal:AbortSignal):Promise<string>{
 const policy=readRemoteExecutionPolicyV1().worker,home=isAbsolute(policy.directory)?policy.directory:join(workspace,policy.directory),cache=new RemoteExecutionBlobCacheV1(join(home,"blobs"),undefined as never,policy.maximumBlobBytes);
 if(!await stat(home).then(info=>info.isDirectory(),()=>false))return "absent";
 let stale=0;
 for(const worker of await readdir(join(home,"work")).catch(()=>[] as string[])){
  for(const task of await readdir(join(home,"work",worker)).catch(()=>[] as string[])){
   if(signal.aborted)return "cancelled";
   const path=join(home,"work",worker,task),info=await stat(path).catch(()=>undefined);
   if(info&&Date.now()-info.mtimeMs>86400000){stale++;if(!dryRun)await rm(path,{recursive:true,force:true}).catch(()=>{});}
  }
 }
 const removed=await cache.prune(dryRun);
 return `${dryRun?"would remove":"removed"} ${removed} blobs and ${stale} stale input roots`;
}

/** 📮️ Talks to the scheduler's worker API. */
function schedulerClient(base:string,token:string|undefined){
 const root=base.replace(/\/+$/u,"");
 return async<T>(method:string,path:string,body:unknown,signal:AbortSignal,timeout=30000):Promise<T|undefined>=>{
  const response=await fetch(root+path,{method,signal:AbortSignal.any([signal,AbortSignal.timeout(timeout)]),headers:{"content-type":"application/json",...(token?{authorization:`Bearer ${token}`}:{})},...(body===undefined?{}:{body:JSON.stringify(body)})});
  if(response.status===204){await response.body?.cancel();return undefined;}
  if(!response.ok){const text=await response.text().catch(()=>"");throw Error(`Scheduler answered ${response.status}: ${text.trim()}`);}
  return await response.json() as T;
 };
}

/** 👷️ Runs a worker agent until its signal aborts: registers with the scheduler, long-polls for work while it has free slots, executes each action in its own private root and sandbox, heartbeats, honours cancellations, reports results and deregisters on the way out. */
export async function runRemoteExecutionWorkerV1(options:RemoteExecutionWorkerOptionsV1):Promise<void>{
 const call=schedulerClient(options.scheduler,options.token),say=(stage:RemoteExecutionWorkerEventV1["stage"],detail:string)=>options.onEvent?.({stage,detail});
 const sandbox=selectRemoteExecutionSandboxV1(options.sandbox),name=options.name??`${hostname()}-${process.pid}`,properties=options.properties??remoteExecutionHostPropertiesV1(),platform=options.platform??`${osPlatform()}-${process.arch}`;
 say("sandbox",sandbox.name);
 let registered:RemoteExecutionWorkerRegisteredV1|undefined,content=options.content;
 const running=new Map<string,AbortController>();
 const register=async():Promise<void>=>{
  for(let delay=250;!options.signal.aborted;delay=Math.min(delay*2,5000)){
   try{
    const answer=await call<RemoteExecutionWorkerRegisteredV1>("POST","/v1/workers",{version:1,name,host:hostname(),platform,properties,concurrency:options.concurrency,sandbox:sandbox.name,runtime:`${runtimeIdentityV1().id} ${runtimeIdentityV1().version}`},options.signal);
    requireRemoteExecutionShapeV1("WorkerRegistered",answer,"Closed registration answer required");
    registered=answer!;
    if(!options.content){
     const scheduler=new URL(options.scheduler),url=answer!.cache.relative?`${scheduler.protocol}//${scheduler.hostname.includes(":")?`[${scheduler.hostname}]`:scheduler.hostname}:${new URL(answer!.cache.url).port}`:answer!.cache.url;
     content=httpRemoteExecutionContentV1({endpoint:url,token:options.cacheToken});
    }
    say("registered",`${answer!.id} cache ${answer!.cache.url}`);return;
   }catch(error){
    if(options.signal.aborted)return;
    say("reconnecting",(error as Error).message);await runtimeSleepV1(delay);
   }
  }
 };
 await register();
 if(!registered||!content)return;
 const store=new RemoteExecutionBlobCacheV1(join(options.directory,"blobs"),content,options.maximumBlobBytes),workDirectory=join(options.directory,"work",name.replace(/[^A-Za-z0-9._-]/gu,"_"));
 await rm(workDirectory,{recursive:true,force:true}).catch(()=>{});
 await store.prune().catch(()=>0);let pruned=Date.now();
 const cancel=(ids:readonly string[]):void=>{for(const id of ids)running.get(id)?.abort();};
 const heartbeat=setInterval(()=>{
  const id=registered!.id;
  void call<RemoteExecutionHeartbeatAnswerV1>("POST",`/v1/workers/${id}/heartbeat`,{version:1,running:[...running.keys()]},options.signal,10000).then(answer=>{if(answer){if(!answer.known)void register();cancel(answer.cancel);}}).catch(()=>{});
 },registered.heartbeatSeconds*1000);
 heartbeat.unref();
 let wake:(()=>void)|undefined;
 const finishTask=async(task:RemoteExecutionTaskV1,controller:AbortController):Promise<void>=>{
  say("started",`${task.id} ${task.action.hash.slice(0,12)}`);
  const result=await executeRemoteExecutionActionV1({instance:task.instance,action:task.action,timeoutSeconds:task.timeoutSeconds,content:content!,directory:options.directory,work:workDirectory,workerName:name,sandbox,materialize:options.materialize,inlineOutputBytes:options.inlineOutputBytes,signal:controller.signal,...(options.keepWork?{keepWork:true}:{}),blobs:store});
  const report:RemoteExecutionTaskResultV1={version:1,outcome:result.outcome,status:result.status,actionResult:result.actionResult?Buffer.from(result.actionResult).toString("base64"):null};
  for(let attempt=0;attempt<5;attempt++){
   try{await call("POST",`/v1/workers/${registered!.id}/tasks/${task.id}/result`,report,AbortSignal.timeout(30000));break;}
   catch(error){if(attempt===4)say("reconnecting",`report lost: ${(error as Error).message}`);else await runtimeSleepV1(200*2**attempt);}
  }
  say(result.outcome==="cancelled"?"cancelled":"finished",`${task.id} ${result.outcome} ${result.status.code}`);
  running.delete(task.id);wake?.();
  if(Date.now()-pruned>30000){pruned=Date.now();void store.prune().catch(()=>{});}
 };
 try{
  while(!options.signal.aborted){
   if(running.size>=options.concurrency){await new Promise<void>(resolve=>{wake=resolve;options.signal.addEventListener("abort",()=>{resolve();},{once:true});});continue;}
   let answer:RemoteExecutionPollAnswerV1|undefined;
   try{answer=await call<RemoteExecutionPollAnswerV1>("POST",`/v1/workers/${registered.id}/poll?wait=${registered.pollSeconds*1000}`,{version:1},options.signal,registered.pollSeconds*1000+15000);}
   catch(error){if(options.signal.aborted)break;say("reconnecting",(error as Error).message);await runtimeSleepV1(1000);continue;}
   if(!answer)continue;
   if(!answer.known){await register();continue;}
   cancel(answer.cancel);
   if(answer.task){const controller=new AbortController();running.set(answer.task.id,controller);void finishTask(answer.task,controller);}
  }
 }finally{
  clearInterval(heartbeat);
  for(const controller of running.values())controller.abort();
  const settle=Date.now()+10000;while(running.size>0&&Date.now()<settle)await runtimeSleepV1(50);
  await call("DELETE",`/v1/workers/${registered.id}`,undefined,AbortSignal.timeout(5000)).catch(()=>{});
  say("stopped",name);
 }
}
