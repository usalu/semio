import{parentPort,workerData}from"node:worker_threads";
const ROOT=workerData.root,B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const routing=await import(ROOT+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const{loadBuildInvocationGraphV1}=await import(B+"🧭️invocation/🕸️graph/🟦️.ts");
const{buildBackendCompactGraphV1,loadBuildBackendV1,buildBackendDefaultIdV1}=await import(B+"🔌️backends/🟦️.ts");
const{sealedGenerationPinV1}=await import(B+"🕸️graph/🟦️.ts");
try{
  await routing.receiveScriptProcessInvocation(process.env,async(original:any)=>{
    const control=routing.createIssuedResourceControlV1(original,"graph:slice-worker",0),tool=workerData.tool??buildBackendDefaultIdV1(),backend=await loadBuildBackendV1(tool);
    const graph:any=await loadBuildInvocationGraphV1(ROOT,workerData.arguments,original,original,control,false,undefined,workerData.graphPolicy,buildBackendCompactGraphV1(tool),await backend.graphExternalInputs?.()??[]);
    parentPort!.postMessage({type:"done",graph,generationPin:sealedGenerationPinV1(tool)});
  });
}catch(error:any){parentPort!.postMessage({type:"error",message:String(error?.message??error),stack:String(error?.stack??"")});}
