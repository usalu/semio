/** 🧱️ Bounds one current-Source claimed node through the actual nonpublishing inference body. */
import * as fs from "node:fs";import * as path from "node:path";import {createHash} from "node:crypto";
import * as api from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import * as scripts from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import {withBuildGraphStagingV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import {projectStampPathsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔖️stamps/🟨️.mjs";
import {resolveClosureRefsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs";
import {inferenceInventoryReferencesHoldV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs";
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
 await processApi.receiveScriptProcessInvocation(process.env,async original=>processApi.withIssuedResourceControlV1(original,"build:captured-one-record",55000,async control=>{
  await api.libraryBootstrap;
  const staging=path.resolve(path.dirname(values[1]),"🗑️generated/i"),leaves:string[]=[];if((path.join(staging,"inference","build-lock-closures","a".repeat(64)+".json")+"~"+"a".repeat(36)).length>256)throw Error("Bounded private closure staging path required");await withBuildGraphStagingV1({root:staging,published:(file:string)=>leaves.push(file)},()=>observation.observeInferenceSnapshotV1(async()=>{
   const setup=performance.now(),registry=readBuildOwnerRegistryV1(JSON.parse(api.readBuildOwnerRegistrySourceV1(workspace))),owner=registry.owners.find(row=>row.root===root);if(!owner||owner.manifest!==root+"/📋️project.json")throw Error("Declared authored claimed owner required");
   const claim=JSON.parse(fs.readFileSync(".cache/b/g/m/generations/c/49620f499d3a2883fbe02d40.json","utf8"));if(claim.state!=="running"||!claim.owners.includes(owner.name))throw Error("Actual current claim membership required");
   const json={...JSON.parse(fs.readFileSync(path.join(workspace,owner.manifest),"utf8")),root},nativeRoot=json.metadata?.nativeRoot??root,native=fs.existsSync(path.join(workspace,nativeRoot,"Cargo.toml")),pipeline=Object.values(json.targets??{}).some((target:any)=>target.metadata?.semio?.release);if(pipeline)throw Error("This bounded native job must not read componentRoots");
   const projectsByRoot=registry.owners.flatMap(row=>[row.root,...row.nativeRoots??[]].map(root=>[root,row.name])),contracts=nodes.projectGeneratorContractsV1(workspace),seed=api.createScriptInputCache({workspaceRoot:workspace,registry,lateClosures:true}),nativeAt=performance.now(),commandInputs=await scripts.closureApiV1(seed,workspace).nativeCommandInputs(),handle=scripts.closureHandleOfV1(commandInputs);if(!handle)throw Error("Actual late native command recipe required");
   console.log(JSON.stringify({phase:"native-command-inputs",milliseconds:Math.round(performance.now()-nativeAt),recipe:handle.recipe}));
   const session={workspaceRoot:workspace,registry,contracts,commandInputs:{closureRecipe:handle.recipe},lateClosures:true,projectsByRoot,componentRoots:[]},environment=nodes.createProjectInferenceEnvironmentV1(session),selectedInputs:string[]=[],job={name:owner.name,root,json,native,pipeline,nativeRoot,selectedTargets:undefined,selectedInputs};
   if(environment.context.invocation!==undefined||environment.context.phaseControl!==undefined||job.selectedInputs!==selectedInputs)throw Error("Exact farm context and selectedInputs identity required");
   console.log(JSON.stringify({phase:"full-node-start",setupMilliseconds:Math.round(performance.now()-setup),name:job.name,native,pipeline,selectedInputs,projectRoots:projectsByRoot.length,contracts:Object.keys(contracts).length,contextKeys:Object.keys(environment.context)}));
   const authorityAt=performance.now(),assembly=api.cacheInternals.projectAssemblyAuthorityV1(workspace,native?nativeRoot:undefined,environment.projectsByRoot,undefined,environment.nativeCache,environment.nativeClosures),digest=(value:unknown)=>createHash("sha256").update(JSON.stringify(value)).digest("hex"),head=[digest([contracts]),digest(registry.catalogProviders.map(({ownerRoot,sourcePath})=>[ownerRoot,sourcePath])),native?digest(handle.key):null,assembly,"closure-references/4"],cacheKey=createHash("sha256").update(root).digest("hex").slice(0,24),recordJob={...job,head,cacheKey},cachePath=path.join(staging,"inference","np",cacheKey+".json");
   console.log(JSON.stringify({phase:"record-authority",milliseconds:Math.round(performance.now()-authorityAt),cacheKey,head,mode:values[3]??"cold"}));
   Object.assign(environment.context,{invocation:original,phaseControl:control});Object.assign(environment.scripts.context,{invocation:original,phaseControl:control});
   let inventoryCalls=0,computations=0;const at=performance.now(),project=await observation.observeProjectInferenceV1({cachePath,key:nodes.projectRecordKeyV1(head,root,json,undefined,selectedInputs),owner:root,stamps:projectStampPathsV1({native}),reusable:async(value:any)=>{inventoryCalls++;return nodes.projectRecordReusableV1(value)&&await inferenceInventoryReferencesHoldV1(workspace,value,control)}},async()=>{computations++;return nodes.inferProjectBodyV1(environment,job,undefined)});
   processApi.requireIssuedResourceControlInvocationV1(control,original);proof.recheck();console.log(JSON.stringify({phase:"inline-wrapper-complete",milliseconds:Math.round(performance.now()-at),computations,inventoryCalls,recipes:Object.keys(project.closureRecipes??{}).length,closureInputs:Object.keys(project.closureInputs??{}).length,physicalRecord:fs.existsSync(cachePath),contextKeys:Object.keys(environment.context)}));
   const resolving=performance.now();console.log(JSON.stringify({phase:"inline-resolve-start"}));const resolved=await resolveClosureRefsV1(project,workspace,environment.scripts),text=JSON.stringify(resolved.project);processApi.requireIssuedResourceControlInvocationV1(control,original);proof.recheck();console.log(JSON.stringify({phase:"inline-resolve-complete",milliseconds:Math.round(performance.now()-resolving),bytes:Buffer.byteLength(text),sha256:createHash("sha256").update(text).digest("hex"),rows:resolved.rows.length,derived:resolved.derived.length,targets:Object.keys(resolved.project.targets??{}).length,modules:environment.scripts.modules.size,privateLeaves:leaves.length}));

  }));
  processApi.requireIssuedResourceControlInvocationV1(control,original);
 }));
 proof.recheck();
}
