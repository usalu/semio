const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const entry=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
await obs.observeInferenceSnapshotV1(async()=>{
 const scripts=inference.createScriptInputCache({workspaceRoot:WS});
 for(const pass of [1,2]){
  const t0=performance.now();const d=await obs.observeDetachedV1(()=>si.relativeScriptInputs([entry],WS,scripts));
  console.log("ms",Math.round(performance.now()-t0),"pass",pass,"roster",d.value.length,"own rows",d.rows.length,"layers",d.layers.map(l=>l.length));
 }
});
