import {createHash,randomBytes} from "node:crypto";
import {spawn} from "node:child_process";
import {mkdir,open,readdir,readFile,realpath,rename,rm,stat,truncate,writeFile} from "node:fs/promises";
import {dirname,join,relative,resolve,sep} from "node:path";
import {pathToFileURL} from "node:url";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ The report of the self-check: whether every module of the replica loaded and every embedded schema and text catalogue is well formed, how many of each were checked, the digest of the checked code (filled in by the command) and the first error. */
export type SelfCheckV1=Readonly<{version:1;ok:boolean;digest:string|null;modules:number;schemas:number;texts:number;error:string|null}>;

/** 🔖️ One sealed copy of the code a daemon proved to start: `files` are the repository-relative paths with their hashes. */
export type SealedCodeV1=Readonly<{version:1;digest:string;createdMs:number;moduleRoot:string;files:readonly Readonly<{path:string;bytes:number;sha256:string}>[]}>;

/** 📚️ One sealed snapshot on disk, newest first in listings. */
export type CodeSnapshotV1=Readonly<{digest:string;createdMs:number;directory:string}>;

/** 🩹️ What the daemon shows about itself without asking the store: the store state, the refused new code and the recurring failures per kind. */
export type HealthV1=Readonly<{version:1;wallMs:number;pid:number;code:string;store:"ok"|"stalled"|"unreachable";storeSinceMs:number;pending:Readonly<{code:string;ok:false;reason:string}>|null;failures:readonly FailureSummaryV1[]}>;

/** 🔁️ A recurring failure: its kind (the first part of its key), how often it happened and when it happened last. */
export type FailureSummaryV1=Readonly<{kind:string;count:number;lastMs:number}>;

/** 🧪️ A pure verdict on a deadline-bounded store state. */
export type StoreStateV1="ok"|"stalled"|"unreachable";

const validators=new Map<string,CompiledJsonSchemaSubset>(),sourceNames=new Set(["🟦️.ts","📜️script.ts"]),testsDirectory="🧪️tests",tokenOf=():string=>randomBytes(8).toString("hex");
const specifier=/\bfrom\s*["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']|\bimport\s*["']([^"']+)["']/gu;
const bytewise=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const posix=(path:string):string=>path.split(sep).join("/");
const gone=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");

/** 🛡️ Validates a value against one definition of the resilience schema. */
export function requireResilienceShapeV1(definition:"SealedCode"|"SelfCheck"|"Health"|"Failure"|"Fixture",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

async function walk(directory:string,prefix:string,into:string[],keep:(name:string,isDirectory:boolean)=>boolean):Promise<void>{
 for(const entry of await readdir(directory,{withFileTypes:true})){
  if(entry.name===testsDirectory||entry.name==="node_modules")continue;
  const path=prefix===""?entry.name:`${prefix}/${entry.name}`;
  if(entry.isDirectory())await walk(join(directory,entry.name),path,into,keep);
  else if(entry.isFile()&&keep(entry.name,false))into.push(path);
 }
}

/** 🧬️ The source files of the module that make up its code: every `🟦️.ts` and `📜️script.ts` below `root`, tests excluded, as sorted `/` paths relative to `root`. */
export async function moduleSourcesV1(root:string):Promise<string[]>{
 const found:string[]=[];
 await walk(root,"",found,name=>sourceNames.has(name));
 return found.sort(bytewise);
}

/** 🕸️ The files the module sources import beyond themselves, directly or through each other, as sorted `/` paths relative to `root` (outside the module they start with `..`): TypeScript sources, JSON schemas and catalogues. Only relative specifiers are followed, because the module has no other kind of runtime import. */
export async function importClosureV1(root:string,sources:readonly string[]):Promise<string[]>{
 const own=new Set(sources),seen=new Set<string>(),extra:string[]=[],queue=sources.map(path=>resolve(root,...path.split("/")));
 for(let index=0;index<queue.length;index++){
  const file=queue[index]!;
  if(seen.has(file))continue;
  seen.add(file);
  const path=posix(relative(root,file));
  if(!own.has(path))extra.push(path);
  if(!file.endsWith(".ts")&&!file.endsWith(".js")&&!file.endsWith(".mjs"))continue;
  let text:string;
  try{text=await readFile(file,"utf8");}catch(error){if(gone(error)){if(!own.has(path))extra.pop();continue;}throw error;}
  for(const match of text.matchAll(specifier)){
   const name=match[1]??match[2]??match[3]!;
   if(!name.startsWith("."))continue;
   const target=resolve(dirname(file),name);
   if(seen.has(target))continue;
   const info=await stat(target).catch(()=>undefined);
   if(info?.isFile())queue.push(target);
  }
 }
 return extra.sort(bytewise);
}

/** 🕳️ Returns the message of the first problem of a text catalogue (`{languages,messages}` with every message in every language and the same `{name}` placeholders everywhere), or `null`. */
export function textsProblemV1(catalogue:unknown):string|null{
 if(typeof catalogue!=="object"||catalogue===null)return "catalogue must be an object";
 const {languages,messages}=catalogue as {languages?:unknown;messages?:unknown};
 if(!Array.isArray(languages)||languages.length===0||languages.some(item=>typeof item!=="string"||item===""))return "languages must be a non-empty list of names";
 if(new Set(languages).size!==languages.length)return "languages must be unique";
 if(typeof messages!=="object"||messages===null||Array.isArray(messages))return "messages must be an object";
 for(const [key,message] of Object.entries(messages)){
  if(typeof message!=="object"||message===null)return `message ${key} must be an object`;
  const keys=Object.keys(message);
  if(keys.length!==languages.length||languages.some(language=>!keys.includes(language as string)))return `message ${key} must have exactly the languages ${languages.join(",")}`;
  let reference:string|undefined;
  for(const language of languages as string[]){
   const text=(message as Record<string,unknown>)[language];
   if(typeof text!=="string"||text==="")return `message ${key} has no text in ${language}`;
   const names=[...new Set([...text.matchAll(/\{([a-z0-9]+)\}/gu)].map(match=>match[1]!))].sort().join(",");
   if(reference===undefined)reference=names;else if(reference!==names)return `message ${key} differs in its placeholders between the languages`;
  }
 }
 return null;
}

/** 🔬️ Loads every module of the replica, compiles every definition of every embedded schema and validates every text catalogue below `root`; it opens no store and no worktree state. Never throws: the first problem comes back in `error`. */
export async function selfCheckV1(root:string,signal?:AbortSignal):Promise<SelfCheckV1>{
 let modules=0,schemas=0,texts=0;
 try{
  const main=process.argv[1]===undefined?undefined:await realpath(process.argv[1]).catch(()=>undefined);
  for(const path of await moduleSourcesV1(root)){
   signal?.throwIfAborted();
   if(main!==undefined&&await realpath(join(root,...path.split("/"))).catch(()=>undefined)===main){modules++;continue;}
   try{await import(pathToFileURL(join(root,...path.split("/"))).href);}catch(error){throw Error(`module ${path} does not load: ${error instanceof Error?error.message:String(error)}`);}
   modules++;
  }
  const documents:string[]=[];
  await walk(root,"",documents,name=>name==="🔣️.json");
  for(const path of documents.sort(bytewise)){
   signal?.throwIfAborted();
   const parts=path.split("/"),kind=parts[parts.length-2];
   if(kind!=="🧬️schema"&&kind!=="📖️texts")continue;
   let document:unknown;
   try{document=JSON.parse(await readFile(join(root,...parts),"utf8"));}catch(error){throw Error(`${path} is not valid JSON: ${(error as Error).message}`);}
   if(kind==="📖️texts"){
    const problem=textsProblemV1(document);
    if(problem!==null)throw Error(`${path}: ${problem}`);
    texts++;
   }else{
    const definitions=(document as {definitions?:Record<string,unknown>}).definitions;
    if(typeof definitions!=="object"||definitions===null)throw Error(`${path}: schema without definitions`);
    for(const name of Object.keys(definitions)){try{compileJsonSchemaSubset({$ref:`#/definitions/${name}`},document as never);}catch(error){throw Error(`${path}: definition ${name} does not compile: ${(error as Error).message}`);}}
    schemas++;
   }
  }
  return {version:1,ok:true,digest:null,modules,schemas,texts,error:null};
 }catch(error){return {version:1,ok:false,digest:null,modules,schemas,texts,error:(error instanceof Error?error.message:String(error)).slice(0,2048)};}
}

/** 🚀️ The outcome of a self-check child process. */
export type SelfCheckRunV1=Readonly<{ok:boolean;reason:string;digest:string|null}>;

/** 🧫️ Runs the self-check in a fresh process: `command` with `args` (the same runtime on the replica script with `self-check`) in `cwd`, killed after `timeoutMs` or when `signal` aborts; success is exit code 0 with a report line that says `ok`. Nothing else is shared with the daemon. */
export function runSelfCheckV1(options:Readonly<{command:string;args:readonly string[];cwd:string;timeoutMs:number;signal?:AbortSignal;environment?:Readonly<Record<string,string|undefined>>}>):Promise<SelfCheckRunV1>{
 return new Promise(done=>{
  let finished=false,out="",err="";
  let child:ReturnType<typeof spawn>;
  const finish=(result:SelfCheckRunV1):void=>{if(finished)return;finished=true;clearTimeout(timer);options.signal?.removeEventListener("abort",abort);done(result);};
  const kill=():void=>{try{child?.kill("SIGKILL");}catch{}};
  const timer=setTimeout(()=>{kill();finish({ok:false,reason:`self-check timed out after ${options.timeoutMs} ms`,digest:null});},options.timeoutMs);
  const abort=():void=>{kill();finish({ok:false,reason:"self-check cancelled",digest:null});};
  if(options.signal?.aborted===true){abort();return;}
  options.signal?.addEventListener("abort",abort,{once:true});
  try{child=spawn(options.command,[...options.args],{cwd:options.cwd,env:{...process.env,...options.environment} as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"],windowsHide:true});}
  catch(error){finish({ok:false,reason:`self-check could not start: ${(error as Error).message}`,digest:null});return;}
  child.stdout?.on("data",chunk=>{if(out.length<65536)out+=String(chunk);});
  child.stderr?.on("data",chunk=>{if(err.length<65536)err+=String(chunk);});
  child.on("error",error=>{finish({ok:false,reason:`self-check could not start: ${error.message}`,digest:null});});
  child.on("close",code=>{
   let report:SelfCheckV1|undefined;
   for(const line of out.split("\n").reverse()){
    if(!line.startsWith("{"))continue;
    try{const value=JSON.parse(line) as unknown;requireResilienceShapeV1("SelfCheck",value,"Closed self-check report required");report=value as SelfCheckV1;break;}catch{}
   }
   if(code===0&&report?.ok===true){finish({ok:true,reason:"",digest:report.digest});return;}
   const detail=report?.error??err.trim().split("\n").filter(line=>line!=="").pop()??"";
   finish({ok:false,reason:(`self-check ${code===null?"was killed":`exited with ${code}`}${detail===""?"":`: ${detail}`}`).slice(0,512),digest:report?.digest??null});
  });
 });
}

/** 🎲️ The deterministic restart delay of one origin for one code digest in `[0, span)` milliseconds: the first four bytes of `sha256(originId \0 digest)` as an unsigned big-endian integer modulo `span`, so that a whole fleet never restarts in the same seconds and a machine always waits the same time for the same change. */
export function restartJitterMillisecondsV1(originId:string,digest:string,span:number=60_000):number{
 if(span<=0)return 0;
 return createHash("sha256").update(`${originId}\0${digest}`).digest().readUInt32BE(0)%span;
}

/** 📁️ The directory below the worktree that holds the sealed code snapshots. */
export function codeDirectoryV1(root:string):string{return join(root,".cache","f","code");}

/** 📚️ Lists the sealed snapshots of a worktree, newest first; a directory without a readable seal is not listed. */
export async function listCodeSnapshotsV1(root:string):Promise<CodeSnapshotV1[]>{
 const base=codeDirectoryV1(root),found:CodeSnapshotV1[]=[];
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!/^[0-9a-f]{64}$/u.test(name))continue;
  try{
   const seal=JSON.parse(await readFile(join(base,name,"sealed.json"),"utf8")) as unknown;
   requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
   if((seal as SealedCodeV1).digest===name)found.push({digest:name,createdMs:(seal as SealedCodeV1).createdMs,directory:join(base,name)});
  }catch{}
 }
 return found.sort((left,right)=>right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
}

/** ✂️ Keeps the newest `keep` sealed snapshots and removes the others and every unfinished `.tmp-` directory older than an hour; returns the removed digests. */
export async function pruneCodeSnapshotsV1(root:string,keep:number,now:number=Date.now()):Promise<string[]>{
 const removed:string[]=[],base=codeDirectoryV1(root);
 for(const snapshot of (await listCodeSnapshotsV1(root)).slice(Math.max(0,keep))){await rm(snapshot.directory,{recursive:true,force:true});removed.push(snapshot.digest);}
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!name.startsWith(".tmp-"))continue;
  const info=await stat(join(base,name)).catch(()=>undefined);
  if(info!==undefined&&now-info.mtimeMs>3_600_000)await rm(join(base,name),{recursive:true,force:true});
 }
 return removed;
}

/** 🔒️ The inputs of a seal: the worktree root, the module root inside it, the digest the checked code has, how to compute that digest of a copy, the clock, how many snapshots to keep (2) and cancellation. */
export type SealOptionsV1=Readonly<{root:string;moduleRoot:string;digest:string;digestOf:(moduleRoot:string)=>Promise<string>;now?:()=>number;keep?:number;signal?:AbortSignal}>;

/** 🔒️ Seals a copy of the code that was just checked into `.cache/f/code/<digest>/`: the module sources and every file they import, mirrored below `files/` at their repository-relative paths, then `sealed.json` with the hashes, written last into a private `.tmp-` directory that is renamed into place, so a snapshot either is complete or does not exist. The digest of the copy must equal `digest` (else the code changed while it was copied and nothing is sealed). An existing seal of the same digest is only refreshed to be the newest. Afterwards only the newest `keep` snapshots stay. */
export async function sealCodeV1(options:SealOptionsV1):Promise<Readonly<{created:boolean;directory:string|null;reason:string|null}>>{
 const {root,moduleRoot,digest,signal}=options,now=options.now??Date.now,base=codeDirectoryV1(root),target=join(base,digest),moduleRelative=posix(relative(root,moduleRoot));
 if(moduleRelative.startsWith("..")||moduleRelative==="")return {created:false,directory:null,reason:"module-outside-root"};
 const existing=await readFile(join(target,"sealed.json"),"utf8").catch(()=>undefined);
 if(existing!==undefined){
  try{
   const seal=JSON.parse(existing) as SealedCodeV1;
   requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
   const temporary=join(target,`sealed.${tokenOf()}`);
   await writeFile(temporary,JSON.stringify({...seal,createdMs:now()}));await rename(temporary,join(target,"sealed.json"));
   await pruneCodeSnapshotsV1(root,options.keep??2,now());
   return {created:false,directory:target,reason:null};
  }catch{await rm(target,{recursive:true,force:true});}
 }
 const sources=await moduleSourcesV1(moduleRoot),all=[...sources.map(path=>resolve(moduleRoot,...path.split("/"))),...(await importClosureV1(moduleRoot,sources)).map(path=>resolve(moduleRoot,...path.split("/")))];
 const temporary=join(base,`.tmp-${tokenOf()}`),copied=join(temporary,"files"),files:{path:string;bytes:number;sha256:string}[]=[];
 try{
  for(const file of all){
   signal?.throwIfAborted();
   const path=posix(relative(root,file));
   if(path.startsWith(".."))throw Error(`outside-root:${path}`);
   const destination=join(copied,...path.split("/")),data=await readFile(file);
   await mkdir(dirname(destination),{recursive:true});
   await writeFile(destination,data);
   files.push({path,bytes:data.length,sha256:sha(data)});
  }
  if(await options.digestOf(join(copied,...moduleRelative.split("/")))!==digest){await rm(temporary,{recursive:true,force:true});return {created:false,directory:null,reason:"code-changed-while-copying"};}
  const seal:SealedCodeV1={version:1,digest,createdMs:now(),moduleRoot:moduleRelative,files:files.sort((left,right)=>bytewise(left.path,right.path))};
  requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
  const handle=await open(join(temporary,"sealed.json"),"w");
  try{await handle.writeFile(JSON.stringify(seal));await handle.sync();}finally{await handle.close();}
  try{await rename(temporary,target);}catch(error){if(!(await stat(join(target,"sealed.json")).then(()=>true,()=>false)))throw error;await rm(temporary,{recursive:true,force:true});}
 }catch(error){
  await rm(temporary,{recursive:true,force:true});
  if(signal?.aborted===true)throw error;
  return {created:false,directory:null,reason:(error instanceof Error?error.message:String(error)).slice(0,256)};
 }
 await pruneCodeSnapshotsV1(root,options.keep??2,now());
 return {created:true,directory:target,reason:null};
}

/** 🔎️ Re-hashes every file of a sealed snapshot against its seal; the first mismatch or missing file comes back as the reason, `null` when the snapshot is intact. */
export async function verifyCodeSnapshotV1(directory:string):Promise<string|null>{
 try{
  const seal=JSON.parse(await readFile(join(directory,"sealed.json"),"utf8")) as SealedCodeV1;
  requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
  for(const file of seal.files){
   const data=await readFile(join(directory,"files",...file.path.split("/"))).catch(()=>undefined);
   if(data===undefined)return `missing ${file.path}`;
   if(data.length!==file.bytes||sha(data)!==file.sha256)return `changed ${file.path}`;
  }
  return null;
 }catch(error){return error instanceof Error?error.message:String(error);}
}

/** ⏱️ The inputs of the store guard: the reachability probe (a `stat` of the store root), the deadline after which an unanswered store call counts as stalled, the clock and the state-change callback. */
export type StoreGuardOptionsV1=Readonly<{probe:()=>Promise<boolean>;deadlineMs:number;now:()=>number;changed:(state:StoreStateV1)=>void}>;

/** 🚧️ Raised instead of issuing a store call while an earlier store call is still stuck or has not returned within its deadline. */
export class StoreStalledV1 extends Error{constructor(){super("Store call refused: an earlier store call did not return within its deadline");this.name="StoreStalledV1";}}

/** 🛑️ A turn of store work: it finished (`done` with its value), it was abandoned because the store stopped answering (`stalled`), or it was not started because an earlier turn is still outstanding (`busy`). */
export type StoreTurnV1<T>=Readonly<{status:"done";value:T}>|Readonly<{status:"stalled"|"busy"}>;

/** 🚧️ A deadline guard around a store that may hang: a blocked filesystem call cannot be cancelled in Node or Bun (it holds a worker thread until the kernel returns), so the guard never waits for one. `reachable` probes the store with at most one outstanding probe and answers `false` once a probe is older than the deadline; `turn` runs a body of store work with an abort signal and abandons it (aborting the signal, so no new call starts once the blocked one returns) as soon as a probe has been outstanding for the deadline; `call` runs one store call under the deadline. While the state is not `ok` no turn and no call starts, so at most the already blocked calls plus one probe are outstanding. When a probe answers again and the abandoned work has returned the state is `ok` again without a restart. */
export type StoreGuardV1={state():StoreStateV1;since():number;reachable():Promise<boolean>;turn<T>(work:(signal:AbortSignal)=>Promise<T>,outer?:AbortSignal):Promise<StoreTurnV1<T>>;call<T>(work:()=>Promise<T>):Promise<T>;outstanding():number};

/** 🏭️ Creates a store guard. */
export function createStoreGuardV1(options:StoreGuardOptionsV1):StoreGuardV1{
 let state:StoreStateV1="ok",since=options.now(),probing:Readonly<{promise:Promise<void>;startedAt:number}>|undefined,turning=false,calling=false,abandoned=0,abandonedAt=0;
 const set=(next:StoreStateV1):void=>{if(next===state)return;state=next;since=options.now();options.changed(next);};
 const wait=(milliseconds:number):Readonly<{promise:Promise<"late">;cancel:()=>void}>=>{let timer:ReturnType<typeof setTimeout>|undefined;return {promise:new Promise<"late">(done=>{timer=setTimeout(()=>{done("late");},Math.max(0,milliseconds));timer.unref?.();}),cancel:()=>{clearTimeout(timer);}};};
 const mayRecover=():boolean=>abandoned===0||options.now()-abandonedAt>5*options.deadlineMs;
 const startProbe=():void=>{
  if(probing!==undefined)return;
  const promise:Promise<void>=Promise.resolve().then(options.probe).then(ok=>{probing=undefined;if(ok){if(state!=="ok"&&mayRecover())set("ok");}else set("unreachable");},()=>{probing=undefined;set("unreachable");});
  probing={promise,startedAt:options.now()};
 };
 const abandon=():void=>{abandoned++;abandonedAt=options.now();set("stalled");};
 return {
  state:()=>state,
  since:()=>since,
  outstanding:()=>(turning?1:0)+(calling?1:0)+(probing===undefined?0:1),
  async reachable(){
   startProbe();
   const current=probing;
   if(current===undefined)return state==="ok";
   const timer=wait(options.deadlineMs-(options.now()-current.startedAt));
   const winner=await Promise.race([current.promise.then(()=>"done" as const),timer.promise]);
   timer.cancel();
   if(winner==="late"&&probing===current&&state==="ok")set("stalled");
   return state==="ok";
  },
  async turn<T>(work:(signal:AbortSignal)=>Promise<T>,outer?:AbortSignal):Promise<StoreTurnV1<T>>{
   if(state!=="ok")return {status:"stalled"};
   if(turning)return {status:"busy"};
   turning=true;
   const controller=new AbortController(),relay=():void=>{controller.abort(outer?.reason);};
   if(outer?.aborted===true)relay();else outer?.addEventListener("abort",relay,{once:true});
   const running=Promise.resolve().then(()=>work(controller.signal));
   let stopped=false,handedOff=false,timer:ReturnType<typeof setTimeout>|undefined;
   const release=():void=>{turning=false;outer?.removeEventListener("abort",relay);};
   const finish=():void=>{abandoned--;release();startProbe();};
   try{
    return await new Promise<StoreTurnV1<T>>((resolveTurn,rejectTurn)=>{
     const watch=():void=>{
      if(stopped)return;
      startProbe();
      if(probing!==undefined&&options.now()-probing.startedAt>=options.deadlineMs){
       stopped=true;handedOff=true;abandon();controller.abort(new StoreStalledV1());
       running.then(finish,finish);
       resolveTurn({status:"stalled"});return;
      }
      timer=setTimeout(watch,Math.max(20,Math.min(1000,options.deadlineMs/4)));timer.unref?.();
     };
     timer=setTimeout(watch,Math.max(20,Math.min(1000,options.deadlineMs/4)));timer.unref?.();
     running.then(value=>{if(stopped)return;stopped=true;clearTimeout(timer);resolveTurn({status:"done",value});},error=>{if(stopped)return;stopped=true;clearTimeout(timer);rejectTurn(error);});
    });
   }finally{if(!handedOff)release();}
  },
  async call<T>(work:()=>Promise<T>):Promise<T>{
   if(state!=="ok"||calling)throw new StoreStalledV1();
   calling=true;
   const running=Promise.resolve().then(work).then(value=>({value}),error=>({error})),timer=wait(options.deadlineMs),first=await Promise.race([running,timer.promise]);
   if(first==="late"){abandon();running.then(()=>{abandoned--;calling=false;startProbe();});throw new StoreStalledV1();}
   timer.cancel();calling=false;
   if("error" in first)throw first.error;
   return first.value;
  }
 };
}

/** 🔁️ The recurrence tracker input: the clock, the window after which a repeated failure is announced again (60 s) and how many keys are remembered (256). */
export type FailuresOptionsV1=Readonly<{now:()=>number;windowMs?:number;capacity?:number}>;

/** 🔁️ Counts failures per key so that a recurring one never goes silent: the first occurrence is announced, a repeat only once per window (with its total count); `clear` forgets the key when the operation worked again, `summary` lists the failure kinds with their lifetime count and last time. */
export type FailuresV1={note(key:string,message:string):Readonly<{announce:boolean;count:number}>;clear(key:string):void;has(key:string):boolean;summary(limit?:number):readonly FailureSummaryV1[];size():number;forgetOlderThan(milliseconds:number):number};

/** 🏭️ Creates a recurrence tracker. */
export function createFailuresV1(options:FailuresOptionsV1):FailuresV1{
 const window=options.windowMs??60_000,capacity=options.capacity??256,entries=new Map<string,{count:number;lastMs:number;announcedMs:number}>(),kinds=new Map<string,{count:number;lastMs:number}>();
 const kindOf=(key:string):string=>{const head=key.split(":")[0]!.toLowerCase().replace(/[^a-z0-9-]/gu,"-").replace(/^[^a-z]+/u,"");return (head===""?"failure":head).slice(0,40);};
 return {
  note(key,_message){
   const now=options.now(),entry=entries.get(key)??{count:0,lastMs:now,announcedMs:-Infinity},kind=kinds.get(kindOf(key))??{count:0,lastMs:now};
   entry.count++;entry.lastMs=now;kind.count++;kind.lastMs=now;
   const announce=entry.count===1||now-entry.announcedMs>=window;
   if(announce)entry.announcedMs=now;
   entries.delete(key);entries.set(key,entry);kinds.set(kindOf(key),kind);
   if(entries.size>capacity)entries.delete(entries.keys().next().value as string);
   return {announce,count:entry.count};
  },
  clear:key=>{entries.delete(key);},
  has:key=>entries.has(key),
  size:()=>entries.size,
  summary:(limit=8)=>[...kinds].map(([kind,value])=>({kind,count:value.count,lastMs:value.lastMs})).sort((left,right)=>right.lastMs-left.lastMs||(left.kind<right.kind?-1:1)).slice(0,limit),
  forgetOlderThan(milliseconds){
   const limit=options.now()-milliseconds;
   let removed=0;
   for(const [key,entry] of entries)if(entry.lastMs<limit){entries.delete(key);removed++;}
   return removed;
  }
 };
}

/** 🧹️ Removes preserved overwritten files that are older than `retainMs` (the directory name starts with the 13-digit millisecond of the hlc that overwrote them) and then the oldest ones beyond `maxEntries` directories; returns how many directories were removed. */
export async function pruneLostV1(directory:string,options:Readonly<{now:number;retainMs:number;maxEntries:number}>):Promise<number>{
 const names=(await readdir(directory).catch(()=>[] as string[])).filter(name=>/^[0-9]{13}-/u.test(name)).sort();
 let removed=0,left=names.length;
 for(const name of names){
  if(options.now-Number(name.slice(0,13))>options.retainMs||left>options.maxEntries){await rm(join(directory,name),{recursive:true,force:true});removed++;left--;}
 }
 return removed;
}

/** 📜️ Rotates a log file that this process does not own exclusively (a service manager keeps it open for appending): when it exceeds `maxBytes` its last `keepBytes` bytes are kept as `<file>.1` and the file is truncated in place, which appending writers survive; returns whether it rotated. */
export async function rotateLogV1(file:string,options:Readonly<{maxBytes:number;keepBytes:number}>):Promise<boolean>{
 const info=await stat(file).catch(()=>undefined);
 if(info===undefined||!info.isFile()||info.size<=options.maxBytes)return false;
 const handle=await open(file,"r");
 let tail:Buffer;
 try{
  const length=Math.min(options.keepBytes,info.size);
  tail=Buffer.alloc(length);
  let done=0;
  while(done<length){const {bytesRead}=await handle.read(tail,done,length-done,info.size-length+done);if(bytesRead===0)break;done+=bytesRead;}
  tail=tail.subarray(0,done);
 }finally{await handle.close();}
 const temporary=`${file}.${tokenOf()}.tmp`;
 await writeFile(temporary,tail);await rename(temporary,`${file}.1`);await truncate(file,0);
 return true;
}

/** 📍️ The path of the health file the daemon keeps for `status`. */
export function healthPathV1(root:string):string{return join(root,".cache","f","health.json");}

/** ✍️ Replaces the health file through tmp + rename; it is advisory, so there is no fsync. */
export async function writeHealthV1(root:string,health:HealthV1):Promise<void>{
 requireResilienceShapeV1("Health",health,"Closed health required");
 const file=healthPathV1(root),temporary=`${file}.${tokenOf()}.tmp`;
 await mkdir(dirname(file),{recursive:true});
 try{await writeFile(temporary,JSON.stringify(health));await rename(temporary,file);}catch(error){await rm(temporary,{force:true});throw error;}
}

/** 👀️ Reads the health file; `undefined` when it is missing, torn or invalid. */
export async function readHealthV1(root:string):Promise<HealthV1|undefined>{
 try{
  const value=JSON.parse(await readFile(healthPathV1(root),"utf8")) as unknown;
  requireResilienceShapeV1("Health",value,"Closed health required");
  return value as HealthV1;
 }catch{return undefined;}
}
