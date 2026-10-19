import {createNativeBuildCallerCapabilitiesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,scriptInvocationEnvironment} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{});
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bzl-act:test",maximumElapsedMilliseconds:900000},capabilities,Date.now()),async original=>{
 const child=Bun.spawn([process.execPath,"test",...process.argv.slice(2)],{cwd:root,env:scriptInvocationEnvironment(original,{...process.env}),stdout:"inherit",stderr:"inherit"});
 process.exitCode=await child.exited;
});
