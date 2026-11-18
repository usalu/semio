import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {readBuildOwnerRegistryV1,buildInvocationGraphSelectionV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const root=process.env.SEMIO_WORKSPACE_ROOT!,out=process.env.R140_OUT!;
const owner:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
const observation:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const registry=readBuildOwnerRegistryV1(JSON.parse(owner.readBuildOwnerRegistrySourceV1(root)));
const selection=buildInvocationGraphSelectionV1(["run-many","--projects=repo-mcp,@semio-tech/norm-din4108","--targets=build"]);
const context={workspaceRoot:root,selection,nativeCache:new Map(),nativeClosures:new Map()},result:any={};
await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
 for(const name of ["repo-mcp","@semio-tech/norm-din4108"]){const o=registry.owners.find((x:any)=>x.name===name)!;const s=performance.now();const v=await owner.discoverBuildOwnerAdmissionFactsV1(o,["build"],registry,context,[]);result[name]={ms:Math.round(performance.now()-s),files:v.files,dependencies:v.dependencies,dependencyInputs:v.dependencyInputs,targets:Object.fromEntries(Object.entries(v.project.targets).filter(([k])=>k==="build").map(([k,t]:any)=>[k,{dependsOn:t.dependsOn,inputs:t.inputs}]))};}
}));
result.owners=registry.owners.map((o:any)=>[o.name,o.root,o.nativeRoots??[]]);result.providers=registry.catalogProviders;
writeFileSync(join(out,"probe6.json"),JSON.stringify(result));
