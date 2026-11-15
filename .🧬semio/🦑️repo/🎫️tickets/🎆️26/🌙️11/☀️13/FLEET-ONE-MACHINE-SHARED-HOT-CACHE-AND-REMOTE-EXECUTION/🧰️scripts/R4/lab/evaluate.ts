import{spawnSync}from"node:child_process";
const MiB=1048576,PROC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts",W="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🙋️wanted/";
const roots={before:process.env.R4_LIVE!,after:process.env.R4_OVERLAY!};
const vm=spawnSync("vm_stat",[],{encoding:"utf8"}).stdout;
const synthetic16=(free:number,inactive:number,file:number,purgeable:number,spec:number)=>`Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free: ${free}.\nPages inactive: ${inactive}.\nPages speculative: ${spec}.\nPages purgeable: ${purgeable}.\nFile-backed pages: ${file}.\n`;
const readings:[string,string,number][]=[["this Mac (32 GB), live vm_stat",vm,32768],["synthetic idle 16 GB Mac",synthetic16(15000,620000,330000,8000,5000),16384],["synthetic 16 GB Mac, build running",synthetic16(9000,420000,300000,6000,3000),16384],["synthetic busy 16 GB Mac",synthetic16(6000,350000,250000,4000,2000),16384],["synthetic starved 16 GB Mac",synthetic16(3000,60000,40000,900,1000),16384]];
for(const[which,root]of Object.entries(roots)){
 const sampling=await import(root+"/"+PROC),wanted=await import(root+"/"+W+"🟦️.ts"),cost=await import(roots.before+"/"+W+"💰️cost/🟦️.ts");
 for(const[name,text,total]of readings){
  const bytes=sampling.createHostAvailableBytesV1({platform:"darwin",run:()=>text,total:()=>total*MiB,environment:{}})(),availableMegabytes=Math.floor(bytes/MiB),host=total===32768?16384:total,reading={hostMegabytes:host,availableMegabytes};
  const admitted=wanted.fleetAdmittedMegabytesV1(reading),grantMemory=wanted.fleetGrantMegabytesV1?wanted.fleetGrantMegabytesV1(reading):admitted,fit=wanted.inferableProjectsV1(admitted),grant=cost.ownerGrantV1(grantMemory,wanted.wantedPolicyV1);
  console.log(which.padEnd(6),name.padEnd(36),"available",String(availableMegabytes).padStart(6),"MB  admitted",String(admitted).padStart(5),"grant/project",String(grant.managedMegabytes*2).padStart(5),"MB  wave",String(fit.waveProjects).padStart(2),"inferred",String(fit.projects).padStart(3),"(machine of "+host+" MB)");
 }
}
