/** 📏️ Measures the wall of a no-op run (and of a run that must execute) behind a busy machine-wide capacity queue, before (no forecast: the run asks for the full grant at the door) and after (forecast quiet: a small grant at once, the full grant only if work has to run). Real gate directory and real library door; the host is simulated as a 16 GB machine whose free memory is what the simulated agents do not hold; the bodies are sleeps. Usage: [MEASURE_PRIORITY=agent|interactive|background] bun measure.ts <overlay-root> <repetitions> [scale] */
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
const [overlay,repetitionsText="5",scaleText="1"]=process.argv.slice(2);
if(!overlay)throw Error("overlay root required");
const scale=Number(scaleText),repetitions=Number(repetitionsText);
const F="🧰️framework";
const processFacade=`${overlay}/${F}/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts`;
const library=`${overlay}/${F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`;
const policyFile=`${overlay}/${F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`;
const gate:any=await import(processFacade),door:any=await import(library),buildPolicy=(await import(policyFile,{with:{type:"json"}})).default;
const policy=gate.readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:20,progressMilliseconds:60000,heartbeatMilliseconds:50,sampleMaximumAgeMilliseconds:0,lendAfterMilliseconds:3600000}});
const ms=(seconds:number)=>Math.round(seconds*1000*scale);
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const args=["run","@x/y:build","--parallel=2"],leaf=door.buildCapacityAskV1(strip(args),{},policy).lineage.at(-1);

async function scenario(mode:"before"|"after",kind:"noop"|"work"){
 const directory=mkdtempSync(join(tmpdir(),"hm-"));
 try{
  let occupied=0;const total=16384;
  const sample=async()=>({host:{version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes:total-occupied,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10},swappedOutMegabytes:0});
  if(mode==="after")for(let index=0;index<5;index++)await gate.updateHostEstimateStoreV1(directory,policy,(store:any)=>gate.recordHostForecastV1(policy,store,leaf,{executed:false,peakMegabytes:1200,activeMilliseconds:8000,peakAtMilliseconds:4000,endedAtMs:Date.now()-3_600_000+index*60_000}));
  const agent=(fixed:number,holdMs:number)=>gate.withHostCapacityV1({directory,policy,request:{id:gate.newHostCapacityIdV1(),operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:fixed,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{},write:()=>undefined,sample,waited:()=>0,observe:async()=>fixed},async()=>{occupied+=fixed;try{await sleep(holdMs);}finally{occupied-=fixed;}});
  const agents=[agent(4200,ms(4)),agent(4200,ms(8)),agent(4200,ms(12))];
  await sleep(ms(0.2));
  const waiting=[(async()=>{await sleep(ms(0.1));return agent(4200,ms(3));})(),(async()=>{await sleep(ms(0.3));return agent(4200,ms(3));})()];
  await sleep(ms(0.6));
  const started=performance.now();let answered=0,executing=0,executed=false;
  const environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_BUILD_TOOL:"bazel",...process.env.MEASURE_PRIORITY?{SEMIO_LEASE_PRIORITY:process.env.MEASURE_PRIORITY}:{}};
  await door.withBuildCapacityV1(args,environment,async()=>{
   answered=performance.now()-started;
   if(kind==="work"){await door.upgradeBuildCapacityV1(undefined,environment);executing=performance.now()-started;executed=true;}
   await sleep(ms(0.4));
  },{directory,policy,strip,tool:"bazel",forecast:mode==="after",sample,observe:async()=>executed?6000:1200,write:()=>undefined,exitCode:()=>0,waited:()=>0,control:control()});
  const wall=performance.now()-started;
  await Promise.all([...agents,...waiting.map(async row=>(await row))]);
  return{wall,answered,executing};
 }finally{rmSync(directory,{recursive:true,force:true});}
}
const rows:Record<string,{wall:number[];answered:number[];executing:number[]}>={};
for(const kind of ["noop","work"] as const)for(const mode of ["before","after"] as const){
 const key=`${kind}/${mode}`;rows[key]={wall:[],answered:[],executing:[]};
 for(let index=0;index<repetitions;index++){const row=await scenario(mode,kind);rows[key]!.wall.push(row.wall);rows[key]!.answered.push(row.answered);rows[key]!.executing.push(row.executing);}
}
const median=(values:number[])=>[...values].sort((a,b)=>a-b)[Math.floor(values.length/2)]!;
const round=(value:number)=>Math.round(value)/1000;
console.log(JSON.stringify({scale,repetitions,peakRssMegabytes:Math.round(process.memoryUsage().rss/1048576),table:Object.fromEntries(Object.entries(rows).map(([key,row])=>[key,{wallMedianS:round(median(row.wall)),wallMinS:round(Math.min(...row.wall)),wallMaxS:round(Math.max(...row.wall)),answeredMedianS:round(median(row.answered)),executingMedianS:round(median(row.executing))}]))}));
process.exit(0);
