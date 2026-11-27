import {decideHostCapacityV1,readHostCapacityPolicyV1,hostFloorMegabytesV1,type HostGrantV1,type HostWaiterV1,type HostSampleV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import buildPolicy from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity);
let seed=7;const rnd=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;};
const pick=(n:number)=>Math.floor(rnd()*n);
let bad=0;
for(let round=0;round<20000&&bad<5;round++){
 const now=1e9,total=16384,avail=pick(12000)+500;
 const host:HostSampleV1={version:1,atMs:now,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes:avail,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:100,openFilesMaximum:100000,attachedImages:0,compressedMegabytes:0,unmanagedMegabytes:pick(3000)} as never;
 const ledger:HostGrantV1[]=Array.from({length:pick(6)},(_,i)=>{const plan=rnd()<0.6,reserved=500+pick(5000);return {id:"g"+i,operation:plan?"graph-plan":"run",jobs:1,cpu:1,reservedMegabytes:reserved,admittedAtMs:now-pick(600000),observedMegabytes:rnd()<0.3?null:pick(reserved+1500),pid:100+i,bootAtMs:0,...plan?{retainReservation:true,label:"graph-plan"}:{}} as never;});
 const waiters:HostWaiterV1[]=Array.from({length:1+pick(3)},(_,i)=>{const grow=rnd()<0.5,mb=500+pick(4000);return {arrivedAtMs:now-pick(300000),pid:500+i,request:{id:"w"+i,operation:"graph-plan",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:mb,perJobMegabytes:0,cpuPerJob:1,retainReservation:true,label:"graph-plan",...grow?{minimumMegabytes:Math.min(256,mb)}:{}}} as never;});
 const e=decideHostCapacityV1(policy,host,ledger,waiters,now);
 const floor=hostFloorMegabytesV1(policy,host),share=total-floor-(host.unmanagedMegabytes??0);
 const held=ledger.reduce((s,g)=>s+Math.max(g.retainReservation?g.reservedMegabytes:0,g.observedMegabytes??g.reservedMegabytes),0);
 const added=e.decisions.filter(d=>d.verdict==="admit").reduce((s,d)=>s+d.reservedMegabytes,0);
 if(held+added>share&&added>0){bad++;console.log(JSON.stringify({host,ledger,waiters:waiters.map(w=>w.request),decisions:e.decisions,held,added,share}));}
}
console.log("violations",bad);
