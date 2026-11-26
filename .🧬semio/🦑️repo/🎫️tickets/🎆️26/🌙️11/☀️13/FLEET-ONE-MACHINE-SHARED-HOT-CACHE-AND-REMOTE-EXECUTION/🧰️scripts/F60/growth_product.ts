/** 📈️ The memory facts of one machine for grant growth: the total, the free-memory floor and interactive reserve of the host-capacity policy at that total, the share of the rest reservations may take and, for this machine, the memory available now.
 * @see hostHoldableMegabytesV1 */
export function growthHostV1(policy:HostCapacityPolicyV1,facts:Readonly<{origin:string;totalMegabytes:number;availableMegabytes?:number}>):GrowthHostV1{
 const host={totalMegabytes:facts.totalMegabytes};
 return Object.freeze({origin:facts.origin,totalMegabytes:facts.totalMegabytes,floorMegabytes:hostFloorMegabytesV1(policy,host),interactiveReserveMegabytes:hostInteractiveReserveMegabytesV1(policy,host),budgetBasisPoints:policy.memory.budgetBasisPoints,...facts.availableMegabytes===undefined?{}:{availableMegabytes:facts.availableMegabytes}});
}

/** 📈️ The machines of the fleet as the warden of each mirrors them to the group store (`<store>/v1/origins/<origin>/memory.json`, host total and time of the sample): only mirrors younger than `maximumAgeMilliseconds` count; `undefined` when the store is unknown or unreadable, so a caller never mistakes an unknown fleet for a small one. Never throws.
 * @see ../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json (`Policy.growth`) */
export function readFleetGrowthHostsV1(policy:HostCapacityPolicyV1,environment:Readonly<Record<string,string|undefined>>,nowMs:number=Date.now(),maximumAgeMilliseconds:number=ADMISSION_POLICY_V1.growth.fleetStalenessMilliseconds):readonly GrowthHostV1[]|undefined{
 const store=environment["SEMIO_FLEET_STORE_ROOT"];
 if(!store)return undefined;
 try{
  const base=join(store,"v1",ADMISSION_POLICY_V1.growth.fleetDirectory),hosts:GrowthHostV1[]=[];
  for(const origin of readdirSync(base)){
   try{
    const row=JSON.parse(readFileSync(join(base,origin,ADMISSION_POLICY_V1.growth.fleetFile),"utf8")) as {atMs?:unknown;host?:{totalMegabytes?:unknown}};
    const total=row.host?.totalMegabytes;
    if(typeof total==="number"&&Number.isFinite(total)&&total>0&&typeof row.atMs==="number"&&nowMs-row.atMs<=maximumAgeMilliseconds)hosts.push(growthHostV1(policy,{origin,totalMegabytes:Math.floor(total)}));
   }catch{continue;}
  }
  return Object.freeze(hosts);
 }catch{return undefined;}
}

/** 📈️ The port through which the admission broker grows a parent memory grant: this machine's facts from the live sample, the fleet from the group store, and `acquire`, a supplementary reservation of the difference requested of the machine-wide gate in the fair queue beside the grant the invocation holds (never the grant row itself: its holder keeps its own books), so every other run, the warden thresholds and the interactive reserve see it. The wait ends after `waitMilliseconds`, when the signal aborts or when the gate refuses; the booking is held until `release` and a gate that is off books nothing and grants at once.
 * @see ../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts (`startAdmissionBrokerV1`) */
export function createGrantGrowthPortV1(input:HostCapacityPortsV1&Readonly<{environment?:NodeJS.ProcessEnv;directory?:string;policy?:HostCapacityPolicyV1;origin?:string}>={}):GrantGrowthPortV1{
 const policy=input.policy??BUILD_HOST_CAPACITY_POLICY_V1,origin=input.origin??"local";
 return Object.freeze({
  local:()=>{const available=hostAvailableBytesV1();return growthHostV1(policy,{origin,totalMegabytes:Math.floor(totalmem()/1048576),...available===undefined?{}:{availableMegabytes:Math.floor(available/1048576)}});},
  fleet:()=>readFleetGrowthHostsV1(policy,input.environment??process.env),
  acquire:ask=>new Promise(accept=>{
   const environment:NodeJS.ProcessEnv={...(input.environment??process.env)};delete environment["SEMIO_HOST_CAPACITY_GRANT"];delete environment["SEMIO_HOST_CAPACITY_PLAN"];
   if(!policy.enabled||environment["SEMIO_HOST_CAPACITY"]==="off"){accept(Object.freeze({granted:true as const,release:()=>{}}));return;}
   const request:HostRequestV1=Object.freeze({id:newHostCapacityIdV1(),operation:"cargo-growth",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:Math.max(1,Math.ceil(ask.extraMegabytes)),perJobMegabytes:0,cpuPerJob:1,label:"unit-growth"}),session=currentAgentSessionV1({environment});
   const controller=new AbortController(),control:ResourceLeaseControl=Object.freeze({signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))});
   let release=()=>{},admitted=false,timedOut=false;const held=new Promise<void>(done=>{release=done;});
   const stop=()=>{if(!admitted)controller.abort(refuseCapacityV1("cancelled","unit-growth-cancelled"));release();};
   const timer=setTimeout(()=>{timedOut=true;stop();},ask.waitMilliseconds);timer.unref?.();
   if(ask.signal.aborted)stop();else ask.signal.addEventListener("abort",stop,{once:true});
   const leave=()=>{clearTimeout(timer);ask.signal.removeEventListener("abort",stop);};
   const refused=()=>{leave();accept(Object.freeze({granted:false as const,reason:timedOut?"timeout" as const:"cancelled" as const}));};
   withHostCapacityV1({observe:()=>Promise.resolve(0),...input,directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment,write:ask.write,session:{address:agentSessionAddressV1(session),weight:session.budget.weight}},async handle=>{admitted=true;clearTimeout(timer);accept(Object.freeze({granted:true as const,release:()=>{leave();release();}}));void handle;await held;}).then(()=>{if(!admitted)refused();},()=>{if(!admitted)refused();else leave();});
  })
 });
}

