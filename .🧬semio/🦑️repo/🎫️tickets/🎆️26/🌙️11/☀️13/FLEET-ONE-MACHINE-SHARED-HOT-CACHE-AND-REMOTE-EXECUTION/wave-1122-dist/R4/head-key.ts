const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/";
const ex=await import(B+"🟦️.ts");
const scope=process.argv[2]!;
console.log(JSON.stringify({head:ex.selectedHeadKeyV1({scope,platform:"darwin-arm64"}),slot:ex.selectedHeadSlotV1({scope,platform:"darwin-arm64"}),manifest:ex.retentionManifestV1().hash}));
