import{brotliDecompressSync}from"node:zlib";import{readdirSync,readFileSync}from"node:fs";import{join}from"node:path";
const root="/home/deck/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const dir=process.argv[2];
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const si=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),obs=await import(B+"/👁️observation/🟨️.mjs");
const recipes=readdirSync(join(dir,"e")).filter(n=>n.endsWith(".br")).map(n=>JSON.parse(brotliDecompressSync(readFileSync(join(dir,"e",n))).toString()).recipe);
const scripts=inference.createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=si.closureApiV1(scripts,root);
await obs.observeInferenceSnapshotV1(async()=>{
for(const recipe of recipes){const s=performance.now();const key=si.closureRecipeKeyV1(recipe);try{await api.resolve([[key,recipe]]);}catch(e){console.log("[DEBUG] ERR",String(e.message).slice(0,80));}
 console.log("[DEBUG]",recipe.fn,(recipe.projectRoot??recipe.port??"").slice(-30),Math.round(performance.now()-s),"ms",JSON.stringify(si.closureEntryCountsV1()));}
});
