import{join}from"node:path";import{readdirSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const obs=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const np=process.argv[2];const f=readdirSync(np).filter(n=>n.endsWith(".json"))[Number(process.argv[3]??0)];
const h=obs.readInferenceHeaderV1(join(np,f));
console.log(f,JSON.stringify(h).slice(0,1200));
