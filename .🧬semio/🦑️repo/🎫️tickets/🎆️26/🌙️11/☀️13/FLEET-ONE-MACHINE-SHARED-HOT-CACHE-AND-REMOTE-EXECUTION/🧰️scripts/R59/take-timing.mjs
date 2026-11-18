import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const P=await import(B+"/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const largest=store=>{const cas=join(store,"v1/main/cas");let best;for(const d of readdirSync(cas))for(const f of readdirSync(join(cas,d))){let x;try{x=brotliDecompressSync(readFileSync(join(cas,d,f)))}catch{continue}if(!best||x.length>best.length)best=x}return best};
const host=P.hostOfRootV1(root);
for(const [label,store,packed] of[["old",process.argv[2],false],["new",process.argv[3],true]]){
 const bytes=largest(store);global.gc?.();const rss0=process.memoryUsage().rss,t=performance.now();
 const j=JSON.parse(bytes.toString()),bundle=packed?P.unpackBundleV1(j):j;
 const local=P.localBundleV1({recipeKey:P.closureKeyV1(bundle.recipe),bundle},host,new Map(),{});
 console.log(label,"doc bytes",bytes.length,"decode+localize ms",Math.round(performance.now()-t),"rss delta MB",Math.round((process.memoryUsage().rss-rss0)/1048576),"layers",local.bundle.layers.length);
}
