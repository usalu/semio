import{test}from"bun:test";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,withIssuedResourceOperationV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const hops=Number(process.env.AUTHREV_HOPS??0);
test("probe",async()=>{
 const events:string[]=[],tick=()=>new Promise<void>(done=>setImmediate(done));
 const original=receiveScriptProcessInvocation(process.env,async invocation=>{
  const control=createIssuedResourceControlV1(invocation,"authrev:drain",0);
  void withIssuedResourceOperationV1(control,async()=>{await tick();events.push("w1-end");}).then(async()=>{
   for(let i=0;i<hops;i++)await null;
   try{await withIssuedResourceOperationV1(control,async()=>{await new Promise<void>(done=>setTimeout(done,50));events.push("late-end");});}catch{events.push("late-refused");}
  });
 });
 await original;events.push("original-settled");await new Promise<void>(done=>setTimeout(done,150));console.log("EVT hops="+hops+" "+events.join(","));
});
