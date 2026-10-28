import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const seen=new Set();
for(const r of idx.rows){if(seen.has(r.kind))continue;seen.add(r.kind);console.log(r.kind,r.path.slice(-90),JSON.stringify(r.fact).slice(0,300));}
const a=idx.rows.find(r=>r.kind==="analysis");console.log(JSON.stringify(a,null,1).slice(0,1500));
