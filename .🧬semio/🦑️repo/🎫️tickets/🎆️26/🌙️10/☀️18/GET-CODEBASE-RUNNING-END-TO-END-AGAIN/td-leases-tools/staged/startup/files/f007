import {spawn} from "node:child_process";
import {formatHostCapacityV1,hostCapacityDirectoryV1,newHostCapacityIdV1,readHostCapacityStatusV1,readHostEstimateStoreV1,requireHostCapacityV1,resolveHostCapacityLanguageV1,sizeHostRequestV1,withHostCapacityV1,type HostCapacityPolicyV1,type HostCapacityPortsV1,type HostRequestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {agentSessionAddressV1,agentSessionCapacityRowsV1,agentSessionRegistryRootV1,currentAgentSessionV1,renderAgentSessionCapacityV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../🎛️control/🟦️.ts";
import {writeDiagnosticLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

/** 🔌️ What the capacity command touches: the result writer, environment, directory and policy; tests replace them. The waiting lines of `capacity run` always go to the diagnostic stream. */
export type CapacityCommandContextV1=HostCapacityPortsV1&Readonly<{environment:NodeJS.ProcessEnv;write:(line:string)=>void;directory?:string;policy?:HostCapacityPolicyV1}>;

const flag=(head:readonly string[],name:string):string|undefined=>{
 for(let index=0;index<head.length;index++){const argument=head[index]!;if(argument===`--${name}`)return head[index+1];if(argument.startsWith(`--${name}=`))return argument.slice(name.length+3);}
 return undefined;
};
const positive=(value:string|undefined,name:string):number|undefined=>{
 if(value===undefined)return undefined;
 const parsed=Number(value);if(!Number.isSafeInteger(parsed)||parsed<1)throw Error(`Capacity option --${name} requires a positive integer`);
 return parsed;
};

/** 📋️ Prints the machine's capacity: the sample, the floor, every running grant and every waiter, as lines or as one JSON document. */
async function status(head:readonly string[],context:CapacityCommandContextV1,policy:HostCapacityPolicyV1,directory:string):Promise<number>{
 const control=createCapacityControlV1();
 try{
  const now=context.now??Date.now,snapshot=await readHostCapacityStatusV1({...context,directory,policy,control:control.control});
  const sessions=agentSessionCapacityRowsV1(snapshot,agentSessionRegistryRootV1(context.environment),context.now===undefined?{}:{now:context.now()});
  if(head.includes("--json")){context.write(JSON.stringify({...snapshot,sessions,estimates:readHostEstimateStoreV1(directory)},null,2));return 0;}
  const language=resolveHostCapacityLanguageV1(context.environment),host=snapshot.host;
  context.write(formatHostCapacityV1(language,"status",{available:host.availableMegabytes,total:host.totalMegabytes,floor:snapshot.evaluation.floorMegabytes,spare:snapshot.evaluation.headroomMegabytes,pressure:host.pressure,swap:host.swapUsedMegabytes??"n/a",load:host.load??"n/a",files:host.openFiles??"n/a",images:host.attachedImages??"n/a",grants:snapshot.grants.length,waiters:snapshot.waiters.length}));
  if(host.counters==="partial")context.write(formatHostCapacityV1(language,"countersPartial",{platform:host.platform,reason:language==="de"?"Probe fehlgeschlagen":"probe failed"}));
  for(const grant of snapshot.grants)context.write(formatHostCapacityV1(language,grant.lentSinceMs!==undefined?"grantLent":grant.quiet===true?"grantQuiet":"grant",{id:grant.id,operation:grant.operation,jobs:grant.jobs,cpu:grant.cpu,reserved:grant.reservedMegabytes,observed:grant.observedMegabytes??"n/a",pid:grant.pid,seconds:Math.round((now()-(grant.lentSinceMs??grant.admittedAtMs))/1000)})+(grant.phase===undefined?"":` [${grant.phase}]`));
  for(const waiter of snapshot.waiters)context.write(formatHostCapacityV1(language,waiter.upgrade===true?"waiterUpgrade":"waiter",{id:waiter.request.id,operation:waiter.request.operation,jobs:waiter.request.preferredJobs,need:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes,pid:waiter.pid,seconds:Math.round((now()-waiter.arrivedAtMs)/1000)}));
  for(const line of renderAgentSessionCapacityV1(sessions,language))context.write(line);
  return 0;
 }finally{control.dispose();}
}

/** ▶️ Runs any command under a capacity grant, so ad-hoc heavy work (a test runner, a compiler, a generator) is admitted like every other expensive operation. */
async function run(head:readonly string[],command:readonly string[],context:CapacityCommandContextV1,policy:HostCapacityPolicyV1,directory:string):Promise<number>{
 if(command.length===0)throw Error("Capacity run requires a command after --");
 const memory=positive(flag(head,"memory"),"memory"),cpu=positive(flag(head,"cpu"),"cpu"),jobs=positive(flag(head,"jobs"),"jobs"),operation=flag(head,"operation")??"command";
 const sized:HostRequestV1|undefined=memory!==undefined?requireHostCapacityV1<HostRequestV1>("Request",{id:newHostCapacityIdV1(),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:memory,perJobMegabytes:0,cpuPerJob:cpu??1}):sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation,...jobs!==undefined?{preferredJobs:jobs}:{}});
 if(!sized)throw Error(`Unknown capacity operation: ${operation}`);
 const waiting=createCapacityControlV1();
 try{
  const session=currentAgentSessionV1({environment:context.environment});
  return await withHostCapacityV1({...context,write:writeDiagnosticLineV1,directory,policy,request:sized,control:waiting.control,session:{address:agentSessionAddressV1(session),weight:session.budget.weight}},async handle=>{
   waiting.dispose();
   const child=spawn(command[0]!,command.slice(1),{stdio:"inherit",env:{...context.environment,...handle.environment,...handle.plan.jobs>0&&sized.perJobMegabytes>0?{SEMIO_BUILD_PARALLEL:String(handle.jobs)}:{}}}),forward=(name:NodeJS.Signals)=>()=>{child.kill(name);},handlers=(["SIGINT","SIGTERM"] as const).map(name=>[name,forward(name)] as const);
   for(const [name,handler] of handlers)process.on(name,handler);
   try{return await new Promise<number>((resolve,reject)=>{child.once("error",reject);child.once("exit",(code,signal)=>resolve(code??(signal===null?1:128+(signal==="SIGINT"?2:15))));});}
   finally{for(const [name,handler] of handlers)process.removeListener(name,handler);}
  });
 }finally{waiting.dispose();}
}

/** 🚪️ Dispatches `repo capacity status [--json]` and `repo capacity run [--operation=<name>|--memory=<MB> [--cpu=<n>]] [--jobs=<n>] -- <command…>`. */
export async function runCapacityCommandV1(args:readonly string[],context:CapacityCommandContextV1):Promise<number>{
 const boundary=args.indexOf("--"),head=args.slice(1,boundary<0?undefined:boundary),tail=boundary<0?[]:args.slice(boundary+1),policy=context.policy??BUILD_HOST_CAPACITY_POLICY_V1,directory=context.directory??hostCapacityDirectoryV1(context.environment);
 if(args[0]==="status")return status(head,context,policy,directory);
 if(args[0]==="run")return run(head,tail,context,policy,directory);
 throw Error("Expected repo capacity status|run");
}
