import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {lstat,readFile,readdir,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {StringDecoder} from "node:string_decoder";
import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {incrementalBazelObservationV1,incrementalBazelSpawnsV1} from "../../@@OBSERVERS@@/@@TS@@";
import {incrementalChildEnvironmentV1,incrementalCliArgumentsV1,incrementalInsideV1,incrementalPlanTasksV1,incrementalSliceFromGraphV1,type IncrementalHostV1} from "../../@@RUNNER@@/@@TS@@";
import type {IncrementalSliceV1,IncrementalToolV1} from "../../@@TS@@";
import {concurrentAttributeV1,concurrentDigestV1,concurrentIndependenceIssuesV1,concurrentLineKeeperV1,concurrentMergeWaitsV1,concurrentMutateV1,concurrentNxFitV1,concurrentTraceWaitsV1,concurrentWaitsV1,type ConcurrentMeasurementV1,type ConcurrentObservationV1,type ConcurrentObservedV1,type ConcurrentSessionResultV1,type ConcurrentSessionV1,type ConcurrentStepResultV1,type ConcurrentStepV1,type ConcurrentWorkloadSetV1,type ConcurrentWorkloadV1} from "../@@TS@@";
import {terminateOwnedChildTree} from "@@EIGHT@@/@@MODULES@@/@@PROCESS@@/@@TERMINATION@@/@@TS@@";

const policy=schema.definitions.Policy.const,order=(left:string,right:string)=>left<right?-1:left>right?1:0;
const both=(english:string,german:string)=>`${english}\n${german}`;

/** 🎞 One finished command: its exit code, the stderr lines worth keeping with their arrival time, the digest of its stdout and when it started and ended. */
export type ConcurrentCaptureV1=Readonly<{code:number|null;lines:readonly Readonly<{atMs:number;line:string}>[];stdout:string;startMs:number;endMs:number}>;

/** ⚙ Everything a workload run needs from the outside: the checkout copy, the closure of a target, one command with its stderr, the observation of a run, the clock and cooperative cancellation. */
export type ConcurrentPortsV1=Readonly<{
 read(path:string):Promise<string>;
 write(path:string,text:string):Promise<void>;
 digest(paths:readonly string[]):Promise<string|null>;
 slice(tool:IncrementalToolV1,target:string,files:readonly string[]):Promise<IncrementalSliceV1>;
 spawn(tool:IncrementalToolV1,args:readonly string[],extra:Readonly<Record<string,string>>,keep:(line:string)=>boolean,signal:AbortSignal):Promise<ConcurrentCaptureV1>;
 reportPath(tool:IncrementalToolV1,session:string,index:number,phase:"concurrent"|"serial"):string|undefined;
 tracePath(session:string,index:number,phase:"concurrent"|"serial"):string;
 readTrace(path:string):Promise<string>;
 observe(tool:IncrementalToolV1,slice:IncrementalSliceV1,reportPath:string|undefined,window:Readonly<{startMs:number;endMs:number}>,concurrent:boolean):Promise<Readonly<{observation:ConcurrentObservationV1|null;observed:ConcurrentObservedV1}>>;
 now():number;
 nonce():string;
 check():void;
 progress(stage:string):Promise<void>;
 signal:AbortSignal;
}>;

/** 💤 Sleeps for a time and wakes early with the abort reason when the signal fires. */
export function concurrentSleepV1(milliseconds:number,signal:AbortSignal):Promise<void>{
 return new Promise((accept,reject)=>{
  if(signal.aborted){reject(signal.reason);return;}
  const stop=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal.removeEventListener("abort",stop);accept();},milliseconds);
  signal.addEventListener("abort",stop,{once:true});
 });
}

type BarriersV1=Readonly<{arrive(id:string,session:string):Promise<void>;leave(session:ConcurrentSessionV1):void}>;
/** 🚥 Lets the sessions that mention a barrier wait for each other; a session that ends or fails before reaching it stops being waited for. */
export function concurrentBarriersV1(sessions:readonly ConcurrentSessionV1[],signal:AbortSignal):BarriersV1{
 const remaining=new Map<string,number>(),waiting=new Map<string,(()=>void)[]>(),arrived=new Set<string>();
 for(const session of sessions)for(const step of session.steps)if(step.kind==="barrier")remaining.set(step.id,(remaining.get(step.id)??0)+1);
 const release=(id:string):void=>{for(const done of waiting.get(id)??[])done();waiting.set(id,[]);};
 const settle=(id:string):void=>{const left=(remaining.get(id)??0)-1;remaining.set(id,left);if(left<=0)release(id);};
 return Object.freeze({
  arrive(id:string,session:string):Promise<void>{
   arrived.add(`${id}\0${session}`);settle(id);
   if((remaining.get(id)??0)<=0)return Promise.resolve();
   return new Promise<void>((accept,reject)=>{
    const stop=()=>reject(signal.reason),done=()=>{signal.removeEventListener("abort",stop);accept();};
    waiting.set(id,[...waiting.get(id)??[],done]);
    if(signal.aborted)stop();else signal.addEventListener("abort",stop,{once:true});
   });
  },
  leave(session:ConcurrentSessionV1):void{for(const step of session.steps)if(step.kind==="barrier"&&!arrived.has(`${step.id}\0${session.id}`)){arrived.add(`${step.id}\0${session.id}`);settle(step.id);}}
 });
}

type WindowV1={startMs:number;endMs:number|null};

/** 📡 What a workload would do with one tool: the closure of every target it runs, the tasks that always execute, the edited files that do not exist in the copy and the files one agent edits that another agent's targets read. */
export type ConcurrentPlanV1=Readonly<{targets:readonly Readonly<{target:string;tasks:number;uncached:readonly string[]}>[];uncached:readonly string[];missing:readonly string[];issues:readonly string[];slices:ReadonlyMap<string,IncrementalSliceV1>}>;

/** 🗓 Plans a workload without running a command of it: slices every run target through the ports and checks that the agents declared unrelated are independent. */
export async function concurrentPlanV1(workload:ConcurrentWorkloadV1,tool:IncrementalToolV1,ports:Pick<ConcurrentPortsV1,"check"|"read"|"slice">):Promise<ConcurrentPlanV1>{
 const sessions=workload.sessions,steps=[...workload.setup,...sessions.flatMap(session=>session.steps)],edited=[...new Set(steps.flatMap(step=>step.kind==="edit"?[step.file]:[]))].sort(order),targets=[...new Set(steps.flatMap(step=>step.kind==="run"?[step.target]:[]))].sort(order);
 const slices=new Map<string,IncrementalSliceV1>(),uncached=new Set<string>(),missing:string[]=[];
 for(const file of edited)try{await ports.read(file);}catch{missing.push(file);}
 for(const target of targets){ports.check();const slice=await ports.slice(tool,target,edited);slices.set(target,slice);for(const[id,task]of Object.entries(slice.tasks))if(!task.cache)uncached.add(id);}
 const touches=(session:ConcurrentSessionV1):string[]=>edited.filter(file=>session.steps.some(step=>step.kind==="run"&&Object.values(slices.get(step.target)!.tasks).some(task=>task.inputs.includes(file))));
 const issues=concurrentIndependenceIssuesV1(sessions.map(session=>({id:session.id,group:session.group,edits:session.steps.flatMap(step=>step.kind==="edit"?[step.file]:[]),touches:touches(session)})));
 return Object.freeze({targets:targets.map(target=>({target,tasks:Object.keys(slices.get(target)!.tasks).length,uncached:Object.entries(slices.get(target)!.tasks).filter(([,task])=>!task.cache).map(([id])=>id).sort(order)})),uncached:[...uncached].sort(order),missing,issues,slices});
}

/** 🏃 Runs one workload with one tool in the checkout copy: the setup in order, then every session at the same time (barriers line them up), then every session once more in a row as the serial reference. Edits are held in one place and restored exactly, also when a command fails or the run is cancelled. */
export async function concurrentRunWorkloadV1(set:ConcurrentWorkloadSetV1,workload:ConcurrentWorkloadV1,tool:IncrementalToolV1,ports:ConcurrentPortsV1):Promise<ConcurrentMeasurementV1>{
 ports.check();
 const sessions=workload.sessions;
 await ports.progress(`${workload.id}:${tool}:slice`);
 const plan=await concurrentPlanV1(workload,tool,ports),slices=plan.slices,uncached=new Set(plan.uncached);
 if(plan.missing.length)throw Error(both(`Edited files are missing in the copy: ${plan.missing.join(", ")}`,`Geänderte Dateien fehlen in der Kopie: ${plan.missing.join(", ")}`));
 if(plan.issues.length)throw Error(both(`The workload ${workload.id} is not independent: ${plan.issues.join("; ")}`,`Die Arbeitslast ${workload.id} ist nicht unabhängig: ${plan.issues.join("; ")}`));
 const held=new Map<string,string>(),editedBy=new Map<string,Set<string>>(),windows:WindowV1[]=[],controller=new AbortController(),signal=AbortSignal.any([ports.signal,controller.signal]);
 const apply=async(owner:string,step:Extract<ConcurrentStepV1,{kind:"edit"}>):Promise<void>=>{
  const current=await ports.read(step.file);
  if(!held.has(step.file))held.set(step.file,current);
  editedBy.set(owner,(editedBy.get(owner)??new Set()).add(step.file));
  await ports.write(step.file,concurrentMutateV1(current,step.op,step.language,ports.nonce()));
 };
 const restore=async(owner:string):Promise<void>=>{
  for(const file of [...editedBy.get(owner)??[]]){const original=held.get(file);if(original!==undefined)await ports.write(file,original);held.delete(file);}
  editedBy.delete(owner);
 };
 const execute=async(mode:"concurrent"|"serial",origin:number,session:string,index:number,step:Extract<ConcurrentStepV1,{kind:"run"|"repo"}>):Promise<ConcurrentStepResultV1>=>{
  const args=step.kind==="run"?["run",step.target]:[...step.args],path=mode==="concurrent"&&step.kind==="run"?ports.reportPath(tool,session,index,mode):undefined,trace=mode==="concurrent"?ports.tracePath(session,index,mode):undefined,extra={...(path?{[policy.taskReport]:path}:{}),...(trace?{[policy.leaseTrace]:trace}:{})},track:WindowV1={startMs:ports.now(),endMs:null},watched=mode==="concurrent"&&tool==="nx"&&step.kind==="run";
  if(watched)windows.push(track);
  let capture:ConcurrentCaptureV1;
  try{capture=await ports.spawn(tool,args,extra,concurrentLineKeeperV1(),signal);}finally{track.endMs=ports.now();}
  const window={startMs:capture.startMs,endMs:capture.endMs},overlap=windows.some(other=>other!==track&&other.startMs<=window.endMs&&(other.endMs??Infinity)>=window.startMs);
  let observation:ConcurrentObservationV1|null=null,observed:ConcurrentObservedV1="none";
  if(mode==="concurrent"&&step.kind==="run")try{const seen=await ports.observe(tool,slices.get(step.target)!,path,window,overlap);observation=seen.observation;observed=seen.observation?seen.observed:"ambiguous";}catch{observation=null;observed="none";}
  const outputs=step.outputs?.length?await ports.digest(step.outputs):null;
  const waits=mode==="concurrent"?concurrentMergeWaitsV1(concurrentWaitsV1(capture.lines.map(row=>({atMs:row.atMs-origin,line:row.line})),capture.endMs-origin,set.classes),concurrentTraceWaitsV1(await ports.readTrace(trace!),origin,set.classes)):[];
  return Object.freeze({index,kind:step.kind,command:Object.freeze([...args]),startMs:capture.startMs-origin,endMs:capture.endMs-origin,exit:capture.code,waits,observation,observed,outputs,stdout:step.kind==="repo"&&step.stdout?capture.stdout:null});
 };
 const barriers=concurrentBarriersV1(sessions,signal);
 const play=async(mode:"concurrent"|"serial",origin:number,session:ConcurrentSessionV1):Promise<ConcurrentSessionResultV1>=>{
  const steps:ConcurrentStepResultV1[]=[];
  try{
   if(mode==="concurrent"&&session.delayMilliseconds)await concurrentSleepV1(session.delayMilliseconds,signal);
   const startMs=ports.now()-origin;
   for(const[index,step]of session.steps.entries()){
    ports.check();await ports.progress(`${workload.id}:${tool}:${mode}:${session.id}:${index}`);
    if(step.kind==="edit")await apply(session.id,step);
    else if(step.kind==="revert")await restore(session.id);
    else if(step.kind==="barrier"){if(mode==="concurrent")await barriers.arrive(step.id,session.id);}
    else if(step.kind==="pause"){if(mode==="concurrent")await concurrentSleepV1(step.milliseconds,signal);}
    else steps.push(await execute(mode,origin,session.id,index,step));
   }
   return Object.freeze({id:session.id,group:session.group,startMs,endMs:ports.now()-origin,steps});
  }finally{await restore(session.id);if(mode==="concurrent")barriers.leave(session);}
 };
 try{
  await ports.progress(`${workload.id}:${tool}:setup`);
  const setupOrigin=ports.now();
  for(const[index,step]of workload.setup.entries()){
   ports.check();
   if(step.kind==="edit")await apply("setup",step);
   else if(step.kind==="run"||step.kind==="repo")await execute("serial",setupOrigin,"setup",index,step);
  }
  await ports.progress(`${workload.id}:${tool}:concurrent`);
  const origin=ports.now();
  const settled=await Promise.allSettled(sessions.map(session=>play("concurrent",origin,session).catch(error=>{controller.abort(error);throw error;}))),failure=settled.find(row=>row.status==="rejected");
  if(failure)throw (failure as PromiseRejectedResult).reason;
  const results=settled.map(row=>(row as PromiseFulfilledResult<ConcurrentSessionResultV1>).value);
  await ports.progress(`${workload.id}:${tool}:serial`);
  const serialOrigin=ports.now(),serial:ConcurrentSessionResultV1[]=[];
  for(const session of sessions)serial.push(await play("serial",serialOrigin,session));
  return Object.freeze({workload:workload.id,tool,uncached:[...uncached].sort(order),sessions:concurrentAttributeV1(results),serial});
 }finally{
  controller.abort(Error("Concurrent workload finished"));
  for(const owner of [...editedBy.keys()])await restore(owner);
  for(const[file,original]of [...held].reverse())await ports.write(file,original);
  held.clear();
 }
}

/** 🏡 What the real ports need beyond the incremental host: the process spawner, replaceable in tests. */
export type ConcurrentHostV1=IncrementalHostV1&Readonly<{spawn?:ConcurrentPortsV1["spawn"]}>;

/** 🌊 Runs one command with its own copy of the CLI and a process tree that is terminated on abort: stdout is only digested, stderr is split into lines stamped with their arrival time and filtered by `keep`. */
export function concurrentSpawnV1(command:string,args:readonly string[],cwd:string,env:Readonly<Record<string,string>>,keep:(line:string)=>boolean,signal:AbortSignal,now:()=>number):Promise<ConcurrentCaptureV1>{
 return new Promise((accept,reject)=>{
  if(signal.aborted){reject(signal.reason);return;}
  const startMs=now(),child=spawn(command,[...args],{cwd,env:env as NodeJS.ProcessEnv,detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"],windowsHide:true}),hash=createHash("sha256"),decoder=new StringDecoder("utf8"),lines:{atMs:number;line:string}[]=[];
  let pending="",bytes=0;
  const stop=()=>terminateOwnedChildTree(child),push=(line:string)=>{if(lines.length<policy.maximumLines&&keep(line))lines.push({atMs:now(),line});};
  signal.addEventListener("abort",stop,{once:true});
  child.stdout!.on("data",(chunk:Buffer)=>{hash.update(chunk);});
  child.stderr!.on("data",(chunk:Buffer)=>{
   bytes+=chunk.length;if(bytes>policy.maximumLogBytes)return;
   pending+=decoder.write(chunk);
   for(let cut=pending.indexOf("\n");cut>=0;cut=pending.indexOf("\n")){push(pending.slice(0,cut).replace(/\r$/u,""));pending=pending.slice(cut+1);}
  });
  child.on("error",error=>{signal.removeEventListener("abort",stop);reject(error);});
  child.on("close",code=>{
   signal.removeEventListener("abort",stop);
   pending+=decoder.end();if(pending)push(pending.replace(/\r$/u,""));
   if(signal.aborted){reject(signal.reason);return;}
   accept(Object.freeze({code,lines,stdout:hash.digest("hex"),startMs,endMs:now()}));
  });
 });
}

async function filesUnder(root:string,path:string,found:{path:string;bytes:Uint8Array|string}[],budget:{left:number}):Promise<void>{
 const absolute=incrementalInsideV1(root,path),meta=await lstat(absolute).catch(()=>undefined);
 if(!meta){found.push({path,bytes:"\0missing"});return;}
 if(meta.isDirectory()){for(const name of (await readdir(absolute)).sort(order))await filesUnder(root,`${path}/${name}`,found,budget);return;}
 if(--budget.left<0)throw Error("Bounded output inventory required");
 found.push({path,bytes:meta.isFile()?await readFile(absolute):"\0link"});
}

/** 🏢 The real ports: reads and writes inside the disposable root, slices from the CLI's own graph, commands through the disposable root's own CLI, observations through Bazel's compact execution log or Nx's run record. */
export function concurrentCliPortsV1(host:ConcurrentHostV1,signal:AbortSignal):ConcurrentPortsV1{
 let sequence=0;const graphs=new Map<IncrementalToolV1,unknown>();
 const cli=(tool:IncrementalToolV1,args:readonly string[],extra:Readonly<Record<string,string>>={})=>{const command=incrementalCliArgumentsV1(host.executable,host.root,host.script,tool,args);return {command:command.command,args:command.args,env:incrementalChildEnvironmentV1(host.environment,extra)};};
 const lastLine=(output:string)=>output.trim().split("\n").at(-1)!.trim();
 return Object.freeze({
  read:path=>readFile(incrementalInsideV1(host.root,path),"utf8"),
  write:async(path,text)=>{await writeFile(incrementalInsideV1(host.root,path),text);},
  digest:async paths=>{if(!paths.length)return null;const found:{path:string;bytes:Uint8Array|string}[]=[];for(const path of paths)await filesUnder(host.root,path,found,{left:100000});return concurrentDigestV1(found);},
  now:host.now,
  nonce:()=>host.now().toString(36).slice(-8).padStart(8,"0")+(++sequence).toString(36),
  check:host.check,
  progress:host.progress,
  signal,
  reportPath:(tool,session,index,phase)=>tool==="bazel"?join(host.scratch,`bazel-${session}-${phase}-${index}-${++sequence}.zst`):undefined,
  tracePath:(session,index,phase)=>join(host.scratch,`trace-${session}-${phase}-${index}-${++sequence}.jsonl`),
  readTrace:async path=>{const text=await readFile(path,"utf8").catch(()=>"");await rm(path,{force:true});return text;},
  slice:async(tool,target,files)=>{
   if(!graphs.has(tool)){const call=cli(tool,["graph",`--file=${join(host.scratch,`graph-${tool}.json`)}`]);graphs.set(tool,JSON.parse(await readFile(lastLine(await host.run(call.command,call.args,host.root,call.env,host.signal)),"utf8")));}
   const planCall=cli(tool,["run",target,"--dry-run"]),planned=incrementalPlanTasksV1(await host.run(planCall.command,planCall.args,host.root,planCall.env,host.signal)),slice=incrementalSliceFromGraphV1(await host.adapter(),graphs.get(tool),target,files,await host.inventory()),missing=planned.ids.filter(id=>!slice.tasks[id]);
   if(missing.length)throw Error(`The graph planner and ${tool} disagree about the closure of ${target}: ${missing.join(", ")}`);
   return slice;
  },
  spawn:host.spawn??((tool,args,extra,keep,stop)=>{const call=cli(tool,args,extra);return concurrentSpawnV1(call.command,call.args,host.root,call.env,keep,stop,host.now);}),
  observe:async(tool,slice,reportPath,window,concurrent)=>{
   const ids=Object.keys(slice.tasks);
   if(tool==="bazel"){
    const observation=incrementalBazelObservationV1(incrementalBazelSpawnsV1(await readFile(reportPath!)),ids);
    await rm(reportPath!,{force:true});
    return {observation:{executed:observation.executed,restored:observation.restored,cached:observation.cached},observed:"exact"};
   }
   return concurrentNxFitV1(JSON.parse(await readFile(host.nxRecord(host.root),"utf8")),ids,window,concurrent);
  }
 });
}
