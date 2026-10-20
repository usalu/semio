import {randomBytes} from "node:crypto";
import {mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {homedir,uptime} from "node:os";
import {isAbsolute,join} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../🔒️leases/🎛️control/🟦️.ts";
import {describeHostReasonV1,evaluateHostCapacityV1,formatHostCapacityV1,hostCapacityErrorV1,isCurrentHostGrantV1,isHostCapacityV1,planHostCapacityV1,requireHostCapacityV1,resolveHostCapacityLanguageV1,softHostReasonV1,type HostCapacityPolicyV1,type HostEvaluationV1,type HostGrantV1,type HostPlanV1,type HostReasonV1,type HostRequestV1,type HostSampleV1,type HostWaiterV1} from "../📏️decision/🟦️.ts";
import {currentHostProbeV1,observeTreeMegabytesV1,readHostSampleV1,type HostSampleRecordV1} from "../📡️sampling/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const;

/** 🗂️ The machine-wide gate directory: one per user and shared by every checkout, so the whole machine is one admission domain. */
export function hostCapacityDirectoryV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const override=environment[protocol.directoryEnvironment];
 if(override)return requireMaterializedPathV1(override);
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):isAbsolute(environment.XDG_CACHE_HOME??"")?environment.XDG_CACHE_HOME!:join(home,".cache");
 return requireMaterializedPathV1(join(base,"semio","host-capacity"));
}

/** 🆔️ Mints one unguessable request id. */
export function newHostCapacityIdV1():string{return randomBytes(16).toString("base64url");}

/** 🎟️ What an admitted operation holds and the environment its children inherit. */
export type HostCapacityHandleV1=Readonly<{id:string;jobs:number;cpu:number;reservedMegabytes:number;plan:HostPlanV1;inherited:boolean;bypassed:boolean;environment:Readonly<Record<string,string>>}>;

/** 🔌️ The ports of one gate: time, process identity, host samples and process-tree observation; tests replace them. */
export type HostCapacityPortsV1=Readonly<{now?:()=>number;pid?:number;sample?:(previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>)=>Promise<HostSampleRecordV1>;observe?:(pid:number)=>Promise<number|null>}>;
/** ⚙️ Everything one admission needs. */
export type HostCapacityOptionsV1=HostCapacityPortsV1&Readonly<{directory:string;policy:HostCapacityPolicyV1;request:HostRequestV1;control:ResourceLeaseControl;environment?:Readonly<Record<string,string|undefined>>;write?:(line:string)=>void}>;
/** 📋️ What the gate currently knows. */
export type HostCapacityStatusV1=Readonly<{host:HostSampleV1;grants:readonly HostGrantV1[];waiters:readonly HostWaiterV1[];evaluation:HostEvaluationV1}>;

/** 📥️ Reads the plan an enclosing grant carried into the environment, or undefined when the process runs under none. */
export function readHostCapacityPlanV1(environment:Readonly<Record<string,string|undefined>>):HostPlanV1|undefined{
 const text=environment[protocol.planEnvironment];
 if(text===undefined||text==="")return undefined;
 let value:unknown;try{value=JSON.parse(text);}catch{throw hostCapacityErrorV1("planRequired",{},environment);}
 return requireHostCapacityV1<HostPlanV1>("Plan",value);
}

const bootAt=(now:number)=>Math.round((now-uptime()*1000)/1000)*1000;
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const readJson=(path:string):unknown=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch{return undefined;}};
const remove=(path:string):void=>{try{unlinkSync(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}};
const modified=(path:string):number=>{try{return statSync(path).mtimeMs;}catch{return 0;}};
const names=(directory:string,child:string):string[]=>readdirSync(join(directory,child)).filter(file=>file.endsWith(".json"));
const grantPath=(directory:string,id:string)=>join(directory,"grants",`${id}.json`);
const queuePath=(directory:string,arrivedAtMs:number,id:string)=>join(directory,"queue",`${String(arrivedAtMs).padStart(15,"0")}-${id}.json`);

function writeJson(path:string,value:unknown):void{
 const text=JSON.stringify(value);if(text.length>protocol.maximumFileBytes)throw hostCapacityErrorV1("recordTooLarge");
 const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,text);renameSync(temporary,path);
}

function prepare(directory:string):string{
 const root=requireMaterializedPathV1(directory);for(const name of ["grants","queue"])mkdirSync(join(root,name),{recursive:true,mode:0o700});
 return root;
}

function readGrants(directory:string,now:number,policy:HostCapacityPolicyV1):HostGrantV1[]{
 const rows:HostGrantV1[]=[],boot=bootAt(now);
 for(const name of names(directory,"grants")){
  const path=join(directory,"grants",name),value=readJson(path);
  if(!isHostCapacityV1("Grant",value)){if(now-modified(path)>policy.wait.lockStaleMilliseconds)remove(path);continue;}
  const grant=value as HostGrantV1;
  if(!isCurrentHostGrantV1({grant,expectedId:name.slice(0,-5),pidAlive:alive(grant.pid),nowMs:now,bootAtMs:boot,modifiedAtMs:modified(path)},policy))remove(path);else rows.push(grant);
 }
 return rows;
}

function readWaiters(directory:string,now:number,policy:HostCapacityPolicyV1):HostWaiterV1[]{
 const rows:HostWaiterV1[]=[],stale=policy.wait.pollMilliseconds*30;
 for(const name of names(directory,"queue")){
  const path=join(directory,"queue",name),value=readJson(path);
  if(!isHostCapacityV1("Waiter",value)){if(now-modified(path)>policy.wait.lockStaleMilliseconds)remove(path);continue;}
  const waiter=value as HostWaiterV1;
  if(!alive(waiter.pid)||now-modified(path)>stale)remove(path);else rows.push(waiter);
 }
 return rows;
}

function takeLock(directory:string,now:number,policy:HostCapacityPolicyV1,pid:number):boolean{
 const lock=join(directory,"lock");
 try{mkdirSync(lock,{mode:0o700});writeFileSync(join(lock,"owner"),`${pid}:${now}`);return true;}
 catch(error){
  if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
  let text="";try{text=readFileSync(join(lock,"owner"),"utf8");}catch{}
  const owner=/^(\d+):(\d+)$/u.exec(text),since=owner?Number(owner[2]):modified(lock);
  if(owner&&!alive(Number(owner[1]))||now-since>policy.wait.lockStaleMilliseconds)rmSync(lock,{recursive:true,force:true});
  return false;
 }
}

async function currentSample(directory:string,policy:HostCapacityPolicyV1,control:ResourceLeaseControl,ports:HostCapacityPortsV1):Promise<HostSampleV1>{
 const now=ports.now??Date.now,path=join(directory,"sample.json"),prior=readJson(path) as HostSampleRecordV1|undefined,cached=isHostCapacityV1("Host",prior?.host)?prior!.host:undefined;
 if(cached&&now()>=cached.atMs&&now()-cached.atMs<=policy.wait.sampleMaximumAgeMilliseconds)return cached;
 const previous=cached?{atMs:cached.atMs,swappedOutMegabytes:prior!.swappedOutMegabytes}:undefined,record=await(ports.sample?ports.sample(previous):readHostSampleV1(policy,currentHostProbeV1(control.signal),previous,now));
 requireHostCapacityV1<HostSampleV1>("Host",record.host);
 try{writeJson(path,record);}catch{}
 return record.host;
}

function waitingLine(language:"en"|"de",request:HostRequestV1,host:HostSampleV1,evaluation:HostEvaluationV1,running:number,seconds:number):string{
 const cap=Math.max(0,host.totalMegabytes-evaluation.floorMegabytes),position=evaluation.decisions.findIndex(row=>row.id===request.id),mine=evaluation.decisions[position]!,before=evaluation.decisions.slice(0,position),spare=evaluation.headroomMegabytes-before.reduce((sum,row)=>sum+row.reservedMegabytes,0);
 return formatHostCapacityV1(language,"waiting",{seconds,operation:request.operation,reason:describeHostReasonV1(language,mine.reason as Exclude<HostReasonV1,"fits"|"reduced"|"override">),need:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,cap),spare,floor:evaluation.floorMegabytes,load:host.load===null?"n/a":host.load,running,ahead:position});
}

/** 🚪️ Admits one operation against the whole machine and runs its body under the grant: it waits in a fair machine-wide queue with progress lines and cancellation, reserves memory and cores while the body runs and always gives them back. */
export async function withHostCapacityV1<T>(options:HostCapacityOptionsV1,body:(handle:HostCapacityHandleV1)=>Promise<T>):Promise<T>{
 const environment=options.environment??process.env,policy=options.policy,request=requireHostCapacityV1<HostRequestV1>("Request",options.request),language=resolveHostCapacityLanguageV1(environment),write=options.write??(line=>process.stderr.write(line+"\n")),now=options.now??Date.now,pid=options.pid??process.pid;
 if(!policy.enabled||environment[protocol.switchEnvironment]==="off"){
  if(policy.enabled)write(formatHostCapacityV1(language,"bypassed",{switch:protocol.switchEnvironment}));
  const plan=planHostCapacityV1(policy,{totalMegabytes:1048576},request,request.preferredJobs);
  return body(Object.freeze({id:request.id,jobs:plan.jobs,cpu:plan.cpu,reservedMegabytes:0,plan,inherited:false,bypassed:true,environment:Object.freeze({})}));
 }
 const inheritedId=environment[protocol.grantEnvironment];
 if(inheritedId!==undefined&&!new RegExp(schema.definitions.Grant.properties.id.pattern,"u").test(inheritedId))throw hostCapacityErrorV1("inheritedIdentifier",{},environment);
 const directory=prepare(options.directory),inheritedPath=inheritedId?grantPath(directory,inheritedId):undefined,inherited=inheritedPath?readJson(inheritedPath):undefined,at=now();
 if(inheritedPath&&isCurrentHostGrantV1({grant:inherited,expectedId:inheritedId,pidAlive:isHostCapacityV1("Grant",inherited)&&alive((inherited as HostGrantV1).pid),nowMs:at,bootAtMs:bootAt(at),modifiedAtMs:modified(inheritedPath)},policy)){
  const grant=inherited as HostGrantV1,carried=readHostCapacityPlanV1(environment),plan=carried?.id===grant.id?carried:requireHostCapacityV1<HostPlanV1>("Plan",{version:1,id:grant.id,jobs:grant.jobs,cpu:grant.cpu,memoryMegabytes:grant.reservedMegabytes,fixedMegabytes:0,floorMegabytes:0});
  return body(Object.freeze({id:grant.id,jobs:grant.jobs,cpu:grant.cpu,reservedMegabytes:grant.reservedMegabytes,plan,inherited:true,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:grant.id,[protocol.planEnvironment]:JSON.stringify(plan)})}));
 }
 if(names(directory,"queue").length>=policy.wait.maximumWaiters)throw Error(schema.definitions.Messages.const[language].errors.queueFull);
 const arrivedAtMs=now(),waiterPath=queuePath(directory,arrivedAtMs,request.id),record:HostWaiterV1={request,arrivedAtMs,pid};
 writeJson(waiterPath,record);
 let grant:HostGrantV1|undefined,admittedHost:HostSampleV1|undefined,lastProgress=-Infinity;
 try{
  while(!grant){
   checkResourceLeaseControl(options.control);
   const host=await currentSample(directory,policy,options.control,options);
   try{utimesSync(waiterPath,new Date(),new Date());}catch{}
   if(takeLock(directory,now(),policy,pid)){
    try{
     const at=now(),ledger=readGrants(directory,at,policy),waiters=readWaiters(directory,at,policy);
     if(!waiters.some(row=>row.request.id===request.id)){writeJson(waiterPath,record);waiters.push(record);}
     const evaluation=evaluateHostCapacityV1(policy,host,ledger,waiters,at),mine=evaluation.decisions.find(row=>row.id===request.id)!;
     if(mine.verdict==="admit"){
      grant=requireHostCapacityV1<HostGrantV1>("Grant",{id:request.id,operation:request.operation,jobs:mine.jobs,cpu:mine.jobs*request.cpuPerJob,reservedMegabytes:mine.reservedMegabytes,admittedAtMs:at,observedMegabytes:null,pid,bootAtMs:bootAt(at)});
      writeJson(grantPath(directory,request.id),grant);remove(waiterPath);admittedHost=host;
      const seconds=Math.round((at-arrivedAtMs)/1000);
      if(mine.reason==="override")write(formatHostCapacityV1(language,"override",{operation:request.operation,seconds,reason:describeHostReasonV1(language,softHostReasonV1(policy,host)??"load")}));
      else if(mine.reason==="reduced")write(formatHostCapacityV1(language,"reduced",{operation:request.operation,jobs:mine.jobs,preferred:request.preferredJobs}));
      else write(formatHostCapacityV1(language,"admitted",{operation:request.operation,jobs:mine.jobs,reserved:mine.reservedMegabytes,available:host.availableMegabytes,floor:evaluation.floorMegabytes}));
     }else if(at-lastProgress>=policy.wait.progressMilliseconds){
      lastProgress=at;write(waitingLine(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000)));
      await options.control.publish({resource:protocol.resource,mode:"shared",elapsedMs:at-arrivedAtMs});checkResourceLeaseControl(options.control);await options.control.yieldContinuation();
     }
    }finally{rmSync(join(directory,"lock"),{recursive:true,force:true});}
   }
   if(grant)break;
   const budget=checkResourceLeaseControl(options.control);
   try{await delay(Math.max(1,Math.min(policy.wait.pollMilliseconds*(0.75+Math.random()*0.5),budget??Infinity)),undefined,{signal:options.control.signal});}catch(error){throw options.control.signal.aborted?options.control.signal.reason:error;}
  }
 }catch(error){remove(waiterPath);throw error;}
 const held=grant!,path=grantPath(directory,held.id),plan=planHostCapacityV1(policy,admittedHost!,request,held.jobs);
 let holding=true;
 const cleanup=()=>{holding=false;try{unlinkSync(path);}catch{}};
 process.once("exit",cleanup);
 const timer=setInterval(()=>{void(options.observe??observeTreeMegabytesV1)(pid).then(observed=>{if(holding&&observed!==null)writeJson(path,{...held,observedMegabytes:observed});}).catch(()=>undefined);},policy.wait.heartbeatMilliseconds);
 timer.unref();
 try{
  return await body(Object.freeze({id:held.id,jobs:held.jobs,cpu:held.cpu,reservedMegabytes:held.reservedMegabytes,plan,inherited:false,bypassed:false,environment:Object.freeze({[protocol.grantEnvironment]:held.id,[protocol.planEnvironment]:JSON.stringify(plan)})}));
 }finally{clearInterval(timer);process.removeListener("exit",cleanup);cleanup();}
}

/** 📋️ Reads the gate without joining its queue: the sample, the live grants, the waiters and what the next request would get. */
export async function readHostCapacityStatusV1(options:HostCapacityPortsV1&Readonly<{directory:string;policy:HostCapacityPolicyV1;control:ResourceLeaseControl}>):Promise<HostCapacityStatusV1>{
 const now=options.now??Date.now,directory=prepare(options.directory),host=await currentSample(directory,options.policy,options.control,options),grants=readGrants(directory,now(),options.policy),waiters=readWaiters(directory,now(),options.policy);
 return Object.freeze({host,grants,waiters,evaluation:evaluateHostCapacityV1(options.policy,host,grants,waiters,now())});
}
