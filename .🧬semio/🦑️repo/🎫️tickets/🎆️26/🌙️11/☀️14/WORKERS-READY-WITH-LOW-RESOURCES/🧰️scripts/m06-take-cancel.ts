import {fileURLToPath} from "node:url";
const workspace = new URL("../".repeat(8),import.meta.url);
const {createGraphFleetClientV1} = await import(new URL("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/🛰️exchange/📡️client/🟦️.ts",workspace).href);
const {createProjectFleetPortV1,runTakeLookV1} = await import(new URL("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🟦️.ts",workspace).href);
const {takeAdmittedV1,takePassV1} = await import(new URL("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/📥️take/🟨️.mjs",workspace).href);

let captured,release,started;
const entered=new Promise(resolve=>{started=resolve;});
const fixture=fileURLToPath(new URL("./../",import.meta.url));
const client=createGraphFleetClientV1({endpoint:"https://fixture.invalid",instance:"graph",upload:false,stateDirectory:fixture,token:undefined},{
 fetch:async(_input,init)=>{
  captured=init.signal;
  await new Promise(resolve=>{
   release=resolve;
   captured.addEventListener("abort",resolve,{once:true});
   started();
  });
  if(captured.aborted)throw captured.reason;
  return new Response(null,{status:404});
 },
});
const port=createProjectFleetPortV1({root:fixture,environment:{},client,owner:"fixture",platform:"macos"});
const abort=new AbortController();
let settled=false;
const attempt=takeAdmittedV1((name,signal)=>runTakeLookV1(()=>port.gate({cachePath:fixture+"/record",fleet:name,keyText:"[]",exportRecord:()=>undefined,import:async()=>false}),signal).then(()=> "miss"),()=>undefined,abort.signal);
const task=takePassV1({names:["fixture"],attempt,signal:abort.signal}).finally(()=>{settled=true;});
await entered;
abort.abort(new Error("test take deadline"));
await new Promise(resolve=>setImmediate(resolve));
const evidence={networkSignalAborted:captured.aborted,passSettled:settled};
release();
await task;
console.log("[DEBUG] "+JSON.stringify(evidence));
if(!evidence.networkSignalAborted)process.exitCode=1;
