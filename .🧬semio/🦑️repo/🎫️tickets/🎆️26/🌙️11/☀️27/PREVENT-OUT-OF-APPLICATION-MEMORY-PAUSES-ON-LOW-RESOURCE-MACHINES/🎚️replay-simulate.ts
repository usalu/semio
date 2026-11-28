/** 🎚️ Replay of the live gate of 2026-11-27 23:19 through the real admission function (MEM-ADMIT): the four live grants and the seven waiting requests are the snapshot (repo capacity status), the host carries the 5857 MB it held outside the grants, and every admitted grant ramps to a share of its reservation like the live ones did (45 %, 43 %, 60 %). Two worlds, one workload: the rigid policy (no `memory.elastic`) and the elastic one. Prints admissions, waits and the least free memory over two hours; `GROW=1` lets every grant grow to its whole reservation at +30 min (the worst case the warden has to cover). */
import buildPolicy from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {evaluateHostCapacityV1,readHostCapacityPolicyV1,withUnmanagedHostV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";

const LOW=process.env.LOW==="1",TOTAL=Number(process.env.TOTAL??(LOW?8192:16384)),CORES=Number(process.env.CORES??(LOW?4:10)),FOREIGN=Number(process.env.FOREIGN??(LOW?1700:5857)),SHARE=Number(process.env.SHARE??0.45),RUN=Number(process.env.RUN??480),HOURS=Number(process.env.HOURS??2),GROW=process.env.GROW==="1",STEP=5,BEAT=15;
const {memory:{elastic:_,...rigid}}=buildPolicy.hostCapacity as never as {memory:{elastic:unknown}},policies={rigid:readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:rigid} as never),elastic:readHostCapacityPolicyV1(buildPolicy.hostCapacity)};
type Live={grant:HostGrantV1;share:number;endsAt:number;beats:number;mean:number;squares:number;peak:number};
const id=(name:string)=>name.padEnd(8,"_").slice(0,32);
const sizes=LOW?[2200,1900,1500,1500,1900,1600,1200]:[3600,1900,3600,1600,1200,1200,1200],holders=LOW?{old:[1500,700,0.45],plan:[1800,1000,0.6],fresh:[1500,650,0.43],tiny:[512,68,0.13]}:{old:[1900,852,0.45],plan:[4599,2746,0.6],fresh:[3600,1557,0.43],tiny:[512,68,0.13]};
function run(name:"rigid"|"elastic"){
 const policy=policies[name],t0=1_000_000_000_000,live:Live[]=[],waiters:{waiter:HostWaiterV1;size:number}[]=[],waits:number[]=[],admittedAt:Record<string,number>={};
 const start=(grant:Partial<HostGrantV1>&{id:string;operation:string;reservedMegabytes:number;observedMegabytes:number},share:number,ageSeconds:number,endsAt:number)=>live.push({grant:{jobs:1,cpu:1,admittedAtMs:t0-ageSeconds*1000,pid:100,bootAtMs:0,priority:"batch",...grant} as HostGrantV1,share,endsAt,beats:Math.floor(ageSeconds/BEAT),mean:grant.observedMegabytes,squares:0,peak:grant.observedMegabytes});
 start({id:id("old"),operation:"command",reservedMegabytes:holders.old[0]!,observedMegabytes:holders.old[1]!,retainReservation:true,session:"a3fba9d52db0.a"},0.45,7079,Infinity);
 start({id:id("plan"),operation:"graph-plan",reservedMegabytes:holders.plan[0]!,observedMegabytes:holders.plan[1]!,retainReservation:true,label:"graph-plan",session:"a3fba9d52db0.b"},0.6,727,1200);
 start({id:id("fresh"),operation:"command",reservedMegabytes:holders.fresh[0]!,observedMegabytes:holders.fresh[1]!,retainReservation:true,session:"a3fba9d52db0.a"},0.43,395,Infinity);
 start({id:id("tiny"),operation:"run-many",reservedMegabytes:512,observedMegabytes:68,quiet:true,rampMilliseconds:300000,session:"a3fba9d52db0.b"},0.13,727,Infinity);
 for(const grant of live){const g=grant.grant as {peakMegabytes?:number;observations?:number;deviationMegabytes?:number};g.peakMegabytes=grant.grant.observedMegabytes!;g.observations=grant.beats;g.deviationMegabytes=grant.grant.id.startsWith("plan")?120:grant.grant.id.startsWith("fresh")?120:40;}
 const enqueue=(index:number,arrived:number,size:number)=>waiters.push({size,waiter:{arrivedAtMs:arrived,pid:1000+index,session:`a3fba9d52db0.${index%2?"a":"b"}`,request:{id:id(`w${index}`),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:size,perJobMegabytes:0,cpuPerJob:1,label:"command",retainReservation:true}} as never});
 const arrivedAgo=[1319,446,439,869,907,996,995];sizes.forEach((size,index)=>enqueue(index,t0-arrivedAgo[index]!*1000,size));
 let next=sizes.length,minAvailable=Infinity,maxOvercommit=-Infinity,admissions=0;const end=HOURS*3600;
 for(let second=0;second<=end;second+=STEP){
  const now=t0+second*1000;
  if(second%600===0&&second>0)enqueue(next,now,sizes[next%sizes.length]!),next++;
  for(const row of [...live])if(second>=row.endsAt)live.splice(live.indexOf(row),1);
  for(const row of live){
   const g=row.grant as HostGrantV1&{peakMegabytes?:number;observations?:number;deviationMegabytes?:number},age=(now-g.admittedAtMs)/1000,full=GROW&&second>=1800&&row.share<1?1:row.share,use=Math.min(1,age/90)*full*g.reservedMegabytes;
   const observed=Math.round(Math.max(g.observedMegabytes!<use&&age>=0?use:use,0));
   if(second%BEAT===0){row.beats++;const delta=observed-row.mean;row.mean+=delta/row.beats;row.squares+=delta*(observed-row.mean);row.peak=Math.max(row.peak,observed);Object.assign(g,{observedMegabytes:observed,peakMegabytes:row.peak,observations:row.beats,deviationMegabytes:row.beats>1?Math.ceil(Math.sqrt(row.squares/(row.beats-1))):0});}
  }
  const held=live.reduce((sum,row)=>sum+(row.grant.observedMegabytes??0),0),available=TOTAL-FOREIGN-held;minAvailable=Math.min(minAvailable,available);
  const host={version:1,atMs:now,platform:"darwin",cores:CORES,totalMegabytes:TOTAL,availableMegabytes:Math.max(0,available),pressure:"normal",swapUsedMegabytes:1718,swapOutMegabytesPerSecond:0,load:3,openFiles:6000,openFilesMaximum:122880,attachedImages:21,compressedMegabytes:3108} as HostSampleV1,ledger=live.map(row=>row.grant);
  if(waiters.length===0)continue;
  const evaluation=evaluateHostCapacityV1(policy,withUnmanagedHostV1(host,ledger),ledger,waiters.map(row=>row.waiter),now);
  for(const decision of evaluation.decisions.filter(row=>row.verdict==="admit")){
   const at=waiters.findIndex(row=>row.waiter.request.id===decision.id);if(at<0)continue;
   const {waiter,size}=waiters.splice(at,1)[0]!;waits.push((now-waiter.arrivedAtMs)/1000);admittedAt[decision.id]=second;admissions++;
   const grant={id:decision.id,operation:"command",jobs:1,cpu:1,reservedMegabytes:decision.reservedMegabytes,observedMegabytes:0,admittedAtMs:now,pid:2000,bootAtMs:0,priority:"batch",retainReservation:true,session:waiter.session,peakMegabytes:0,observations:0,deviationMegabytes:0} as HostGrantV1;
   live.push({grant,share:SHARE,endsAt:second+RUN,beats:0,mean:0,squares:0,peak:0});void size;
  }
  maxOvercommit=Math.max(maxOvercommit,live.reduce((sum,row)=>sum+(row.grant.reservedMegabytes),0)-live.reduce((sum,row)=>sum+(row.grant.observedMegabytes??0),0));
 }
 const sorted=[...waits].sort((a,b)=>a-b),mean=waits.reduce((s,v)=>s+v,0)/Math.max(1,waits.length);
 return {policy:name,admitted:admissions,stillWaiting:waiters.length,meanWaitSeconds:Math.round(mean),medianWaitSeconds:Math.round(sorted[Math.floor(sorted.length/2)]??0),maxWaitSeconds:Math.round(sorted[sorted.length-1]??0),firstSevenAdmittedAtSeconds:sizes.map((_,index)=>admittedAt[id(`w${index}`)]??null),leastAvailableMegabytes:minAvailable,floorMegabytes:Math.max(1536,Math.min(4096,Math.floor(TOTAL*0.125))),overcommittedMegabytes:Math.round(maxOvercommit)};
}
console.log(JSON.stringify({total:TOTAL,cores:CORES,grow:GROW,rigid:run("rigid"),elastic:run("elastic")},null,1));
