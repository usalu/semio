import{cpuUsage,memoryUsage}from"node:process";
import{cpSync,mkdirSync,readdirSync,rmSync,statSync}from"node:fs";
import{join}from"node:path";
const seconds=Number(process.argv[2]??"300"),scratch=process.argv[3]!,root=process.argv[4]??"/Users/ueli/Documents/semio",origin=process.argv[5]??"g1-m01";
const M="/Users/ueli/Documents/semio/🧰️framework";
const rssBefore=memoryUsage.rss(),cpuBefore=cpuUsage(),startedAt=performance.now();
const{createFleetMetricsCollectorV1}=await import(M+"/🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🟦️.ts");
const{fleetMetricsSourcesV1}=await import(M+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts");
const store=join(scratch,"store"),rings=join(scratch,"rings");
rmSync(scratch,{recursive:true,force:true});mkdirSync(join(store,"v1","origins",origin),{recursive:true});mkdirSync(join(store,"v1","fleet"),{recursive:true});
for(const f of["pool.json","ready.json","head.json","slice-memory.json"])try{cpSync(`/Users/Shared/semio-store/v1/origins/${origin}/${f}`,join(store,"v1","origins",origin,f));}catch{}
try{cpSync("/Users/Shared/semio-store/v1/fleet/cache.json",join(store,"v1","fleet","cache.json"));}catch{}
const environment={...process.env,SEMIO_FLEET_STORE_ROOT:store};
const collector=createFleetMetricsCollectorV1({directory:rings,sources:fleetMetricsSourcesV1({root,environment,origin}),store:{root:store,origin},report:(l:string)=>console.log(l)});
const rssLoaded=memoryUsage.rss(),cpuLoaded=cpuUsage(),loadedAt=performance.now();
let peak=rssLoaded,steps=0,published=0,failed=0;const durations:number[]=[];
const start=performance.now();
while(performance.now()-start<seconds*1000){
 const tickStart=performance.now(),s=collector.step();steps++;published+=s.published;failed+=s.failed;durations.push((performance.now()-tickStart)*1000);
 peak=Math.max(peak,memoryUsage.rss());
 const wait=1000-((performance.now()-start)%1000);await Bun.sleep(wait);
}
collector.close();
const cpu=cpuUsage(cpuLoaded),wall=(performance.now()-start)/1000,sorted=durations.sort((a,b)=>a-b);
const bytes=readdirSync(join(store,"v1","origins",origin,"metrics")).reduce((n,f)=>n+statSync(join(store,"v1","origins",origin,"metrics",f)).size,0);
console.log(JSON.stringify({seconds:wall.toFixed(1),steps,cpuPercentOfOneCore:((cpu.user+cpu.system)/1e6/wall*100).toFixed(3),cpuUserSec:(cpu.user/1e6).toFixed(3),cpuSystemSec:(cpu.system/1e6).toFixed(3),loadCpuMs:(((cpuLoaded.user+cpuLoaded.system)-(cpuBefore.user+cpuBefore.system))/1000).toFixed(0),loadMs:(loadedAt-startedAt).toFixed(0),rssBeforeImportMB:(rssBefore/1048576).toFixed(1),rssAfterImportMB:(rssLoaded/1048576).toFixed(1),rssPeakMB:(peak/1048576).toFixed(1),rssEndMB:(memoryUsage.rss()/1048576).toFixed(1),stepMeanUs:(durations.reduce((a,b)=>a+b,0)/durations.length).toFixed(0),stepP50Us:sorted[Math.floor(sorted.length/2)]!.toFixed(0),stepP99Us:sorted[Math.floor(sorted.length*0.99)]!.toFixed(0),stepMaxUs:sorted.at(-1)!.toFixed(0),filesPublished:published,publishFailures:failed,publishedDirBytes:bytes}));
