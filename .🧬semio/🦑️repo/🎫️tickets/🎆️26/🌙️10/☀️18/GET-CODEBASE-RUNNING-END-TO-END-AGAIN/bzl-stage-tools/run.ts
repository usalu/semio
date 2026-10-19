// 🚀 Runs a harness script under a genuinely issued process envelope, as the real runner would supply it.
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const root=process.cwd(),[script,...rest]=process.argv.slice(2),selection=(process.env.BZL_STAGE_SELECTION??"run fixture:build").split(" "),capabilities=createNativeBuildCallerCapabilitiesV1(root,selection,"bazel",{});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bzl-stage:harness",maximumElapsedMilliseconds:3300000},capabilities,Date.now()),async original=>{
 const child=Bun.spawn([process.execPath,script!,...rest],{cwd:root,env:scriptInvocationEnvironment(original,{...process.env}),stdout:"inherit",stderr:"inherit"});
 process.exitCode=await child.exited;
});
