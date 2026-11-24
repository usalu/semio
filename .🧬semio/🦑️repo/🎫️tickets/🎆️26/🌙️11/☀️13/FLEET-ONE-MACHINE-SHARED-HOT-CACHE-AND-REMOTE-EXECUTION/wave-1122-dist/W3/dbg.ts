const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",X=L+"🕸️graph/📥️load/🤝️selected-exchange/";
const ex=await import(X+"🟦️.ts"),at=await import(X+"🔌️attempt/🟦️.ts");
const client=at.selectedFleetClientV1(root);console.log("usable",client.usable?.(),Object.keys(client));
const scope=process.argv[2];
const key=ex.selectedHeadKeyV1({scope,platform:"darwin-arm64"});console.log(key);
console.log(await client.getResult(key).then((a:any)=>typeof a==="string"?a:Object.keys(a)).catch((e:any)=>String(e)));
