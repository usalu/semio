/** 🧭 For real command scripts in a mirror with the `providers` batch: the bounded own roster (what a product node now names), the full roster through every provider port (what it named before) and, for the modules that landed in generations 624-628 (cargofix witness, actenv action, hotpath2 identity), whether each command still names them itself or only through a provider reference. usage (inside the mirror seal): bun reach.ts <mirror-root> <command-script>... */
const [root,...scripts]=process.argv.slice(2) as [string,...string[]],L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import(L+"/🏗️build/👁️observation/🟨️.mjs") as any,si=await import(L+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any;
const landed=["🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/","🏗️bazel/🌿️environment/🏃️action/","🏗️bazel/🌿️environment/🔏️identity/"];
for(const script of scripts){
 const r=await observation.observeInferenceSnapshotV1(async()=>{const c=si.createScriptInputCache({workspaceRoot:root});const dir=script.slice(0,script.lastIndexOf("/"));
  const own=(await si.targetScriptSources({options:{cwd:dir,command:"repo ./📜️script.ts test"}},root,c,dir)).closure??[];const full=await si.relativeScriptInputs([root+"/"+script],root,c,true);return{own,full};});
 const files=(l:any[])=>l.filter((x:any)=>typeof x==="string"),own=files(r.own),full=files(r.full),refs=r.own.filter((x:any)=>typeof x?.input==="string").map((x:any)=>x.input.split(":")[1]);
 console.log("[DEBUG] reach",JSON.stringify({script:script.split("/").slice(-4,-1).join("/"),own:own.length,full:full.length,library_own:own.filter((f:string)=>f.includes("📚️library/")).length,library_full:full.filter((f:string)=>f.includes("📚️library/")).length,ports:refs.length,landed:landed.map(p=>own.some((f:string)=>f.includes(p))?"own":full.some((f:string)=>f.includes(p))?"provider-only":"absent")}));
}
