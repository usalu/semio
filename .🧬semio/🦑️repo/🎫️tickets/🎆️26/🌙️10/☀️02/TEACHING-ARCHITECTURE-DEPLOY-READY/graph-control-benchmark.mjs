/** ⏱️ Measures actual issued Original controls without replacing any authority guard. */
import path from "node:path";import {pathToFileURL} from "node:url";
const root=process.cwd(),load=p=>import(pathToFileURL(path.join(root,p))),started=performance.now(),rows=[];
const processApi=await load("🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts"),inference=await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const scripts=await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),write=process.stderr.write;
let writes=0,bytes=0;process.stderr.write=function(text){writes++;bytes+=Buffer.byteLength(String(text));return true};
try{
 await processApi.withScriptProcessEnvelope(processApi.createScriptProcessEnvelope({version:1,owner:"repository:build",maximumElapsedMilliseconds:30000},{fixture:"graph-control-cost"},Date.now()),async invocation=>{
  processApi.requireIssuedScriptInvocationV1(invocation);
  for(const depth of [1,8,32,128]){
   const stop=new AbortController();let phase=processApi.createIssuedResourceControlV1(invocation,"build:command-source-imports",0,stop.signal);
   for(let i=1;i<depth;i++)phase=processApi.createRequestedIssuedResourceControlV1(phase,0);
   for(const force of [false,true]){
    const context={invocation,phaseControl:phase},count=1000,at=performance.now(),beforeWrites=writes,beforeBytes=bytes;
    for(let i=0;i<count;i++){if(performance.now()-started>29000)throw Error("Bounded own benchmark deadline");await scripts.yieldInferenceV1(context,"build:command-source-imports",i,force)}
    rows.push({depth,force,count,milliseconds:Math.round((performance.now()-at)*1000)/1000,writes:writes-beforeWrites,bytes:bytes-beforeBytes});
   }
   const primary=Object.freeze({cancelledAtDepth:depth});stop.abort(primary);let refused=false;try{await scripts.yieldInferenceV1({invocation,phaseControl:phase},"build:command-source-imports",1001,true)}catch(error){refused=error===primary}if(!refused)throw Error("Exact actual cancellation must survive benchmark");
   processApi.requireIssuedScriptInvocationV1(invocation);
  }
 });
}finally{process.stderr.write=write}
console.log(JSON.stringify({milliseconds:Math.round(performance.now()-started),rows,cancellationPreserved:true}));
