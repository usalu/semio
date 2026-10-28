// records the natural drift between a fact index and the live tree: stale rows, their consumers, and a snapshot of the bytes of every stale file
import{readFileSync,writeFileSync,mkdirSync,existsSync}from"node:fs";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const obs=await import(L+"👁️observation/🟨️.mjs");const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const [,, facts, outDir]=process.argv;
const idx=G.unpackFactIndexV1(readFileSync(facts),1<<29);
const sel=G.selectFactClassesV1(idx);
const hold=async(rows,signal)=>{const plain=rows.filter(r=>r.kind!=="closure"&&r.kind!=="field"&&r.kind!=="digest");return plain.length===0||obs.observationsHold(plain,64,new Set(),true,signal);};
const t=Date.now();const{stale}=await G.staleFactRowsV1(idx,sel.ids,hold,{});
const kinds={};for(const id of stale)kinds[idx.rows[id].kind]=(kinds[idx.rows[id].kind]??0)+1;
const consumers=new Map();for(const at of sel.classes){const c=idx.classes[at];for(const id of c.r)if(stale.has(id))consumers.set(id,(consumers.get(id)??0)+c.o.length);}
mkdirSync(outDir+"/snap",{recursive:true});
const files={};
for(const id of stale){const r=idx.rows[id];if(r.kind==="analysis"||r.kind==="file"||r.kind==="typescriptText"){try{const b=readFileSync(r.path);const sha=createHash("sha256").update(b).digest("hex");writeFileSync(outDir+"/snap/"+sha,b);files[r.path]=sha;}catch{files[r.path]=null;}}}
writeFileSync(outDir+"/drift.json",JSON.stringify({at:new Date().toISOString(),kinds,stale:[...stale].map(id=>({id,kind:idx.rows[id].kind,path:idx.rows[id].path,consumers:consumers.get(id)??0})),files}));
console.log(JSON.stringify({staleRows:stale.size,kinds,owners:G.staleConsumersV1(idx,sel.classes,stale).size,filesSnapshotted:Object.keys(files).length,ms:Date.now()-t}));
const top=[...stale].sort((a,b)=>(consumers.get(b)??0)-(consumers.get(a)??0)).slice(0,15);
for(const id of top){const r=idx.rows[id];console.log(consumers.get(id),r.kind,r.path.replace(R,"").slice(-110));}
