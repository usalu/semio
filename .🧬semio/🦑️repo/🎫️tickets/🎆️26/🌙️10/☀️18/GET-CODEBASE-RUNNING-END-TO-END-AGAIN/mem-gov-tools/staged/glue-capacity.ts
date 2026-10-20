import buildPolicy from "../🔣️policy.json" with {type:"json"};
import {hostCapacityDirectoryV1,newHostCapacityIdV1,readHostCapacityPolicyV1,sizeHostRequestV1,withHostCapacityV1,type HostActionKindV1,type HostCapacityHandleV1,type HostCapacityPolicyV1,type HostCapacityPortsV1,type HostRequestV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {availableParallelism} from "node:os";
import {currentWorkerHostV1,planWorkersV1,readWorkerPolicyV1,workerActionKindV1} from "../👷️workers/🟦️.ts";

/** 🌡️ The authored host capacity policy of the neutral build policy. */
export const BUILD_HOST_CAPACITY_POLICY_V1:HostCapacityPolicyV1=readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity);

/** 🧭️ What an invocation asks the host for: the gated operation, the jobs it would like and the action kinds it will run. */
export type BuildCapacityAskV1=Readonly<{operation:string;preferredJobs:number;kinds:readonly HostActionKindV1[]}>;

/** 🔎️ Reads the last value of one option before the argument separator. */
function option(args:readonly string[],name:string,short?:string):string|undefined{
 let last:string|undefined;
 for(let index=0;index<args.length&&args[index]!=="--";index++){
  const argument=args[index]!;
  if(argument===`--${name}`||short!==undefined&&argument===`-${short}`){const value=args[index+1];if(value!==undefined&&!value.startsWith("-")){last=value;index++;}}
  else if(argument.startsWith(`--${name}=`)||short!==undefined&&argument.startsWith(`-${short}=`))last=argument.slice(argument.indexOf("=")+1);
 }
 return last;
}

/** 🏷️ Classifies the targets an invocation names into action kinds by the worker pool's rules. */
function requestedKinds(args:readonly string[]):HostActionKindV1[]{
 const worker=readWorkerPolicyV1((buildPolicy as unknown as {workers:unknown}).workers),named=args[0]==="run"?[args[1]?.split(":")[1]]:(option(args,"targets","t")??option(args,"target")??"").split(",");
 return named.filter((target):target is string=>typeof target==="string"&&target.length>0).map(target=>workerActionKindV1(worker,target,false));
}

/** 🎯️ Reads which operation, how many jobs and which kinds an already tool-stripped argument list asks for; answers undefined for light commands. */
export function buildCapacityAskV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>,policy:HostCapacityPolicyV1=BUILD_HOST_CAPACITY_POLICY_V1):BuildCapacityAskV1|undefined{
 const command=args[0];
 if(command===undefined||!policy.operations.some(row=>row.name===command))return undefined;
 const head=args.slice(0,args.indexOf("--")<0?undefined:args.indexOf("--"));
 if(head.some(argument=>argument==="--help"||argument==="-h"))return undefined;
 const requested=command==="cargo"?option(head,"parallelism"):option(head,"parallel"),fromEnvironment=command==="cargo"?undefined:environment.SEMIO_BUILD_PARALLEL,parsed=Number(requested??fromEnvironment);
 const preferred=Number.isSafeInteger(parsed)&&parsed>0?parsed:command==="cargo"?availableParallelism():planWorkersV1(readWorkerPolicyV1((buildPolicy as unknown as {workers:unknown}).workers),currentWorkerHostV1()).localJobs;
 return Object.freeze({operation:command,preferredJobs:preferred,kinds:Object.freeze(command==="cargo"?[]:requestedKinds(head))});
}

/** ✍️ Rewrites the parallelism option of an invocation to the granted jobs so a tool can never exceed its grant. */
export function grantedBuildArgumentsV1(args:readonly string[],operation:string,jobs:number):string[]{
 const name=operation==="cargo"?"--parallelism":"--parallel",boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary),tail=boundary<0?[]:args.slice(boundary),kept:string[]=[];
 for(let index=0;index<head.length;index++){const argument=head[index]!;if(argument===name){index++;continue;}if(argument.startsWith(name+"="))continue;kept.push(argument);}
 return [...kept,`${name}=${jobs}`,...tail];
}

/** 🛑️ A control for the wait before any invocation exists: it is cancelled by an interrupt or termination signal and has no deadline of its own, so the invocation's budget starts when the grant does. */
export function createCapacityControlV1(process_:Pick<NodeJS.Process,"once"|"removeListener">=process):Readonly<{control:ResourceLeaseControl;dispose:()=>void}>{
 const controller=new AbortController(),handlers=(["SIGINT","SIGTERM"] as const).map(name=>[name,()=>controller.abort(Error(`Host capacity wait cancelled by ${name}`))] as const);
 for(const [name,handler] of handlers)process_.once(name,handler);
 return Object.freeze({control:Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))}),dispose:()=>{for(const [name,handler] of handlers)process_.removeListener(name,handler);}});
}

/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy: waits fairly with progress lines, then hands the body the arguments rewritten to the granted jobs and sets the environment the children inherit; light commands pass untouched. */
export async function withBuildCapacityV1<T>(args:readonly string[],environment:NodeJS.ProcessEnv,body:(granted:Readonly<{args:readonly string[];handle:HostCapacityHandleV1|undefined}>)=>Promise<T>,ports:HostCapacityPortsV1&Readonly<{directory?:string;policy?:HostCapacityPolicyV1;write?:(line:string)=>void;control?:ResourceLeaseControl;strip?:(args:readonly string[],environment:NodeJS.ProcessEnv)=>readonly string[]}>={}):Promise<T>{
 const policy=ports.policy??BUILD_HOST_CAPACITY_POLICY_V1,stripped=ports.strip?ports.strip(args,environment):(await import("../🧭️invocation/🟦️.ts")).parseBuildInvocationV1(args,environment).args,ask=buildCapacityAskV1(stripped,environment,policy);
 if(!ask)return body({args,handle:undefined});
 const request:HostRequestV1|undefined=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds});
 if(!request)return body({args,handle:undefined});
 const waiting=ports.control?undefined:createCapacityControlV1(),tasks=request.perJobMegabytes>0;
 try{
  return await withHostCapacityV1({...ports,directory:ports.directory??hostCapacityDirectoryV1(environment),policy,request,control:ports.control??waiting!.control,environment},async handle=>{
   waiting?.dispose();
   const previous=Object.fromEntries([...Object.keys(handle.environment),"SEMIO_BUILD_PARALLEL"].map(key=>[key,environment[key]]));
   Object.assign(environment,handle.environment,tasks?{SEMIO_BUILD_PARALLEL:String(handle.jobs)}:{});
   try{return await body({args:tasks?grantedBuildArgumentsV1(args,ask.operation,handle.jobs):args,handle});}
   finally{for(const [key,value] of Object.entries(previous))if(value===undefined)delete environment[key];else environment[key]=value;}
  });
 }finally{waiting?.dispose();}
}
