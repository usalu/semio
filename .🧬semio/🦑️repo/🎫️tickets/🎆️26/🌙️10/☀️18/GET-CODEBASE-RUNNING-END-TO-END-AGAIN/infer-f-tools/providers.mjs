import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const recorded=new Map();for(const r of idx.rows)if(r.kind==="analysis"){const k=r.fact.provider.split(":")[1]+" @"+r.fact.revision.slice(0,12);recorded.set(k,(recorded.get(k)??0)+1);}
console.log("RECORDED");for(const[k,n]of recorded)console.log(n,k.slice(0,90));
const prepared=inference.prepareSourceAnalysesV1();
console.log("CURRENT");for(const[name,p]of Object.entries(prepared))console.log(name,JSON.stringify(Object.keys(p)),(p.provider?.id??p.id??"").slice(0,80),(p.provider?.revision??p.revision??"").slice(0,12));
