import{readFileSync}from"node:fs";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const holders=new Map();for(const c of idx.classes)for(const r of c.r)holders.set(r,(holders.get(r)??0)+c.o.length);
const rows=idx.rows.map((r,i)=>[r,i]).filter(([r])=>r.kind==="digest"||r.kind==="field");
let stale=0;
for(const[r,i]of rows){let now;try{now=createHash("sha256").update(readFileSync(R+r.path)).digest("hex");}catch{now="!missing";}
 const rec=r.fact?.sha256;const bad=rec!==undefined&&rec!==now;if(bad)stale++;
 if(bad||process.argv[3])console.log(bad?"STALE":"ok   ",r.kind,holders.get(i),r.path.replace(R,"").slice(-100));}
console.log("digest/field rows",rows.length,"stale",stale);
