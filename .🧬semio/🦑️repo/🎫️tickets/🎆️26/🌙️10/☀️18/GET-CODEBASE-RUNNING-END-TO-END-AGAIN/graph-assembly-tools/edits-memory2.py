import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
M=L+"/🏗️build/🧠️memory/"
apply(M+"🟨️.mjs",[
 ('''/** 🔌️ The live managed bytes of the JavaScript heap and the collection that releases them; both are ports so every platform and every test supplies its own. */
export function createMemoryPortsV1(){
 return Object.freeze({
  sample:()=>{const memory=process.memoryUsage();return memory.heapUsed+memory.external;},
  collect:()=>{runtimeCollectGarbageV1(true);},
 });
}
''','''/** 🔌️ The live managed bytes of the JavaScript heap and the collection that releases them; both are ports so every platform and every test supplies its own. The sample reads the runtime counters (constant time): a heap walk costs time proportional to the heap and ran every few milliseconds. */
export function createMemoryPortsV1(){
 return Object.freeze({
  sample:()=>{const memory=process.memoryUsage();return memory.heapUsed+memory.external;},
  collect:()=>{runtimeCollectGarbageV1(true);},
 });
}

/** 📐️ The managed bytes that stay live after one collection: the fixed working set of the process (discovery results, parsers, caches) that no admission can influence, so a governor charges only what grows beyond it. */
export function measureMemoryBaselineV1(ports=createMemoryPortsV1()){
 ports.collect();return ports.sample();
}
'''),
 ('export function createMemoryGovernorV1({budgetBytes,workers,sample,collect,sleep=defaultSleep,now=()=>performance.now(),signal,report,environment={},policy=P}){\n if(!Number.isSafeInteger(workers)||workers<1||!(budgetBytes>0))throw Error("Positive worker count and memory budget required");',
  'export function createMemoryGovernorV1({budgetBytes,workers,baselineBytes=0,sample,collect,sleep=defaultSleep,now=()=>performance.now(),signal,report,environment={},policy=P}){\n if(!Number.isSafeInteger(workers)||workers<1||!(budgetBytes>0)||!(baselineBytes>=0))throw Error("Positive worker count and memory budget required");'),
 ('lastUsed=read();lastSampleAt=at;','lastUsed=Math.max(0,read()-baselineBytes);lastSampleAt=at;'),
])
apply(M+"🟨️.d.mts",[
 ('export function createMemoryPortsV1():MemoryPortsV1;\n','export function createMemoryPortsV1():MemoryPortsV1;\nexport function measureMemoryBaselineV1(ports?:MemoryPortsV1):number;\n'),
 ('export interface MemoryGovernorOptionsV1{budgetBytes:number;workers:number;sample?','export interface MemoryGovernorOptionsV1{budgetBytes:number;workers:number;baselineBytes?:number;sample?'),
])
apply(M+"🟨️.mjs",[
 (' * 🚦️ Gates the start of concurrent project inferences by the managed memory: at most `workers` run,',' * 🚦️ Gates the start of concurrent project inferences by the managed memory above `baselineBytes` (the fixed working set that no admission can influence, see {@link measureMemoryBaselineV1}): at most `workers` run,'),
 ('@param {{budgetBytes:number;workers:number;sample?','@param {{budgetBytes:number;workers:number;baselineBytes?:number;sample?'),
])
