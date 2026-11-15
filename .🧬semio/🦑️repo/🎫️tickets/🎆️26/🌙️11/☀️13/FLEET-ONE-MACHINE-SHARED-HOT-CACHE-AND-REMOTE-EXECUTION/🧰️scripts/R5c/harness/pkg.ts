const inference=await import("../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const ws="/Users/Shared/semio",PROV="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers";
await observation.observeInferenceSnapshotV1(async()=>{
 const cache=scriptInputs.createScriptInputCache({workspaceRoot:ws,lateClosures:false}),api=scriptInputs.closureApiV1(cache,ws);
 for(const [name,list] of await api.sharedClosureGroups(PROV,"🔣️.json")){if(!name.startsWith("toolClosure:")&&!name.includes("repository-tooling"))continue;const files=list.filter((x:any)=>typeof x==="string").length,pk=list.filter((x:any)=>x?.externalDependencies).flatMap((x:any)=>x.externalDependencies);console.log(name,"files",files,"packages",pk.length,pk.slice(0,6).join(","));}
});
