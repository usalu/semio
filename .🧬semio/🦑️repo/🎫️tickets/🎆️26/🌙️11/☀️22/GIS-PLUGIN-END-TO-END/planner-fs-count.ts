import fsm from "node:fs";
const counts=new Map<string,{calls:number;paths:Map<string,number>}>();
for(const name of ["statSync","lstatSync","readdirSync","realpathSync","readFileSync","existsSync","openSync","readlinkSync","accessSync"] as const){
 const original=(fsm as any)[name];if(typeof original!=="function")continue;
 const wrapped=function(this:unknown,...args:any[]){const entry=counts.get(name)??{calls:0,paths:new Map()};counts.set(name,entry);entry.calls++;const key=String(args[0]);entry.paths.set(key,(entry.paths.get(key)??0)+1);return original.apply(this,args);};
 (fsm as any)[name]=wrapped;
}
import {open} from "node:fs/promises";
const startCpu=process.cpuUsage();
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

const cpu=process.cpuUsage(startCpu);
console.log("cpu user s",(cpu.user/1e6).toFixed(1),"sys s",(cpu.system/1e6).toFixed(1),"peak rss MB",Math.round(process.resourceUsage().maxRSS/1e6*(process.platform==="darwin"?1e6/1024/1024*1:1)/1));
for(const [name,entry] of counts){const distinct=entry.paths.size,top=[...entry.paths].sort((a,b)=>b[1]-a[1]).slice(0,3).map(([p,n])=>n+"x "+p.replace(process.cwd()+"/","").slice(-70));console.log("fs."+name,"calls",entry.calls,"distinct",distinct,"repeat factor",(entry.calls/Math.max(1,distinct)).toFixed(1),"top",JSON.stringify(top));}
