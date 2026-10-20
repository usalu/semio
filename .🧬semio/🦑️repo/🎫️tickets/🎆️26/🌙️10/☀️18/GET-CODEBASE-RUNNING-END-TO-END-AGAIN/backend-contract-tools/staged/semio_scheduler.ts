import {availableParallelism} from "node:os";
import {lstat,readdir} from "node:fs/promises";
import {join,resolve} from "node:path";
import {spawn} from "node:child_process";
import {semioActionKeyV1,semioSelectFilesV1,type SemioEntryV1,type SemioEventV1,type SemioRecordTaskV1,type SemioRecordV1,type SemioStatusV1} from "../🟦️.ts";
import {SemioStoreV1,semioFileDigestV1} from "../🗄️store/🟦️.ts";
import {packSemioEntryV1,unpackSemioEntryV1,type SemioRemoteV1} from "../🌐️remote/🟦️.ts";
import type {SemioExecutorV1} from "../🚀️exec/🟦️.ts";
import {buildCacheDecisionV1,buildTaskInputsV1,planBuildTasksV1,type BuildTaskRequestV1,type BuildTaskV1} from "../../../🧮️task-model/🟦️.ts";
import type {BuildGraphV1} from "../../../🕸️graph/🟦️.ts";

/** 🔌️ What a run needs from the outside: the workspace, the stores, how to list files, how to run a task body, the environment, a clock and a log. */
export type SemioPortsV1=Readonly<{workspace:string;store:SemioStoreV1;remote?:SemioRemoteV1;inventory():Promise<readonly string[]>;execute:SemioExecutorV1;environment:Readonly<Record<string,string|undefined>>;now():number;write(chunk:string):void}>;
/** 🎛️ Cancellation and progress of one run. */
export type SemioHooksV1=Readonly<{signal:AbortSignal;onEvent?:(event:SemioEventV1)=>void|Promise<void>}>;

const EXCLUDED=new Set([".git",".🧬semio",".cache",".nx","node_modules","target","dist","build","coverage",".venv","__pycache__"]);
const order=(left:string,right:string)=>left<right?-1:left>right?1:0;

/** 📚️ Lists the workspace's source files: Git's tracked and untracked-but-not-ignored files, or a portable walk when Git is absent. */
export async function semioInventoryV1(workspace:string):Promise<string[]>{
 const git=await new Promise<string[]|undefined>(accept=>{
  const child=spawn("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z"],{cwd:workspace,stdio:["ignore","pipe","ignore"],windowsHide:true}),chunks:Buffer[]=[];
  child.stdout.on("data",chunk=>chunks.push(chunk));child.once("error",()=>accept(undefined));
  child.once("close",code=>accept(code===0?Buffer.concat(chunks).toString().split("\0").filter(Boolean):undefined));
 });
 if(git)return [...new Set(git)].filter(path=>!path.split("/").some(part=>EXCLUDED.has(part))).sort(order);
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

/** 🏗️ Runs a request: plans the closure, restores every task whose action key is cached locally or remotely, executes the others with at most `parallel` at a time, stores their outputs, skips what depends on a failure, and stops on cancellation. The run record tells what each task did. */
export async function runSemioTasksV1(graph:BuildGraphV1,request:BuildTaskRequestV1,ports:SemioPortsV1,hooks:SemioHooksV1):Promise<SemioRecordV1>{
 const plan=planBuildTasksV1(graph,request),ids=Object.keys(plan.tasks).sort(order),total=ids.length;
 const own=new AbortController(),signal=AbortSignal.any([hooks.signal,own.signal]),slots=new Slots(Math.max(1,request.parallel??availableParallelism()));
 const records:SemioRecordTaskV1[]=[],results=new Map<string,Promise<Outcome>>(),digests=new Map<string,Promise<string>>();
 let done=0,inventory:Promise<readonly string[]>|undefined;
 const emit=async(event:Omit<SemioEventV1,"done"|"total">)=>{if(hooks.onEvent)await hooks.onEvent({...event,done,total});};
 const digest=(path:string)=>{let known=digests.get(path);if(!known){known=semioFileDigestV1(join(ports.workspace,path));digests.set(path,known);}return known;};
 const outputsOf=(task:BuildTaskV1)=>(task.definition.outputs??[]).map(pattern=>pattern.replaceAll("{projectRoot}",task.root==="."?"":task.root).replaceAll("{projectName}",task.project).replaceAll("{workspaceRoot}/","").replaceAll("{workspaceRoot}","").replace(/^\.\//u,"").replace(/^\/+/u,""));
 const finish=async(task:BuildTaskV1,started:number,status:SemioStatusV1,outputs:string,detail:Partial<SemioRecordTaskV1>={}):Promise<Outcome>=>{
  done++;records.push({id:task.id,status,startedMs:started,endedMs:ports.now(),...detail});
  await emit({kind:"end",task:task.id,status});
  return {status,digest:outputs};
 };
 const schedule=(id:string):Promise<Outcome>=>{
  let known=results.get(id);
  if(!known){known=work(plan.tasks[id]!);results.set(id,known);}
  return known;
 };
 const work=async(task:BuildTaskV1):Promise<Outcome>=>{
  const dependencies=await Promise.all(task.dependencies.map(schedule)),started=ports.now();
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
    if(inputs.some(input=>input.kind==="unsupported"))decision={cache:false,reason:"unsupported-input"};
    else{
     inventory??=ports.inventory();
     const files=semioSelectFilesV1(await inventory,inputs.flatMap(input=>input.kind==="file"&&!input.exclude?[input.pattern]:[]),[...inputs.flatMap(input=>input.kind==="file"&&input.exclude?[input.pattern]:[]),...outputs.flatMap(path=>[path,path+"/**"])]);
     const rows=await Promise.all(files.map(async path=>[path,await digest(path)] as const));
     key=semioActionKeyV1({task:task.id,execution:task.definition.execution,args:task.args,outputs:declared??[],files:rows,environment:inputs.flatMap(input=>input.kind==="environment"?[[input.name,ports.environment[input.name]??""] as const]:[]),dependencies:task.dependencies.map(dependency=>[dependency,dependencies[task.dependencies.indexOf(dependency)]!.digest] as const)});
    }
   }
   const restore=async(entry:SemioEntryV1,source:"local"|"remote")=>{await ports.store.restore(ports.workspace,entry,signal);return finish(task,started,"restored",entry.digest,{source,key:entry.key});};
   const execute=async(owner:boolean):Promise<Outcome>=>{
    if(!owner||key!==undefined){const hit=key===undefined?undefined:await ports.store.lookup(key);if(hit)return restore(hit,"local");}
    let result;
    try{result=task.definition.execution.kind==="noop"?{exitCode:0}:await ports.execute(task,signal,ports.write);}
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
 await Promise.all(ids.map(schedule));
 const failed=records.some(row=>row.status==="failed"),cancelled=hooks.signal.aborted;
 return {version:1,exitCode:cancelled?130:failed?1:0,cancelled,tasks:records};
}
