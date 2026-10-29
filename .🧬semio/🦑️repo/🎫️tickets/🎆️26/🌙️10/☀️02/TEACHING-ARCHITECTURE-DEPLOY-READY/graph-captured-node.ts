/** 🧱️ Bounds one current-Source claimed node through the actual nonpublishing inference body. */
import * as fs from "node:fs";import * as path from "node:path";import {createHash} from "node:crypto";
import * as api from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import * as scripts from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import {withBuildGraphStagingV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import * as nodes from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs";
import * as observation from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
import {readBuildOwnerRegistryV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import * as processApi from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {requireCapturedBuildCommandV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts";
/** 🔏️ Authenticates the loader and projected Original before one production computation. */
export async function runCapturedBuildSourceV1(values:readonly string[],proof:any):Promise<void>{
 requireCapturedBuildCommandV1(proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity);proof.recheck();
 const workspace=process.cwd(),physical=import.meta.semioCapturedPhysicalSources as any,resolutions=import.meta.semioCapturedSourceResolutions as any,root=values[0];
 if(physical?.root!==workspace||resolutions?.root!==workspace||!physical.sources?.some((row:any)=>row.path===values[1]))throw Error("Actual captured node entry required");
 console.log(JSON.stringify({phase:"captured-node-entry",startupMilliseconds:Date.now()-Number(values[2]),physicalRows:physical.sources.length,resolutionRows:resolutions.files.length}));
 await processApi.receiveScriptProcessInvocation(process.env,async original=>processApi.withIssuedResourceControlV1(original,"build:captured-one-node",30000,async control=>{
  await api.libraryBootstrap;
  const staging=path.resolve(path.dirname(values[1]),"🗑️generated/n"),leaves:string[]=[];if((path.join(staging,"inference","build-lock-closures","a".repeat(64)+".json")+"~"+"a".repeat(36)).length>256)throw Error("Bounded private closure staging path required");await withBuildGraphStagingV1({root:staging,published:(file:string)=>leaves.push(file)},()=>observation.observeInferenceSnapshotV1(async()=>{
   const setup=performance.now(),registry=readBuildOwnerRegistryV1(JSON.parse(api.readBuildOwnerRegistrySourceV1(workspace))),owner=registry.owners.find(row=>row.root===root);if(!owner||owner.manifest!==root+"/📋️project.json")throw Error("Declared authored claimed owner required");
   const claim=JSON.parse(fs.readFileSync(".cache/b/g/m/generations/c/49620f499d3a2883fbe02d40.json","utf8"));if(claim.state!=="running"||!claim.owners.includes(owner.name))throw Error("Actual current claim membership required");
   const json={...JSON.parse(fs.readFileSync(path.join(workspace,owner.manifest),"utf8")),root},nativeRoot=json.metadata?.nativeRoot??root,native=fs.existsSync(path.join(workspace,nativeRoot,"Cargo.toml")),pipeline=Object.values(json.targets??{}).some((target:any)=>target.metadata?.semio?.release);if(pipeline)throw Error("This bounded native job must not read componentRoots");
   const projectsByRoot=registry.owners.flatMap(row=>[row.root,...row.nativeRoots??[]].map(root=>[root,row.name])),contracts=nodes.projectGeneratorContractsV1(workspace),seed=api.createScriptInputCache({workspaceRoot:workspace,registry,lateClosures:true}),nativeAt=performance.now(),commandInputs=await scripts.closureApiV1(seed,workspace).nativeCommandInputs(),handle=scripts.closureHandleOfV1(commandInputs);if(!handle)throw Error("Actual late native command recipe required");
   console.log(JSON.stringify({phase:"native-command-inputs",milliseconds:Math.round(performance.now()-nativeAt),recipe:handle.recipe}));
   const session={workspaceRoot:workspace,registry,contracts,commandInputs:{closureRecipe:handle.recipe},lateClosures:true,projectsByRoot,componentRoots:[]},environment=nodes.createProjectInferenceEnvironmentV1(session),selectedInputs:string[]=[],job={name:owner.name,root,json,native,pipeline,nativeRoot,selectedTargets:undefined,selectedInputs};
   if(environment.context.invocation!==undefined||environment.context.phaseControl!==undefined||job.selectedInputs!==selectedInputs)throw Error("Exact farm context and selectedInputs identity required");
   console.log(JSON.stringify({phase:"full-node-start",setupMilliseconds:Math.round(performance.now()-setup),name:job.name,native,pipeline,selectedInputs,projectRoots:projectsByRoot.length,contracts:Object.keys(contracts).length,contextKeys:Object.keys(environment.context)}));
   const at=performance.now(),result=await observation.captureObservedSourceInputsV1(()=>nodes.inferProjectBodyV1(environment,job,undefined),undefined,false),text=JSON.stringify(result.value);
   processApi.requireIssuedResourceControlInvocationV1(control,original);proof.recheck();console.log(JSON.stringify({phase:"full-node-complete",milliseconds:Math.round(performance.now()-at),name:result.value.name,root:result.value.root,unavailable:result.value.unavailable??null,targets:Object.keys(result.value.targets??{}),bytes:Buffer.byteLength(text),sha256:createHash("sha256").update(text).digest("hex"),observations:result.observations.length,modules:environment.scripts.modules.size,privateLeaves:leaves.map(file=>path.relative(staging,file)),maximumPrivatePathUnits:Math.max(0,...leaves.map(file=>file.length))}));
  }));
  processApi.requireIssuedResourceControlInvocationV1(control,original);
 }));
 proof.recheck();
}
