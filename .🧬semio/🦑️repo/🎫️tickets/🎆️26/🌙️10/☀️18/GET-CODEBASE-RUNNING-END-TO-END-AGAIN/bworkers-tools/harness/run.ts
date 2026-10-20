// 🚀 Runs a harness script under a genuinely issued process envelope, as the real runner would supply it.
import {join} from "node:path";
const root=process.cwd(),at=(path:string)=>join(root,path);
const {createNativeBuildCallerCapabilitiesV1}=await import(at("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts"));
const {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment}=await import(at("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
const [script,...rest]=process.argv.slice(2),selection=(process.env.BW_SELECTION??"run synthetic:build").split(" "),capabilities=createNativeBuildCallerCapabilitiesV1(root,selection,"bazel",{});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bworkers:harness",maximumElapsedMilliseconds:3300000},capabilities,Date.now()),async (original:any)=>{
 const child=Bun.spawn([process.execPath,script!,...rest],{cwd:root,env:scriptInvocationEnvironment(original,{...process.env}),stdout:"inherit",stderr:"inherit"});
 process.exitCode=await child.exited;
});
