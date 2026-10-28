import{setOverlay,headContent}from"./overlay.mjs";
import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";process.chdir(R);
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");const obs=await import(L+"👁️observation/🟨️.mjs");
const f="🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
const entry=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts";
async function roster(){const scripts=si.createScriptInputCache({workspaceRoot:R.slice(0,-1)});return obs.observeInferenceSnapshotV1(()=>si.relativeScriptInputs([entry],R.slice(0,-1),scripts));}
const a=await roster();
const extra="🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/🟦️.ts";import("node:fs").then(()=>{});const live=readFileSync(R+f);setOverlay(new Map([[R+f,Buffer.concat([live,Buffer.from("\nimport \""+"../../"+"🧊️3d/📦️packages/🟦️typescript/🟦️.ts"+"\";\n")])]]));obs.releaseInferenceMemoryV1();
const b=await roster();
console.log("live",a.length,"reverted",b.length,"only live",a.filter(x=>!b.includes(x)).slice(0,5));
