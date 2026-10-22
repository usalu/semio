import {availableParallelism} from "node:os";
import {lstat,readdir} from "node:fs/promises";
import {dirname,join,resolve} from "node:path";
import {gitV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🌿️git/🟦️.ts";
import {ToolErrorV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {semioActionKeyV1,semioCanonicalV1,semioHashV1,semioSelectFilesV1,type SemioEntryV1,type SemioEventV1,type SemioRecordTaskV1,type SemioRecordV1,type SemioStatusV1} from "../🟦️.ts";
import {SemioStoreV1,semioFileDigestV1} from "../🗄️store/🟦️.ts";
import {packSemioEntryV1,unpackSemioEntryV1,type SemioRemoteV1} from "../🌐️remote/🟦️.ts";
import type {SemioExecutorV1} from "../🚀️exec/🟦️.ts";
import {readReadinessDeclarationV1,resolveReadinessV1,watchReadinessV1,type ReadinessWatchV1} from "../../../🟢️readiness/🟦️.ts";
import {buildCacheDecisionV1,buildTaskInputsV1,planBuildTasksV1,type BuildTaskRequestV1,type BuildTaskV1} from "../../../🧮️task-model/🟦️.ts";
import type {BuildGraphV1} from "../../../🕸️graph/🟦️.ts";

/** 🔌️ What a run needs from the outside: the workspace, the stores, how to list files, how to run a task body, the environment, a clock and a log. */
export type SemioPortsV1=Readonly<{workspace:string;store:SemioStoreV1;remote?:SemioRemoteV1;inventory():Promise<readonly string[]>;execute:SemioExecutorV1;environment:Readonly<Record<string,string|undefined>>;now():number;write(chunk:string):void;runtime?(command:string,signal:AbortSignal):Promise<string>}>;
/** 🎛️ Cancellation and progress of one run. */
export type SemioHooksV1=Readonly<{signal:AbortSignal;onEvent?:(event:SemioEventV1)=>void|Promise<void>;onSettled?:()=>void|Promise<void>}>;

const EXCLUDED=new Set([".git",".🧬semio",".cache",".nx","node_modules","target","dist","build","coverage",".venv","__pycache__"]);
const order=(left:string,right:string)=>left<right?-1:left>right?1:0;

/** 🌳️ Whether the workspace lies inside a Git work tree, judged from the `.git` entry of it or of a parent. */
async function insideGitV1(workspace:string):Promise<boolean>{
 for(let directory=resolve(workspace);;directory=dirname(directory)){
  try{await lstat(join(directory,".git"));return true;}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code!=="ENOENT"&&code!=="ENOTDIR")throw error;}
  if(dirname(directory)===directory)return false;
 }
}

/** 📚️ Lists the workspace's source files: Git's tracked and untracked-but-not-ignored files; a walk that skips the declared excluded directories only where Git cannot be asked (the workspace is no Git work tree, or the Git tool is not installed). A Git failure inside a work tree (timeout, damaged index) fails with its reason instead of changing the file set and so the cache keys. */
export async function semioInventoryV1(workspace:string):Promise<string[]>{
 if(await insideGitV1(workspace)){
  let result:Awaited<ReturnType<typeof gitV1>>|undefined;
  try{result=await gitV1(["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z"],{cwd:workspace});}
  catch(error){if(!(error instanceof ToolErrorV1)||error.code!=="tool-missing")throw error;}
  if(result){
   if(result.status!==0)throw Error(`Git could not list the files of ${workspace} (status ${result.signal??result.status}): ${result.stderr.trim()}\nGit konnte die Dateien von ${workspace} nicht auflisten (Status ${result.signal??result.status}): ${result.stderr.trim()}`);
   return [...new Set(result.stdout.split("\0").filter(Boolean))].filter(path=>!path.split("/").some(part=>EXCLUDED.has(part))).sort(order);
  }
 }
 const files:string[]=[],pending=[""];
 while(pending.length){
  const directory=pending.pop()!;
  for(const entry of await readdir(join(workspace,directory),{withFileTypes:true})){
   const path=directory?directory+"/"+entry.name:entry.name;
   if(entry.isDirectory()){if(!EXCLUDED.has(entry.name))pending.push(path);}else if(entry.isFile()||entry.isSymbolicLink())files.push(path);
  }
 }
 return files.sort(order);
}

class Slots{
 private free:number;private readonly queue:(()=>void)[]=[];
 constructor(count:number){this.free=count;}
 acquire(signal:AbortSignal):Promise<void>{
  if(signal.aborted)return Promise.reject(signal.reason??Error("Cancelled"));
  if(this.free>0){this.free--;return Promise.resolve();}
  return new Promise((accept,refuse)=>{const grant=()=>{signal.removeEventListener("abort",abort);accept();},abort=()=>{const at=this.queue.indexOf(grant);if(at>=0)this.queue.splice(at,1);refuse(signal.reason??Error("Cancelled"));};this.queue.push(grant);signal.addEventListener("abort",abort,{once:true});});
 }
 release():void{const next=this.queue.shift();if(next)next();else this.free++;}
}

type Outcome=Readonly<{status:SemioStatusV1;digest:string}>;
type Deferred=Readonly<{promise:Promise<Outcome>;accept(outcome:Outcome):void}>;

const READY:Outcome=Object.freeze({status:"executed",digest:"none"}),LOST:Outcome=Object.freeze({status:"failed",digest:"none"});

/** 🏗️ Runs a request: plans the closure, restores every task whose action key is cached locally or remotely, executes the others with at most `parallel` at a time, stores their outputs, skips what depends on a failure, and stops on cancellation. Continuous tasks (servers) start first, count as ready when they say so (printed marker, declared readiness probe, or their process starting), hold no slot, and run until the closure completes or the run is cancelled; tasks that depend on one wait for its readiness only. `onSettled` fires once every regular task finished without failure, which is where a watcher starts. The run record tells what each task did. */
export async function runSemioTasksV1(graph:BuildGraphV1,request:BuildTaskRequestV1,ports:SemioPortsV1,hooks:SemioHooksV1):Promise<SemioRecordV1>{
 const plan=planBuildTasksV1(graph,request),ids=Object.keys(plan.tasks).sort(order),total=ids.length;
 const own=new AbortController(),signal=AbortSignal.any([hooks.signal,own.signal]),slots=new Slots(Math.max(1,request.parallel??availableParallelism()));
 const records:SemioRecordTaskV1[]=[],results=new Map<string,Promise<Outcome>>(),digests=new Map<string,Promise<string>>(),services=new Map<string,Deferred>(),runtimes=new Map<string,Promise<string>>();
 let done=0,inventory:Promise<readonly string[]>|undefined,closing=false;
 const continuous=(id:string)=>plan.tasks[id]!.definition.continuous===true,required=(id:string)=>ids.some(other=>plan.tasks[other]!.dependencies.includes(id));
 const emit=async(event:Omit<SemioEventV1,"done"|"total">)=>{if(hooks.onEvent)await hooks.onEvent({...event,done,total});};
 const digest=(path:string)=>{let known=digests.get(path);if(!known){known=semioFileDigestV1(join(ports.workspace,path));digests.set(path,known);}return known;};
 const service=(id:string):Deferred=>{let known=services.get(id);if(!known){let accept!:(outcome:Outcome)=>void;const promise=new Promise<Outcome>(resolve=>{accept=resolve;});known={promise,accept};services.set(id,known);}return known;};
 const outputsOf=(task:BuildTaskV1)=>(task.definition.outputs??[]).map(pattern=>pattern.replaceAll("{projectRoot}",task.root==="."?"":task.root).replaceAll("{projectName}",task.project).replaceAll("{workspaceRoot}/","").replaceAll("{workspaceRoot}","").replace(/^\.\//u,"").replace(/^\/+/u,""));
 const finish=async(task:BuildTaskV1,started:number,status:SemioStatusV1,outputs:string,detail:Partial<SemioRecordTaskV1>={}):Promise<Outcome>=>{
  done++;records.push({id:task.id,status,startedMs:started,endedMs:ports.now(),...detail});
  await emit({kind:"end",task:task.id,status});
  return {status,digest:outputs};
 };
 const schedule=(id:string):Promise<Outcome>=>{
  let known=results.get(id);
  if(!known){known=(continuous(id)?serve(plan.tasks[id]!):work(plan.tasks[id]!)).catch(error=>{own.abort(error);throw error;});results.set(id,known);}
  return known;
 };
 const waitFor=(dependency:string)=>continuous(dependency)?(schedule(dependency),service(dependency).promise):schedule(dependency);
 const serve=async(task:BuildTaskV1):Promise<Outcome>=>{
  const ready=service(task.id);
  const dependencies=await Promise.all(task.dependencies.map(waitFor)),started=ports.now();
  if(signal.aborted){ready.accept(LOST);return finish(task,started,"cancelled","none");}
  if(dependencies.some(row=>row.status==="failed"||row.status==="skipped"||row.status==="cancelled")){ready.accept(LOST);return finish(task,started,"skipped","none",{reason:"dependency"});}
  await emit({kind:"start",task:task.id});
  const execution=task.definition.execution,markers=typeof execution.readyWhen==="string"?[execution.readyWhen]:Array.isArray(execution.readyWhen)?(execution.readyWhen as string[]):[],declaration=markers.length?undefined:readReadinessDeclarationV1(task.definition.metadata,task.id);
  const watch:ReadinessWatchV1|undefined=declaration?watchReadinessV1(resolveReadinessV1(declaration,{...ports.environment,...(execution.env??{})},task.id),signal):undefined;
  let observed=false,tail="";const seen=new Set<string>();
  const mark=()=>{if(!observed){observed=true;ready.accept(READY);ports.write(`[semio] task-ready ${task.id}\n`);}};
  if(watch)void watch.ready.then(()=>mark(),()=>undefined);
  const write=(chunk:string)=>{
   ports.write(chunk);
   if(watch){watch.observe(chunk);return;}
   if(markers.length){tail=(tail+chunk).slice(-4096);for(const marker of markers)if(tail.includes(marker))seen.add(marker);if(seen.size===markers.length)mark();}
  };
  let result;
  try{result=await ports.execute(task,signal,write,watch||markers.length?undefined:mark);}
  catch(error){watch?.stop();if(!observed)ready.accept(LOST);if(closing||signal.aborted)return finish(task,started,closing&&!hooks.signal.aborted?"executed":"cancelled","none",closing?{reason:"stopped"}:{});return finish(task,started,"failed","none",{reason:String((error as Error).message??error)});}
  watch?.stop();
  if(closing||signal.aborted){if(!observed)ready.accept(LOST);return finish(task,started,hooks.signal.aborted?"cancelled":"executed","none",{reason:"stopped"});}
  const reason=`Continuous task exited ${observed?"while required by selected tasks":"before readiness"}: ${task.id}`;
  if(!observed)ready.accept(LOST);
  if(!observed||required(task.id)||result.exitCode!==0){const row=await finish(task,started,"failed","none",{exitCode:result.exitCode,reason});own.abort(Error(reason));return row;}
  return finish(task,started,"executed","none");
 };
 const work=async(task:BuildTaskV1):Promise<Outcome>=>{
  const dependencies=await Promise.all(task.dependencies.map(waitFor)),started=ports.now();
  if(signal.aborted)return finish(task,started,"cancelled","none");
  if(dependencies.some(row=>row.status==="failed"||row.status==="skipped"||row.status==="cancelled"))return finish(task,started,"skipped","none",{reason:"dependency"});
  try{await slots.acquire(signal);}catch{return finish(task,started,"cancelled","none");}
  try{
   await emit({kind:"start",task:task.id});
   let decision=buildCacheDecisionV1(task,request.skipCache===true);
   const outputs=outputsOf(task),declared=task.definition.outputs,optional=(task.definition.metadata as {semio?:{optionalOutputs?:unknown}}|undefined)?.semio?.optionalOutputs===true;
   const refuseMissing=task.definition.cache===true&&!optional;
   let key:string|undefined;
   if(decision.cache){
    const inputs=buildTaskInputsV1(graph,task);
    if(inputs.some(input=>input.kind==="unsupported"||input.kind==="runtime"&&!ports.runtime))decision={cache:false,reason:"unsupported-input"};
    else{
     inventory??=ports.inventory();
     const files=semioSelectFilesV1(await inventory,inputs.flatMap(input=>input.kind==="file"&&!input.exclude?[input.pattern]:[]),[...inputs.flatMap(input=>input.kind==="file"&&input.exclude?[input.pattern]:[]),...outputs.flatMap(path=>[path,path+"/**"])]);
     const rows=await Promise.all(files.map(async path=>[path,await digest(path)] as const));
     const external=inputs.flatMap(input=>input.kind==="external"?input.names.map(name=>[`external:${name}`,semioHashV1(semioCanonicalV1((graph.externalNodes??{})[name]??(graph.externalNodes??{})["npm:"+name]??null))] as const):[]);
     const runtime=await Promise.all(inputs.flatMap(input=>input.kind==="runtime"?[input.command]:[]).map(async command=>{let known=runtimes.get(command);if(!known){known=ports.runtime!(command,signal);runtimes.set(command,known);}return [`runtime:${command}`,await known] as const;}));
     key=semioActionKeyV1({task:task.id,execution:task.definition.execution,args:task.args,outputs:declared??[],files:rows,environment:[...inputs.flatMap(input=>input.kind==="environment"?[[input.name,ports.environment[input.name]??""] as const]:[]),...external,...runtime],dependencies:task.dependencies.map(dependency=>[dependency,(dependencies[task.dependencies.indexOf(dependency)]??LOST).digest] as const)});
    }
   }
   const restore=async(entry:SemioEntryV1,source:"local"|"remote")=>{await ports.store.restore(ports.workspace,entry,signal);return finish(task,started,"restored",entry.digest,{source,key:entry.key});};
   const execute=async(owner:boolean,held:AbortSignal=signal):Promise<Outcome>=>{
    if(!owner||key!==undefined){const hit=key===undefined?undefined:await ports.store.lookup(key);if(hit)return restore(hit,"local");}
    let result;
    try{result=task.definition.execution.kind==="noop"?{exitCode:0}:await ports.execute(task,held,ports.write);}
    catch(error){if(signal.aborted)return finish(task,started,"cancelled","none");return finish(task,started,"failed","none",{reason:String((error as Error).message??error)});}
    if(result.exitCode!==0)return finish(task,started,"failed","none",{exitCode:result.exitCode});
    const captured=await ports.store.capture(ports.workspace,task.id,key??"0".repeat(64),outputs,key!==undefined,signal);
    if(captured.missing.length&&refuseMissing)return finish(task,started,"failed","none",{reason:`Declared output was not produced: ${captured.missing.join(", ")}`});
    if(key!==undefined){
     await ports.store.publish(captured.entry);
     if(ports.remote&&request.skipRemoteCache!==true)await ports.remote.put(key,await packSemioEntryV1(captured.entry,sha=>ports.store.readBlob(sha)),signal).catch(error=>{if(signal.aborted)throw error;});
    }
    return finish(task,started,"executed",captured.entry.digest,key?{key}:{});
   };
   if(decision.cache&&key!==undefined){
    const local=await ports.store.lookup(key);
    if(local)return await restore(local,"local");
    if(ports.remote&&request.skipRemoteCache!==true){
     const bytes=await ports.remote.get(key,signal);
     if(bytes){try{const entry=await unpackSemioEntryV1(bytes,key,(sha,body)=>ports.store.putBlob(sha,body));await ports.store.publish(entry);return await restore(entry,"remote");}catch(error){if(signal.aborted)return finish(task,started,"cancelled","none");}}
    }
    return await ports.store.withFlight(key,signal,execute);
   }
   return await execute(true);
  }catch(error){
   if(signal.aborted)return finish(task,started,"cancelled","none");
   return finish(task,started,"failed","none",{reason:String((error as Error).message??error)});
  }finally{
   slots.release();
   if(request.bail===true&&records.some(row=>row.status==="failed"))own.abort(Error("Bail"));
  }
 };
 const all=ids.map(schedule),regular=ids.filter(id=>!continuous(id));
 try{
  await Promise.all(regular.map(schedule));
  if(!records.some(row=>row.status==="failed")&&!signal.aborted&&hooks.onSettled)await hooks.onSettled();
  await Promise.all(plan.selected.filter(continuous).map(schedule));
 }finally{
  closing=true;own.abort(Error("The selected task closure completed"));
  await Promise.allSettled(all);
 }
 const failed=records.some(row=>row.status==="failed"),cancelled=hooks.signal.aborted;
 return {version:1,exitCode:cancelled?130:failed?1:0,cancelled,tasks:records};
}
