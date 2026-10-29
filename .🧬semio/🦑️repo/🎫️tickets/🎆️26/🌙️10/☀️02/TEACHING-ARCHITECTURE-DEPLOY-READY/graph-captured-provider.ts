/** 🧊️ Measures one provider query inside the canonical authenticated Source loader. */
import * as fs from "node:fs";import * as path from "node:path";
import * as api from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import * as scripts from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import * as observation from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
import * as argsApi from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📤️arguments/🟨️.mjs";
import * as processApi from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {requireCapturedBuildCommandV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts";
/** 🔏️ Requires the actual captured identities before consuming projected Original authority. */
export async function runCapturedBuildSourceV1(values:readonly string[],proof:any):Promise<void>{
 requireCapturedBuildCommandV1(proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity);proof.recheck();
 const workspace=process.cwd(),physical=import.meta.semioCapturedPhysicalSources as any,resolutions=import.meta.semioCapturedSourceResolutions as any,root=values[0];
 if(physical?.root!==workspace||resolutions?.root!==workspace||!physical.sources?.some((row:any)=>row.path===values[1])||!resolutions.files?.some((row:any)=>row.path===values[1]))throw Error("Actual same-root captured provider entry required");
 console.log(JSON.stringify({phase:"captured-entry",startupMilliseconds:Date.now()-Number(values[2]),physicalRows:physical.sources.length,resolutionRows:resolutions.files.length,identityMatches:import.meta.semioCapturedImplementationIdentity===proof.sha256,loaderMatches:import.meta.semioCapturedLoaderIdentity===proof.loaderSha256}));
 await processApi.receiveScriptProcessInvocation(process.env,async invocation=>processApi.withIssuedResourceControlV1(invocation,"build:captured-provider-query",30000,async phaseControl=>{
  const started=performance.now(),rows:any[]=[];await api.libraryBootstrap;const cache=api.createScriptInputCache({workspaceRoot:workspace,invocation,phaseControl});cache.lateClosures=true;
  const closures=scripts.closureApiV1(cache,workspace),project=JSON.parse(fs.readFileSync(path.join(root,"📋️project.json"),"utf8"));
  await observation.observeInferenceSnapshotV1(async()=>{for(const [name,authored]of Object.entries(project.targets)){
   const at=performance.now(),target=structuredClone(authored) as any;
   if(target.metadata?.semio?.execution?.kind!=="source"){
    const manifest=root+"/Cargo.toml",cwd=target.options.cwd??root,bound=argsApi.bindBuildCommandV1(target.options.command,workspace,cwd,root,[]).command,owner=argsApi.declaredNativeOwnerCommandV1(manifest,cwd,bound),driver="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
    target.options={...target.options,ownerArguments:true,cwd:".",nativeOwnerCommand:owner,command:"repo "+JSON.stringify(driver)+" native owner-command --manifest "+JSON.stringify(manifest)+" --cwd "+JSON.stringify(cwd)+" -- "+bound};
   }
   const result=await observation.captureObservedSourceInputsV1(async()=>{const list=await closures.nativeTargetCommandInputs(target,undefined,root);await closures.some(list,["/🏃️process/🧪️testing/🦀️cargo/"]);await closures.query(list,"workspaceKeys",target.options.command);return closures.generatedDependencies(target,root,root+"/Cargo.toml")},undefined,false);
   const row={name,milliseconds:Math.round(performance.now()-at),observations:result.observations.length,modules:cache.modules.size};rows.push(row);console.log(JSON.stringify(row));
  }});
  processApi.requireIssuedResourceControlInvocationV1(phaseControl,invocation);proof.recheck();requireCapturedBuildCommandV1(proof,import.meta.semioCapturedImplementationIdentity,import.meta.semioCapturedLoaderIdentity);
  console.log(JSON.stringify({phase:"captured-query-complete",milliseconds:Math.round(performance.now()-started),rows}));
 }));
 proof.recheck();
}
