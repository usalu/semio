import{test,setDefaultTimeout}from"bun:test";
import{readFileSync,writeFileSync}from"node:fs";
import{heapStats}from"bun:jsc";
setDefaultTimeout(1500000);
const{declareLibraryBuildTestEnvelopeV1}=await import("/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🪪️native-birth/🟦️.ts");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
const ROOT="/home/deck/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",out=process.env.ENTRIES_RESULT??"result.json";
const mb=(n:number)=>Math.round(n/1048576),status=()=>Object.fromEntries(readFileSync("/proc/self/status","utf8").split("\n").filter(l=>/^(VmRSS|VmHWM):/.test(l)).map(l=>[l.split(":")[0],Math.round(parseInt(l.split(/\s+/)[1])/1024)]));
test("settle and owner stage in process",async()=>{
 setInterval(()=>{const h=heapStats();console.error("TICK "+JSON.stringify({rss:status().VmRSS,heap:mb(h.heapSize),extra:mb(h.extraMemorySize)}));},10000);
 const routing=await import(ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const{loadBuildInvocationGraphV1}=await import(B+"🧭️invocation/🕸️graph/🟦️.ts");
 const{buildBackendCompactGraphV1,loadBuildBackendV1,buildBackendDefaultIdV1}=await import(B+"🔌️backends/🟦️.ts");
 const{withBuildTestInvocationV1}=await import("@semio-tech/process/testing/invocation");
 const started=performance.now();
 await withBuildTestInvocationV1(async(original:any)=>{
  const control=routing.createIssuedResourceControlV1(original,"graph:slice",0),tool=buildBackendDefaultIdV1(),backend=await loadBuildBackendV1(tool);
  const graph:any=await loadBuildInvocationGraphV1(ROOT,["run","repo:test-build-memory","--dry-run"],original,original,control,false,undefined,"fresh",buildBackendCompactGraphV1(tool),await backend.graphExternalInputs?.()??[]);
  console.error("SLICE_PROJECTS "+Object.keys(graph.projects??{}).length);
 });
 writeFileSync(out,JSON.stringify({wallMs:Math.round(performance.now()-started),hwm:status().VmHWM}));
});
