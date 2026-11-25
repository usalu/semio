/** 📏️ [DEBUG] probe: one real router command walked whole and pruned with fresh caches; prints wall and CPU per walk. */
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",root="/Users/ueli/Documents/semio";
const corpus=(await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🔣️.json",{with:{type:"json"}})).default;
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const item=corpus.realCommands[0],entry=root+"/"+item.script,cpu=()=>{const c=process.cpuUsage();return (c.user+c.system)/1e6;};
for(const label of ["whole","whole-again","whole-3"]){const t=performance.now(),c=cpu();const r=await si.relativeScriptInputs([entry],root,inference.createScriptInputCache({workspaceRoot:root}),true);console.log("[DEBUG]",label,r.length,Math.round(performance.now()-t),"ms",(cpu()-c).toFixed(1),"cpu");}
process.exit(0);
