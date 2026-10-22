import {randomUUID} from "node:crypto";
import {decodeProtobufV1,REAPI_V1} from "../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {remoteExecutionDigestV1,remoteExecutionPlatformKeyV1,remoteExecutionPlatformMatchesV1,type RemoteExecutionDigestV1,type RemoteExecutionHeartbeatAnswerV1,type RemoteExecutionPollAnswerV1,type RemoteExecutionPropertyV1,type RemoteExecutionTaskResultV1,type RemoteExecutionTaskStatusV1,type RemoteExecutionTaskV1,type RemoteExecutionWorkerRegisteredV1,type RemoteExecutionWorkerRegistrationV1} from "../🟦️.ts";

/** 🎞️ The stages an operation passes through, in the protocol's own words. */
export type RemoteExecutionStageV1="CACHE_CHECK"|"QUEUED"|"EXECUTING"|"COMPLETED";

/** 🧾️ What a finished operation carries: the status, the serialized ActionResult when there is one, and whether it came from the action cache. */
export type RemoteExecutionOutcomeV1=Readonly<{status:RemoteExecutionTaskStatusV1;actionResult:Uint8Array|null;cached:boolean;worker:string}>;

/** 📸️ An immutable view of one operation. */
export type RemoteExecutionSnapshotV1=Readonly<{name:string;instance:string;action:RemoteExecutionDigestV1;stage:RemoteExecutionStageV1;outcome?:RemoteExecutionOutcomeV1}>;

/** 📣️ Something worth logging. */
export type RemoteExecutionSchedulerEventV1=Readonly<{stage:"submitted"|"cached"|"assigned"|"completed"|"retried"|"cancelled"|"worker-joined"|"worker-left"|"worker-lost"|"unmatched";detail:string}>;

/** ⚙️ Bounds of the scheduler; all durations are seconds. */
export type RemoteExecutionSchedulerOptionsV1=Readonly<{content:RemoteExecutionContentV1;unmatchedSeconds:number;lingerSeconds:number;leaseSeconds:number;maximumAttempts:number;defaultActionSeconds:number;maximumQueued:number;heartbeatSeconds:number;pollSeconds:number;cache:Readonly<{url:string;relative:boolean}>;language:"en"|"de"|"none";sweepMilliseconds?:number;now?:()=>number;onEvent?:(event:RemoteExecutionSchedulerEventV1)=>void}>;

type Operation={
 id:string;instance:string;action:RemoteExecutionDigestV1;key:string;platform:RemoteExecutionPropertyV1[];timeoutSeconds:number;stage:RemoteExecutionStageV1;version:number;createdAt:number;queuedAt:number;
 attempt:number;worker?:string;startedAt?:number;outcome?:RemoteExecutionOutcomeV1;subscribers:Set<()=>void>;cancelRequested:boolean;lingerTimer?:ReturnType<typeof setTimeout>;
};

type Worker={
 id:string;registration:RemoteExecutionWorkerRegistrationV1;lastSeen:number;running:Set<string>;cancel:Set<string>;waiter?:(answer:RemoteExecutionPollAnswerV1)=>void;draining:boolean;
};

const OK:RemoteExecutionTaskStatusV1={code:0,message:""},MISSING=(digest:RemoteExecutionDigestV1):RemoteExecutionTaskStatusV1=>({code:9,message:`Missing blob ${digest.hash}/${digest.size}`,missing:[digest]});

/** 🗓️ The scheduler: deduplicates identical actions, answers from the action cache, queues actions per platform, hands them to long-polling workers, tracks leases and heartbeats, retries lost work, cancels abandoned work and keeps counters. Pure logic over a content store; transports are separate. */
export class RemoteExecutionSchedulerV1{
 private readonly operations=new Map<string,Operation>();
 private readonly byKey=new Map<string,Operation>();
 private readonly queue:Operation[]=[];
 private readonly workers=new Map<string,Worker>();
 private readonly counters={queued:0,executing:0,completed:0,cached:0,failed:0,cancelled:0,retried:0,deduplicated:0};
 private readonly clock:()=>number;
 private timer?:ReturnType<typeof setInterval>;
 readonly startedAt:string;

 constructor(readonly options:RemoteExecutionSchedulerOptionsV1){
  this.clock=options.now??Date.now;this.startedAt=new Date(this.clock()).toISOString();
  const every=options.sweepMilliseconds??1000;
  if(every>0){this.timer=setInterval(()=>{this.sweep();},every);this.timer.unref();}
 }

 private emit(stage:RemoteExecutionSchedulerEventV1["stage"],detail:string):void{this.options.onEvent?.({stage,detail});}
 private touch(operation:Operation):void{operation.version++;for(const wake of operation.subscribers)wake();}
 private snapshot(operation:Operation):RemoteExecutionSnapshotV1{return Object.freeze({name:`operations/${operation.id}`,instance:operation.instance,action:operation.action,stage:operation.stage,...(operation.outcome?{outcome:operation.outcome}:{})});}
 private label(operation:Operation):string{return `${operation.action.hash.slice(0,12)}/${operation.action.size}`;}

 private finish(operation:Operation,outcome:RemoteExecutionOutcomeV1):void{
  if(operation.stage==="COMPLETED")return;
  operation.stage="COMPLETED";operation.outcome=outcome;
  const index=this.queue.indexOf(operation);if(index>=0)this.queue.splice(index,1);
  if(operation.worker){this.workers.get(operation.worker)?.running.delete(`${operation.id}.${operation.attempt}`);}
  if(operation.lingerTimer)clearTimeout(operation.lingerTimer);
  this.byKey.delete(operation.key);
  if(outcome.cached)this.counters.cached++;
  else if(outcome.status.code===0)this.counters.completed++;
  else if(outcome.status.code===1)this.counters.cancelled++;
  else this.counters.failed++;
  this.emit("completed",`${this.label(operation)} code ${outcome.status.code}${outcome.cached?" cached":""}`);
  this.touch(operation);
  setTimeout(()=>{this.operations.delete(operation.id);},60000).unref();
  this.dispatch();
 }

 private async prepare(operation:Operation,skipCache:boolean):Promise<void>{
  try{
   const bytes=await this.options.content.read(operation.instance,operation.action);
   if(!bytes)return this.finish(operation,{status:MISSING(operation.action),actionResult:null,cached:false,worker:""});
   const action=decodeProtobufV1(REAPI_V1.action,bytes);
   let platform:{name:string;value:string}[]=action.platform?.properties??[];
   if(platform.length===0&&action.command_digest){
    const commandDigest=remoteExecutionDigestV1(action.command_digest.hash,action.command_digest.size_bytes),command=await this.options.content.read(operation.instance,commandDigest);
    if(!command)return this.finish(operation,{status:MISSING(commandDigest),actionResult:null,cached:false,worker:""});
    platform=decodeProtobufV1(REAPI_V1.command,command).platform?.properties??[];
   }
   operation.platform=platform.map(row=>({name:row.name,value:row.value}));operation.key=`${operation.instance}|${operation.action.hash}`;
   const seconds=action.timeout?Math.ceil(Number(action.timeout.seconds)+Number(action.timeout.nanos)/1e9):0;
   operation.timeoutSeconds=seconds>0?Math.min(seconds,this.options.defaultActionSeconds*4):this.options.defaultActionSeconds;
   if(!skipCache){
    const cached=await this.options.content.readActionResult(operation.instance,operation.action.hash).catch(()=>undefined);
    if(cached){
     try{decodeProtobufV1(REAPI_V1.actionResult,cached);this.emit("cached",this.label(operation));return this.finish(operation,{status:OK,actionResult:cached,cached:true,worker:""});}catch{}
    }
   }
   if(operation.stage==="COMPLETED")return;
   if(this.queue.length>=this.options.maximumQueued)return this.finish(operation,{status:{code:8,message:"Execution queue is full"},actionResult:null,cached:false,worker:""});
   operation.stage="QUEUED";operation.queuedAt=this.clock();this.queue.push(operation);this.counters.queued++;this.touch(operation);this.dispatch();
  }catch(error){
   this.finish(operation,{status:{code:14,message:`Preparing the action failed: ${error instanceof Error?error.message:String(error)}`},actionResult:null,cached:false,worker:""});
  }
 }

 private attach(operation:Operation,wake:()=>void):void{
  operation.subscribers.add(wake);
  if(operation.lingerTimer){clearTimeout(operation.lingerTimer);delete operation.lingerTimer;}
 }

 private detach(operation:Operation,wake:()=>void):void{
  operation.subscribers.delete(wake);
  if(operation.subscribers.size>0||operation.stage==="COMPLETED")return;
  operation.lingerTimer=setTimeout(()=>{this.cancel(operation);},this.options.lingerSeconds*1000);
  operation.lingerTimer.unref();
 }

 private cancel(operation:Operation):void{
  if(operation.stage==="COMPLETED"||operation.subscribers.size>0)return;
  this.emit("cancelled",this.label(operation));
  if(operation.stage==="EXECUTING"&&operation.worker){
   const worker=this.workers.get(operation.worker);
   if(worker){operation.cancelRequested=true;worker.cancel.add(`${operation.id}.${operation.attempt}`);worker.waiter?.({version:1,task:null,cancel:[...worker.cancel],known:true});delete worker.waiter;return;}
  }
  this.finish(operation,{status:{code:1,message:"Cancelled: no client waits for the result"},actionResult:null,cached:false,worker:operation.worker??""});
 }

 private async *watch(operation:Operation,signal:AbortSignal):AsyncGenerator<RemoteExecutionSnapshotV1>{
  let wake!:()=>void,seen=-1;
  const changed=():void=>{wake();};
  this.attach(operation,changed);
  try{
   for(;;){
    if(operation.version!==seen){seen=operation.version;yield this.snapshot(operation);if(operation.stage==="COMPLETED")return;}
    if(signal.aborted)return;
    await new Promise<void>(resolve=>{wake=resolve;signal.addEventListener("abort",()=>{resolve();},{once:true});if(operation.version!==seen)resolve();});
   }
  }finally{this.detach(operation,changed);}
 }

 /** 🚀️ Submits an action (or joins the identical one already in flight) and streams its operation snapshots until it completes or the caller goes away. */
 execute(request:Readonly<{instance:string;action:RemoteExecutionDigestV1;skipCacheLookup:boolean}>,signal:AbortSignal):AsyncGenerator<RemoteExecutionSnapshotV1>{
  const key=`${request.instance}|${request.action.hash}`,existing=this.byKey.get(key);
  if(existing&&existing.stage!=="COMPLETED"){this.counters.deduplicated++;return this.watch(existing,signal);}
  const operation:Operation={id:randomUUID(),instance:request.instance,action:request.action,key,platform:[],timeoutSeconds:this.options.defaultActionSeconds,stage:"CACHE_CHECK",version:0,createdAt:this.clock(),queuedAt:this.clock(),attempt:1,subscribers:new Set(),cancelRequested:false};
  this.operations.set(operation.id,operation);this.byKey.set(key,operation);this.emit("submitted",this.label(operation));
  const watching=this.watch(operation,signal);
  void this.prepare(operation,request.skipCacheLookup);
  return watching;
 }

 /** 🔁️ Re-attaches to a known operation (`WaitExecution`). */
 wait(name:string,signal:AbortSignal):AsyncGenerator<RemoteExecutionSnapshotV1>|undefined{
  const operation=this.operations.get(name.replace(/^operations\//u,""));
  return operation?this.watch(operation,signal):undefined;
 }

 /** 👷️ Registers a worker. */
 register(registration:RemoteExecutionWorkerRegistrationV1):RemoteExecutionWorkerRegisteredV1{
  const id=randomUUID();
  this.workers.set(id,{id,registration,lastSeen:this.clock(),running:new Set(),cancel:new Set(),draining:false});
  this.emit("worker-joined",`${registration.name} ${registration.platform} x${registration.concurrency}`);
  this.dispatch();
  return {version:1,id,heartbeatSeconds:this.options.heartbeatSeconds,pollSeconds:this.options.pollSeconds,leaseSeconds:this.options.leaseSeconds,cache:this.options.cache,language:this.options.language};
 }

 /** 👋️ Removes a worker; whatever it still runs is requeued. */
 deregister(workerId:string):void{
  const worker=this.workers.get(workerId);
  if(!worker)return;
  this.drop(worker,"worker-left");
 }

 private drop(worker:Worker,stage:"worker-left"|"worker-lost"):void{
  this.workers.delete(worker.id);
  worker.waiter?.({version:1,task:null,cancel:[],known:false});delete worker.waiter;
  this.emit(stage,worker.registration.name);
  for(const id of worker.running){
   const operation=this.operations.get(id.split(".")[0]!);
   if(operation&&operation.stage==="EXECUTING"&&`${operation.id}.${operation.attempt}`===id)this.requeue(operation,`Worker ${worker.registration.name} left`);
  }
  this.dispatch();
 }

 private requeue(operation:Operation,reason:string):void{
  if(operation.cancelRequested)return this.finish(operation,{status:{code:1,message:"Cancelled"},actionResult:null,cached:false,worker:operation.worker??""});
  if(operation.attempt>=this.options.maximumAttempts)return this.finish(operation,{status:{code:14,message:`${reason}; gave up after ${operation.attempt} attempts`},actionResult:null,cached:false,worker:operation.worker??""});
  operation.attempt++;operation.stage="QUEUED";delete operation.worker;delete operation.startedAt;operation.queuedAt=this.clock();this.queue.unshift(operation);this.counters.retried++;
  this.emit("retried",`${this.label(operation)}: ${reason}`);this.touch(operation);
 }

 private dispatch():void{
  for(const worker of this.workers.values()){
   if(!worker.waiter||worker.draining)continue;
   if(worker.running.size>=worker.registration.concurrency)continue;
   const index=this.queue.findIndex(operation=>remoteExecutionPlatformMatchesV1(operation.platform,worker.registration.properties));
   if(index<0)continue;
   const operation=this.queue.splice(index,1)[0]!,task:RemoteExecutionTaskV1={id:`${operation.id}.${operation.attempt}`,instance:operation.instance,action:operation.action,timeoutSeconds:operation.timeoutSeconds,attempt:operation.attempt};
   operation.stage="EXECUTING";operation.worker=worker.id;operation.startedAt=this.clock();this.counters.executing++;worker.running.add(task.id);
   this.emit("assigned",`${this.label(operation)} -> ${worker.registration.name}`);
   this.touch(operation);
   const answer=worker.waiter;delete worker.waiter;answer({version:1,task,cancel:[...worker.cancel],known:true});
  }
 }

 /** 📥️ A worker's long poll: answers with a task as soon as one matches, with the tasks to cancel, or with nothing after `waitMilliseconds`. */
 poll(workerId:string,waitMilliseconds:number,signal:AbortSignal):Promise<RemoteExecutionPollAnswerV1>{
  const worker=this.workers.get(workerId);
  if(!worker)return Promise.resolve({version:1,task:null,cancel:[],known:false});
  worker.lastSeen=this.clock();
  if(worker.cancel.size>0)return Promise.resolve({version:1,task:null,cancel:[...worker.cancel],known:true});
  worker.waiter?.({version:1,task:null,cancel:[],known:true});
  return new Promise(resolve=>{
   let timer:ReturnType<typeof setTimeout>|undefined;
   const settle=(answer:RemoteExecutionPollAnswerV1):void=>{if(timer)clearTimeout(timer);signal.removeEventListener("abort",onAbort);if(worker.waiter===settle)delete worker.waiter;resolve(answer);},
    onAbort=():void=>{settle({version:1,task:null,cancel:[],known:this.workers.has(workerId)});};
   worker.waiter=settle;timer=setTimeout(()=>{worker.lastSeen=this.clock();settle({version:1,task:null,cancel:[...worker.cancel],known:this.workers.has(workerId)});},waitMilliseconds);
   signal.addEventListener("abort",onAbort,{once:true});
   this.dispatch();
  });
 }

 /** 💓️ Marks a worker alive and tells it which of its tasks to abandon. */
 heartbeat(workerId:string,running:readonly string[]):RemoteExecutionHeartbeatAnswerV1{
  const worker=this.workers.get(workerId);
  if(!worker)return {version:1,known:false,cancel:[]};
  worker.lastSeen=this.clock();
  for(const id of [...worker.running])if(!running.includes(id)){
   const operation=this.operations.get(id.split(".")[0]!);
   if(operation&&operation.stage==="EXECUTING"&&`${operation.id}.${operation.attempt}`===id&&this.clock()-(operation.startedAt??0)>this.options.leaseSeconds*1000)this.requeue(operation,"Worker no longer runs the task");
   worker.running.delete(id);
  }
  return {version:1,known:true,cancel:[...worker.cancel].filter(id=>running.includes(id))};
 }

 /** 🏁️ Takes a worker's report of a task. */
 report(workerId:string,taskId:string,result:RemoteExecutionTaskResultV1):boolean{
  const worker=this.workers.get(workerId),operation=this.operations.get(taskId.split(".")[0]!);
  if(worker){worker.lastSeen=this.clock();worker.running.delete(taskId);worker.cancel.delete(taskId);}
  if(!operation||operation.stage!=="EXECUTING"||`${operation.id}.${operation.attempt}`!==taskId||operation.worker!==workerId)return false;
  const bytes=result.actionResult===null?null:new Uint8Array(Buffer.from(result.actionResult,"base64")),name=worker?.registration.name??"";
  if(result.outcome==="cancelled"){operation.cancelRequested?this.finish(operation,{status:{code:1,message:"Cancelled"},actionResult:null,cached:false,worker:name}):this.requeue(operation,"Worker abandoned the task");return true;}
  if(result.outcome==="failed"){
   if([10,14].includes(result.status.code)&&operation.attempt<this.options.maximumAttempts)this.requeue(operation,result.status.message);
   else this.finish(operation,{status:result.status,actionResult:null,cached:false,worker:name});
   return true;
  }
  if(bytes===null){this.finish(operation,{status:{code:13,message:"Worker reported success without a result"},actionResult:null,cached:false,worker:name});return true;}
  let cacheable=false;
  try{cacheable=decodeProtobufV1(REAPI_V1.actionResult,bytes).exit_code===0;}catch{this.finish(operation,{status:{code:13,message:"Worker returned a malformed result"},actionResult:null,cached:false,worker:name});return true;}
  const finish=():void=>{this.finish(operation,{status:OK,actionResult:bytes,cached:false,worker:name});};
  if(cacheable)void this.options.content.writeActionResult(operation.instance,operation.action.hash,bytes).catch(()=>{}).finally(finish);else finish();
  return true;
 }

 /** 🧹️ Applies timeouts: dead workers, tasks beyond their deadline, actions no worker can ever run. */
 sweep():void{
  const now=this.clock(),lease=this.options.leaseSeconds*1000;
  for(const worker of [...this.workers.values()])if(!worker.waiter&&now-worker.lastSeen>lease||worker.waiter&&now-worker.lastSeen>lease*3)this.drop(worker,"worker-lost");
  for(const operation of this.operations.values()){
   if(operation.stage==="EXECUTING"&&operation.startedAt!==undefined&&now-operation.startedAt>(operation.timeoutSeconds*1000+lease)){
    const worker=operation.worker?this.workers.get(operation.worker):undefined;worker?.cancel.add(`${operation.id}.${operation.attempt}`);
    this.finish(operation,{status:{code:4,message:`Action exceeded its timeout of ${operation.timeoutSeconds}s`},actionResult:null,cached:false,worker:worker?.registration.name??""});
   }
   if(operation.stage==="QUEUED"&&now-operation.queuedAt>this.options.unmatchedSeconds*1000&&![...this.workers.values()].some(worker=>remoteExecutionPlatformMatchesV1(operation.platform,worker.registration.properties))){
    this.emit("unmatched",`${this.label(operation)}: ${remoteExecutionPlatformKeyV1(operation.platform)}`);
    this.finish(operation,{status:{code:9,message:`No worker offers the platform ${remoteExecutionPlatformKeyV1(operation.platform)||"(any)"}; ${this.workers.size} worker(s) are registered`},actionResult:null,cached:false,worker:""});
   }
  }
 }

 /** 📊️ A secret-free view of the scheduler for `GET /status`. */
 status():Readonly<{counters:Record<string,number>;workers:readonly Record<string,unknown>[];platforms:readonly {key:string;queued:number;workers:number}[];queued:number;running:number}>{
  const now=this.clock(),platforms=new Map<string,{key:string;queued:number;workers:number}>();
  const row=(key:string)=>{let found=platforms.get(key);if(!found){found={key,queued:0,workers:0};platforms.set(key,found);}return found;};
  for(const operation of this.queue)row(remoteExecutionPlatformKeyV1(operation.platform)).queued++;
  const workers=[...this.workers.values()].map(worker=>{
   for(const key of new Set([...platforms.keys()].filter(key=>remoteExecutionPlatformMatchesV1(key===""?[]:key.split(";").map(part=>({name:part.slice(0,part.indexOf("=")),value:part.slice(part.indexOf("=")+1)})),worker.registration.properties))))row(key).workers++;
   return {id:worker.id,name:worker.registration.name,host:worker.registration.host,platform:worker.registration.platform,properties:worker.registration.properties,concurrency:worker.registration.concurrency,running:worker.running.size,sandbox:worker.registration.sandbox,lastSeenSeconds:Math.max(0,Math.round((now-worker.lastSeen)/1000)),draining:worker.draining};
  });
  let running=0;for(const worker of this.workers.values())running+=worker.running.size;
  return {counters:{...this.counters,queued:this.queue.length,executing:running},workers,platforms:[...platforms.values()],queued:this.queue.length,running};
 }

 /** 🛑️ Stops timers and releases every waiting worker. */
 close():void{
  if(this.timer)clearInterval(this.timer);
  for(const worker of this.workers.values()){worker.waiter?.({version:1,task:null,cancel:[],known:false});delete worker.waiter;}
 }
}
