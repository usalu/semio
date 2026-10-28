import{setOverlay,headContent,changedSinceHead}from"./overlay.mjs";
import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(L+"👁️observation/🟨️.mjs");const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const changed=changedSinceHead().filter(f=>/\.(ts|tsx|mjs|cjs|js|json)$/.test(f));
const inIndex=new Set(idx.rows.filter(r=>r.kind==="analysis"||r.kind==="file").map(r=>r.path));
const relevant=changed.filter(f=>inIndex.has(R+f));
console.log("changed since HEAD",changed.length,"in index as analysis/file rows",relevant.length);
const sel=G.selectFactClassesV1(idx);
const hold=async(rows,signal)=>{const plain=rows.filter(r=>r.kind!=="closure"&&r.kind!=="field"&&r.kind!=="digest");return plain.length===0||obs.observationsHold(plain,64,new Set(),true,signal);};
async function run(label,files){
 const m=new Map();for(const f of files){const b=headContent(f);if(b)m.set(R+f,b);}setOverlay(m);
 const t=Date.now();const{stale}=await G.staleFactRowsV1(idx,sel.ids,hold,{});
 const kinds={};for(const id of stale)kinds[idx.rows[id].kind]=(kinds[idx.rows[id].kind]??0)+1;
 const owners=G.staleConsumersV1(idx,sel.classes,stale);
 console.log(JSON.stringify({label,edited:m.size,staleRows:stale.size,kinds,staleOwners:owners.size,ms:Date.now()-t}));
 return owners;
}
await run("no-edit",[]);
await run("all-since-00:26",relevant);
for(const f of relevant.slice(0,Number(process.argv[3]??6)))await run("single "+f.slice(-70),[f]);
