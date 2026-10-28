const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const rows=inference.loadedImplementationRowsV1.filter(r=>r.path.endsWith("🔣️policy.json")).map(r=>[r.path.slice(-40),r.fact.sha256.slice(0,12),r.fact.ignore?.length??0]);
console.log(JSON.stringify({revision:inference.currentImplementationRevisionV1().slice(0,16),rows}));
