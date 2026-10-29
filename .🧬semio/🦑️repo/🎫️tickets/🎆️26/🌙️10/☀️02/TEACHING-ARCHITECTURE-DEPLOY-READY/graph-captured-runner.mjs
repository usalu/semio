/** 🚰️ Captures and launches the bounded provider probe through canonical owned auxiliary DATA. */
import path from "node:path";import {pathToFileURL} from "node:url";
const workspace=process.cwd(),entry=process.argv[2]??".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/graph-captured-provider.ts",load=p=>import(pathToFileURL(path.join(workspace,p))),base="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const routing=await load("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),capture=await load(base+"/🏗️build/📥️source/🔏️closure/🏗️compiler/🟦️.ts"),closure=await load(base+"/🏗️build/📥️source/🔏️closure/🟦️.ts"),execution=await load("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"captured-provider-fixture",maximumElapsedMilliseconds:60000},{fixture:"captured-provider-differential"},Date.now()),async original=>{
 const control=routing.createIssuedResourceControlV1(original,"build:captured-provider-differential",60000);
 await routing.withIssuedResourceOperationV1(control,async()=>{
  routing.requireIssuedResourceControlInvocationV1(control,original);const at=performance.now(),snapshot=await capture.captureCompilerBuildSourceClosureV1(workspace,entry,original,control,process.env),{text,sha256}=closure.buildSourceClosureTextV1(snapshot);
  console.log(JSON.stringify({phase:"capture-complete",milliseconds:Math.round(performance.now()-at),files:snapshot.files.length,bytes:Buffer.byteLength(text)}));
  routing.requireIssuedResourceControlInvocationV1(control,original);
  await execution.runOwnedCommand(process.execPath,["--eval",closure.buildSourceClosureStreamLoaderV1,"--",sha256,"🧰️framework/📦️packages/🦀️rust",entry,String(Date.now()),process.argv[3]??"cold"],workspace,"captured-provider-query",routing.scriptInvocationBudget(original,0),{control,signal:control.signal,env:{...routing.scriptResourceProcessEnvironmentV1(control,process.env),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},auxiliaryInput:{version:1,text},outputStyle:"stream-without-prefixes",output:{maximumBytes:131072,maximumLines:256},onProgress:line=>execution.writeOwnedCommandOutputV1(process.stderr,line+"\n",control)});
  routing.requireIssuedResourceControlInvocationV1(control,original);
 });
});
