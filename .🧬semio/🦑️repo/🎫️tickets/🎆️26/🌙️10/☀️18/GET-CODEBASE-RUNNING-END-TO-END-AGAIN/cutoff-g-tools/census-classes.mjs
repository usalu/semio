import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const R="/Users/ueli/Documents/semio/";
const cls=idx.classes.map((c,i)=>({i,n:c.o.length,rows:c.r.length})).sort((a,b)=>b.n*b.rows-a.n*a.rows).slice(0,15);
for(const{i,n,rows}of cls){const c=idx.classes[i];const kinds={};for(const r of c.r){const k=idx.rows[r].kind;kinds[k]=(kinds[k]??0)+1;}
 const sample=idx.rows[c.r[0]];console.log("class",i,"owners",n,"rows",rows,JSON.stringify(kinds),sample.kind,sample.path.replace(R,"").slice(0,110));}
// analysis rows: by owner count buckets
const ownersOf=new Map();for(const c of idx.classes)for(const r of c.r)ownersOf.set(r,(ownersOf.get(r)??0)+c.o.length);
const b={};for(const[r,n]of ownersOf){if(idx.rows[r].kind!=="analysis")continue;const key=n>=3500?">=3500":n>=1000?"1000-3499":n>=100?"100-999":n>=10?"10-99":"<10";b[key]=(b[key]??0)+1;}
console.log("analysis rows by owner count",JSON.stringify(b));
const lib="library/";const cnt={};for(const[r,n]of ownersOf){const row=idx.rows[r];if(row.kind!=="analysis"||n<3500)continue;const p=row.path.replace(R,"");const key=p.includes("🏗️build/")?"build":p.includes("🏃️process/")?"process":p.split("/").slice(0,2).join("/");cnt[key]=(cnt[key]??0)+1;}
console.log("analysis>=3500 by area",JSON.stringify(cnt));
