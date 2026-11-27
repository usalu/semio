import{test}from"bun:test";
import*as jsc from"bun:jsc";
import{settleGraphMemoryV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🧽️settle/🟨️.mjs";
const v=process.env.F106_VARIANT!,tick=()=>new Promise<void>(r=>setTimeout(r,0)),immediate=()=>new Promise<void>(r=>setImmediate(r));
test("settle fresh",async()=>{
 let garbage:object[]|undefined=Array.from({length:3_000_000},(_,at)=>({at,text:"graph-record-"+at}));
 let released=0;
 const o:any={force:true,environment:{},report:()=>undefined};
 o.release=()=>{released=garbage!.length;garbage=undefined;};
 if(v==="yield-in-release-timer")o.release=async()=>{released=garbage!.length;garbage=undefined;await tick();};
 if(v==="yield-in-release-immediate")o.release=async()=>{released=garbage!.length;garbage=undefined;await immediate();};
 if(v==="yield-in-release-microtask")o.release=async()=>{released=garbage!.length;garbage=undefined;await Promise.resolve();};
 if(v==="fullGC-port"){const m=process.memoryUsage;o.ports={sample:()=>{const u=m.call(process);return u.heapUsed+u.external;},collect:()=>{jsc.fullGC();}};}
 if(v==="gcAndSweep-port"){o.ports={sample:()=>{const u=process.memoryUsage();return u.heapUsed+u.external;},collect:()=>{jsc.gcAndSweep();}};}
 if(v==="gc-port-baseline"){o.ports={sample:()=>{const u=process.memoryUsage();return u.heapUsed+u.external;},collect:()=>{Bun.gc(true);}};}
 const result=await settleGraphMemoryV1(o);
 console.log("RESULT",v,result!.managedBeforeMegabytes,result!.managedMegabytes,released);
});
