const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",WS=R.slice(0,-1);process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const entry=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
const r=await obs.observeInferenceSnapshotV1(async()=>si.relativeScriptInputs([entry],WS,inference.createScriptInputCache({workspaceRoot:WS})));
for(const needle of process.argv.slice(2))console.log(needle,r.filter(p=>p.includes(needle)).length);
console.log("roster",r.length);
