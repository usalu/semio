import{cpuUsage,memoryUsage}from"node:process";
import{mkdirSync,readdirSync,rmSync,statSync}from"node:fs";
import{join}from"node:path";
const seconds=Number(process.argv[2]??"300"),scratch=process.argv[3]!,root=process.argv[4]??"/Users/ueli/Documents/semio",origin=process.argv[5]??"g1-m01",preload=process.argv[6]!=="bare";
const F="/Users/ueli/Documents/semio/🧰️framework",B=F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",H=F+"/🔨️modules/🏃️process/🏗️build";
const rss=()=>memoryUsage.rss();
const bare=rss();
if(preload){
 // what the readiness service already holds in memory without the collector
 await import(H+"/🏠️machine-state/🟦️.ts");await import(H+"/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts");await import(H+"/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts");
 await import(B+"/../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts");await import(B+"/📈️counter-ledger/🟦️.ts");await import(B+"/🗄️cas/🗃️store/🟦️.ts");await import(B+"/../⚡️caching/🚀️bootstrap/📊️nx-cache/🟦️.ts");await import(B+"/🌐️remote-execution/🚀️offload/📊️counters/🟦️.ts");await import(F+"/🔨️modules/🏃️process/🔖️identity/🟦️.ts");
}
Bun.gc(true);
const base=rss(),cpuBefore=cpuUsage();
const{createFleetMetricsCollectorV1}=await import(H+"/📈️metrics/🛰️collector/🟦️.ts");
const{fleetMetricsSourcesV1}=await import(B+"/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts");
const afterImport=rss();
const store=join(scratch,"store"),rings=join(scratch,"rings");
rmSync(scratch,{recursive:true,force:true});mkdirSync(join(store,"v1","origins",origin),{recursive:true});mkdirSync(join(store,"v1","fleet"),{recursive:true});
const sourceDir="/Users/Shared/semio-store/v1/origins/"+origin;
const{cpSync}=await import("node:fs");
for(const f of["pool.json","ready.json","head.json","slice-memory.json"])try{cpSync(`${sourceDir}/${f}`,join(store,"v1","origins",origin,f));}catch{}
try{cpSync("/Users/Shared/semio-store/v1/fleet/cache.json",join(store,"v1","fleet","cache.json"));}catch{}
const environment={...process.env,SEMIO_FLEET_STORE_ROOT:store};
const collector=createFleetMetricsCollectorV1({directory:rings,sources:fleetMetricsSourcesV1({root,environment,origin}),store:{root:store,origin},report:(l:string)=>console.log(l)});
let peak=rss();const sampler=setInterval(()=>{peak=Math.max(peak,rss());},20);setTimeout(()=>clearInterval(sampler),5000);
const stepCpu:number[]=[],cpuStart=cpuUsage(),start=performance.now(),durations:number[]=[];let published=0,failed=0,bytes=0;
while(performance.now()-start<seconds*1000){
 const began=performance.now(),c0=cpuUsage(),s=collector.step(),c1=cpuUsage(c0);peak=Math.max(peak,rss());durations.push((performance.now()-began)*1000);const used=c1.user+c1.system;stepCpu.push(used);published+=s.published;failed+=s.failed;
 await Bun.sleep(1000-((performance.now()-start)%1000));
}
clearInterval(sampler);collector.close();peak=Math.max(peak,rss());
const cpu=cpuUsage(cpuStart),wall=(performance.now()-start)/1000,sorted=durations.sort((a,b)=>a-b),dir=join(store,"v1","origins",origin,"metrics");
for(const f of readdirSync(dir))bytes+=statSync(join(dir,f)).size;
const mb=(n:number)=>(n/1048576).toFixed(1);
const sc=[...stepCpu].sort((a,b)=>a-b),firstCpu=stepCpu[0]!,restCpu=stepCpu.slice(1).reduce((a,b)=>a+b,0);
console.log(JSON.stringify({firstStepCpuMs:(firstCpu/1000).toFixed(1),stepsCpuPercentExcludingFirst:(restCpu/1e6/wall*100).toFixed(3),stepCpuP50Us:sc[Math.floor(sc.length/2)],stepCpuP99Us:sc[Math.floor(sc.length*0.99)],seconds:wall.toFixed(1),steps:durations.length,preloadedBaseline:preload,cpuPercentOfOneCore:((cpu.user+cpu.system)/1e6/wall*100).toFixed(3),cpuUserSec:(cpu.user/1e6).toFixed(3),cpuSystemSec:(cpu.system/1e6).toFixed(3),bareMB:mb(bare),baseMB:mb(base),importDeltaMB:mb(afterImport-base),peakOverBaseMB:mb(peak-base),endOverBaseMB:mb(rss()-base),peakMB:mb(peak),stepMeanUs:(durations.reduce((a,b)=>a+b,0)/durations.length).toFixed(0),stepP50Us:sorted[Math.floor(sorted.length/2)]!.toFixed(0),stepP99Us:sorted[Math.floor(sorted.length*0.99)]!.toFixed(0),stepMaxUs:sorted.at(-1)!.toFixed(0),filesTouched:published,publishFailures:failed,publishedDirBytes:bytes}));
