const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const scan=await import(L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts");
const real=process.argv[3]==="real";
const r=await scan.supersedeBuildGraphObservationBlocksV1("/Users/ueli/Documents/semio/.cache/b/g/m",{lease:async()=>({release(){}})},{cancelled:()=>false,progress:(l,d,t)=>{if(d===t)console.log(l,d,t)}},{dryRun:!real});
console.log(JSON.stringify(r));
