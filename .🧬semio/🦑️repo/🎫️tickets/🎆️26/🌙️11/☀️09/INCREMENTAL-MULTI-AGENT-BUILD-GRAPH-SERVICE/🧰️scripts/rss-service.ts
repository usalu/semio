import{readFileSync,writeFileSync}from"node:fs";
import{heapStats}from"bun:jsc";
const ROOT="/home/deck/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",out=process.argv[2]??"rss-service.json";
const mb=(n:number)=>Math.round(n/1048576),status=()=>Object.fromEntries(readFileSync("/proc/self/status","utf8").split("\n").filter(l=>/^(VmRSS|VmHWM|RssAnon):/.test(l)).map(l=>[l.split(":")[0],Math.round(parseInt(l.split(/\s+/)[1])/1024)]));
const rows:any[]=[];let peak=0;setInterval(()=>{const s=status();peak=Math.max(peak,s.VmRSS);const h=heapStats();console.error("TICK "+JSON.stringify({rss:s.VmRSS,heap:mb(h.heapSize),extra:mb(h.extraMemorySize),objs:h.objectCount,top:h.heapSize>400*1048576?Object.entries(h.objectTypeCounts).sort((a:any,b:any)=>b[1]-a[1]).slice(0,6).map(([k,v])=>k+"="+v).join(","):""}));},2000);
async function mark(phase:string,extra:any={}){const before=status();Bun.gc(true);await new Promise(r=>setTimeout(r,1500));Bun.gc(true);await new Promise(r=>setTimeout(r,1500));const h=heapStats(),after=status(),row={phase,peakSoFarMb:peak,rssBeforeGcMb:before.VmRSS,rssMb:after.VmRSS,hwmMb:after.VmHWM,heapMb:mb(h.heapSize),extraMb:mb(h.extraMemorySize),objects:h.objectCount,top:Object.entries(h.objectTypeCounts).sort((a:any,b:any)=>b[1]-a[1]).slice(0,6).map(([k,v])=>k+"="+v),...extra};rows.push(row);console.error(JSON.stringify(row));}
await mark("start");
const routing=await import(ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const{loadBuildInvocationGraphV1}=await import(B+"🧭️invocation/🕸️graph/🟦️.ts");
await routing.receiveScriptProcessInvocation(process.env,async(original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"graph:resident",0);
  await mark("modules-loaded");
  const graph:any=await loadBuildInvocationGraphV1(ROOT,["graph","--graph-policy=current"],original,original,control,false,undefined,"current");
  const sizes:Record<string,number>={};for(const key of Object.keys(graph))sizes[key]=JSON.stringify(graph[key]).length;
  await mark("graph-loaded",{projects:Object.keys(graph.projects??{}).length,edges:graph.dependencies?.length,jsonMbByKey:Object.fromEntries(Object.entries(sizes).map(([k,v])=>[k,mb(v)]))});
  const slice=JSON.stringify(graph);await mark("graph-serialised",{jsonMb:mb(slice.length)});
});
writeFileSync(out,JSON.stringify({peakMb:peak,rows},null,1));console.error("PEAK_RSS_MB",peak);process.exit(0);
