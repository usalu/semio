const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {evaluateHostCapacityV1,sizeHostRequestV1,estimateHostRatioV1,scaleHostRequestV1,rampHostRequestV1,recordHostOutcomeV1,emptyHostEstimateStoreV1,readHostCapacityPolicyV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts");
import policyJson from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import fix from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/📈️memory-fit/🧫️fixtures/🔣️.json";
const policy=readHostCapacityPolicyV1(policyJson.hostCapacity);
function run(opts:{policy:any;strip:boolean;train:{ratio:number;active:number}[];draw:{ratio:number;active:number}[];n:number;total?:number;foreign?:number;seed:number}){
 const total=opts.total??16384,foreign=opts.foreign??6000,pol=opts.policy;
 let store=emptyHostEstimateStoreV1();
 opts.train.forEach((s,i)=>{store=recordHostOutcomeV1(pol,store,["run","run:test"],{operation:"run",jobs:1,peakMegabytes:Math.round(s.ratio*3584/10000),defaultMegabytes:3584,activeMilliseconds:s.active,peakAtMilliseconds:s.active,endedAtMs:1_000_000+i});});
 const estimate=estimateHostRatioV1(pol,store,["run","run:test"],2_000_000);
 const base=sizeHostRequestV1(pol,{id:"base00001",operation:"run",preferredJobs:1,kinds:["test"]})!;
 const scaled=scaleHostRequestV1(pol,base,estimate.ratioBasisPoints,total-2048);
 const request=opts.strip?scaled:rampHostRequestV1(base,scaled,estimate);
 let s=opts.seed;const rnd=()=>{s=(s*1664525+1013904223)>>>0;return s/4294967296;};
 const runs=Array.from({length:opts.n},()=>{const d=opts.draw[Math.floor(rnd()*opts.draw.length)]!;return {peak:d.ratio*3584/10000,ttp:d.active,duration:d.active+30000};});
 const waiters=runs.map((_,i)=>({request:{...request,id:`req${String(i).padStart(6,"0")}`},arrivedAtMs:i,pid:1}));
 let grants:any[]=[];const state=new Map<string,{adm:number,run:typeof runs[0]}>();let minFree=Infinity,maxConc=0,maxRamp=0,done=0,t=0;const waiting=new Set(waiters.map(w=>w.request.id));const adm:number[]=[];
 const real=(r:typeof runs[0],start:number,at:number)=>r.peak*(0.2+0.8*Math.min(1,(at-start)/r.ttp));
 for(t=0;t<4*3600_000&&(done<runs.length);t+=1000){
  grants=grants.filter(g=>{const st=state.get(g.id)!;if(t-st.adm>=st.run.duration){done++;return false;}return true;});
  const sum=grants.reduce((a,g)=>a+real(state.get(g.id)!.run,state.get(g.id)!.adm,t),0),free=total-foreign-sum;minFree=Math.min(minFree,free);maxConc=Math.max(maxConc,grants.length);
  maxRamp=Math.max(maxRamp,grants.filter(g=>g.rampSlot&&t-g.admittedAtMs<(g.rampMilliseconds??pol.memory.rampMilliseconds)).length);
  for(const g of grants){const st=state.get(g.id)!;const age=t-st.adm;if(age>=3000&&(age-3000)%15000<1000)g.observedMegabytes=Math.round(real(st.run,st.adm,t));}
  const avail=Math.max(0,free),pressure=avail*10000<total*pol.pressure.availableCriticalBasisPoints?"critical":avail*10000<total*pol.pressure.availableWarnBasisPoints?"warn":"normal";
  const host={version:1,atMs:t,platform:"darwin",cores:8,totalMegabytes:total,availableMegabytes:Math.round(avail),pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:null,openFilesMaximum:null,attachedImages:null} as any;
  const queue=waiters.filter(w=>waiting.has(w.request.id));
  if(queue.length===0)continue;
  const ev=evaluateHostCapacityV1(pol,host,grants,queue,t);
  for(const d of ev.decisions)if(d.verdict==="admit"){const w=waiters.find(x=>x.request.id===d.id)!;waiting.delete(d.id);const idx=waiters.indexOf(w);state.set(d.id,{adm:t,run:runs[idx]!});adm.push(t);grants.push({id:d.id,operation:"run",jobs:d.jobs,cpu:d.jobs,reservedMegabytes:d.reservedMegabytes,admittedAtMs:t,observedMegabytes:null,pid:1,bootAtMs:0,...w.request.rampMilliseconds===undefined?{}:{rampMilliseconds:w.request.rampMilliseconds},...w.request.rampSlot===undefined?{}:{rampSlot:w.request.rampSlot}});}
 }
 return {estimate,reservedMB:request.fixedMegabytes+request.perJobMegabytes,minFree:Math.round(minFree),maxConc,maxRamp,finished:done===runs.length?t:undefined,firstAdmits:adm.slice(0,6)};
}
const all=fix.classes.run.map(s=>({ratio:s.ratioBasisPoints,active:s.activeMilliseconds}));
const hot=all.filter(s=>s.ratio<8000),cold=all.filter(s=>s.ratio>=8000);
console.log("samples",all.length,"hot",hot.length,"cold",cold.length);
const oldPolicy=readHostCapacityPolicyV1({...policyJson.hostCapacity,memory:{...policyJson.hostCapacity.memory,maximumRamping:64}});
const mk=(n:number)=>[...Array(n)].flatMap(()=>hot);
for(const [name,train,draw] of [["trained on all",all,all],["hot only",mk(3).slice(0,15),all],["wrong",[...Array(16)].map(()=>({ratio:3000,active:200000})),all]] as const){
 const res:Record<string,any[]>={old:[],new:[]};
 for(const [label,pol,strip] of [["old",oldPolicy,true],["new",policy,false]] as const)for(let seed=1;seed<=16;seed++){const r=run({policy:pol,strip,train:train as any,draw:draw as any,n:8,seed});res[label]!.push(r);}
 console.log(name,"old minFree",res.old!.map(r=>r.minFree).join(","),"| new",res.new!.map(r=>r.minFree).join(","),"| new maxRamp",Math.max(...res.new!.map(r=>r.maxRamp)),"old maxConc",Math.max(...res.old!.map(r=>r.maxConc)),"new maxConc",Math.max(...res.new!.map(r=>r.maxConc)),"finished old/new median",res.old![8].finished,res.new![8].finished);
}
