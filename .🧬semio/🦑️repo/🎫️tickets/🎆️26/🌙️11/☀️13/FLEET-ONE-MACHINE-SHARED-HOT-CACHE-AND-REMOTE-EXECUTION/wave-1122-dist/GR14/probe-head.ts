const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/";
const ex=await import(B+"🟦️.ts"),at=await import(B+"🔌️attempt/🟦️.ts"),port=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const scope=process.argv[2],client=at.selectedFleetClientV1(root);
if(!client){console.log("no client");process.exit(0);}
const t=Date.now(),r=await ex.lookSelectedGenerationV1({client,scope,platform:"darwin-arm64",host:port.hostOfRootV1(root),lookMaximumMilliseconds:60000,say:(k:string,f:unknown)=>console.log("say",k,JSON.stringify(f))});
console.log(typeof r==="string"?r:{owners:Object.keys(r.head.owners).length,created:new Date(r.head.createdMs).toISOString(),docs:r.head.documents,graph:r.graph.length},Date.now()-t,"ms");
