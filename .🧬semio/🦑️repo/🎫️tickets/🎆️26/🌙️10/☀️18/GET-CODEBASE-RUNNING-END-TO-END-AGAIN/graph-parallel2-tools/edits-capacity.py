L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
T = L + "/🏗️build/🌡️host-capacity/🟦️.ts"
new = '''/** 🏊️ Asks the machine-wide gate for the worker processes of the graph inference farm beside the grant the invocation already holds: a fresh request in the fair queue, answered when the gate admits it, never given up on a timer. `granted` resolves with the jobs and the megabytes each of them may hold (undefined when the wait was cancelled); `release` cancels a wait or gives the grant back, and the grant is held until then. A gate that is off grants every preferred job. */
export function requestBuildFarmCapacityV1(input:HostCapacityPortsV1&Readonly<{preferredJobs:number;environment?:NodeJS.ProcessEnv;directory?:string;policy?:HostCapacityPolicyV1;write?:(line:string)=>void;signal?:AbortSignal}>):Readonly<{granted:Promise<Readonly<{jobs:number;megabytes:number}>|undefined>;release:()=>void}>{
 const policy=input.policy??BUILD_HOST_CAPACITY_POLICY_V1,environment:NodeJS.ProcessEnv={...(input.environment??process.env)};
 delete environment["SEMIO_HOST_CAPACITY_GRANT"];delete environment["SEMIO_HOST_CAPACITY_PLAN"];
 const request=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:"exec",preferredJobs:input.preferredJobs,kinds:["build"]}),megabytes=request?.perJobMegabytes??policy.kinds.build.megabytes;
 if(!policy.enabled||environment["SEMIO_HOST_CAPACITY"]==="off"||!request)return Object.freeze({granted:Promise.resolve(Object.freeze({jobs:input.preferredJobs,megabytes})),release:()=>{}});
 const controller=new AbortController(),control:ResourceLeaseControl=Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))});
 let release=()=>{},admitted=false;const held=new Promise<void>(done=>{release=done;});
 const cancel=()=>{if(!admitted)controller.abort(Error("Farm capacity wait cancelled"));release();};
 if(input.signal){if(input.signal.aborted)cancel();else input.signal.addEventListener("abort",cancel,{once:true});}
 const granted=new Promise<Readonly<{jobs:number;megabytes:number}>|undefined>(accept=>{
  withHostCapacityV1({...input,directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment},async handle=>{admitted=true;accept(Object.freeze({jobs:handle.jobs,megabytes:Math.floor(handle.plan.memoryMegabytes/Math.max(1,handle.jobs))||megabytes}));await held;}).then(()=>accept(undefined),()=>accept(undefined));
 });
 return Object.freeze({granted,release:cancel});
}

'''
anchor = '/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy:'
EDITS = [(T, anchor, new + anchor)]
