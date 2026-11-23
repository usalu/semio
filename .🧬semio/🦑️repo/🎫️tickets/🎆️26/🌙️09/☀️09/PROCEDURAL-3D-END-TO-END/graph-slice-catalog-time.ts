const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",root="/Users/ueli/Documents/semio";
const obs=await import(G+"/👁️observation/🟨️.mjs"),producer=await import(G+"/🕸️graph/📥️inference/🟨️.mjs"),{readBuildOwnerRegistryV1}=await import(G+"/🕸️graph/🚪️admission/🟦️.ts");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const od=await import(G+"/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"),pg=await import(G+"/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs");
const {catalogBuildContributionForManifestV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts");
const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));
const t=async(label:string,f:()=>any)=>{const a=performance.now();const r=await obs.captureObservedSourceInputsV1(async()=>f(),undefined,false);console.log(label,Math.round(performance.now()-a)+"ms");return r.value;};
for(let i=0;i<2;i++){
 const providers=registry.catalogProviders.filter((p:any)=>p.consumerRoots.some((r:string)=>"semio-hub-procedural-component"&&true));
 const contributions=await t("contributions",()=>od.catalogBuildContributionsV1(root,providers.map(({ownerRoot,sourcePath}:any)=>({ownerRoot,sourcePath}))));
 const files=await t("candidates",()=>od.catalogNativeManifestCandidatesV1(root,registry,(p:string)=>catalogBuildContributionForManifestV1(contributions,p),{workspaceRoot:root}));
 const catalog=await t("collectPlaygroundCatalog",()=>pg.collectPlaygroundCatalog(files,root,contributions));
 console.log("providers",providers.length,"contrib",contributions.length,"files",files.length,"catalog",catalog.length);
}
