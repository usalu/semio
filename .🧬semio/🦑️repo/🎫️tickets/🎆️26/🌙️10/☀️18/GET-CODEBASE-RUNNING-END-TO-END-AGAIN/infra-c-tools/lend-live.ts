import {join} from "node:path";
const root=process.cwd(),lib=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),core=join(root,"🧰️framework/🔨️modules/🏃️process");
const capacity=await import(join(lib,"🌡️host-capacity/🟦️.ts")),gate=await import(join(core,"🏗️build/🌡️host-capacity/🟦️.ts")),dog=await import(join(core,"🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts"));
const mode=process.argv[2]??"waiting",seconds=Number(process.argv[3]??"90"),directory=process.argv[4]??gate.hostCapacityDirectoryV1(process.env),base=capacity.BUILD_HOST_CAPACITY_POLICY_V1,policy=process.argv[4]?gate.readHostCapacityPolicyV1({...base,memory:{...base.memory,floorBasisPoints:0,floorMinimumMegabytes:0,floorMaximumMegabytes:0}}):base,log=(line:string)=>console.log(`${new Date().toISOString().slice(11,19)} ${line}`);
const probe=capacity.createCapacityControlV1();
try{
 const request={id:gate.newHostCapacityIdV1(),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:2048,perJobMegabytes:0,cpuPerJob:1};
 log(`asking the gate ${directory} for a 2048 MB / 1 core grant (pid ${process.pid}, mode ${mode})`);
 await gate.withHostCapacityV1({directory,policy,request,control:probe.control,environment:process.env,session:undefined,write:log},async handle=>{
  log(`granted ${handle.id}; now ${mode==="waiting"?"only waiting in a declared queue wait":"working on a plain timer"} for ${seconds} s`);
  const end=Date.now()+seconds*1000;
  while(Date.now()<end)await (mode==="waiting"?dog.waitingDelayV1(250):new Promise(done=>setTimeout(done,250)));
  log("done; leaving the grant");
 });
}finally{probe.dispose();}
