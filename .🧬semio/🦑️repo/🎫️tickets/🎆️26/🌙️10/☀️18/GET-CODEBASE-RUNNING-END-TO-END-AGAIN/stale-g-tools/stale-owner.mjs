import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(L+"👁️observation/🟨️.mjs");const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const only=process.argv[3]?new Set(JSON.parse(readFileSync(process.argv[3],"utf8"))):undefined;
const sel=G.selectFactClassesV1(idx,only);
const hold=async(rows,signal)=>{const plain=rows.filter(r=>r.kind!=="closure"&&r.kind!=="field"&&r.kind!=="digest");return plain.length===0||obs.observationsHold(plain,64,new Set(),true,signal);};
const t0=Date.now();
const{stale,calls}=await G.staleFactRowsV1(idx,sel.ids,hold,{});
const kinds={};for(const id of stale)kinds[idx.rows[id].kind]=(kinds[idx.rows[id].kind]??0)+1;
const owners=G.staleConsumersV1(idx,sel.classes,stale);
console.log(JSON.stringify({checked:sel.ids.length,stale:stale.size,kinds,owners:owners.size,calls,ms:Date.now()-t0}));
for(const id of [...stale].slice(0,100000)){const r=idx.rows[id];console.log(r.kind,r.path.replace(R,"").slice(0,200),JSON.stringify(r.fact).slice(0,160));}
