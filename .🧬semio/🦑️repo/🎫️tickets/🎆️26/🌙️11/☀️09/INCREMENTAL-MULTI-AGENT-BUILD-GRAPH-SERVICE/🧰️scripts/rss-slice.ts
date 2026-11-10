import{readFileSync,writeFileSync}from"node:fs";
import{heapStats}from"bun:jsc";
const ROOT="/home/deck/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",out=process.argv[2]??"rss-slice.json",policy=(process.argv[3]??"fresh") as "fresh"|"current";
const mb=(n:number)=>Math.round(n/1048576),status=()=>Object.fromEntries(readFileSync("/proc/self/status","utf8").split("\n").filter(l=>/^(VmRSS|VmHWM):/.test(l)).map(l=>[l.split(":")[0],Math.round(parseInt(l.split(/\s+/)[1])/1024)]));
let last=performance.now(),maxLag=0,over1s=0,peak=0;const lags:number[]=[];
setInterval(()=>{const now=performance.now(),lag=now-last-100;last=now;if(lag>maxLag)maxLag=lag;if(lag>1000){over1s++;lags.push(Math.round(lag));}peak=Math.max(peak,status().VmRSS);},100);
setInterval(()=>{const h=heapStats();console.error("TICK "+JSON.stringify({rss:status().VmRSS,heap:mb(h.heapSize),extra:mb(h.extraMemorySize),lagMax:Math.round(maxLag),over1s}));},5000);
const routing=await import(ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const{loadBuildInvocationGraphV1}=await import(B+"🧭️invocation/🕸️graph/🟦️.ts");
const{buildBackendCompactGraphV1,loadBuildBackendV1,buildBackendDefaultIdV1}=await import(B+"🔌️backends/🟦️.ts");
const started=performance.now();
await routing.receiveScriptProcessInvocation(process.env,async(original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"graph:slice",0),tool=buildBackendDefaultIdV1(),backend=await loadBuildBackendV1(tool);
  const graph:any=await loadBuildInvocationGraphV1(ROOT,["run","repo:test-build-observation","--output=static"],original,original,control,false,undefined,policy,buildBackendCompactGraphV1(tool),await backend.graphExternalInputs?.()??[]);
  console.error("SLICE_PROJECTS "+Object.keys(graph.projects??{}).length);
});
const result={wallMs:Math.round(performance.now()-started),peakMb:peak,hwm:status().VmHWM,maxLagMs:Math.round(maxLag),over1s,lags:lags.slice(0,30)};
writeFileSync(out,JSON.stringify(result));console.error("RESULT "+JSON.stringify(result));process.exit(0);
