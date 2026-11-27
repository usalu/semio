import {open} from "node:fs/promises";
import {streamJsonObjectV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🌊️json-stream/🟦️.ts";
import {readBuildOwnerRegistryV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import * as producer from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import {captureObservedSourceInputsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
const root=process.cwd(),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),byName=new Map(registry.owners.map(o=>[o.name,o]));
const f=await open(process.argv[2]!,"r"),suffixes:string[]=[];
for await(const e of streamJsonObjectV1(f,new Set(["owners"])))if("element" in e&&e.key==="owners")suffixes.push((e.element as any).suffix);
await f.close();
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const nativeCache=new Map(),nativeClosures=new Map(),times:number[]=[],rowsCount:number[]=[];
const step=Math.max(1,Math.floor(suffixes.length/Number(process.argv[3]??20)));
for(let i=0;i<suffixes.length;i+=step){
 const [name,rest]=suffixes[i]!.split("\0"),[targets,inputs]=JSON.parse(rest!),t=performance.now();
 try{const {observations}=await captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(byName.get(name!),targets,registry,{workspaceRoot:root,selection:{kind:"all"},nativeCache,nativeClosures,preparationControl:{advance:async()=>{}}},inputs),undefined,false);rowsCount.push(observations.length);}catch(e){console.log("fail",name,String((e as Error).message).slice(0,120));}
 times.push(performance.now()-t);if(times.length%20===0){Bun.gc(true);const m=process.memoryUsage();console.log("after",times.length,"owners heapUsed MB",Math.round(m.heapUsed/1e6),"rss MB",Math.round(m.rss/1e6),"external MB",Math.round(m.external/1e6));}
}
times.sort((a,b)=>a-b);const sum=times.reduce((a,b)=>a+b,0);
console.log("sampled",times.length,"of",suffixes.length,"median ms",Math.round(times[times.length>>1]!),"max ms",Math.round(times.at(-1)!),"mean ms",Math.round(sum/times.length),"mean rows",Math.round(rowsCount.reduce((a,b)=>a+b,0)/rowsCount.length),"extrapolated total s for",suffixes.length,"owners:",Math.round(sum/times.length*suffixes.length/1000));
const u=process.resourceUsage();console.log("resourceUsage userCPU s",(u.userCPUTime/1e6).toFixed(1),"systemCPU s",(u.systemCPUTime/1e6).toFixed(1),"fsRead",u.fsRead,"fsWrite",u.fsWrite,"minorFaults",u.minorPageFault,"majorFaults",u.majorPageFault,"voluntaryCtx",u.voluntaryContextSwitches,"maxRSS MB",Math.round(u.maxRSS/1048576));
