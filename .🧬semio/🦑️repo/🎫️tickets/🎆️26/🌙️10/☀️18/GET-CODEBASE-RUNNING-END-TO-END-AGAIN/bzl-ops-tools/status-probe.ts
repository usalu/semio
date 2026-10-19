import {createNativeBuildCallerCapabilitiesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts";
import {bazelBuildCacheToolV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/⚙️operations/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,advanceScriptInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["cache","status"],"bazel",{}),action=process.argv[2]??"status";
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bzl-ops:probe",maximumElapsedMilliseconds:240000},capabilities,Date.now()),async original=>{
 const phaseControl=createIssuedResourceControlV1(original,"bzl-ops:probe",200000),context={root,invocation:original,phaseControl,progress:async(stage:string)=>{console.log("progress "+stage);}},tool=bazelBuildCacheToolV1();
 if(action==="clear"){await tool.clear(context);}
 console.log(JSON.stringify(await tool.status(context),null,2));
});
